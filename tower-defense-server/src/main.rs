use log::trace;
use tower_defense::entity::STRUCTURE_MAP;
use warp::Filter;

mod game;
mod handler;
mod server;

#[tokio::main]
pub async fn main() {
    pretty_env_logger::init();

    trace!("Initializing routes");

    let health_route = warp::path!("health").and_then(handler::health_handler);

    let game_ws = warp::path("game")
        .and(warp::ws())
        .and_then(handler::game_handler);

    let resources = warp::path("resources").and(warp::fs::dir("resources/www"));

    let structure_data = warp::path("structures").map(|| warp::reply::json(&*STRUCTURE_MAP));

    let routes = health_route
        .or(game_ws)
        .or(resources)
        .or(structure_data)
        .with(warp::cors().allow_any_origin());

    warp::serve(routes).run(([127, 0, 0, 1], 6767)).await;
}
