use crate::entity::enemy::enemy::EnemyModel;
use crate::entity::enemy::enemy_type::EnemyModelMap;
use std::fs::File;
use std::io::BufReader;

const MAX_HEALTH: f64 = 60.0;
const DAMAGE: u64 = 1;
const MOVE_SPEED: f64 = 120.0;
const REWARD: usize = 40;

pub fn register_red_model(model_map: &mut EnemyModelMap) {
    model_map.insert(String::from("Red"), &*RED_MODEL);
}

lazy_static! {
    pub static ref RED_MODEL: EnemyModel = {
        let file = File::open("resources/www/enemies/red_idle.json")
            .expect("Could not find json file for red idle");
        let reader = BufReader::new(file);
        let idle_frames =
            serde_json::from_reader(reader).expect("Could not parse gif frames for red idle");

        let file = File::open("resources/www/enemies/red_dying.json")
            .expect("Could not find json file for red dying");
        let reader = BufReader::new(file);
        let dying_frames =
            serde_json::from_reader(reader).expect("Could not parse gif frames for red dying");

        EnemyModel::new(
            MAX_HEALTH,
            DAMAGE,
            MOVE_SPEED,
            REWARD,
            500.0,
            idle_frames,
            dying_frames,
            String::from("enemies/red_idle.png"),
            String::from("enemies/red_dying.png"),
        )
    };
}
