use home::home_dir;
use std::path::PathBuf;

pub fn base_dir() -> PathBuf {
    home_dir().unwrap().join(".prontus")
}

pub fn image_dir() -> PathBuf {
    base_dir().join("img_cache")
}
