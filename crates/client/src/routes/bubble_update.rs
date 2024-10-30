// {"bubble_id":3756933,"pinned_message_id":92063041,"pinned_message_expires_at":"2024-10-30 04:46:37"}
// {"bubble_id":3756933,"pinned_message_id":null}
// {"bubble_id":3901612,"addmember":"owner"}
// {"bubble_id":"3901612","category_id":679345}

use serde::{Deserialize, Serialize};
use crate::Bubble;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum BubbleUpdateModification {
    SetPinnedMessage((u64, chrono::NaiveDateTime)),
    RemovePinnedMessage(),
    AddMemberPermission(String),
    RemoveMemberPermission(String),
    ChangeCategoryPermission(String),
    ChangeTitlePermission(String),
    LeaveGroupPermission(String),
    DeleteGroupPermission(String),
    CreateMessagePermission(String),
    SetCategory(Option<u64>),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BubbleUpdateResponse {
    pub ok: bool,
    pub bubble: Bubble
}
