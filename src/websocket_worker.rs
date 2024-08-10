use futures_util::{SinkExt, StreamExt};
use log::{debug, error, info};
use pusher::PusherBuilder;
use serde::{Deserialize, Serialize};
use slint::Weak;
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message;

use client::ProntoClient;
use crate::settings::Settings;
use crate::AppWindow;

#[derive(Clone, Debug)]
pub enum WebsocketTasks {
    ChangeChannel(u64),
    SubscribeUser(u64),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ConnectionEstablished {
    pub socket_id: String,
    pub activity_timeout: u16,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "event")]
pub enum PusherResponse {
    #[serde(rename = "pusher:ping")]
    Ping { data: String },
    #[serde(rename = "pusher:connection_established")]
    ConnectionEstablished { data: String },
    #[serde(rename = "pusher:error")]
    Error { data: String },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum PusherRequest {
    #[serde(rename = "pusher:subscribe")]
    Subscribe { channel: String },
    #[serde(rename = "pusher:unsubscribe")]
    Unsubscribe { channel: String },
    #[serde(rename = "pusher:pong")]
    Pong { data: String },
}

pub async fn worker(
    _ui: Weak<AppWindow>,
    mut rx: mpsc::Receiver<WebsocketTasks>,
) -> Result<(), ()> {
    let settings = Settings::load("settings.json").unwrap();
    let client = if let Some(pronto_api_token) = settings.pronto_api_token {
        Arc::new(ProntoClient::new(settings.base_url.clone(), &pronto_api_token.clone()).unwrap())
    } else {
        panic!("No Pronto API token provided");
    };
    // TODO
    Ok(())
}
