use crate::game::game_server::GameServer;
use crate::game::players::Players;
use crate::game::server_message::{LobbyMessage, OutgoingLobbyMessage};
use crate::game::{Client, IncomingGameMessage, OutgoingGameMessage};
use crate::{GamesDb, SavedGamesDb};
use futures::future::err;
use log::{debug, error, info, warn};
use rand::distributions::Alphanumeric;
use rand::Rng;
use serde::Serialize;
use std::error::Error;
use std::sync::Arc;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::sync::{mpsc, Mutex};
use tokio::task::JoinHandle;
use tower_defense::map::levels::MAP_LEVEL_1;
use tower_defense::GameLoad;
use warp::ws::WebSocket;

const KEY_LENGTH: usize = 8;

#[derive(Serialize)]
pub struct ChatMessage {
    client: String,
    message: String,
}

pub struct GameLobby {
    server: Option<Arc<Mutex<GameServer>>>,
    players: Players,
    messages: Vec<ChatMessage>,
    id: String,
    tx: Sender<LobbyMessage>,
    handle: JoinHandle<()>,
    game_handle: Option<JoinHandle<()>>,
    saved_games: SavedGamesDb,
}

impl GameLobby {
    pub fn new(id: String, ws: WebSocket, games: GamesDb, saved_games: SavedGamesDb) -> Self {
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
            saved_games,
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
                LobbyMessage::Load { client, lobby_id } => {
                    Self::load_game(&games, &id, client, lobby_id).await
                }
                LobbyMessage::Ping(name, n) => Self::handle_ping(&games, &id, name, n).await,
                LobbyMessage::Chat { client, message } => {
                    Self::handle_chat_message(&games, &id, client, message).await
                }
                LobbyMessage::GameMessage(data, client) => {
                    Self::handle_game_message(&games, &id, data, client).await
                }
                LobbyMessage::Disconnect(name) => Self::handle_disconnect(&games, &id, name).await,
                LobbyMessage::Save(_) => Self::handle_save(&games, &id).await,
            }
        }
    }

    async fn handle_game_events(games: GamesDb, id: String, mut rx: Receiver<OutgoingGameMessage>) {
        while let Some(result) = rx.recv().await {
            if let Some(game) = games.lock().await.get_mut(&id) {
                if let OutgoingGameMessage::CoinsReceived(coins) = result {
                    game.receive_coins(coins);
                } else {
                    game.broadcast_message(&OutgoingLobbyMessage::Update(result), None);
                }
            } else {
                error!("Could not find game from handle game events");
            }
        }
    }

    async fn start_game(games: &GamesDb, id: &str, name: String) {
        if let Some(lobby) = games.lock().await.get_mut(id) {
            if lobby.players.get_host().get_name() != name {
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
                lobby.game_handle = Some(handle);
                GameServer::start(game_server.clone());
                lobby.server = Some(game_server);
            }
        }
    }

    async fn load_game(games: &GamesDb, id: &str, name: String, lobby_id: String) {
        if let Some(lobby) = games.lock().await.get_mut(id) {
            if lobby.players.get_host().get_name() != name {
                info!("Only the host can start the game");
                return;
            } else {
                if let Some(saved_game) = lobby.saved_games.lock().await.get(&lobby_id) {
                    let (tx, rx) = mpsc::channel(32);
                    let game_server = GameServer::load(&MAP_LEVEL_1, tx, saved_game);
                    let game_server = Arc::new(Mutex::new(game_server));
                    let handle = tokio::spawn(GameLobby::handle_game_events(
                        games.clone(),
                        String::from(id),
                        rx,
                    ));
                    lobby.game_handle = Some(handle);
                    GameServer::start(game_server.clone());
                    lobby.server = Some(game_server);
                } else {
                    info!("Lobby not found");
                }
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

    async fn handle_chat_message(games: &GamesDb, id: &str, client_name: String, message: String) {
        if let Some(lobby) = games.lock().await.get_mut(id) {
            if let Some(client) = lobby.players.find_client(&client_name) {
                if client.get_name() == client_name {
                    lobby.messages.push(ChatMessage {
                        client: client_name.clone(),
                        message: message.clone(),
                    });
                    lobby.broadcast_message(
                        &OutgoingLobbyMessage::NewChatMessage(ChatMessage {
                            client: client_name,
                            message,
                        }),
                        None,
                    )
                }
            }
        }
    }

    async fn handle_game_message(
        games: &GamesDb,
        id: &str,
        data: IncomingGameMessage,
        client: String,
    ) {
        if let Some(lobby) = games.lock().await.get_mut(id) {
            if let Some(player) = lobby.players.find_client_mut(&client) {
                match &lobby.server {
                    Some(game) => game.lock().await.handle_game_message(data, player),
                    None => warn!(
                        "Game {} has not yet started and cannot receive game messages",
                        id
                    ),
                }
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

    async fn handle_save(games: &GamesDb, id: &str) {
        if let Some(lobby) = games.lock().await.get_mut(id) {
            if let Some(server) = &lobby.server {
                match serde_json::to_string(&*server.lock().await) {
                    Ok(game) => {
                        let id = loop {
                            let id = generate_lobby_key();

                            if !lobby.saved_games.lock().await.contains_key(&id) {
                                break id;
                            }
                        };
                        lobby.saved_games.lock().await.insert(id, game);
                    }
                    Err(error) => error!("{}", error),
                }
            }
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
                if let OutgoingLobbyMessage::Update(gm) = message {
                    player.send_message(&OutgoingLobbyMessage::ClientUpdate(
                        (*gm).clone(),
                        player.get_coins(),
                    ))?;
                } else {
                    player.send_message(message)?;
                }
            }
        }

        Ok(())
    }

    fn receive_coins(&mut self, amount: usize) {
        for player in self.players.iter_mut() {
            player.receive_coins(amount);
        }
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

fn generate_lobby_key() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(KEY_LENGTH)
        .map(char::from)
        .collect()
}
