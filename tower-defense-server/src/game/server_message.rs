use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::Formatter;
use tower_defense::entity::StructureType;
use tower_defense::math::Vector2;

#[derive(Deserialize, Debug)]
#[serde(tag = "message", content = "data")]
pub enum IncomingLobbyMessage {
    Start,
    Ping(u64),
}

impl fmt::Display for IncomingLobbyMessage {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Deserialize, Debug)]
#[serde(tag = "message", content = "data")]
pub enum LobbyMessage {
    Start(String),
    Ping(String, u64),
    GameMessage(IncomingGameMessage),
    Disconnect(String),
}

impl fmt::Display for LobbyMessage {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Serialize)]
#[serde(tag = "message", content = "data")]
pub enum OutgoingLobbyMessage {
    Players(Vec<String>),
    Lobby(String),
    Pong(u64),
    GameClosed,
    Update(OutgoingGameMessage),
}

#[derive(Deserialize, Debug)]
#[serde(tag = "message", content = "data")]
pub enum IncomingGameMessage {
    PlaceStructure {
        structure: StructureType,
        pos: Vector2,
    },
    UpgradeStructure {
        id: usize,
    },
}

impl fmt::Display for IncomingGameMessage {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Serialize)]
#[serde(tag = "message", content = "data")]
pub enum OutgoingGameMessage {
    Map(String),
    Update(String),
}
