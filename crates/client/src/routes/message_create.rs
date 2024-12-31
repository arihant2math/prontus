use crate::custom_json::ToJson;
use crate::models::Message;
use chrono::{DateTime, Utc};
use reqwest::Client;
use serde::{Deserialize, Serialize, Serializer};
use serde_json::{json, Value};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MessageModifyResponse {
    pub ok: bool,
    pub message: Message,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MessageModifyRequest {
    channel_id: u64,
    message: String,
    user_id: u64,
    time: DateTime<Utc>,
    parent: Option<u64>,
}

impl ToJson for MessageModifyRequest {
    fn to_json(&self) -> Value {
        let uuid = Uuid::new_v4().to_string();
        let time_string = self.time.format("%Y-%m-%d %H:%M:%S").to_string();
        if self.parent.is_some() {
            json!({
                    "bubble_id": self.channel_id,
                    "created_at": time_string,
                    "message": self.message,
                    "id": Value::Null,
                    "sendState": "sending",
                    "user_id": self.user_id,
                    "uuid": uuid,
                    "parentmessage_id": self.parent
            })
        } else {
            json!({
                "bubble_id": self.channel_id,
                "created_at": time_string,
                "message": self.message,
                "id": Value::Null,
                "sendState": "sending",
                "user_id": self.user_id,
                "uuid": uuid
            })
        }
    }
}

impl Serialize for MessageModifyRequest {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.to_json().serialize(serializer)
    }
}

pub type MessageModifyResult = crate::APIResult<MessageModifyResponse>;

pub async fn post(
    pronto_base_url: &str,
    client: &Client,
    channel_id: u64,
    message: String,
    user_id: u64,
    time: DateTime<Utc>,
    parent: Option<u64>,
) -> Result<MessageModifyResult, reqwest::Error> {
    let uuid = Uuid::new_v4().to_string();
    let time_string = time.format("%Y-%m-%d %H:%M:%S").to_string();
    let request = if parent.is_some() {
        client
            .post(format!("{pronto_base_url}v1/message.create"))
            .json(&json!(
                {
                    "bubble_id": channel_id,
                    "created_at": time_string,
                    "message": message,
                    "id": Value::Null,
                    "sendState": "sending",
                    "user_id": user_id,
                    "uuid": uuid,
                    "parentmessage_id": parent
                }
            ))
    } else {
        client
            .post("https://stanfordohs.pronto.io/api/v1/message.create")
            .json(&json!(
            {
                "bubble_id": channel_id,
                "created_at": time_string,
                "message": message,
                "id": Value::Null,
                "sendState": "sending",
                "user_id": user_id,
                "uuid": uuid
            }
                ))
    };
    let response = request.send().await?;
    let response: MessageModifyResult = response.json().await?;
    Ok(response)
}
