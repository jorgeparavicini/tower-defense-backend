use crate::server::{game_connection, game_connection_join};
use crate::GamesDb;
use warp::reject::Reject;
use warp::{http::StatusCode, Rejection, Reply};

pub async fn create_game(ws: warp::ws::Ws, games: GamesDb) -> Result<impl Reply, Rejection> {
    Ok(ws.on_upgrade(move |socket| game_connection(socket, games)))
}

#[derive(Debug)]
pub struct LobbyNotFoundError;
impl Reject for LobbyNotFoundError {}

pub async fn join_game(
    game_id: String,
    ws: warp::ws::Ws,
    games: GamesDb,
) -> Result<impl Reply, Rejection> {
    if games.lock().await.contains_key(&game_id) {
        return Ok(ws.on_upgrade(move |socket| game_connection_join(socket, games, game_id)));
    }
    Err(warp::reject::custom(LobbyNotFoundError))
}

pub async fn health_handler() -> Result<impl Reply, Rejection> {
    Ok(StatusCode::OK)
}
