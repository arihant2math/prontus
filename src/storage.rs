use std::{fs, thread};
use std::fs::OpenOptions;
use std::io::{Read, Write};
use std::path::PathBuf;
use base64::prelude::*;
use home::home_dir;

use reqwest::Client;

pub fn get_image_path(url: &str) -> PathBuf {
    let ext = url.split(".").last();
    // URL safe is also file safe (to me)
    let filename = BASE64_URL_SAFE.encode(url);
    home_dir().unwrap().join(".prontus").join(filename)
}

async fn save_url(client: &Client, url: &str, file: &PathBuf) {
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(file).unwrap();
    let mut response = client.get(url).await.unwrap();
    let bytes = response.bytes().await.unwrap();
    file.write_all(&bytes).unwrap();
}

pub fn load_url_path(client: &Client, url: String) -> PathBuf {
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
            save_url(client, &url, &path);
            let mut file = OpenOptions::new().read(true).open(&path).unwrap();
            let mut buffer = Vec::new();
            file.read_to_end(&mut buffer).unwrap();
            buffer
        });
    }
    p
}

pub fn load_image(client: &Client, url: String) -> image::RgbaImage {
    let path = load_url_path(client, url);
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
        let image = load_image(&Client::new(), "https://www.google.com/images/branding/googlelogo/1x/googlelogo_color_272x92dp.png".to_string());
    }
}
