use reqwest::Client;

// https://stanfordohs.pronto.io/api/v1/device.ping
pub async fn get(pronto_base_url: &str, client: &Client) -> Result<(), reqwest::Error> {
    let _ = client.get(format!("{pronto_base_url}v1/device.ping"))
        .send()
        .await?;
    // TODO: Catch error
    Ok(())
}