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
    let r = client
        .get(format!("{pronto_base_url}{}", "clients/users/search"))
        .query(&request)
        .send()
        .await?;
    let text = r.text().await?;
    let json = serde_json::from_str(&text);
    match json {
        Ok(json) => Ok(json),
        Err(_) => {
            let jd = &mut serde_json::Deserializer::from_str(&text);
            let result: Result<GetUserSearchResponse, _> = serde_path_to_error::deserialize(jd);
            let e = result.unwrap_err();
            log::error!("Error parsing json response: {:?}.", e);
            Err(crate::ResponseError::from(e))
        }
    }
}
