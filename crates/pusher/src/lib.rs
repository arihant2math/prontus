mod message;

use client::ProntoClient;
use futures_util::{SinkExt, StreamExt};
use log::error;
pub use message::*;
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use tokio::net::TcpStream;
use tokio::sync::{broadcast, mpsc, RwLock};
use tokio_tungstenite::tungstenite::{Bytes, Message, Utf8Bytes};
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};

#[derive(Clone, Debug)]
pub enum PusherServerMessageWrapper {
    PusherServerMessage(PusherServerMessage),
    Ping,
    Shutdown,
}

async fn read_task(
    stream: Arc<RwLock<WebSocketStream<MaybeTlsStream<TcpStream>>>>,
    message_output: broadcast::Sender<PusherServerMessageWrapper>,
) {
    loop {
        let message = stream.write().await.next().await;
        let message = match message {
            Some(message) => message,
            _ => {
                break;
            }
        };
        match message {
            Ok(Message::Text(message)) => {
                let data: PusherServerMessage = PusherServerMessage::from(message.as_str().to_string());
                let _ = message_output.send(PusherServerMessageWrapper::PusherServerMessage(data.into()));
            }
            Ok(Message::Ping(_)) => {
                let _ = message_output.send(PusherServerMessageWrapper::Ping);
            }
            Ok(Message::Close(_)) => {
                let _ = message_output.send(PusherServerMessageWrapper::Shutdown);
                break;
            }
            Err(e) => {
                error!("Error: {:?}", e);
            }
            _ => {}
        }
    }
}

#[derive(Clone, Debug)]
pub enum PusherClientMessageWrapper {
    PusherClientMessage(PusherClientMessage),
    Pong,
    Shutdown,
}

async fn write_task(
    stream: Arc<RwLock<WebSocketStream<MaybeTlsStream<TcpStream>>>>,
    mut message_input: mpsc::Receiver<PusherClientMessageWrapper>,
) {
    loop {
        let message = message_input.recv().await;
        let message = match message {
            Some(message) => message,
            _ => {
                break;
            }
        };
        match message {
            PusherClientMessageWrapper::PusherClientMessage(pcm) => {
                let _ = stream
                    .write()
                    .await
                    .send(Message::Text(Utf8Bytes::from(pcm.to_string())))
                    .await;
            }
            PusherClientMessageWrapper::Pong => {
                let _ = stream.write().await.send(Message::Pong(Bytes::new())).await;
            }
            PusherClientMessageWrapper::Shutdown => {
                let _ = stream.write().await.send(Message::Close(None)).await;
                break;
            }
        }
    }
}

async fn ping_task(
    mut server_messages: broadcast::Receiver<PusherServerMessageWrapper>,
    client_message: mpsc::Sender<PusherClientMessageWrapper>,
) {
    // Respond to all pings with pongs to ensure the connection doesn't close
    loop {
        let message = server_messages.recv().await;
        let message = match message {
            Ok(message) => message,
            _ => {
                break;
            }
        };
        match message {
            PusherServerMessageWrapper::Ping => {
                let _ = client_message.send(PusherClientMessageWrapper::Pong).await;
            }
            PusherServerMessageWrapper::Shutdown => {
                let _ = client_message
                    .send(PusherClientMessageWrapper::Shutdown)
                    .await;
                break;
            }
            _ => {}
        }
    }
}

#[tokio::main]
async fn task_thread(
    ws_stream: Arc<RwLock<WebSocketStream<MaybeTlsStream<TcpStream>>>>,
    server_messages_tx: broadcast::Sender<PusherServerMessageWrapper>,
    server_messages_rx: broadcast::Receiver<PusherServerMessageWrapper>,
    client_messages_tx: mpsc::Sender<PusherClientMessageWrapper>,
    client_messages_rx: mpsc::Receiver<PusherClientMessageWrapper>,
) {
    let write_task = write_task(ws_stream.clone(), client_messages_rx);
    let ping_task = ping_task(server_messages_rx, client_messages_tx);
    let read_task = read_task(ws_stream, server_messages_tx);
    let wt = tokio::task::spawn(write_task);
    let pt = tokio::task::spawn(ping_task);
    let rt = tokio::task::spawn(read_task);
    let _ = tokio::join!(wt, pt, rt);
    // TODO: Remove below after testing (the join should be enough)
    loop {
        tokio::time::sleep(Duration::from_secs(5)).await;
    }
}

/// A pusher client handles sending and receiving messages to and from pusher.
/// The communication happens on a thread because tokio task scoping is not great for this,
/// although this might change in the future.
#[derive(Clone)]
pub struct PusherClient {
    client: Arc<ProntoClient>,
    server_messages: Arc<RwLock<broadcast::Receiver<PusherServerMessageWrapper>>>,
    client_message: Arc<RwLock<mpsc::Sender<PusherClientMessageWrapper>>>,
    details: Arc<RwLock<Option<PusherServerConnectionEstablished>>>,
}

impl PusherClient {
    /// Connect to the pusher socket, spawn the communication thread, and initialize channels
    pub async fn new(client: Arc<ProntoClient>) -> Self {
        // TODO: make this portable
        let (ws_stream, _) = connect_async("wss://ws-mt1.pusher.com/app/f44139496d9b75f37d27?protocol=7&client=js&version=8.3.0&flash=false")
            .await
            .unwrap();
        let ws_stream = Arc::new(RwLock::new(ws_stream));
        let (message_output_tx, message_output_rx) = broadcast::channel(128);
        let (message_input_tx, message_input_rx) = mpsc::channel(128);

        thread::spawn({
            let message_output_rx = message_output_tx.subscribe();
            let message_input_tx = message_input_tx.clone();
            move || {
                task_thread(
                    ws_stream,
                    message_output_tx,
                    message_output_rx,
                    message_input_tx,
                    message_input_rx,
                );
            }
        });

        let client = Self {
            client,
            server_messages: Arc::new(RwLock::new(message_output_rx)),
            client_message: Arc::new(RwLock::new(message_input_tx)),
            details: Arc::new(RwLock::new(None)),
        };

        client
    }

    pub async fn init(&self) {
        loop {
            let message = self.server_messages().await.recv().await.unwrap();
            match message {
                PusherServerMessageWrapper::PusherServerMessage(
                    PusherServerMessage::ConnectionEstablished(details),
                ) => {
                    let mut details_guard = self.details.write().await;
                    *details_guard = Some(details);
                    break;
                }
                _ => {}
            }
        }
    }

    /// Get authentication and subscribe to a channel
    pub async fn subscribe(&self, channel: String) {
        let details = self.details.read().await.clone().unwrap();
        let message =
            PusherClientMessage::subscribe(self.client.clone(), &details.socket_id, &channel).await;
        let _ = self
            .client_message()
            .await
            .send(PusherClientMessageWrapper::PusherClientMessage(message))
            .await;
    }

    /// Get a broadcast receiver for server messages
    pub async fn server_messages(&self) -> broadcast::Receiver<PusherServerMessageWrapper> {
        self.server_messages.read().await.resubscribe()
    }

    /// Get a mpsc sender for sending client messages
    pub async fn client_message(&self) -> mpsc::Sender<PusherClientMessageWrapper> {
        self.client_message.read().await.clone()
    }

    // TODO: this is an async drop
    pub async fn shutdown(&self) {
        self.client_message.read().await.send(PusherClientMessageWrapper::Shutdown).unwrap();
    }
}
