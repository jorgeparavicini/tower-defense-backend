use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::Formatter;
use tower_defense::core::Map;
use tower_defense::entity::StructureType;
use tower_defense::math::Vector2;
use tower_defense::Game;

#[derive(Deserialize, Debug)]
#[serde(tag = "message", content = "data")]
pub enum ReceiveMessage {
    Ping(u64),
    PlaceStructure {
        structure: StructureType,
        pos: Vector2,
    },
}

impl fmt::Display for ReceiveMessage {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Serialize)]
#[serde(tag = "message", content = "data")]
pub enum SendMessage<'a> {
    Pong(u64),
    Map(&'a Map),
    Update(&'a Game),
}
