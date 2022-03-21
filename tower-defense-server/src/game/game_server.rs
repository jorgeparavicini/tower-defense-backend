use std::error::Error;
use std::sync::{Arc};
use std::time::Instant;
use futures::{stream, StreamExt};
use futures::stream::SplitStream;
use log::{error, info};
use serde::Serialize;
use tokio::sync::{mpsc, Mutex};
use tokio::time::{self, Duration, Interval};
use warp::ws::{Message, WebSocket};
use tower_defense::game::Game;
use tower_defense::map::Map;
use crate::game::ServerMessage;

type ClientSender = mpsc::UnboundedSender<std::result::Result<Message, warp::Error>>;
type ClientReceiver = SplitStream<WebSocket>;

const TICK_RATE: u64 = 1;

#[derive(Debug, Clone)]
struct GameError;

pub struct GameServer<'a> {
    client: ClientSender,
    receiver: ClientReceiver,
    game: Game<'a>,
    interval: Interval,
    last_instant: Instant,
}

impl<'a> GameServer<'a> {
    pub fn new(map: &'a Map, client: ClientSender, receiver: ClientReceiver) -> Self {
        Self {
            client,
            receiver,
            game: Game::new(map),
            interval: time::interval(Duration::from_millis(1000 / TICK_RATE)),
            last_instant: Instant::now(),
        }
    }

    pub async fn start(&mut self) -> Result<(), Box<dyn Error>> {
        // Send map to client
        let map_message = ServerMessage::new(
            String::from("map"),
            self.game.get_map());
        self.send_message(map_message)?;

        let this = Arc::new(Mutex::new(self));
        let game_loop = Self::game_loop(this.clone());
        let listener = Self::client_listener(this.clone());
        //let game_loop_2 = self.game_tick();
        tokio::task::spawn(game_loop);
        Ok(())
    }

    async fn game_loop(this: Arc<Mutex<&mut Self>>) {
        this.lock().await.last_instant = Instant::now();
        stream::unfold(this, |state| async move {
            return match state.lock().await.tick().await {
                Ok(()) => Some(((), state.clone())),
                Err(_) => None
            }
        }).for_each(|_| async {}).await;
    }

    async fn tick(&mut self) -> Result<(), GameError>{
        self.interval.tick().await;

        let now = Instant::now();
        let delta_time = now - self.last_instant;
        self.game.update(delta_time.as_micros() as f64 / 1_000_000.0);

        let pos = self.game.get_coords();
        if let Err(_) = self.client.send(Ok(Message::text(format!("{{\"pos\": {{\"x\": {}, \"y\": {}}}}}", pos.x(), pos.y())))) {
            info!("Closing game");
            return Err(GameError);
        }

        Ok(())
    }


    fn send_message<'b, T>(&self, message: ServerMessage<'b, T>) -> Result<(), Box<dyn Error>>
        where T: Serialize
    {
        let json = serde_json::to_string(&message)?;
        self.client.send(Ok(Message::text(json)))?;

        Ok(())
    }

    async fn client_listener(this: Arc<Mutex<&mut Self>>) {
        info!("Started listening");
        while let Some(result) = this.lock().await.receiver.next().await {
            let msg = match result {
                Ok(msg) => msg,
                Err(e) => {
                    error!("Error receiving ws message: {}", e);
                    break;
                }
            };

            this.lock().await.on_message_received(msg);
        }
    }

    fn on_message_received(&mut self, msg: Message) {
        info!("Message Received: {}", msg.to_str().expect("Kek"));
    }

    //async fn client_listener()
}