use crate::Category;
use crate::UserInfo;
use crate::{Membership, MembershipInfo};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BubbleMembershipItem {
    Membership(Membership),
    MembershipInfo(MembershipInfo),
    Other(()),
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
    pub voice_only: bool,
    #[serde(rename = "deleteanymessage")]
    pub delete_any_message: String,
    #[serde(rename = "changetitle")]
    pub change_title: String,
    #[serde(rename = "grantchangetitle")]
    pub grant_change_title: bool,
    #[serde(rename = "changecategory")]
    pub change_category: String,
    #[serde(rename = "grantchangecategory")]
    pub grant_change_category: bool,
    #[serde(rename = "addmember")]
    pub add_member: String,
    #[serde(rename = "grantaddmember")]
    pub grant_add_member: bool,
    #[serde(rename = "removemember")]
    pub remove_member: String,
    #[serde(rename = "grantremovemember")]
    pub grant_remove_member: bool,
    #[serde(rename = "leavegroup")]
    pub leave_group: String,
    #[serde(rename = "grantleavegroup")]
    pub grant_leave_group: bool,
    #[serde(rename = "deletegroup")]
    pub delete_group: String,
    #[serde(rename = "grantdeletegroup")]
    pub grant_delete_group: Option<bool>,
    #[serde(rename = "setrole")]
    pub set_role: String,
    pub create_announcement: String,
    pub assign_task: String,
    pub create_message: String,
    pub grant_create_message: bool,
    #[serde(rename = "issupergroup")]
    pub is_supergroup: Option<bool>,
    pub archived: u8,
    #[serde(rename = "dmpartner")]
    pub dm_partner: Option<UserInfo>,
    pub category: Option<Category>,
    pub memberships: Option<Vec<BubbleMembershipItem>>,
}
