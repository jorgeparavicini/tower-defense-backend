use crate::game::GameLobby;
use log::trace;
use std::collections::HashMap;
use std::convert::Infallible;
use std::sync::Arc;
use tokio::sync::Mutex;
use tower_defense::entity::STRUCTURE_MODEL_MAP;
use warp::Filter;

mod game;
mod handler;
mod server;

pub type GamesDb = Arc<Mutex<HashMap<String, GameLobby>>>;

#[tokio::main]
pub async fn main() {
    pretty_env_logger::init();

    let games = Arc::new(Mutex::new(HashMap::new()));

    trace!("Initializing routes");

    let health_route = warp::path!("health").and_then(handler::health_handler);

    let game_ws = warp::path("game")
        .and(warp::path("create"))
        .and(warp::ws())
        .and(with_games_db(games.clone()))
        .and_then(handler::create_game);

    let join_game = warp::path("game")
        .and(warp::path("join"))
        .and(warp::path::param())
        .and(warp::ws())
        .and_then(handler::join_game);

    let resources = warp::path("resources").and(warp::fs::dir("resources/www"));

    let structure_data = warp::path("structures").map(|| warp::reply::json(&*STRUCTURE_MODEL_MAP));

    let routes = health_route
        .or(game_ws)
        .or(resources)
        .or(structure_data)
        .with(warp::cors().allow_any_origin());

    warp::serve(routes).run(([127, 0, 0, 1], 6767)).await;
}

fn with_games_db(
    games_db: GamesDb,
) -> impl Filter<Extract = (ItemsDb,), Error = Infallible> + Clone {
    warp::any().map(move || games_db)
}
