use crate::entity::structure::structure_type::StructureType;
use crate::math::Vector2;
use serde::Serialize;
use std::sync::atomic::{AtomicUsize, Ordering};

pub trait StructureData {
    fn get_max_health(&self) -> f64;
}

#[derive(Serialize)]
pub struct Structure {
    id: usize,
    pos: Vector2,
    health: f64,
    structure_type: StructureType,
}

impl Structure {
    pub(super) fn new(structure_type: StructureType, pos: Vector2) -> Self {
        static ID_COUNTER: AtomicUsize = AtomicUsize::new(0);
        let id = ID_COUNTER.fetch_add(1, Ordering::Relaxed);

        Self {
            id,
            pos,
            health: structure_type.get_structure_data().get_max_health(),
            structure_type,
        }
    }

    pub fn get_structure_type(&self) -> &StructureType {
        &self.structure_type
    }

    pub fn get_position(&self) -> &Vector2 {
        &self.pos
    }

    pub fn get_health(&self) -> f64 {
        self.health
    }

    pub fn apply_damage(&mut self, damage: f64) {
        self.health -= damage;
    }

    pub fn heal(&mut self, healing_amount: f64) {
        self.health += healing_amount;
    }
}
