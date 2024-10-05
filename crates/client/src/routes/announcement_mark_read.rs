use serde::{Deserialize, Serialize};
pub use crate::Announcement;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GetAnnouncementMarkReadResponse {
    pub ok: bool,
    pub announcement: Announcement,
}

pub type GetAnnouncementMarkReadResult = crate::APIResult<GetAnnouncementMarkReadResponse>;

#[derive(Serialize, Deserialize)]
pub struct GetAnnouncementMarkReadRequest {
    pub announcement_id: u64,
}

client_macros::api!(get, "v2/announcement.markread", GetAnnouncementMarkReadResult, GetAnnouncementMarkReadRequest);
