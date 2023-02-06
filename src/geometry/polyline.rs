use glam::Vec2;

use super::LineSegment2;

#[derive(Debug)]
pub struct Polyline(pub Vec<Vec2>);

impl Polyline {
    pub fn new() -> Polyline {
        Polyline(Vec::new())
    }

    pub fn line_segments(&self) -> Vec<LineSegment2> {
        // TODO: make this an iterator instead?
        if self.0.is_empty() {
            vec![]
        } else {
            let mut edges = vec![];
            for point_index in 0..(self.0.len() - 1) {
                let next_point_index = (point_index + 1) % self.0.len();
                edges.push(LineSegment2::new(
                    self.0[point_index],
                    self.0[next_point_index],
                ));
            }
            edges
        }
    }

    pub fn closest_point(&self, point: Vec2) -> Vec2 {
        bodge_assert!(!self.0.is_empty());
        bodge_assert!(point.is_finite());
        if self.0.len() == 1 {
            self.0[0]
        } else {
            let line_segments = self.line_segments();
            let first_closest_point = self.line_segments()[0].closest_point(point);
            let mut closest = (first_closest_point.distance(point), first_closest_point);
            for line_segment in line_segments.iter().skip(1) {
                let closest_point = line_segment.closest_point(point);
                let distance = closest_point.distance(point);
                if distance < closest.0 {
                    closest = (distance, closest_point)
                }
            }
            closest.1
        }
    }
}
