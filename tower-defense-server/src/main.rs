use log::trace;
use warp::Filter;

mod server;
mod handler;
mod game;

#[tokio::main]
pub async fn main() {
    pretty_env_logger::init();

    trace!("Initializing routes");

    let health_route = warp::path!("health").and_then(handler::health_handler);


    let game_ws = warp::path("game")
        .and(warp::ws())
        .and_then(handler::game_handler);

    let resources = warp::path("resources")
        .and(warp::fs::dir("resources/www"));

    let routes = health_route
        .or(game_ws)
        .or(resources)
        .with(warp::cors().allow_any_origin());

    warp::serve(routes).run(([127, 0, 0, 1], 6767)).await;
}