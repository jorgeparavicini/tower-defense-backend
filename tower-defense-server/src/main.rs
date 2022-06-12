extern crate core;

use crate::game::GameLobby;
use futures::TryFutureExt;
use handler::LobbyNotFoundError;
use log::trace;
use std::collections::HashMap;
use std::convert::Infallible;
use std::sync::Arc;
use tokio::sync::Mutex;
use tower_defense::entity::{ENEMY_MODEL_MAP, STRUCTURE_MODEL_MAP};
use warp::http::StatusCode;
use warp::{Filter, Rejection};

mod game;
mod handler;
mod server;

pub type GamesDb = Arc<Mutex<HashMap<String, GameLobby>>>;
pub type SavedGamesDb = Arc<Mutex<HashMap<String, String>>>;

#[tokio::main]
pub async fn main() {
    pretty_env_logger::init();

    let games = Arc::new(Mutex::new(HashMap::new()));
    let saved_games = Arc::new(Mutex::new(HashMap::new()));

    trace!("Initializing routes");

    let health_route = warp::path!("health").and_then(handler::health_handler);

    let game_ws = warp::path("game")
        .and(warp::path("create"))
        .and(warp::ws())
        .and(with_games_db(games.clone()))
        .and(with_saved_games_db(saved_games.clone()))
        .and_then(handler::create_game);

    let join_game = warp::path("game")
        .and(warp::path("join"))
        .and(warp::path::param())
        .and(warp::ws())
        .and(with_games_db(games.clone()))
        .and_then(handler::join_game)
        .recover(|err: Rejection| async move {
            if let Some(LobbyNotFoundError) = err.find() {
                Ok(StatusCode::NOT_FOUND)
            } else {
                Err(err)
            }
        });

    let resources = warp::path("resources").and(warp::fs::dir("resources/www"));

    let structure_data = warp::path("structures").map(|| warp::reply::json(&*STRUCTURE_MODEL_MAP));

    let enemy_data = warp::path("enemies").map(|| warp::reply::json(&*ENEMY_MODEL_MAP));

    let saved_games = warp::path("games")
        .and(with_saved_games_db(saved_games.clone()))
        .and_then(handler::get_saved_games);

    let routes = health_route
        .or(game_ws)
        .or(join_game)
        .or(resources)
        .or(structure_data)
        .or(enemy_data)
        .or(saved_games)
        .with(warp::cors().allow_any_origin());

    warp::serve(routes).run(([127, 0, 0, 1], 6767)).await;
}

fn with_games_db(
    games_db: GamesDb,
) -> impl Filter<Extract = (GamesDb,), Error = Infallible> + Clone {
    warp::any().map(move || games_db.clone())
}

fn with_saved_games_db(
    saved_games: SavedGamesDb,
) -> impl Filter<Extract = (SavedGamesDb,), Error = Infallible> + Clone {
    warp::any().map(move || saved_games.clone())
}
