use crate::game::client::Client;
use crate::game::{ReceiveMessage, SendMessage};
use futures::{stream, StreamExt};
use log::{debug, error, info};
use std::error::Error;
use std::time::Instant;
use tokio::time::{self, Duration, Interval};
use tower_defense::map::Map;
use tower_defense::Game;

const TICK_RATE: u64 = 30;

#[derive(Debug, Clone)]
struct GameError;

struct Players {
    host: Client,
    clients: Vec<Client>,
}

trait IntoMutIterator {
    type Item;
    type IntoIter;
}

impl Players {
    fn new(host: Client) -> Self {
        Self {
            host,
            clients: vec![],
        }
    }
}

impl<'a> Players {
    fn iter_mut(&'a mut self) -> PlayersMutIterator<'a> {
        PlayersMutIterator {
            players: self,
            index: 0,
        }
    }
}

impl<'a> IntoIterator for &'a Players {
    type Item = &'a Client;
    type IntoIter = PlayersIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        PlayersIterator {
            players: self,
            index: 0,
        }
    }
}

struct PlayersIterator<'a> {
    players: &'a Players,
    index: usize,
}

impl<'a> Iterator for PlayersIterator<'a> {
    type Item = &'a Client;

    fn next(&mut self) -> Option<Self::Item> {
        let result = match self.index {
            0 => Some(&self.players.host),
            _ => self.players.clients.get(self.index - 1),
        };
        self.index += 1;
        result
    }
}

struct PlayersMutIterator<'a> {
    players: &'a mut Players,
    index: usize,
}

impl<'a> Iterator for PlayersMutIterator<'a> {
    type Item = &'a mut Client;

    fn next(&mut self) -> Option<Self::Item> {
        let result = if self.index == 0 {
            let ptr = &mut self.players.host as *mut Client;
            unsafe { Some(&mut *ptr) }
        } else if self.index <= self.players.clients.len() {
            Some(unsafe {
                &mut *self
                    .players
                    .clients
                    .as_mut_ptr()
                    .offset((self.index - 1) as isize)
            })
        } else {
            None
        };

        self.index += 1;
        result
    }
}

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
        for client in &self.players {
            let map_message = SendMessage::Map(self.game.get_map());
            client.send_message(map_message)?;
        }

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
                }
            }
        }

        debug!("Sending message");
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
