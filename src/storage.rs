// TODO: Async exists ...

use std::{fs, thread};
use std::fs::OpenOptions;
use std::io::{Read, Write};
use std::path::PathBuf;
use base64::prelude::*;
use home::home_dir;

pub fn get_image_path(url: &str) -> PathBuf {
    let ext = url.split(".").last();
    // URL safe is also file safe (to me)
    let mut filename = BASE64_URL_SAFE.encode(url);
    if let Some(ext) = ext {
        filename += ".";
        filename += ext;
    }
    home_dir().unwrap().join(".prontus").join(filename)
}

fn save_url(url: &str, file: &PathBuf) {
    let mut file = OpenOptions::new().create(true).write(true).truncate(true).open(file).unwrap();
    let mut response = reqwest::blocking::get(url).unwrap();
    let bytes = response.bytes().unwrap();
    file.write_all(&bytes).unwrap();
}

pub fn load_url_path(url: String) -> PathBuf {
    let path = get_image_path(&url);
    let p = path.clone();
    if path.exists() {
        let mut file = OpenOptions::new().read(true).open(&path).unwrap();
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).unwrap();
    } else {
        // TODO: Bad idea if extensions do not match ...
        fs::copy(home_dir().unwrap().join(".prontus").join("default.jpg"), &path).unwrap();
        thread::spawn(move || {
            save_url(&url, &path);
            let mut file = OpenOptions::new().read(true).open(&path).unwrap();
            let mut buffer = Vec::new();
            file.read_to_end(&mut buffer).unwrap();
            buffer
        });
    }
    p
}

pub fn load_image(url: String) -> image::RgbaImage {
    let path = load_url_path(url);
    let reader = image::io::Reader::open(&path)
        .unwrap()
        .with_guessed_format()
        .unwrap();
    reader.decode().unwrap().to_rgba8()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_image() {
        let image = load_image("https://www.google.com/images/branding/googlelogo/1x/googlelogo_color_272x92dp.png".to_string());
    }
}
