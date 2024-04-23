mod bubble;
mod bubble_history;
mod bubble_info;
mod bubble_list;
mod device_ping;
mod message_create;
mod message_edit;
mod message_delete;
mod user_info;

use std::sync::Arc;
use chrono::Utc;
use reqwest::header::{HeaderMap, HeaderValue};
use crate::secret::PRONTO_API_TOKEN;

pub struct ProntoClient {
    pub api_base_url: String,
    pub http_client: reqwest::Client,
}

impl ProntoClient {
    pub fn new(api_base_url: String, pronto_session: &str, pronto_api_token: &str, pacct_2245_5302428: &str) -> Self {
        // create the cookie store
        let cookies = vec![
            format!("pronto_session={}", pronto_session),
            format!("api_token={}", pronto_api_token),
            format!("pacct_2245_5302428={}", pacct_2245_5302428),
        ];
        let jar = reqwest::cookie::Jar::default();
        for cookie in cookies {
            jar.add_cookie_str(&cookie, &reqwest::Url::parse(&api_base_url).unwrap());
        }

        let mut headers = HeaderMap::new();
        headers.insert("Accept", HeaderValue::from_str("application/json, text/plain, */*").unwrap());
        headers.insert("Accept-Language", HeaderValue::from_str("en-US,en;q=0.5").unwrap());
        headers.insert("Authorization", HeaderValue::from_str(&format!("Bearer {PRONTO_API_TOKEN}")).unwrap());
        let client = reqwest::Client::builder()
            .cookie_store(true)
            .cookie_provider(Arc::new(jar))
            .default_headers(headers)
            .build()
            .unwrap();
        Self {
            api_base_url,
            http_client: client,
        }
    }

    pub async fn get_user_info(&self) -> user_info::GetUserInfoResponse {
        user_info::get(&self.api_base_url, &self.http_client).await
    }

    pub async fn get_bubble_list(&self) -> bubble_list::GetBubbleListResponse {
        bubble_list::get(&self.api_base_url, &self.http_client).await
    }

    pub async fn get_bubble_info(&self, bubble_id: u64) -> bubble_info::GetBubbleInfoResponse {
        bubble_info::get(&self.api_base_url, &self.http_client, bubble_id).await
    }

    pub async fn get_bubble_history(&self, bubble_id: u64, latest_message_id: Option<u64>) -> bubble_history::GetBubbleHistoryResponse {
        bubble_history::get(&self.api_base_url, &self.http_client, bubble_id, latest_message_id).await
    }

    pub async fn post_message(&self, user_id: u64, bubble_id: u64, message: String, parent_message_id: Option<u64>) -> message_create::MessageModifyResponse {
        message_create::post(&self.api_base_url, &self.http_client, bubble_id, message, user_id, Utc::now(), parent_message_id).await
    }

    pub async fn edit_message(&self, message_id: u64, message: String) -> message_edit::MessageModifyResponse {
        message_edit::post(&self.api_base_url, &self.http_client, message_id, message).await
    }

    pub async fn delete_message(&self, message_id: u64) -> message_edit::MessageModifyResponse {
        message_delete::delete(&self.api_base_url, &self.http_client, message_id).await
    }
}
