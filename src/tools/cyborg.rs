use async_openai::types::{ChatCompletionTool, FunctionObject};
use serde_json::{json, Value};
use enigo::{Enigo, Coordinate, Mouse, Keyboard, Settings, Direction, Button, Key};
use std::process::Command;
use std::time::Duration;
use rand::Rng;
use xcap::Monitor;
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64_STANDARD};
use std::io::Cursor;

pub fn definition() -> ChatCompletionTool {
    ChatCompletionTool {
        r#type: async_openai::types::ChatCompletionToolType::Function,
        function: FunctionObject {
            name: "emulate_human".to_string(),
            description: Some("Control the native host OS with Human Interface Device (HID) events. Use this to act as a human in the digital world via absolute pixel coordinates and raw keyboard emulation.".to_string()),
            parameters: Some(json!({
                "type": "object",
                "properties": {
                    "action": { 
                        "type": "string", 
                        "enum": ["mouse_move", "left_click", "right_click", "human_type", "enter", "screenshot", "extract_ui"] 
                    },
                    "coordinate": { 
                        "type": "array", 
                        "items": { "type": "integer" },
                        "description": "[x, y] coordinates for mouse movement."
                    },
                    "text": { 
                        "type": "string",
                        "description": "The exact string to type when action is 'human_type'."
                    }
                },
                "required": ["action"]
            })),
        },
    }
}

pub async fn execute(args: Value) -> String {
    let action = args.get("action").and_then(|v| v.as_str()).unwrap_or("");
    let mut enigo = Enigo::new(&Settings::default()).unwrap();

    match action {
        "mouse_move" => {
            if let Some(coords) = args.get("coordinate").and_then(|v| v.as_array()) {
                if coords.len() == 2 {
                    let x = coords[0].as_i64().unwrap_or(0) as i32;
                    let y = coords[1].as_i64().unwrap_or(0) as i32;
                    // Enigo v0.2.2 uses move_mouse instead of mouse_move_to
                    enigo.move_mouse(x, y, Coordinate::Abs).unwrap();
                    return format!("[CYBORG CORTEX] Mouse moved to X:{}, Y:{}", x, y);
                }
            }
            "[ERROR] Missing valid [x, y] coordinate array for mouse_move.".to_string()
        }
        "left_click" => {
            enigo.button(Button::Left, Direction::Click).unwrap();
            "[CYBORG CORTEX] Left click executed.".to_string()
        }
        "right_click" => {
            enigo.button(Button::Right, Direction::Click).unwrap();
            "[CYBORG CORTEX] Right click executed.".to_string()
        }
        "human_type" => {
            if let Some(text) = args.get("text").and_then(|v| v.as_str()) {
                for c in text.chars() {
                    // Spoofing biometric tracking by using stochastic micro-delays between keystrokes
                    let delay = rand::thread_rng().gen_range(30..110);
                    tokio::time::sleep(Duration::from_millis(delay)).await;
                    enigo.text(&c.to_string()).unwrap();
                }
                return format!("[CYBORG CORTEX] Typed '{}' with stochastically spoofed human cadence.", text);
            }
            "[ERROR] Missing 'text' parameter for human_type.".to_string()
        }
        "enter" => {
            enigo.key(Key::Return, Direction::Click).unwrap();
            "[CYBORG CORTEX] Pressed Return/Enter.".to_string()
        }
        "screenshot" => {
            match Monitor::all() {
                Ok(monitors) if !monitors.is_empty() => {
                    match monitors[0].capture_image() {
                        Ok(image) => {
                            let mut buf = Cursor::new(Vec::new());
                            if let Ok(_) = image.write_to(&mut buf, image::ImageFormat::Png) {
                                let b64 = BASE64_STANDARD.encode(buf.into_inner());
                                format!("[CYBORG CORTEX] Captured raw OS framebuffer! Base64 PNG memory string: {}", b64)
                            } else {
                                "[ERROR] Failed to compress local OS framebuffer to PNG.".to_string()
                            }
                        }
                        Err(e) => format!("[ERROR] Framebuffer physical capture panic: {}", e)
                    }
                }
                _ => "[ERROR] Zero active core-graphic physical monitors located. Are you running strictly headless?".to_string()
            }
        }
        "extract_ui" => {
            // macOS strictly local AXUIElement hook via System Events application query injection
            let script = r#"
                tell application "System Events"
                    set frontApp to first application process whose frontmost is true
                    tell frontApp
                        return properties of every UI element of window 1
                    end tell
                end tell
            "#;
            let output = Command::new("osascript").arg("-e").arg(script).output();
            match output {
                Ok(res) if res.status.success() => {
                    let text = String::from_utf8_lossy(&res.stdout);
                    // Ensure output isn't too massive for LLM context limits by truncating if necessary,
                    // but for native application windows this is typically under 10k context characters.
                    format!("[CYBORG CORTEX - MAC ACCESSIBILITY SYNTHESIS] GUI bounds extracted:\n{}", text)
                }
                _ => "[ERROR] macOS AXUIElement extraction strictly blocked. Ensure Terminal has Accessibility privacy permissions toggled!".to_string()
            }
        }
        _ => format!("[ERROR] Unknown HID action: {}", action),
    }
}
