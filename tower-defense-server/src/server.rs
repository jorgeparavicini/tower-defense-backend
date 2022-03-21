use std::time::{Duration};
use crate::{Client, Clients};
use crate::game::game_server::GameServer;
use futures::{FutureExt, StreamExt, stream};
use futures::stream::SplitStream;
use log::{debug, info};
use serde::Deserialize;
use serde_json::from_str;
use tokio::{sync::mpsc, time};
use tokio::sync::mpsc::UnboundedSender;
use tokio::time::{Instant, Interval};
use tokio_stream::wrappers::UnboundedReceiverStream;
use warp::ws::{Message, WebSocket};
use tower_defense::levels::MAP_LEVEL_1;

#[derive(Deserialize, Debug)]
pub struct PingRequest {
    ping: u64,
}


pub async fn game_connection(ws: WebSocket) {
    let (client_ws_sender, mut client_ws_rcv) = ws.split();
    let (client_sender, client_rcv) = mpsc::unbounded_channel();
    let client_rcv = UnboundedReceiverStream::new(client_rcv);

    tokio::task::spawn(client_rcv.forward(client_ws_sender).map(|result| {
        if let Err(e) = result {
            eprintln!("error sending websocket msg: {}", e);
        }
    }));

    println!("Client connected");
    let mut game_server = GameServer::new(&MAP_LEVEL_1, client_sender, client_ws_rcv);
    game_server.start().await;
/*
    debug!("Generating map");
    let map = serde_json::to_string(&MAP_LEVEL_1.deref()).unwrap();
    debug!("Sending map: {}", map);

    client_sender.send(Ok(Message::text(map)));

    tokio::task::spawn(game_tick(client_sender.clone()));
    tokio::task::spawn(async move {
        client_listener(&mut client_ws_rcv, client_sender.clone()).await
    });*/
}

async fn game_tick(sender: UnboundedSender<Result<Message, warp::Error>>) {
    /*const TICK_DURATION: u64 = 1000 / TICK_RATE;
    let tick_interval = time::interval(Duration::from_millis(TICK_DURATION));

    struct StreamState<'a> {
        game: Game<'a>,
        last_instant: Instant,
        interval: Interval
    }

    let state = StreamState {
        game: Game::new(),
        last_instant: Instant::now(),
        interval: tick_interval
    };


    stream::unfold(state, |mut state| async {
        state.interval.tick().await;
        let now = Instant::now();

        let delta_time = now - state.last_instant;
        state.last_instant = now;
        state.game.update(delta_time.as_micros() as f64 / 1_000_000.0);

        let pos = state.game.get_coords();
        if let Err(_) = sender.send(Ok(Message::text(format!("{{\"pos\": {{\"x\": {}, \"y\": {}}}}}", pos.x(), pos.y())))) {
            info!("Closing game");
            return None;
        }
        Some(((), state))
    }).for_each(|_| async {}).await;*/
}

async fn client_listener(client_ws_rcv: &mut SplitStream<WebSocket>, sender: UnboundedSender<Result<Message, warp::Error>>) {
    while let Some(result) = client_ws_rcv.next().await {
        let msg = match result {
            Ok(msg) => msg,
            Err(e) => {
                eprintln!("error receiving ws message for id: {}): {}", "ño", e);
                break;
            }
        };
        client_msg_received(msg, sender.clone()).await;
    }
}

async fn client_msg_received(msg: Message, sender: UnboundedSender<Result<Message, warp::Error>>) {
    println!("received message from client: {:?}", msg);
    let message = match msg.to_str() {
        Ok(v) => v,
        Err(_) => return,
    };

    if let Ok(ping) = from_str(&message) {
        //time::sleep(time::Duration::from_millis(1000)).await;
        send_pong(sender.clone(), ping);
    }
}

fn send_pong(sender: UnboundedSender<Result<Message, warp::Error>>, ping: PingRequest) {
    if let Err(e) = sender.send(Ok(Message::text(format!("{{\"pong\": {}}}", ping.ping)))) {
        eprintln!("Error sending pong: {}", e);
    }
}