use crate::entity::gif::GifFrames;
use crate::entity::structure::structure::{
    RegisterStructureModel, StructureBase, StructureFactory, StructureModel, StructureModelMap,
    StructureUpdate,
};
use crate::entity::{Enemy, GameStructure, Structure, StructureType};
use crate::map::path::{Line, PathComponent};
use crate::math::Vector2;
use serde::{Deserialize, Serialize, Serializer};
use serde_json::Value;
use std::fs::File;
use std::io::BufReader;

/****************************************
* States
*****************************************/

#[derive(Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
enum State {
    Idle,
    Attack {
        attack_start: f64,
        projectile_pos_x: f64,
        projectile_pos_y: f64,
        target_x: f64,
        target_y: f64,
    },
    Cooldown {
        attack_end: f64,
    },
}

impl State {
    fn update(self, enemies: &mut Vec<Enemy>, time: f64, tower: &SingleShotTowerV1) -> Self {
        match self {
            Self::Idle => self.idle_update(enemies, time, tower),
            Self::Attack {
                attack_start,
                projectile_pos_x,
                projectile_pos_y,
                target_x,
                target_y,
            } => self.attack_update(
                attack_start,
                Vector2::new(projectile_pos_x, projectile_pos_y),
                Vector2::new(target_x, target_y),
                enemies,
                time,
                tower,
            ),
            Self::Cooldown { attack_end } => self.cooldown_update(attack_end, time, tower),
        }
    }

    fn idle_update(self, enemies: &mut Vec<Enemy>, time: f64, tower: &SingleShotTowerV1) -> Self {
        for enemy in enemies.iter() {
            if (&tower.get_offset_position() - enemy.get_position()).magnitude()
                < tower.model.attack_range
            {
                let pos = enemy.get_position().clone();
                let projectile_pos = tower.get_position().clone();
                return State::Attack {
                    attack_start: time,
                    projectile_pos_x: projectile_pos.x(),
                    projectile_pos_y: projectile_pos.y(),
                    target_x: pos.x(),
                    target_y: pos.y(),
                };
            }
        }

        self
    }

    fn attack_update(
        self,
        attack_start: f64,
        _: Vector2,
        target: Vector2,
        enemies: &mut Vec<Enemy>,
        time: f64,
        tower: &SingleShotTowerV1,
    ) -> Self {
        const TRAVEL_DURATION: f64 = 1000.0;
        const RADIUS: f64 = 10.0;
        let t = 1.0 - (time - attack_start) / TRAVEL_DURATION;
        let line = Line::new(tower.get_position().clone(), target.clone());
        let new_pos = line.coords_at(t);

        for enemy in enemies.iter_mut() {
            let distance = (&new_pos - enemy.get_position()).magnitude();
            if distance < RADIUS {
                enemy.apply_damage(tower.model.attack_damage);
                return Self::Cooldown { attack_end: time };
            }
        }

        Self::Attack {
            attack_start,
            projectile_pos_x: new_pos.x(),
            projectile_pos_y: new_pos.y(),
            target_x: target.x(),
            target_y: target.y(),
        }
    }

    fn cooldown_update(self, attack_end: f64, time: f64, tower: &SingleShotTowerV1) -> Self {
        if (attack_end + tower.model.attack_cooldown) < time {
            return Self::Idle {};
        }
        self
    }
}

/****************************************
* Single Shot V1
*****************************************/

#[derive(Serialize)]
pub struct SingleShotTowerV1 {
    #[serde(flatten)]
    base: StructureBase,
    #[serde(serialize_with = "model_serialize")]
    model: &'static SingleShotTowerV1Model,
    state: Option<State>,
}

impl SingleShotTowerV1 {
    const MAX_HEALTH: f64 = 100.0;
    const IDLE_SPRITESHEET: &'static str = "structures/single/single_shot_v1_idle.png";
    const ATTACK_SPRITESHEET: &'static str = "structures/single/single_shot_v1_attack.png";
    const RADIUS: f64 = 50.0;
    const Y_OFFSET: f64 = 50.0;
    const ATTACK_RANGE: f64 = 100.0;
    const ATTACK_DAMAGE: f64 = 150.0;
    const ATTACK_COOLDOWN: f64 = 2000.0;
    const ATTACK_DAMAGE_DELAY: f64 = 300.0;
    const ATTACK_DURATION: f64 = 500.0;

    pub fn load(value: &Value) -> Self {
        let base: StructureBase = serde_json::from_value(value.clone()).unwrap();
        let state: State = serde_json::from_value(value["state"].clone()).unwrap();
        Self {
            base,
            model: &SINGLE_SHOT_TOWER_V1_MODEL,
            state: Some(state),
        }
    }
}

impl Structure for SingleShotTowerV1 {
    fn get_id(&self) -> usize {
        self.base.get_id()
    }

