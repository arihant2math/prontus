mod handler;
pub use handler::{handler, Command, CommandHandler, Handler};
pub(crate) use handler::HandlerWrapper;

use std::future::Future;
use std::sync::Arc;
use async_trait::async_trait;
use client::ProntoClient;
use log::{error, warn};
use pusher::{PusherClient, PusherServerEventType, PusherServerMessage, PusherServerMessageWrapper};

pub trait TokenLoader {
    type Error;
    type Future: Future<Output=Result<String, Self::Error>> + Send;

    fn load(&self, user_id: u64) -> Self::Future;
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
        Self { client, pusher_client, handler, inited: false }
    }

    pub async fn init(&mut self) {
        self.inited = true;
        self.pusher_client.init().await;
        let user_info= self.client.get_user_info(None).await.unwrap().user;
        self.pusher_client
            .subscribe(format!("private-organization.{}", user_info.organizations[0].id))
            .await;
        self.pusher_client
            .subscribe(format!("private-user.{}", user_info.id))
            .await;
    }

    pub async fn run(&self) {
        loop {
            let message = self.pusher_client.server_messages().await.recv().await;
            match message {
                Ok(PusherServerMessageWrapper::PusherServerMessage(message)) => {
                    match message {
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
                    }
                }
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
        Self { client: None, handler: None }
    }

    pub fn client(mut self, client: Arc<ProntoClient>) -> Self {
        self.client = Some(client);
        self
    }

    pub fn handler(mut self, handler: impl Handler<Error=Box<dyn error::Error + Send + Sync>, Future=std::pin::Pin<dyn Future<Output=Result<(), Handler::Error>>>> + 'static) -> Self {
        self.handler = Some(HandlerWrapper::new(handler));
        self
    }

    pub async fn build(self) -> Bot {
        Bot::new(self.client.unwrap(), self.handler.unwrap()).await
    }
}
