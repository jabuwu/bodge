use glam::Vec2;

use super::{Line2, LineSegment2};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LineRay2 {
    pub start: Vec2,
    pub axis: Vec2,
}

macro_rules! line_ray_2_validity_check {
    ($line_ray_2:expr) => {
        bodge_assert!($line_ray_2.start.is_finite());
        bodge_assert!($line_ray_2.axis.is_finite());
        bodge_assert!($line_ray_2.axis.is_normalized());
    };
}

impl LineRay2 {
    pub fn new(start: Vec2, axis: Vec2) -> LineRay2 {
        let line_ray = LineRay2 {
            start,
            axis: axis.normalize_or_zero(),
        };
        line_ray_2_validity_check!(line_ray);
        line_ray
    }

    pub fn closest_point(&self, point: Vec2) -> Vec2 {
        line_ray_2_validity_check!(self);
        self.start + (point - self.start).dot(self.axis).max(0.) * self.axis
    }

    pub fn segment(&self, length: f32) -> LineSegment2 {
        line_ray_2_validity_check!(self);
        LineSegment2::new(self.start, self.start + self.axis * length)
    }

    pub fn line(&self) -> Line2 {
        line_ray_2_validity_check!(self);
        Line2::new_from_point_axis(self.start, self.axis)
    }
}
