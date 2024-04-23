use serde::{Deserialize, Serialize};
use crate::client::user_info::UserInfo;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Category {
    pub id: u64,
    pub title: String,
    pub sort_order: Option<u32>
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Bubble {
    pub id: u64,
    pub user_id: u64,
    pub title: String,
    pub isdm: bool,
    pub voice_only: bool,
    pub issupergroup: Option<bool>,
    pub archived: u8,
    pub dmpartner: Option<UserInfo>,
    pub category: Option<Category>
}