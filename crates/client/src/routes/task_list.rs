// POST /api/v1/task.list
// Request = {"organization_id":2245,"completed":false}
// Response = {"ok":true,"pagesize":50,"tasks":[{"id":153736,"assigneeuser_id":5302428,"bubble_id":null,"organization_id":2245,"user_id":5302428,"notes":"Test Notes","remindedassignee":false,"title":"Test Task","uuid":"1ed52c0f-41cb-4860-8c26-6542705b8a2c","assigneeuser":{"id":5302428,"firstname":"Ashwin","fullname":"Ashwin Naren","lastname":"Naren","role":"user","autotranslate":false,"dropinorder":0,"hasactivity":true,"inactive":false,"isbot":0,"isonline":true,"isverified":false,"language":"en","locale":"en_US","maxstreams":10,"mute":false,"permissions":{"change_name":"system","change_email":"system","change_phone":"system","remove_user":"system","change_title":"admin","change_pronouns":"admin","change_own_name":false,"change_own_email":false,"change_own_phone":false,"change_own_title":true,"change_own_pronouns":true},"profilepic":true,"profilepicpath":"\/files\/users\/5302428\/profilepic?pronto_time=1695523284","profilepicurl":"https:\/\/files.chat.trypronto.com\/files\/users\/5302428\/profilepic?pronto_time=1695523284","status":0,"username":null,"acceptedtos":"2024-09-25 02:40:01","deactivated_at":null,"email_verified_at":"2024-09-25 02:40:01","lastpresencetime":"2024-10-05 04:48:02","lastseen":"2024-10-05 04:58:59","muteuntil":null,"phone_verified_at":null,"sentwelcomemsg":"2023-08-15 19:22:02","created_at":"2023-08-04 00:44:12","updated_at":"2024-10-05 04:48:45"},"taskmedia":[],"user":{"id":5302428,"firstname":"Ashwin","fullname":"Ashwin Naren","lastname":"Naren","role":"user","autotranslate":false,"dropinorder":0,"hasactivity":true,"inactive":false,"isbot":0,"isonline":true,"isverified":false,"language":"en","locale":"en_US","maxstreams":10,"mute":false,"permissions":{"change_name":"system","change_email":"system","change_phone":"system","remove_user":"system","change_title":"admin","change_pronouns":"admin","change_own_name":false,"change_own_email":false,"change_own_phone":false,"change_own_title":true,"change_own_pronouns":true},"profilepic":true,"profilepicpath":"\/files\/users\/5302428\/profilepic?pronto_time=1695523284","profilepicurl":"https:\/\/files.chat.trypronto.com\/files\/users\/5302428\/profilepic?pronto_time=1695523284","status":0,"username":null,"acceptedtos":"2024-09-25 02:40:01","deactivated_at":null,"email_verified_at":"2024-09-25 02:40:01","lastpresencetime":"2024-10-05 04:48:02","lastseen":"2024-10-05 04:58:59","muteuntil":null,"phone_verified_at":null,"sentwelcomemsg":"2023-08-15 19:22:02","created_at":"2023-08-04 00:44:12","updated_at":"2024-10-05 04:48:45"},"completed":null,"due":"2024-11-06 00:00:00","reminder_local":"2024-11-06 16:09:02","reminder_utc":"2024-11-06 16:09:02","created_at":"2024-09-22 23:09:44","updated_at":"2024-09-23 05:00:01"}],"hasmore":false}

use crate::Task;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PostTaskListRequest {
    pub organization_id: u64,
    pub completed: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PostTaskListResponse {
    pub ok: bool,
    pub pagesize: i64,
    pub tasks: Vec<Task>,
    pub hasmore: bool,
}

pub type PostTaskListResult = crate::APIResult<PostTaskListResponse>;

client_macros::api!(
    post,
    "v1/task.list",
    PostTaskListResult,
    PostTaskListRequest
);
