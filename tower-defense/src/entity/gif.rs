#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Copy, Clone)]
pub struct Frame {
    x: f64,
    y: f64,
    w: f64,
    h: f64,
}

#[derive(Serialize, Deserialize, Copy, Clone)]
pub struct Size {
    w: f64,
    h: f64,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct GifFrame {
    frame: Frame,
    rotated: bool,
    trimmed: bool,
    spriteSourceSize: Frame,
    sourceSize: Size,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct GifFrames {
    frames: Vec<GifFrame>,
}
