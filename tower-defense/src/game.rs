use crate::map::Map;
use crate::path::Coords;

pub struct Game<'a> {
    map: &'a Map,
    pos: Coords,
    time: f64
}

impl<'a> Game<'a> {
    pub fn new(map: &Map) -> Game {
        Game {
            map,
            pos: Coords::new(0.0, 0.0),
            time: 0.0
        }
    }

    pub fn update(&mut self, delta_time: f64) {
        self.time += delta_time * 0.2;
        self.pos = self.map.get_path().coords_at(self.time);
    }

    pub fn get_coords(&self) -> &Coords {
        &self.pos
    }

    pub fn get_map(&self) -> &Map {
        &self.map
    }
}