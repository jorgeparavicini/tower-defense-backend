use std::collections::HashMap;
use std::convert::Infallible;
use std::sync::Arc;
use log::trace;
use tokio::sync::{mpsc, RwLock};
use warp::{Filter, Rejection, ws::Message};

mod server;
mod handler;
mod game;

type Result<T> = std::result::Result<T, Rejection>;
type Clients = Arc<RwLock<HashMap<String, Client>>>;

#[derive(Debug, Clone)]
pub struct Client {
    pub user_id: usize,
    pub topics: Vec<String>,
    pub sender: Option<mpsc::UnboundedSender<std::result::Result<Message, warp::Error>>>,
}

#[tokio::main]
pub async fn main() {
    pretty_env_logger::init();

    trace!("Initializing routes");

    let clients: Clients = Arc::new(RwLock::new(HashMap::new()));

    let health_route = warp::path!("health").and_then(handler::health_handler);

    let register = warp::path("register");
    let register_routes = register
        .and(warp::post())
        .and(warp::body::json())
        .and(with_clients(clients.clone()))
        .and_then(handler::register_handler)
        .or(register
            .and(warp::delete())
            .and(warp::path::param())
            .and(with_clients(clients.clone()))
            .and_then(handler::unregister_handler));

    let publish = warp::path!("publish")
        .and(warp::body::json())
        .and(with_clients(clients.clone()))
        .and_then(handler::publish_handler);

    let game_ws = warp::path("game")
        .and(warp::ws())
        .and_then(handler::game_handler);

    let resources = warp::path("resources")
        .and(warp::fs::dir("resources/www"));

    let routes = health_route
        .or(register_routes)
        .or(publish)
        .or(game_ws)
        .or(resources)
        .with(warp::cors().allow_any_origin());

    warp::serve(routes).run(([127, 0, 0, 1], 6767)).await;
}

fn with_clients(clients: Clients) -> impl Filter<Extract=(Clients, ), Error=Infallible> + Clone {
    warp::any().map(move || clients.clone())
}