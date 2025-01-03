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

mod retrieval;

pub use crate::retrieval::PublicLookupService;
use base64::prelude::*;
use encrypt_internal::{load_secret_key, DMEncryption};
use std::fmt::Display;
use std::string::FromUtf8Error;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DecryptionError {
    Base64Error(#[from] base64::DecodeError),
    Utf8Error(#[from] FromUtf8Error),
    CryptoError(#[from] encrypt_internal::Error),
}

impl Display for DecryptionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DecryptionError::Base64Error(e) => write!(f, "Base64 error: {}", e),
            DecryptionError::Utf8Error(e) => write!(f, "UTF-8 error: {}", e),
            DecryptionError::CryptoError(e) => write!(f, "Crypto error: {}", e),
        }
    }
}

pub struct Encrypt {
    pub dm_encryption: DMEncryption,
    pub lookup_service: PublicLookupService,
}

impl Encrypt {
    /// Lookup other user and load current user's secret key
    pub async fn new(
        public_lookup_service: PublicLookupService,
        org_id: u64,
        user_id: u64,
    ) -> Option<Self> {
        let key = public_lookup_service.lookup(org_id, user_id)?;
        // TODO: handle error
        let secret_key = load_secret_key().ok()?;
        let dm_encryption = DMEncryption::new(secret_key, key);
        Some(Self {
            dm_encryption,
            lookup_service: public_lookup_service,
        })
    }

    pub fn encrypt(&self, data: &str) -> String {
        let nonce = DMEncryption::generate_random_nonce();
        let mut encrypted_data = self.dm_encryption.encrypt(data.as_bytes(), &nonce).unwrap();
        let mut nonce_vec = nonce.to_vec();
        let initial_len = nonce_vec.len() as u64;
        let mut data = Vec::with_capacity(initial_len as usize + encrypted_data.len());
        data.append(&mut initial_len.to_le_bytes().to_vec());
        data.append(&mut nonce_vec);
        data.append(&mut encrypted_data);
        BASE64_STANDARD.encode(&data)
    }

    pub fn decrypt(&self, data: &str) -> Result<String, DecryptionError> {
        let decoded_data = BASE64_STANDARD.decode(data)?;
        let initial_len = u64::from_le_bytes([
            decoded_data[0],
            decoded_data[1],
            decoded_data[2],
            decoded_data[3],
            decoded_data[4],
            decoded_data[5],
            decoded_data[6],
            decoded_data[7],
        ]);
        let nonce = decoded_data[8..initial_len as usize + 8].to_vec();
        let decoded_data = &decoded_data[initial_len as usize + 8..];
        let nonce = DMEncryption::convert_nonce(&nonce);
        Ok(String::from_utf8(
            self.dm_encryption.decrypt(&decoded_data, &nonce)?,
        )?)
    }
}

#[cfg(test)]
mod tests {
    use crate::retrieval::PublicLookupService;
    use crate::Encrypt;
    use encrypt_internal::DMEncryption;

    #[test]
    fn test_validity() {
        let current_user_keys = encrypt_internal::generate_key_pair();
        let other_user_keys = encrypt_internal::generate_key_pair();
        let encrypt = Encrypt {
            dm_encryption: DMEncryption::new(
                current_user_keys.secret_key,
                other_user_keys.public_key,
            ),
            lookup_service: PublicLookupService {
                organizations: Default::default(),
            },
        };
        let decrypt = Encrypt {
            dm_encryption: DMEncryption::new(
                other_user_keys.secret_key,
                current_user_keys.public_key,
            ),
            lookup_service: PublicLookupService {
                organizations: Default::default(),
            },
        };
        let data = "Hello, World!";
        let encrypted_data = encrypt.encrypt(data);
        let decrypted_data = decrypt.decrypt(&encrypted_data).unwrap();
        assert_eq!(data, decrypted_data);
    }
}
