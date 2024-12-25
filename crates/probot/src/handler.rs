use client::ProntoClient;
use pusher::PusherServerEventType;
use std::error;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

pub trait Handler {
    type Error;

    #[allow(async_fn_in_trait)]
    async fn handle(
        &self,
        pronto_client: Arc<ProntoClient>,
        input: PusherServerEventType,
    ) -> Result<(), Self::Error>;
}

pub struct FunctionHandler<F> {
    function: F,
}

impl<F, E> Handler for FunctionHandler<F>
where
    F: Fn(Arc<ProntoClient>, PusherServerEventType) -> Pin<Box<dyn Future<Output=Result<(), E>>>>,
    E: error::Error + Send + Sync + 'static,
{
    type Error = E;

    async fn handle(
        &self,
        pronto_client: Arc<ProntoClient>,
        input: PusherServerEventType,
    ) -> Result<(), Self::Error> {
        async {
            (self.function)(pronto_client, input).await
        }.await
    }
}

pub fn handler<F, I, E>(function: F) -> FunctionHandler<F>
where
    F: Fn(Arc<ProntoClient>, PusherServerEventType) -> Pin<Box<dyn Future<Output=Result<(), E>>>>,
{
    FunctionHandler { function }
}

pub trait CommandTrait {
    fn execute(&self, input: String) -> String;
}

pub struct Command {
    name: String,
    handler: Box<dyn CommandTrait>,
}

/// Handle responses to messages that start with arbritrary strings of text
pub struct CommandHandler {
    commands: Vec<Command>,
}

impl CommandHandler {
    pub fn new() -> Self {
        Self {
            commands: Vec::new(),
        }
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

    async fn handle(
        &self,
        pronto_client: Arc<ProntoClient>,
        input: PusherServerEventType,
    ) -> Result<(), Self::Error> {
        let input = match input {
            PusherServerEventType::PusherServerMessageAddedEvent(message) => message,
            _ => return Ok(()),
        };
        let response = self.handle(input.message.message.clone());
        let user_info = pronto_client.user_info(None).await.unwrap().user;
        // TODO: parent message
        pronto_client
            .send_message(user_info.id, input.message.bubble_id, response, None)
            .await?;
        Ok(())
    }
}

pub struct NoopHandler;

impl Handler for NoopHandler {
    type Error = Box<dyn error::Error + Send + Sync>;

    async fn handle(&self, _: Arc<ProntoClient>, _: PusherServerEventType) -> Result<(), Self::Error> {
        Ok(())
    }
}
