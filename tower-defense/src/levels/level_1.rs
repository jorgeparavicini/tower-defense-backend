use crate::core::{Map, Size};
use crate::math::{Rect, Vector2};

lazy_static! {
    pub static ref MAP_LEVEL_1: Map = {
        Map::new(
            String::from("map_1/map_1.png"),
            String::from("map_1/stone_filler.png"),
            Size::new(600, 400),
            6,
            Rect::new(
                Vector2::new(550.0, 170.0),
                Vector2::new(600.0, 170.0),
                Vector2::new(550.0, 190.0),
            ),
        )
    };
}
