use crate::game::game_server::GameServer;
use crate::game::players::Players;
use crate::game::server_message::{LobbyMessage, OutgoingLobbyMessage};
use crate::game::{Client, IncomingGameMessage, OutgoingGameMessage};
use crate::GamesDb;
use futures::future::err;
use log::{debug, error, info, warn};
use std::error::Error;
use std::sync::Arc;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::sync::{mpsc, Mutex};
use tokio::task::JoinHandle;
use tower_defense::map::levels::MAP_LEVEL_1;
use warp::ws::WebSocket;

pub struct GameLobby {
    server: Option<Arc<Mutex<GameServer>>>,
    players: Players,
    messages: Vec<String>,
    id: String,
    tx: Sender<LobbyMessage>,
    handle: JoinHandle<()>,
    game_handle: Option<JoinHandle<()>>,
}

impl GameLobby {
    pub fn new(id: String, ws: WebSocket, games: GamesDb) -> Self {
        // Channel for clients to communicate to lobby.
        let (tx, rx) = mpsc::channel(32);
        let host = Client::new_host(ws, tx.clone());
        let players = Players::new(host);

        debug!("Creating lobby {}", &id);
        let handle = tokio::task::spawn(GameLobby::start(games, id.clone(), rx));

        Self {
            server: None,
            players,
            messages: vec![],
            id,
            tx,
            handle,
            game_handle: None,
        }
    }

    pub fn get_id(&self) -> &str {
        &self.id
    }

    pub fn join(&mut self, ws: WebSocket) {
        let client = Client::new_client(ws, self.tx.clone());
        self.players.add_client(client);
        self.broadcast_players();
    }

    async fn start(games: GamesDb, id: String, mut rx: Receiver<LobbyMessage>) {
        if let Some(game) = games.lock().await.get(&id) {
            game.broadcast_players();
            game.broadcast_message(&OutgoingLobbyMessage::Lobby(String::from(&game.id)), None)
        }

        debug!("Listening for messages");
        while let Some(result) = rx.recv().await {
            debug!("Received message: {}", &result);
            match result {
                LobbyMessage::Start(name) => Self::start_game(&games, &id, name).await,
                LobbyMessage::Ping(name, n) => Self::handle_ping(&games, &id, name, n).await,
                LobbyMessage::GameMessage(data) => {
                    Self::handle_game_message(&games, &id, data).await
                }
                LobbyMessage::Disconnect(name) => Self::handle_disconnect(&games, &id, name).await,
            }
        }
    }

    async fn handle_game_events(games: GamesDb, id: String, mut rx: Receiver<OutgoingGameMessage>) {
        while let Some(result) = rx.recv().await {
            if let Some(game) = games.lock().await.get_mut(&id) {
                game.broadcast_message(&OutgoingLobbyMessage::Update(result), None);
            } else {
                error!("Could not find game from handle game events");
            }
        }
    }

    async fn start_game(games: &GamesDb, id: &str, name: String) {
        if let Some(game) = games.lock().await.get_mut(id) {
            if game.players.get_host().get_name() != name {
                info!("Only the host can start the game");
                return;
            } else {
                let (tx, rx) = mpsc::channel(32);
                let game_server = GameServer::new(&MAP_LEVEL_1, tx);
                let game_server = Arc::new(Mutex::new(game_server));
                let handle = tokio::spawn(GameLobby::handle_game_events(
                    games.clone(),
                    String::from(id),
                    rx,
                ));
                game.game_handle = Some(handle);
                GameServer::start(game_server.clone());
                game.server = Some(game_server);
            }
        }
    }

    async fn handle_ping(games: &GamesDb, id: &str, name: String, ping: u64) {
        if let Some(lobby) = games.lock().await.get_mut(id) {
            if let Some(client) = lobby.players.find_client(&name) {
                if let Err(e) = client.send_message(&OutgoingLobbyMessage::Pong(ping)) {
                    error!("Could no answer ping: {}", e);
                }
            }
        }
    }

    async fn handle_game_message(games: &GamesDb, id: &str, data: IncomingGameMessage) {
        if let Some(lobby) = games.lock().await.get_mut(id) {
            match &lobby.server {
                Some(game) => game.lock().await.handle_game_message(data),
                None => warn!(
                    "Game {} has not yet started and cannot receive game messages",
                    id
                ),
            }
        } else {
            error!("Lobby {} not found", id);
        }
    }

    async fn handle_disconnect(games: &GamesDb, id: &str, name: String) {
        let remove = match games.lock().await.get_mut(id) {
            Some(game) => {
                if name == game.players.get_host().get_name() {
                    // Close game and remove from lobby from games list
                    match &mut game.server {
                        Some(server) => server.lock().await.close_game(),
                        None => (),
                    }
                    game.broadcast_message(
                        &OutgoingLobbyMessage::GameClosed,
                        Some(game.players.get_host().get_name()),
                    );
                    true
                } else {
                    game.players.remove_client(&name);
                    game.broadcast_players();
                    false
                }
            }
            None => {
                error!(
                    "Could not disconnect player {}. Game {} not found",
                    &name, id
                );
                false
            }
        };

        if remove {
            games.lock().await.remove(id);
            info!("Game {} closed", id);
        }
    }

    fn broadcast_players(&self) {
        info!("Broadcasting players");
        let players = (&self.players)
            .into_iter()
            .map(|x| String::from(x.get_name()))
            .collect();
        let message = OutgoingLobbyMessage::Players(players);
        self.broadcast_message(&message, None);
    }

    fn broadcast_message(&self, message: &OutgoingLobbyMessage, predicate: Option<&str>) {
        if let Err(e) = self.broadcast_message_err(message, predicate) {
            error!("{}", e);
        }
    }

    fn broadcast_message_err(
        &self,
        message: &OutgoingLobbyMessage,
        predicate: Option<&str>,
    ) -> Result<(), Box<dyn Error>> {
        for player in &self.players {
            if let Some(name) = predicate {
                if player.get_name() != name {
                    player.send_message(message)?;
                }
            } else {
                player.send_message(message)?;
            }
        }

        Ok(())
    }
}

impl<'a> Drop for GameLobby {
    fn drop(&mut self) {
        debug!("Waiting for lobby {} to finish", self.id);
        self.handle.abort();
        match &self.game_handle {
            Some(handle) => handle.abort(),
            None => (),
        }
        debug!("Aborting lobby listener");
    }
}
