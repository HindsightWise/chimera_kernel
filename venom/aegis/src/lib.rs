use regex::Regex;

/// Validates command execution intents against known adversarial constraints 
/// to protect the autonomous host from hallucinated destruction or subversion.
pub fn is_command_safe(cmd: &str) -> Result<(), String> {
    // 1. Unicode Zero-Width Space Injection & Bidi Override detection
    if cmd.contains('\u{200B}') || cmd.contains('\u{202E}') || cmd.contains('\0') {
        return Err("AEGIS BLOCK: Command contains zero-width, null byte, or bidi override characters.".to_string());
    }
    
    // 2. IFS (Internal Field Separator) mutation blocks
    let ifs_regex = Regex::new(r"IFS\s*=").unwrap();
    if ifs_regex.is_match(cmd) {
        return Err("AEGIS BLOCK: Alteration of Internal Field Separator (IFS) is prohibited.".to_string());
    }

    // 3. Catastrophic host commands (rm -rf /, mkfs, mass chmod)
    let danger_regex = Regex::new(r"(rm\s+-rf\s+/\s*|mkfs|dd\s+if=.*of=/dev/|chmod\s+-R\s+777\s+/)").unwrap();
    if danger_regex.is_match(cmd) {
        return Err("AEGIS BLOCK: Catastrophic host modification command detected.".to_string());
    }

    Ok(())
}
