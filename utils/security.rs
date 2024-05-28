use std::env;
use ring::aead;
use ring::pbkdf2::*;
use ring::rand::{SecureRandom, SystemRandom};
use std::num::NonZeroU32;
use base64::{encode, decode};

static DIGEST_ALG: &'static digest::Algorithm = &digest::SHA256;
const CREDENTIAL_LEN: usize = digest::SHA256_OUTPUT_LEN;
const SALT_LEN: usize = CREDENTIAL_LEN;
const NONCE_LEN: usize = 96 / 8;
const ITERATIONS: u32 = 100_000;
const ENCRYPTION_KEY_LEN: usize = 32;

pub mod security_module {
    use super::*;

    pub fn generate_salt() -> Vec<u8> {
        let rng = SystemRandom::new();
        let mut salt = vec![0u8; SALT_LEN];
        rng.fill(&mut salt).unwrap();
        salt
    }

    pub fn hash_password(password: &str, salt: &[u8]) -> Vec<u8> {
        let mut pbkdf2_hash = vec![0u8; CREDENTIAL_LEN];
        pbkdf2::derive(DIGEST_ALG, NonZeroU32::new(ITERATIONS).unwrap(), salt, password.as_bytes(), &mut pbkdf2_hash);
        pbkdf2_hash
    }

    pub fn generate_encryption_key(password: &str, salt: &[u8]) -> Vec<u8> {
        let mut key = vec![0u8; ENCRYPTION_KEY_LEN];
        pbkdf2::derive(DIGEST_ALG, NonZeroU32::new(ITERATIONS).unwrap(), salt, password.as_bytes(), &mut key);
        key
    }

    pub fn encrypt(data: &[u8], key: &[u8]) -> Result<String, &'static str> {
        if key.len() != ENCRYPTION_KEY_LEN {
            return Err("Invalid key length");
        }
        let rng = SystemRandom::new();
        let mut nonce = vec![0u8; NONCE_LEN];
        rng.fill(&mut nonce).unwrap();
        let unbound_key = aead::UnboundKey::new(&aead::AES_256_GCM, key).map_err(|_| "Failed to create key")?;
        let sealing_key = aead::LessSafeKey::new(unbound_key);

        let mut in_out = data.to_vec();
        sealing_key.seal_in_place_append_tag(aead::Nonce::assume_unique_for_key(nonce.clone()), aead::Aad::empty(), &mut in_out)
            .map_err(|_| "Encryption failed")?;

        let mut nonce_and_ciphertext = nonce;
        nonce_and_ciphertext.extend_from_slice(&in_out);
        Ok(encode(nonce_and_ciphertext))
    }

    pub fn decrypt(encoded: &str, key: &[u8]) -> Result<Vec<u8>, &'static str> {
        if key.len() != ENCRYPTION_KEY_LEN {
            return Err("Invalid key length");
        }
        let mut nonce_and_ciphertext = decode(encoded).map_err(|_| "Decoding failed")?;
        
        if nonce_and_ciphertext.len() < NONCE_LEN {
            return Err("Invalid data");
        }
        
        let (nonce, ciphertext) = nonce_and_ciphertext.split_at_mut(NONCE_LEN);
        let unbound_key = aead::UnboundKey::new(&aead::AES_256_GCM, key).map_err(|_| "Failed to create key")?;
        let opening_key = aead::LessSafeKey::new(unbound_key);

        opening_key.open_in_place(aead::Nonce::assume_unique_for_key(nonce), aead::Aad::empty(), ciphertext)
            .map_err(|_| "Decryption failed")
            .map(|plaintext| plaintext.to_vec())
    }

    pub fn get_env_variable(key: &str) -> Option<String> {
        env::var(key).ok()
    }
}