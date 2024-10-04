use crate::Member;
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
pub struct PostMessageSearchRequest {
    pub search_type: String,
    pub size: u64,
    pub from: u64,
    pub orderby: String,
    pub query: String,
}

impl Default for PostMessageSearchRequest {
    fn default() -> Self {
        Self {
            search_type: "messages".to_string(),
            size: 25,
            from: 0,
            orderby: "newest".to_string(),
            query: "".to_string(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PostMessageSearchResponse {
    pub ok: bool,
    #[serde(rename = "pagesize")]
    pub page_size: u64,
    #[serde(rename = "memberships")]
    pub membership: Vec<Member>,
}

pub type PostMessageSearchResult = crate::APIResult<PostMessageSearchResponse>;

client_macros::api!(post, "v1/message.search", PostMessageSearchResult, PostMessageSearchRequest);
