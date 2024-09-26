use reqwest::Client;
use serde::{Deserialize, Serialize};
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
    #[serde(rename = "hasmore")]
    pub has_more: bool,
    pub cursors: Cursors,
}

pub type GetAnnouncementListResult = crate::APIResult<GetAnnouncementListResponse>;

#[derive(Serialize, Deserialize)]
pub struct GetAnnouncementListRequest {
    /// UNREAD, or RECEIVED
    pub query: String,
    /// 20 should work
    #[serde(rename = "perPage")]
    pub per_page: u64,
}
pub async fn get(
    pronto_base_url: &str,
    client: &Client,
    request: GetAnnouncementListRequest,
) -> Result<GetAnnouncementListResult, reqwest::Error> {
    let r = client
        .get(format!("{pronto_base_url}v2/announcement.list"))
        .json(&request)
        .send()
        .await?;
    let json = r.json::<GetAnnouncementListResult>().await?;
    Ok(json)
}
