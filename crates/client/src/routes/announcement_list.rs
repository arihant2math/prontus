use serde::{Deserialize, Serialize};
pub use crate::Announcement;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Cursors {
    pub prev: Option<String>,
    pub next: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GetAnnouncementListResponse {
    pub ok: bool,
    pub announcements: Vec<Announcement>,
    pub pagesize: u64,
    #[serde(rename = "hasmore")]
    pub has_more: bool,
    pub cursors: Cursors,
}

pub type GetAnnouncementListResult = crate::APIResult<GetAnnouncementListResponse>;

#[derive(Serialize, Deserialize)]
pub struct GetAnnouncementListRequest {
    /// UNREAD, or RECEIVED
    pub query: String,
    /// 20 should work
    #[serde(rename = "perPage")]
    pub per_page: u64,
}

client_macros::api!(get, "v2/announcement.list", GetAnnouncementListResult, GetAnnouncementListRequest);
