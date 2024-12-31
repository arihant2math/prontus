use crate::Message;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GetBubbleHistoryResponse {
    pub ok: bool,
    pub pagesize: u64,
    pub messages: Vec<Message>,
    #[serde(rename = "parentmessages")]
    pub parent_messages: Vec<Message>,
}

pub type GetBubbleHistoryResult = crate::APIResult<GetBubbleHistoryResponse>;

pub async fn get(
    pronto_base_url: &str,
    client: &Client,
    bubble_id: u64,
    latest_message_id: Option<u64>,
) -> Result<GetBubbleHistoryResult, crate::ResponseError> {
    let initial_time = std::time::Instant::now();
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
    let elapsed = initial_time.elapsed();
    log::debug!(target : "request_perf" , "Network: {} ms" , elapsed . as_millis ());
    let text = r.text().await?;
    log::trace!("Response: {}", text);
    let json = serde_json::from_str(&text);
    match json {
        Ok(json) => Ok(json),
        Err(_e) => {
            let json = serde_json::from_str::<GetBubbleHistoryResponse>(&text);
            let e = json.unwrap_err();
            log::error!("Error parsing json response: {:?}.", e);
            let json = serde_json::from_str::<serde_json::Value>(&text);
            if json.is_err() {
                return Err(crate::ResponseError::NotJson(text));
            }
            Err(crate::ResponseError::from(e))
        }
    }
}
