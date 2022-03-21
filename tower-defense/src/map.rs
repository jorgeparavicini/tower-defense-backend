use serde::Serialize;
use crate::path::{Coords, Line, Path};

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

    #[serde(skip_serializing)]
    path: Path,
}


impl Size {
    pub fn new(x: i32, y: i32) -> Size {
        Size { x, y }
    }
}

impl Map {
    pub fn new(background_image: String, background_filler_image: String, size: Size) -> Map {
        let path = Path::new(vec![
            Box::new(Line::new(Coords::new(0.0, 200.0), Coords::new(100.0, 200.0))),
            Box::new(Line::new(Coords::new(100.0, 200.0), Coords::new(100.0, 100.0))),
            Box::new(Line::new(Coords::new(100.0, 100.0), Coords::new(300.0, 100.0))),
        ]);

        Map {
            background_image,
            background_filler_image,
            size,
            path,
        }
    }

    pub fn get_size(&self) -> Size {
        self.size.clone()
    }

    pub fn get_path(&self) -> &Path {
        &self.path
    }
}