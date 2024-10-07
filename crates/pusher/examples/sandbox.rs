use client::ProntoClient;
use pusher::PusherClient;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let client = Arc::new(
        ProntoClient::new(
            "https://stanfordohs.pronto.io/api/".to_string(),
            "[insert token here]",
        )
        .unwrap(),
    );

    let pusher_client = PusherClient::new(client).await;
    let mut sub = pusher_client.server_messages().await;
    pusher_client.init().await;
    loop {
        let message = sub.recv().await;
        match message {
            Ok(message) => {
                println!("{:?}", message);
            }
            _ => {
                break;
            }
        }
    }
}
