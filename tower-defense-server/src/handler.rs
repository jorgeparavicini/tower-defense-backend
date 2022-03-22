use crate::{server::game_connection};
use warp::{http::StatusCode, Reply, Rejection};


type Result<T> = std::result::Result<T, Rejection>;

pub async fn game_handler(ws: warp::ws::Ws) -> Result<impl Reply> {
    Ok(ws.on_upgrade(move |socket| game_connection(socket)))
}

pub async fn health_handler() -> Result<impl Reply> {
    Ok(StatusCode::OK)
}