    fn get_position(&self) -> &Vector2 {
        self.base.get_position()
    }

    fn get_offset_position(&self) -> Vector2 {
        let pos = self.base.get_position();
        Vector2::new(pos.x(), pos.y() - SingleShotTowerV1::Y_OFFSET)
    }

    fn set_position(&mut self, pos: Vector2) {
        self.base.set_position(pos)
    }

    fn get_radius(&self) -> &f64 {
        self.base.get_radius()
    }

    fn get_upgrade(&self) -> Option<StructureType> {
        Some(StructureType::LightningTower)
    }

    fn get_health(&self) -> f64 {
        self.base.get_health()
    }

    fn inflict_damage(&mut self, damage: f64) {
        self.base.inflict_damage(damage)
    }

    fn heal(&mut self, amount: f64) {
        self.base.heal(amount)
    }
}

impl StructureUpdate for SingleShotTowerV1 {
    fn update(&mut self, enemies: &mut Vec<Enemy>, time: f64) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.update(enemies, time, self));
        }
    }
}

impl GameStructure for SingleShotTowerV1 {}

impl StructureFactory for SingleShotTowerV1 {
    fn new(pos: Vector2) -> Self {
        let base = StructureBase::new(
            SingleShotTowerV1::MAX_HEALTH,
            pos,
            SingleShotTowerV1::RADIUS,
        );
        SingleShotTowerV1 {
            base,
            model: &SINGLE_SHOT_TOWER_V1_MODEL,
            state: Some(State::Idle),
        }
    }
}

impl RegisterStructureModel for SingleShotTowerV1 {
    fn register_model(model_map: &mut StructureModelMap) {
        model_map.insert(
            String::from("SingleShotTowerV1"),
            Box::new((*SINGLE_SHOT_TOWER_V1_MODEL).clone()) as Box<dyn StructureModel + 'static>,
        );
    }
}

fn model_serialize<S>(_x: &SingleShotTowerV1Model, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    s.serialize_str("SingleShotTowerV1")
}

/****************************************
* Single Shot Tower V1 Model
*****************************************/
#[derive(Serialize, Clone)]
pub struct SingleShotTowerV1Model {
    attack_frames: GifFrames,
    idle_frames: GifFrames,
    icon: String,
    attack_spritesheet: String,
    idle_spritesheet: String,
    radius: f64,
    max_health: f64,
    attack_range: f64,
    attack_damage: f64,
    attack_cooldown: f64,
    attack_damage_delay: f64,
    attack_duration: f64,
    can_be_bought: bool,
    can_be_upgraded: bool,
    name: String,
    level: i64,
    cost: usize,
}

impl StructureModel for SingleShotTowerV1Model {
    fn get_cost(&self) -> usize {
        self.cost
    }
}

/****************************************
* Static
*****************************************/

lazy_static! {
    pub static ref SINGLE_SHOT_TOWER_V1_MODEL: SingleShotTowerV1Model = {
        let file = File::open("resources/www/structures/single/single_shot_v1_attack.json")
            .expect("Could not find json file for Blitz Turm V1");
        let reader = BufReader::new(file);
        let attack_frames = serde_json::from_reader(reader)
            .expect("Could not parse gif frames for Lightning tower v1 attack animation");

        let file = File::open("resources/www/structures/single/single_shot_v1_idle.json")
            .expect("Could not find json file for Blitz Turm V1");
        let reader = BufReader::new(file);
        let idle_frames = serde_json::from_reader(reader)
            .expect("Could not parse gif frames for Lightning tower v1 idle animation");

        // The attack damage delay is the time it takes from the animation start until the damage
        // is applied. If it were longer than the entire attack duration the damage would
        // never get applied.
        debug_assert!(SingleShotTowerV1::ATTACK_DAMAGE_DELAY < SingleShotTowerV1::ATTACK_DURATION);

        SingleShotTowerV1Model {
            attack_frames,
            idle_frames,
            icon: String::from("structures/single/single_shot_v1_icon.png"),
            attack_spritesheet: SingleShotTowerV1::ATTACK_SPRITESHEET.to_string(),
            idle_spritesheet: SingleShotTowerV1::IDLE_SPRITESHEET.to_string(),
            radius: SingleShotTowerV1::RADIUS,
            max_health: SingleShotTowerV1::MAX_HEALTH,
            attack_range: SingleShotTowerV1::ATTACK_RANGE,
            attack_damage: SingleShotTowerV1::ATTACK_DAMAGE,
            attack_cooldown: SingleShotTowerV1::ATTACK_COOLDOWN,
            attack_damage_delay: SingleShotTowerV1::ATTACK_DAMAGE_DELAY,
            attack_duration: SingleShotTowerV1::ATTACK_DURATION,
            can_be_bought: true,
            can_be_upgraded: false,
            name: String::from("Single Shot Tower"),
            level: 1,
            cost: 150
        }
    };
}
