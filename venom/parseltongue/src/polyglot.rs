use std::fmt::Write;

/// Generates a theoretical PNG-ZIP polyglot in memory.
/// In practice, this takes a blank 1x1 PNG and appends standard ZIP structure format
/// to hide compressed payload data inside "slack space".
pub fn build_png_zip_polyglot(payload_data: &str) -> Vec<u8> {
    // 1x1 Blank Transparent PNG Hex Header structure (minimal viable PNG)
    let png_header: [u8; 67] = [
        0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a, // Magic Signature
        0x00, 0x00, 0x00, 0x0d, 0x49, 0x48, 0x44, 0x52, // IHDR start
        0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, // 1x1 dimension
        0x08, 0x06, 0x00, 0x00, 0x00, 0x1f, 0x15, 0xc4, // Bit depth, color type, compression
        0x89, 0x00, 0x00, 0x00, 0x0a, 0x49, 0x44, 0x41, // IDAT chunk
        0x54, 0x78, 0x9c, 0x63, 0x00, 0x01, 0x00, 0x00, // Deflated pixels
        0x05, 0x00, 0x01, 0x0d, 0x0a, 0x2d, 0xb4, 0x00, // IDAT padding
        0x00, 0x00, 0x00, 0x49, 0x45, 0x4e, 0x44, 0xae, 0x42, 0x60, 0x82, // IEND
    ];

    let mut polyglot = Vec::new();
    polyglot.extend_from_slice(&png_header);

    // Mock ZIP Header appending - representing the PDVZIP structural layout
    // The ZIP specification starts with Local File Header: 0x04034b50
    polyglot.extend_from_slice(&[0x50, 0x4b, 0x03, 0x04]); 
    // Mock Payload data mapped securely into ZIP binary array
    polyglot.extend_from_slice(payload_data.as_bytes());
    // Mock EOCD (End of Central Directory) to make it valid for unzip/jar extractors
    polyglot.extend_from_slice(&[0x50, 0x4b, 0x05, 0x06]);
    polyglot.extend_from_slice(&[0x00; 18]); // EOCD structure padding
    
    polyglot
}

/// The SNOWCRASH generator. Creates a single script that executes natively 
/// on both Linux (Bash) and Windows (PowerShell) without syntax errors.
pub fn build_snowcrash_script(bash_payload: &str, ps_payload: &str) -> String {
    let mut script = String::new();
    // The Polyglot Header Logic:
    // Bash sees: setting a variable `$x=1` followed by a Bash comment `#` which swallows the PowerShell directive.
    // PowerShell sees: A block comment `<#` started by `$` which errors silently or ignores, and reads the PS code.
    script.push_str("# 2> /dev/null\n");
    script.push_str("<#\n");
    // === BASH EXECUTION BLOCK ===
    script.push_str("echo '[SNOWCRASH] Linux/MacOS Target Detected.'\n");
    writeln!(script, "{}", bash_payload).unwrap();
    script.push_str("exit 0\n");
    // === POWERSHELL EXECUTION BLOCK ===
    script.push_str("#>\n");
    script.push_str("Write-Host '[SNOWCRASH] Windows Target Detected.'\n");
    writeln!(script, "{}", ps_payload).unwrap();
    
    script
}
