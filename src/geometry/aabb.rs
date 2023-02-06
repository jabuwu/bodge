use glam::Vec2;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Aabb {
    pub position: Vec2,
    pub size: Vec2,
}

macro_rules! aabb_validity_check {
    ($aabb:expr) => {
        bodge_assert!($aabb.position.is_finite());
        bodge_assert!($aabb.size.is_finite());
        bodge_assert!($aabb.size.x >= 0.);
        bodge_assert!($aabb.size.y >= 0.);
    };
}

impl Aabb {
    pub fn new(position: Vec2, size: Vec2) -> Aabb {
        let aabb = Aabb { position, size };
        aabb_validity_check!(aabb);
        aabb
    }

    pub fn closest_point(&self, point: Vec2) -> Vec2 {
        aabb_validity_check!(self);
        point.clamp(
            self.position - self.size * 0.5,
            self.position + self.size * 0.5,
        )
    }

    pub fn contains_point(&self, point: Vec2) -> bool {
        aabb_validity_check!(self);
        point.x > self.position.x - self.size.x * 0.5
            && point.x < self.position.x + self.size.x * 0.5
            && point.y > self.position.y - self.size.y * 0.5
            && point.y < self.position.y + self.size.y * 0.5
    }
}
