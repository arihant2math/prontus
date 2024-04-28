use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;
use std::sync::Arc;

use base64::prelude::*;
use home::home_dir;

use crate::client::ProntoClient;

pub fn get_url_path(url: &str) -> PathBuf {
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
    let path = get_url_path(&url);
    if !path.exists() {
        save_url(client, &url, &path).await;
    }
    path
}

pub fn load_image_path(path_buf: PathBuf) -> image::RgbaImage {
    let reader = image::io::Reader::open(&path_buf).unwrap().with_guessed_format().unwrap();
    reader
        .decode()
        .map(|image| image.to_rgba8())
        .unwrap_or_else(|err| {
            eprintln!("Failed to load image: {:?}", err);
            image::RgbaImage::default()
        })
}

pub async fn load_image(client: Arc<ProntoClient>, url: String) -> image::RgbaImage {
    let path = load_url_path(client, url).await;
    load_image_path(path)
}

pub fn fast_load_image(url: &str) -> Option<image::RgbaImage> {
    let path = get_url_path(url);
    if !path.exists() {
        return None;
    }
    Some(load_image_path(path))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_image() {
        let image = load_image(&Client::new(), "https://www.google.com/images/branding/googlelogo/1x/googlelogo_color_272x92dp.png".to_string());
    }
}
