use std::error;
use std::future::Future;
use async_trait::async_trait;
use client::ProntoClient;
use pusher::{PusherClient, PusherServerEventType};

pub trait Handler {
    type Error;
    type Future: Future<Output=Result<(), Self::Error>> + Send;

    fn handle(&self, pronto_client: ProntoClient, pusher_client: PusherClient, input: PusherServerEventType) -> Self::Future;
}

pub struct FunctionHandler<F> {
    function: F,
}

impl<F, E> Handler for FunctionHandler<F>
where
    F: Fn(ProntoClient, PusherClient, PusherServerEventType) -> Result<(), E>,
    E: error::Error + Send + Sync + 'static,
{
    type Error = E;
    type Future = std::pin::Pin<Box<dyn Future<Output=Result<(), Self::Error>> + Send>>;

    fn handle(&self, pronto_client: ProntoClient, pusher_client: PusherClient, input: PusherServerEventType) -> Result<(), Self::Error> {
        (self.function)(pronto_client, pusher_client, input)
    }
}

pub fn handler<F, I, E>(function: F) -> FunctionHandler<F>
where
    F: Fn(I) -> Result<(), E>,
{
    FunctionHandler { function }
}

pub(crate) struct HandlerWrapper {
    handler: Box<dyn Handler<Error=Box<dyn error::Error + Send + Sync>, Future=std::pin::Pin<dyn Future<Output=Result<(), Handler::Error>>>>>,
}

#[async_trait]
impl HandlerWrapper {
    pub fn new(handler: impl Handler<Error=Box<dyn error::Error + Send + Sync>, Future=std::pin::Pin<dyn Future<Output=Result<(), Handler::Error>>>> + 'static) -> Self {
        Self { handler: Box::new(handler) }
    }

    pub async fn handle(&self, pronto_client: ProntoClient, pusher_client: PusherClient, input: PusherServerEventType) {
        let result = self.handler.handle(pronto_client, pusher_client, input).await;
        if let Err(err) = result {
            eprintln!("Error: {}", err);
        }
    }
}

pub trait CommandTrait {
    fn execute(&self, input: String) -> String;
}

pub struct Command {
    name: String,
    handler: Box<dyn CommandTrait>,
}

pub struct CommandHandler {
    commands: Vec<Command>,
}

impl CommandHandler {
    pub fn new() -> Self {
        Self { commands: Vec::new() }
    }

    pub fn add_command(mut self, command: Command) -> Self {
        self.commands.push(command);
        self
    }

    pub fn handle(&self, input: String) -> String {
        for command in &self.commands {
            if input.starts_with(&command.name) {
                return command.handler.execute(input);
            }
        }
        "Command not found".to_string()
    }
}

impl Handler for CommandHandler {
    type Error = Box<dyn error::Error + Send + Sync>;
    type Future = std::pin::Pin<Box<dyn Future<Output=Result<(), Self::Error>> + Send>>;

    async fn handle(&self, pronto_client: ProntoClient, _pusher_client: PusherClient, input: PusherServerEventType) -> Self::Future {
        Box::pin(async {
            let input = match input {
                PusherServerEventType::PusherServerMessageAddedEvent(message) => message,
                _ => return Ok(()),
            };
            let response = self.handle(input.message.message.clone());
            let user_info = pronto_client.get_user_info(None).await.unwrap().user;
            // TODO: parent message
            pronto_client.post_message(user_info.id, input.message.bubble_id, response, None).await;
            Ok(())
        })
    }
}
