use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use crate::client::bubble::Bubble;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GetBubbleListResponse {
    pub ok: bool,
    pub bubbles: Vec<Bubble>,
}

pub fn get(pronto_base_url: &str, client: &Client) -> GetBubbleListResponse {
    let r = client.get(format!("{pronto_base_url}v2/bubble.list"))
        .send()
        .unwrap();
    // println!("{}", r.text().unwrap());
    let json = r.json::<GetBubbleListResponse>().unwrap();
    json
    // GetBubbleListResponse { ok: true, bubbles: vec![] }
}
