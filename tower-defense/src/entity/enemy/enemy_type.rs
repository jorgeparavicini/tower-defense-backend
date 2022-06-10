use crate::entity::enemy::enemy::{Enemy, EnemyData};
use crate::entity::enemy::RECRUIT;
use rand::Rng;
use serde::Serialize;

#[derive(Serialize, Copy, Clone)]
pub enum EnemyType {
    Recruit,
}

impl EnemyType {
    pub fn new(self, spawn_time: f64) -> Enemy {
        Enemy::new(self, spawn_time)
    }

    pub fn get_enemy_data(&self) -> &'static Box<dyn EnemyData + Send + Sync> {
        match self {
            EnemyType::Recruit => &*RECRUIT,
        }
    }

    pub fn random() -> EnemyType {
        let rng = rand::thread_rng().gen_range(0..1);
        match rng {
            0 => EnemyType::Recruit,
            _ => panic!(),
        }
    }
}
