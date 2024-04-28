use futures_util::StreamExt;
use slint::Weak;
use tokio::sync::mpsc;
use tokio_tungstenite::connect_async;

use crate::AppWindow;

#[derive(Clone, Debug)]
enum WebsocketTasks {
    ChangeChannel(u64)
}


pub async fn websocket_worker(_ui: Weak<AppWindow>, rx: mpsc::Receiver<WebsocketTasks>) {
    let url = url::Url::parse("wss://ws-mt1.pusher.com/app/f44139496d9b75f37d27?protocol=7&client=js&version=8.3.0&flash=false").unwrap();
    let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");

    let (write, read) = ws_stream.split();

    read.for_each(|message| async move {
        println!("{:?}", message);
    }).await;
    // TODO
}