use crate::map::{Map, Size};

lazy_static! {
    pub static ref MAP_LEVEL_1: Map = {
        Map::new(String::from("map_1/map_1.png"),
            String::from("map_1/stone_filler.png"),
            Size::new(600, 400))
    };
}