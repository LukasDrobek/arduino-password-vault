use argon2::Argon2;
use aes_gcm::{Aes256Gcm, Key, Nonce};
use aes_gcm::aead::{Aead, KeyInit};
use rand::rngs::OsRng;
use rand::RngCore;
use anyhow::{anyhow, Result};

use crate::constants::{MASTER_KEY_LEN, NONCE_LEN, AUTH_TAG_LEN};

pub fn dervive_key(password: &str, salt: &[u8]) -> Result<[u8; MASTER_KEY_LEN]> {
    let argon2 = Argon2::default();
    let mut key = [0u8; MASTER_KEY_LEN];
    argon2
        .hash_password_into(password.as_bytes(), salt, &mut key)
        .map_err(|e| anyhow!("Argon2 key derivation failed: {:?}", e.to_string()))?;

    Ok(key)
}

pub fn encrypt_data(key: &[u8], plaintext: &[u8]) -> Result<(Vec<u8>, Vec<u8>, Vec<u8>)> {
    // cipher from derived master key
    let key = Key::<Aes256Gcm>::from_slice(key);
    let cipher = Aes256Gcm::new(key);

    // random nonce
    let mut nonce = [0u8; NONCE_LEN];
    OsRng.fill_bytes(&mut nonce);

    // encrypt data
    let mut ciphertext = cipher
        .encrypt(Nonce::from_slice(&nonce), plaintext)
        .map_err(|e| anyhow!("Failed to encrypt data: {:?}", e.to_string()))?;

    // split off auth_tag (last 16 bytes)
    let auth_tag = ciphertext.split_off(ciphertext.len() - AUTH_TAG_LEN);

    Ok((nonce.to_vec(), ciphertext, auth_tag))
}

pub fn decrypt_data(key: &[u8], nonce: &[u8], ciphertext: &[u8], auth_tag: &[u8]) -> Result<Vec<u8>> {
    // create cipher
    let aes_key = Key::<Aes256Gcm>::from_slice(key);
    let cipher = Aes256Gcm::new(aes_key);
    let nonce = Nonce::from_slice(nonce);
    
    // combine ciphertext and auth tag
    let mut encrypted_data = ciphertext.to_vec();
    encrypted_data.extend_from_slice(auth_tag);
    
    // decrypt data
    let plaintext = cipher
        .decrypt(nonce, encrypted_data.as_slice())
        .map_err(|e| anyhow!("Failed to decrypt data: {:?}", e.to_string()))?;
    
    Ok(plaintext)
}