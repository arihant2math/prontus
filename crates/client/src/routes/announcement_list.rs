use reqwest::Client;
pub use crate::Announcement;

struct Cursors {
    pub prev: Option<String>,
    pub next: Option<String>,
}

struct Root {
    pub ok: bool,
    pub announcements: Vec<Announcement>,
    pub pagesize: u64,
    pub hasmore: bool,
    pub cursors: Cursors,
}

pub async fn get(
    pronto_base_url: &str,
    client: &Client,
) -> Result<(), reqwest::Error> {
    Ok(())
}
