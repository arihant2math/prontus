use crate::UserInfo;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PostUserSearchResponse {
    pub data: Vec<UserInfo>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum PostUserSearchRelation {
    All,
    Connections,
    BubbleIds(Vec<u64>),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PostUserSearchRequest {
    page_size: u64,
    query: String,
    relation: PostUserSearchRelation, // &filter[query]=test
                                      // page[size]=30
                                      // relation can be [all, connections, or "filter[bubble_ids][]=2747415"]
}

pub type PostUserSearchResult = crate::APIResult<PostUserSearchResponse>;

pub async fn post(
    pronto_base_url: &str,
    client: &reqwest::Client,
    request: PostUserSearchRequest,
) -> Result<PostUserSearchResult, crate::ResponseError> {
    let r = client
        .post(format!("{pronto_base_url}{}", "clients/users/search"))
        .json(&request)
        .send()
        .await?;
    let text = r.text().await?;
    #[allow(unused)]
    let json = serde_json::from_str(&text);
    unimplemented!();
    #[allow(unreachable_code)]
    match json {
        Ok(json) => Ok(json),
        Err(_) => {
            let json = serde_json::from_str::<PostUserSearchResponse>(&text);
            let e = json.unwrap_err();
            log::error!("Error parsing json response: {:?}.", e);
            Err(crate::ResponseError::from(e))
        }
    }
}
