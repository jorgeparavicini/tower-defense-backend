use crate::core::Map;
use crate::entity::{Enemy, EnemyType, Structure, StructureType};
use crate::math::Vector2;
use serde::Serialize;
use std::cell::RefCell;

#[derive(Debug, Clone)]
pub struct GameError;

pub trait GameUpdate {
    fn update(&mut self, field: &mut GameField);
}

pub trait GameStructure: Structure + GameUpdate + erased_serde::Serialize {}

serialize_trait_object!(GameStructure);

#[derive(Serialize, Default)]
pub struct GameField {
    #[serde(skip_serializing)]
    map: &'static Map,
    time: f64,
    enemies: Vec<Enemy>,
    structures: Vec<Box<dyn GameStructure>>,
    current_lives: u64,
}

#[derive(Serialize)]
pub struct Game {
    #[serde(flatten)]
    field: RefCell<GameField>,
}

impl Game {
    pub fn new(map: &'static Map) -> Game {
        let field = GameField {
            map,
            time: 0.0,
            enemies: vec![],
            structures: vec![StructureType::Grunt.new(Vector2::new(10.0, 100.0))],
            current_lives: map.get_max_lives() - 2,
        };
        Game { field }
    }

    pub fn start(&mut self) {
        let enemy = EnemyType::Recruit.new(self.field.time);
        self.field.enemies.push(enemy);
    }

    pub fn update(&mut self, delta_time: f64) {
        self.field.time += delta_time;

        for enemy in self.field.borrow_mut().structures.iter_mut() {
            enemy.update(&mut self.field.borrow_mut());
        }
        self.move_enemies();
        self.check_enemies_in_base();
    }

    pub fn get_map(&self) -> &Map {
        &self.field.map
    }

    pub fn try_place_structure(
        &mut self,
        structure: StructureType,
        pos: Vector2,
    ) -> Result<(), GameError> {
        // TODO: Check if structure position is valid
        let structure = structure.new(pos);
        self.field.structures.push(structure);

        Ok(())
    }

    fn move_enemies(&mut self) {
        for enemy in self.field.enemies.iter_mut() {
            let move_speed = enemy.get_enemy_type().get_enemy_data().get_move_speed();
            let t = self.field.time - enemy.get_spawn_time();
            enemy.set_position(self.field.map.get_path().coords_at(t * move_speed));
        }
    }

    fn check_enemies_in_base(&mut self) {
        let rect = self.field.map.get_base();
        self.field.enemies.retain(|enemy| {
            let is_inside = rect.is_inside(enemy.get_position());
            if is_inside {
                self.field.current_lives -= 1;
            }

            !is_inside
        });
    }
}
