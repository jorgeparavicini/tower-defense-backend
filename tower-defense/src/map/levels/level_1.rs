use crate::entity::EnemyType;
use crate::map::path::{Line, Path};
use crate::map::wave::WaveElement;
use crate::map::{Map, Size};
use crate::math::{Rect, Vector2};

lazy_static! {
    pub static ref MAP_LEVEL_1: Map = {
        let path = Path::new(vec![
            Box::new(Line::new(
                Vector2::new(0.0, 220.0),
                Vector2::new(100.0, 220.0),
            )),
            Box::new(Line::new(
                Vector2::new(100.0, 220.0),
                Vector2::new(100.0, 100.0),
            )),
            Box::new(Line::new(
                Vector2::new(100.0, 100.0),
                Vector2::new(220.0, 100.0),
            )),
            Box::new(Line::new(
                Vector2::new(220.0, 100.0),
                Vector2::new(220.0, 260.0),
            )),
            Box::new(Line::new(
                Vector2::new(220.0, 260.0),
                Vector2::new(380.0, 260.0),
            )),
            Box::new(Line::new(
                Vector2::new(380.0, 260.0),
                Vector2::new(380.0, 180.0),
            )),
            Box::new(Line::new(
                Vector2::new(380.0, 180.0),
                Vector2::new(600.0, 180.0),
            )),
        ]);

        let wave = vec![
            WaveElement::new(500.0, EnemyType::Recruit),
            WaveElement::new(1000.0, EnemyType::Recruit),
        ];

        Map::new(
            String::from("map_1/map_1.png"),
            String::from("map_1/stone_filler.png"),
            Size::new(600, 400),
            6,
            path,
            Rect::new(
                Vector2::new(550.0, 170.0),
                Vector2::new(600.0, 170.0),
                Vector2::new(550.0, 190.0),
            ),
            wave,
        )
    };
}
