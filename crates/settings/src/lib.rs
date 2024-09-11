use std::io::Write;
use bincode::config::Configuration;
use bincode::{Decode, Encode};
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use tokio::io::AsyncWriteExt;

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
pub struct Settings {
    pub saved_email: Option<String>,
    pub saved_phone: Option<String>,
    pub api_key: Option<String>,
    pub theme: Theme,
}

impl Settings {
    pub fn path() -> PathBuf {
        home::home_dir()
            .unwrap()
            .join(".prontus")
            .join("settings.bnc")
    }

    pub fn load() -> Result<Self, std::io::Error> {
        let path = Self::path();
        if path.exists() {
            Ok(
                bincode::decode_from_std_read(&mut std::fs::File::open(path)?, BINCODE_CONFIG)
                    .unwrap(),
            )
        } else {
            Ok(Self::default())
        }
    }

    pub fn save(&self) -> Result<(), std::io::Error> {
        let path = Self::path();
        std::fs::create_dir_all(path.parent().unwrap())?;
        let data = bincode::encode_to_vec(&self, BINCODE_CONFIG)?;
        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&path)?;
        file.write(&data)?;
        Ok(())
    }

    pub async fn load_async() -> Result<Self, std::io::Error> {
        let path = Self::path();
        if path.exists() {
            // TODO: switch to OpenOptions
            let data = tokio::fs::read(&path)?;
            Ok(bincode::decode_from_slice(&data, BINCODE_CONFIG).unwrap().0)
        } else {
            Ok(Self::default())
        }
    }

    pub async fn save_async(&self) -> Result<(), std::io::Error> {
        let path = Self::path();
        tokio::fs::create_dir_all(path.parent().unwrap())?;
        let data = bincode::encode_to_vec(&self, BINCODE_CONFIG)?;
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
