mod settings_file_v0;
mod settings_file_v1;

use bincode::config::Configuration;
use bincode::{Decode, Encode};
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use tokio::io::AsyncWriteExt;
use crate::settings_file_v0::SettingsFileV0;
use crate::settings_file_v1::SettingsFileV1;

#[derive(Debug, thiserror::Error)]
pub enum SettingsError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Bincode error: {0}")]
    BincodeDecodeError(#[from] bincode::error::DecodeError),
    #[error("Bincode error: {0}")]
    BincodeEncodeError(#[from] bincode::error::EncodeError),
}

pub type Result<T> = std::result::Result<T, SettingsError>;

pub const BINCODE_CONFIG: Configuration = bincode::config::standard();

#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize, Encode, Decode)]
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

#[derive(Clone, Debug, Default, Serialize, Deserialize, Encode, Decode)]
pub struct Sidebar {
    pub show_dm_profile_pictures: bool,
    pub hide_categories: bool,
    pub hide_recents_categories: bool,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, Encode, Decode)]
pub struct Auth {
    pub saved_email: Option<String>,
    pub saved_phone: Option<String>,
    pub api_key: Option<String>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, Encode, Decode)]
pub struct Appearance {
    pub theme: Theme,
    pub sidebar: Sidebar,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, Encode, Decode)]
pub struct Options {
    pub rich_text: bool,
    pub notifications: bool,
    pub experiments: bool,
    pub error_reporting: bool,
    pub analytics: bool,
}

#[derive(Clone, Debug, Encode, Decode)]
pub enum SettingsFile {
    V0(SettingsFileV0),
    V1(SettingsFileV1)
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, Encode, Decode)]
pub struct Settings {
    pub auth: Auth,
    pub appearance: Appearance,
    pub options: Options
}

impl Settings {
    pub fn path() -> PathBuf {
        home::home_dir()
            .unwrap()
            .join(".prontus")
            .join("settings.bnc")
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
            let data = tokio::fs::read(&path).await?;
            match bincode::decode_from_slice::<SettingsFile, Configuration>(&data, BINCODE_CONFIG) {
                Ok((settings, _)) => Ok(settings.into()),
                Err(_) => {
                    Self::delete().await?;
                    Ok(Self::default())
                },
            }
        } else {
            Ok(Self::default())
        }
    }

    pub async fn save(&self) -> Result<()> {
        let path = Self::path();
        tokio::fs::create_dir_all(path.parent().unwrap()).await?;
        let data = bincode::encode_to_vec(&SettingsFile::from(self.clone()), BINCODE_CONFIG)?;
        let mut file = tokio::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&path)
            .await?;
        file.write(&data).await?;
        Ok(())
    }
}

impl From<SettingsFile> for Settings {
    fn from(settings: SettingsFile) -> Self {
        match settings {
            SettingsFile::V0(v0) => v0.into(),
            SettingsFile::V1(v1) => v1.into(),
        }
    }
}

impl From<Settings> for SettingsFile {
    fn from(settings: Settings) -> Self {
        SettingsFile::V1(settings.into())
    }
}