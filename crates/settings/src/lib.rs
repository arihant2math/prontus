use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use tokio::io::AsyncWriteExt;
use log::{debug, info};

#[derive(Debug, thiserror::Error)]
pub enum SettingsError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("SIMD JSON error: {0}")]
    SimdJSON(#[from] simd_json::Error)
}

pub type Result<T> = std::result::Result<T, SettingsError>;

#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Theme {
    Light,
    Dark,
    Auto,
}

impl Default for Theme {
    fn default() -> Self {
        Theme::Auto
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum CategoryDisplayLevel {
    All,
    NonSingleton,
    None
}

impl Default for CategoryDisplayLevel {
    fn default() -> Self {
        Self::All
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct SidebarAppearance {
    #[serde(default)]
    pub category_display_level: CategoryDisplayLevel,
    #[serde(default)]
    pub show_unread_channels_on_collapse: bool,
    #[serde(default)]
    pub hide_dm_profile_pictures: bool,
    /// Hide aggregation categories like "Recents"
    #[serde(default)]
    pub hide_super_categories: bool,
}

const fn _true() -> bool {
    true
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MessageAppearance {
    #[serde(default)]
    pub compact: bool,
    #[serde(default)]
    pub hide_embeds: bool,
    #[serde(default = "_true")]
    pub rich_text: bool,
}

impl Default for MessageAppearance {
    fn default() -> Self {
        MessageAppearance {
            compact: false,
            hide_embeds: false,
            rich_text: true
        }
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Auth {
    #[serde(default)]
    pub saved_email: Option<String>,
    #[serde(default)]
    pub saved_phone: Option<String>,
    #[serde(default)]
    pub api_key: Option<String>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Appearance {
    #[serde(default)]
    pub theme: Theme,
    #[serde(default)]
    pub sidebar: SidebarAppearance,
    #[serde(default)]
    pub messages: MessageAppearance,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Options {
    #[serde(default)]
    pub notifications: bool,
    #[serde(default)]
    pub experiments: bool,
    #[serde(default)]
    pub error_reporting: bool,
    #[serde(default)]
    pub analytics: bool,
    #[serde(default)]
    pub read_messages: bool,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Settings {
    #[serde(default)]
    pub auth: Auth,
    #[serde(default)]
    pub appearance: Appearance,
    #[serde(default)]
    pub options: Options
}

impl Settings {
    pub fn path() -> PathBuf {
        // TODO: remove this deletion in the far far future
        let old_settings = home::home_dir()
            .unwrap()
            .join(".prontus")
            .join("settings.bnc");
        if old_settings.exists() {
            info!("Deleting old settings file");
            std::fs::remove_file(old_settings).unwrap();
        }
        home::home_dir()
            .unwrap()
            .join(".prontus")
            .join("settings.json")
    }

    pub async fn delete() -> Result<()> {
        info!("Deleting settings file");
        let path = Self::path();
        tokio::fs::remove_file(path).await?;
        Ok(())
    }

    pub async fn load() -> Result<Self> {
        debug!("Loading settings");
        let path = Self::path();
        if path.exists() {
            // TODO: switch to OpenOptions
            let mut data = tokio::fs::read_to_string(&path).await?;
            unsafe {
                Ok(simd_json::from_str(&mut data)?)
            }
        } else {
            Ok(Self::default())
        }
    }

    pub async fn save(&self) -> Result<()> {
        debug!("Saving settings");
        let path = Self::path();
        tokio::fs::create_dir_all(path.parent().unwrap()).await?;
        let data = simd_json::to_string(&self)?;
        let mut file = tokio::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&path)
            .await?;
        file.write(data.as_bytes()).await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::Settings;

    #[tokio::test]
    async fn load() {
        Settings::load().await.unwrap();
    }
}
