use crate::client::ProntoClient;
use crate::util::{base_dir, image_dir};
use base64::prelude::BASE64_URL_SAFE;
use base64::Engine;
use futures_util::StreamExt;
use log::info;
use slint::{Rgba8Pixel, SharedPixelBuffer};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use thiserror::Error;
use tokio::fs::OpenOptions;
use tokio::io::AsyncWriteExt;

#[derive(Debug, Error)]
pub enum StorageError {
    #[error("IO error: {0}")]
    IOError(#[from] std::io::Error),
    #[error("Reqwest error: {0}")]
    ReqwestError(#[from] reqwest::Error),
    #[error("Image error: {0}")]
    ImageError(#[from] image::ImageError),
}

pub fn get_url_path(url: &str) -> PathBuf {
    // URL safe is also file safe (to me)
    let filename = BASE64_URL_SAFE.encode(url);
    image_dir().join(filename)
}

async fn save_url(
    client: Arc<ProntoClient>,
    url: &str,
    path: &PathBuf,
) -> Result<(), StorageError> {
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(path)
        .await?;
    info!("Downloading {}...", url);

    let mut stream = client.http_client.get(url).send().await?.bytes_stream();

    while let Some(chunk_result) = stream.next().await {
        let chunk = chunk_result?;
        file.write_all(&chunk).await?;
    }

    file.flush().await?;

    info!("Downloaded {}", url);
    Ok(())
}

pub fn load_image_from_path(path_buf: &PathBuf) -> Result<image::RgbaImage, image::ImageError> {
    let reader = image::io::Reader::open(path_buf)
        .unwrap()
        .with_guessed_format()
        .unwrap();
    reader.decode().map(|image| image.to_rgba8())
}

#[derive(Clone)]
pub struct ImageService {
    client: Arc<ProntoClient>,
    images: HashMap<String, SharedPixelBuffer<Rgba8Pixel>>,
    loading_image: SharedPixelBuffer<Rgba8Pixel>,
}

impl ImageService {
    pub fn new(client: Arc<ProntoClient>) -> Self {
        let loading_image = load_image_from_path(&base_dir().join("default.jpg")).unwrap();
        let mut r = Self {
            images: HashMap::new(),
            loading_image: SharedPixelBuffer::<Rgba8Pixel>::clone_from_slice(
                loading_image.as_raw(),
                loading_image.width(),
                loading_image.height(),
            ),
            client,
        };
        r.init();
        r
    }

    pub fn init(&mut self) {
        // list of images in image cache folder
        self.images = HashMap::new();
        let images = std::fs::read_dir(image_dir()).unwrap();
        for image in images {
            let image = image.unwrap();
            let path = image.path();
            if path.is_file() {
                let image_buffer = load_image_from_path(&path);
                if let Ok(image_buffer) = image_buffer {
                    self.images.insert(
                        path.file_name().unwrap().to_str().unwrap().to_string(),
                        SharedPixelBuffer::<Rgba8Pixel>::clone_from_slice(
                            image_buffer.as_raw(),
                            image_buffer.width(),
                            image_buffer.height(),
                        ),
                    );
                }
            }
        }
    }

    pub async fn get(&mut self, url: &str) -> Result<SharedPixelBuffer<Rgba8Pixel>, StorageError> {
        let path = get_url_path(&url);
        let filename = path.file_name().unwrap().to_str().unwrap();
        match self.images.get(filename) {
            Some(image) => Ok(image.clone()),
            None => {
                save_url(self.client.clone(), url, &path).await?;
                let image = load_image_from_path(&path)?;
                let buffer = SharedPixelBuffer::<Rgba8Pixel>::clone_from_slice(
                    image.as_raw(),
                    image.width(),
                    image.height(),
                );
                self.images.insert(filename.to_string(), buffer.clone());
                Ok(buffer)
            }
        }
    }

    pub fn block_get(&mut self, url: &str) -> Result<SharedPixelBuffer<Rgba8Pixel>, StorageError> {
        tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(self.get(url))
    }

    pub fn exists(&self, url: &str) -> bool {
        self.images.contains_key(url)
    }

    pub fn fast_get(&self, url: &str) -> Option<SharedPixelBuffer<Rgba8Pixel>> {
        self.images.get(url).cloned()
    }

    pub fn loading_image(&self) -> SharedPixelBuffer<Rgba8Pixel> {
        self.loading_image.clone()
    }
}
