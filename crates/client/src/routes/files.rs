// PUT https://stanfordohs.pronto.io/api/files?filename=image.png
// Request = [[ the image ]]
// Response = {"data":{"key":"0a43fa48-403c-4a4e-8af5-ca0c01bab35c","expires":"2024-09-18T15:44:32Z","name":"image.png","size":74720,"type":"image/png"}}

use reqwest::Client;
use serde::{Deserialize, Serialize};

pub struct PutFileRequest {
    pub file_name: String,
    pub file_data: Vec<u8>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PutFileResponseData {
    pub key: String,
    pub expires: String,
    pub name: String,
    pub size: u64,
    pub r#type: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PutFileResponse {
    pub data: PutFileResponseData,
}

pub type PutFileResult = crate::APIResult<PutFileResponse>;

#[allow(unused_variables)]
pub async fn put(pronto_base_url: &str, client: &Client, request: PutFileRequest) -> Result<PutFileResult, crate::ResponseError> {
    client.put(&format!("{}/api/files?filename={}", pronto_base_url, request.file_name))
        .body(request.file_data)
        .send()
        .await?;
    todo!("parse response");
}
