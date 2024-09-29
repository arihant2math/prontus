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
use crate::membership_update::{MembershipUpdateModification, PostMembershipUpdateRequest};
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

    pub async fn set_bubble_alias(&self, bubble_id: u64, alias: String) -> Result<membership_update::PostMembershipUpdateResponse, ResponseError> {
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

    pub async fn post_message(
        &self,
        user_id: u64,
        bubble_id: u64,
        message: String,
        parent_message_id: Option<u64>,
    ) -> Result<message_create::MessageModifyResponse, ResponseError> {
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
