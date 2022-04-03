use crate::entity::enemy::enemy::EnemyData;
// TODO: Implement Serializer https://serde.rs/impl-serialize.html, because the structs have no fields

pub struct Recruit {}

impl Recruit {
    const MAX_HEALTH: f64 = 100.0;
    const DAMAGE: u64 = 1;
    const MOVE_SPEED: f64 = 50.0;
}

impl EnemyData for Recruit {
    fn get_max_health(&self) -> f64 {
        Recruit::MAX_HEALTH
    }

    fn get_damage(&self) -> u64 {
        Recruit::DAMAGE
    }

    fn get_move_speed(&self) -> f64 {
        Recruit::MOVE_SPEED
    }
}

lazy_static! {
    pub static ref RECRUIT: Box<dyn EnemyData + Sync + Send> = Box::new(Recruit {});
}
