use crate::user_info::UserInfo;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MessageMedia {
    pub id: u64,
    pub url: String,
    pub mediatype: String,
    pub urlmimetype: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MessageResource {
    pub id: u64,
    pub providerurl: String,
    pub snippet: String,
    pub url: String,
    pub title: String,
    pub thumbnailurl: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Reactions {
    #[serde(rename = "reactiontype_id")]
    pub id: u64,
    pub count: u64,
    pub users: Vec<u64>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Message {
    pub id: u64,
    pub user_id: u64,
    pub bubble_id: u64,
    pub message: String,
    pub user: UserInfo,
    #[serde(default, rename = "parentmessage_id")]
    pub parent_message_id: Option<u64>,
    #[serde(default, rename = "reactionsummary")]
    pub reactions: Vec<Reactions>,
    #[serde(default, rename = "messagemedia")]
    pub message_media: Vec<MessageMedia>,
    #[serde(default)]
    pub resource: Option<MessageResource>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GetBubbleHistoryResponse {
    pub ok: bool,
    pub pagesize: u64,
    pub messages: Vec<Message>,
    pub parentmessages: Vec<Message>,
}

pub type GetBubbleHistoryResult = crate::APIResult<GetBubbleHistoryResponse>;

pub async fn get(
    pronto_base_url: &str,
    client: &Client,
    bubble_id: u64,
    latest_message_id: Option<u64>,
) -> Result<GetBubbleHistoryResult, reqwest::Error> {
    // TODO: catch {"ok":false,"error":"BUBBLE_NOTFOUND"}
    let r = if let Some(latest_message_id) = latest_message_id {
        client
            .get(format!("{pronto_base_url}v1/bubble.history"))
            .query(&json!({ "bubble_id": bubble_id, "latest": latest_message_id }))
            .send()
    } else {
        client
            .get(format!("{pronto_base_url}v1/bubble.history"))
            .query(&json!({ "bubble_id": bubble_id }))
            .send()
    }
    .await?;
    let json = r.json::<GetBubbleHistoryResult>().await?;
    Ok(json)
}
