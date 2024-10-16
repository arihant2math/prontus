#![deny(
    bad_style,
    const_err,
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

use crypto_box::aead::{Aead, AeadCore, OsRng};
use crypto_box::{ChaChaBox, PublicKey, SecretKey};
use keyring::Entry;

pub struct DMEncryption {
    pub current_user_secret_key: SecretKey,
    pub other_user_public_key: PublicKey,
}

impl DMEncryption {
    pub fn new(current_user_secret_key: &[u8], other_user_public_key: &[u8]) -> Self {
        let current_user_secret_key = SecretKey::from_slice(current_user_secret_key).unwrap();
        let other_user_public_key = PublicKey::from_slice(other_user_public_key).unwrap();
        DMEncryption {
            current_user_secret_key,
            other_user_public_key,
        }
    }

    pub fn encrypt(&self, message: &str) -> Vec<u8> {
        let chachabox = ChaChaBox::new(&self.other_user_public_key, &self.current_user_secret_key);
        let nonce = ChaChaBox::generate_nonce(&mut OsRng);
        let encrypted = chachabox.encrypt(&nonce, message.as_bytes()).unwrap();
        encrypted
    }

    pub fn decrypt(&self, encrypted: &[u8]) -> Vec<u8> {
        let chachabox = ChaChaBox::new(&self.other_user_public_key, &self.current_user_secret_key);
        let nonce = ChaChaBox::generate_nonce(&mut OsRng);
        let decrypted = chachabox.decrypt(&nonce, encrypted).unwrap();
        decrypted
    }
}

static SERVICE: &str = "prontus-encrypt";
static DEFAULT_USER: &str = "com_prontus_default";

pub fn load_secret_key() -> [u8; 32] {
    let secret_vector = Entry::new(SERVICE, DEFAULT_USER)
        .unwrap()
        .get_secret()
        .unwrap();
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
