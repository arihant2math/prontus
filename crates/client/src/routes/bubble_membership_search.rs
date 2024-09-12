use crate::Member;
use reqwest::Client;
use serde::{Deserialize, Serialize};

use serde_with::serde_as;
use serde_with::DisplayFromStr;

fn _true() -> bool {
    true
}

fn default_order_by() -> Vec<String> {
    vec!["firstname".to_string(), "lastname".to_string()]
}

#[serde_as]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GetBubbleMembershipSearchRequest {
    #[serde_as(as = "DisplayFromStr")]
    pub bubble_id: u64,
    #[serde(default = "_true", rename = "includeself")]
    pub include_self: bool,
    pub page: u64,
    #[serde(default = "default_order_by", rename = "orderby")]
    pub order_by: Vec<String>,
    // TODO:
    // search: Option<String>
    // role: Option<String>
}

impl Default for GetBubbleMembershipSearchRequest {
    fn default() -> Self {
        Self {
            bubble_id: 0,
            include_self: true,
            page: 1,
            order_by: vec!["firstname".to_string(), "lastname".to_string()],
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GetBubbleMembershipSearchResponse {
    pub ok: bool,
    #[serde(rename = "pagesize")]
    pub page_size: u64,
    pub membership: Vec<Member>,
}

pub type GetBubbleMembershipSearchResult = crate::APIResult<GetBubbleMembershipSearchResponse>;

pub async fn get(
    pronto_base_url: &str,
    client: &Client,
    request: GetBubbleMembershipSearchRequest,
) -> Result<GetBubbleMembershipSearchResult, reqwest::Error> {
    let r = client
        .get(format!("{pronto_base_url}v1/bubble.membershipsearch"))
        .json(&request)
        .send()
        .await?;
    let json = r.json::<GetBubbleMembershipSearchResult>().await?;
    Ok(json)
}
