use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
pub use crate::Announcement;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Cursors {
    pub prev: Option<String>,
    pub next: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GetAnnouncementListResponse {
    pub ok: bool,
    pub announcements: Vec<Announcement>,
    pub pagesize: u64,
    pub hasmore: bool,
    pub cursors: Cursors,
}

pub type GetAnnouncementListResult = crate::APIResult<GetAnnouncementListResponse>;

pub async fn get(
    pronto_base_url: &str,
    client: &Client,
) -> Result<GetAnnouncementListResult, reqwest::Error> {
    let r = client
        .get(format!("{pronto_base_url}v2/announcement.list"))
        .send()
        .await?;
    let json = r.json::<GetAnnouncementListResult>().await?;
    Ok(json)
}
