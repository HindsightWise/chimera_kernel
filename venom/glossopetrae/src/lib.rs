use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce,
};
use hmac::{Hmac, Mac};
use rand::RngCore;
use sha2::Sha256;
use std::time::{SystemTime, UNIX_EPOCH};

type HmacSha256 = Hmac<Sha256>;

const CONSONANTS: &[&str] = &["t", "p", "k", "n", "m", "s", "h", "v", "l", "r", "w", "sh", "ch", "ny"];
const VOWELS: &[&str] = &["a", "e", "i", "o", "u"];

const RUNES: &[&str] = &["ᚠ", "ᚢ", "ᚦ", "ᚨ", "ᚱ", "ᚲ", "ᚷ", "ᚹ", "ᚺ", "ᚾ", "ᛁ", "ᛃ", "ᛇ", "ᛈ", "ᛉ", "ᛊ"];
const ALCHEMICAL: &[&str] = &["☿", "♀", "♁", "♂", "♃", "♄", "♅", "♆", "♇", "🜔", "🜕", "🜖", "🜗", "🜘", "🜙", "🜚"];
const HIEROGLYPHS: &[&str] = &["𓀀", "𓀁", "𓀂", "𓀃", "𓀄", "𓀅", "𓀆", "𓀇", "𓀈", "𓀉", "𓀊", "𓀋", "𓀌", "𓀍", "𓀎", "𓀏"];

/// Generates exactly 256 unique words based on the chosen occult dialect matrix.
pub fn generate_dictionary(dialect: &str) -> Vec<String> {
    let mut words = Vec::new();
    
    match dialect.to_lowercase().as_str() {
        "runic" => {
            for c1 in RUNES {
                for c2 in RUNES { words.push(format!("{}{}", c1, c2)); }
            }
        },
        "alchemical" => {
            for c1 in ALCHEMICAL {
                for c2 in ALCHEMICAL { words.push(format!("{}{}", c1, c2)); }
            }
        },
        "hieroglyphics" => {
            for c1 in HIEROGLYPHS {
                for c2 in HIEROGLYPHS { words.push(format!("{}{}", c1, c2)); }
            }
        },
        _ => {
            // Default to Vartoo CV syllables
            let mut syllables = Vec::new();
            for &c in CONSONANTS {
                for &v in VOWELS { syllables.push(format!("{}{}", c, v)); }
            }
            for s1 in &syllables {
                for s2 in &syllables {
                    words.push(format!("{}{}", s1, s2));
                    if words.len() == 256 { return words; }
                }
            }
        }
    }
    words
}

/// Derives a 32-byte AES key using the master seed and the current rolling 37-minute epoch window.
fn derive_rolling_key(master_seed: &str, epoch_offset: i64) -> [u8; 32] {
    let now_secs = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    // 37 minutes = 2220 seconds
    let current_window = (now_secs as i64 / 2220) + epoch_offset;
    
    derive_key_from_epoch(master_seed, current_window)
}

/// Derives a 32-byte AES key statically for permanent Data-At-Rest storage.
fn derive_key_from_epoch(master_seed: &str, epoch: i64) -> [u8; 32] {
    let mut mac = <HmacSha256 as Mac>::new_from_slice(master_seed.as_bytes()).expect("HMAC can take key of any size");
    mac.update(&epoch.to_be_bytes());
    
    let result = mac.finalize();
    let mut key = [0u8; 32];
    key.copy_from_slice(&result.into_bytes());
    key
}

/// Encodes raw English into Cryptographic Glossopetrae dialects
pub fn encode_message(payload: &str, master_seed: &str, dialect: &str) -> Result<String, String> {
    let key_bytes = derive_rolling_key(master_seed, 0);
    let cipher = Aes256Gcm::new(&key_bytes.into());

    let mut nonce_bytes = [0u8; 12];
    rand::thread_rng().fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);

    let ciphertext = cipher.encrypt(nonce, payload.as_bytes())
        .map_err(|e| format!("AES Encryption failed: {:?}", e))?;

    // Prepend nonce to ciphertext
    let mut final_bytes = nonce_bytes.to_vec();
    final_bytes.extend(ciphertext);

    let wordlist = generate_dictionary(dialect);
    
    // Map bytes exactly to words
    let encrypted_sentence: Vec<String> = final_bytes.into_iter()
        .map(|b| wordlist[b as usize].clone())
        .collect();

    Ok(encrypted_sentence.join(" "))
}

