use serde::{Serialize, Deserialize};
use tower_defense::core::{Map};
use tower_defense::Game;

#[derive(Deserialize)]
#[serde(tag = "message", content = "data")]
pub enum ReceiveMessage {
    Ping(u64),
    Command
}


#[derive(Serialize)]
#[serde(tag = "message", content = "data")]
pub enum SendMessage<'a> {
    Pong(u64),
    Map(&'a Map),
    Update(&'a Game)
}