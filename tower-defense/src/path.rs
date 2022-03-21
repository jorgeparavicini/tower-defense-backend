#[derive(Clone, Debug)]
pub struct Coords {
    x: f64,
    y: f64,
}

pub trait PathComponent {
    fn length(&self) -> f64;

    fn coords_at(&self, t: f64) -> Coords;
}

pub struct Line {
    start: Coords,
    end: Coords,
}

impl Coords {
    pub fn new(x: f64, y: f64) -> Coords {
        Coords { x, y }
    }

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) ->f64 {
        self.y
    }
}

impl PartialEq for Coords {
    fn eq(&self, other: &Self) -> bool {
        return (self.x - other.x).abs() < f64::EPSILON && (self.y - other.y).abs() < f64::EPSILON;
    }
}

impl Line {
    pub fn new(start: Coords, end: Coords) -> Line {
        Line { start, end }
    }
}

impl PathComponent for Line {
    fn length(&self) -> f64 {
        let v = Coords {
            x: self.end.x - self.start.x,
            y: self.end.y - self.start.y,
        };
        (v.x.powi(2) + v.y.powi(2)).sqrt()
    }

    fn coords_at(&self, t: f64) -> Coords {
        return Coords {
            x: self.start.x + (self.end.x - self.start.x) * t,
            y: self.start.y + (self.end.y - self.start.y) * t,
        };
    }
}

pub struct Path {
    path: Vec<Box<dyn PathComponent + Send + Sync>>,
    length: f64,
}

impl Path {
    pub fn new(path: Vec<Box<dyn PathComponent + Send + Sync>>) -> Path {
        let length = path.iter().map(|x| x.length()).sum();
        Path {
            path,
            length,
        }
    }

    pub fn length(&self) -> f64 {
        self.length
    }

    pub fn coords_at(&self, t: f64) -> Coords {
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
        unreachable!("The loop should always return");
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
    use crate::path::{Coords, Line, Path};

    #[test]
    fn coords_at() {
        let path = Path::new(vec![
            Box::new(Line::new(Coords::new(0.0, 10.0), Coords::new(10.0, 10.0))),
            Box::new(Line::new(Coords::new(10.0, 10.0), Coords::new(10.0, 0.0))),
        ]);

        assert_eq!(path.coords_at(0.0), Coords::new(0.0, 10.0));
        println!("{:#?}", path.coords_at(0.5));
    }
}