use std::path::PathBuf;
use bincode::{Encode, Decode};
use bincode::config::Configuration;

pub const BINCODE_CONFIG: Configuration = bincode::config::standard();

#[derive(Copy, Clone, Debug, PartialEq, Encode, Decode)]
pub enum Theme {
    Light,
    Dark,
    Auto
}

impl Default for Theme {
    fn default() -> Self {
        Theme::Auto
    }
}

#[derive(Clone, Debug, Default, Encode, Decode)]
pub struct Settings {
    pub saved_email: Option<String>,
    pub saved_phone: Option<String>,
    pub api_key: Option<String>,
    pub theme: Theme
}

impl Settings {
    pub fn path() -> PathBuf {
        home::home_dir().unwrap().join(".prontus").join("settings.bnc")
    }

    pub fn load() -> Result<Self, std::io::Error> {
        let path = Self::path();
        if path.exists() {
            Ok(bincode::decode_from_std_read(&mut std::fs::File::open(path)?, BINCODE_CONFIG).unwrap())
        } else {
            Ok(Self::default())
        }
    }

    pub fn save(&self) -> Result<(), std::io::Error> {
        let path = Self::path();
        std::fs::create_dir_all(path.parent().unwrap())?;
        bincode::encode_into_std_write(&self, &mut std::fs::File::create(path)?, BINCODE_CONFIG).unwrap();
        Ok(())
    }
}
