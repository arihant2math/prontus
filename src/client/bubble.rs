use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Bubble {
    pub id: u64,
    pub user_id: u64,
    pub title: String,
    pub isdm: bool,
    pub voice_only: bool,
    pub issupergroup: Option<bool>,
    pub archived: u8,
}