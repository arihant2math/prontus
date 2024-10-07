/// # Probot
/// Probot is a framework for building bots for Pronto.
///
/// ## Example
///
/// A simple noop example:
///
/// ```no_run
/// use probot::{BotBuilder, NoopHandler};
///
/// #[tokio::main]
/// async fn main() {
///     let mut bot = BotBuilder::new()
///     .load_client("https://stanfordohs.pronto.io/api/".to_string(), "[your token here]".to_string(), 0)
///     .handler(NoopHandler)
///     .build()
///     .await;
///     bot.init().await;
///     bot.run().await;
/// }
/// ```
///
/// To do anything useful you'll have to implement a handler. Here is an example of a handler that responds messages with the text "hi".
///
/// ```
/// use probot::{ProntoClient, Handler};
/// use probot::pusher::PusherServerEventType;
///
/// pub struct HelloHandler;
///
/// impl Handler for HelloHandler {
///     type Error = Box<dyn error::Error + Send + Sync>;
///     type Future = std::pin::Pin<Box<dyn Future<Output=Result<(), Self::Error>> + Send>>;
///
///     async fn handle(&self, pronto_client: ProntoClient, input: PusherServerEventType) -> Self::Future {
///         Box::pin(async {
///             let input = match input {
///                 PusherServerEventType::PusherServerMessageAddedEvent(message) => message,
///                 _ => return Ok(()),
///             };
///             if input.message.message.clone().to_lowercase().contains("hi") {
///                 let user_info = pronto_client.user_info(None).await.unwrap().user;
///                 pronto_client.send_message(user_info.id, input.message.bubble_id, response, None).await?;
///             }
///             Ok(())
///         })
///     }
/// }
/// ```
///
///
mod handler;

pub(crate) use handler::HandlerWrapper;
pub use handler::{handler, Command, CommandHandler, Handler, NoopHandler};
use std::collections::HashMap;

use async_trait::async_trait;
pub use client;
pub use client::ProntoClient;
use log::{error, warn};
pub use pusher;
use pusher::{
    PusherClient, PusherServerEventType, PusherServerMessage, PusherServerMessageWrapper,
};
use std::future::Future;
use std::sync::Arc;

pub trait TokenLoader {
    type Error;
    type Future: Future<Output = Result<Option<String>, Self::Error>> + Send;

    fn load(&self, user_id: u64) -> Self::Future;
}

impl TokenLoader for String {
    type Error = Box<dyn error::Error + Send + Sync>;
    type Future =
        std::pin::Pin<Box<dyn Future<Output = Result<Option<String>, Self::Error>> + Send>>;

    fn load(&self, _: u64) -> Self::Future {
        Box::pin(async { Ok(Some(self.clone())) })
    }
}

impl TokenLoader for HashMap<u64, String> {
    type Error = Box<dyn error::Error + Send + Sync>;
    type Future =
        std::pin::Pin<Box<dyn Future<Output = Result<Option<String>, Self::Error>> + Send>>;

    fn load(&self, user_id: u64) -> Self::Future {
        Box::pin(async { Ok(self.get(&user_id).cloned()) })
    }
}

pub struct Bot {
    client: Arc<ProntoClient>,
    pusher_client: PusherClient,
    handler: HandlerWrapper,
    inited: bool,
}

impl Bot {
    pub async fn new(client: Arc<ProntoClient>, handler: HandlerWrapper) -> Self {
        let pusher_client = PusherClient::new(client.clone()).await;
        Self {
            client,
            pusher_client,
            handler,
            inited: false,
        }
    }

    pub async fn init(&mut self) {
        self.inited = true;
        self.pusher_client.init().await;
        let user_info = self.client.user_info(None).await.unwrap().user;
        self.pusher_client
            .subscribe(format!(
                "private-organization.{}",
                user_info.organizations[0].id
            ))
            .await;
        self.pusher_client
            .subscribe(format!("private-user.{}", user_info.id))
            .await;
    }

    pub async fn run(&self) {
        loop {
            let message = self.pusher_client.server_messages().await.recv().await;
            match message {
                Ok(PusherServerMessageWrapper::PusherServerMessage(message)) => match message {
                    PusherServerMessage::Event(event) => {
                        self.handler.handle(self.client.clone(), event.event).await;
                    }
                    PusherServerMessage::Error(e) => {
                        error!("Received error: {:?}", e);
                    }
                    PusherServerMessage::Other(raw) => {
                        warn!("Received unknown message: {:?}", raw);
                    }
                    _ => {}
                },
                Err(e) => {
                    error!("Error receiving message: {:?}", e);
                }
                _ => {}
            }
        }
    }
}

pub struct BotBuilder {
    client: Option<Arc<ProntoClient>>,
    handler: Option<HandlerWrapper>,
}

impl BotBuilder {
    pub fn new() -> Self {
        Self {
            client: None,
            handler: None,
        }
    }

    pub fn client(mut self, client: Arc<ProntoClient>) -> Self {
        self.client = Some(client);
        self
    }

    pub fn load_client(
        mut self,
        base_url: String,
        token_loader: impl TokenLoader,
        user_id: u64,
    ) -> Self {
        self.client = Some(Arc::new(
            ProntoClient::new(base_url, token_loader.load(user_id).unwrap()).unwrap(),
        ));
        self
    }

    pub fn handler(
        mut self,
        handler: impl Handler<
                Error = Box<dyn error::Error + Send + Sync>,
                Future = std::pin::Pin<dyn Future<Output = Result<(), Handler::Error>>>,
            > + 'static,
    ) -> Self {
        self.handler = Some(HandlerWrapper::new(handler));
        self
    }

    pub async fn build(self) -> Bot {
        Bot::new(self.client.unwrap(), self.handler.unwrap()).await
    }
}
