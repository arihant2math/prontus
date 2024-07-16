use crate::client::message_create::MessageModifyResult;
use reqwest::Client;
use serde_json::json;

pub async fn post(
    pronto_base_url: &str,
    client: &Client,
    message_id: u64,
    message: String,
) -> Result<MessageModifyResult, reqwest::Error> {
    let r = client
        .post(format!("{pronto_base_url}v1/message.edit"))
        .json(&json!({ "message": message, "message_id": message_id }))
        .send()
        .await?;
    let json = r.json().await?;
    Ok(json)
}
