use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GetDevicePingResponse {
    pub ok: bool,
}

pub type GetDevicePingResult = crate::APIResult<GetDevicePingResponse>;

// https://stanfordohs.pronto.io/api/v1/device.ping
pub async fn get(pronto_base_url: &str, client: &Client) -> Result<GetDevicePingResult, reqwest::Error> {
    let response = client.get(format!("{pronto_base_url}v1/device.ping"))
        .send()
        .await?;
    let json = response.json::<GetDevicePingResult>().await?;
    Ok(json)
}