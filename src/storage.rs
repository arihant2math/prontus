use std::{fs, thread};
use std::fs::OpenOptions;
use std::io::Read;
use std::path::PathBuf;
use base64::prelude::*;
use home::home_dir;

pub fn get_image_path(url: &str) -> PathBuf {
    let filename = BASE64_URL_SAFE.encode(url);
    home_dir().unwrap().join(".prontus").join(filename)
}

fn save_image(url: &str, file: &PathBuf) {
    let mut file = OpenOptions::new().create(true).write(true).truncate(true).open(file).unwrap();
    reqwest::blocking::get(url).unwrap().copy_to(&mut file).unwrap();
}

pub fn load_image(url: String) -> PathBuf {
    let path = get_image_path(&url);
    let p = path.clone();
    if path.exists() {
        let mut file = OpenOptions::new().read(true).open(&path).unwrap();
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).unwrap();
    } else {
        fs::copy(home_dir().unwrap().join(".prontus").join("default.jpg"), &path).unwrap();
        thread::spawn(move || {
            save_image(&url, &path);
            let mut file = OpenOptions::new().read(true).open(&path).unwrap();
            let mut buffer = Vec::new();
            file.read_to_end(&mut buffer).unwrap();
            buffer
        });
    }
    p
}
