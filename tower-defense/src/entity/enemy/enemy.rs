use crate::entity::enemy::enemy_type::EnemyType;
use crate::math::Vector2;
use serde::Serialize;
use std::sync::atomic::{AtomicUsize, Ordering};

pub trait EnemyData {
    fn get_max_health(&self) -> f64;
    fn get_damage(&self) -> u64;
    fn get_move_speed(&self) -> f64;
    fn get_coin_reward(&self) -> usize;
}

#[derive(Serialize)]
pub struct Enemy {
    id: usize,
    pos: Vector2,
    health: f64,
    enemy_type: EnemyType,
    #[serde(skip_serializing)]
    spawn_time: f64,
}

impl Enemy {
    pub(super) fn new(enemy_type: EnemyType, spawn_time: f64) -> Self {
        static ID_COUNTER: AtomicUsize = AtomicUsize::new(0);
        let id = ID_COUNTER.fetch_add(1, Ordering::Relaxed);
        Self {
            id,
            pos: Vector2::new(0.0, 0.0),
            health: enemy_type.get_enemy_data().get_max_health(),
            enemy_type,
            spawn_time,
        }
    }

    pub fn get_enemy_type(&self) -> &EnemyType {
        &self.enemy_type
    }

    pub fn get_spawn_time(&self) -> f64 {
        self.spawn_time
    }

    pub fn set_position(&mut self, new_pos: Vector2) {
        self.pos = new_pos;
    }

    pub fn get_health(&self) -> f64 {
        self.health
    }

    pub fn apply_damage(&mut self, damage: f64) {
        self.health -= damage;
    }

    pub fn get_position(&self) -> &Vector2 {
        &self.pos
    }
}
