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
pub use crate::membership_update::{MembershipUpdateModification, NotificationsPreference, PostMembershipUpdateRequest};
pub use crate::message_create::MessageModifyResponse;
use crate::message_edit::MessageEditRequest;
use crate::pusher_auth::PusherAuthRequest;
use crate::reaction_add::ReactionModifyRequest;

pub mod api_error;
pub mod models;
pub mod routes;
pub(crate) mod serde_datetime;

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

    pub async fn pusher_auth(
        &self,
        socket_id: &str,
        channel_name: &str,
    ) -> Result<pusher_auth::PusherAuthResponse, ResponseError> {
        Ok(pusher_auth::post(
            &self.api_base_url,
            &self.http_client,
            PusherAuthRequest{
                socket_id: socket_id.to_string(),
                channel_name: channel_name.to_string(),
            },
        )
        .await?)
    }

    pub async fn get_current_user_info(
        &self,
    ) -> Result<user_info::GetUserInfoResponse, ResponseError> {
        self.get_user_info(None).await
    }

    pub async fn get_user_info(
        &self,
        id: Option<u64>,
    ) -> Result<user_info::GetUserInfoResponse, ResponseError> {
        Ok(user_info::get(
            &self.api_base_url,
            &self.http_client,
            GetUserInfoRequest { id },
        )
            .await?
            .to_result()?)
    }

    pub async fn create_dm(&self, organization_id: u64, user_id: u64) -> Result<dm_create::PostDMCreateResponse, ResponseError> {
        Ok(dm_create::post(
            &self.api_base_url,
            &self.http_client,
            dm_create::PostDMCreateRequest {
                organization_id,
                user_id,
            },
        )
            .await?
            .to_result()?)
    }

    pub async fn get_bubble_list(&self) -> Result<GetBubbleListResponse, ResponseError> {
        Ok(bubble_list::get(&self.api_base_url, &self.http_client)
            .await?
            .to_result()?)
    }

    pub async fn get_bubble_info(
        &self,
        bubble_id: u64,
    ) -> Result<GetBubbleInfoResponse, ResponseError> {
        Ok(
            bubble_info::get(&self.api_base_url, &self.http_client, bubble_info::GetBubbleInfoRequest { bubble_id: bubble_id })
                .await?
                .to_result()?,
        )
    }

    pub async fn get_bubble_history(
        &self,
        bubble_id: u64,
        latest_message_id: Option<u64>,
    ) -> Result<GetBubbleHistoryResponse, ResponseError> {
        Ok(bubble_history::get(
            &self.api_base_url,
            &self.http_client,
            bubble_id,
            latest_message_id,
        )
            .await?
            .to_result()?)
    }

    pub async fn update_bubble_mark(
        &self,
        bubble_id: u64,
        message_id: u64,
    ) -> Result<bubble_mark::PostBubbleMarkResponse, ResponseError> {
        Ok(bubble_mark::post(
            &self.api_base_url,
            &self.http_client,
            PostBubbleMarkRequest {
                bubble_id,
                message_id,
            },
        ).await?
            .to_result()?)
    }

    pub async fn get_bubble_membership(
        &self,
        request: PostBubbleMembershipSearchRequest
    ) -> Result<bubble_membership_search::PostBubbleMembershipSearchResponse, ResponseError> {
        Ok(bubble_membership_search::post(
            &self.api_base_url,
            &self.http_client,
            request
        )
            .await?
            .to_result()?)
    }

    pub async fn pin_bubble(&self, bubble_id: u64, state: bool) -> Result<membership_update::PostMembershipUpdateResponse, ResponseError> {
        Ok(membership_update::post(
            &self.api_base_url,
            &self.http_client,
            PostMembershipUpdateRequest {
                bubble_id,
                modification: MembershipUpdateModification::IsPinned(state),
            }
        )
            .await?
            .to_result()?)
    }

    pub async fn hide_bubble(&self, bubble_id: u64) -> Result<membership_update::PostMembershipUpdateResponse, ResponseError> {
        Ok(membership_update::post(
            &self.api_base_url,
            &self.http_client,
            PostMembershipUpdateRequest {
                bubble_id,
                modification: MembershipUpdateModification::Hide,
            }
        )
            .await?
            .to_result()?)
    }

    pub async fn mute_bubble(&self, bubble_id: u64, state: bool) -> Result<membership_update::PostMembershipUpdateResponse, ResponseError> {
        if state {
            Ok(membership_update::post(
                &self.api_base_url,
                &self.http_client,
                PostMembershipUpdateRequest {
                    bubble_id,
                    modification: MembershipUpdateModification::Mute(None),
                }
            )
                .await?
                .to_result()?)
        } else {
            Ok(membership_update::post(
                &self.api_base_url,
                &self.http_client,
                PostMembershipUpdateRequest {
                    bubble_id,
                    modification: MembershipUpdateModification::Unmute,
                }
            )
                .await?
                .to_result()?)
        }
    }

    pub async fn set_bubble_alias(&self, bubble_id: u64, alias: Option<String>) -> Result<membership_update::PostMembershipUpdateResponse, ResponseError> {
        if let Some(alias) = alias {
            Ok(membership_update::post(
                &self.api_base_url,
                &self.http_client,
                PostMembershipUpdateRequest {
                    bubble_id,
                    modification: MembershipUpdateModification::Alias(alias),
                }
            )
                .await?
                .to_result()?)
        } else {
            Ok(membership_update::post(
                &self.api_base_url,
                &self.http_client,
                PostMembershipUpdateRequest {
                    bubble_id,
                    modification: MembershipUpdateModification::RemoveAlias,
                }
            )
                .await?
                .to_result()?)
        }
    }

    pub async fn remove_bubble_alias(&self, bubble_id: u64) -> Result<membership_update::PostMembershipUpdateResponse, ResponseError> {
        Ok(membership_update::post(
            &self.api_base_url,
            &self.http_client,
            PostMembershipUpdateRequest {
                bubble_id,
                modification: MembershipUpdateModification::RemoveAlias,
            }
        )
            .await?
            .to_result()?)
    }

    pub async fn set_bubble_notifications_preferences(&self, bubble_id: u64, preference: NotificationsPreference) -> Result<membership_update::PostMembershipUpdateResponse, ResponseError> {
        Ok(membership_update::post(
            &self.api_base_url,
            &self.http_client,
            PostMembershipUpdateRequest {
                bubble_id,
                modification: MembershipUpdateModification::NotificationsPreference(preference),
            }
        )
            .await?
            .to_result()?)
    }

    pub async fn post_message(
        &self,
        user_id: u64,
        bubble_id: u64,
        message: String,
        parent_message_id: Option<u64>,
    ) -> Result<MessageModifyResponse, ResponseError> {
        Ok(message_create::post(
            &self.api_base_url,
            &self.http_client,
            bubble_id,
            message,
            user_id,
            Utc::now(),
            parent_message_id,
        )
        .await?
        .to_result()?)
    }

    pub async fn edit_message(
        &self,
        message_id: u64,
        message: String,
    ) -> Result<MessageModifyResponse, ResponseError> {
        Ok(
            message_edit::post(&self.api_base_url, &self.http_client, MessageEditRequest {
                message_id,
                message
            })
                .await?
                .to_result()?,
        )
    }

    pub async fn delete_message(
        &self,
        message_id: u64,
    ) -> Result<message_delete::DeleteMessageResult, ResponseError> {
        Ok(
            message_delete::post(&self.api_base_url, &self.http_client, message_id)
                .await?
                .to_result()?,
        )
    }

    pub async fn add_reaction(
        &self,
        message_id: u64,
        reaction_type: ReactionType,
    ) -> Result<MessageModifyResponse, ResponseError> {
        Ok(reaction_add::post(
            &self.api_base_url,
            &self.http_client,
            ReactionModifyRequest {
                message_id,
                reaction_type_id: reaction_type as i32 as u64
            },
        )
            .await?
            .to_result()?)
    }

    pub async fn remove_reaction(
        &self,
        message_id: u64,
        reaction_type: ReactionType,
    ) -> Result<MessageModifyResponse, ResponseError> {
        Ok(reaction_remove::post(
            &self.api_base_url,
            &self.http_client,
            ReactionModifyRequest {
                message_id,
                reaction_type_id: reaction_type as i32 as u64
            }
        )
            .await?
            .to_result()?)
    }

    pub async fn user_token_login(&self, token: &str) -> Result<TokenLoginResponse, ResponseError> {
        // TODO: pass in device info
        Ok(user_token_login::post(
            &self.api_base_url,
            &self.http_client,
            vec![token.to_string()],
        )
            .await?
            .to_result()?)
    }

    pub async fn get_announcement_list(&self) -> Result<announcement_list::GetAnnouncementListResponse, ResponseError> {
        Ok(announcement_list::get(&self.api_base_url, &self.http_client, GetAnnouncementListRequest {
            query: "RECEIVED".to_string(),
            per_page: 20,
        })
            .await?
            .to_result()?)
    }
}

