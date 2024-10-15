use std::collections::HashMap;
use serde::{Deserialize, Serialize};

static URL: &str = "https://github.com/arihant2math/prontus/";

#[derive(Clone, Debug, Serialize, Deserialize)]
struct ServerResponse {
    pub organizations: HashMap<String, String>
}

pub struct PublicLookupService {
    pub organizations: HashMap<String, String>
}

impl PublicLookupService {
    fn new() -> Self {
        let organizations = reqwest::blocking::get(URL).unwrap().json::<ServerResponse>().unwrap().organizations;
        Self {
            organizations
        }
    }

    pub async fn fetch() -> Self {
        let response = reqwest::get(URL).await.unwrap().json::<ServerResponse>().await.unwrap();
        Self {
            organizations: response.organizations
        }
    }

    pub fn lookup(&self, org_id: u64, user_id: u64) -> Option<&String> {
        let key = self.generate_key(org_id, user_id);
        self.organizations.get(&key)
    }

    pub fn generate_key(&self, org_id: u64, user_id: u64) -> String {
        let lookup_bytes = org_id.to_le_bytes().iter().chain(user_id.to_le_bytes().iter()).cloned().collect::<Vec<u8>>();
        let lookup_hash = blake3::hash(&lookup_bytes);
        lookup_hash.to_string()
    }
}
