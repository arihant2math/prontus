use crate::user_info::UserInfo;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Category {
    pub id: u64,
    pub title: String,
    pub sort_order: Option<u32>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Bubble {
    pub id: u64,
    pub user_id: u64,
    pub title: String,
    pub isdm: bool,
    pub deleteanymessage: String,
    pub changetitle: String,
    pub changecategory: String,
    pub removemember: String,
    pub leavegroup: String,
    pub deletegroup: String,
    pub setrole: String,
    pub create_announcement: String,
    pub assign_task: String,
    pub create_message: String,
    pub grant_create_message: bool,
    pub voice_only: bool,
    pub issupergroup: Option<bool>,
    pub archived: u8,
    pub dmpartner: Option<UserInfo>,
    pub category: Option<Category>,
}
