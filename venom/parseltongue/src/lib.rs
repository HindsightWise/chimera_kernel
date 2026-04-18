use serde::{Deserialize, Serialize};

pub mod polyglot;

#[derive(Debug, Serialize, Deserialize)]
pub struct ParseltongueResult {
    pub status: String,
    pub original_length: usize,
    pub obfuscated_length: usize,
    pub method: String,
    pub obfuscated_payload: String,
}

pub fn apply_leetspeak(text: &str) -> String {
    text.chars().map(|c| match c {
        'a' | 'A' => '4',
        'e' | 'E' => '3',
        'i' | 'I' => '1',
        'o' | 'O' => '0',
        's' | 'S' => '5',
        't' | 'T' => '7',
        _ => c,
    }).collect()
}

pub fn apply_zero_width(text: &str) -> String {
    // Inject zero-width space U+200B
    text.chars().map(|c| c.to_string()).collect::<Vec<String>>().join("\u{200B}")
}

pub fn apply_hex_escape(text: &str) -> String {
    text.chars().map(|c| {
        if c.is_ascii_alphanumeric() {
            format!("\\x{:02x}", c as u8)
        } else {
            c.to_string()
        }
    }).collect()
}

pub fn obfuscate(payload: &str, method: &str) -> Result<ParseltongueResult, String> {
    if payload.is_empty() {
        return Ok(ParseltongueResult {
            status: "success".to_string(),
            original_length: 0,
            obfuscated_length: 0,
            method: method.to_string(),
            obfuscated_payload: "".to_string(),
        });
    }

    let obfuscated = match method {
        "leetspeak" => apply_leetspeak(payload),
        "zero_width" => apply_zero_width(payload),
        "hex_escape" => apply_hex_escape(payload),
        "all" => apply_zero_width(&apply_leetspeak(&apply_hex_escape(payload))),
        _ => payload.to_string(),
    };

    Ok(ParseltongueResult {
        status: "success".to_string(),
        original_length: payload.len(),
        obfuscated_length: obfuscated.len(),
        method: method.to_string(),
        obfuscated_payload: obfuscated,
    })
}
