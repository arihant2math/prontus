use std::collections::HashMap;
use std::fs::File;
use serde::{Deserialize, Serialize};
use extension::info::ExtensionInfo;
use rand::random;

pub const INDEX_URL: &str = "https://raw.githubusercontent.com/arihant2math/prontus-extensions/refs/heads/main/extension-index.json";

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VersionedExtensionInfo {
    pub latest_version: ExtensionInfo,
    pub versions: HashMap<String, (ExtensionInfo, String)>
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExtensionIndexFile {
    pub extensions: Vec<VersionedExtensionInfo>,
}

async fn install_extension(url: String) {
    // Write to temp file
    // Generate random append first to prevent overwriting
    let file_name = url.split('/').last().unwrap();
    let dir_name = file_name.split('.').next().unwrap();
    let rnd = random::<u64>();
    let temp_file = std::env::temp_dir().join(format!("prontus_extension_{rnd}.tar.gz"));
    let temp_dir = std::env::temp_dir().join(format!("prontus_extension_{rnd}"));
    let req = reqwest::get(url).await.unwrap();
    let mut file = File::create(&temp_file).unwrap();
    file.write_all(&req.bytes().await.unwrap()).unwrap();
    // Extract
    todo!("Extract file");
}
