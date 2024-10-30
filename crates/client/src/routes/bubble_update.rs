// {"bubble_id":3756933,"pinned_message_id":92063041,"pinned_message_expires_at":"2024-10-30 04:46:37"}
// {"bubble_id":3756933,"pinned_message_id":null}
// {"bubble_id":3901612,"addmember":"owner"}
// {"bubble_id":"3901612","category_id":679345}

use serde::{Deserialize, Serialize};
use serde_json::json;
use client_macros::api;
use crate::Bubble;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[non_exhaustive]
pub enum BubbleUpdateModification {
    SetTitle(String),
    SetPinnedMessage((u64, chrono::NaiveDateTime)),
    RemovePinnedMessage(),
    SetCategory(Option<u64>),
    AddMemberPermission(String),
    RemoveMemberPermission(String),
    ChangeCategoryPermission(String),
    ChangeTitlePermission(String),
    LeaveGroupPermission(String),
    DeleteGroupPermission(String),
    CreateMessagePermission(String),
}

#[derive(Clone, Debug, Deserialize)]
pub struct PostBubbleUpdateRequest {
    pub bubble_id: u64,
    pub modification: BubbleUpdateModification,
}

impl Serialize for PostBubbleUpdateRequest {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.to_json().serialize(serializer)
    }
}

impl PostBubbleUpdateRequest {
    pub fn to_json(&self) -> serde_json::Value {
        match &self.modification {
            BubbleUpdateModification::SetTitle(title) => {
                json!({
                    "bubble_id": self.bubble_id,
                    "title": title
                })
            }
            BubbleUpdateModification::SetPinnedMessage((message_id, expires_at)) => {
                json!({
                    "bubble_id": self.bubble_id,
                    "pinned_message_id": message_id,
                    "pinned_message_expires_at": expires_at
                })
            }
            BubbleUpdateModification::RemovePinnedMessage() => {
                json!({
                    "bubble_id": self.bubble_id,
                    "pinned_message_id": null
                })
            }
            BubbleUpdateModification::AddMemberPermission(permission) => {
                json!({
                    "bubble_id": self.bubble_id,
                    "addmember": permission
                })
            }
            BubbleUpdateModification::RemoveMemberPermission(permission) => {
                json!({
                    "bubble_id": self.bubble_id,
                    "removemember": permission
                })
            }
            BubbleUpdateModification::ChangeCategoryPermission(permission) => {
                json!({
                    "bubble_id": self.bubble_id,
                    "changecategory": permission
                })
            }
            BubbleUpdateModification::ChangeTitlePermission(permission) => {
                json!({
                    "bubble_id": self.bubble_id,
                    "changetitle": permission
                })
            }
            BubbleUpdateModification::LeaveGroupPermission(permission) => {
                json!({
                    "bubble_id": self.bubble_id,
                    "leavegroup": permission
                })
            }
            BubbleUpdateModification::DeleteGroupPermission(permission) => {
                json!({
                    "bubble_id": self.bubble_id,
                    "deletegroup": permission
                })
            }
            BubbleUpdateModification::CreateMessagePermission(permission) => {
                json!({
                    "bubble_id": self.bubble_id,
                    "create_message": permission
                })
            }
            BubbleUpdateModification::SetCategory(category_id) => {
                json!({
                    "bubble_id": self.bubble_id,
                    "category_id": category_id
                })
            }
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BubbleUpdateResponse {
    pub ok: bool,
    pub bubble: Bubble
}

pub type BubbleUpdateResult = crate::APIResult<BubbleUpdateResponse>;

api!(
    post,
    "v1/membership.update",
    BubbleUpdateResult,
    PostBubbleUpdateRequest
);
