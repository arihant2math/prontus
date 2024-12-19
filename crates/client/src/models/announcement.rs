use crate::{UserInfo};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Targets {
    pub organization_id: Option<u64>,
    pub bubble_ids: Option<Vec<u64>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AnnouncementMedia {
    pub id: u64,
    pub uuid: String,
    pub url: String,
    pub title: Option<String>,
    pub mediatype: String,
    #[serde(rename = "urlmimetype")]
    pub url_mimetype: String,
    pub width: u64,
    pub height: u64,
    pub filesize: u64,
    pub path: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Announcement {
    pub id: u64,
    pub organization_id: u64,
    pub senderuser_id: u64,
    pub targets: Targets,
    pub announcement: String,
    pub created_at: String,
    pub updated_at: Option<String>,
    pub deleted_at: Option<String>,
    pub sent: String,
    pub scheduled: Option<String>,
    pub read: Option<String>,
    pub lang: String,
    pub announcementtrans: Vec<String>,
    pub sender: UserInfo,
    pub announcementmedia: Vec<AnnouncementMedia>,
}
