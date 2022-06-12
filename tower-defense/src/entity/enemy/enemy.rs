use crate::entity::enemy::enemy_type::EnemyType;
use crate::entity::gif::GifFrames;
use crate::map::Map;
use crate::math::Vector2;
use log::error;
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicUsize, Ordering};

#[derive(Serialize, PartialEq, Deserialize)]
#[serde(tag = "type", content = "data")]
enum State {
    Idle,
    Dying { time_of_death: f64 },
    Dead,
}

impl State {
    fn update(self, map: &'static Map, time: f64, enemy: &mut Enemy) -> Self {
        match self {
            Self::Idle => self.idle_update(map, time, enemy),
            Self::Dying { time_of_death } => self.dying_update(time, time_of_death, enemy),
            Self::Dead => {
                error!("Cannot update dead enemy");
                self
            }
        }
    }

    fn idle_update(self, map: &'static Map, time: f64, enemy: &mut Enemy) -> Self {
        if enemy.health <= 0.0 {
            return State::Dying {
                time_of_death: time,
            };
        }
        let move_speed = enemy.get_enemy_type().get_model().get_move_speed();
        let t = (time - enemy.get_spawn_time()) / 1000.0;
        enemy.set_position(map.get_path().coords_at(t * move_speed));

        Self::Idle
    }

    fn dying_update(self, time: f64, time_of_death: f64, enemy: &Enemy) -> Self {
        if (enemy.get_enemy_type().get_model().death_duration + time_of_death) < time {
            return Self::Dead;
        }

        self
    }
}

#[derive(Serialize)]
pub struct EnemyModel {
    max_health: f64,
    damage: u64,
    move_speed: f64,
    coin_reward: usize,
    death_duration: f64,
    idle_frames: GifFrames,
    dying_frames: GifFrames,
    idle_spritesheet: String,
    dying_spritesheet: String,
}

impl EnemyModel {
    pub(super) fn new(
        max_health: f64,
        damage: u64,
        move_speed: f64,
        coin_reward: usize,
        death_duration: f64,
        idle_frames: GifFrames,
        dying_frames: GifFrames,
        idle_spritesheet: String,
        dying_spritesheet: String,
    ) -> Self {
        Self {
            max_health,
            damage,
            move_speed,
            coin_reward,
            death_duration,
            idle_frames,
            dying_frames,
            idle_spritesheet,
            dying_spritesheet,
        }
    }

    pub fn get_max_health(&self) -> f64 {
        self.max_health
    }

    pub fn get_damage(&self) -> u64 {
        self.damage
    }

    pub fn get_move_speed(&self) -> f64 {
        self.move_speed
    }

    pub fn get_coin_reward(&self) -> usize {
        self.coin_reward
    }
}

#[derive(Serialize, Deserialize)]
pub struct Enemy {
    id: usize,
    pos: Vector2,
    health: f64,
    enemy_type: EnemyType,
    spawn_time: f64,
    state: Option<State>,
}

impl Enemy {
    pub(super) fn new(enemy_type: EnemyType, spawn_time: f64) -> Self {
        static ID_COUNTER: AtomicUsize = AtomicUsize::new(0);
        let id = ID_COUNTER.fetch_add(1, Ordering::Relaxed);
        Self {
            id,
            pos: Vector2::new(0.0, 0.0),
            health: enemy_type.get_model().get_max_health(),
            enemy_type,
            spawn_time,
            state: Some(State::Idle),
        }
    }

    pub fn get_enemy_type(&self) -> &EnemyType {
        &self.enemy_type
    }

    pub fn get_spawn_time(&self) -> f64 {
        self.spawn_time
    }

    pub fn is_alive(&self) -> bool {
        match self.state.as_ref().unwrap() {
            State::Idle => true,
            _ => false,
        }
    }

    pub fn is_dead(&self) -> bool {
        match self.state.as_ref().unwrap() {
            State::Dead => true,
            _ => false,
        }
    }

    pub fn update(&mut self, time: f64, map: &'static Map) {
        if let Some(state) = self.state.take() {
            self.state = Some(state.update(map, time, self));
        }
    }

    fn set_position(&mut self, new_pos: Vector2) {
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
