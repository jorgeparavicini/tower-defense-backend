use crate::entity::structure::structure::StructureData;

pub struct Grunt {}

impl Grunt {
    const MAX_HEALTH: f64 = 100.0;
}

impl StructureData for Grunt {
    fn get_max_health(&self) -> f64 {
        Grunt::MAX_HEALTH
    }
}

lazy_static! {
    pub static ref GRUNT: Box<dyn StructureData + Sync + Send> = Box::new(Grunt {});
}
