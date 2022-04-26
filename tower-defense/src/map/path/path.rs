use crate::math::Vector2;

pub trait PathComponent {
    fn length(&self) -> f64;

    fn coords_at(&self, t: f64) -> Vector2;

    fn start(&self) -> Vector2;

    fn end(&self) -> Vector2;
}

pub struct Line {
    start: Vector2,
    end: Vector2,
}

impl Line {
    pub fn new(start: Vector2, end: Vector2) -> Line {
        Line { start, end }
    }
}

impl PathComponent for Line {
    fn length(&self) -> f64 {
        let v = Vector2::new(self.end.x() - self.start.x(), self.end.y() - self.start.y());
        (v.x().powi(2) + v.y().powi(2)).sqrt()
    }

    fn coords_at(&self, t: f64) -> Vector2 {
        return Vector2::new(
            self.start.x() + (self.end.x() - self.start.x()) * t,
            self.start.y() + (self.end.y() - self.start.y()) * t,
        );
    }

    fn start(&self) -> Vector2 {
        self.start.clone()
    }

    fn end(&self) -> Vector2 {
        self.end.clone()
    }
}

pub struct Path {
    path: Vec<Box<dyn PathComponent + Send + Sync>>,
    end: Vector2,
    length: f64,
}

impl Path {
    pub fn new(path: Vec<Box<dyn PathComponent + Send + Sync>>) -> Path {
        let length = path.iter().map(|x| x.length()).sum();
        let end = path.last().unwrap().end();
        Path { path, end, length }
    }

    pub fn length(&self) -> f64 {
        self.length
    }

    pub fn coords_at_clamped(&self, t: f64) -> Vector2 {
        let t = clamp(t);
        let mut accumulated_t = 0.0;
        for i in 0..self.path.len() {
            let l = self.path[i].length() / self.length;
            let new_t = accumulated_t + l;
            if new_t >= t {
                let x = (t - accumulated_t) / l;
                return self.path[i].coords_at(x);
            }
            accumulated_t = new_t;
        }
        self.end.clone()
    }

    pub fn coords_at(&self, t: f64) -> Vector2 {
        self.coords_at_clamped(t / self.length)
    }
}

fn clamp(t: f64) -> f64 {
    if t > 1.0 {
        return 1.0;
    }
    if t < 0.0 {
        return 0.0;
    }
    t
}

#[cfg(test)]
mod path_tests {
    use crate::map::path::{Line, Path};
    use crate::math::Vector2;

    #[test]
    fn coords_at() {
        let path = Path::new(vec![
            Box::new(Line::new(
                Vector2::new(0.0, 200.0),
                Vector2::new(100.0, 200.0),
            )),
            Box::new(Line::new(
                Vector2::new(100.0, 200.0),
                Vector2::new(100.0, 100.0),
            )),
            Box::new(Line::new(
                Vector2::new(100.0, 100.0),
                Vector2::new(300.0, 100.0),
            )),
        ]);

        //assert_eq!(path.coords_at(0.0), Coords::new(0.0, 10.0));
        println!("{:#?}", path.coords_at(0.1));
        println!("{:#?}", path.coords_at(0.2));
        println!("{:#?}", path.coords_at(0.3));
        println!("{:#?}", path.coords_at(0.4));
        println!("{:#?}", path.coords_at(0.5));
        println!("{:#?}", path.coords_at(0.6));
        println!("{:#?}", path.coords_at(0.7));
        println!("{:#?}", path.coords_at(0.8));
        println!("{:#?}", path.coords_at(0.9));
        println!("{:#?}", path.coords_at(1.0));
    }
}
