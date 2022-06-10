use crate::entity::gif::GifFrames;
use crate::entity::structure::structure::{
    RegisterStructureModel, StructureBase, StructureFactory, StructureModel, StructureModelMap,
    StructureUpdate,
};
use crate::entity::{Enemy, GameStructure, Structure};
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
    fn update(self, enemies: &mut Vec<Enemy>, time: f64, tower: &LightningTower) -> Self {
        match self {
            Self::Idle => self.idle_update(enemies, time, tower),
            Self::Attack {
                attack_start,
                did_attack,
            } => self.attack_update(attack_start, did_attack, enemies, time, tower),
            Self::Cooldown { attack_end } => self.cooldown_update(attack_end, time, tower),
        }
    }

    fn idle_update(self, enemies: &mut Vec<Enemy>, time: f64, tower: &LightningTower) -> Self {
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
        tower: &LightningTower,
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

    fn cooldown_update(self, attack_end: f64, time: f64, tower: &LightningTower) -> Self {
        if (attack_end + tower.model.attack_cooldown) < time {
            return self;
        }

        Self::Idle {}
    }
}

/****************************************
* Grunt
*****************************************/

#[derive(Serialize)]
pub struct LightningTower {
    #[serde(flatten)]
    base: StructureBase,
    #[serde(serialize_with = "model_serialize")]
    model: &'static LightningTowerModel,
    state: Option<State>,
}

impl LightningTower {
    const MAX_HEALTH: f64 = 100.0;
    const IDLE_SPRITESHEET: &'static str = "structures/blitz_turm/blitz_turm_v2_idle.png";
    const ATTACK_SPRITESHEET: &'static str = "structures/blitz_turm/blitz_turm_v2.png";
    const RADIUS: f64 = 20.0;
    const Y_OFFSET: f64 = 50.0;
    const ATTACK_RANGE: f64 = 50.0;
    const ATTACK_DAMAGE: f64 = 60.0;
    const ATTACK_COOLDOWN: f64 = 1000.0;
    const ATTACK_DAMAGE_DELAY: f64 = 650.0;
    const ATTACK_DURATION: f64 = 1000.0;
}

impl Structure for LightningTower {
    fn get_id(&self) -> usize {
        self.base.get_id()
    }

    fn get_position(&self) -> &Vector2 {
        self.base.get_position()
    }

    fn get_offset_position(&self) -> Vector2 {
        let pos = self.base.get_position();
        Vector2::new(pos.x(), pos.y() - LightningTower::Y_OFFSET)
    }

    fn set_position(&mut self, pos: Vector2) {
        self.base.set_position(pos)
    }

    fn get_radius(&self) -> &f64 {
        self.base.get_radius()
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

impl StructureUpdate for LightningTower {
    fn update(&mut self, enemies: &mut Vec<Enemy>, time: f64) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.update(enemies, time, self));
        }
    }
}

impl GameStructure for LightningTower {}

impl StructureFactory for LightningTower {
    fn new(pos: Vector2) -> Self {
        let base = StructureBase::new(LightningTower::MAX_HEALTH, pos, LightningTower::RADIUS);
        LightningTower {
            base,
            model: &LIGHTNING_TOWER_MODEL,
            state: Some(State::Idle),
        }
    }
}

impl RegisterStructureModel for LightningTower {
    fn register_model(model_map: &mut StructureModelMap) {
        model_map.insert(
            String::from("LightningTower"),
            Box::new((*LIGHTNING_TOWER_MODEL).clone()) as Box<dyn StructureModel + 'static>,
        );
    }
}

fn model_serialize<S>(_x: &LightningTowerModel, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    s.serialize_str("LightningTower")
}

/****************************************
* Grunt Model
*****************************************/
#[derive(Serialize, Clone)]
pub struct LightningTowerModel {
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
}

impl StructureModel for LightningTowerModel {}

/****************************************
* Static
*****************************************/

lazy_static! {
    static ref LIGHTNING_TOWER_MODEL: LightningTowerModel = {
        let file = File::open("resources/www/structures/blitz_turm/blitz_turm_v2.json")
            .expect("Could not find json file for Blitz Turm");
        let reader = BufReader::new(file);
        let attack_frames = serde_json::from_reader(reader)
            .expect("Could not parse gif frames for Lightning tower attack animation");

        let file = File::open("resources/www/structures/blitz_turm/blitz_turm_v2_idle.json")
            .expect("Could not find json file for Blitz Turm");
        let reader = BufReader::new(file);
        let idle_frames = serde_json::from_reader(reader)
            .expect("Could not parse gif frames for Lightning tower idle animation");

        // The attack damage delay is the time it takes from the animation start until the damage
        // is applied. If it were longer than the entire attack duration the damage would
        // never get applied.
        debug_assert!(LightningTower::ATTACK_DAMAGE_DELAY < LightningTower::ATTACK_DURATION);

        LightningTowerModel {
            attack_frames,
            idle_frames,
            icon: String::from("structures/blitz_turm/blitz_turm_v2_icon.png"),
            attack_spritesheet: LightningTower::ATTACK_SPRITESHEET.to_string(),
            idle_spritesheet: LightningTower::IDLE_SPRITESHEET.to_string(),
            radius: LightningTower::RADIUS,
            max_health: LightningTower::MAX_HEALTH,
            attack_range: LightningTower::ATTACK_RANGE,
            attack_damage: LightningTower::ATTACK_DAMAGE,
            attack_cooldown: LightningTower::ATTACK_COOLDOWN,
            attack_damage_delay: LightningTower::ATTACK_DAMAGE_DELAY,
            attack_duration: LightningTower::ATTACK_DURATION,
        }
    };
}
