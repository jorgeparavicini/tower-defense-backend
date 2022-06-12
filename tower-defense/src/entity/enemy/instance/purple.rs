use crate::entity::enemy::enemy::EnemyModel;
use crate::entity::enemy::enemy_type::EnemyModelMap;
use std::fs::File;
use std::io::BufReader;

const MAX_HEALTH: f64 = 150.0;
const DAMAGE: u64 = 2;
const MOVE_SPEED: f64 = 20.0;
const REWARD: usize = 120;

pub fn register_purple_model(model_map: &mut EnemyModelMap) {
    model_map.insert(String::from("Purple"), &*PURPLE_MODEL);
}

lazy_static! {
    pub static ref PURPLE_MODEL: EnemyModel = {
        let file = File::open("resources/www/enemies/purple_idle.json")
            .expect("Could not find json file for purple idle");
        let reader = BufReader::new(file);
        let idle_frames =
            serde_json::from_reader(reader).expect("Could not parse gif frames for purple idle");

        let file = File::open("resources/www/enemies/purple_dying.json")
            .expect("Could not find json file for purple dying");
        let reader = BufReader::new(file);
        let dying_frames =
            serde_json::from_reader(reader).expect("Could not parse gif frames for purple dying");

        EnemyModel::new(
            MAX_HEALTH,
            DAMAGE,
            MOVE_SPEED,
            REWARD,
            500.0,
            idle_frames,
            dying_frames,
            String::from("enemies/purple_idle.png"),
            String::from("enemies/purple_dying.png"),
        )
    };
}
