use home::home_dir;
use log::info;
use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io;
use thiserror::Error;

#[derive(Serialize, Deserialize, PartialEq, Debug, Copy, Clone)]
pub enum ThemePreference {
    Light,
    Dark,
    System,
}

impl Default for ThemePreference {
    fn default() -> Self {
        ThemePreference::System
    }
}

fn default_base_url() -> String {
    "https://stanfordohs.pronto.io/api/".to_string()
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Settings {
    pub pronto_api_token: Option<String>,
    pub websocket_auth_token: Option<String>,
    #[serde(default)]
    pub color_preference: ThemePreference,
    #[serde(default = "default_base_url")]
    pub base_url: String,
}

#[derive(Debug, Error)]
pub enum SettingsError {
    #[error("IO error")]
    IO(#[from] io::Error),
    #[error("JSON error")]
    SerdeJson(#[from] serde_json::Error),
}

impl Settings {
    pub fn load(file_name: &str) -> Result<Settings, SettingsError> {
        let mut options = OpenOptions::new();
        let options = options.read(true);
        let file_name = home_dir().unwrap().join(".prontus").join(file_name);
        if !file_name.exists() {
            let create = options.create_new(true).write(true).open(&file_name)?;
            let _ = serde_json::to_writer_pretty(create, &Settings::default())?;
            info!("Created settings file: {}", file_name.display());
            return Ok(Settings::default());
        }
        let file = options.open(file_name)?;
        let settings: Settings = serde_json::from_reader(file)?;
        Ok(settings)
    }

    pub fn save(&self, file_name: &str) -> Result<(), SettingsError> {
        let mut options = OpenOptions::new();
        let options = options.write(true).create(true).truncate(true);
        let file_name = home_dir().unwrap().join(".prontus").join(file_name);
        let file = options.open(file_name)?;
        serde_json::to_writer_pretty(file, &self)?;
        Ok(())
    }
}
