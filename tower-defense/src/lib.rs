pub mod core;
pub mod entity;
mod game;
pub mod levels;
pub mod math;
pub mod path;

pub use game::Game;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate erased_serde;
