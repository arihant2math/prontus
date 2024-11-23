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
///     .await
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
/// use std::error;
/// use std::future::Future;
/// use std::sync::Arc;
/// use probot::{ProntoClient, Handler};
/// use probot::pusher::PusherServerEventType;
///
/// pub struct HelloHandler;
///
/// impl Handler for HelloHandler {
///     type Error = Box<dyn error::Error + Send + Sync>;
///
///     async fn handle(&self, pronto_client: Arc<ProntoClient>, input: PusherServerEventType) -> Result<(), Self::Error> {
///         let input = match input {
///             PusherServerEventType::PusherServerMessageAddedEvent(message) => message,
///                 _ => return Ok(()),
///             };
///         if input.message.message.clone().to_lowercase().contains("hi") {
///             let user_info = pronto_client.user_info(None).await.unwrap().user;
///             pronto_client.send_message(user_info.id, input.message.bubble_id, "hello".to_string(), None).await?;
///         }
///         Ok(())
///     }
/// }
/// ```
///
///
mod handler;

pub use handler::{handler, Command, CommandHandler, Handler, NoopHandler};
use std::collections::HashMap;

pub use client;
pub use client::ProntoClient;
use log::{error, warn};
pub use pusher;
use pusher::{
    PusherClient, PusherServerMessage, PusherServerMessageWrapper,
};
use std::error;
use std::sync::Arc;

pub trait TokenLoader {
    type Error: error::Error;

    #[allow(async_fn_in_trait)]
    async fn load(&self, user_id: u64) -> Result<Option<String>, Self::Error>;
}

#[derive(Copy, Clone, Debug)]
pub struct Never;

impl error::Error for Never {}

impl std::fmt::Display for Never {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Never")
    }
}

impl TokenLoader for String {
    type Error = Never;

    async fn load(&self, _: u64) -> Result<Option<String>, Self::Error> {
        Ok(Some(self.clone()))
    }
}

impl TokenLoader for HashMap<u64, String> {
    type Error = Never;

    async fn load(&self, user_id: u64) -> Result<Option<String>, Self::Error> {
        Ok(self.get(&user_id).cloned())
    }
}

pub struct Bot<T: Handler> {
    client: Arc<ProntoClient>,
    pusher_client: PusherClient,
    handler: T,
    inited: bool,
}

impl<T: Handler> Bot<T> {
    pub async fn new(client: Arc<ProntoClient>, handler: T) -> Self {
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

pub struct BotBuilder<T: Handler> {
    client: Option<Arc<ProntoClient>>,
    handler: Option<T>,
}

impl<T: Handler> BotBuilder<T> {
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

    pub async fn load_client(
        mut self,
        base_url: String,
        token_loader: impl TokenLoader,
        user_id: u64,
    ) -> Self {
        self.client = Some(Arc::new(
            ProntoClient::new(base_url, &token_loader.load(user_id).await.unwrap().unwrap()).unwrap(),
        ));
        self
    }

    pub fn handler(
        mut self,
        handler: T,
    ) -> Self {
        self.handler = Some(handler);
        self
    }

    pub async fn build(self) -> Bot<T> {
        Bot::new(self.client.unwrap(), self.handler.unwrap()).await
    }
}
