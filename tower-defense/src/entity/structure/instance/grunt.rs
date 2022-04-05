use crate::entity::gif::GifFrames;
use crate::entity::structure::structure::StructureData;
use serde::Serialize;
use std::fs::File;
use std::io::BufReader;

#[derive(Serialize)]
pub struct Grunt {
    #[serde(flatten)]
    frames: GifFrames,
    max_health: f64,
    gif_name: String,
}

impl Grunt {
    const MAX_HEALTH: f64 = 100.0;
    const GIF_NAME: &'static str = "structures/blitz_turm/blitz_turm_v2_.png";
}

impl StructureData for Grunt {
    fn get_max_health(&self) -> f64 {
        self.max_health
    }

    fn get_gif_data(&self) -> &GifFrames {
        &self.frames
    }

    fn get_gif(&self) -> &str {
        &self.gif_name
    }
}

serialize_trait_object!(StructureData);

lazy_static! {
    pub static ref GRUNT: Box<dyn StructureData + Sync + Send> = {
        let file = File::open("resources/www/structures/blitz_turm/blitz_turm_v2_.json")
            .expect("Could not find json file for Blitz Turm");
        let reader = BufReader::new(file);

        let frames =
            serde_json::from_reader(reader).expect("Could not parse gif frames for Blitz Turm");
        Box::new(Grunt {
            frames,
            max_health: Grunt::MAX_HEALTH,
            gif_name: Grunt::GIF_NAME.to_string(),
        })
    };
}
