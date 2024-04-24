use std::{fs, thread};
use std::fs::OpenOptions;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::sync::Arc;
use base64::prelude::*;
use home::home_dir;
use slint::ModelTracker;

use crate::client::ProntoClient;

pub fn get_image_path(url: &str) -> PathBuf {
    let ext = url.split(".").last();
    // URL safe is also file safe (to me)
    let filename = BASE64_URL_SAFE.encode(url);
    home_dir().unwrap().join(".prontus").join(filename)
}

async fn save_url(client: Arc<ProntoClient>, url: &str, file: &PathBuf) {
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(file).unwrap();
    let response = client.http_client.get(url).send().await.unwrap();
    let bytes = response.bytes().await.unwrap();
    file.write_all(&bytes).unwrap();
}

pub async fn load_url_path(client: Arc<ProntoClient>, url: String) -> PathBuf {
    let path = get_image_path(&url);
    let p = path.clone();
    if path.exists() {
        let mut file = OpenOptions::new().read(true).open(&path).unwrap();
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).unwrap();
    } else {
        save_url(client, &url, &path).await;
    }
    p
}

pub async fn load_image(client: Arc<ProntoClient>, url: String) -> image::RgbaImage {
    let path = load_url_path(client, url).await;
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
