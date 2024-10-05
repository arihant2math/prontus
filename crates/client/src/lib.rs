extern crate alloc;

use std::sync::Arc;

use chrono::Utc;
use reqwest::header::{HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::bubble_history::GetBubbleHistoryResponse;
use crate::bubble_info::GetBubbleInfoResponse;
use crate::bubble_list::GetBubbleListResponse;
use crate::user_token_login::TokenLoginResponse;

use crate::user_info::GetUserInfoRequest;
pub use api_error::APIError;
pub use models::*;
pub use routes::*;
use crate::announcement_list::GetAnnouncementListRequest;
use crate::bubble_mark::PostBubbleMarkRequest;
pub use crate::bubble_membership_search::PostBubbleMembershipSearchRequest;
use crate::files::PutFileResponse;
pub use crate::membership_update::{MembershipUpdateModification, NotificationsPreference, PostMembershipUpdateRequest};
pub use crate::message_create::MessageModifyResponse;
use crate::message_edit::MessageEditRequest;
use crate::pusher_auth::PusherAuthRequest;
use crate::reaction_add::ReactionModifyRequest;

pub mod api_error;
pub mod models;
pub mod routes;
pub(crate) mod serde_datetime;
mod client;

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum APIResult<T> {
    Ok(T),
    Err(APIError),
}

impl<T> Clone for APIResult<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        match self {
            APIResult::Ok(t) => APIResult::Ok(t.clone()),
            APIResult::Err(e) => APIResult::Err(e.clone()),
        }
    }
}

impl<T> APIResult<T> {
    pub fn to_result(self) -> Result<T, APIError> {
        match self {
            APIResult::Ok(t) => Ok(t),
            APIResult::Err(e) => Err(e),
        }
    }
}

#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
pub enum ReactionType {
    Null = -1,
    Like = 1,
    Dislike = 2,
    Laugh = 3,
    Love = 4,
    Cry = 5,
    Amazed = 6,
}

// Pronto name -> api response permission mappings
// Admin = System
// Manager = Owner
// User = Member

impl From<i32> for ReactionType {
    fn from(i: i32) -> Self {
        match i {
            1 => ReactionType::Like,
            2 => ReactionType::Dislike,
            3 => ReactionType::Laugh,
            4 => ReactionType::Love,
            5 => ReactionType::Cry,
            6 => ReactionType::Amazed,
            _ => ReactionType::Null,
        }
    }
}

pub struct ProntoClient {
    pub api_base_url: String,
    pub http_client: reqwest::Client,
}

