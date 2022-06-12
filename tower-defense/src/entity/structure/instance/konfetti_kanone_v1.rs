use crate::entity::gif::GifFrames;
use crate::entity::structure::structure::{
    RegisterStructureModel, StructureBase, StructureFactory, StructureModel, StructureModelMap,
    StructureUpdate,
};
use crate::entity::{Enemy, GameStructure, Structure, StructureType};
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
    Attack { attack_start: f64, did_attack: bool },
    Cooldown { attack_end: f64 },
}

impl State {
    fn update(self, enemies: &mut Vec<Enemy>, time: f64, tower: &KonfettiKanoneV1) -> Self {
        match self {
            Self::Idle => self.idle_update(enemies, time, tower),
            Self::Attack {
                attack_start,
                did_attack,
            } => self.attack_update(attack_start, did_attack, enemies, time, tower),
            Self::Cooldown { attack_end } => self.cooldown_update(attack_end, time, tower),
        }
    }

    fn idle_update(self, enemies: &mut Vec<Enemy>, time: f64, tower: &KonfettiKanoneV1) -> Self {
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
        tower: &KonfettiKanoneV1,
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

    fn cooldown_update(self, attack_end: f64, time: f64, tower: &KonfettiKanoneV1) -> Self {
        if (attack_end + tower.model.attack_cooldown) < time {
            return Self::Idle {};
        }
        self
    }
}

/****************************************
* Konfetti Kanone
*****************************************/

#[derive(Serialize)]
pub struct KonfettiKanoneV1 {
    #[serde(flatten)]
    base: StructureBase,
    #[serde(serialize_with = "model_serialize")]
    model: &'static KonfettiKanoneModel,
    state: Option<State>,
}

impl KonfettiKanoneV1 {
    const MAX_HEALTH: f64 = 100.0;
    const IDLE_SPRITESHEET: &'static str = "structures/konfetti_kanone/konfetti_kanone_v1_idle.png";
    const ATTACK_SPRITESHEET: &'static str =
        "structures/konfetti_kanone/konfetti_kanone_v1_attack.png";
    const RADIUS: f64 = 50.0;
    const Y_OFFSET: f64 = 50.0;
    const ATTACK_RANGE: f64 = 120.0;
    const ATTACK_DAMAGE: f64 = 80.0;
    const ATTACK_COOLDOWN: f64 = 5000.0;
    const ATTACK_DAMAGE_DELAY: f64 = 650.0;
    const ATTACK_DURATION: f64 = 1000.0;

    pub fn load(value: &Value) -> Self {
        let base: StructureBase = serde_json::from_value(value.clone()).unwrap();
        let state: State = serde_json::from_value(value["state"].clone()).unwrap();
        Self {
            base,
            model: &KONFETTI_KANONE_MODEL,
            state: Some(state),
        }
    }
}

impl Structure for KonfettiKanoneV1 {
    fn get_id(&self) -> usize {
        self.base.get_id()
    }

    fn get_position(&self) -> &Vector2 {
        self.base.get_position()
    }

    fn get_offset_position(&self) -> Vector2 {
        let pos = self.base.get_position();
        Vector2::new(pos.x(), pos.y() - KonfettiKanoneV1::Y_OFFSET)
    }

    fn set_position(&mut self, pos: Vector2) {
        self.base.set_position(pos)
    }

    fn get_radius(&self) -> &f64 {
        self.base.get_radius()
    }

    fn get_upgrade(&self) -> Option<StructureType> {
        Some(StructureType::KonfettiKanoneV2)
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

impl StructureUpdate for KonfettiKanoneV1 {
    fn update(&mut self, enemies: &mut Vec<Enemy>, time: f64) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.update(enemies, time, self));
        }
    }
}

impl GameStructure for KonfettiKanoneV1 {}

impl StructureFactory for KonfettiKanoneV1 {
    fn new(pos: Vector2) -> Self {
        let base = StructureBase::new(KonfettiKanoneV1::MAX_HEALTH, pos, KonfettiKanoneV1::RADIUS);
        KonfettiKanoneV1 {
            base,
            model: &KONFETTI_KANONE_MODEL,
            state: Some(State::Idle),
        }
    }
}

impl RegisterStructureModel for KonfettiKanoneV1 {
    fn register_model(model_map: &mut StructureModelMap) {
        model_map.insert(
            String::from("KonfettiKanoneV1"),
            Box::new((*KONFETTI_KANONE_MODEL).clone()) as Box<dyn StructureModel + 'static>,
        );
    }
}

fn model_serialize<S>(_x: &KonfettiKanoneModel, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    s.serialize_str("KonfettiKanoneV1")
}

/****************************************
* Lightning Tower Model
*****************************************/
#[derive(Serialize, Clone)]
pub struct KonfettiKanoneModel {
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

impl StructureModel for KonfettiKanoneModel {
    fn get_cost(&self) -> usize {
        self.cost
    }
}

/****************************************
* Static
*****************************************/

lazy_static! {
    pub static ref KONFETTI_KANONE_MODEL: KonfettiKanoneModel = {
        let file = File::open("resources/www/structures/konfetti_kanone/konfetti_kanone_v1_attack.json")
            .expect("Could not find json file for Konfetti Kanone V1");
        let reader = BufReader::new(file);
        let attack_frames = serde_json::from_reader(reader)
            .expect("Could not parse gif frames for Konfetti Kanone V1 attack animation");

        let file = File::open("resources/www/structures/konfetti_kanone/konfetti_kanone_v1_idle.json")
            .expect("Could not find json file for Konfetti Kanone V1");
        let reader = BufReader::new(file);
        let idle_frames = serde_json::from_reader(reader)
            .expect("Could not parse gif frames for Konfetti Kanone V1 idle animation");

        // The attack damage delay is the time it takes from the animation start until the damage
        // is applied. If it were longer than the entire attack duration the damage would
        // never get applied.
        debug_assert!(KonfettiKanoneV1::ATTACK_DAMAGE_DELAY < KonfettiKanoneV1::ATTACK_DURATION);

        KonfettiKanoneModel {
            attack_frames,
            idle_frames,
            icon: String::from("structures/konfetti_kanone/konfetti_kanone_v1_icon.png"),
            attack_spritesheet: KonfettiKanoneV1::ATTACK_SPRITESHEET.to_string(),
            idle_spritesheet: KonfettiKanoneV1::IDLE_SPRITESHEET.to_string(),
            radius: KonfettiKanoneV1::RADIUS,
            max_health: KonfettiKanoneV1::MAX_HEALTH,
            attack_range: KonfettiKanoneV1::ATTACK_RANGE,
            attack_damage: KonfettiKanoneV1::ATTACK_DAMAGE,
            attack_cooldown: KonfettiKanoneV1::ATTACK_COOLDOWN,
            attack_damage_delay: KonfettiKanoneV1::ATTACK_DAMAGE_DELAY,
            attack_duration: KonfettiKanoneV1::ATTACK_DURATION,
            can_be_bought: true,
            can_be_upgraded: true,
            name: String::from("Konfetti Kanone"),
            level: 2,
            cost: 300
        }
    };
}
