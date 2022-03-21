use serde::Serialize;

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
}