use futures_util::{future, pin_mut, SinkExt, StreamExt};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::handshake::client::Request;
use tokio_tungstenite::tungstenite::Message;

async fn read_stdin(tx: futures_channel::mpsc::UnboundedSender<Message>) {
    let mut stdin = tokio::io::stdin();
    loop {
        let mut buf = vec![0; 1024];
        let n = match stdin.read(&mut buf).await {
            Err(_) | Ok(0) => break,
            Ok(n) => n,
        };
        buf.truncate(n);
        tx.unbounded_send(Message::binary(buf)).unwrap();
    }
}

#[tokio::main]
async fn main() {
    // wss://ws-mt1.pusher.com/app/f44139496d9b75f37d27?protocol=7&client=js&version=8.3.0&flash=false
    let initial_request = Request::builder()
        .uri("wss://ws-mt1.pusher.com/app/f44139496d9b75f37d27?protocol=7&version=1.0.0")
        .method("GET")
        .header("Accept", "*/*")
        .header("Accept-Encoding", "gzip, deflate, br")
        .header("Accept-Language", "en-US,en;q=0.5")
        .header("Cache-Control", "no-cache")
        .header("Connection", "keep-alive, Upgrade")
        .header("DNT", "1")
        .header("Host", "ws-mt1.pusher.com")
        .header("Origin", "https://stanfordohs.pronto.io")
        .header("Pragma", "no-cache")
        .header("Sec-Fetch-Dest", "empty")
        .header("Sec-Fetch-Mode", "websocket")
        .header("Sec-Fetch-Site", "cross-site")
        .header("Sec-Fetch-GPC", "1")
        .header("Sec-WebSocket-Extensions", "permessage-deflate")
        .header("Sec-WebSocket-Key", "hCJh3Rq1twbZP9wJBNs/9w==")
        .header("Sec-WebSocket-Version", "13")
        .header("Upgrade", "websocket")
        .body(()).unwrap();

    let (stdin_tx, stdin_rx) = futures_channel::mpsc::unbounded();
    tokio::spawn(read_stdin(stdin_tx));

    let (mut ws_stream, _) = connect_async(initial_request)
        .await
        .unwrap();

    let m1 = r#"{"event":"pusher:subscribe","data":{"auth":"f44139496d9b75f37d27:8eec9fb482c6566096a08f1b64aa85b83a8f3fc3bee50fe9d52ccefa4e9dca1c","channel":"private-organization.2245"}}"#;
    let m2 = r#"{"event":"pusher:subscribe","data":{"auth":"f44139496d9b75f37d27:c6c60fb49b9780cfc8a234beb480c7220bc972da8d7a1852c1654fa3e102b13c","channel":"private-user.5302428"}}"#;
    let m3 = r#"{"event":"pusher:subscribe","data":{"auth":"f44139496d9b75f37d27:83c28b904087efb705cd8852dd604d566386857b9877554a59401d43c6382a01","channel":"private-bubble.3729522.NVeMQpBjWNn5JwUCOErZivL6C7gQxn6yDNxNB8WA"}}"#;


    let (write, read) = ws_stream.split();

    let stdin_to_ws = stdin_rx.map(Ok).forward(write);
    let ws_to_stdout = {
        read.for_each(|message| async {
            let data = message.unwrap().into_data();
            tokio::io::stdout().write_all(&data).await.unwrap();
        })
    };

    pin_mut!(stdin_to_ws, ws_to_stdout);
    future::select(stdin_to_ws, ws_to_stdout).await;
}
