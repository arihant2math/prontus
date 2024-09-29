use serde::{Deserialize, Serialize};
use serde_json::json;
use crate::Membership;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum MembershipUpdateModification {
    RemoveAlias,
    Alias(String),
    IsPinned(bool),
    Mute(Option<String>),
    Unmute,
    Hide,
}

#[derive(Clone, Debug, Deserialize)]
pub struct PostMembershipUpdateRequest {
    pub bubble_id: u64,
    pub modification: MembershipUpdateModification,
}

impl Serialize for PostMembershipUpdateRequest {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.to_json().serialize(serializer)
    }
}

impl PostMembershipUpdateRequest {
    pub fn to_json(&self) -> serde_json::Value {
        match &self.modification {
            MembershipUpdateModification::RemoveAlias => {
                json!({
                    "bubble_id": self.bubble_id,
                    "alias": ""
                })
            },
            MembershipUpdateModification::Alias(alias) => {
                json!({
                    "bubble_id": self.bubble_id,
                    "alias": alias
                })
            },
            MembershipUpdateModification::IsPinned(is_pinned) => {
                json!({
                    "bubble_id": self.bubble_id,
                    "is_pinned": is_pinned
                })
            },
            MembershipUpdateModification::Mute(until) => {
                json!({
                    "bubble_id": self.bubble_id,
                    "mute": true,
                    "muteuntil": until
                })
            },
            MembershipUpdateModification::Unmute => {
                json!({
                    "bubble_id": self.bubble_id,
                    "mute": false
                })
            },
            MembershipUpdateModification::Hide => {
                json!({
                    "bubble_id": self.bubble_id,
                    "ishidden": 1
                })
            },
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PostMembershipUpdateResponse {
    pub ok: bool,
    pub membership: Membership
}

pub type PostMembershipUpdateResult = crate::APIResult<PostMembershipUpdateResponse>;

client_macros::api!(post, "v1/membership.update", PostMembershipUpdateResult, PostMembershipUpdateRequest);
