use crate::math::{Rect, Vector2};
use crate::path::{Line, Path};
use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
pub struct Size {
    x: i32,
    y: i32,
}

#[derive(Serialize)]
pub struct Map {
    background_image: String,
    background_filler_image: String,
    size: Size,
    max_lives: u64,

    #[serde(skip_serializing)]
    path: Path,

    #[serde(skip_serializing)]
    base: Rect,
}

impl Size {
    pub fn new(x: i32, y: i32) -> Size {
        Size { x, y }
    }
}

impl Map {
    pub fn new(
        background_image: String,
        background_filler_image: String,
        size: Size,
        max_lives: u64,
        base: Rect,
    ) -> Map {
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

        Map {
            background_image,
            background_filler_image,
            size,
            max_lives,
            path,
            base,
        }
    }

    pub fn get_size(&self) -> Size {
        self.size.clone()
    }

    pub fn get_path(&self) -> &Path {
        &self.path
    }

    pub fn get_max_lives(&self) -> u64 {
        self.max_lives
    }

    pub fn get_base(&self) -> &Rect {
        &self.base
    }
}
