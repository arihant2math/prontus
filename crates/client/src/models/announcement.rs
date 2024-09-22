use crate::{MessageMedia, UserInfo};

pub struct Targets {
    pub organization_id: Option<i64>,
    pub bubble_ids: Option<Vec<i64>>,
}

struct Announcement {
    pub id: i64,
    pub organization_id: i64,
    pub senderuser_id: i64,
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
    pub announcementmedia: Vec<MessageMedia>,
}
