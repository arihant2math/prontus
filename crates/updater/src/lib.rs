use std::collections::HashMap;
use serde::{Serialize, Deserialize};

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
    pub versions: HashMap<String, Version>
}

pub const ALPHA_UPDATE_URL: &str = "https://raw.githubusercontent.com/arihant2math/prontus-update/refs/heads/main/alpha.json";
pub const BETA_UPDATE_URL: &str = "https://raw.githubusercontent.com/arihant2math/prontus-update/refs/heads/main/beta.json";
pub const RC_UPDATE_URL: &str = "https://raw.githubusercontent.com/arihant2math/prontus-update/refs/heads/main/rc.json";
pub const STABLE_UPDATE_URL: &str = "https://raw.githubusercontent.com/arihant2math/prontus-update/refs/heads/main/stable.json";

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum UpdateChannel {
    Alpha,
    Beta,
    RC,
    Stable,
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
    async fn get_update_file_with_url(url: &str) -> Result<UpdateFile, Box<dyn std::error::Error>> {
        let client = reqwest::Client::new();
        let res = client.get(url).send().await?;
        let text = res.text().await?;
        let json = serde_json::from_str::<UpdateFile>(&text)?;
        Ok(json)
    }

    pub async fn update_file(channel: UpdateChannel) -> Result<UpdateFile, Box<dyn std::error::Error>> {
        Self::get_update_file_with_url(channel.url()).await
    }

    pub async fn update_available(&self) -> bool {
        let current_version = version::VERSION;
        let latest_version = self.latest_version.clone().unwrap_or_else(|| current_version.to_string());
        current_version != latest_version
    }

    pub async fn latest_version_details(&self) -> Result<Option<Version>, Box<dyn std::error::Error>> {
        let latest_version = &self.latest_version;
        if let Some(latest_version) = latest_version {
            Ok(self.versions.get(latest_version).cloned())
        } else {
            Ok(None)
        }
    }

    pub async fn no_start_active(&self) -> bool {
        self.notices.iter().any(|notice| match notice {
            Notice::NoStart(ns) => if ns.version == version::VERSION { true } else { false },
            _ => false,
        })
    }
}

// TODO: eventually support auto updates

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_channels() {
        // ensure all channels can be parsed normally
        let channels = [UpdateChannel::Alpha, UpdateChannel::Beta, UpdateChannel::RC, UpdateChannel::Stable];
        for channel in channels {
            UpdateFile::update_file(channel).await.unwrap();
        }
    }
}
