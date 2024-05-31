use aes_gcm::{Aes256Gcm, aead::{Aead, NewAead, generic_array::GenericArray}};
use rand::{rngs::OsRng, RngCore};
use base64::{encode};

const ENCRYPTION_KEY_LEN: usize = 32;
const NONCE_LEN: usize = 12; // AES GCM standard nonce length

pub fn encrypt(data: &[u8], key: &[u8]) -> Result<String, &'static str> {
    validate_key_length(key)?;
    let nonce = generate_nonce().map_err(|_| "Failed to generate nonce")?;
    let ciphertext = encrypt_data(data, key, &nonce)?;
    let base64_encoded = base64_encode(&nonce, &ciphertext);
    Ok(base64_encoded)
}

fn validate_key_length(key: &[u8]) -> Result<(), &'static str> {
    if key.len() != ENCRYPTION_KEY_LEN {
        return Err("Invalid key length");
    }
    Ok(())
}

fn generate_nonce() -> Result<[u8; NONCE_LEN], ()> {
    let mut nonce = [0u8; NONCE_LEN];
    OsRng.fill_bytes(&mut nonce);
    Ok(nonce)
}

fn encrypt_data(data: &[u8], key: &[u8], nonce: &[u8; NONCE_LEN]) -> Result<Vec<u8>, &'static str> {
    let cipher = Aes256Gcm::new(GenericArray::from_slice(key));
    let nonce = GenericArray::from_slice(nonce);
    cipher.encrypt(nonce, data.as_ref())
        .map_err(|_| "Encryption failed")
}

fn base64_encode(nonce: &[u8; NONCE_LEN], ciphertext: &[u8]) -> String {
    let mut nonce_and_ciphertext = Vec::with_capacity(NONCE_LEN + ciphertext.len());
    nonce_and_ciphertext.extend_from_slice(nonce);
    nonce_and_ciphertext.extend_from_slice(ciphertext);
    encode(nonce_and_ciphertext)
}