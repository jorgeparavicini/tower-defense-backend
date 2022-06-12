use crate::entity::enemy::enemy::{Enemy, EnemyModel};
use crate::entity::enemy::instance::{
    register_blue_model, register_purple_model, register_red_model, BLUE_MODEL, PURPLE_MODEL,
    RED_MODEL,
};
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

pub type EnemyModelMap = HashMap<String, &'static EnemyModel>;

#[derive(Serialize, Copy, Clone, Deserialize, Debug, EnumIter)]
pub enum EnemyType {
    Blue,
    Purple,
    Red,
}

impl EnemyType {
    pub fn new(self, spawn_time: f64) -> Enemy {
        Enemy::new(self, spawn_time)
    }

    pub fn get_model(&self) -> &'static EnemyModel {
        match self {
            EnemyType::Blue => &*BLUE_MODEL,
            EnemyType::Purple => &*PURPLE_MODEL,
            EnemyType::Red => &*RED_MODEL,
        }
    }

    pub fn random() -> EnemyType {
        let rng = rand::thread_rng().gen_range(0..3);
        match rng {
            0 => EnemyType::Blue,
            1 => EnemyType::Purple,
            2 => EnemyType::Red,
            _ => panic!(),
        }
    }

    fn register_model(&self, model_map: &mut EnemyModelMap) {
        match self {
            EnemyType::Blue => register_blue_model(model_map),
            EnemyType::Purple => register_purple_model(model_map),
            EnemyType::Red => register_red_model(model_map),
        }
    }
}

/****************************************
* Structure Map
*****************************************/

lazy_static! {
    pub static ref ENEMY_MODEL_MAP: EnemyModelMap = {
        let mut map: EnemyModelMap = HashMap::new();
        for enemy_type in EnemyType::iter() {
            enemy_type.register_model(&mut map);
        }

        map
    };
}
