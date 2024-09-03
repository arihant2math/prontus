use std::fmt::Debug;
use std::sync::Arc;
use futures_util::{SinkExt, StreamExt};
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message;
use client::ProntoClient;
use pusher::{PusherClientMessage, PusherServerMessage};

#[tokio::main]
async fn main() {
    let client = Arc::new(ProntoClient::new("https://stanfordohs.pronto.io/api/".to_string(),
                                            "DdGfHDsYKsIF9D3ZIXKShiXEUUf46Us5bXA4tSRj.1227720825")
        .unwrap());

    let (mut ws_stream, _) = connect_async("wss://ws-mt1.pusher.com/app/f44139496d9b75f37d27?protocol=7&client=js&version=8.3.0&flash=false")
        .await
        .unwrap();

    let mut socket_id: Option<String>;
    let mut activity_timeout: Option<u64> = None;

    loop {
        let message = ws_stream.next().await.unwrap();
        match message {
            Ok(message) => {
                match message {
                    Message::Ping(_) => {
                        ws_stream.send(Message::Pong(vec![])).await.unwrap();
                    }
                    Message::Text(text) => {
                        let data: PusherServerMessage = PusherServerMessage::from(text);
                        match data {
                            PusherServerMessage::ConnectionEstablished(ce) => {
                                socket_id = Some(ce.socket_id);
                                activity_timeout = Some(ce.activity_timeout);
                                println!("Connection established with id {socket_id:?}", );
                                println!("Subscribing to private-organization.2245");
                                // subscribe to private-organization.2245
                                let message = PusherClientMessage::subscribe(client.clone(), socket_id.as_ref().unwrap(), "private-organization.2245").await;
                                ws_stream.send(Message::Text(message.to_string())).await.unwrap();
                                println!("Subscribing to private-user.5302428");
                                let message = PusherClientMessage::subscribe(client.clone(), socket_id.as_ref().unwrap(), "private-user.5302428").await;
                                ws_stream.send(Message::Text(message.to_string())).await.unwrap();
                            }
                            PusherServerMessage::SubscriptionSucceeded(event) => {
                                println!("Subscription succeeded for channel {}", event.channel);
                            }
                            PusherServerMessage::Other(other) => {
                                println!("Other: {:?}", other);
                            }
                            PusherServerMessage::Error(e) => {
                                println!("Error: {:?}", e);
                            }
                            o => {
                                println!("Other: {:?}", o);
                            }
                        }
                    }
                    other => {
                        println!("Other: {:?}", other);
                    }
                }
            }
            Err(e) => {
                println!("Error: {:?}", e);
            }
        }
    }
}
