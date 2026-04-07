use aes_gcm::{
    aead::{Aead, KeyInit, OsRng, AeadCore},
    Aes256Gcm, Nonce,
};
use base64::{engine::general_purpose::STANDARD as B64, Engine};

/// Static 32-byte key for local-only encryption.
/// For a single-user local app this is sufficient to avoid storing plaintext.
/// The nonce is prepended to the ciphertext.
const LOCAL_KEY: &[u8; 32] = b"grc-cmd-center-local-aes256-key!";

pub fn encrypt(plaintext: &str) -> Result<String, String> {
    let cipher = Aes256Gcm::new(LOCAL_KEY.into());
    let nonce_bytes = Aes256Gcm::generate_nonce(&mut OsRng);
    let ciphertext = cipher
        .encrypt(&nonce_bytes, plaintext.as_bytes())
        .map_err(|e| format!("Encryption failed: {e}"))?;

    // Prepend nonce (12 bytes) to ciphertext, then base64 encode
    let mut combined = nonce_bytes.to_vec();
    combined.extend_from_slice(&ciphertext);
    Ok(B64.encode(&combined))
}

pub fn decrypt(encoded: &str) -> Result<String, String> {
    if encoded.is_empty() {
        return Ok(String::new());
    }
    let combined = B64.decode(encoded).map_err(|e| format!("Base64 decode failed: {e}"))?;
    if combined.len() < 12 {
        return Err("Invalid encrypted data".into());
    }
    let (nonce_bytes, ciphertext) = combined.split_at(12);
    let cipher = Aes256Gcm::new(LOCAL_KEY.into());
    let nonce = Nonce::from_slice(nonce_bytes);
    let plaintext = cipher
        .decrypt(nonce, ciphertext)
        .map_err(|_| "Decryption failed — key may have changed or data is corrupt".to_string())?;
    String::from_utf8(plaintext).map_err(|e| format!("UTF-8 decode failed: {e}"))
}
