use crate::game::client::Client;
use crate::game::{ReceiveMessage, SendMessage};
use futures::{stream, StreamExt};
use log::{debug, error, info, trace};
use std::error::Error;
use std::time::Instant;
use tokio::time::{self, Duration, Interval};
use tower_defense::map::Map;
use tower_defense::Game;

const TICK_RATE: u64 = 30;

#[derive(Debug, Clone)]
struct GameError;

pub struct GameServer {
    players: Players,
    game: Game,
    interval: Interval,
    last_instant: Instant,
}

impl GameServer {
    pub fn new(map: &'static Map, host: Client) -> Self {
        Self {
            players: Players::new(host),
            game: Game::new(map),
            interval: time::interval(Duration::from_millis(1000 / TICK_RATE)),
            last_instant: Instant::now(),
        }
    }

    pub fn start(self) -> Result<(), Box<dyn Error>> {
        // Send map to clients
        let map_message = SendMessage::Map(self.game.get_map());
        self.broadcast_message(&map_message)?;

        tokio::task::spawn(self.game_loop());
        Ok(())
    }

    async fn game_loop(mut self) {
        self.last_instant = Instant::now();
        self.game.start();
        stream::unfold(self, |mut state| async move {
            return match state.tick().await {
                Ok(()) => Some(((), state)),
                Err(_) => None,
            };
        })
        .for_each(|_| async {})
        .await;
        // TODO: Identify with id
        info!("Game loop terminating");
    }

    async fn tick(&mut self) -> Result<(), GameError> {
        self.interval.tick().await;

        let now = Instant::now();
        let delta_time = now - self.last_instant;
        self.game.update(delta_time.as_micros() as f64 / 1_000.0);

        for client in &mut self.players.iter_mut() {
            for message in client.get_messages().await {
                match message {
                    ReceiveMessage::Ping(ping) => {
                        if let Err(e) = client.send_message(&SendMessage::Pong(ping)) {
                            error!("Could not send pong message: {}", e);
                        }
                    }
                    ReceiveMessage::PlaceStructure { structure, pos } => {
                        if let Err(_) = self.game.try_place_structure(structure, pos) {
                            // TODO: Send "could not place structure" message to client.
                        }
                    }
                    ReceiveMessage::Close => {
                        if client.is_host() {
                            // TODO: Inform clients
                            info!("Host disconnected, closing game");
                            let game_closed_message = SendMessage::GameClosed;
                            for x in &mut self.players.clients {
                                if let Err(e) = x.send_message(&game_closed_message) {
                                    error!("Could not send disconnect message to client");
                                }
                            }
                            return Err(GameError);
                        } else {
                            info!("Client disconnected");
                            // TODO: Inform game and other clients
                        }
                    }
                }
            }
        }

        trace!("Sending message");
        if let Err(_) = self.broadcast_message(&SendMessage::Update(&self.game)) {
            error!("Failed sending broadcast message. Closing game");
            return Err(GameError);
        }

        self.last_instant = now;

        Ok(())
    }

    fn broadcast_message(&self, message: &SendMessage) -> Result<(), Box<dyn Error>> {
        for client in &self.players {
            client.send_message(message)?;
        }

        Ok(())
    }
}
