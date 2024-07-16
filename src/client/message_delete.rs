use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DeleteMessageResult {
    pub ok: bool,
}

pub type DeleteMessageResultResult = crate::APIResult<DeleteMessageResult>;

pub async fn post(
    pronto_base_url: &str,
    client: &Client,
    message_id: u64,
) -> Result<DeleteMessageResultResult, reqwest::Error> {
    let r = client
        .post(format!("{pronto_base_url}v1/message.delete"))
        .json(&json!({"message_id": message_id }))
        .send()
        .await?;
    let json: DeleteMessageResultResult = r.json().await?;
    Ok(json)
}
