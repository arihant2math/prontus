use crate::UserInfo;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GetUserSearchResponse {
    pub data: Vec<UserInfo>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub enum GetUserSearchRelation {
    #[default]
    All,
    Connections,
    BubbleIds(Vec<u64>),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GetUserSearchRequest {
    pub page_size: u64,
    pub query: String,
    pub relation: GetUserSearchRelation,
    // &filter[query]=test
    // page[size]=30
    // relation can be [all, connections, or "filter[bubble_ids][]=2747415"]
}

impl Default for GetUserSearchRequest {
    fn default() -> Self {
        Self {
            query: "".to_string(),
            page_size: 30,
            relation: GetUserSearchRelation::default(),
        }
    }
}

pub type GetUserSearchResult = crate::APIResult<GetUserSearchResponse>;

pub async fn get(
    pronto_base_url: &str,
    client: &reqwest::Client,
    request: GetUserSearchRequest,
) -> Result<GetUserSearchResult, crate::ResponseError> {
    let mut query_url = format!("{pronto_base_url}{}", "clients/users/search");
    query_url.push_str("?");
    query_url.push_str(&format!("page[size]={}", request.page_size));
    query_url.push_str("&");
    query_url.push_str(&format!("filter[query]={}", request.query));
    let r = client.get(query_url).send().await?;
    let text = r.text().await?;
    log::trace!("Response: {}", text);
    let json = serde_json::from_str(&text);
    match json {
        Ok(json) => Ok(json),
        Err(_) => {
            let json = serde_json::from_str::<GetUserSearchResponse>(&text);
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
