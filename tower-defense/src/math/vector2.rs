use serde::{Deserialize, Serialize};
use std::ops;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Vector2 {
    x: f64,
    y: f64,
}

impl Vector2 {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub fn dot(&self, other: &Vector2) -> f64 {
        self.x * other.x + self.y * other.y
    }

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }
}

impl<'a> ops::Add for &'a Vector2 {
    type Output = Vector2;

    fn add(self, rhs: &'a Vector2) -> Self::Output {
        Vector2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<'a> ops::Sub for &'a Vector2 {
    type Output = Vector2;

    fn sub(self, rhs: &'a Vector2) -> Self::Output {
        Vector2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}
