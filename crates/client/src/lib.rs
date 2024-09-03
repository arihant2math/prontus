use std::sync::Arc;

use chrono::Utc;
use reqwest::header::{HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};
use thiserror::Error;

pub use api_error::APIError;
pub use bubble::{Bubble, Category};
pub use bubble_history::{GetBubbleHistoryResponse, Message, MessageMedia};
pub use bubble_info::{GetBubbleInfoResponse};
pub use user_info::UserInfo;
use crate::user_login::DeviceInfo;
use crate::user_token_login::{TokenLoginResponse, TokenLoginResult};

pub mod api_error;
mod bubble;
mod bubble_history;
mod bubble_info;
mod bubble_list;
mod device_ping;
mod message_create;
mod message_delete;
mod message_edit;
mod pusher_auth;
mod reaction_add;
mod reaction_remove;
mod user_info;
pub mod user_login;
pub mod user_token_login;
pub mod user_verify;

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
            socket_id,
            channel_name,
        )
        .await?)
    }

    pub async fn get_user_info(&self) -> Result<user_info::GetUserInfoResponse, ResponseError> {
        Ok(user_info::get(&self.api_base_url, &self.http_client)
            .await?
            .to_result()?)
    }

    pub async fn get_bubble_list(
        &self,
    ) -> Result<bubble_list::GetBubbleListResponse, ResponseError> {
        Ok(bubble_list::get(&self.api_base_url, &self.http_client)
            .await?
            .to_result()?)
    }

    pub async fn get_bubble_info(
        &self,
        bubble_id: u64,
    ) -> Result<bubble_info::GetBubbleInfoResponse, ResponseError> {
        Ok(
            bubble_info::get(&self.api_base_url, &self.http_client, bubble_id)
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
    ) -> Result<message_create::MessageModifyResponse, ResponseError> {
        Ok(
            message_edit::post(&self.api_base_url, &self.http_client, message_id, message)
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
    ) -> Result<message_create::MessageModifyResponse, ResponseError> {
        Ok(reaction_add::post(
            &self.api_base_url,
            &self.http_client,
            message_id,
            reaction_type as i32 as u64,
        )
        .await?
        .to_result()?)
    }

    pub async fn remove_reaction(
        &self,
        message_id: u64,
        reaction_type: ReactionType,
    ) -> Result<message_create::MessageModifyResponse, ResponseError> {
        Ok(reaction_remove::post(
            &self.api_base_url,
            &self.http_client,
            message_id,
            reaction_type as i32 as u64,
        )
            .await?
            .to_result()?)
    }

    pub async fn user_token_login(&self, token: &str, _device_info: DeviceInfo) -> Result<TokenLoginResponse, ResponseError> {
        // TODO: pass in device info
        Ok(user_token_login::post(&self.api_base_url, &self.http_client, vec![token.to_string()])
            .await?.to_result()?)
    }
}
