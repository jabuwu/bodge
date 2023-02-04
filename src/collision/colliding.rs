use glam::Vec2;

use crate::geometry::{Circle, LineSegment2};

pub trait Colliding<T>
where
    Self: Sized,
{
    fn colliding(&self, other: &T) -> bool;
}

impl Colliding<Circle> for Circle {
    fn colliding(&self, other: &Circle) -> bool {
        let combined_radius = (self.radius + other.radius) * 0.5;
        self.center.distance_squared(other.center) < combined_radius * combined_radius
    }
}

impl Colliding<LineSegment2> for Circle {
    fn colliding(&self, other: &LineSegment2) -> bool {
        // TODO: optimize?
        other.closest_point(self.center).distance(self.center) < self.radius * 0.5
    }
}

impl Colliding<Circle> for LineSegment2 {
    fn colliding(&self, other: &Circle) -> bool {
        other.colliding(self)
    }
}

impl Colliding<LineSegment2> for LineSegment2 {
    fn colliding(&self, other: &LineSegment2) -> bool {
        fn ccw(a: Vec2, b: Vec2, c: Vec2) -> bool {
            (c.y - a.y) * (b.x - a.x) > (b.y - a.y) * (c.x - a.x)
        }
        ccw(self.start, other.start, other.end) != ccw(self.end, other.start, other.end)
            && ccw(self.start, self.end, other.start) != ccw(self.start, self.end, other.end)
    }
}
