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

use crate::retrieval::PublicLookupService;
pub use encrypt_internal::*;
use std::string::FromUtf8Error;
use base64::DecodeError;
use base64::prelude::*;

pub struct Encrypt {
    pub dm_encryption: DMEncryption,
    pub lookup_service: PublicLookupService,
}

impl Encrypt {
    pub async fn new(
        public_lookup_service: PublicLookupService,
        org_id: u64,
        user_id: u64,
    ) -> Option<Self> {
        let key = public_lookup_service
            .lookup(org_id, user_id)?
            .as_bytes();
        let secret_key = load_secret_key();
        let dm_encryption = DMEncryption::new(&secret_key, key);
        Some(Self {
            dm_encryption,
            lookup_service: public_lookup_service,
        })
    }

    // TODO: these methods should not be lossy and instead map to u16 codepoints
    pub fn encrypt(&self, data: &str) -> String {
        let encrypted_data = self.dm_encryption.encrypt(data);
        BASE64_STANDARD.encode(&encrypted_data)
    }

    pub fn decrypt(&self, data: &str) -> Result<String, FromUtf8Error> {
        let decoded_data = BASE64_STANDARD.decode(data).unwrap();
        String::from_utf8(self.dm_encryption.decrypt(&decoded_data))
    }
}
