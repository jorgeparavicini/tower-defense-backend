use crate::game::game_server::GameServer;
use crate::game::players::Players;
use crate::game::server_message::LobbyMessage;
use crate::game::{Client, ClientReceiver, ClientSender};
use tokio::sync::mpsc;
use tokio::sync::mpsc::{Receiver, Sender};

pub struct GameLobby {
    server: Option<GameServer>,
    players: Players,
    messages: Vec<String>,
    id: String,
    rx: Receiver<LobbyMessage>,
    tx: Sender<LobbyMessage>,
}

impl GameLobby {
    pub fn new(id: String, host_sender: ClientSender, host_receiver: ClientReceiver) -> Self {
        // Channel for clients to communicate to lobby.
        let (tx, rx) = mpsc::channel(32);
        let host = Client::new_host(host_sender, host_receiver, tx.clone());
        let players = Players::new(host);
        Self {
            server: None,
            players,
            messages: vec![],
            id,
            rx,
            tx,
        }
    }

    pub fn get_id(&self) -> &str {
        &self.id
    }
}
