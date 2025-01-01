use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::Write;
use thiserror::Error;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Download {
    pub url: String,
    pub checksum: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Version {
    pub version: String,
    pub major: u64,
    pub minor: u64,
    pub patch: u64,
    pub downloads: HashMap<String, Download>,
    pub release_page: String,
    pub release_date: String,
    pub changelog: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Alert {
    pub title: String,
    pub description: String,
    pub url: String,
    pub severity: String,
    pub alert_type: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct NoStart {
    pub version: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum Notice {
    Alert(Alert),
    NoStart(NoStart),
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct UpdateFile {
    pub latest_version: Option<String>,
    pub notices: Vec<Notice>,
    pub versions: HashMap<String, Version>,
}

pub const ALPHA_UPDATE_URL: &str =
    "https://raw.githubusercontent.com/arihant2math/prontus-update/refs/heads/main/alpha.json";
pub const BETA_UPDATE_URL: &str =
    "https://raw.githubusercontent.com/arihant2math/prontus-update/refs/heads/main/beta.json";
pub const RC_UPDATE_URL: &str =
    "https://raw.githubusercontent.com/arihant2math/prontus-update/refs/heads/main/rc.json";
pub const STABLE_UPDATE_URL: &str =
    "https://raw.githubusercontent.com/arihant2math/prontus-update/refs/heads/main/stable.json";

#[derive(Debug, Error)]
pub enum UpdateError {
    #[error("Failed to get update file: {0}")]
    ReqwestError(#[from] reqwest::Error),
    #[error("Failed to parse update file: {0}")]
    SerdeError(#[from] serde_json::Error),
    #[error("I/O error: {0}")]
    IOError(#[from] std::io::Error),
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum UpdateChannel {
    Alpha,
    Beta,
    RC,
    Stable,
}

impl From<&str> for UpdateChannel {
    fn from(s: &str) -> Self {
        match s {
            "alpha" => UpdateChannel::Alpha,
            "beta" => UpdateChannel::Beta,
            "rc" => UpdateChannel::RC,
            "stable" => UpdateChannel::Stable,
            _ => panic!("Invalid update channel"),
        }
    }
}

impl UpdateChannel {
    pub fn url(&self) -> &'static str {
        match self {
            UpdateChannel::Alpha => ALPHA_UPDATE_URL,
            UpdateChannel::Beta => BETA_UPDATE_URL,
            UpdateChannel::RC => RC_UPDATE_URL,
            UpdateChannel::Stable => STABLE_UPDATE_URL,
        }
    }
}

impl UpdateFile {
    async fn get_update_file_with_url(url: &str) -> Result<UpdateFile, UpdateError> {
        let client = reqwest::Client::new();
        let res = client.get(url).send().await?;
        let text = res.text().await?;
        let json = serde_json::from_str::<UpdateFile>(&text)?;
        Ok(json)
    }

    pub async fn update_file(
        channel: UpdateChannel,
    ) -> Result<UpdateFile, UpdateError> {
        Self::get_update_file_with_url(channel.url()).await
    }

    pub fn update_available(&self) -> bool {
        let current_version = version::VERSION;
        let latest_version = self
            .latest_version
            .clone()
            .unwrap_or_else(|| current_version.to_string());
        current_version != latest_version
    }

    pub fn latest_version_details(&self) -> Result<Option<Version>, UpdateError> {
        let latest_version = &self.latest_version;
        if let Some(latest_version) = latest_version {
            Ok(self.versions.get(latest_version).cloned())
        } else {
            Ok(None)
        }
    }

    pub fn no_start_active(&self) -> bool {
        self.notices.iter().any(|notice| match notice {
            Notice::NoStart(ns) => {
                if ns.version == version::VERSION {
                    true
                } else {
                    false
                }
            }
            _ => false,
        })
    }
}

pub enum InstallUpdateType {
    WindowsMSI,
    WindowsEXE,
    WindowsStandAlone,
    MacOSDiskImage,
    LinuxDEB,
    LinuxRPM,
    LinuxAppImage,
}

pub struct InstallUpdate {
    pub update_type: InstallUpdateType,
    pub download_url: String,
    pub file_name: String,
    pub checksum: String,
}

impl InstallUpdate {
    pub async fn download(&self) -> Result<(), UpdateError> {
        let dir = std::env::temp_dir();
        let file_path = dir.join(&self.file_name);
        let client = reqwest::Client::new();
        let mut res = client.get(&self.download_url).send().await?;
        let mut file = std::fs::File::create(&file_path)?;
        while let Some(chunk) = res.chunk().await? {
            file.write_all(&chunk)?;
        }
        Ok(())
    }

    pub async fn install(&self) -> Result<(), Box<dyn std::error::Error>> {
        let dir = std::env::temp_dir();
        let file_path = dir.join(&self.file_name);
        // TODO: eventually support installing updates
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_channels() {
        // ensure all channels can be parsed normally
        let channels = [
            UpdateChannel::Alpha,
            UpdateChannel::Beta,
            UpdateChannel::RC,
            UpdateChannel::Stable,
        ];
        for channel in channels {
            UpdateFile::update_file(channel).await.unwrap();
        }
    }
}
