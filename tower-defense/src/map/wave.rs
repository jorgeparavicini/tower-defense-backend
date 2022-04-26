use crate::entity::EnemyType;
use std::collections::VecDeque;

#[derive(Copy, Clone)]
pub struct WaveElement {
    spawn_time: f64,
    enemy: EnemyType,
}

impl WaveElement {
    pub fn new(spawn_time: f64, enemy: EnemyType) -> Self {
        WaveElement { spawn_time, enemy }
    }
}

pub struct Wave {
    elements: VecDeque<WaveElement>,
}

impl Wave {
    pub fn new(elements: Vec<WaveElement>) -> Self {
        Wave {
            elements: VecDeque::from(elements),
        }
    }

    pub fn update(&mut self, time: f64) -> Option<EnemyType> {
        if let Some(element) = self.elements.get(0) {
            if element.spawn_time <= time {
                return Some(self.elements.pop_front().unwrap().enemy);
            }
        }

        None
    }
}
