use crate::game::game_server::GameServer;
use futures::{FutureExt, StreamExt};
use log::error;
use tokio::{sync::mpsc};
use tokio_stream::wrappers::UnboundedReceiverStream;
use warp::ws::{WebSocket};
use tower_defense::levels::MAP_LEVEL_1;
use crate::game::Client;


pub async fn game_connection(ws: WebSocket) {
    let (client_ws_sender, client_ws_rcv) = ws.split();
    let (client_sender, client_rcv) = mpsc::unbounded_channel();
    let client_rcv = UnboundedReceiverStream::new(client_rcv);

    tokio::task::spawn(client_rcv.forward(client_ws_sender).map(|result| {
        if let Err(e) = result {
            eprintln!("error sending websocket msg: {}", e);
        }
    }));

    println!("Client connected");
    let client = Client::new(client_sender, client_ws_rcv);
    let game_server = GameServer::new(&MAP_LEVEL_1, client);
    if let Err(e) = game_server.start() {
        error!("Game Server did could not be started: {}", e);
    }
}
