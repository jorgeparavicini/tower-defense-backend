use crate::game::GameServer;
use crate::game::{Client, ClientReceiver, ClientSender};
use crate::{GameLobby, GamesDb};
use futures::{FutureExt, StreamExt};
use log::{error, info};
use rand::distributions::Alphanumeric;
use rand::Rng;
use tokio::sync::mpsc;
use tokio_stream::wrappers::UnboundedReceiverStream;
use tower_defense::map::levels::MAP_LEVEL_1;
use warp::ws::WebSocket;

const KEY_LENGTH: usize = 8;

pub async fn game_connection(ws: WebSocket, games: GamesDb) {
    let (sender, receiver) = get_client_from_ws(ws);
    let host = Client::new_host(sender, receiver);

    let id = loop {
        let id = generate_lobby_key();

        if !games.lock().await.contains_key(&id) {
            break id;
        }
    };

    let lobby = GameLobby::new(id, host);
    games
        .lock()
        .await
        .insert(String::from(lobby.get_id()), lobby);
}

fn get_client_from_ws(ws: WebSocket) -> (ClientSender, ClientReceiver) {
    let (client_ws_sender, client_ws_rcv) = ws.split();
    let (client_sender, client_rcv) = mpsc::unbounded_channel();
    let client_rcv = UnboundedReceiverStream::new(client_rcv);

    tokio::task::spawn(client_rcv.forward(client_ws_sender).map(|result| {
        if let Err(e) = result {
            error!("error sending websocket msg: {}", e);
        }
    }));

    info!("Client connected");
    (client_sender, client_ws_rcv)
}

fn generate_lobby_key() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(KEY_LENGTH)
        .map(char::from)
        .collect()
}
