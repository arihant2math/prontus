use reqwest::blocking::Client;

// https://stanfordohs.pronto.io/api/v1/device.ping
pub fn get(pronto_base_url: &str, client: &Client) -> Result<(), ()> {
    let _ = client.get(format!("{pronto_base_url}v1/device.ping"))
        .send()
        .unwrap();
    Ok(())
}