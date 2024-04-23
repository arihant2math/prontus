use serde::{Serialize, Deserialize};
use serde_json::json;
use crate::client::user_info::UserInfo;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Message {
    pub id: u64,
    pub user_id: u64,
    pub bubble_id: u64,
    pub message: String,
    pub user: UserInfo
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GetBubbleHistoryResponse {
    pub ok: bool,
    pub pagesize: u64,
    pub messages: Vec<Message>,
}

pub fn get(pronto_base_url: &str, client: &reqwest::blocking::Client, bubble_id: u64, latest_message_id: Option<u64>) -> GetBubbleHistoryResponse {
    // TODO: catch {"ok":false,"error":"BUBBLE_NOTFOUND"}
    let mut r = if let Some(latest_message_id) = latest_message_id {
        client.get(format!("{pronto_base_url}v1/bubble.history"))
            .query(&json!({ "bubble_id": bubble_id, "latest": latest_message_id }))
            .send()
    } else {
        client.get(format!("{pronto_base_url}v1/bubble.history"))
            .query(&json!({ "bubble_id": bubble_id }))
            .send()
    }.unwrap();
    let json = r.json::<GetBubbleHistoryResponse>().unwrap();
    json
}
