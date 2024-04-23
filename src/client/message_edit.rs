use serde_json::json;
use crate::client::message_create::MessageModifyResponse;

pub fn post(pronto_base_url: &str, client: &reqwest::blocking::Client, message: String, message_id: u64) -> MessageModifyResponse {
    let r = client.post(format!("{pronto_base_url}v1/message.edit"))
        .json(&json!({ "message": message, "message_id": message_id }))
        .send().unwrap();
    let json = r.json::<MessageModifyResponse>().unwrap();
    json
}
