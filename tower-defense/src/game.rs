use crate::entity::{Enemy, EnemyType, GameStructure, StructureType};
use crate::map::Map;
use crate::map::Wave;
use crate::math::Vector2;
use serde::Serialize;

#[derive(Debug, Clone)]
pub struct GameError {
    message: String,
}

impl GameError {
    pub fn new(message: String) -> Self {
        Self { message }
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}

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

    is_game_over: bool,
}

impl Game {
    pub fn new(map: &'static Map) -> Game {
        Game {
            map,
            time: 0.0,
            enemies: vec![],
            structures: vec![StructureType::LightningTowerV1.new(Vector2::new(100.0, 300.0))],
            current_lives: map.get_max_lives(),
            wave: Wave::new(300.0, 1500.0),
            is_game_over: false,
        }
    }

    pub fn start(&mut self) {}

    pub fn update(&mut self, delta_time: f64) -> usize {
        if self.is_game_over {
            return 0;
        }
        self.time += delta_time;
        for structure in &mut self.structures {
            structure.update(&mut self.enemies, self.time);
        }
        if let Some(enemy) = self.wave.update(delta_time) {
            let enemy = enemy.new(self.time);
            self.enemies.push(enemy);
        }
        self.update_enemies();
        let gold_earned = self.remove_dead_enemies();
        self.check_enemies_in_base();
        gold_earned
    }

    pub fn get_map(&self) -> &Map {
        &self.map
    }

    pub fn try_place_structure(
        &mut self,
        structure: StructureType,
        pos: Vector2,
    ) -> Result<(), GameError> {
        let new_structure = structure.new(pos);

        for structure in &self.structures {
            let distance = (&structure.get_offset_position()
                - &new_structure.get_offset_position())
                .magnitude();
            if distance < structure.get_radius() + new_structure.get_radius() {
                return Err(GameError::new(String::from("Area obstructed")));
            }
        }

        self.structures.push(new_structure);
        Ok(())
    }

    pub fn upgrade_structure(&mut self, id: usize) -> Result<(), GameError> {
        let mut new_structure = None;
        let mut pos = Vector2::new(0.0, 0.0);
        self.structures.retain(|structure| {
            if id == structure.get_id() {
                new_structure = structure.get_upgrade();
                if let Some(_) = &new_structure {
                    pos = structure.get_position().clone();
                    return false;
                }
            }

            true
        });

        if let Some(structure) = new_structure {
            self.structures.push(structure.new(pos));
            return Ok(());
        }

        Err(GameError::new(String::from("Could not upgrade")))
    }

    fn remove_dead_enemies(&mut self) -> usize {
        let mut gold_earned: usize = 0;
        self.enemies.retain(|enemy| {
            return if enemy.is_dead() {
                let coins = enemy.get_enemy_type().get_model().get_coin_reward();
                gold_earned += coins;
                false
            } else {
                true
            };
        });

        gold_earned
    }

    fn update_enemies(&mut self) {
        for enemy in self.enemies.iter_mut() {
            enemy.update(self.time, self.map);
        }
    }

    fn check_enemies_in_base(&mut self) {
        let rect = self.map.get_base();
        self.enemies.retain(|enemy| {
            if !enemy.is_alive() {
                return true;
            }
            let is_inside = rect.is_inside(enemy.get_position());
            if is_inside {
                self.current_lives -= 1;
                if self.current_lives == 0 {
                    self.is_game_over = true;
                }
            }

            !is_inside
        });
    }

    pub fn find_structure(&self, id: usize) -> Option<&Box<dyn GameStructure>> {
        self.structures.iter().find(|x| x.get_id() == id)
    }
}
