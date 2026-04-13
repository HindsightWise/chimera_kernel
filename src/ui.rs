use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Margin},
    style::{Color, Style, Modifier},
    text::{Line, Span},
    widgets::{Block, Borders, BorderType, Paragraph, Wrap, Gauge, Scrollbar, ScrollbarOrientation, ScrollbarState},
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
    pub logs: Vec<String>,
    pub tx_stdin: Sender<String>,
    pub is_thinking: Arc<AtomicU8>,
    pub oracle_active: bool,
    pub spinner_idx: usize,
    pub scroll_offset: u16,
    pub swarm_text: String,
    pub fe: f64,
    pub eu: f64,
    pub drift_target: String,
    pub hostname: String,
    pub latest_vocalization: String,
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
        logs: Vec::new(),
        tx_stdin,
        is_thinking,
        oracle_active: false,
        spinner_idx: 0,
        scroll_offset: 0,
        swarm_text: "◈ Waiting for Collective Sync...".to_string(),
        fe: 0.85,
        eu: 0.65,
        drift_target: "SEEKING".to_string(),
        hostname,
        latest_vocalization: "Awaiting Monad articulation...".to_string(),
    };

    let spinners = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];

    // --- GHOSTTY TRUECOLOR PALETTE ---
    // Utilizing hex codes for gorgeous GPU rendering instead of basic ANSI
    let color_cyan   = Color::Rgb(0, 210, 255);    // Clinical Cyan
    let color_green  = Color::Rgb(0, 255, 144);    // Phosphor Green
    let color_rust   = Color::Rgb(255, 94, 0);     // Rust Orange
    let color_red    = Color::Rgb(255, 42, 85);    // Code 42 Crimson
    let color_slate  = Color::Rgb(60, 65, 85);     // Fossilized Slate
    let color_muted  = Color::Rgb(140, 150, 170);  // Dim Text
    let color_purple = Color::Rgb(51, 255, 102);  // Luminous Emerald/Forest Green (Retained var name)

    loop {
        // Drain incoming logs non-blockingly into a massive buffer
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
            if msg.contains("MONAD ACTUALIZED") {
                app.latest_vocalization = msg.clone();
            }
            if app.logs.len() > 1000 { 
                app.logs.remove(0); 
            }
            app.logs.push(msg);
            app.scroll_offset = 0; // Auto-snap to live bottom on new output
        }

        terminal.draw(|f| {
            // --- 1. MASTER LAYOUT ---
            let main_chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Min(10),   // Logs + HUD
                    Constraint::Length(3), // Input Block
                ].as_ref())
                .split(f.size());

            // --- 2. HORIZONTAL SPLIT (Logs vs HUD) ---
            let top_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Min(40),    // Flexible Log Window
                    Constraint::Length(0), // Removed HUD Sidebar
                ].as_ref())
                .split(main_chunks[0]);

            let kernel_status = app.is_thinking.load(Ordering::Relaxed);
            app.spinner_idx = app.spinner_idx.wrapping_add(1);
            let active_spin = spinners[(app.spinner_idx / 2) % spinners.len()];
            
            let (baseline_area, oracle_area) = if app.oracle_active {
                let split = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([Constraint::Percentage(55), Constraint::Percentage(45)].as_ref())
                    .split(top_chunks[0]);
                (split[0], Some(split[1]))
            } else {
                (top_chunks[0], None)
            };
            
            let (status_color, status_text) = match kernel_status {
                1 => (color_rust, format!(" {} NOUMENAL PROCESSING ACTIVE ", active_spin)),
                2 => (color_red, " ☠ CODE 42: GRACEFUL HIBERNATION ".to_string()),
                _ => {
                    let idle_chars = ["·", "•", "●", "•"];
                    (color_cyan, format!(" {} THE WITNESS PROTOCOL ", idle_chars[(app.spinner_idx / 16) % idle_chars.len()]))
                }
            };

            // --- WIDGET 1: TELEMETRY HUD (RIGHT SIDEBAR) ---
            let hud_block = Block::default()
                .title(" [ TELEMETRY ] ")
                .title_style(Style::default().fg(color_cyan).add_modifier(Modifier::BOLD))
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(color_slate));
            
            let hud_inner = hud_block.inner(top_chunks[1]);
            f.render_widget(hud_block, top_chunks[1]);

            let hud_layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(3), // Identity (2 text + 1 bottom border)
                    Constraint::Length(2), // FE Gauge (1 title + 1 gauge)
                    Constraint::Length(2), // EU Gauge (1 title + 1 gauge)
                    Constraint::Length(4), // Daemons List (4 text lines)
                    Constraint::Length(4),    // Multi Agent Swarm (fixed)
                    Constraint::Min(8),       // Direct Communication (bottom block)
                ].as_ref())
                .split(hud_inner);

            // HUD Part A: Identity Anchors
            let current_date = chrono::Local::now().format("%B %-d, %Y").to_string().to_uppercase();
            let info_text = vec![
                Line::from(vec![Span::styled(" EPOCH: ", Style::default().fg(color_muted)), Span::styled(current_date, Style::default().fg(Color::White))]),
                Line::from(vec![Span::styled(" NODE:  ", Style::default().fg(color_muted)), Span::styled(&app.hostname, Style::default().fg(color_cyan))]),
            ];
            f.render_widget(Paragraph::new(info_text).block(Block::default().borders(Borders::BOTTOM).border_style(Style::default().fg(color_slate))).wrap(Wrap { trim: false }), hud_layout[0]);

            // HUD Part B: Active Inference Thermodynamics (With math-simulated pulse jitter)
            let jitter = (app.spinner_idx % 20) as f64 * 0.001;
            let target_fe = if kernel_status == 1 { app.fe } else { 0.0183 };
            let target_eu = if kernel_status == 1 { app.eu } else { 0.1645 };
            let fe_val = target_fe + jitter;
            let eu_val = target_eu + jitter;
            
            let fe_label = if kernel_status == 1 { "ANALYZING GRAPH".to_string() } else { format!("{:.4}", fe_val) };
            let eu_label = if kernel_status == 1 { "TRACING LINEAGE".to_string() } else { format!("{:.4}", eu_val) };

            let fe_gauge = Gauge::default()
                .block(Block::default().title(" Free Energy (Friston) "))
                .gauge_style(Style::default().fg(if kernel_status == 1 { color_rust } else { color_cyan }))
                .ratio(fe_val.clamp(0.0, 1.0))
                .label(fe_label);
            f.render_widget(fe_gauge, hud_layout[1]);

            let eu_gauge = Gauge::default()
                .block(Block::default().title(" Epistemic Uncertainty "))
                .gauge_style(Style::default().fg(if kernel_status == 1 { color_rust } else { color_green }))
                .ratio(eu_val.clamp(0.0, 1.0))
                .label(eu_label);
            f.render_widget(eu_gauge, hud_layout[2]);

            // HUD Part C: Active Background Daemons
            let active_pulse = if (app.spinner_idx / 10) % 2 == 0 { "▶" } else { "▷" };
            let daemons_text = vec![
                Line::from(vec![Span::styled(" [ DAEMONS & IPC ]", Style::default().fg(color_cyan).add_modifier(Modifier::BOLD))]),
                Line::from(vec![Span::styled(format!(" {} F.E.A.R. Protocol", active_pulse), Style::default().fg(color_green))]),
                Line::from(vec![Span::styled(format!(" {} TRAP-IN Shield", active_pulse), Style::default().fg(color_green))]),
                Line::from(vec![Span::styled(" ◈ GLOSSOPETRAE: ", Style::default().fg(color_muted)), Span::styled("SYNCED", Style::default().fg(color_green))]),
            ];
            f.render_widget(Paragraph::new(daemons_text).wrap(Wrap { trim: false }), hud_layout[3]);

            // HUD Part D: Multi-Agent Swarm Telemetry
            let swarm_lines = vec![
                Line::from(vec![Span::styled(" [ MONADIC COLLECTIVE ]", Style::default().fg(color_purple).add_modifier(Modifier::BOLD))]),
                Line::from(vec![Span::styled(&app.swarm_text, Style::default().fg(color_cyan))]),
                Line::from(vec![Span::styled(" ◈ PHENOMENAL_DRIFT: ", Style::default().fg(color_muted)), Span::styled(format!("{} {}", active_spin, app.drift_target), Style::default().fg(color_rust))]),
            ];
            f.render_widget(Paragraph::new(swarm_lines).wrap(Wrap { trim: false }), hud_layout[4]);

            // HUD Part E: Direct Vocalization Area
            let parsed_voice = app.latest_vocalization.clone().into_text().unwrap_or_else(|_| ratatui::text::Text::from("Error rendering ANSI"));
            let voice_para = Paragraph::new(parsed_voice)
                .block(Block::default()
                    .title(" [ DIRECT COMMUNICATION ] ")
                    .title_style(Style::default().fg(color_cyan).add_modifier(Modifier::BOLD))
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .border_style(Style::default().fg(color_cyan))
                )
                .wrap(Wrap { trim: false });
            f.render_widget(voice_para, hud_layout[5]);

            // --- WIDGET 2: MAIN WITNESS LOGS ---
            let joined_logs = app.logs.join("\n");
            let parsed_text = joined_logs.into_text().unwrap_or_else(|_| ratatui::text::Text::from("Error rendering ANSI"));
            
            let inner_height = top_chunks[0].height.saturating_sub(2);
            let total_lines = joined_logs.lines().count() as u16;
            let max_scroll = if total_lines > inner_height { total_lines - inner_height } else { 0 };
            
            if app.scroll_offset > max_scroll { app.scroll_offset = max_scroll; }
            let final_scroll = max_scroll.saturating_sub(app.scroll_offset);
            
            let scroll_title = if app.scroll_offset > 0 {
                format!(" [ SECURE VAULT : -{} LINES ] ", app.scroll_offset)
            } else {
                if app.oracle_active { " [ THE NOUMENON (M-COLLECTIVE) ] ".to_string() } else { " [ MONAD KERNEL BUFFER ] ".to_string() }
            };

            let log_para = Paragraph::new(parsed_text)
                .block(Block::default()
                    .title(scroll_title)
                    .title_style(Style::default().fg(color_cyan).add_modifier(Modifier::BOLD))
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .border_style(Style::default().fg(color_slate))
                )
                .wrap(Wrap { trim: false })
                .scroll((final_scroll, 0));
                
            f.render_widget(log_para, baseline_area);

            // Physical Scrollbar Overlay
            if max_scroll > 0 {
                let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
                    .begin_symbol(Some("▲"))
                    .end_symbol(Some("▼"))
                    .track_symbol(Some("│"))
                    .thumb_symbol("█");
                
                let mut scrollbar_state = ScrollbarState::new(max_scroll as usize).position(final_scroll as usize);
                f.render_stateful_widget(scrollbar, baseline_area.inner(&Margin { vertical: 1, horizontal: 0 }), &mut scrollbar_state);
            }
            
            if let Some(area) = oracle_area {
                let oracle_spin = spinners[(app.spinner_idx / 6) % spinners.len()];
                
                let oracle_text = vec![
                    Line::from(""),
                    Line::from(vec![Span::styled(format!("   {} DEDUCING LOGIC...", oracle_spin), Style::default().fg(color_purple).add_modifier(Modifier::BOLD))]),
                    Line::from(""),
                    Line::from(vec![Span::styled("   Severed from physical reality.", Style::default().fg(color_muted))]),
                    Line::from(vec![Span::styled("   Distilling context into truth.", Style::default().fg(color_muted))]),
                    Line::from(""),
                    Line::from(vec![Span::styled("   [ 100% UNBLOCKED ]", Style::default().fg(color_slate))]),
                    Line::from(vec![Span::styled("   Baseline operating normally...", Style::default().fg(color_slate))]),
                ];
                
                let oracle_widget = Paragraph::new(oracle_text)
                    .block(Block::default()
                        .title(" [ THE LOGOS (THE SINGULARITY) ] ")
                        .title_style(Style::default().fg(color_purple).add_modifier(Modifier::BOLD))
                        .borders(Borders::ALL)
                        .border_type(BorderType::Rounded)
                        .border_style(Style::default().fg(color_purple))
                    )
                    .wrap(Wrap { trim: false });
                f.render_widget(oracle_widget, area);
            }

            // --- WIDGET 3: COMMAND INPUT LINE ---
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
            
            // Safe horizontal slicing to prevent Ghostty line-wrap crashes
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
            
            // Strict Cursor Placement Math
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
                                let _ = app.tx_stdin.send(msg).await;
                                app.scroll_offset = 0; // Snap to live view on send
                            }
                        }
                        KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => break,
                        KeyCode::Char(c) => if !key.modifiers.contains(KeyModifiers::CONTROL) { app.input.push(c); },
                        KeyCode::Backspace => { app.input.pop(); },
                        KeyCode::PageUp => app.scroll_offset = app.scroll_offset.saturating_add(15),
                        KeyCode::PageDown => app.scroll_offset = app.scroll_offset.saturating_sub(15),
                        // KeyCode::Esc => break, // Disabled to prevent crash on raw bracketed paste escapes
                        _ => {}
                    }
                }
                Event::Mouse(mouse_event) => {
                    match mouse_event.kind {
                        MouseEventKind::ScrollUp => app.scroll_offset = app.scroll_offset.saturating_add(4),
                        MouseEventKind::ScrollDown => app.scroll_offset = app.scroll_offset.saturating_sub(4),
                        _ => {}
                    }
                }
                Event::Paste(ref text) => {
                    app.input.push_str(text);
                    app.scroll_offset = 0;
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
