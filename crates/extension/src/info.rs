use std::error;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::PathBuf;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Permissions {
    pub read_settings: bool,
    pub write_settings: bool,
    pub full_network: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExtensionInfo {
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    pub authors: Option<Vec<String>>,
    pub license: Option<String>,
    pub repository: Option<String>,
    pub homepage: Option<String>,
    pub documentation: Option<String>,
    pub keywords: Option<Vec<String>>,
    pub permissions: Permissions
}

impl TryFrom<PathBuf> for ExtensionInfo {
    type Error = Box<dyn error::Error + Send + Sync>;

    fn try_from(value: PathBuf) -> Result<Self, Self::Error> {
        if !value.is_file() {
            return Err("ExtensionInfo must be a file".into());
        }
        let file = File::open(value)?;
        let mut buf = String::new();
        BufReader::new(file).read_to_string(&mut buf)?;
        let info: ExtensionInfo = toml::from_str(&buf)?;
        Ok(info)
    }
}
