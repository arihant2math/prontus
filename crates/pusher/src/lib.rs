use tokio::net::TcpStream;
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};

pub struct Pusher {
    web_socket: WebSocketStream<MaybeTlsStream<TcpStream>>,
}

impl Pusher {
    pub async fn connect(&mut self) {
        self.web_socket.await.unwrap();
    }
}

pub struct PusherBuilder {
    web_socket_address: WebSocketAddress,
}

impl PusherBuilder {
    pub fn new(web_socket_address: WebSocketAddress) -> Self {
        PusherBuilder { web_socket_address }
    }

    pub async fn build(self) -> Pusher {
        let (web_socket, response) = connect_async(&self.web_socket_address.url).await.unwrap();
        Pusher { web_socket }
    }
}

pub struct WebSocketAddress {
    url: String,
}

impl WebSocketAddress {
    pub fn new(cluster_name: &str, key: &str, secure: bool) -> Self {
        let version = env!("CARGO_PKG_VERSION");
        if secure {
            WebSocketAddress {
                url: format!("wss://ws-mt1.pusher.com/app/{key}?protocol=7&client=rust&version={version}&flash=false")
            }
        } else {
            WebSocketAddress {
                url: format!("ws://ws-mt1.pusher.com/app/{key}?protocol=7&client=rust&version={version}&flash=false")
            }
        }
    }
}
