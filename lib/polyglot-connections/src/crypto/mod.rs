use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm, Key, Nonce,
};
use std::error::Error;

/// Encrypts a payload using AES-256-GCM.
/// 
/// The returned vector contains the randomly generated 12-byte nonce prepended
/// to the encrypted ciphertext, allowing the decryptor to extract it.
///
/// # Arguments
/// * `data` - The raw bytes of the serialized payload to encrypt.
/// * `key_bytes` - A 32-byte (256-bit) symmetric key.
pub fn encrypt_payload(data: &[u8], key_bytes: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
    if key_bytes.len() != 32 {
        return Err("Encryption key must be exactly 32 bytes".into());
    }

    let key = Key::<Aes256Gcm>::from_slice(key_bytes);
    let cipher = Aes256Gcm::new(key);
    
    // Generate a random 96-bit (12-byte) nonce
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
    
    // Encrypt the payload
    let ciphertext = cipher.encrypt(&nonce, data)
        .map_err(|e| format!("Encryption failed: {}", e))?;
    
    // Prepend the 12-byte nonce to the ciphertext
    let mut final_payload = Vec::with_capacity(nonce.len() + ciphertext.len());
    final_payload.extend_from_slice(nonce.as_slice());
    final_payload.extend_from_slice(&ciphertext);
    
    Ok(final_payload)
}

/// Decrypts an AES-256-GCM payload.
///
/// Expects the first 12 bytes of the input data to be the nonce.
///
/// # Arguments
/// * `encrypted_data` - The encrypted data with the prepended 12-byte nonce.
/// * `key_bytes` - The 32-byte symmetric key used for encryption.
pub fn decrypt_payload(encrypted_data: &[u8], key_bytes: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
    if key_bytes.len() != 32 {
        return Err("Decryption key must be exactly 32 bytes".into());
    }
    
    if encrypted_data.len() < 12 {
        return Err("Invalid encrypted payload: too short to contain nonce".into());
    }

    let key = Key::<Aes256Gcm>::from_slice(key_bytes);
    let cipher = Aes256Gcm::new(key);
    
    // Extract the 12-byte nonce
    let (nonce_bytes, ciphertext) = encrypted_data.split_at(12);
    let nonce = Nonce::from_slice(nonce_bytes);
    
    // Decrypt the payload
    let plaintext = cipher.decrypt(nonce, ciphertext)
        .map_err(|e| format!("Decryption failed: {}", e))?;
        
    Ok(plaintext)
}
