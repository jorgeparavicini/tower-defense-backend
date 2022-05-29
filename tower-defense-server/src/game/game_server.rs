use crate::game::{IncomingGameMessage, OutgoingGameMessage};
use futures::{stream, StreamExt};
use log::{debug, error, trace};
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::mpsc::Sender;
use tokio::sync::Mutex;
use tokio::time::{self, Duration, Interval};
use tower_defense::map::Map;
use tower_defense::Game;

const TICK_RATE: u64 = 30;

#[derive(Debug, Clone)]
struct GameError;

#[derive(Debug, Clone)]
struct GameClosed;

pub struct GameServer {
    game: Game,
    interval: Interval,
    last_instant: Instant,
    closed: bool,
    tx: Sender<OutgoingGameMessage>,
}

impl GameServer {
    pub fn new(map: &'static Map, tx: Sender<OutgoingGameMessage>) -> Self {
        Self {
            game: Game::new(map),
            interval: time::interval(Duration::from_millis(1000 / TICK_RATE)),
            last_instant: Instant::now(),
            closed: false,
            tx,
        }
    }

    pub fn start(this: Arc<Mutex<GameServer>>) {
        tokio::task::spawn(Self::game_loop(this));
    }

    async fn game_loop(this: Arc<Mutex<GameServer>>) {
        {
            let mut game = this.lock().await;
            match serde_json::to_string(game.game.get_map()) {
                Ok(json) => game.broadcast_message(OutgoingGameMessage::Map(json)).await,
                Err(e) => {
                    error!("Could not convert map to json: {}", e);
                }
            };
            game.last_instant = Instant::now();
            game.game.start();
        }
        stream::unfold(this, |state| async {
            let result = state.lock().await.tick().await;
            return match result {
                Ok(()) => Some(((), state)),
                Err(_) => None,
            };
        })
        .for_each(|_| async {})
        .await;
        debug!("Game loop terminating");
    }

    async fn tick(&mut self) -> Result<(), GameError> {
        if self.closed {
            return Err(GameError);
        }

        self.interval.tick().await;

        let now = Instant::now();
        let delta_time = now - self.last_instant;
        self.game.update(delta_time.as_micros() as f64 / 1_000.0);

        /*for client in &mut self.players.iter_mut() {
            for message in client.get_messages().await {
                //match message {
                /*IncomingGameMessage::Ping(ping) => {
                    /*if let Err(e) = client.send_message(&SendMessage::Pong(ping)) {
                        error!("Could not send pong message: {}", e);
                    }*/
                }
                IncomingGameMessage::PlaceStructure { structure, pos } => {
                    if let Err(_) = self.game.try_place_structure(structure, pos) {
                        // TODO: Send "could not place structure" message to client.
                    }
                }*/
                //}
            }
        }*/

        trace!("Sending message");
        match serde_json::to_string(&self.game) {
            Ok(json) => {
                self.broadcast_message(OutgoingGameMessage::Update(json))
                    .await
            }
            Err(e) => {
                error!("Could not convert game to json: {}", e);
            }
        };
        self.last_instant = now;

        Ok(())
    }

    pub fn handle_game_message(&mut self, message: IncomingGameMessage) {
        match message {
            IncomingGameMessage::PlaceStructure { structure, pos } => {
                if let Err(_) = self.game.try_place_structure(structure, pos) {
                    error!("Could not place structure");
                }
            }
        }
    }

    pub fn close_game(&mut self) {
        self.closed = true;
    }

    async fn broadcast_message(&self, message: OutgoingGameMessage) {
        if let Err(e) = self.tx.send(message).await {
            error!("Could not transmit message: {}", e);
        }
    }
}
