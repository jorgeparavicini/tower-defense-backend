pub trait EnemyData {
    fn get_max_health(&self) -> f64;
    fn get_damage(&self) -> u64;
    fn get_move_speed(&self) -> f64;
}

