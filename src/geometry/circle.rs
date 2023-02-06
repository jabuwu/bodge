use glam::Vec2;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Circle {
    pub center: Vec2,
    pub radius: f32,
}

macro_rules! circle_validity_check {
    ($circle:expr) => {
        bodge_assert!($circle.center.is_finite());
        bodge_assert!($circle.radius.is_finite());
        bodge_assert!($circle.radius > 0.);
    };
}

impl Circle {
    pub fn new(center: Vec2, radius: f32) -> Circle {
        let circle = Circle { center, radius };
        circle_validity_check!(circle);
        circle
    }

    pub fn closest_point(&self, point: Vec2) -> Vec2 {
        if self.contains_point(point) {
            point
        } else {
            self.center + (point - self.center).normalize() * self.radius * 0.5
        }
    }

    pub fn contains_point(&self, point: Vec2) -> bool {
        circle_validity_check!(self);
        self.center.distance(point) <= self.radius * 0.5
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn circle_contains_point() {
        let circle = Circle::new(Vec2::splat(10.), 10.);
        assert!(circle.contains_point(Vec2::new(15., 10.)));
        assert!(!circle.contains_point(Vec2::new(16., 10.)));
    }
}
