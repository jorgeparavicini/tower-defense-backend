use crate::math::vector2::Vector2;

pub struct Rect {
    a: Vector2,
    b: Vector2,
    ab: Vector2,
    bc: Vector2,
}

impl Rect {
    pub fn new(a: Vector2, b: Vector2, c: Vector2) -> Self {
        let ab = &b - &a;
        let bc = &c - &a;
        Self { a, b, ab, bc }
    }

    pub fn is_inside(&self, m: &Vector2) -> bool {
        let am = m - &self.a;
        let bm = m - &self.b;
        let dot_ab_am = self.ab.dot(&am);
        let dot_ab_ab = self.ab.dot(&self.ab);
        let dot_bc_bm = self.bc.dot(&bm);
        let dot_bc_bc = self.bc.dot(&self.bc);

        0.0 <= dot_ab_am && dot_ab_am <= dot_ab_ab && 0.0 <= dot_bc_bm && dot_bc_bm <= dot_bc_bc
    }
}

#[cfg(test)]
mod rect_tests {
    use crate::math::Rect;
    use crate::math::Vector2;

    #[test]
    fn is_inside() {
        let rect = Rect::new(
            Vector2::new(0.0, 0.0),
            Vector2::new(15.0, 0.0),
            Vector2::new(0.0, 15.0),
        );

        assert!(rect.is_inside(&Vector2::new(7.0, 6.0)));
        assert!(rect.is_inside(&Vector2::new(0.0, 6.0)));
        assert!(rect.is_inside(&Vector2::new(7.0, 0.0)));
        assert!(rect.is_inside(&Vector2::new(15.0, 15.0)));
        assert!(rect.is_inside(&Vector2::new(0.0, 0.0)));
        assert!(rect.is_inside(&Vector2::new(15.0, 0.0)));
        assert!(rect.is_inside(&Vector2::new(0.0, 15.0)));
        assert!(!rect.is_inside(&Vector2::new(-1.0, 0.0)));
        assert!(!rect.is_inside(&Vector2::new(0.0, -1.0)));
        assert!(!rect.is_inside(&Vector2::new(16.0, 0.0)));
        assert!(!rect.is_inside(&Vector2::new(0.0, 16.0)));
    }
}
