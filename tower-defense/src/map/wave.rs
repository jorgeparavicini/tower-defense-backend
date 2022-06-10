use crate::entity::EnemyType;
use log::info;
use rand::Rng;

pub struct Wave {
    min_respawn_duration: f64,
    max_respawn_duration: f64,
    next_respawn: f64,
}

impl Wave {
    pub fn new(min_respawn_duration: f64, max_respawn_duration: f64) -> Self {
        Wave {
            min_respawn_duration,
            max_respawn_duration,
            next_respawn: 0.0,
        }
    }

    pub fn update(&mut self, delta_time: f64) -> Option<EnemyType> {
        self.next_respawn -= delta_time;
        if self.next_respawn < 0.0 {
            self.next_respawn =
                rand::thread_rng().gen_range(self.min_respawn_duration..self.max_respawn_duration);
            return Some(EnemyType::random());
        }

        None
    }
}