#[derive(Debug, Error)]
pub enum NewClientError {
    #[error("Reqwuest error: {0}")]
    ReqwestError(#[from] reqwest::Error),
    #[error("Header parse error: {0}")]
    HeaderParseError(#[from] reqwest::header::InvalidHeaderValue),
    #[error("Url parse error: {0}")]
    UrlParseError(#[from] url::ParseError),
}

#[derive(Debug, Error)]
pub enum ResponseError {
    #[error("Reqwuest error: {0}")]
    ReqwestError(#[from] reqwest::Error),
    #[error("Serde JSON error: {0}")]
    SerdeJsonError(#[from] serde_json::Error),
    #[error("API error: {0}")]
    ApiError(String),
}

impl From<APIError> for ResponseError {
    fn from(e: APIError) -> Self {
        ResponseError::ApiError(e.to_string())
    }
}

impl ProntoClient {
    /// Create a new ProntoClient
    pub fn new(api_base_url: String, pronto_api_token: &str) -> Result<Self, NewClientError> {
        // create the cookie store
        let cookies = vec![format!("api_token={}", pronto_api_token)];
        let jar = reqwest::cookie::Jar::default();
        for cookie in cookies {
            jar.add_cookie_str(&cookie, &reqwest::Url::parse(&api_base_url)?);
        }

        let mut headers = HeaderMap::new();
        headers.insert(
            "Accept",
            HeaderValue::from_str("application/json, text/plain, */*")?,
        );
        headers.insert("Accept-Language", HeaderValue::from_str("en-US,en;q=0.5")?);
        headers.insert(
            "Authorization",
            HeaderValue::from_str(&format!("Bearer {pronto_api_token}"))?,
        );
        let client = reqwest::Client::builder()
            .cookie_store(true)
            .cookie_provider(Arc::new(jar))
            .default_headers(headers)
            .brotli(true)
            .build()?;
        Ok(Self {
            api_base_url,
            http_client: client,
        })
    }

    pub async fn upload_file(&self, filename: &str, file: Vec<u8>) -> Result<PutFileResponse, ResponseError> {
        Ok(files::put(&self.api_base_url, &self.http_client, files::PutFileRequest {
            file_name: filename.to_string(),
            file_data: file,
        }).await?.to_result()?)
    }
}

// TODO: All of this
// GET https://stanfordohs.pronto.io/api/clients/files/0a43fa48-403c-4a4e-8af5-ca0c01bab35c/normalized?preset=PHOTO
// Request = None
// Response = {"data":{"original":{"mimetype":"image\/png","key":"0a43fa48-403c-4a4e-8af5-ca0c01bab35c","name":"image.png","width":1002,"height":832,"filesize":74720},"normalized":{"mimetype":"image\/png","key":"e6e3084c-7222-4241-85a6-1ee11d584a39","name":"image.png","width":1002,"height":832,"filesize":52600},"is_animated":false}}
// When sending message, see {"ok":true,"message":{"id":89171261,"bubble_id":3738656,"user_id":5302428,"message":"","resource_id":null,"clickcount":0,"likecount":0,"dislikecount":0,"viewcount":0,"version":0,"user_edited_version":0,"user_edited_at":null,"created_at":"2024-09-17 15:45:15","updated_at":"2024-09-17 15:45:15","livestream_id":null,"videosession_id":null,"systemmessageparts":null,"uuid":"e0703e87-c181-4f2e-858c-a679e16ebdf9","task_id":null,"parentmessage_id":null,"firstchildmessage_id":null,"lastchildmessage_id":null,"systemevent":null,"reactionsummary":[],"lang":null,"videosession":null,"user":{"id":5302428,"firstname":"Ashwin","lastname":"Naren","username":null,"locale":"","lastseen":"2024-09-17 15:45:15","profilepic":true,"status":0,"created_at":"2023-08-04 00:44:12","updated_at":"2024-09-17 15:44:47","deactivated_at":null,"email_verified_at":"2024-09-15 23:34:54","phone_verified_at":null,"isverified":false,"dropinorder":0,"maxstreams":10,"autotranslate":false,"isonline":true,"lastpresencetime":"2024-09-17 15:44:06","acceptedtos":"2024-09-15 23:34:54","sentwelcomemsg":"2023-08-15 19:22:02","role":"user","mute":false,"muteuntil":null,"isbot":0,"fullname":"Ashwin Naren","hasactivity":true,"inactive":false,"language":"en","permissions":{"change_name":"system","change_email":"system","change_phone":"system","remove_user":"system","change_title":"admin","change_pronouns":"admin","change_own_name":false,"change_own_email":false,"change_own_phone":false,"change_own_title":true,"change_own_pronouns":true},"profilepicpath":"\/files\/users\/5302428\/profilepic?pronto_time=1695523284","profilepicurl":"https:\/\/files.chat.trypronto.com\/files\/users\/5302428\/profilepic?pronto_time=1695523284"},"mentions":[],"messagemedia":[{"message_id":89171261,"title":"image.png","url":"https:\/\/files.chat.trypronto.com\/files\/media\/3738656\/d56f7980-750b-11ef-9a28-6f7e119ffd69","uuid":"e6e3084c-7222-4241-85a6-1ee11d584a39","width":1002,"height":832,"filesize":52600,"duration":null,"updated_at":"2024-09-17 15:45:16","created_at":"2024-09-17 15:45:16","id":7081542,"mediatype":"PHOTO","urlmimetype":"image\/png","thumbnailmimetype":null,"path":"\/files\/media\/3738656\/d56f7980-750b-11ef-9a28-6f7e119ffd69","thumbnailpath":null}]}}

// POST https://stanfordohs.pronto.io/api/v1/bubble.create
// Request = {"title":"test","organization_id":2245}
// Response = {"ok":true,"bubble":{"id":3844880,"user_id":5302428,"title":"test","iconurl":null,"channelcode":"3fSwDVzZoPsJJNEFWSPUaSEarSKuU4NrQEfJay6B","created_at":"2024-10-05 04:54:13","updated_at":"2024-10-05 04:54:13","livestream_id":null,"recordlivestream":false,"videosession_id":null,"organization_id":2245,"isdm":false,"voice_only":false,"category_id":null,"deleteanymessage":"owner","changetitle":"owner","grantchangetitle":true,"changecategory":"owner","grantchangecategory":true,"addmember":"member","grantaddmember":true,"removemember":"member","grantremovemember":true,"leavegroup":"member","grantleavegroup":true,"deletegroup":"owner","videosessionrecordlocal":"member","grantvideosessionrecordlocal":true,"videosessionrecordcloud":"member","grantvideosessionrecordcloud":true,"setrole":"owner","archived":0,"translation_enabled":true,"externalid":null,"async_reactions_enabled":false,"async_read_receipts_enabled":false,"async_mentions_enabled":false,"is_supergroup":false,"create_announcement":"owner","grant_create_announcement":true,"create_message":"member","grant_create_message":true,"assign_task":"member","grant_assign_task":true,"create_videosession":"member","grant_create_videosession":true,"pinned_message_id":null,"pinned_message_user_id":null,"pinned_message_expires_at":null,"pinned_message_at":null,"pin_message":"owner","grant_pin_message":true,"dmpartner":null,"category":null,"tasks_enabled":true,"organization":{"id":2245,"name":"Stanford Online High School","created_at":"2022-05-23 21:21:47","updated_at":"2024-08-29 21:50:51","profilepic":1,"profilepicupdated":"2023-04-25 02:01:06","tasks_enabled":true,"uuid":"5a688730dade11ec9efe71a87fba95ed","deleteanymessage":null,"changetitle":null,"grantchangetitle":null,"changecategory":null,"grantchangecategory":null,"addmember":null,"grantaddmember":null,"removemember":null,"grantremovemember":null,"leavegroup":null,"grantleavegroup":null,"deletegroup":null,"videosessionrecordlocal":null,"grantvideosessionrecordlocal":null,"videosessionrecordcloud":null,"grantvideosessionrecordcloud":null,"setrole":null,"shortname":"stanfordohs","url":null,"logo":null,"announcements_enabled":true,"grant_create_announcement":true,"grant_create_group":true,"grant_add_user":true,"grant_search_org":true,"grant_create_dm":true,"create_announcement":"manager","create_group":"user","add_user":"admin","search_org":"user","create_dm":"user","integrations_enabled":true,"grant_delete_any_announcement":true,"delete_any_announcement":"admin","meetings_enabled":true,"audio_messages_enabled":true,"maxstreams":10,"imports_enabled":true,"search_enabled":true,"create_api_tokens":"admin","bubble_membership_cap":3000,"badgecount_writing_enabled":1,"badgecount_reading_enabled":1,"experimental_notifications_enabled":0,"supergroups_enabled":true,"meetings_captions_enabled":false,"giphy_rating":"PG-13","user_title_enabled":false,"user_pronouns_enabled":true,"categories":[{"id":670656,"color":null,"sortorder":29,"title":"OHS - home of Pixels"},{"id":679345,"color":null,"sortorder":149,"title":"Clubs"},{"id":751065,"color":null,"sortorder":222,"title":"HRMCK - Homeroom: Model UN Club - S2"},{"id":850026,"color":null,"sortorder":299,"title":"UM150 - Multivariable Calculus* - S1 - 2024-2025"},{"id":850027,"color":null,"sortorder":300,"title":"HRM12Q - Homeroom: Topic Group Q - S1 - 2024-2025"},{"id":868001,"color":null,"sortorder":342,"title":"UM51A - UN Linear Algebra* - S1 - 2024-2025"}],"permissions":{"DEPRECATED":"Don't use this permissions object anymore.","create_announcement":"manager","create_group":"user","add_user":"admin","search_org":"user"},"profilepicurl":"https:\/\/files.chat.trypronto.com\/files\/orgs\/2245\/profilepic?pronto_time=1682388066","profilepicpath":"\/files\/orgs\/2245\/profilepic?pronto_time=1682388066","create_group_announcement":"admin","grant_create_group_announcement":false},"memberships":[{"id":42193609,"user_id":5302428,"bubble_id":3844880,"mark":0,"friends":true,"system":true,"mute":false,"created_at":"2024-10-05 04:54:13","updated_at":"2024-10-05 04:54:13","markupdated":"2024-10-05 04:54:13","isdropin":false,"banned":false,"reactions":true,"notificationrollup":true,"removedby":null,"muteuntil":null,"is_pinned":false,"supergroup_alert_seen":false,"role":"owner","snooze":null,"user":{"id":5302428,"firstname":"Ashwin","lastname":"Naren","username":null,"locale":"en_US","lastseen":"2024-10-05 04:54:13","profilepic":true,"status":0,"created_at":"2023-08-04 00:44:12","updated_at":"2024-10-05 04:48:45","deactivated_at":null,"email_verified_at":"2024-09-25 02:40:01","phone_verified_at":null,"isverified":false,"dropinorder":0,"maxstreams":10,"autotranslate":false,"isonline":true,"lastpresencetime":"2024-10-05 04:48:02","acceptedtos":"2024-09-25 02:40:01","sentwelcomemsg":"2023-08-15 19:22:02","role":"user","mute":false,"muteuntil":null,"isbot":0,"fullname":"Ashwin Naren","hasactivity":true,"inactive":false,"language":"en","permissions":{"change_name":"system","change_email":"system","change_phone":"system","remove_user":"system","change_title":"admin","change_pronouns":"admin","change_own_name":false,"change_own_email":false,"change_own_phone":false,"change_own_title":true,"change_own_pronouns":true},"profilepicpath":"\/files\/users\/5302428\/profilepic?pronto_time=1695523284","profilepicurl":"https:\/\/files.chat.trypronto.com\/files\/users\/5302428\/profilepic?pronto_time=1695523284"}}],"users":[{"id":5302428,"firstname":"Ashwin","lastname":"Naren","username":null,"locale":"en_US","lastseen":"2024-10-05 04:54:13","profilepic":true,"status":0,"created_at":"2023-08-04 00:44:12","updated_at":"2024-10-05 04:48:45","deactivated_at":null,"email_verified_at":"2024-09-25 02:40:01","phone_verified_at":null,"isverified":false,"dropinorder":0,"maxstreams":10,"autotranslate":false,"isonline":true,"lastpresencetime":"2024-10-05 04:48:02","acceptedtos":"2024-09-25 02:40:01","sentwelcomemsg":"2023-08-15 19:22:02","role":"user","mute":false,"muteuntil":null,"isbot":0,"fullname":"Ashwin Naren","hasactivity":true,"inactive":false,"language":"en","permissions":{"change_name":"system","change_email":"system","change_phone":"system","remove_user":"system","change_title":"admin","change_pronouns":"admin","change_own_name":false,"change_own_email":false,"change_own_phone":false,"change_own_title":true,"change_own_pronouns":true},"profilepicpath":"\/files\/users\/5302428\/profilepic?pronto_time=1695523284","profilepicurl":"https:\/\/files.chat.trypronto.com\/files\/users\/5302428\/profilepic?pronto_time=1695523284"}]},"stats":[{"bubble_id":3844880,"mark":90845817,"updated":"2024-10-05 04:54:14","unread":0,"unread_mentions":0,"latest_message_id":90845817,"latest_message_created_at":"2024-10-05 04:54:14","unclaimed_task_count":0}]}

// POST /api/v1/bubble.invite
// Request = {"bubble_id":3844880,"invitations":[{"user_id":5302428}],"sendemails":false,"sendsms":false}
// Response = {"ok":true,"users":[],"invitationgroups":[]}

// POST https://stanfordohs.pronto.io/api/v1/bubble.delete
// Request = {"bubble_id":"3844880"}
// Response = {"ok":true}


// POST /api/v1/task.list
// Request = {"organization_id":2245,"completed":false}
// Response = {"ok":true,"pagesize":50,"tasks":[{"id":153736,"assigneeuser_id":5302428,"bubble_id":null,"organization_id":2245,"user_id":5302428,"notes":"Test Notes","remindedassignee":false,"title":"Test Task","uuid":"1ed52c0f-41cb-4860-8c26-6542705b8a2c","assigneeuser":{"id":5302428,"firstname":"Ashwin","fullname":"Ashwin Naren","lastname":"Naren","role":"user","autotranslate":false,"dropinorder":0,"hasactivity":true,"inactive":false,"isbot":0,"isonline":true,"isverified":false,"language":"en","locale":"en_US","maxstreams":10,"mute":false,"permissions":{"change_name":"system","change_email":"system","change_phone":"system","remove_user":"system","change_title":"admin","change_pronouns":"admin","change_own_name":false,"change_own_email":false,"change_own_phone":false,"change_own_title":true,"change_own_pronouns":true},"profilepic":true,"profilepicpath":"\/files\/users\/5302428\/profilepic?pronto_time=1695523284","profilepicurl":"https:\/\/files.chat.trypronto.com\/files\/users\/5302428\/profilepic?pronto_time=1695523284","status":0,"username":null,"acceptedtos":"2024-09-25 02:40:01","deactivated_at":null,"email_verified_at":"2024-09-25 02:40:01","lastpresencetime":"2024-10-05 04:48:02","lastseen":"2024-10-05 04:58:59","muteuntil":null,"phone_verified_at":null,"sentwelcomemsg":"2023-08-15 19:22:02","created_at":"2023-08-04 00:44:12","updated_at":"2024-10-05 04:48:45"},"taskmedia":[],"user":{"id":5302428,"firstname":"Ashwin","fullname":"Ashwin Naren","lastname":"Naren","role":"user","autotranslate":false,"dropinorder":0,"hasactivity":true,"inactive":false,"isbot":0,"isonline":true,"isverified":false,"language":"en","locale":"en_US","maxstreams":10,"mute":false,"permissions":{"change_name":"system","change_email":"system","change_phone":"system","remove_user":"system","change_title":"admin","change_pronouns":"admin","change_own_name":false,"change_own_email":false,"change_own_phone":false,"change_own_title":true,"change_own_pronouns":true},"profilepic":true,"profilepicpath":"\/files\/users\/5302428\/profilepic?pronto_time=1695523284","profilepicurl":"https:\/\/files.chat.trypronto.com\/files\/users\/5302428\/profilepic?pronto_time=1695523284","status":0,"username":null,"acceptedtos":"2024-09-25 02:40:01","deactivated_at":null,"email_verified_at":"2024-09-25 02:40:01","lastpresencetime":"2024-10-05 04:48:02","lastseen":"2024-10-05 04:58:59","muteuntil":null,"phone_verified_at":null,"sentwelcomemsg":"2023-08-15 19:22:02","created_at":"2023-08-04 00:44:12","updated_at":"2024-10-05 04:48:45"},"completed":null,"due":"2024-11-06 00:00:00","reminder_local":"2024-11-06 16:09:02","reminder_utc":"2024-11-06 16:09:02","created_at":"2024-09-22 23:09:44","updated_at":"2024-09-23 05:00:01"}],"hasmore":false}

// POST	/api/v1/task.complete
// Request = {"task_id":153736}
// Response = {"ok":true,"task":{"id":153736,"assigneeuser_id":5302428,"bubble_id":null,"organization_id":2245,"user_id":5302428,"notes":"Test Notes","remindedassignee":false,"title":"Test Task","uuid":"1ed52c0f-41cb-4860-8c26-6542705b8a2c","assigneeuser":{"id":5302428,"firstname":"Ashwin","fullname":"Ashwin Naren","lastname":"Naren","role":"user","autotranslate":false,"dropinorder":0,"hasactivity":true,"inactive":false,"isbot":0,"isonline":true,"isverified":false,"language":"en","locale":"en_US","maxstreams":10,"mute":false,"permissions":{"change_name":"system","change_email":"system","change_phone":"system","remove_user":"system","change_title":"admin","change_pronouns":"admin","change_own_name":false,"change_own_email":false,"change_own_phone":false,"change_own_title":true,"change_own_pronouns":true},"profilepic":true,"profilepicpath":"\/files\/users\/5302428\/profilepic?pronto_time=1695523284","profilepicurl":"https:\/\/files.chat.trypronto.com\/files\/users\/5302428\/profilepic?pronto_time=1695523284","status":0,"username":null,"acceptedtos":"2024-09-25 02:40:01","deactivated_at":null,"email_verified_at":"2024-09-25 02:40:01","lastpresencetime":"2024-10-05 04:48:02","lastseen":"2024-10-05 05:00:32","muteuntil":null,"phone_verified_at":null,"sentwelcomemsg":"2023-08-15 19:22:02","created_at":"2023-08-04 00:44:12","updated_at":"2024-10-05 04:48:45"},"taskmedia":[],"user":{"id":5302428,"firstname":"Ashwin","fullname":"Ashwin Naren","lastname":"Naren","role":"user","autotranslate":false,"dropinorder":0,"hasactivity":true,"inactive":false,"isbot":0,"isonline":true,"isverified":false,"language":"en","locale":"en_US","maxstreams":10,"mute":false,"permissions":{"change_name":"system","change_email":"system","change_phone":"system","remove_user":"system","change_title":"admin","change_pronouns":"admin","change_own_name":false,"change_own_email":false,"change_own_phone":false,"change_own_title":true,"change_own_pronouns":true},"profilepic":true,"profilepicpath":"\/files\/users\/5302428\/profilepic?pronto_time=1695523284","profilepicurl":"https:\/\/files.chat.trypronto.com\/files\/users\/5302428\/profilepic?pronto_time=1695523284","status":0,"username":null,"acceptedtos":"2024-09-25 02:40:01","deactivated_at":null,"email_verified_at":"2024-09-25 02:40:01","lastpresencetime":"2024-10-05 04:48:02","lastseen":"2024-10-05 05:00:32","muteuntil":null,"phone_verified_at":null,"sentwelcomemsg":"2023-08-15 19:22:02","created_at":"2023-08-04 00:44:12","updated_at":"2024-10-05 04:48:45"},"completed":"2024-10-05 05:00:32","due":"2024-11-06 00:00:00","reminder_local":"2024-11-06 16:09:02","reminder_utc":"2024-11-06 16:09:02","created_at":"2024-09-22 23:09:44","updated_at":"2024-10-05 05:00:32"}}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Once;

    static INIT: Once = Once::new();

    async fn get_client() -> ProntoClient {
        INIT.call_once(|| {
            simple_logger::init_with_level(log::Level::Debug).unwrap();
        });
        let settings = settings::Settings::load().await.unwrap();
        let client = ProntoClient::new(
            "https://stanfordohs.pronto.io/api/".to_string(), &settings.auth.api_key.unwrap());
        client.unwrap()
    }

    #[tokio::test]
    async fn test_client() {
        get_client().await;
    }

    #[tokio::test]
    async fn test_get_current_user_info() {
        let client = get_client().await;
        let _response = client.current_user_info().await.unwrap();
    }

    #[tokio::test]
    async fn test_get_bubble_list() {
        let client = get_client().await;
        let _response = client.bubble_list().await.unwrap();
    }

    #[tokio::test]
    async fn test_get_bubble_info() {
        let client = get_client().await;
        let bubble_list = client.bubble_list().await.unwrap();
        let bubble_id = bubble_list.bubbles[0].id;
        let _response = client.bubble_info(bubble_id).await.unwrap();
    }

    #[tokio::test]
    async fn test_get_announcement_list() {
        let client = get_client().await;
        client.list_announcements().await.unwrap();
    }

    #[tokio::test]
    async fn test_get_bubble_history() {
        let client = get_client().await;
        let bubble_list = client.bubble_list().await.unwrap();
        let bubble_id = bubble_list.bubbles[0].id;
        let _response = client.bubble_history(bubble_id, None).await.unwrap();
    }
}
