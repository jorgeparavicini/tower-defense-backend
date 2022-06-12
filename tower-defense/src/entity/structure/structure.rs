use crate::entity::structure::instance::{LIGHTNING_TOWER_MODEL, LIGHTNING_TOWER_V1_MODEL};
use crate::entity::structure::LightningTower;
use crate::entity::structure::LightningTowerV1;
use crate::entity::{
    Enemy, KonfettiKanoneV1, KonfettiKanoneV2, SingleShotTowerV1, KONFETTI_KANONE_MODEL,
    KONFETTI_KANONE_MODEL_V2, SINGLE_SHOT_TOWER_V1_MODEL,
};
use crate::math::Vector2;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use std::fmt::Formatter;
use std::sync::atomic::{AtomicUsize, Ordering};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

pub type StructureModelMap = HashMap<String, Box<dyn StructureModel + 'static>>;

pub trait StructureUpdate {
    fn update(&mut self, enemies: &mut Vec<Enemy>, time: f64);
}

pub trait GameStructure:
    Structure + StructureUpdate + erased_serde::Serialize + Send + Sync
{
}

serialize_trait_object!(GameStructure);

/****************************************
* Structure
*****************************************/

pub trait Structure {
    fn get_id(&self) -> usize;
    fn get_position(&self) -> &Vector2;
    fn get_offset_position(&self) -> Vector2;
    fn set_position(&mut self, pos: Vector2);
    fn get_radius(&self) -> &f64;
    fn get_upgrade(&self) -> Option<StructureType>;

    fn get_health(&self) -> f64;
    fn inflict_damage(&mut self, damage: f64);
    fn heal(&mut self, amount: f64);
}

/****************************************
* Structure Base
*****************************************/

#[derive(Serialize, Deserialize)]
pub struct StructureBase {
    id: usize,
    pos: Vector2,
    health: f64,
    radius: f64,
}

impl StructureBase {
    pub(crate) fn new(health: f64, pos: Vector2, radius: f64) -> Self {
        static ID_COUNTER: AtomicUsize = AtomicUsize::new(0);
        let id = ID_COUNTER.fetch_add(1, Ordering::Relaxed);

        StructureBase {
            id,
            pos,
            health,
            radius,
        }
    }
}

impl Structure for StructureBase {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_position(&self) -> &Vector2 {
        &self.pos
    }

    fn get_offset_position(&self) -> Vector2 {
        Vector2::new(0.0, 0.0)
    }

    fn set_position(&mut self, pos: Vector2) {
        self.pos = pos;
    }

    fn get_radius(&self) -> &f64 {
        &self.radius
    }

    fn get_upgrade(&self) -> Option<StructureType> {
        None
    }

    fn get_health(&self) -> f64 {
        self.health
    }

    fn inflict_damage(&mut self, damage: f64) {
        self.health -= damage;
    }

    fn heal(&mut self, amount: f64) {
        self.health += amount;
    }
}

/****************************************
* Structure Factory
*****************************************/

pub trait StructureFactory {
    fn new(pos: Vector2) -> Self;
}

pub trait StructureModel: Sync + Send + erased_serde::Serialize {
    fn get_cost(&self) -> usize;
}

pub trait RegisterStructureModel {
    fn register_model(model_map: &mut StructureModelMap);
}

serialize_trait_object!(StructureModel);

/****************************************
* Structure Type
*****************************************/

#[derive(Serialize, Deserialize, Debug, EnumIter)]
pub enum StructureType {
    LightningTowerV1,
    LightningTower,
    KonfettiKanoneV1,
    KonfettiKanoneV2,
    SingleShotTowerV1,
}

impl StructureType {
    pub fn new(self, pos: Vector2) -> Box<dyn GameStructure> {
        match self {
            StructureType::LightningTowerV1 => Box::new(LightningTowerV1::new(pos)),
            StructureType::LightningTower => Box::new(LightningTower::new(pos)),
            StructureType::KonfettiKanoneV1 => Box::new(KonfettiKanoneV1::new(pos)),
            StructureType::KonfettiKanoneV2 => Box::new(KonfettiKanoneV2::new(pos)),
            StructureType::SingleShotTowerV1 => Box::new(SingleShotTowerV1::new(pos)),
        }
    }

    fn register_model(&self, model_map: &mut StructureModelMap) {
        match self {
            StructureType::LightningTowerV1 => LightningTowerV1::register_model(model_map),
            StructureType::LightningTower => LightningTower::register_model(model_map),
            StructureType::KonfettiKanoneV1 => KonfettiKanoneV1::register_model(model_map),
            StructureType::KonfettiKanoneV2 => KonfettiKanoneV2::register_model(model_map),
            StructureType::SingleShotTowerV1 => SingleShotTowerV1::register_model(model_map),
        }
    }

    pub fn get_model(&self) -> Box<&dyn StructureModel> {
        match self {
            StructureType::LightningTowerV1 => Box::new(&*LIGHTNING_TOWER_V1_MODEL),
            StructureType::LightningTower => Box::new(&*LIGHTNING_TOWER_MODEL),
            StructureType::KonfettiKanoneV1 => Box::new(&*KONFETTI_KANONE_MODEL),
            StructureType::KonfettiKanoneV2 => Box::new(&*KONFETTI_KANONE_MODEL_V2),
            StructureType::SingleShotTowerV1 => Box::new(&*SINGLE_SHOT_TOWER_V1_MODEL),
        }
    }
}

impl fmt::Display for StructureType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/****************************************
* Structure Map
*****************************************/

lazy_static! {
    pub static ref STRUCTURE_MODEL_MAP: StructureModelMap = {
        let mut map: StructureModelMap = HashMap::new();
        for structure_type in StructureType::iter() {
            structure_type.register_model(&mut map);
        }

        map
    };
}
