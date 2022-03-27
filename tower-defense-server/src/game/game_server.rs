use std::error::Error;
use std::time::{Instant};
use futures::{stream, StreamExt};
use log::{debug, error, info};
use tokio::time::{self, Duration, Interval};
use tower_defense::Game;
use tower_defense::core::Map;
use crate::game::client::Client;
use crate::game::{ReceiveMessage, SendMessage};


const TICK_RATE: u64 = 30;

#[derive(Debug, Clone)]
struct GameError;

pub struct GameServer {
    client: Client,
    game: Game,
    interval: Interval,
    last_instant: Instant,
}

impl GameServer {
    pub fn new(map: &'static Map, client: Client) -> Self {
        Self {
            client,
            game: Game::new(map),
            interval: time::interval(Duration::from_millis(1000 / TICK_RATE)),
            last_instant: Instant::now(),
        }
    }

    pub fn start(self) -> Result<(), Box<dyn Error>> {
        // Send map to client
        let map_message = SendMessage::Map(self.game.get_map());
        self.client.send_message(map_message)?;

        tokio::task::spawn(self.game_loop());
        Ok(())
    }

    async fn game_loop(mut self) {
        self.last_instant = Instant::now();
        stream::unfold(self, |mut state| async move {
            return match state.tick().await {
                Ok(()) => Some(((), state)),
                Err(_) => None
            };
        }).for_each(|_| async {}).await;
    }

    async fn tick(&mut self) -> Result<(), GameError> {
        self.interval.tick().await;

        let now = Instant::now();
        let delta_time = now - self.last_instant;
        self.game.update(delta_time.as_micros() as f64 / 1_000_000.0);

        for message in self.client.get_messages().await {
            match message {
                ReceiveMessage::Ping(ping) => {
                    if let Err(e) = self.client.send_message(SendMessage::Pong(ping)) {
                        error!("Could not send pong message: {}", e);
                    }
                },
                ReceiveMessage::Command => {}
            }
        }

        let pos = self.game.get_coords();
        debug!("Sending message");
        if let Err(_) = self.client.send_message(SendMessage::Update(pos)) {
            info!("Closing game");
            return Err(GameError);
        }

        Ok(())
    }
}