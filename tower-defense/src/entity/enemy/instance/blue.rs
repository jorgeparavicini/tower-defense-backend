use crate::entity::enemy::enemy::EnemyModel;
use crate::entity::enemy::enemy_type::EnemyModelMap;
use std::fs::File;
use std::io::BufReader;

const MAX_HEALTH: f64 = 80.0;
const DAMAGE: u64 = 1;
const MOVE_SPEED: f64 = 100.0;
const REWARD: usize = 100;

pub fn register_blue_model(model_map: &mut EnemyModelMap) {
    model_map.insert(String::from("Blue"), &*BLUE_MODEL);
}

lazy_static! {
    pub static ref BLUE_MODEL: EnemyModel = {
        let file = File::open("resources/www/enemies/blue_idle.json")
            .expect("Could not find json file for blue idle");
        let reader = BufReader::new(file);
        let idle_frames =
            serde_json::from_reader(reader).expect("Could not parse gif frames for blue idle");

        let file = File::open("resources/www/enemies/blue_dying.json")
            .expect("Could not find json file for blue dying");
        let reader = BufReader::new(file);
        let dying_frames =
            serde_json::from_reader(reader).expect("Could not parse gif frames for blue dying");

        EnemyModel::new(
            MAX_HEALTH,
            DAMAGE,
            MOVE_SPEED,
            REWARD,
            500.0,
            idle_frames,
            dying_frames,
            String::from("enemies/blue_idle.png"),
            String::from("enemies/blue_dying.png"),
        )
    };
}
