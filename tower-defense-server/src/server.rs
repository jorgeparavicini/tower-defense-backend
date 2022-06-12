use crate::{GameLobby, GamesDb, SavedGamesDb};
use rand::distributions::Alphanumeric;
use rand::Rng;
use warp::ws::WebSocket;

const KEY_LENGTH: usize = 8;

pub async fn game_connection(ws: WebSocket, games: GamesDb, saved_games: SavedGamesDb) {
    let id = loop {
        let id = generate_lobby_key();

        if !games.lock().await.contains_key(&id) {
            break id;
        }
    };

    let lobby = GameLobby::new(id, ws, games.clone(), saved_games.clone());
    games
        .lock()
        .await
        .insert(String::from(lobby.get_id()), lobby);
}

pub async fn game_connection_join(ws: WebSocket, games: GamesDb, game_id: String) {
    if let Some(game) = games.lock().await.get_mut(&game_id) {
        game.join(ws);
    }
}

fn generate_lobby_key() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(KEY_LENGTH)
        .map(char::from)
        .collect()
}
