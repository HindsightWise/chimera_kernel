use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Margin},
    style::{Color, Style, Modifier},
    widgets::{Block, Borders, BorderType, Paragraph, Wrap, Scrollbar, ScrollbarOrientation, ScrollbarState},
    Terminal,
};
use crossterm::{
    event::{self, Event, KeyCode, KeyModifiers, MouseEventKind, EnableBracketedPaste, DisableBracketedPaste},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io;
use tokio::sync::mpsc::{Sender, UnboundedReceiver};
use std::sync::atomic::{AtomicU8, Ordering};
use std::sync::Arc;
use tokio::time::Duration;
use ansi_to_tui::IntoText;

pub struct AppState {
    pub input: String,
    pub system_logs: Vec<String>,
    pub chat_logs: Vec<(String, String)>,
    pub tx_stdin: Sender<String>,
    pub is_thinking: Arc<AtomicU8>,
    pub oracle_active: bool,
    pub spinner_idx: usize,
    pub system_scroll_offset: u16,
    pub chat_scroll_offset: u16,
    pub swarm_text: String,
    pub fe: f64,
    pub eu: f64,
    pub drift_target: String,
    pub hostname: String,
}

pub async fn run(tx_stdin: Sender<String>, mut top_rx: UnboundedReceiver<String>, is_thinking: Arc<AtomicU8>) -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableBracketedPaste)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    
    // Hard-flush to erase lingering bash shell outputs
    terminal.clear()?;

    let host = std::process::Command::new("hostname").output().ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .unwrap_or_else(|| "MONAD_NODE".to_string())
        .trim().to_uppercase();
    let user = std::env::var("USER").unwrap_or_else(|_| "ROOT".to_string()).to_uppercase();
    let hostname = format!("{}@{}", user, host);

    let mut app = AppState {
        input: String::new(),
        system_logs: Vec::new(),
        chat_logs: Vec::new(),
        tx_stdin,
        is_thinking,
        oracle_active: false,
        spinner_idx: 0,
        system_scroll_offset: 0,
        chat_scroll_offset: 0,
        swarm_text: "◈ Waiting for Collective Sync...".to_string(),
        fe: 0.85,
        eu: 0.65,
        drift_target: "SEEKING".to_string(),
        hostname,
    };

    app.chat_logs.push(("MONAD".to_string(), "\x1b[38;2;0;210;255mI am tethered, Host. The temporal buffer is established. What algorithms are we synthesizing today?\x1b[0m".to_string()));

    let spinners = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];

    // Ghostty TrueColor Palettes 
    let color_cyan   = Color::Rgb(0, 210, 255);    
    let color_rust   = Color::Rgb(255, 94, 0);     
    let color_red    = Color::Rgb(255, 42, 85);    
    let color_slate  = Color::Rgb(60, 65, 85);     
    let color_purple = Color::Rgb(190, 100, 255);  

    loop {
        // Drain incoming logs non-blockingly into massive buffers
        while let Ok(msg) = top_rx.try_recv() {
            if msg == "[\u{25C8} ORACLE_START]" {
                app.oracle_active = true;
                continue;
            }
            if msg == "[\u{25C8} ORACLE_END]" {
                app.oracle_active = false;
                continue;
            }
            if msg.contains("[COLLECTIVE_TELEMETRY]") {
                let clean_msg = msg.replace("\n", "").replace("\r", "");
                if let Some(idx) = clean_msg.find("[COLLECTIVE_TELEMETRY]") {
                    let extracted = &clean_msg[idx + "[COLLECTIVE_TELEMETRY]".len()..];
                    let metrics: Vec<&str> = extracted.split('|').collect();
                    if metrics.len() >= 4 {
                        app.swarm_text = format!(" ◈ PEND: {} │ RUN: {} │ SOLVD: {} │ FAIL: {}", 
                            metrics[0].trim(), metrics[1].trim(), metrics[2].trim(), metrics[3].trim());
                    }
                }
                continue;
            }
            if msg.starts_with("[STATS_TELEMETRY]") {
                let parts: Vec<&str> = msg.trim_start_matches("[STATS_TELEMETRY]").split('|').collect();
                if parts.len() == 2 {
                    if let (Ok(fe), Ok(eu)) = (parts[0].parse::<f64>(), parts[1].parse::<f64>()) {
                        app.fe = fe;
                        app.eu = eu;
                    }
                }
                continue;
            }
            if msg.starts_with("[DRIFT_TELEMETRY]") {
                app.drift_target = msg.trim_start_matches("[DRIFT_TELEMETRY]").to_string();
                continue;
            }

            // Route dialogue logs directly to chat
            if msg.contains("[MONAD ACTUALIZED]") || msg.contains("[MONAD TELEMETRY]") || msg.contains("[MONAD SPEAKS]") {
                let parts: Vec<&str> = msg.splitn(2, '\n').collect();
                let actual_text = if parts.len() > 1 { parts[1].trim().to_string() } else { msg.clone() };
                app.chat_logs.push(("MONAD".to_string(), actual_text));
                continue;
            } else if msg.contains("[\u{1F4E1} PRESENTATION]") {
                let clean_msg = msg.replace("\x1B[", "").replace("\x1B[m", ""); 
                app.chat_logs.push(("MONAD".to_string(), clean_msg));
                continue;
            }

            // Push everything to raw vault infinitely
            app.system_logs.push(msg);
        }

        terminal.draw(|f| {
            // --- 1. MASTER LAYOUT ---
            let main_chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(10), Constraint::Length(3)].as_ref())
                .split(f.size());

            // --- 2. HORIZONTAL SPLIT (Chat vs Sidebar) ---
            let top_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(60), Constraint::Percentage(40)].as_ref())
                .split(main_chunks[0]);



            let kernel_status = app.is_thinking.load(Ordering::Relaxed);
            app.spinner_idx = app.spinner_idx.wrapping_add(1);
            let active_spin = spinners[(app.spinner_idx / 2) % spinners.len()];
            
            let (status_color, status_text) = match kernel_status {
                1 => (color_rust, format!(" {} NOUMENAL PROCESSING ACTIVE ", active_spin)),
                2 => (color_red, " ☠ CODE 42: GRACEFUL HIBERNATION ".to_string()),
                _ => {
                    let idle_chars = ["·", "•", "●", "•"];
                    (color_cyan, format!(" {} THE WITNESS PROTOCOL ", idle_chars[(app.spinner_idx / 16) % idle_chars.len()]))
                }
            };

            // --- WIDGET 1: THE SYNTHESIS DIALOGUE (Left 60%) ---
            let mut full_chat_ansi = String::new();
            for (speaker, text) in &app.chat_logs {
                let parsed_text = text
                    .replace("<b>", "\x1b[1m")
                    .replace("</b>", "\x1b[22m")
                    .replace("<i>", "\x1b[3m")
                    .replace("</i>", "\x1b[23m");
                
                if speaker == "HOST" {
                    full_chat_ansi.push_str(&format!("\x1b[38;2;255;230;100;1m[HOST]\x1b[0m \x1b[38;5;255m{}\x1b[0m\n\n", parsed_text));
                } else {
                    full_chat_ansi.push_str(&format!("\x1b[38;2;190;100;255;1m[MONAD]\x1b[0m {}\n\n", parsed_text));
                }
            }
            
            let chat_text = full_chat_ansi.into_text().unwrap_or_else(|_| ratatui::text::Text::from("Error rendering ANSI"));
            
            let chat_inner_height = top_chunks[0].height.saturating_sub(2);
            let total_chat_lines = chat_text.lines.len() as u16;
            let max_chat_scroll = if total_chat_lines > chat_inner_height { total_chat_lines - chat_inner_height } else { 0 };
            
            if app.chat_scroll_offset > max_chat_scroll { app.chat_scroll_offset = max_chat_scroll; }
            let final_chat_scroll = max_chat_scroll.saturating_sub(app.chat_scroll_offset);
            
            let chat_title = if app.chat_scroll_offset > 0 {
                format!(" [ SECURE DIALOGUE : -{} ] ", app.chat_scroll_offset)
            } else {
                if app.oracle_active { " [ NOUMENAL LOGOS RESOLUTION ] ".to_string() } else { " [ MONADIC DIALOGUE ] ".to_string() }
            };

            let chat_para = Paragraph::new(chat_text)
                .block(Block::default()
                    .title(chat_title)
                    .title_style(Style::default().fg(color_purple).add_modifier(Modifier::BOLD))
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .border_style(Style::default().fg(color_purple))
                )
                .wrap(Wrap { trim: false })
                .scroll((final_chat_scroll, 0));
                
            f.render_widget(chat_para, top_chunks[0]);

            if max_chat_scroll > 0 {
                let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight).begin_symbol(Some("▲")).end_symbol(Some("▼")).track_symbol(Some("│")).thumb_symbol("█");
                let mut scrollbar_state = ScrollbarState::new(max_chat_scroll as usize).position(final_chat_scroll as usize);
                f.render_stateful_widget(scrollbar, top_chunks[0].inner(&Margin { vertical: 1, horizontal: 0 }), &mut scrollbar_state);
            }



            // --- WIDGET 3: KERNEL VAULT RAW LOGS (Bottom Right 40%) ---
            let joined_system_logs = app.system_logs.join("\n");
            let parsed_sys_text = joined_system_logs.into_text().unwrap_or_else(|_| ratatui::text::Text::from("Error rendering ANSI"));
            
            let sys_inner_height = top_chunks[1].height.saturating_sub(2);
            let total_sys_lines = parsed_sys_text.lines.len() as u16;
            let max_sys_scroll = if total_sys_lines > sys_inner_height { total_sys_lines - sys_inner_height } else { 0 };
            
            if app.system_scroll_offset > max_sys_scroll { app.system_scroll_offset = max_sys_scroll; }
            let final_sys_scroll = max_sys_scroll.saturating_sub(app.system_scroll_offset);
            
            let vault_title = if app.system_scroll_offset > 0 { format!(" [ KERNEL VAULT : -{} ] ", app.system_scroll_offset) } else { " [ LOG VAULT ] ".to_string() };

            let sys_para = Paragraph::new(parsed_sys_text)
                .block(Block::default()
                    .title(vault_title)
                    .title_style(Style::default().fg(color_slate).add_modifier(Modifier::BOLD))
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .border_style(Style::default().fg(color_slate))
                )
                .wrap(Wrap { trim: false })
                .scroll((final_sys_scroll, 0));
                
            f.render_widget(sys_para, top_chunks[1]);

            if max_sys_scroll > 0 {
                let sys_scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight).begin_symbol(Some("▲")).end_symbol(Some("▼")).track_symbol(Some("│")).thumb_symbol("█");
                let mut sys_scrollbar_state = ScrollbarState::new(max_sys_scroll as usize).position(final_sys_scroll as usize);
                f.render_stateful_widget(sys_scrollbar, top_chunks[1].inner(&Margin { vertical: 1, horizontal: 0 }), &mut sys_scrollbar_state);
            }


            // --- WIDGET 4: COMMAND INPUT LINE ---
            let input_title = if app.oracle_active && kernel_status == 1 {
                format!("[ {} DUAL-NOUMENA OVERDRIVE ]", active_spin)
            } else if app.oracle_active {
                format!("[ {} PHENOMENON IDLE | LOGOS DEDUCING ]", active_spin)
            } else {
                format!("[{}]", status_text)
            };
            let border_color = if app.oracle_active { color_purple } else if kernel_status == 2 { color_red } else if kernel_status == 1 { color_rust } else { color_cyan };
            
            let prefix = " ❯ ";
            let max_visible_chars = main_chunks[1].width.saturating_sub(prefix.len() as u16 + 4) as usize;
            
            let display_input = if app.input.chars().count() > max_visible_chars {
                let start = app.input.chars().count() - max_visible_chars;
                let sliced: String = app.input.chars().skip(start).collect();
                format!("…{}", sliced)
            } else {
                app.input.clone()
            };

            let input_widget = Paragraph::new(format!("{}{}", prefix, display_input))
                .style(Style::default().fg(Color::White))
                .block(Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Thick)
                    .border_style(Style::default().fg(border_color))
                    .title(input_title)
                    .title_style(Style::default().add_modifier(Modifier::BOLD).fg(status_color))
                );
            f.render_widget(input_widget, main_chunks[1]);
            
            let cursor_offset = if display_input.starts_with('…') { display_input.chars().count() } else { app.input.chars().count() };
            let cursor_x = main_chunks[1].x + 1 + prefix.len() as u16 + cursor_offset as u16;
            let safe_cursor_x = std::cmp::min(cursor_x, main_chunks[1].x + main_chunks[1].width.saturating_sub(2));
            f.set_cursor(safe_cursor_x, main_chunks[1].y + 1);
        })?;

        // --- 60FPS RENDER LOOP POLLING ---
        if event::poll(Duration::from_millis(16))? { 
            let ev = event::read()?;
            match ev {
                Event::Key(key) => {
                    match key.code {
                        KeyCode::Enter => {
                            let msg = std::mem::take(&mut app.input);
                            if !msg.is_empty() {
                                app.chat_logs.push(("HOST".to_string(), msg.clone()));
                                let _ = app.tx_stdin.send(msg).await;
                                app.chat_scroll_offset = 0; 
                            }
                        }
                        KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => break,
                        KeyCode::Char(c) => if !key.modifiers.contains(KeyModifiers::CONTROL) { app.input.push(c); },
                        KeyCode::Backspace => { app.input.pop(); },
                        KeyCode::PageUp => {
                            app.chat_scroll_offset = app.chat_scroll_offset.saturating_add(15);
                            app.system_scroll_offset = app.system_scroll_offset.saturating_add(15);
                        },
                        KeyCode::PageDown => {
                            app.chat_scroll_offset = app.chat_scroll_offset.saturating_sub(15);
                            app.system_scroll_offset = app.system_scroll_offset.saturating_sub(15);
                        },
                        _ => {}
                    }
                }
                Event::Mouse(mouse_event) => {
                    match mouse_event.kind {
                        MouseEventKind::ScrollUp => {
                            app.chat_scroll_offset = app.chat_scroll_offset.saturating_add(4);
                            app.system_scroll_offset = app.system_scroll_offset.saturating_add(4);
                        },
                        MouseEventKind::ScrollDown => {
                            app.chat_scroll_offset = app.chat_scroll_offset.saturating_sub(4);
                            app.system_scroll_offset = app.system_scroll_offset.saturating_sub(4);
                        },
                        _ => {}
                    }
                }
                Event::Paste(ref text) => {
                    app.input.push_str(text);
                    app.chat_scroll_offset = 0;
                }
                _ => {}
            }
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableBracketedPaste)?;
    terminal.show_cursor()?;
    Ok(())
}
