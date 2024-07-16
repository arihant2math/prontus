use std::sync::Arc;
use futures_util::{SinkExt, StreamExt};
use log::{error, info};
use serde::{Deserialize, Serialize};
use slint::Weak;
use tokio::sync::mpsc;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message;

use crate::AppWindow;
use crate::client::ProntoClient;
use crate::settings::Settings;

#[derive(Clone, Debug)]
pub enum WebsocketTasks {
    ChangeChannel(u64),
    SubscribeUser(u64),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ConnectionEstablished {
    pub socket_id: String,
    pub activity_timeout: u16
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

#[allow(unused_mut, unused_variables)]
pub async fn worker(_ui: Weak<AppWindow>, mut rx: mpsc::Receiver<WebsocketTasks>) -> Result<(), ()> {
    let settings = Settings::load("settings.json").unwrap();
    let client = if let Some(pronto_api_token) = settings.pronto_api_token {
        Arc::new(ProntoClient::new(settings.base_url.clone(), &pronto_api_token.clone()).unwrap())
    } else {
        panic!("No Pronto API token provided");
    };
    let url = url::Url::parse("wss://ws-mt1.pusher.com/app/f44139496d9b75f37d27?protocol=7&client=js&version=8.3.0&flash=false").unwrap();
    // let (mut ws_stream, _) = connect_async(url).await.expect("Failed to connect");

    let auth = client.pusher_auth("707489.396716", "private-organization.2245").await.unwrap().auth;
    let auth_string = format!("f44139496d9b75f37d27.w{auth}");

    let json = serde_json::json!({
        "event": "pusher:subscribe",
        "data": {
            "auth": auth_string,
            "channel": "private-organization.2245"
        }
    });
    // ws_stream.send(Message::Text(json.to_string())).await.unwrap();
    loop {
    //     match ws_stream.next().await {
    //         Some(Ok(Message::Text(text))) => {
    //             println!("Received: {text}");
    //             if let Ok(response) = serde_json::from_str::<PusherResponse>(&text) {
    //                 match response {
    //                     #[allow(unused_variables)]
    //                     PusherResponse::ConnectionEstablished { data } => {
    //                         info!("Responding with pong");
    //                         ws_stream.send(Message::Pong(vec![])).await.unwrap();
    //                     }
    //                     r => {
    //                         println!("Other: {r:?}");
    //                     }
    //                 }
    //             }
    //         }
    //         Some(Ok(Message::Ping(_))) => {
    //             ws_stream.send(Message::Pong(vec![])).await.unwrap();
    //         }
    //         Some(Ok(Message::Close(_))) => {
    //             error!("Connection closed");
    //         }
    //         Some(Ok(Message::Binary(_))) => {
    //             error!("Received binary message");
    //         }
    //         _ => {}
    //     }
    }
    // TODO
    Ok(())
}