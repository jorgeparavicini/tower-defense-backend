use crate::entity::structure::structure::{Structure, StructureData};
use crate::entity::structure::GRUNT;
use crate::math::Vector2;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum StructureType {
    Grunt,
}

impl StructureType {
    pub fn new(self, pos: Vector2) -> Structure {
        Structure::new(self, pos)
    }

    pub fn get_structure_data(&self) -> &'static Box<dyn StructureData + Send + Sync> {
        match self {
            StructureType::Grunt => &*GRUNT,
        }
    }
}
