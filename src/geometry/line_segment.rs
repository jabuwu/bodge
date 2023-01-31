use glam::Vec2;

use super::Line2;

/// A 2D line segment represented by two distinct vertices.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LineSegment2 {
    pub start: Vec2,
    pub end: Vec2,
}

macro_rules! line_segment_2_validity_check {
    ($line_segment_2:expr) => {
        bodge_assert!($line_segment_2.start.is_finite());
        bodge_assert!($line_segment_2.end.is_finite());
    };
}

impl LineSegment2 {
    pub fn new(start: Vec2, end: Vec2) -> LineSegment2 {
        let line_segment = LineSegment2 { start, end };
        line_segment_2_validity_check!(line_segment);
        line_segment
    }

    pub fn center(&self) -> Vec2 {
        line_segment_2_validity_check!(self);
        (self.start + self.end) * 0.5
    }

    pub fn length(&self) -> f32 {
        line_segment_2_validity_check!(self);
        self.start.distance(self.end)
    }

    pub fn axis(&self) -> Vec2 {
        line_segment_2_validity_check!(self);
        (self.end - self.start).normalize_or_zero()
    }

    pub fn closest_point(&self, point: Vec2) -> Vec2 {
        let ab = self.end - self.start;
        let t = ((point - self.start).dot(ab) / ab.dot(ab)).clamp(0., 1.);
        self.start + t * ab
    }

    pub fn perpendicular_bisector(&self) -> Line2 {
        line_segment_2_validity_check!(self);
        Line2::new_from_point_axis(self.center(), -self.axis().perp())
    }

    pub fn is_same(&self, other: LineSegment2) -> bool {
        (self.start == other.start && self.end == other.end)
            || (self.start == other.end && self.end == other.start)
    }

    pub fn line(&self) -> Line2 {
        Line2::new_from_points(self.start, self.end)
    }
}