// TODO: File upload
// PUT https://stanfordohs.pronto.io/api/files?filename=image.png
// Request Payload = image
// Response = {"data":{"key":"0a43fa48-403c-4a4e-8af5-ca0c01bab35c","expires":"2024-09-18T15:44:32Z","name":"image.png","size":74720,"type":"image/png"}}
// GET https://stanfordohs.pronto.io/api/clients/files/0a43fa48-403c-4a4e-8af5-ca0c01bab35c/normalized?preset=PHOTO
// Request Payload = None
// Response = {"data":{"original":{"mimetype":"image\/png","key":"0a43fa48-403c-4a4e-8af5-ca0c01bab35c","name":"image.png","width":1002,"height":832,"filesize":74720},"normalized":{"mimetype":"image\/png","key":"e6e3084c-7222-4241-85a6-1ee11d584a39","name":"image.png","width":1002,"height":832,"filesize":52600},"is_animated":false}}
// When sending message, see {"ok":true,"message":{"id":89171261,"bubble_id":3738656,"user_id":5302428,"message":"","resource_id":null,"clickcount":0,"likecount":0,"dislikecount":0,"viewcount":0,"version":0,"user_edited_version":0,"user_edited_at":null,"created_at":"2024-09-17 15:45:15","updated_at":"2024-09-17 15:45:15","livestream_id":null,"videosession_id":null,"systemmessageparts":null,"uuid":"e0703e87-c181-4f2e-858c-a679e16ebdf9","task_id":null,"parentmessage_id":null,"firstchildmessage_id":null,"lastchildmessage_id":null,"systemevent":null,"reactionsummary":[],"lang":null,"videosession":null,"user":{"id":5302428,"firstname":"Ashwin","lastname":"Naren","username":null,"locale":"","lastseen":"2024-09-17 15:45:15","profilepic":true,"status":0,"created_at":"2023-08-04 00:44:12","updated_at":"2024-09-17 15:44:47","deactivated_at":null,"email_verified_at":"2024-09-15 23:34:54","phone_verified_at":null,"isverified":false,"dropinorder":0,"maxstreams":10,"autotranslate":false,"isonline":true,"lastpresencetime":"2024-09-17 15:44:06","acceptedtos":"2024-09-15 23:34:54","sentwelcomemsg":"2023-08-15 19:22:02","role":"user","mute":false,"muteuntil":null,"isbot":0,"fullname":"Ashwin Naren","hasactivity":true,"inactive":false,"language":"en","permissions":{"change_name":"system","change_email":"system","change_phone":"system","remove_user":"system","change_title":"admin","change_pronouns":"admin","change_own_name":false,"change_own_email":false,"change_own_phone":false,"change_own_title":true,"change_own_pronouns":true},"profilepicpath":"\/files\/users\/5302428\/profilepic?pronto_time=1695523284","profilepicurl":"https:\/\/files.chat.trypronto.com\/files\/users\/5302428\/profilepic?pronto_time=1695523284"},"mentions":[],"messagemedia":[{"message_id":89171261,"title":"image.png","url":"https:\/\/files.chat.trypronto.com\/files\/media\/3738656\/d56f7980-750b-11ef-9a28-6f7e119ffd69","uuid":"e6e3084c-7222-4241-85a6-1ee11d584a39","width":1002,"height":832,"filesize":52600,"duration":null,"updated_at":"2024-09-17 15:45:16","created_at":"2024-09-17 15:45:16","id":7081542,"mediatype":"PHOTO","urlmimetype":"image\/png","thumbnailmimetype":null,"path":"\/files\/media\/3738656\/d56f7980-750b-11ef-9a28-6f7e119ffd69","thumbnailpath":null}]}}

// GET https://stanfordohs.pronto.io/api/v1/announcement.markread
// Request Payload = {"announcement_id":32222}
// Response: {"ok":true,"announcement":{"id":32222,"organization_id":2245,"senderuser_id":5279806,"targets":{"bubble_ids":[2828820]},"announcement":"\u2b50\ufe0f Hey Sophomores! \u2b50\ufe0f\n\nJust a quick reminder that our first movie night is starting at 5pm pst today!! Hope to see you there!\n\nZoom link: https:\/\/stanford.zoom.us\/j\/91400975734?pwd=2lA6b9bRi0VvzEKngRijdEgMyN75nv.1\n\n-Izzy and Neel","created_at":"2024-09-28 23:05:50","updated_at":"2024-09-28 23:05:51","deleted_at":null,"sent":"2024-09-28 23:05:51","scheduled":null,"read":"2024-09-30 01:22:02","lang":"en","sender":{"id":5279806,"firstname":"Izzy","lastname":"Nguyen","username":null,"locale":"","lastseen":"2024-09-29 04:11:38","profilepic":true,"status":0,"created_at":"2023-07-28 18:17:03","updated_at":"2024-09-29 04:11:50","deactivated_at":null,"email_verified_at":"2024-09-20 22:48:01","phone_verified_at":null,"isverified":false,"dropinorder":0,"maxstreams":10,"autotranslate":true,"isonline":false,"lastpresencetime":"2024-09-29 04:11:50","acceptedtos":"2024-09-20 22:48:01","sentwelcomemsg":null,"role":"user","mute":true,"muteuntil":null,"isbot":0,"fullname":"Izzy Nguyen","hasactivity":true,"inactive":false,"language":"en","permissions":{"change_name":"system","change_email":"system","change_phone":"system","remove_user":"system","change_title":"admin","change_pronouns":"admin","change_own_name":false,"change_own_email":false,"change_own_phone":false,"change_own_title":true,"change_own_pronouns":true},"profilepicpath":"\/files\/users\/5279806\/profilepic?pronto_time=1700314741","profilepicurl":"https:\/\/files.chat.trypronto.com\/files\/users\/5279806\/profilepic?pronto_time=1700314741"},"announcementmedia":[{"id":8696,"announcement_id":32222,"title":"Screenshot 2024-09-28 at 4.05.06\u202fPM.png","url":"https:\/\/files.chat.trypronto.com\/files\/orgs\/2245\/announcements\/32222\/3422de00-7dee-11ef-9549-dbc45047dbab","thumbnail":"","width":1268,"height":950,"filesize":640894,"duration":null,"created_at":"2024-09-28 23:05:50","updated_at":"2024-09-28 23:05:50","uuid":"4e407183-10d9-488d-b5be-91dbe0d1b685","mediatype":"PHOTO","urlmimetype":"image\/png","thumbnailmimetype":null,"path":"\/files\/orgs\/2245\/announcements\/32222\/3422de00-7dee-11ef-9549-dbc45047dbab","thumbnailpath":"","external":false}],"announcementtrans":[]}}

// GET /api/v1/message.search
// Request Payload = {"search_type":"files","size":25,"from":0,"orderby":"newest","query":"test"}
// {"ok":true,"current_page_offset":0,"pagesize":25,"next_page_offset":25,"total_results":0,"results":[],"bubbles":[]}

