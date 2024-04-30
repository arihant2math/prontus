use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PusherAuthRequest {
    pub socket_id: String,
    pub channel_name: String
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PusherAuthResponse {
    pub auth: String
}

pub async fn post(pronto_base_url: &str, client: &reqwest::Client, socket_id: &str, channel_name: &str) -> Result<PusherAuthResponse, reqwest::Error> {
    let r = client.post(format!("{pronto_base_url}v1/pusher.auth"))
        .json(&PusherAuthRequest { socket_id: socket_id.to_string(), channel_name: channel_name.to_string() })
        .send()
        .await?;
    let json = r.json().await?;
    Ok(json)
}