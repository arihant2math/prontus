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
pub struct PostBubbleMembershipSearchRequest {
    #[serde_as(as = "DisplayFromStr")]
    pub bubble_id: u64,
    #[serde(default = "_true", rename = "includeself")]
    pub include_self: bool,
    pub page: u64,
    #[serde(default = "default_order_by", rename = "orderby")]
    pub order_by: Vec<String>,
    // TODO: fix this
    // search: Option<String>
    // role: Option<String>
}

impl Default for PostBubbleMembershipSearchRequest {
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
pub struct PostBubbleMembershipSearchResponse {
    pub ok: bool,
    #[serde(rename = "pagesize")]
    pub page_size: u64,
    #[serde(rename = "memberships")]
    pub membership: Vec<Member>,
}

pub type PostBubbleMembershipSearchResult = crate::APIResult<PostBubbleMembershipSearchResponse>;

client_macros::api!(
    post,
    "v1/bubble.membershipsearch",
    PostBubbleMembershipSearchResult,
    PostBubbleMembershipSearchRequest
);