// GET /api/v1/message.search
// Request Payload = {"search_type":"messages","size":25,"from":0,"orderby":"newest","query":"test"}
// {"ok":true,"current_page_offset":0,"pagesize":25,"next_page_offset":25,"total_results":191,"results":[{"message_id":90639242,"message":{"id":90639242,"bubble_id":3738656,"user_id":5302428,"message":"bump test","resource_id":null,"clickcount":0,"likecount":0,"dislikecount":0,"viewcount":0,"version":0,"user_edited_version":0,"user_edited_at":null,"created_at":"2024-10-03 03:52:36","updated_at":"2024-10-03 03:52:37","livestream_id":null,"videosession_id":null,"systemmessageparts":null,"uuid":"6ec69c92-04e0-47b6-af59-19de6b455c4e","task_id":null,"parentmessage_id":null,"firstchildmessage_id":null,"lastchildmessage_id":null,"systemevent":null,"reactionsummary":[],"lang":"en","videosession":null,"resource":null,"messagemedia":[],"user":{"id":5302428,"firstname":"Ashwin","lastname":"Naren","username":null,"locale":"en_US","lastseen":"2024-10-03 04:35:47","profilepic":true,"status":0,"created_at":"2023-08-04 00:44:12","updated_at":"2024-10-03 04:35:18","deactivated_at":null,"email_verified_at":"2024-09-25 02:40:01","phone_verified_at":null,"isverified":false,"dropinorder":0,"maxstreams":10,"autotranslate":false,"isonline":true,"lastpresencetime":"2024-10-03 04:35:18","acceptedtos":"2024-09-25 02:40:01","sentwelcomemsg":"2023-08-15 19:22:02","role":"user","mute":false,"muteuntil":null,"isbot":0,"fullname":"Ashwin Naren","hasactivity":true,"inactive":false,"language":"en","permissions":{"change_name":"system","change_email":"system","change_phone":"system","remove_user":"system","change_title":"admin","change_pronouns":"admin","change_own_name":false,"change_own_email":false,"change_own_phone":false,"change_own_title":true,"change_own_pronouns":true},"profilepicpath":"\/files\/users\/5302428\/profilepic?pronto_time=1695523284","profilepicurl":"https:\/\/files.chat.trypronto.com\/files\/users\/5302428\/profilepic?pronto_time=1695523284"},"task":null,"messagetrans":[],"mentions":[]},"highlight":{"messagelangs":{"en":["bump <pronto_hl>test<\/pronto_hl>"]},"message":["bump <pronto_hl>test<\/pronto_hl>"]}},{"message_id":90639070,"message":{"id":90639070,"bubble_id":3738656,"user_id":5302428,"message":"time test","resource_id":null,"clickcount":0,"likecount":0,"dislikecount":0,"viewcount":0,"version":0,"user_edited_version":0,"user_edited_at":null,"created_at":"2024-10-03 03:49:27","updated_at":"2024-10-03 03:49:27","livestream_id":null,"videosession_id":null,"systemmessageparts":null,"uuid":"3b586817-4273-48b9-b489-0eba26d310d3","task_id":null,"parentmessage_id":null,"firstchildmessage_id":null,"lastchildmessage_id":null,"systemevent":null,"reactionsummary":[],"lang":"en","videosession":null,"resource":null,"messagemedia":[],"user":{"id":5302428,"firstname":"Ashwin","lastname":"Naren","username":null,"locale":"en_US","lastseen":"2024-10-03 04:35:47","profilepic":true,"status":0,"created_at":"2023-08-04 00:44:12","updated_at":"2024-10-03 04:35:18","deactivated_at":null,"email_verified_at":"2024-09-25 02:40:01","phone_verified_at":null,"isverified":false,"dropinorder":0,"maxstreams":10,"autotranslate":false,"isonline":true,"lastpresencetime":"2024-10-03 04:35:18","acceptedtos":"2024-09-25 02:40:01","sentwelcomemsg":"2023-08-15 19:22:02","role":"user","mute":false,"muteuntil":null,"isbot":0,"fullname":"Ashwin Naren","hasactivity":true,"inactive":false,"language":"en","permissions":{"change_name":"system","change_email":"system","change_phone":"system","remove_user":"system","change_title":"admin","change_pronouns":"admin","change_own_name":false,"change_own_email":false,"change_own_phone":false,"change_own_title":true,"change_own_pronouns":true},"profilepicpath":"\/files\/users\/5302428\/profilepic?pronto_time=1695523284","profilepicurl":"https:\/\/files.chat.trypronto.com\/files\/users\/5302428\/profilepic?pronto_time=1695523284"},"task":null,"messagetrans":[],"mentions":[]},"highlight":{"messagelangs":{"en":["time <pronto_hl>test<\/pronto_hl>"]},"message":["time <pronto_hl>test<\/pronto_hl>"]}},{"message_id":90638076,"message":{"id":90638076,"bubble_id":3738656,"user_id":5302428,"message":"test 3","resource_id":null,"clickcount":0,"likecount":0,"dislikecount":0,"viewcount":0,"version":0,"user_edited_version":0,"user_edited_at":null,"created_at":"2024-10-03 03:29:47","updated_at":"2024-10-03 03:29:49","livestream_id":null,"videosession_id":null,"systemmessageparts":null,"uuid":"4c18099b-5ba1-44fa-a772-dca99b1cfa4d","task_id":null,"parentmessage_id":88491592,"firstchildmessage_id":null,"lastchildmessage_id":null,"systemevent":null,"reactionsummary":[],"lang":"en","videosession":null,"resource":null,"messagemedia":[],"user":{"id":5302428,"firstname":"Ashwin","lastname":"Naren","username":null,"locale":"en_US","lastseen":"2024-10-03 04:35:47","profilepic":true,"status":0,"created_at":"2023-08-04 00:44:12","updated_at":"2024-10-03 04:35:18","deactivated_at":null,"email_verified_at":"2024-09-25 02:40:01","phone_verified_at":null,"isverified":false,"dropinorder":0,"maxstreams":10,"autotranslate":false,"isonline":true,"lastpresencetime":"2024-10-03 04:35:18","acceptedtos":"2024-09-25 02:40:01","sentwelcomemsg":"2023-08-15 19:22:02","role":"user","mute":false,"muteuntil":null,"isbot":0,"fullname":"Ashwin Naren","hasactivity":true,"inactive":false,"language":"en","permissions":{"change_name":"system","change_email":"system","change_phone":"system","remove_user":"system","change_title":"admin","change_pronouns":"admin","change_own_name":false,"change_own_email":false,"change_own_phone":false,"change_own_title":true,"change_own_pronouns":true},"profilepicpath":"\/files\/users\/5302428\/profilepic?pronto_time=1695523284","profilepicurl":"https:\/\/files.chat.trypronto.com\/files\/users\/5302428\/profilepic?pronto_time=1695523284"},"task":null,"messagetrans":[],"mentions":[]},"highlight":{"messagelangs":{"en":["<pronto_hl>test<\/pronto_hl> 3"]},"message":["<pronto_hl>test<\/pronto_hl> 3"]}},{"message_id":90638006,"message":{"id":90638006,"bubble_id":3738656,"user_id":5302428,"message":"test 2","resource_id":null,"clickcount":0,"likecount":0,"dislikecount":0,"viewcount":0,"version":0,"user_edited_version":0,"user_edited_at":null,"created_at":"2024-10-03 03:28:39","updated_at":"2024-10-03 03:28:40","livestream_id":null,"videosession_id":null,"systemmessageparts":null,"uuid":"ccbd4dac-e08c-4a59-b380-8adbe3eb715a","task_id":null,"parentmessage_id":88491592,"firstchildmessage_id":null,"lastchildmessage_id":null,"systemevent":null,"reactionsummary":[],"lang":"en","videosession":null,"resource":null,"messagemedia":[],"user":{"id":5302428,"firstname":"Ashwin","lastname":"Naren","username":null,"locale":"en_US","lastseen":"2024-10-03 04:35:47","profilepic":true,"status":0,"created_at":"2023-08-04 00:44:12","updated_at":"2024-10-03 04:35:18","deactivated_at":null,"email_verified_at":"2024-09-25 02:40:01","phone_verified_at":null,"isverified":false,"dropinorder":0,"maxstreams":10,"autotranslate":false,"isonline":true,"lastpresencetime":"2024-10-03 04:35:18","acceptedtos":"2024-09-25 02:40:01","sentwelcomemsg":"2023-08-15 19:22:02","role":"user","mute":false,"muteuntil":null,"isbot":0,"fullname":"Ashwin Naren","hasactivity":true,"inactive":false,"language":"en","permissions":{"change_name":"system","change_email":"system","change_phone":"system","remove_user":"system","change_title":"admin","change_pronouns":"admin","change_own_name":false,"change_own_email":false,"change_own_phone":false,"change_own_title":true,"change_own_pronouns":true},"profilepicpath":"\/files\/users\/5302428\/profilepic?pronto_time=1695523284","profilepicurl":"https:\/\/files.chat.trypronto.com\/files\/users\/5302428\/profilepic?pronto_time=1695523284"},"task":null,"messagetrans":[],"mentions":[]},"highlight":{"messagelangs":{"en":["<pronto_hl>test<\/pronto_hl> 2"]},"message":["<pronto_hl>test<\/pronto_hl> 2"]}},{"message_id":90587737,"message":{"id":90587737,"bubble_id":3832006,"user_id":6056544,"message":"my geo test reuslts came out","resource_id":null,"clickcount":0,"likecount":0,"dislikecount":0,"viewcount":0,"version":0,"user_edited_version":0,"user_edited_at":null,"created_at":"2024-10-02 20:13:54","updated_at":"2024-10-02 20:13:55","livestream_id":null,"videosession_id":null,"systemmessageparts":null,"uuid":"a54d663a-0055-42a4-b87f-4ae094ad8d74","task_id":null,"parentmessage_id":null,"firstchildmessage_id":null,"lastchildmessage_id":null,"systemevent":null,"reactionsummary":[],"lang":"en","videosession":null,"resource":null,"messagemedia":[],"user":{"id":6056544,"firstname":"Wynn","lastname":"Liang","username":null,"locale":null,"lastseen":"2024-10-03 04:30:18","profilepic":true,"status":0,"created_at":"2024-08-01 21:33:45","updated_at":"2024-10-03 04:15:46","deactivated_at":null,"email_verified_at":"2024-09-14 21:49:53","phone_verified_at":null,"isverified":false,"dropinorder":0,"maxstreams":10,"autotranslate":false,"isonline":true,"lastpresencetime":"2024-10-03 03:44:01","acceptedtos":"2024-09-14 21:49:53","sentwelcomemsg":"2024-08-09 17:13:13","role":"user","mute":false,"muteuntil":null,"isbot":0,"fullname":"Wynn Liang","hasactivity":true,"inactive":false,"language":"en","permissions":{"change_name":"system","change_email":"system","change_phone":"system","remove_user":"system","change_title":"admin","change_pronouns":"admin","change_own_name":false,"change_own_email":false,"change_own_phone":false,"change_own_title":true,"change_own_pronouns":true},"profilepicpath":"\/files\/users\/6056544\/profilepic?pronto_time=1725836412","profilepicurl":"https:\/\/files.chat.trypronto.com\/files\/users\/6056544\/profilepic?pronto_time=1725836412"},"task":null,"messagetrans":[],"mentions":[]},"highlight":{"messagelangs":{"en":["my geo <pronto_hl>test<\/pronto_hl> reuslts came out"]},"message":["my geo <pronto_hl>test<\/pronto_hl> reuslts came out"]}},{"message_id":90502763,"message":{"id":90502763,"bubble_id":3832006,"user_id":6056529,"message":"what are these tests?","resource_id":null,"clickcount":0,"likecount":0,"dislikecount":0,"viewcount":0,"version":0,"user_edited_version":0,"user_edited_at":null,"created_at":"2024-10-02 00:47:09","updated_at":"2024-10-02 00:47:10","livestream_id":null,"videosession_id":null,"systemmessageparts":null,"uuid":"1dedd37f-1161-4bef-8977-0035cdf5bcbf","task_id":null,"parentmessage_id":null,"firstchildmessage_id":null,"lastchildmessage_id":null,"systemevent":null,"reactionsummary":[],"lang":"en","videosession":null,"resource":null,"messagemedia":[],"user":{"id":6056529,"firstname":"Ryan","lastname":"Ing","username":null,"locale":"en_US","lastseen":"2024-10-02 22:54:09","profilepic":true,"status":0,"created_at":"2024-08-01 21:33:43","updated_at":"2024-10-02 03:55:45","deactivated_at":null,"email_verified_at":"2024-09-13 16:10:06","phone_verified_at":null,"isverified":false,"dropinorder":0,"maxstreams":10,"autotranslate":true,"isonline":false,"lastpresencetime":"2024-10-02 03:55:45","acceptedtos":"2024-09-13 16:10:06","sentwelcomemsg":null,"role":"user","mute":true,"muteuntil":null,"isbot":0,"fullname":"Ryan Ing","hasactivity":true,"inactive":false,"language":"en","permissions":{"change_name":"system","change_email":"system","change_phone":"system","remove_user":"system","change_title":"admin","change_pronouns":"admin","change_own_name":false,"change_own_email":false,"change_own_phone":false,"change_own_title":true,"change_own_pronouns":true},"profilepicpath":"\/files\/users\/6056529\/profilepic?pronto_time=1723225178","profilepicurl":"https:\/\/files.chat.trypronto.com\/files\/users\/6056529\/profilepic?pronto_time=1723225178"},"task":null,"messagetrans":[],"mentions":[]},"highlight":{"messagelangs":{"en":["what are these <pronto_hl>tests<\/pronto_hl>?"]}}},{"message_id":90502754,"message":{"id":90502754,"bubble_id":3832006,"user_id":6056544,"message":"im taking a generation test","resource_id":null,"clickcount":0,"likecount":0,"dislikecount":0,"viewcount":0,"version":22,"user_edited_version":0,"user_edited_at":null,"created_at":"2024-10-02 00:47:01","updated_at":"2024-10-02 00:52:51","livestream_id":null,"videosession_id":null,"systemmessageparts":null,"uuid":"0b46be32-437e-4786-88e0-20293def6ec2","task_id":null,"parentmessage_id":null,"firstchildmessage_id":90502773,"lastchildmessage_id":90503303,"systemevent":null,"reactionsummary":[],"lang":"en","videosession":null,"resource":null,"messagemedia":[],"user":{"id":6056544,"firstname":"Wynn","lastname":"Liang","username":null,"locale":null,"lastseen":"2024-10-03 04:30:18","profilepic":true,"status":0,"created_at":"2024-08-01 21:33:45","updated_at":"2024-10-03 04:15:46","deactivated_at":null,"email_verified_at":"2024-09-14 21:49:53","phone_verified_at":null,"isverified":false,"dropinorder":0,"maxstreams":10,"autotranslate":false,"isonline":true,"lastpresencetime":"2024-10-03 03:44:01","acceptedtos":"2024-09-14 21:49:53","sentwelcomemsg":"2024-08-09 17:13:13","role":"user","mute":false,"muteuntil":null,"isbot":0,"fullname":"Wynn Liang","hasactivity":true,"inactive":false,"language":"en","permissions":{"change_name":"system","change_email":"system","change_phone":"system","remove_user":"system","change_title":"admin","change_pronouns":"admin","change_own_name":false,"change_own_email":false,"change_own_phone":false,"change_own_title":true,"change_own_pronouns":true},"profilepicpath":"\/files\/users\/6056544\/profilepic?pronto_time=1725836412","profilepicurl":"https:\/\/files.chat.trypronto.com\/files\/users\/6056544\/profilepic?pronto_time=1725836412"},"task":null,"messagetrans":[],"mentions":[]},"highlight":{"messagelangs":{"en":["im taking a generation <pronto_hl>test<\/pronto_hl>"]},"message":["im taking a generation <pronto_hl>test<\/pronto_hl>"]}},{"message_id":90502596,"message":{"id":90502596,"bubble_id":3832006,"user_id":6056987,"message":"i got a 0 for F on the online test so i have no feelings :)","resource_id":null,"clickcount":0,"likecount":0,"dislikecount":0,"viewcount":0,"version":0,"user_edited_version":0,"user_edited_at":null,"created_at":"2024-10-02 00:45:10","updated_at":"2024-10-02 00:45:10","livestream_id":null,"videosession_id":null,"systemmessageparts":null,"uuid":"973d5269-00af-4688-88a1-7b968399f79d","task_id":null,"parentmessage_id":90501681,"firstchildmessage_id":null,"lastchildmessage_id":null,"systemevent":null,"reactionsummary":[],"lang":"en","videosession":null,"resource":null,"messagemedia":[],"user":{"id":6056987,"firstname":"Bobby","lastname":"Wan","username":null,"locale":null,"lastseen":"2024-10-03 04:22:58","profilepic":true,"status":0,"created_at":"2024-08-02 04:16:16","updated_at":"2024-10-03 04:23:17","deactivated_at":null,"email_verified_at":"2024-08-21 04:23:12","phone_verified_at":null,"isverified":false,"dropinorder":0,"maxstreams":10,"autotranslate":false,"isonline":false,"lastpresencetime":"2024-10-03 04:23:17","acceptedtos":"2024-08-21 04:23:12","sentwelcomemsg":"2024-08-14 00:55:57","role":"user","mute":false,"muteuntil":null,"isbot":0,"fullname":"Bobby Wan","hasactivity":true,"inactive":false,"language":"en","permissions":{"change_name":"system","change_email":"system","change_phone":"system","remove_user":"system","change_title":"admin","change_pronouns":"admin","change_own_name":false,"change_own_email":false,"change_own_phone":false,"change_own_title":true,"change_own_pronouns":true},"profilepicpath":"\/files\/users\/6056987\/profilepic?pronto_time=1727050206","profilepicurl":"https:\/\/files.chat.trypronto.com\/files\/users\/6056987\/profilepic?pronto_time=1727050206"},"task":null,"messagetrans":[],"mentions":[]},"highlight":{"messagelangs":{"en":["i got a 0 for F on the online <pronto_hl>test<\/pronto_hl> so i have no feelings :)"]},"message":["i got a 0 for F on the online <pronto_hl>test<\/pronto_hl> so i have no feelings :)"]}},{"message_id":90502281,"message":{"id":90502281,"bubble_id":3832006,"user_id":5301875,"message":"BUT THE STUPID FRICKING TEST ONLINE","resource_id":null,"clickcount":0,"likecount":0,"dislikecount":0,"viewcount":0,"version":0,"user_edited_version":0,"user_edited_at":null,"created_at":"2024-10-02 00:41:34","updated_at":"2024-10-02 00:41:35","livestream_id":null,"videosession_id":null,"systemmessageparts":null,"uuid":"4cf11c6b-5562-4045-9b3a-352b28d2c3e0","task_id":null,"parentmessage_id":90501681,"firstchildmessage_id":null,"lastchildmessage_id":null,"systemevent":null,"reactionsummary":[],"lang":"en","videosession":null,"resource":null,"messagemedia":[],"user":{"id":5301875,"firstname":"Vidhu","lastname":"Anand","username":null,"locale":"","lastseen":"2024-10-03 02:55:38","profilepic":true,"status":0,"created_at":"2023-08-04 00:33:18","updated_at":"2024-10-03 02:55:53","deactivated_at":null,"email_verified_at":"2024-08-23 10:34:59","phone_verified_at":null,"isverified":false,"dropinorder":0,"maxstreams":10,"autotranslate":false,"isonline":false,"lastpresencetime":"2024-10-03 02:55:53","acceptedtos":"2024-08-23 10:34:59","sentwelcomemsg":"2023-08-04 15:07:01","role":"user","mute":false,"muteuntil":null,"isbot":0,"fullname":"Vidhu Anand","hasactivity":true,"inactive":false,"language":"en","permissions":{"change_name":"system","change_email":"system","change_phone":"system","remove_user":"system","change_title":"admin","change_pronouns":"admin","change_own_name":false,"change_own_email":false,"change_own_phone":false,"change_own_title":true,"change_own_pronouns":true},"profilepicpath":"\/files\/users\/5301875\/profilepic?pronto_time=1726287476","profilepicurl":"https:\/\/files.chat.trypronto.com\/files\/users\/5301875\/profilepic?pronto_time=1726287476"},"task":null,"messagetrans":[],"mentions":[]},"highlight":{"messagelangs":{"en":["BUT THE STUPID FRICKING <pronto_hl>TEST<\/pronto_hl> ONLINE"]},"message":["BUT THE STUPID FRICKING <pronto_hl>TEST<\/pronto_hl> ONLINE"]}},{"message_id":90498257,"message":{"id":90498257,"bubble_id":3832006,"user_id":6056987,"message":"i just retook the test again","resource_id":null,"clickcount":0,"likecount":0,"dislikecount":0,"viewcount":0,"version":0,"user_edited_version":0,"user_edited_at":null,"created_at":"2024-10-02 00:03:50","updated_at":"2024-10-02 00:03:50","livestream_id":null,"videosession_id":null,"systemmessageparts":null,"uuid":"41c44e7c-b386-4961-bdc3-4ec531128bab","task_id":null,"parentmessage_id":90497980,"firstchildmessage_id":null,"lastchildmessage_id":null,"systemevent":null,"reactionsummary":[],"lang":"en","videosession":null,"resource":null,"messagemedia":[],"user":{"id":6056987,"firstname":"Bobby","lastname":"Wan","username":null,"locale":null,"lastseen":"2024-10-03 04:22:58","profilepic":true,"status":0,"created_at":"2024-08-02 04:16:16","updated_at":"2024-10-03 04:23:17","deactivated_at":null,"email_verified_at":"2024-08-21 04:23:12","phone_verified_at":null,"isverified":false,"dropinorder":0,"maxstreams":10,"autotranslate":false,"isonline":false,"lastpresencetime":"2024-10-03 04:23:17","acceptedtos":"2024-08-21 04:23:12","sentwelcomemsg":"2024-08-14 00:55:57","role":"user","mute":false,"muteuntil":null,"isbot":0,"fullname":"Bobby Wan","hasactivity":true,"inactive":false,"language":"en","permissions":{"change_name":"system","change_email":"system","change_phone":"system","remove_user":"system","change_title":"admin","change_pronouns":"admin","change_own_name":false,"change_own_email":false,"change_own_phone":false,"change_own_title":true,"change_own_pronouns":true},"profilepicpath":"\/files\/users\/6056987\/profilepic?pronto_time=1727050206","profilepicurl":"https:\/\/files.chat.trypronto.com\/files\/users\/6056987\/profilepic?pronto_time=1727050206"},"task":null,"messagetrans":[],"mentions":[]},"highlight":{"messagelangs":{"en":["i just retook the <pronto_hl>test<\/pronto_hl> again"]},"message":["i just retook the <pronto_hl>test<\/pronto_hl> again"]}},{"message_id":90497990,"message":{"id":90497990,"bubble_id":3832006,"user_id":6056987,"message":"i just retook the test and got that again","resource_id":null,"clickcount":0,"likecount":0,"dislikecount":0,"viewcount":0,"version":0,"user_edited_version":0,"user_edited_at":null,"created_at":"2024-10-02 00:01:25","updated_at":"2024-10-02 00:01:26","livestream_id":null,"videosession_id":null,"systemmessageparts":null,"uuid":"6c544bda-ef6d-4905-b0bb-400bc0ea7259","task_id":null,"parentmessage_id":90497532,"firstchildmessage_id":null,"lastchildmessage_id":null,"systemevent":null,"reactionsummary":[],"lang":"en","videosession":null,"resource":null,"messagemedia":[],"user":{"id":6056987,"firstname":"Bobby","lastname":"Wan","username":null,"locale":null,"lastseen":"2024-10-03 04:22:58","profilepic":true,"status":0,"created_at":"2024-08-02 04:16:16","updated_at":"2024-10-03 04:23:17","deactivated_at":null,"email_verified_at":"2024-08-21 04:23:12","phone_verified_at":null,"isverified":false,"dropinorder":0,"maxstreams":10,"autotranslate":false,"isonline":false,"lastpresencetime":"2024-10-03 04:23:17","acceptedtos":"2024-08-21 04:23:12","sentwelcomemsg":"2024-08-14 00:55:57","role":"user","mute":false,"muteuntil":null,"isbot":0,"fullname":"Bobby Wan","hasactivity":true,"inactive":false,"language":"en","permissions":{"change_name":"system","change_email":"system","change_phone":"system","remove_user":"system","change_title":"admin","change_pronouns":"admin","change_own_name":false,"change_own_email":false,"change_own_phone":false,"change_own_title":true,"change_own_pronouns":true},"profilepicpath":"\/files\/users\/6056987\/profilepic?pronto_time=1727050206","profilepicurl":"https:\/\/files.chat.trypronto.com\/files\/users\/6056987\/profilepic?pronto_time=1727050206"},"task":null,"messagetrans":[],"mentions":[]},"highlight":{"messagelangs":{"en":["i just retook the <pronto_hl>test<\/pronto_hl> and got that again"]},"message":["i just retook the <pronto_hl>test<\/pronto_hl> and got that again"]}},{"message_id":90497207,"message":{"id":90497207,"bubble_id":3832006,"user_id":6056987,"message":"found it:https:\/\/www.truity.com\/test\/generation","resource_id":946355,"clickcount":0,"likecount":0,"dislikecount":0,"viewcount":0,"version":1,"user_edited_version":0,"user_edited_at":null,"created_at":"2024-10-01 23:53:48","updated_at":"2024-10-01 23:53:51","livestream_id":null,"videosession_id":null,"systemmessageparts":null,"uuid":"4fc337dc-3224-480a-89cc-7fe47accf41d","task_id":null,"parentmessage_id":null,"firstchildmessage_id":null,"lastchildmessage_id":null,"systemevent":null,"reactionsummary":[],"lang":"en","videosession":null,"resource":{"id":946355,"providername":"Truity","providerurl":"https:\/\/www.truity.com","snippet":"Are you an old soul, or young at heart? If you\u2019ve ever read about your generation\u2019s stereotype and felt it didn\u2019t quite fit you, this quiz is for you. We assess era-specific values and beliefs to help you uncover how your birth year connects with your personality\u2014and where you truly fit in.","url":"https:\/\/www.truity.com\/test\/generation","thumbnailurl":"https:\/\/d31u95r9ywbjex.cloudfront.net\/sites\/default\/files\/test_images\/header_generations.png","thumbnailheight":500,"thumbnailwidth":1200,"imageurl":null,"videourl":null,"videoheight":0,"videowidth":0,"title":"What\u2019s Your True Generation?","externalid":"https:\/\/www.truity.com\/test\/generation","meta":null,"clickcount":0,"likecount":0,"dislikecount":0,"viewcount":0,"embedly":false,"publisheddate":"2024-10-01 23:53:51","created_at":"2024-10-01 23:53:51","updated_at":"2024-10-01 23:53:51","serviceprovider":"Microlink","resourcetype":"LINK"},"messagemedia":[],"user":{"id":6056987,"firstname":"Bobby","lastname":"Wan","username":null,"locale":null,"lastseen":"2024-10-03 04:22:58","profilepic":true,"status":0,"created_at":"2024-08-02 04:16:16","updated_at":"2024-10-03 04:23:17","deactivated_at":null,"email_verified_at":"2024-08-21 04:23:12","phone_verified_at":null,"isverified":false,"dropinorder":0,"maxstreams":10,"autotranslate":false,"isonline":false,"lastpresencetime":"2024-10-03 04:23:17","acceptedtos":"2024-08-21 04:23:12","sentwelcomemsg":"2024-08-14 00:55:57","role":"user","mute":false,"muteuntil":null,"isbot":0,"fullname":"Bobby Wan","hasactivity":true,"inactive":false,"language":"en","permissions":{"change_name":"system","change_email":"system","change_phone":"system","remove_user":"system","change_title":"admin","change_pronouns":"admin","change_own_name":false,"change_own_email":false,"change_own_phone":false,"change_own_title":true,"change_own_pronouns":true},"profilepicpath":"\/files\/users\/6056987\/profilepic?pronto_time=1727050206","profilepicurl":"https:\/\/files.chat.trypronto.com\/files\/users\/6056987\/profilepic?pronto_time=1727050206"},"task":null,"messagetrans":[],"mentions":[]},"highlight":{"messagelangs":{"en":["found it:https:\/\/www.truity.com\/<pronto_hl>test<\/pronto_hl>\/generation"]},"message":["found it:https:\/\/www.truity.com\/<pronto_hl>test<\/pronto_hl>\/generation"]}},{"message_id":90497177,"message":{"id":90497177,"bubble_id":3832006,"user_id":6056987,"message":"i forgor just search generation test in google","resource_id":null,"clickcount":0,"likecount":0,"dislikecount":0,"viewcount":0,"version":0,"user_edited_version":0,"user_edited_at":null,"created_at":"2024-10-01 23:53:28","updated_at":"2024-10-01 23:53:29","livestream_id":null,"videosession_id":null,"systemmessageparts":null,"uuid":"32f5426b-2876-458c-9aeb-1a762171b1a8","task_id":null,"parentmessage_id":null,"firstchildmessage_id":null,"lastchildmessage_id":null,"systemevent":null,"reactionsummary":[],"lang":"en","videosession":null,"resource":null,"messagemedia":[],"user":{"id":6056987,"firstname":"Bobby","lastname":"Wan","username":null,"locale":null,"lastseen":"2024-10-03 04:22:58","profilepic":true,"status":0,"created_at":"2024-08-02 04:16:16","updated_at":"2024-10-03 04:23:17","deactivated_at":null,"email_verified_at":"2024-08-21 04:23:12","phone_verified_at":null,"isverified":false,"dropinorder":0,"maxstreams":10,"autotranslate":false,"isonline":false,"lastpresencetime":"2024-10-03 04:23:17","acceptedtos":"2024-08-21 04:23:12","sentwelcomemsg":"2024-08-14 00:55:57","role":"user","mute":false,"muteuntil":null,"isbot":0,"fullname":"Bobby Wan","hasactivity":true,"inactive":false,"language":"en","permissions":{"change_name":"system","change_email":"system","change_phone":"system","remove_user":"system","change_title":"admin","change_pronouns":"admin","change_own_name":false,"change_own_email":false,"change_own_phone":false,"change_own_title":true,"change_own_pronouns":true},"profilepicpath":"\/files\/users\/6056987\/profilepic?pronto_time=1727050206","profilepicurl":"https:\/\/files.chat.trypronto.com\/files\/users\/6056987\/profilepic?pronto_time=1727050206"},"task":null,"messagetrans":[],"mentions":[]},"highlight":{"messagelangs":{"en":["i forgor just search generation <pronto_hl>test<\/pronto_hl> in google"]},"message":["i forgor just search generation <pronto_hl>test<\/pronto_hl> in google"]}},{"message_id":90497148,"message":{"id":90497148,"bubble_id":3832006,"user_id":6056568,"message":"send me that test","resource_id":null,"clickcount":0,"likecount":0,"dislikecount":0,"viewcount":0,"version":0,"user_edited_version":0,"user_edited_at":null,"created_at":"2024-10-01 23:53:07","updated_at":"2024-10-01 23:53:08","livestream_id":null,"videosession_id":null,"systemmessageparts":null,"uuid":"7682003f-a70c-4ecc-9582-0d85e4fdd4ed","task_id":null,"parentmessage_id":null,"firstchildmessage_id":null,"lastchildmessage_id":null,"systemevent":null,"reactionsummary":[],"lang":"en","videosession":null,"resource":null,"messagemedia":[],"user":{"id":6056568,"firstname":"Kieren","lastname":"Rao","username":null,"locale":null,"lastseen":"2024-10-03 04:35:34","profilepic":true,"status":0,"created_at":"2024-08-01 21:33:48","updated_at":"2024-10-03 04:27:34","deactivated_at":null,"email_verified_at":null,"phone_verified_at":null,"isverified":false,"dropinorder":0,"maxstreams":10,"autotranslate":false,"isonline":true,"lastpresencetime":"2024-10-03 03:21:48","acceptedtos":"2024-08-05 19:51:27","sentwelcomemsg":"2024-08-05 19:51:27","role":"user","mute":false,"muteuntil":null,"isbot":0,"fullname":"Kieren Rao","hasactivity":true,"inactive":false,"language":"en","permissions":{"change_name":"system","change_email":"system","change_phone":"system","remove_user":"system","change_title":"admin","change_pronouns":"admin","change_own_name":false,"change_own_email":false,"change_own_phone":false,"change_own_title":true,"change_own_pronouns":true},"profilepicpath":"\/files\/users\/6056568\/profilepic?pronto_time=1727923904","profilepicurl":"https:\/\/files.chat.trypronto.com\/files\/users\/6056568\/profilepic?pronto_time=1727923904"},"task":null,"messagetrans":[],"mentions":[]},"highlight":{"messagelangs":{"en":["send me that <pronto_hl>test<\/pronto_hl>"]},"message":["send me that <pronto_hl>test<\/pronto_hl>"]}},{"message_id":90497112,"message":{"id":90497112,"bubble_id":3832006,"user_id":6056987,"message":"i took a generation test and it said i was born 1927","resource_id":null,"clickcount":0,"likecount":0,"dislikecount":0,"viewcount":0,"version":0,"user_edited_version":0,"user_edited_at":null,"created_at":"2024-10-01 23:52:47","updated_at":"2024-10-01 23:52:48","livestream_id":null,"videosession_id":null,"systemmessageparts":null,"uuid":"50743f6e-b9b3-4476-a26b-067c9bb59691","task_id":null,"parentmessage_id":null,"firstchildmessage_id":null,"lastchildmessage_id":null,"systemevent":null,"reactionsummary":[],"lang":"en","videosession":null,"resource":null,"messagemedia":[],"user":{"id":6056987,"firstname":"Bobby","lastname":"Wan","username":null,"locale":null,"lastseen":"2024-10-03 04:22:58","profilepic":true,"status":0,"created_at":"2024-08-02 04:16:16","updated_at":"2024-10-03 04:23:17","deactivated_at":null,"email_verified_at":"2024-08-21 04:23:12","phone_verified_at":null,"isverified":false,"dropinorder":0,"maxstreams":10,"autotranslate":false,"isonline":false,"lastpresencetime":"2024-10-03 04:23:17","acceptedtos":"2024-08-21 04:23:12","sentwelcomemsg":"2024-08-14 00:55:57","role":"user","mute":false,"muteuntil":null,"isbot":0,"fullname":"Bobby Wan","hasactivity":true,"inactive":false,"language":"en","permissions":{"change_name":"system","change_email":"system","change_phone":"system","remove_user":"system","change_title":"admin","change_pronouns":"admin","change_own_name":false,"change_own_email":false,"change_own_phone":false,"change_own_title":true,"change_own_pronouns":true},"profilepicpath":"\/files\/users\/6056987\/profilepic?pronto_time=1727050206","profilepicurl":"https:\/\/files.chat.trypronto.com\/files\/users\/6056987\/profilepic?pronto_time=1727050206"},"task":null,"messagetrans":[],"mentions":[]},"highlight":{"messagelangs":{"en":["i took a generation <pronto_hl>test<\/pronto_hl> and it said i was born 1927"]},"message":["i took a generation <pronto_hl>test<\/pronto_hl> and it said i was born 1927"]}},{"message_id":90478736,"message":{"id":90478736,"bubble_id":3835919,"user_id":5302428,"message":"I was testing missile on a sandbox","resource_id":null,"clickcount":0,"likecount":0,"dislikecount":0,"viewcount":0,"version":0,"user_edited_version":0,"user_edited_at":null,"created_at":"2024-10-01 21:30:17","updated_at":"2024-10-01 21:30:17","livestream_id":null,"videosession_id":null,"systemmessageparts":null,"uuid":"bfaafb5a-22ba-4163-addd-063f6e68b2c7","task_id":null,"parentmessage_id":null,"firstchildmessage_id":null,"lastchildmessage_id":null,"systemevent":null,"reactionsummary":[],"lang":"en","videosession":null,"resource":null,"messagemedia":[],"user":{"id":5302428,"firstname":"Ashwin","lastname":"Naren","username":null,"locale":"en_US","lastseen":"2024-10-03 04:35:47","profilepic":true,"status":0,"created_at":"2023-08-04 00:44:12","updated_at":"2024-10-03 04:35:18","deactivated_at":null,"email_verified_at":"2024-09-25 02:40:01","phone_verified_at":null,"isverified":false,"dropinorder":0,"maxstreams":10,"autotranslate":false,"isonline":true,"lastpresencetime":"2024-10-03 04:35:18","acceptedtos":"2024-09-25 02:40:01","sentwelcomemsg":"2023-08-15 19:22:02","role":"user","mute":false,"muteuntil":null,"isbot":0,"fullname":"Ashwin Naren","hasactivity":true,"inactive":false,"language":"en","permissions":{"change_name":"system","change_email":"system","change_phone":"system","remove_user":"system","change_title":"admin","change_pronouns":"admin","change_own_name":false,"change_own_email":false,"change_own_phone":false,"change_own_title":true,"change_own_pronouns":true},"profilepicpath":"\/files\/users\/5302428\/profilepic?pronto_time=1695523284","profilepicurl":"https:\/\/files.chat.trypronto.com\/files\/users\/5302428\/profilepic?pronto_time=1695523284"},"task":null,"messagetrans":[],"mentions":[]},"highlight":{"messagelangs":{"en":["I was <pronto_hl>testing<\/pronto_hl> missile on a sandbox"]}}},{"message_id":90448630,"message":{"id":90448630,"bubble_id":3832003,"user_id":6056572,"message":"the \"test for round 1\" is the qualifier basically, right??","resource_id":null,"clickcount":0,"likecount":0,"dislikecount":0,"viewcount":0,"version":0,"user_edited_version":0,"user_edited_at":null,"created_at":"2024-10-01 18:04:28","updated_at":"2024-10-01 18:04:29","livestream_id":null,"videosession_id":null,"systemmessageparts":null,"uuid":"00988ad5-93af-4c23-b6a4-9242b396556e","task_id":null,"parentmessage_id":null,"firstchildmessage_id":null,"lastchildmessage_id":null,"systemevent":null,"reactionsummary":[],"lang":"en","videosession":null,"resource":null,"messagemedia":[],"user":{"id":6056572,"firstname":"Aydyn","lastname":"Sathyamoorthy","username":null,"locale":null,"lastseen":"2024-10-03 04:25:18","profilepic":true,"status":0,"created_at":"2024-08-01 21:33:49","updated_at":"2024-10-03 04:27:12","deactivated_at":null,"email_verified_at":"2024-09-30 01:25:39","phone_verified_at":null,"isverified":false,"dropinorder":0,"maxstreams":10,"autotranslate":false,"isonline":false,"lastpresencetime":"2024-10-03 04:27:12","acceptedtos":"2024-09-30 01:25:39","sentwelcomemsg":"2024-08-18 01:12:22","role":"user","mute":true,"muteuntil":null,"isbot":0,"fullname":"Aydyn Sathyamoorthy","hasactivity":true,"inactive":false,"language":"en","permissions":{"change_name":"system","change_email":"system","change_phone":"system","remove_user":"system","change_title":"admin","change_pronouns":"admin","change_own_name":false,"change_own_email":false,"change_own_phone":false,"change_own_title":true,"change_own_pronouns":true},"profilepicpath":"\/files\/users\/6056572\/profilepic?pronto_time=1724639204","profilepicurl":"https:\/\/files.chat.trypronto.com\/files\/users\/6056572\/profilepic?pronto_time=1724639204"},"task":null,"messagetrans":[],"mentions":[]},"highlight":{"messagelangs":{"en":["the \"<pronto_hl>test<\/pronto_hl> for round 1\" is the qualifier basically, right??"]},"message":["the \"<pronto_hl>test<\/pronto_hl> for round 1\" is the qualifier basically, right??"]}},{"message_id":90398149,"message":{"id":90398149,"bubble_id":3832003,"user_id":5302595,"message":"We are so excited to have OHS participate in the Latin League this year! Thank you all for signing up and thank you especially to Aydyn for showing interest in Certamen :) \n\nWe have registered two groups in the Latin League: \u201cPurple\u201d and \u201cGold.\u201d The names are arbitrary, and both are Varsity high school teams. The first round is coming up on October 9th-16th. \n\nAnnouncements: \u2028\n1.) We need to decide who is on what team. (Keeping in mind that each team has a max of 7 members, 5 of who participate in each testing round). Due to the number of participants we have, it is likely that we will have one group of 6 and another group of 7. Once decided, these teams will remain for the rest of the year (ie: there will be no switching between the two groups). \u2028\n2.) We need to choose one day (and a 45-minute period) in which BOTH teams log on to take the test for Round 1 in breakout rooms. (This would most likely be a Friday within the October 9-16th testing time but we can see what time works best). \u2028\n\nTo accomplish these things, please finish the following: \n1.) Please introduce yourselves to the chat and include your background in Latin (ie: what level you are at) and any experience you have with Certamen by filling in the google sheet attached. Please also indicate what time you are available on October 11th to take the exam (if possible, please include multiple times so that we can find the best option that works for everyone).  https:\/\/docs.google.com\/spreadsheets\/d\/1FHo5N821J_pEDtYtooYUf39Evk4jKprSFBSP34x8oFk\/edit?gid=0#gid=0\n\nThis will be the first step in determining who is on what team so please fill this out as soon as you can!!","resource_id":null,"clickcount":0,"likecount":0,"dislikecount":0,"viewcount":0,"version":3,"user_edited_version":0,"user_edited_at":null,"created_at":"2024-10-01 03:05:35","updated_at":"2024-10-01 12:34:47","livestream_id":null,"videosession_id":null,"systemmessageparts":null,"uuid":"fd724aae-a6f5-4284-aa69-3edfbdb3ed03","task_id":null,"parentmessage_id":null,"firstchildmessage_id":null,"lastchildmessage_id":null,"systemevent":null,"reactionsummary":[{"reactiontype_id":1,"count":2,"users":[5302454,6056312]},{"reactiontype_id":4,"count":1,"users":[5302465]}],"lang":"en","videosession":null,"resource":null,"messagemedia":[],"user":{"id":5302595,"firstname":"Hannah","lastname":"Nakagome","username":null,"locale":"","lastseen":"2024-10-03 04:07:29","profilepic":true,"status":0,"created_at":"2023-08-04 00:44:26","updated_at":"2024-10-03 04:08:39","deactivated_at":null,"email_verified_at":"2024-09-27 15:09:08","phone_verified_at":null,"isverified":false,"dropinorder":0,"maxstreams":10,"autotranslate":false,"isonline":false,"lastpresencetime":"2024-10-03 04:08:39","acceptedtos":"2024-09-27 15:09:08","sentwelcomemsg":"2023-08-06 02:39:57","role":"user","mute":false,"muteuntil":null,"isbot":0,"fullname":"Hannah Nakagome","hasactivity":true,"inactive":false,"language":"en","permissions":{"change_name":"system","change_email":"system","change_phone":"system","remove_user":"system","change_title":"admin","change_pronouns":"admin","change_own_name":false,"change_own_email":false,"change_own_phone":false,"change_own_title":true,"change_own_pronouns":true},"profilepicpath":"\/files\/users\/5302595\/profilepic?pronto_time=1691608049","profilepicurl":"https:\/\/files.chat.trypronto.com\/files\/users\/5302595\/profilepic?pronto_time=1691608049"},"task":null,"messagetrans":[],"mentions":[]},"highlight":{"messagelangs":{"en":["\u2026(Keeping in mind that each team has a max of 7 members, 5 of who participate in each <pronto_hl>testing<\/pronto_hl> round).\u2026"]},"message":["\u2026We need to choose one day (and a 45-minute period) in which BOTH teams log on to take the <pronto_hl>test<\/pronto_hl> for Round 1 in breakout rooms.\u2026"]}},{"message_id":90397349,"message":{"id":90397349,"bubble_id":2828820,"user_id":5302366,"message":"dang health has a test?","resource_id":null,"clickcount":0,"likecount":0,"dislikecount":0,"viewcount":0,"version":10,"user_edited_version":0,"user_edited_at":null,"created_at":"2024-10-01 02:56:37","updated_at":"2024-10-01 13:33:15","livestream_id":null,"videosession_id":null,"systemmessageparts":null,"uuid":"f3f610f9-2ebc-493c-bcec-36c2769e11ce","task_id":null,"parentmessage_id":null,"firstchildmessage_id":90397406,"lastchildmessage_id":90414961,"systemevent":null,"reactionsummary":[],"lang":"en","videosession":null,"resource":null,"messagemedia":[],"user":{"id":5302366,"firstname":"Lujia","lastname":"Dong","username":null,"locale":"","lastseen":"2024-10-03 04:22:55","profilepic":true,"status":0,"created_at":"2023-08-04 00:44:04","updated_at":"2024-10-03 04:23:25","deactivated_at":null,"email_verified_at":"2024-07-12 18:01:38","phone_verified_at":null,"isverified":false,"dropinorder":0,"maxstreams":10,"autotranslate":false,"isonline":false,"lastpresencetime":"2024-10-03 04:23:25","acceptedtos":"2024-07-12 18:01:38","sentwelcomemsg":null,"role":"user","mute":true,"muteuntil":null,"isbot":0,"fullname":"Lujia Dong","hasactivity":true,"inactive":false,"language":"en","permissions":{"change_name":"system","change_email":"system","change_phone":"system","remove_user":"system","change_title":"admin","change_pronouns":"admin","change_own_name":false,"change_own_email":false,"change_own_phone":false,"change_own_title":true,"change_own_pronouns":true},"profilepicpath":"\/files\/users\/5302366\/profilepic?pronto_time=1727078226","profilepicurl":"https:\/\/files.chat.trypronto.com\/files\/users\/5302366\/profilepic?pronto_time=1727078226"},"task":null,"messagetrans":[],"mentions":[]},"highlight":{"messagelangs":{"en":["dang health has a <pronto_hl>test<\/pronto_hl>?"]},"message":["dang health has a <pronto_hl>test<\/pronto_hl>?"]}},{"message_id":90386643,"message":{"id":90386643,"bubble_id":3832006,"user_id":5301950,"message":"<@6056523> what are you, randomly trying to conduct a turing test","resource_id":null,"clickcount":0,"likecount":0,"dislikecount":0,"viewcount":0,"version":0,"user_edited_version":0,"user_edited_at":null,"created_at":"2024-10-01 00:59:19","updated_at":"2024-10-01 00:59:20","livestream_id":null,"videosession_id":null,"systemmessageparts":null,"uuid":"c9f870a2-2609-4a13-9d93-c0da96f36b56","task_id":null,"parentmessage_id":90386537,"firstchildmessage_id":null,"lastchildmessage_id":null,"systemevent":null,"reactionsummary":[],"lang":"en","videosession":null,"resource":null,"messagemedia":[],"user":{"id":5301950,"firstname":"Aimee","lastname":"Burmeister","username":null,"locale":"en_US","lastseen":"2024-10-03 04:33:08","profilepic":true,"status":0,"created_at":"2023-08-04 00:33:26","updated_at":"2024-10-03 04:33:34","deactivated_at":null,"email_verified_at":"2024-06-23 23:41:47","phone_verified_at":null,"isverified":false,"dropinorder":0,"maxstreams":10,"autotranslate":false,"isonline":false,"lastpresencetime":"2024-10-03 04:33:34","acceptedtos":"2024-06-23 23:41:45","sentwelcomemsg":null,"role":"user","mute":false,"muteuntil":null,"isbot":0,"fullname":"Aimee Burmeister","hasactivity":true,"inactive":false,"language":"en","permissions":{"change_name":"system","change_email":"system","change_phone":"system","remove_user":"system","change_title":"admin","change_pronouns":"admin","change_own_name":false,"change_own_email":false,"change_own_phone":false,"change_own_title":true,"change_own_pronouns":true},"profilepicpath":"\/files\/users\/5301950\/profilepic?pronto_time=1727048212","profilepicurl":"https:\/\/files.chat.trypronto.com\/files\/users\/5301950\/profilepic?pronto_time=1727048212"},"task":null,"messagetrans":[],"mentions":[{"id":6056523,"firstname":"Jackson","lastname":"He","profilepic":true,"deactivated_at":null,"created_at":"2024-08-01 21:33:42","updated_at":"2024-10-03 04:35:45","fullname":"Jackson He","hasactivity":false,"inactive":false,"language":null,"permissions":{"change_name":"admin","change_email":"admin","change_phone":"admin","remove_user":"admin","change_title":"admin","change_pronouns":"admin","change_own_name":true,"change_own_email":true,"change_own_phone":true,"change_own_title":true,"change_own_pronouns":true},"profilepicpath":"\/files\/users\/6056523\/profilepic?pronto_time=1727044449","profilepicurl":"https:\/\/files.chat.trypronto.com\/files\/users\/6056523\/profilepic?pronto_time=1727044449"}]},"highlight":{"messagelangs":{"en":["@Jackson He what are you, randomly trying to conduct a turing <pronto_hl>test<\/pronto_hl>"]},"message":["@Jackson He what are you, randomly trying to conduct a turing <pronto_hl>test<\/pronto_hl>"]}},{"message_id":90319137,"message":{"id":90319137,"bubble_id":3756933,"user_id":5302428,"message":"try not to kill me since I'm testing","resource_id":null,"clickcount":0,"likecount":0,"dislikecount":0,"viewcount":0,"version":0,"user_edited_version":0,"user_edited_at":null,"created_at":"2024-09-30 16:26:23","updated_at":"2024-09-30 16:26:24","livestream_id":null,"videosession_id":null,"systemmessageparts":null,"uuid":"189a8c73-1a12-4cdf-8676-679a115c15ee","task_id":null,"parentmessage_id":null,"firstchildmessage_id":null,"lastchildmessage_id":null,"systemevent":null,"reactionsummary":[],"lang":"en","videosession":null,"resource":null,"messagemedia":[],"user":{"id":5302428,"firstname":"Ashwin","lastname":"Naren","username":null,"locale":"en_US","lastseen":"2024-10-03 04:35:47","profilepic":true,"status":0,"created_at":"2023-08-04 00:44:12","updated_at":"2024-10-03 04:35:18","deactivated_at":null,"email_verified_at":"2024-09-25 02:40:01","phone_verified_at":null,"isverified":false,"dropinorder":0,"maxstreams":10,"autotranslate":false,"isonline":true,"lastpresencetime":"2024-10-03 04:35:18","acceptedtos":"2024-09-25 02:40:01","sentwelcomemsg":"2023-08-15 19:22:02","role":"user","mute":false,"muteuntil":null,"isbot":0,"fullname":"Ashwin Naren","hasactivity":true,"inactive":false,"language":"en","permissions":{"change_name":"system","change_email":"system","change_phone":"system","remove_user":"system","change_title":"admin","change_pronouns":"admin","change_own_name":false,"change_own_email":false,"change_own_phone":false,"change_own_title":true,"change_own_pronouns":true},"profilepicpath":"\/files\/users\/5302428\/profilepic?pronto_time=1695523284","profilepicurl":"https:\/\/files.chat.trypronto.com\/files\/users\/5302428\/profilepic?pronto_time=1695523284"},"task":null,"messagetrans":[],"mentions":[]},"highlight":{"messagelangs":{"en":["try not to kill me since I'm <pronto_hl>testing<\/pronto_hl>"]}}},{"message_id":90081526,"message":{"id":90081526,"bubble_id":2828820,"user_id":5302519,"message":"nvm I was beat to actually testing it","resource_id":null,"clickcount":0,"likecount":0,"dislikecount":0,"viewcount":0,"version":0,"user_edited_version":0,"user_edited_at":null,"created_at":"2024-09-26 23:27:51","updated_at":"2024-09-26 23:27:51","livestream_id":null,"videosession_id":null,"systemmessageparts":null,"uuid":"bc9aa35c-4624-49f1-b81d-857f809d8082","task_id":null,"parentmessage_id":90074578,"firstchildmessage_id":null,"lastchildmessage_id":null,"systemevent":null,"reactionsummary":[],"lang":"en","videosession":null,"resource":null,"messagemedia":[],"user":{"id":5302519,"firstname":"Paul","lastname":"Eastlund","username":null,"locale":"","lastseen":"2024-10-03 04:35:32","profilepic":true,"status":0,"created_at":"2023-08-04 00:44:18","updated_at":"2024-10-03 04:17:46","deactivated_at":null,"email_verified_at":"2024-09-07 01:25:24","phone_verified_at":null,"isverified":false,"dropinorder":0,"maxstreams":10,"autotranslate":false,"isonline":true,"lastpresencetime":"2024-10-03 04:17:00","acceptedtos":"2024-09-07 01:25:24","sentwelcomemsg":null,"role":"user","mute":false,"muteuntil":null,"isbot":0,"fullname":"Paul Eastlund","hasactivity":true,"inactive":false,"language":"en","permissions":{"change_name":"system","change_email":"system","change_phone":"system","remove_user":"system","change_title":"admin","change_pronouns":"admin","change_own_name":false,"change_own_email":false,"change_own_phone":false,"change_own_title":true,"change_own_pronouns":true},"profilepicpath":"\/files\/users\/5302519\/profilepic?pronto_time=1727114475","profilepicurl":"https:\/\/files.chat.trypronto.com\/files\/users\/5302519\/profilepic?pronto_time=1727114475"},"task":null,"messagetrans":[],"mentions":[]},"highlight":{"messagelangs":{"en":["nvm I was beat to actually <pronto_hl>testing<\/pronto_hl> it"]}}},{"message_id":90018155,"message":{"id":90018155,"bubble_id":2828820,"user_id":5302393,"message":"(This happened to my dad, it took him 3 or 4 tests and a ton of money to renew his license. Yes, renew, he had been driving for almost 40 years prior to this)","resource_id":null,"clickcount":0,"likecount":0,"dislikecount":0,"viewcount":0,"version":0,"user_edited_version":0,"user_edited_at":null,"created_at":"2024-09-26 15:32:38","updated_at":"2024-09-26 15:32:38","livestream_id":null,"videosession_id":null,"systemmessageparts":null,"uuid":"C7549ECF-9C72-4445-8CCB-B0D04B2D531F","task_id":null,"parentmessage_id":null,"firstchildmessage_id":null,"lastchildmessage_id":null,"systemevent":null,"reactionsummary":[],"lang":"en","videosession":null,"resource":null,"messagemedia":[],"user":{"id":5302393,"firstname":"Ansti","lastname":"Kalogeropoulos","username":null,"locale":"en_GR","lastseen":"2024-10-02 23:24:59","profilepic":true,"status":0,"created_at":"2023-08-04 00:44:07","updated_at":"2024-10-02 23:15:58","deactivated_at":null,"email_verified_at":"2024-07-31 20:36:24","phone_verified_at":null,"isverified":false,"dropinorder":0,"maxstreams":10,"autotranslate":false,"isonline":false,"lastpresencetime":"2024-10-02 23:15:58","acceptedtos":"2024-07-31 20:36:24","sentwelcomemsg":null,"role":"user","mute":false,"muteuntil":null,"isbot":0,"fullname":"Ansti Kalogeropoulos","hasactivity":true,"inactive":false,"language":"en","permissions":{"change_name":"system","change_email":"system","change_phone":"system","remove_user":"system","change_title":"admin","change_pronouns":"admin","change_own_name":false,"change_own_email":false,"change_own_phone":false,"change_own_title":true,"change_own_pronouns":true},"profilepicpath":"\/files\/users\/5302393\/profilepic?pronto_time=1727893143","profilepicurl":"https:\/\/files.chat.trypronto.com\/files\/users\/5302393\/profilepic?pronto_time=1727893143"},"task":null,"messagetrans":[],"mentions":[]},"highlight":{"messagelangs":{"en":["(This happened to my dad, it took him 3 or 4 <pronto_hl>tests<\/pronto_hl> and a ton of money to renew his license. Yes, renew, he had been driving for almost 40 years prior to this)"]}}},{"message_id":90017971,"message":{"id":90017971,"bubble_id":2828820,"user_id":5302393,"message":"Here you can get a licence at age 18 and you don\u2019t even need to take a test or anything, just illegally pay the driving centre 200 euros or they\u2019ll fail you until you do \ud83d\ude1d","resource_id":null,"clickcount":0,"likecount":0,"dislikecount":0,"viewcount":0,"version":6,"user_edited_version":0,"user_edited_at":null,"created_at":"2024-09-26 15:31:14","updated_at":"2024-09-26 16:43:48","livestream_id":null,"videosession_id":null,"systemmessageparts":null,"uuid":"B22BD1C2-BA7B-4684-B737-A9D5DCE54C8E","task_id":null,"parentmessage_id":null,"firstchildmessage_id":90018025,"lastchildmessage_id":90018025,"systemevent":null,"reactionsummary":[{"reactiontype_id":4,"count":1,"users":[5302367]}],"lang":"en","videosession":null,"resource":null,"messagemedia":[],"user":{"id":5302393,"firstname":"Ansti","lastname":"Kalogeropoulos","username":null,"locale":"en_GR","lastseen":"2024-10-02 23:24:59","profilepic":true,"status":0,"created_at":"2023-08-04 00:44:07","updated_at":"2024-10-02 23:15:58","deactivated_at":null,"email_verified_at":"2024-07-31 20:36:24","phone_verified_at":null,"isverified":false,"dropinorder":0,"maxstreams":10,"autotranslate":false,"isonline":false,"lastpresencetime":"2024-10-02 23:15:58","acceptedtos":"2024-07-31 20:36:24","sentwelcomemsg":null,"role":"user","mute":false,"muteuntil":null,"isbot":0,"fullname":"Ansti Kalogeropoulos","hasactivity":true,"inactive":false,"language":"en","permissions":{"change_name":"system","change_email":"system","change_phone":"system","remove_user":"system","change_title":"admin","change_pronouns":"admin","change_own_name":false,"change_own_email":false,"change_own_phone":false,"change_own_title":true,"change_own_pronouns":true},"profilepicpath":"\/files\/users\/5302393\/profilepic?pronto_time=1727893143","profilepicurl":"https:\/\/files.chat.trypronto.com\/files\/users\/5302393\/profilepic?pronto_time=1727893143"},"task":null,"messagetrans":[],"mentions":[]},"highlight":{"messagelangs":{"en":["Here you can get a licence at age 18 and you don\u2019t even need to take a <pronto_hl>test<\/pronto_hl> or anything, just illegally pay the driving centre 200 euros or they\u2019ll fail you until you do \ud83d\ude1d"]},"message":["Here you can get a licence at age 18 and you don\u2019t even need to take a <pronto_hl>test<\/pronto_hl> or anything, just illegally pay the driving centre 200 euros or they\u2019ll fail you until you do \ud83d\ude1d"]}},{"message_id":90010641,"message":{"id":90010641,"bubble_id":2828820,"user_id":6060658,"message":"I just got my learner's permit too (also in Florida) the test really is super easy. The most annoying part is the 4-hour drugs and alcohol course.","resource_id":null,"clickcount":0,"likecount":0,"dislikecount":0,"viewcount":0,"version":0,"user_edited_version":0,"user_edited_at":null,"created_at":"2024-09-26 14:23:36","updated_at":"2024-09-26 14:23:36","livestream_id":null,"videosession_id":null,"systemmessageparts":null,"uuid":"ca00003b-12e0-4843-9a6c-7a7e763aaf2f","task_id":null,"parentmessage_id":90006870,"firstchildmessage_id":null,"lastchildmessage_id":null,"systemevent":null,"reactionsummary":[],"lang":"en","videosession":null,"resource":null,"messagemedia":[],"user":{"id":6060658,"firstname":"Sam","lastname":"Anes","username":null,"locale":null,"lastseen":"2024-10-03 04:35:04","profilepic":true,"status":0,"created_at":"2024-08-04 06:36:03","updated_at":"2024-10-03 04:26:46","deactivated_at":null,"email_verified_at":"2024-08-31 15:01:50","phone_verified_at":null,"isverified":false,"dropinorder":0,"maxstreams":10,"autotranslate":false,"isonline":true,"lastpresencetime":"2024-10-03 03:13:39","acceptedtos":"2024-08-31 15:01:50","sentwelcomemsg":"2024-08-14 16:26:21","role":"user","mute":false,"muteuntil":null,"isbot":0,"fullname":"Sam Anes","hasactivity":true,"inactive":false,"language":"en","permissions":{"change_name":"system","change_email":"system","change_phone":"system","remove_user":"system","change_title":"admin","change_pronouns":"admin","change_own_name":false,"change_own_email":false,"change_own_phone":false,"change_own_title":true,"change_own_pronouns":true},"profilepicpath":"\/files\/users\/6060658\/profilepic?pronto_time=1724410012","profilepicurl":"https:\/\/files.chat.trypronto.com\/files\/users\/6060658\/profilepic?pronto_time=1724410012"},"task":null,"messagetrans":[],"mentions":[]},"highlight":{"messagelangs":{"en":["I just got my learner's permit too (also in Florida) the <pronto_hl>test<\/pronto_hl> really is super easy. The most annoying part is the 4-hour drugs and alcohol course."]},"message":["I just got my learner's permit too (also in Florida) the <pronto_hl>test<\/pronto_hl> really is super easy. The most annoying part is the 4-hour drugs and alcohol course."]}}],"bubbles":[{"bubble_id":2828820,"ishidden":false,"isdm":false,"title":"\ud83e\uddc3 sophomores that yap a lot"},{"bubble_id":3738656,"ishidden":false,"isdm":true,"title":"Ashwin Naren"},{"bubble_id":3756933,"ishidden":false,"isdm":false,"title":"KSP Circle"},{"bubble_id":3832003,"ishidden":false,"isdm":false,"title":"Latin League 2024-2025"},{"bubble_id":3832006,"ishidden":false,"isdm":false,"title":"\ud83d\udc96 MS and FRIENDS \ud83d\udc96 GANDHI'S BDAY HE'S NOW 155!!!!! \ud83c\udf82\ud83e\udd73"},{"bubble_id":3835919,"ishidden":false,"isdm":true,"title":"Greyson Wyler"}]}
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
        let _response = client.get_current_user_info().await.unwrap();
    }

    #[tokio::test]
    async fn test_get_bubble_list() {
        let client = get_client().await;
        let _response = client.get_bubble_list().await.unwrap();
    }

    #[tokio::test]
    async fn test_get_bubble_info() {
        let client = get_client().await;
        let bubble_list = client.get_bubble_list().await.unwrap();
        let bubble_id = bubble_list.bubbles[0].id;
        let _response = client.get_bubble_info(bubble_id).await.unwrap();
    }

    #[tokio::test]
    async fn test_get_announcement_list() {
        let client = get_client().await;
        client.get_announcement_list().await.unwrap();
    }

    #[tokio::test]
    async fn test_get_bubble_history() {
        let client = get_client().await;
        let bubble_list = client.get_bubble_list().await.unwrap();
        let bubble_id = bubble_list.bubbles[0].id;
        let _response = client.get_bubble_history(bubble_id, None).await.unwrap();
    }
}
