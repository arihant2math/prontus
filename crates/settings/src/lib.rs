use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use tokio::io::AsyncWriteExt;

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

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Sidebar {
    pub show_dm_profile_pictures: bool,
    pub hide_categories: bool,
    pub hide_recents_categories: bool,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Auth {
    pub saved_email: Option<String>,
    pub saved_phone: Option<String>,
    pub api_key: Option<String>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Appearance {
    pub theme: Theme,
    pub sidebar: Sidebar,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Options {
    pub rich_text: bool,
    pub notifications: bool,
    pub experiments: bool,
    pub error_reporting: bool,
    pub analytics: bool,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Settings {
    pub auth: Auth,
    pub appearance: Appearance,
    pub options: Options
}

impl Settings {
    pub fn path() -> PathBuf {
        // TODO: remove this in the far far future
        let old_settings = home::home_dir()
            .unwrap()
            .join(".prontus")
            .join("settings.json");
        if old_settings.exists() {
            std::fs::remove_file(old_settings).unwrap();
        }
        home::home_dir()
            .unwrap()
            .join(".prontus")
            .join("settings.json")
    }

    pub async fn delete() -> Result<()> {
        let path = Self::path();
        tokio::fs::remove_file(path).await?;
        Ok(())
    }

    pub async fn load() -> Result<Self> {
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
