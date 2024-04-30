use std::sync::Arc;
use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use slint::Weak;
use tokio::sync::mpsc;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite;

use crate::{AppWindow, PRONTO_BASE_URL};
use crate::client::ProntoClient;
use crate::settings::Settings;

#[derive(Clone, Debug)]
pub enum WebsocketTasks {
    ChangeChannel(u64)
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "event")]
pub enum PusherResponse {
    #[serde(rename = "pusher:connection_established")]
    ConnectionEstablished { data: String },
}


pub async fn worker(_ui: Weak<AppWindow>, mut rx: mpsc::Receiver<WebsocketTasks>) -> Result<(), ()> {
    let settings = Settings::load("settings.json").unwrap();
    let client = if let Some(pronto_api_token) = settings.pronto_api_token {
        Arc::new(ProntoClient::new(PRONTO_BASE_URL.to_string(), &settings.pronto_session.clone().unwrap_or("".to_string()), &pronto_api_token.clone(), &settings.pacct.clone().unwrap_or("".to_string())).unwrap())
    } else {
        panic!("No Pronto API token provided");
    };
    let url = url::Url::parse("wss://ws-mt1.pusher.com/app/f44139496d9b75f37d27?protocol=7&client=js&version=8.3.0&flash=false").unwrap();
    let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");

    let auth = client.pusher_auth("707489.396716", "private-organization.2245").await.unwrap().auth;

    let (mut write, read) = ws_stream.split();

    let auth_string = format!("f44139496d9b75f37d27.w{auth}");
    let json = serde_json::json!({
        "event": "pusher:subscribe",
        "data": {
            "auth": auth_string,
            "channel": "private-organization.2245"
        }
    });

    read.for_each(|response| async move {
        match response {
            Ok(message) => {
                let response: PusherResponse = serde_json::from_str(&message.to_string()).unwrap();
                println!("Response: {:?}", response);
            },
            Err(e) => println!("Error: {:?}", e),
        }
    }).await;

    write.send(tungstenite::Message::from(serde_json::to_string(&json).unwrap())).await.unwrap();

    loop {
        let task = rx.recv().await.unwrap();
        match task {
            WebsocketTasks::ChangeChannel(channel_id) => { // TODO: Fix
                let json = serde_json::json!({
                "event": "pusher:subscribe",
                "data": {
                    "channel": format!("private-organization.{}", channel_id)
                }
            });
                write.send(tungstenite::Message::from(serde_json::to_string(&json).unwrap())).await.unwrap();
            }
        }
    }

    Ok(())
    // TODO
}