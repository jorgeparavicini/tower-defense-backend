use crate::entity::{Enemy, EnemyType, GameStructure, StructureType};
use crate::map::Map;
use crate::map::Wave;
use crate::math::Vector2;
use serde::Serialize;

#[derive(Debug, Clone)]
pub struct GameError;

#[derive(Serialize)]
pub struct Game {
    #[serde(skip_serializing)]
    map: &'static Map,
    time: f64,
    enemies: Vec<Enemy>,
    structures: Vec<Box<dyn GameStructure>>,
    current_lives: u64,

    #[serde(skip_serializing)]
    wave: Wave,
}

impl Game {
    pub fn new(map: &'static Map) -> Game {
        Game {
            map,
            time: 0.0,
            enemies: vec![],
            structures: vec![StructureType::LightningTower.new(Vector2::new(100.0, 300.0))],
            current_lives: map.get_max_lives() - 2,
            wave: Wave::new(map.get_wave_elements()),
        }
    }

    pub fn start(&mut self) {
        let enemy = EnemyType::Recruit.new(self.time);
        self.enemies.push(enemy);
    }

    pub fn update(&mut self, delta_time: f64) {
        self.time += delta_time;
        for structure in &mut self.structures {
            structure.update(&mut self.enemies, self.time);
        }
        if let Some(enemy) = self.wave.update(self.time) {
            let enemy = enemy.new(self.time);
            self.enemies.push(enemy);
        }
        self.remove_dead_enemies();
        self.move_enemies();
        self.check_enemies_in_base();
    }

    pub fn get_map(&self) -> &Map {
        &self.map
    }

    pub fn try_place_structure(
        &mut self,
        structure: StructureType,
        pos: Vector2,
    ) -> Result<(), GameError> {
        // TODO: Check if structure position is valid
        let structure = structure.new(pos);
        self.structures.push(structure);

        Ok(())
    }

    fn remove_dead_enemies(&mut self) {
        self.enemies.retain(|enemy| enemy.get_health() > 0.0)
    }

    fn move_enemies(&mut self) {
        for enemy in self.enemies.iter_mut() {
            let move_speed = enemy.get_enemy_type().get_enemy_data().get_move_speed();
            let t = (self.time - enemy.get_spawn_time()) / 1000.0;
            enemy.set_position(self.map.get_path().coords_at(t * move_speed));
        }
    }

    fn check_enemies_in_base(&mut self) {
        let rect = self.map.get_base();
        self.enemies.retain(|enemy| {
            let is_inside = rect.is_inside(enemy.get_position());
            if is_inside {
                self.current_lives -= 1;
            }

            !is_inside
        });
    }
}