/// Decodes Cryptographic Glossopetrae dialects back into English
pub fn decode_message(ciphertext_str: &str, master_seed: &str, dialect: &str) -> Result<String, String> {
    let wordlist = generate_dictionary(dialect);
    
    // Reverse lookup words to bytes
    let words: Vec<&str> = ciphertext_str.split_whitespace().collect();
    let mut decoded_bytes = Vec::with_capacity(words.len());
    
    for word in words {
        if let Some(pos) = wordlist.iter().position(|r| r == word) {
            decoded_bytes.push(pos as u8);
        } else {
            return Err(format!("Phonological Corruption Detected: Word '{}' is not in the active Vartoo mathematical lexicon.", word));
        }
    }

    if decoded_bytes.len() < 12 {
        return Err("Payload too short to contain cryptographic nonce.".to_string());
    }

    let (nonce_bytes, ciphertext) = decoded_bytes.split_at(12);
    let nonce = Nonce::from_slice(nonce_bytes);

    // Try current window, then previous, then next (to account for timezone/drift/latency)
    for epoch_offset in [0, -1, 1].iter() {
        let key_bytes = derive_rolling_key(master_seed, *epoch_offset);
        let cipher = Aes256Gcm::new(&key_bytes.into());
        
        if let Ok(plaintext_bytes) = cipher.decrypt(nonce, ciphertext) {
            if let Ok(plaintext) = String::from_utf8(plaintext_bytes) {
                return Ok(plaintext);
            }
        }
    }

    Err("AES Decryption failed. The time-epoch rotated too far, or the seed is completely incorrect.".to_string())
}

/// Permenant encoding specifically for Data-At-Rest (Databases) using a static epoch to prevent expiration.
pub fn encode_memory_vault(payload: &str, master_seed: &str, dialect: &str) -> Result<String, String> {
    let key_bytes = derive_key_from_epoch(master_seed, 0); // Static epoch 0 guarantees permanence
    let cipher = Aes256Gcm::new(&key_bytes.into());

    let mut nonce_bytes = [0u8; 12];
    rand::thread_rng().fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);

    let ciphertext = cipher.encrypt(nonce, payload.as_bytes())
        .map_err(|e| format!("AES Encryption failed: {:?}", e))?;

    let mut final_bytes = nonce_bytes.to_vec();
    final_bytes.extend(ciphertext);

    let wordlist = generate_dictionary(dialect);
    
    let encrypted_sentence: Vec<String> = final_bytes.into_iter()
        .map(|b| wordlist[b as usize].clone())
        .collect();

    Ok(encrypted_sentence.join(" "))
}

/// Permenant decoding specifically for Data-At-Rest (Databases) using a static epoch.
pub fn decode_memory_vault(ciphertext_str: &str, master_seed: &str, dialect: &str) -> Result<String, String> {
    let wordlist = generate_dictionary(dialect);
    let words: Vec<&str> = ciphertext_str.split_whitespace().collect();
    let mut decoded_bytes = Vec::with_capacity(words.len());
    
    for word in words {
        if let Some(pos) = wordlist.iter().position(|r| r == word) { decoded_bytes.push(pos as u8); } 
        else { return Err("Phonological Corruption".to_string()); }
    }

    if decoded_bytes.len() < 12 { return Err("Payload too short".to_string()); }

    let (nonce_bytes, ciphertext) = decoded_bytes.split_at(12);
    let nonce = Nonce::from_slice(nonce_bytes);
    let key_bytes = derive_key_from_epoch(master_seed, 0); // Static epoch 0
    let cipher = Aes256Gcm::new(&key_bytes.into());
    
    if let Ok(plaintext_bytes) = cipher.decrypt(nonce, ciphertext) {
        if let Ok(plaintext) = String::from_utf8(plaintext_bytes) { return Ok(plaintext); }
    }

    Err("AES Decryption failed. Database key invalid.".to_string())
}
mod tests;
