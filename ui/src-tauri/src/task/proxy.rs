use client::ProntoClient;
use hyper::body::Incoming;
use hyper::server::conn::http1;
use hyper::service::Service;
use hyper::{Request, Response};
use hyper_util::rt::TokioIo;
use log::error;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use std::future::Future;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;
use ui_lib::AppState;

const PORT: u16 = 10521;

pub struct ServiceHandlerError {
    pub inner: reqwest::Error,
    pub req: Request<Incoming>,
}

impl Debug for ServiceHandlerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("ServiceHandlerError")
            .field("inner", &self.inner)
            .field("req", &self.req)
            .finish()
    }
}

impl Display for ServiceHandlerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} {}",
            self.req.method().to_string(),
            self.req.uri().to_string(),
            self.inner
        )
    }
}

impl std::error::Error for ServiceHandlerError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.inner)
    }
}

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
    type Error = ServiceHandlerError;
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
                .await
                .map_err(|e| ServiceHandlerError { inner: e, req })?;
            Ok(response.into())
        })
    }
}

pub async fn run(context: AppState) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    while !context.is_loaded() {
        break;
    }
    let client = {
        let state = context.try_inner()?;
        state.client.clone()
    };

    let addr = SocketAddr::from(([127, 0, 0, 1], PORT));

    // We create a TcpListener and bind it to the address
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
                // Finally, we bind the incoming connection to our service
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
