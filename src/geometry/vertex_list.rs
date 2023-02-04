use glam::Vec2;

use crate::collision::Colliding;

use super::LineSegment2;

pub struct VertexList2(pub Vec<Vec2>);

impl VertexList2 {
    pub fn new() -> VertexList2 {
        VertexList2(Vec::new())
    }

    pub fn edges(&self) -> Vec<LineSegment2> {
        // TODO: make this an iterator instead?
        let mut edges = vec![];
        for point_index in 0..self.0.len() {
            let next_point_index = (point_index + 1) % self.0.len();
            edges.push(LineSegment2::new(
                self.0[point_index],
                self.0[next_point_index],
            ));
        }
        edges
    }

    pub fn is_simple_polygon(&self) -> bool {
        // TODO: rewrite with a sweep line algorithm
        let edges = self.edges();
        for edge_index in 0..edges.len() {
            let edge = edges[edge_index];
            for other_edge_index in
                (edge_index + 2)..(edges.len() - if edge_index == 0 { 1 } else { 0 })
            {
                let other_edge = edges[other_edge_index];
                if edge.colliding(&other_edge) {
                    return false;
                }
            }
        }
        true
    }

    pub fn is_clockwise(&self) -> bool {
        let mut area = 0.;
        for i in 0..self.0.len() {
            let j = (i + 1) % self.0.len();
            area += self.0[i].x * self.0[j].y;
            area -= self.0[j].x * self.0[i].y;
        }
        area / 2. > 0.
    }

    pub fn make_clockwise(&mut self) {
        if !self.is_clockwise() {
            let mut new_points = vec![];
            for point in self.0.iter().rev() {
                new_points.push(*point);
            }
            self.0 = new_points;
        }
    }

    pub fn make_counterclockwise(&mut self) {
        if self.is_clockwise() {
            let mut new_points = vec![];
            for point in self.0.iter().rev() {
                new_points.push(*point);
            }
            self.0 = new_points;
        }
    }
}
