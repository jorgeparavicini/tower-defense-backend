use crate::entity::structure::LightningTower;
use crate::entity::Enemy;
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
    fn set_position(&mut self, pos: Vector2);

    fn get_health(&self) -> f64;
    fn inflict_damage(&mut self, damage: f64);
    fn heal(&mut self, amount: f64);
}

/****************************************
* Structure Base
*****************************************/

#[derive(Serialize)]
pub struct StructureBase {
    id: usize,
    pos: Vector2,
    health: f64,
}

impl StructureBase {
    pub(crate) fn new(health: f64, pos: Vector2) -> Self {
        static ID_COUNTER: AtomicUsize = AtomicUsize::new(0);
        let id = ID_COUNTER.fetch_add(1, Ordering::Relaxed);

        StructureBase { id, pos, health }
    }
}

impl Structure for StructureBase {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_position(&self) -> &Vector2 {
        &self.pos
    }

    fn set_position(&mut self, pos: Vector2) {
        self.pos = pos;
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

pub trait StructureModel: Sync + Send + erased_serde::Serialize {}

pub trait RegisterStructureModel {
    fn register_model(model_map: &mut StructureModelMap);
}

serialize_trait_object!(StructureModel);

/****************************************
* Structure Type
*****************************************/

#[derive(Serialize, Deserialize, Debug, EnumIter)]
pub enum StructureType {
    LightningTower,
}

impl StructureType {
    pub fn new(self, pos: Vector2) -> Box<dyn GameStructure> {
        match self {
            StructureType::LightningTower => Box::new(LightningTower::new(pos)),
        }
    }

    fn register_model(&self, model_map: &mut StructureModelMap) {
        match self {
            StructureType::LightningTower => LightningTower::register_model(model_map),
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
