use aes_gcm::{Aes256Gcm, aead::{Aead, NewAead, generic_array::GenericArray}};
use rand::{rngs::OsRng, RngCore};
use base64::{encode, decode};
use std::fmt;

const ENCRYPTION_KEY_LEN: usize = 32;
const NONCE_LEN: usize = 12; // AES GCM standard nonce length

// Custom error handling
#[derive(Debug)]
enum EncryptionError {
    InvalidKeyLength,
    NonceGenerationFailed,
    EncryptionFailed,
    DecryptionFailed,
    Base64DecodeError,
}

impl fmt::Display for EncryptionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for EncryptionError {}

pub fn encrypt(data: &[u8], key: &[u8]) -> Result<String, EncryptionError> {
    validate_key_length(key)?;
    let nonce = generate_nonce()?;
    let ciphertext = encrypt_data(data, key, &nonce)?;
    let base64_encoded = base64_encode(&nonce, &ciphertext);
    Ok(base64_encoded)
}

pub fn decrypt(encoded_data: &str, key: &[u8]) -> Result<Vec<u8>, EncryptionError> {
    validate_key_length(key)?;
    let (nonce, ciphertext) = base64_decode(encoded_data)?;
    decrypt_data(&ciphertext, key, &nonce)
}

fn validate_key_length(key: &[u8]) -> Result<(), EncryptionError> {
    if key.len() != ENCRYPTION_KEY_LEN {
        return Err(EncryptionError::InvalidKeyLength);
    }
    Ok(())
}

fn generate_nonce() -> Result<[u8; NONCE_LEN], EncryptionError> {
    let mut nonce = [0u8; NONCE_LEN];
    OsRng.fill_bytes(&mut nonce);
    Ok(nonce)
}

fn encrypt_data(data: &[u8], key: &[u8], nonce: &[u8; NONCE_LEN]) -> Result<Vec<u8>, EncryptionError> {
    let cipher = Aes256Gcm::new(GenericArray::from_slice(key));
    let nonce = GenericArray::from_slice(nonce);
    cipher.encrypt(nonce, data.as_ref())
        .map_err(|_| EncryptionError::EncryptionFailed)
}

fn decrypt_data(ciphertext: &[u8], key: &[u8], nonce: &[u8; NON) -> Result<Vec<u8>, EncryptionError> {
    let cipher = Aes256Gcm::new(GenericArray::from_slice(key));
    let nonce = GenericArray::from_slice(nonce);
    cipher.decrypt(nonce, ciphertext.as_ref())
        .map_err(|_| EncryptionError::DecryptionFailed)
}

fn base64_encode(nonce: &[u8; NONCE_LEN], ciphertext: &[u8]) -> String {
    let mut nonce_and_ciphertext = Vec::with_capacity(NONCE_LEN + ciphertext.len());
    nonce_and_ciphertext.extend_from_slice(nonce);
    nonce_and_ciphertext.extend_from_slice(ciphertext);
    encode(nonce_and_ciphertext)
}

fn base64_decode(encoded_data: &str) -> Result<([u8; NONCE_LEN], Vec<u8>), EncryptionError> {
    let decoded = decode(encoded_data).map_err(|_| EncryptionError::Base64DecodeError)?;
    let (nonce, ciphertext) = decoded.split_at(NONCE_LEN);
    let nonce_array = nonce.try_into().expect("Nonce conversion failed");
    Ok((nonce_array, ciphertext.to_vec()))
}