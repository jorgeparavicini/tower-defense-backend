pub mod entity;
mod game;
pub mod map;
pub mod math;

pub use game::{Game, GameLoad};

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate erased_serde;
