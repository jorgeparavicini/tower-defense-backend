use crate::entity::gif::GifFrames;
use crate::entity::structure::structure::{
    RegisterStructureModel, StructureBase, StructureFactory, StructureModel, StructureModelMap,
    StructureUpdate,
};
use crate::entity::{Enemy, GameStructure, Structure, StructureType};
use crate::math::Vector2;
use serde::{Serialize, Serializer};
use std::fs::File;
use std::io::BufReader;

/****************************************
* States
*****************************************/

#[derive(Serialize)]
#[serde(tag = "type", content = "data")]
enum State {
    Idle,
    Attack { attack_start: f64, did_attack: bool },
    Cooldown { attack_end: f64 },
}

impl State {
    fn update(self, enemies: &mut Vec<Enemy>, time: f64, tower: &LightningTowerV1) -> Self {
        match self {
            Self::Idle => self.idle_update(enemies, time, tower),
            Self::Attack {
                attack_start,
                did_attack,
            } => self.attack_update(attack_start, did_attack, enemies, time, tower),
            Self::Cooldown { attack_end } => self.cooldown_update(attack_end, time, tower),
        }
    }

    fn idle_update(self, enemies: &mut Vec<Enemy>, time: f64, tower: &LightningTowerV1) -> Self {
        for enemy in enemies.iter() {
            if (&tower.get_offset_position() - enemy.get_position()).magnitude()
                < tower.model.attack_range
            {
                return State::Attack {
                    attack_start: time,
                    did_attack: false,
                };
            }
        }

        self
    }

    fn attack_update(
        self,
        attack_start: f64,
        did_attack: bool,
        enemies: &mut Vec<Enemy>,
        time: f64,
        tower: &LightningTowerV1,
    ) -> Self {
        if (attack_start + tower.model.attack_duration) < time {
            return Self::Cooldown { attack_end: time };
        }

        if !did_attack && (attack_start + tower.model.attack_damage_delay) < time {
            for enemy in enemies.iter_mut() {
                let distance = (&tower.get_offset_position() - enemy.get_position()).magnitude();
                if distance < tower.model.attack_range {
                    enemy.apply_damage(tower.model.attack_damage);
                }
            }

            return Self::Attack {
                attack_start,
                did_attack: true,
            };
        }

        self
    }

    fn cooldown_update(self, attack_end: f64, time: f64, tower: &LightningTowerV1) -> Self {
        if (attack_end + tower.model.attack_cooldown) < time {
            return self;
        }

        Self::Idle {}
    }
}

/****************************************
* Lightning Tower v1
*****************************************/

#[derive(Serialize)]
pub struct LightningTowerV1 {
    #[serde(flatten)]
    base: StructureBase,
    #[serde(serialize_with = "model_serialize")]
    model: &'static LightningTowerV1Model,
    state: Option<State>,
}

impl LightningTowerV1 {
    const MAX_HEALTH: f64 = 100.0;
    const IDLE_SPRITESHEET: &'static str = "structures/blitz_turm/blitz_turm_v1.png";
    const ATTACK_SPRITESHEET: &'static str = "structures/blitz_turm/blitz_turm_v1_attack.png";
    const RADIUS: f64 = 20.0;
    const Y_OFFSET: f64 = 50.0;
    const ATTACK_RANGE: f64 = 35.0;
    const ATTACK_DAMAGE: f64 = 50.0;
    const ATTACK_COOLDOWN: f64 = 3000.0;
    const ATTACK_DAMAGE_DELAY: f64 = 650.0;
    const ATTACK_DURATION: f64 = 1000.0;
}

impl Structure for LightningTowerV1 {
    fn get_id(&self) -> usize {
        self.base.get_id()
    }

    fn get_position(&self) -> &Vector2 {
        self.base.get_position()
    }

    fn get_offset_position(&self) -> Vector2 {
        let pos = self.base.get_position();
        Vector2::new(pos.x(), pos.y() - LightningTowerV1::Y_OFFSET)
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

impl StructureUpdate for LightningTowerV1 {
    fn update(&mut self, enemies: &mut Vec<Enemy>, time: f64) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.update(enemies, time, self));
        }
    }
}

impl GameStructure for LightningTowerV1 {}

impl StructureFactory for LightningTowerV1 {
    fn new(pos: Vector2) -> Self {
        let base = StructureBase::new(LightningTowerV1::MAX_HEALTH, pos, LightningTowerV1::RADIUS);
        LightningTowerV1 {
            base,
            model: &LIGHTNING_TOWER_V1_MODEL,
            state: Some(State::Idle),
        }
    }
}

impl RegisterStructureModel for LightningTowerV1 {
    fn register_model(model_map: &mut StructureModelMap) {
        model_map.insert(
            String::from("LightningTowerV1"),
            Box::new((*LIGHTNING_TOWER_V1_MODEL).clone()) as Box<dyn StructureModel + 'static>,
        );
    }
}

fn model_serialize<S>(_x: &LightningTowerV1Model, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    s.serialize_str("LightningTowerV1")
}

/****************************************
* Lightning Tower V1 Model
*****************************************/
#[derive(Serialize, Clone)]
pub struct LightningTowerV1Model {
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

impl StructureModel for LightningTowerV1Model {
    fn get_cost(&self) -> usize {
        self.cost
    }
}

/****************************************
* Static
*****************************************/

lazy_static! {
    pub static ref LIGHTNING_TOWER_V1_MODEL: LightningTowerV1Model = {
        let file = File::open("resources/www/structures/blitz_turm/blitz_turm_v1_attack.json")
            .expect("Could not find json file for Blitz Turm V1");
        let reader = BufReader::new(file);
        let attack_frames = serde_json::from_reader(reader)
            .expect("Could not parse gif frames for Lightning tower v1 attack animation");

        let file = File::open("resources/www/structures/blitz_turm/blitz_turm_v1.json")
            .expect("Could not find json file for Blitz Turm V1");
        let reader = BufReader::new(file);
        let idle_frames = serde_json::from_reader(reader)
            .expect("Could not parse gif frames for Lightning tower v1 idle animation");

        // The attack damage delay is the time it takes from the animation start until the damage
        // is applied. If it were longer than the entire attack duration the damage would
        // never get applied.
        debug_assert!(LightningTowerV1::ATTACK_DAMAGE_DELAY < LightningTowerV1::ATTACK_DURATION);

        LightningTowerV1Model {
            attack_frames,
            idle_frames,
            icon: String::from("structures/blitz_turm/blitz_turm_v1_icon.png"),
            attack_spritesheet: LightningTowerV1::ATTACK_SPRITESHEET.to_string(),
            idle_spritesheet: LightningTowerV1::IDLE_SPRITESHEET.to_string(),
            radius: LightningTowerV1::RADIUS,
            max_health: LightningTowerV1::MAX_HEALTH,
            attack_range: LightningTowerV1::ATTACK_RANGE,
            attack_damage: LightningTowerV1::ATTACK_DAMAGE,
            attack_cooldown: LightningTowerV1::ATTACK_COOLDOWN,
            attack_damage_delay: LightningTowerV1::ATTACK_DAMAGE_DELAY,
            attack_duration: LightningTowerV1::ATTACK_DURATION,
            can_be_bought: true,
            can_be_upgraded: true,
            name: String::from("Lightning Tower"),
            level: 1,
            cost: 150
        }
    };
}
