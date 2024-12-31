#![deny(
    bad_style,
    dead_code,
    improper_ctypes,
    non_shorthand_field_patterns,
    no_mangle_generic_items,
    overflowing_literals,
    path_statements,
    patterns_in_fns_without_body,
    unconditional_recursion,
    unused,
    unused_allocation,
    unused_comparisons,
    unused_parens,
    while_true
)]

pub use aead::{Error, Result};
use crypto_box::aead::{Aead, AeadCore, OsRng};
use crypto_box::{aead, ChaChaBox, Nonce, PublicKey, SecretKey};
use keyring::Entry;

pub struct DMEncryption {
    pub current_user_secret_key: SecretKey,
    pub other_user_public_key: PublicKey,
}

impl DMEncryption {
    pub fn new(current_user_secret_key: [u8; 32], other_user_public_key: [u8; 32]) -> Self {
        let current_user_secret_key = SecretKey::from_bytes(current_user_secret_key);
        let other_user_public_key = PublicKey::from_bytes(other_user_public_key);
        DMEncryption {
            current_user_secret_key,
            other_user_public_key,
        }
    }

    pub fn generate_random_nonce() -> Nonce {
        ChaChaBox::generate_nonce(&mut OsRng)
    }

    pub fn convert_nonce(nonce: &[u8]) -> Nonce {
        Nonce::from_slice(nonce).clone()
    }

    pub fn encrypt(&self, message: &[u8], nonce: &Nonce) -> Result<Vec<u8>> {
        let chachabox = ChaChaBox::new(&self.other_user_public_key, &self.current_user_secret_key);
        let encrypted = chachabox.encrypt(&nonce, message)?;
        Ok(encrypted)
    }

    pub fn decrypt(&self, encrypted: &[u8], nonce: &Nonce) -> Result<Vec<u8>> {
        let chachabox = ChaChaBox::new(&self.other_user_public_key, &self.current_user_secret_key);
        let decrypted = chachabox.decrypt(nonce, encrypted)?;
        Ok(decrypted)
    }
}

const SERVICE: &str = "prontus-encrypt";
const DEFAULT_USER: &str = "com_prontus_default";

pub fn load_secret_key() -> [u8; 32] {
    let secret_vector = Entry::new(SERVICE, DEFAULT_USER)
        .expect("Failed to load secret key, please report this issue to the keyring developers")
        .get_secret()
        .unwrap();
    // Keep in array to allow for Copy
    let mut secret_key = [0u8; 32];
    secret_key.copy_from_slice(&secret_vector);
    secret_key
}

pub fn store_secret_key(secret_key: [u8; 32]) {
    let entry = Entry::new(SERVICE, DEFAULT_USER).unwrap();
    entry.set_secret(&secret_key).unwrap();
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct KeyPair {
    pub secret_key: [u8; 32],
    pub public_key: [u8; 32],
}

pub fn generate_key_pair() -> KeyPair {
    let secret_key = SecretKey::generate(&mut OsRng);
    let public_key = secret_key.public_key();
    KeyPair {
        secret_key: secret_key.to_bytes(),
        public_key: public_key.to_bytes(),
    }
}

pub fn load_key_pair(secret_key: [u8; 32]) -> KeyPair {
    let secret_key = SecretKey::from_slice(&secret_key).unwrap();
    let public_key = secret_key.public_key();
    KeyPair {
        secret_key: secret_key.to_bytes(),
        public_key: public_key.to_bytes(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encryption() {
        let current_user_keys = generate_key_pair();
        let other_user_keys = generate_key_pair();
        let encrypt_dm =
            DMEncryption::new(current_user_keys.secret_key, other_user_keys.public_key);
        let decrypt_dm =
            DMEncryption::new(other_user_keys.secret_key, current_user_keys.public_key);
        let data = b"Hello, World!";
        let nonce = DMEncryption::generate_random_nonce();
        let encrypted_data = encrypt_dm.encrypt(data, &nonce).unwrap();
        let decrypted_data = decrypt_dm.decrypt(&encrypted_data, &nonce).unwrap();
        assert_eq!(data.to_vec(), decrypted_data);
    }
}
