use crate::game::{Client, IncomingGameMessage, OutgoingGameMessage};
use futures::{stream, StreamExt};
use log::{debug, error, trace};
use serde::Serialize;
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::mpsc::Sender;
use tokio::sync::Mutex;
use tokio::time::{self, Duration, Interval};
use tower_defense::entity::{
    GameStructure, KonfettiKanoneV1, KonfettiKanoneV2, LightningTower, LightningTowerV1,
};
use tower_defense::map::Map;
use tower_defense::{Game, GameLoad};

const TICK_RATE: u64 = 30;

#[derive(Debug, Clone)]
struct GameError;

#[derive(Debug, Clone)]
struct GameClosed;

#[derive(Serialize)]
pub struct GameServer {
    #[serde(flatten)]
    game: Game,

    #[serde(skip_serializing)]
    interval: Interval,

    #[serde(skip_serializing)]
    last_instant: Instant,

    #[serde(skip_serializing)]
    closed: bool,

    #[serde(skip_serializing)]
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

    pub fn load(map: &'static Map, tx: Sender<OutgoingGameMessage>, data: &str) -> Self {
        let game: serde_json::Value = serde_json::from_str(data).unwrap();
        let game_load: GameLoad = serde_json::from_value(game.clone()).unwrap();
        let mut structures: Vec<Box<dyn GameStructure>> = vec![];
        for structure in game["structures"].as_array().unwrap() {
            let model = structure["model"].as_str().unwrap();
            match model {
                "LightningTower" => structures.push(Box::new(LightningTower::load(structure))),
                "LightningTowerV1" => structures.push(Box::new(LightningTowerV1::load(structure))),
                "KonfettiKanoneV1" => structures.push(Box::new(KonfettiKanoneV1::load(structure))),
                "KonfettiKanoneV2" => structures.push(Box::new(KonfettiKanoneV2::load(structure))),
                _ => panic!("Unknown model: {}", model),
            };
        }

        Self {
            game: Game::load(map, game_load, structures),
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
        let gold_earned = self.game.update(delta_time.as_micros() as f64 / 1_000.0);

        self.broadcast_message(OutgoingGameMessage::CoinsReceived(gold_earned))
            .await;

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

    pub fn handle_game_message(&mut self, message: IncomingGameMessage, client: &mut Client) {
        match message {
            IncomingGameMessage::PlaceStructure { structure, pos } => {
                let cost = structure.get_model().get_cost();
                let coin = client.get_coins();
                if cost > coin {
                    return;
                }
                if let Ok(_) = self.game.try_place_structure(structure, pos) {
                    client.remove_coins(cost);
                }
            }
            IncomingGameMessage::UpgradeStructure { id } => {
                if let Some(structure) = self.game.find_structure(id) {
                    if let Some(upgrade) = structure.get_upgrade() {
                        let cost = upgrade.get_model().get_cost();
                        let coins = client.get_coins();
                        if cost > coins {
                            return;
                        }
                        if let Ok(_) = self.game.upgrade_structure(id) {
                            client.remove_coins(cost);
                        }
                    }
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
