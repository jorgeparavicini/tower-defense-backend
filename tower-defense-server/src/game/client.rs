use std::collections::VecDeque;
use std::error::Error;
use std::sync::Arc;
use futures::stream::SplitStream;
use futures::StreamExt;
use log::{debug, error, info};
use serde::Serialize;
use tokio::sync::{mpsc, RwLock};
use tokio::task::{JoinHandle, spawn};
use warp::ws::{Message, WebSocket};
use crate::game::ServerMessage;

pub type ClientSender = mpsc::UnboundedSender<std::result::Result<Message, warp::Error>>;
pub type Messages = Arc<RwLock<VecDeque<Message>>>;
pub type ClientReceiver = SplitStream<WebSocket>;

// TODO: Graceful shutdown?
pub struct Client {
    sender: ClientSender,
    messages: Messages,
    handle: JoinHandle<()>
}

impl Client {
    pub fn new(sender: ClientSender, receiver: ClientReceiver) -> Self {
        let messages = Arc::new(RwLock::new(VecDeque::new()));
        let handle = spawn(Client::client_listener(
            messages.clone(),
            receiver));
        Self {
            sender,
            messages,
            handle,
        }
    }

    pub async fn get_messages(&mut self) -> VecDeque<Message> {
        std::mem::take(&mut *self.messages.write().await)
    }

    pub fn send_message<'a, T>(&self, message: ServerMessage<'a, T>) -> Result<(), Box<dyn Error>>
        where T: Serialize
    {
        let json = serde_json::to_string(&message)?;
        self.sender.send(Ok(Message::text(json)))?;

        Ok(())
    }

    async fn client_listener(message_queue: Messages, mut receiver: ClientReceiver) {
        while let Some(result) = receiver.next().await {
            let msg = match result {
                Ok(msg) => msg,
                Err(e) => {
                    error!("Error receiving message: {}", e);
                    break;
                }
            };

            info!("Message received");
            message_queue.write().await.push_back(msg);
        }
    }
}

impl Drop for Client {
    fn drop(&mut self) {
        self.handle.abort();
        debug!("Terminating client listener");
    }
}