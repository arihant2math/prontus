use std::collections::HashMap;
use reqwest::Client;
use serde_json::json;

pub async fn post(pronto_base_url: &str, client: &Client, message_id: u64) -> HashMap<String, bool> {
    let r = client.post(format!("{pronto_base_url}v1/message.delete"))
        .json(&json!({"message_id": message_id }))
        .send()
        .await
        .unwrap();
    let json = r.json::<HashMap<String, bool>>().await.unwrap();
    json
}
