use crate::Organization;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserInfo {
    pub id: u64,
    pub firstname: String,
    pub lastname: String,
    pub username: Option<String>,
    pub profilepicurl: String,
    pub isverified: bool,
    pub isonline: bool,
    pub role: String,
    pub mute: bool,
    pub isbot: u8,
    pub hasmobileapp: Option<bool>,
    pub fullname: String,
    pub hasactivity: bool,
    pub inactive: bool,
    pub language: String,
    pub organizations: Vec<Organization>,
}
