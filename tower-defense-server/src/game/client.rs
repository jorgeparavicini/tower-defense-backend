use crate::game::server_message::LobbyMessage;
use crate::game::{ReceiveMessage, SendMessage};
use futures::stream::SplitStream;
use futures::StreamExt;
use log::{debug, error, trace};
use std::collections::VecDeque;
use std::error::Error;
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tokio::sync::mpsc::Sender;
use tokio::sync::{mpsc, RwLock};
use tokio::task::{spawn, JoinHandle};
use warp::ws::{Message, WebSocket};

pub type ClientSender = mpsc::UnboundedSender<Result<Message, warp::Error>>;
pub type Messages = Arc<RwLock<VecDeque<ReceiveMessage>>>;
pub type ClientReceiver = SplitStream<WebSocket>;

// TODO: Graceful shutdown?
pub struct Client {
    sender: ClientSender,
    tx: Sender<LobbyMessage>,
    handle: JoinHandle<()>,
    is_host: bool,
}

impl Client {
    fn new(
        sender: ClientSender,
        receiver: ClientReceiver,
        is_host: bool,
        tx: Sender<LobbyMessage>,
    ) -> Self {
        let messages = Arc::new(RwLock::new(VecDeque::new()));
        let handle = spawn(Client::client_listener(messages.clone(), receiver));
        Self {
            sender,
            tx,
            handle,
            is_host,
        }
    }

    pub fn new_host(
        sender: ClientSender,
        receiver: ClientReceiver,
        tx: Sender<LobbyMessage>,
    ) -> Self {
        Self::new(sender, receiver, true, tx)
    }

    pub fn new_client(
        sender: ClientSender,
        receiver: ClientReceiver,
        tx: Sender<LobbyMessage>,
    ) -> Self {
        Self::new(sender, receiver, false, tx)
    }

    pub async fn get_messages(&mut self) -> VecDeque<ReceiveMessage> {
        std::mem::take(&mut *self.messages.write().await)
    }

    pub fn send_message(&self, message: &SendMessage) -> Result<(), Box<dyn Error>> {
        trace!("Sending message");
        let json = serde_json::to_string(message)?;
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

            if msg.is_close() {
                let message = ReceiveMessage::Close;
                debug!("Sending close signal");
                message_queue.write().await.push_back(message);
                debug!("Terminating client listener");
                break;
            }

            if msg.is_text() {
                if let Ok(mut result) = serde_json::from_str(msg.to_str().unwrap()) {
                    if let ReceiveMessage::Ping(ping) = result {
                        let ping = Duration::from_millis(ping);
                        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
                        let pong = if now > ping {
                            now - ping
                        } else {
                            Duration::from_millis(0)
                        }
                        .as_millis() as u64;
                        result = ReceiveMessage::Ping(pong);
                    }

                    debug!("Received message {}", result.to_string());
                    message_queue.write().await.push_back(result);
                } else {
                    error!("Could not read message received: {}", msg.to_str().unwrap());
                }
            } else {
                error!("Received unrecognized message.");
            }
        }
    }

    pub fn is_host(&self) -> bool {
        self.is_host
    }
}

impl Drop for Client {
    fn drop(&mut self) {
        self.handle.abort();
        debug!("Aborting client listener");
    }
}
