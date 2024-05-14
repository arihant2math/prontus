use std::path::PathBuf;
use home::home_dir;

pub fn base_dir() -> PathBuf {
    home_dir().unwrap().join(".prontus")
}

pub fn image_dir() -> PathBuf {
    base_dir().join("img_cache")
}