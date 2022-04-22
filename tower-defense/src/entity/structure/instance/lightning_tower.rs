use crate::entity::gif::GifFrames;
use crate::entity::structure::structure::{
    RegisterStructureModel, StructureBase, StructureFactory, StructureModel, StructureModelMap,
    StructureUpdate,
};
use crate::entity::{Enemy, GameStructure, Structure};
use crate::math::Vector2;
use log::info;
use serde::{Serialize, Serializer};
use std::fs::File;
use std::io::BufReader;

/****************************************
* Grunt
*****************************************/

#[derive(Serialize)]
pub struct LightningTower {
    #[serde(flatten)]
    base: StructureBase,
    #[serde(serialize_with = "model_serialize")]
    model: &'static LightningTowerModel,
    last_attack_time: Option<f64>,
}

impl LightningTower {
    const MAX_HEALTH: f64 = 100.0;
    const GIF_NAME: &'static str = "structures/blitz_turm/blitz_turm_v2_.png";
    const ATTACK_RANGE: f64 = 100.0;
    const ATTACK_DAMAGE: f64 = 50.0;
    const ATTACK_COOLDOWN: f64 = 1.0;
}

impl Structure for LightningTower {
    fn get_id(&self) -> usize {
        self.base.get_id()
    }

    fn get_position(&self) -> &Vector2 {
        self.base.get_position()
    }

    fn set_position(&mut self, pos: Vector2) {
        self.base.set_position(pos)
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
        if let Some(last_attack) = self.last_attack_time {
            if last_attack + self.model.attack_cooldown > time {
                return;
            }
        }
        let did_attack = enemies.iter_mut().any(|enemy| {
            if (self.get_position() - enemy.get_position()).magnitude() < self.model.attack_range {
                enemy.apply_damage(self.model.attack_damage);
                return true;
            }
            false
        });

        if did_attack {
            self.last_attack_time = Some(time);
            info!("Attacked");
        }
    }
}

impl GameStructure for LightningTower {}

impl StructureFactory for LightningTower {
    fn new(pos: Vector2) -> Self {
        let base = StructureBase::new(LightningTower::MAX_HEALTH, pos);
        LightningTower {
            base,
            model: &LIGHTNING_TOWER_MODEL,
            last_attack_time: None,
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
    #[serde(flatten)]
    frames: GifFrames,
    max_health: f64,
    gif_name: String,
    attack_range: f64,
    attack_damage: f64,
    attack_cooldown: f64,
}

impl StructureModel for LightningTowerModel {}

/****************************************
* Static
*****************************************/

lazy_static! {
    static ref LIGHTNING_TOWER_MODEL: LightningTowerModel = {
        let file = File::open("resources/www/structures/blitz_turm/blitz_turm_v2_.json")
            .expect("Could not find json file for Blitz Turm");
        let reader = BufReader::new(file);

        let frames =
            serde_json::from_reader(reader).expect("Could not parse gif frames for Blitz Turm");
        LightningTowerModel {
            frames,
            max_health: LightningTower::MAX_HEALTH,
            gif_name: LightningTower::GIF_NAME.to_string(),
            attack_range: LightningTower::ATTACK_RANGE,
            attack_damage: LightningTower::ATTACK_DAMAGE,
            attack_cooldown: LightningTower::ATTACK_COOLDOWN,
        }
    };
}
