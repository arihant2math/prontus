use crate::Category;
use crate::UserInfo;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Bubble {
    pub id: u64,
    #[serde(rename = "channelcode")]
    pub channel_code: String,
    pub user_id: u64,
    pub title: String,
    #[serde(rename = "isdm")]
    pub is_dm: bool,
    #[serde(rename = "addmember")]
    pub delete_any_message: String,
    #[serde(rename = "changename")]
    pub change_title: String,
    #[serde(rename = "changecategory")]
    pub change_category: String,
    #[serde(rename = "addmember")]
    pub remove_member: String,
    #[serde(rename = "addmember")]
    pub leave_group: String,
    #[serde(rename = "deletegroup")]
    pub delete_group: String,
    #[serde(rename = "setrole")]
    pub set_role: String,
    pub create_announcement: String,
    pub assign_task: String,
    pub create_message: String,
    pub grant_create_message: bool,
    pub voice_only: bool,
    #[serde(rename = "issupergroup")]
    pub is_supergroup: Option<bool>,
    pub archived: u8,
    #[serde(rename = "dmpartner")]
    pub dm_partner: Option<UserInfo>,
    pub category: Option<Category>,
}
