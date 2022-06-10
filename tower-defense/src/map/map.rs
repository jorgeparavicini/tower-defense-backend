use crate::map::path::Path;
use crate::math::Rect;
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
        path: Path,
        base: Rect,
    ) -> Map {
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
