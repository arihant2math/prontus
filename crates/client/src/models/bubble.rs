use crate::Category;
use crate::UserInfo;
use serde::{Deserialize, Serialize};
use crate::{Membership, MembershipInfo};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BubbleMembershipItem {
    Membership(Membership),
    MembershipInfo(MembershipInfo),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Bubble {
    pub id: u64,
    #[serde(rename = "channelcode")]
    pub channel_code: String,
    pub user_id: u64,
    pub title: String,
    #[serde(rename = "isdm")]
    pub is_dm: bool,
    #[serde(rename = "deleteanymessage")]
    pub delete_any_message: String,
    #[serde(rename = "changetitle")]
    pub change_title: String,
    #[serde(rename = "changecategory")]
    pub change_category: String,
    #[serde(rename = "removemember")]
    pub remove_member: String,
    #[serde(rename = "leavegroup")]
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
    pub memberships: Vec<BubbleMembershipItem>,
}
