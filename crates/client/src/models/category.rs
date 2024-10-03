use serde::{Deserialize, Serialize};

struct UserCategory {
    pub id: i64,
    pub user_id: i64,
    pub category_id: i64,
    pub alias: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Category {
    pub id: u64,
    pub title: String,
    pub sort_order: Option<u32>,
    #[serde(rename = "usercategory")]
    pub user_category: UserCategory
}
