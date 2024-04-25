use reqwest::Client;
use serde_json::json;
use crate::client::message_create::MessageModifyResponse;

pub async fn post(pronto_base_url: &str, client: &Client, message_id: u64, reaction_type_id: u64) -> MessageModifyResponse {
    let r = client.post(format!("{pronto_base_url}v1//message.addreaction"))
        .json(&json!({"message_id": message_id, "reactiontype_id": reaction_type_id}))
        .send()
        .await
        .unwrap();
    let json = r.json::<MessageModifyResponse>().await.unwrap();
    json
}
