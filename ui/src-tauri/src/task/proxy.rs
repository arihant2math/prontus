use ui_lib::AppState;
use client::ProntoClient;
use hyper::body::Incoming;
use hyper::server::conn::http1;
use hyper::service::Service;
use hyper::{Request, Response};
use hyper_util::rt::TokioIo;
use log::error;
use std::future::Future;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;

pub struct ServiceHandler {
    client: Arc<ProntoClient>,
}

impl ServiceHandler {
    pub fn new(client: Arc<ProntoClient>) -> Self {
        Self { client }
    }
}

impl Service<Request<Incoming>> for ServiceHandler {
    type Response = Response<reqwest::Body>;
    type Error = reqwest::Error;
    type Future =
        std::pin::Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn call(&self, req: Request<Incoming>) -> Self::Future {
        let client = self.client.clone();
        Box::pin(async move {
            let response = client
                .http_client
                .request(
                    req.method().clone(),
                    format!(
                        "https://stanfordohs.pronto.io{}",
                        req.uri().clone().to_string()
                    ),
                )
                .send()
                .await?;
            Ok(response.into())
        })
    }
}

pub async fn run(
    context: AppState,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    loop {
        if context.is_loaded().await {
            break;
        }
    }
    let client = {
        let state = context.inner();
        let state = state.read().await;
        let state = state.try_inner()?;
        state.client.clone()
    };

    let addr = SocketAddr::from(([127, 0, 0, 1], 10521));

    // We create a TcpListener and bind it to 127.0.0.1:3000
    let listener = TcpListener::bind(addr).await?;

    // We start a loop to continuously accept incoming connections
    loop {
        let (stream, _) = listener.accept().await?;

        // Use an adapter to access something implementing `tokio::io` traits as if they implement
        // `hyper::rt` IO traits.
        let io = TokioIo::new(stream);

        // Spawn a tokio task to serve multiple connections concurrently
        tokio::task::spawn({
            let client = client.clone();
            async move {
                // Finally, we bind the incoming connection to our `hello` service
                if let Err(err) = http1::Builder::new()
                    // `service_fn` converts our function in a `Service`
                    .serve_connection(io, ServiceHandler::new(client.clone()))
                    .await
                {
                    error!("Error serving connection: {:?}", err);
                }
            }
        });
    }
}
