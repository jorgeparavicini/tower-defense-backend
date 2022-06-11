use crate::game::server_message::{IncomingLobbyMessage, LobbyMessage, OutgoingLobbyMessage};
use crate::game::IncomingGameMessage;
use futures::stream::SplitStream;
use futures::{FutureExt, StreamExt};
use log::{debug, error, info, trace};
use names::Generator;
use std::collections::VecDeque;
use std::error::Error;
use tokio::sync::mpsc;
use tokio::sync::mpsc::Sender;
use tokio::task::{spawn, JoinHandle};
use tokio_stream::wrappers::UnboundedReceiverStream;
use warp::ws::{Message, WebSocket};

pub type ClientSender = mpsc::UnboundedSender<Result<Message, warp::Error>>;
pub type ClientReceiver = SplitStream<WebSocket>;

// TODO: Graceful shutdown?
pub struct Client {
    sender: ClientSender,
    handle: JoinHandle<()>,
    is_host: bool,
    name: String,
    coins: usize,
}

impl Client {
    fn new(ws: WebSocket, is_host: bool, tx: Sender<LobbyMessage>) -> Self {
        let name = Generator::default().next().unwrap();
        let (sender, receiver) = Self::start_ws_forwarder(ws, name.clone());
        let handle = spawn(Client::client_listener(tx, receiver, name.clone()));

        info!("Client connected");
        Self {
            sender,
            handle,
            is_host,
            name,
            coins: 500,
        }
    }

    fn start_ws_forwarder(ws: WebSocket, name: String) -> (ClientSender, ClientReceiver) {
        let (client_ws_sender, receiver) = ws.split();
        let (sender, client_rcv) = mpsc::unbounded_channel();
        let client_rcv = UnboundedReceiverStream::new(client_rcv);

        tokio::task::spawn(client_rcv.forward(client_ws_sender).map(move |result| {
            if let Err(e) = result {
                error!("Error sending message: {} to client {}", e, &name);
            }
        }));

        (sender, receiver)
    }

    pub fn new_host(ws: WebSocket, tx: Sender<LobbyMessage>) -> Self {
        Self::new(ws, true, tx)
    }

    pub fn new_client(ws: WebSocket, tx: Sender<LobbyMessage>) -> Self {
        Self::new(ws, false, tx)
    }

    pub async fn get_messages(&mut self) -> VecDeque<IncomingGameMessage> {
        //std::mem::take(&mut *self.messages.write().await)
        return VecDeque::new();
    }

    pub fn send_message(&self, message: &OutgoingLobbyMessage) -> Result<(), Box<dyn Error>> {
        trace!("Sending message");
        let json = serde_json::to_string(message)?;
        self.sender.send(Ok(Message::text(json)))?;

        Ok(())
    }

    async fn client_listener(
        tx: Sender<LobbyMessage>,
        mut receiver: ClientReceiver,
        client: String,
    ) {
        while let Some(result) = receiver.next().await {
            let msg = match result {
                Ok(msg) => msg,
                Err(e) => {
                    error!("Error receiving message: {}", e);
                    break;
                }
            };

            if msg.is_close() {
                let message = LobbyMessage::Disconnect(client.clone());
                info!("Client {} disconnected", &client);
                Self::send(&tx, message, &client).await;
                break;
            }

            if msg.is_text() {
                if let Ok(result) = serde_json::from_str(msg.to_str().unwrap()) {
                    let message = LobbyMessage::GameMessage(result, client.clone());
                    Self::send(&tx, message, &client).await;
                } else if let Ok(result) = serde_json::from_str(msg.to_str().unwrap()) {
                    let message = match result {
                        IncomingLobbyMessage::Start => LobbyMessage::Start(client.clone()),
                        IncomingLobbyMessage::Ping(n) => LobbyMessage::Ping(client.clone(), n),
                    };
                    Self::send(&tx, message, &client).await;
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

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_coins(&self) -> usize {
        self.coins
    }

    pub fn receive_coins(&mut self, amount: usize) {
        self.coins += amount;
    }

    pub fn remove_coins(&mut self, amount: usize) {
        self.coins -= amount;
    }

    async fn send(tx: &Sender<LobbyMessage>, message: LobbyMessage, client: &str) {
        if let Err(e) = tx.send(message).await {
            error!(
                "Client message send failure. Client: {}, error: {}",
                client, e
            );
        }
    }
}

impl Drop for Client {
    fn drop(&mut self) {
        self.handle.abort();
        debug!("Aborting client listener");
    }
}
