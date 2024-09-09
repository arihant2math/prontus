use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Category {
    pub id: u64,
    pub title: String,
    pub sort_order: Option<u32>,
}
