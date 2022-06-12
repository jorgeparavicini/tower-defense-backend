use crate::map::path::{Line, Path};
use crate::map::{Map, Size};
use crate::math::{Rect, Vector2};

lazy_static! {
    pub static ref MAP_LEVEL_1: Map = {
        let path = Path::new(vec![
            Box::new(Line::new(
                Vector2::new(0.0, 180.0),
                Vector2::new(310.0, 180.0),
            )),
            Box::new(Line::new(
                Vector2::new(310.0, 180.0),
                Vector2::new(310.0, 570.0),
            )),
            Box::new(Line::new(
                Vector2::new(310.0, 570.0),
                Vector2::new(635.0, 570.0),
            )),
            Box::new(Line::new(
                Vector2::new(635.0, 570.0),
                Vector2::new(635.0, 140.0),
            )),
            Box::new(Line::new(
                Vector2::new(635.0, 140.0),
                Vector2::new(1685.0, 140.0),
            )),
            Box::new(Line::new(
                Vector2::new(1685.0, 140.0),
                Vector2::new(1685.0, 795.0),
            )),
            Box::new(Line::new(
                Vector2::new(1685.0, 795.0),
                Vector2::new(160.0, 795.0),
            )),
            Box::new(Line::new(
                Vector2::new(160.0, 795.0),
                Vector2::new(160.0, 1080.0),
            )),
        ]);

        Map::new(
            String::from("map_1/map_1.png"),
            String::from("map_1/stone_filler.png"),
            Size::new(1920, 1080),
            6,
            path,
            Rect::new(
                Vector2::new(140.0, 1050.0),
                Vector2::new(140.0, 1080.0),
                Vector2::new(180.0, 1080.0),
            ),
        )
    };
}
