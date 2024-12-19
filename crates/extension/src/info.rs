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
    pub id: String,
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

#[derive(Debug, thiserror::Error)]
pub enum ExtensionInfoCreationError {
    #[error("ExtensionInfo must be a file")]
    NotAFile,
    #[error("IO Error: {0}")]
    Io(#[from] std::io::Error),
    #[error("TOML Error: {0}")]
    Toml(#[from] toml::de::Error),
}

impl TryFrom<PathBuf> for ExtensionInfo {
    type Error = ExtensionInfoCreationError;

    fn try_from(value: PathBuf) -> Result<Self, Self::Error> {
        if !value.is_file() {
            return Err(ExtensionInfoCreationError::NotAFile);
        }
        let file = File::open(value)?;
        let mut buf = String::new();
        BufReader::new(file).read_to_string(&mut buf)?;
        let info: ExtensionInfo = toml::from_str(&buf)?;
        Ok(info)
    }
}
