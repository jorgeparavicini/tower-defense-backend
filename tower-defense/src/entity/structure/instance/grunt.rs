use crate::entity::gif::GifFrames;
use crate::entity::structure::structure::{
    RegisterStructureModel, StructureBase, StructureFactory, StructureModel, StructureModelMap,
};
use crate::entity::Structure;
use crate::math::Vector2;
use serde::{Serialize, Serializer};
use std::fs::File;
use std::io::BufReader;

/****************************************
* Grunt
*****************************************/

#[derive(Serialize)]
pub struct Grunt {
    #[serde(flatten)]
    base: StructureBase,
    #[serde(serialize_with = "model_serialize")]
    model: &'static GruntModel,
    last_attack_time: Option<f64>,
}

impl Grunt {
    const MAX_HEALTH: f64 = 100.0;
    const GIF_NAME: &'static str = "structures/blitz_turm/blitz_turm_v2_.png";
}

impl Structure for Grunt {
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

impl StructureFactory for Grunt {
    fn new(pos: Vector2) -> Self {
        let base = StructureBase::new(Grunt::MAX_HEALTH, pos);
        Grunt {
            base,
            model: &GRUNT_MODEL,
            last_attack_time: None,
        }
    }
}

impl RegisterStructureModel for Grunt {
    fn register_model(model_map: &mut StructureModelMap) {
        model_map.insert(
            String::from("Grunt"),
            Box::new((*GRUNT_MODEL).clone()) as Box<dyn StructureModel + 'static>,
        );
    }
}

fn model_serialize<S>(x: &GruntModel, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    s.serialize_str("Grunt")
}

/****************************************
* Grunt Model
*****************************************/

#[derive(Serialize, Clone)]
pub struct GruntModel {
    #[serde(flatten)]
    frames: GifFrames,
    max_health: f64,
    gif_name: String,
}

impl StructureModel for GruntModel {}

/****************************************
* Static
*****************************************/

lazy_static! {
    static ref GRUNT_MODEL: GruntModel = {
        let file = File::open("resources/www/structures/blitz_turm/blitz_turm_v2_.json")
            .expect("Could not find json file for Blitz Turm");
        let reader = BufReader::new(file);

        let frames =
            serde_json::from_reader(reader).expect("Could not parse gif frames for Blitz Turm");
        GruntModel {
            frames,
            max_health: Grunt::MAX_HEALTH,
            gif_name: Grunt::GIF_NAME.to_string(),
        }
    };
}
