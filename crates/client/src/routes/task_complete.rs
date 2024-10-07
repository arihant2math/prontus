// POST	/api/v1/task.complete
// Request = {"task_id":153736}
// Response = {"ok":true,"task":{"id":153736,"assigneeuser_id":5302428,"bubble_id":null,"organization_id":2245,"user_id":5302428,"notes":"Test Notes","remindedassignee":false,"title":"Test Task","uuid":"1ed52c0f-41cb-4860-8c26-6542705b8a2c","assigneeuser":{"id":5302428,"firstname":"Ashwin","fullname":"Ashwin Naren","lastname":"Naren","role":"user","autotranslate":false,"dropinorder":0,"hasactivity":true,"inactive":false,"isbot":0,"isonline":true,"isverified":false,"language":"en","locale":"en_US","maxstreams":10,"mute":false,"permissions":{"change_name":"system","change_email":"system","change_phone":"system","remove_user":"system","change_title":"admin","change_pronouns":"admin","change_own_name":false,"change_own_email":false,"change_own_phone":false,"change_own_title":true,"change_own_pronouns":true},"profilepic":true,"profilepicpath":"\/files\/users\/5302428\/profilepic?pronto_time=1695523284","profilepicurl":"https:\/\/files.chat.trypronto.com\/files\/users\/5302428\/profilepic?pronto_time=1695523284","status":0,"username":null,"acceptedtos":"2024-09-25 02:40:01","deactivated_at":null,"email_verified_at":"2024-09-25 02:40:01","lastpresencetime":"2024-10-05 04:48:02","lastseen":"2024-10-05 05:00:32","muteuntil":null,"phone_verified_at":null,"sentwelcomemsg":"2023-08-15 19:22:02","created_at":"2023-08-04 00:44:12","updated_at":"2024-10-05 04:48:45"},"taskmedia":[],"user":{"id":5302428,"firstname":"Ashwin","fullname":"Ashwin Naren","lastname":"Naren","role":"user","autotranslate":false,"dropinorder":0,"hasactivity":true,"inactive":false,"isbot":0,"isonline":true,"isverified":false,"language":"en","locale":"en_US","maxstreams":10,"mute":false,"permissions":{"change_name":"system","change_email":"system","change_phone":"system","remove_user":"system","change_title":"admin","change_pronouns":"admin","change_own_name":false,"change_own_email":false,"change_own_phone":false,"change_own_title":true,"change_own_pronouns":true},"profilepic":true,"profilepicpath":"\/files\/users\/5302428\/profilepic?pronto_time=1695523284","profilepicurl":"https:\/\/files.chat.trypronto.com\/files\/users\/5302428\/profilepic?pronto_time=1695523284","status":0,"username":null,"acceptedtos":"2024-09-25 02:40:01","deactivated_at":null,"email_verified_at":"2024-09-25 02:40:01","lastpresencetime":"2024-10-05 04:48:02","lastseen":"2024-10-05 05:00:32","muteuntil":null,"phone_verified_at":null,"sentwelcomemsg":"2023-08-15 19:22:02","created_at":"2023-08-04 00:44:12","updated_at":"2024-10-05 04:48:45"},"completed":"2024-10-05 05:00:32","due":"2024-11-06 00:00:00","reminder_local":"2024-11-06 16:09:02","reminder_utc":"2024-11-06 16:09:02","created_at":"2024-09-22 23:09:44","updated_at":"2024-10-05 05:00:32"}}

use crate::Task;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PostTaskCompleteRequest {
    pub task_id: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PostTaskCompleteResponse {
    pub ok: bool,
    pub task: Task,
}

pub type PostTaskCompleteResult = crate::APIResult<PostTaskCompleteResponse>;

client_macros::api!(
    post,
    "v1/task.complete",
    PostTaskCompleteResult,
    PostTaskCompleteRequest
);
