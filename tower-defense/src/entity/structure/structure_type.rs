use crate::entity::structure::structure::{Structure, StructureData};
use crate::entity::structure::GRUNT;
use crate::math::Vector2;
use crate::Game;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use std::fmt::Formatter;
use std::rc::Weak;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

lazy_static! {
    pub static ref STRUCTURE_MAP: HashMap<String, &'static Box<dyn StructureData + Send + Sync>> = {
        let mut map = HashMap::new();
        for structure_type in StructureType::iter() {
            map.insert(
                structure_type.to_string(),
                structure_type.get_structure_data(),
            );
        }

        map
    };
}

#[derive(Serialize, Deserialize, Debug, EnumIter)]
pub enum StructureType {
    Grunt,
}

impl StructureType {
    pub fn new(self, pos: Vector2, game: Weak<Game>) -> Structure {
        Structure::new(self, pos, game)
    }

    pub fn get_structure_data(&self) -> &'static Box<dyn StructureData + Send + Sync> {
        match self {
            StructureType::Grunt => &*GRUNT,
        }
    }
}

impl fmt::Display for StructureType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
