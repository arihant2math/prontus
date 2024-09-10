use crate::Organization;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserInfo {
    pub id: u64,
    pub firstname: String,
    pub lastname: String,
    pub username: Option<String>,
    pub pronouns: Option<String>,
    #[serde(rename = "profilepicurl")]
    pub profile_picture_url: String,
    #[serde(rename = "isverified")]
    pub verified: bool,
    #[serde(rename = "isonline")]
    pub online: bool,
    pub role: String,
    pub mute: bool,
    #[serde(rename = "isbot")]
    pub is_bot: u8,
    #[serde(rename = "hasmobileapp")]
    pub has_mobile_app: Option<bool>,
    pub fullname: String,
    #[serde(rename = "hasactivity")]
    pub has_activity: bool,
    pub inactive: bool,
    pub language: String,
    #[serde(default)]
    pub organizations: Vec<Organization>,
}
