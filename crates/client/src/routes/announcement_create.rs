// POST /api/v1/announcement.create
// Request = {"targets":{"bubble_ids":[3756933]},"announcement":"Server Maintenance starting in 5"}
// Response = {"ok":true,"announcement":{"id":32544,"organization_id":2245,"senderuser_id":5302428,"targets":{"bubble_ids":[3756933]},"announcement":"Server Maintenance starting in 5","created_at":"2024-10-05 05:03:36","updated_at":"2024-10-05 05:03:36","deleted_at":null,"sent":null,"scheduled":null,"lang":null,"sender":{"id":5302428,"firstname":"Ashwin","lastname":"Naren","username":null,"locale":"en_US","lastseen":"2024-10-05 05:03:35","profilepic":true,"status":0,"created_at":"2023-08-04 00:44:12","updated_at":"2024-10-05 04:48:45","deactivated_at":null,"email_verified_at":"2024-09-25 02:40:01","phone_verified_at":null,"isverified":false,"dropinorder":0,"maxstreams":10,"autotranslate":false,"isonline":true,"lastpresencetime":"2024-10-05 04:48:02","acceptedtos":"2024-09-25 02:40:01","sentwelcomemsg":"2023-08-15 19:22:02","role":"user","mute":false,"muteuntil":null,"isbot":0,"fullname":"Ashwin Naren","hasactivity":true,"inactive":false,"language":"en","permissions":{"change_name":"system","change_email":"system","change_phone":"system","remove_user":"system","change_title":"admin","change_pronouns":"admin","change_own_name":false,"change_own_email":false,"change_own_phone":false,"change_own_title":true,"change_own_pronouns":true},"profilepicpath":"\/files\/users\/5302428\/profilepic?pronto_time=1695523284","profilepicurl":"https:\/\/files.chat.trypronto.com\/files\/users\/5302428\/profilepic?pronto_time=1695523284"},"announcementmedia":[]}}

use serde::{Deserialize, Serialize};
use crate::Announcement;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PostAnnouncementCreateRequestTargets {
    pub bubble_ids: Vec<u64>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PostAnnouncementCreateRequest {
    pub targets: PostAnnouncementCreateRequestTargets,
    pub announcement: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PostAnnouncementCreateResponse {
    pub ok: bool,
    pub announcement: Announcement,
}

pub type PostAnnouncementCreateResult = crate::APIResult<PostAnnouncementCreateResponse>;

client_macros::api!(post, "v1/announcement.create", PostAnnouncementCreateResult, PostAnnouncementCreateRequest);
