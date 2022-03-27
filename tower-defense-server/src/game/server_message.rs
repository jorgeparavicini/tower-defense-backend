use serde::{Serialize, Deserialize};
use tower_defense::core::Map;

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
}

/*
// TODO: Convert to Enum with available messages
#[derive(Serialize)]
pub struct ServerMessage<'a, T> where T: Serialize {
    message: String,
    data: &'a T,
}

impl<'a, T> ServerMessage<'a, T> where T: Serialize {
    pub fn new(message: String, data: &'a T) -> Self {
        Self {
            message,
            data,
        }
    }
}*/