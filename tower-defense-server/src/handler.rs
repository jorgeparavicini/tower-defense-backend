use crate::server::game_connection;
use crate::GamesDb;
use warp::{http::StatusCode, Rejection, Reply};

type Result<T> = std::result::Result<T, Rejection>;

pub async fn create_game(ws: warp::ws::Ws, games: GamesDb) -> Result<impl Reply> {
    Ok(ws.on_upgrade(move |socket| game_connection(socket, games)))
}

pub async fn join_game(ws: warp::ws::Ws, game_id: i64) -> Result<impl Reply> {}

pub async fn health_handler() -> Result<impl Reply> {
    Ok(StatusCode::OK)
}
