use crate::Membership;
use serde::{Deserialize, Serialize};
use serde_json::json;
use crate::custom_json::ToJson;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum NotificationsPreference {
    All,
    Mentions,
    MentionsExcludeAll,
    Nothing,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum MembershipUpdateModification {
    RemoveAlias,
    Alias(String),
    IsPinned(bool),
    Mute(Option<String>),
    Unmute,
    Hide,
    NotificationsPreference(NotificationsPreference),
    Meetings(bool),
}

#[derive(Clone, Debug, Deserialize)]
pub struct PostMembershipUpdateRequest {
    pub bubble_id: u64,
    pub modification: MembershipUpdateModification,
}

impl ToJson for PostMembershipUpdateRequest {
    fn to_json(&self) -> serde_json::Value {
        match &self.modification {
            MembershipUpdateModification::RemoveAlias => {
                json!({
                    "bubble_id": self.bubble_id,
                    "alias": ""
                })
            }
            MembershipUpdateModification::Alias(alias) => {
                json!({
                    "bubble_id": self.bubble_id,
                    "alias": alias
                })
            }
            MembershipUpdateModification::IsPinned(is_pinned) => {
                json!({
                    "bubble_id": self.bubble_id,
                    "is_pinned": is_pinned
                })
            }
            MembershipUpdateModification::Mute(until) => {
                json!({
                    "bubble_id": self.bubble_id,
                    "mute": true,
                    "muteuntil": until
                })
            }
            MembershipUpdateModification::Unmute => {
                json!({
                    "bubble_id": self.bubble_id,
                    "mute": false
                })
            }
            MembershipUpdateModification::Hide => {
                json!({
                    "bubble_id": self.bubble_id,
                    "ishidden": 1
                })
            }
            MembershipUpdateModification::NotificationsPreference(value) => {
                let preference = match value {
                    NotificationsPreference::All => "ALL",
                    NotificationsPreference::Mentions => "MENTIONS",
                    NotificationsPreference::MentionsExcludeAll => "MENTIONS_EXCLUDE_ALL",
                    NotificationsPreference::Nothing => "NONE",
                };
                json!({
                    "bubble_id": self.bubble_id,
                    "notifications_preference": preference
                })
            }
            MembershipUpdateModification::Meetings(value) => {
                json!({
                    "bubble_id": self.bubble_id,
                    "meetings": value
                })
            }
        }
    }
}

impl Serialize for PostMembershipUpdateRequest {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.to_json().serialize(serializer)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PostMembershipUpdateResponse {
    pub ok: bool,
    pub membership: Membership,
}

pub type PostMembershipUpdateResult = crate::APIResult<PostMembershipUpdateResponse>;

client_macros::api!(
    post,
    "v1/membership.update",
    PostMembershipUpdateResult,
    PostMembershipUpdateRequest
);
