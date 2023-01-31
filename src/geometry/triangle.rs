use glam::Vec2;

use super::{Circle, Line2, LineSegment2};

/// A 2D triangle represented by three distinct vertices.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Triangle2 {
    pub vertices: [Vec2; 3],
}

macro_rules! triangle_2_validity_check {
    ($triangle_2:expr) => {
        bodge_assert!($triangle_2.vertices[0].is_finite());
        bodge_assert!($triangle_2.vertices[1].is_finite());
        bodge_assert!($triangle_2.vertices[2].is_finite());
    };
}

impl Triangle2 {
    pub fn new(a: Vec2, b: Vec2, c: Vec2) -> Triangle2 {
        let triangle = Triangle2 {
            vertices: [a, b, c],
        };
        triangle_2_validity_check!(triangle);
        triangle
    }

    pub fn a(&self) -> Vec2 {
        triangle_2_validity_check!(self);
        self.vertices[0]
    }

    pub fn set_a(&mut self, a: Vec2) {
        self.vertices[0] = a;
        triangle_2_validity_check!(self);
    }

    pub fn b(&self) -> Vec2 {
        triangle_2_validity_check!(self);
        self.vertices[1]
    }

    pub fn set_b(&mut self, b: Vec2) {
        self.vertices[1] = b;
        triangle_2_validity_check!(self);
    }

    pub fn c(&self) -> Vec2 {
        triangle_2_validity_check!(self);
        self.vertices[2]
    }

    pub fn set_c(&mut self, c: Vec2) {
        self.vertices[2] = c;
        triangle_2_validity_check!(self);
    }

    pub fn a_angle(&self) -> f32 {
        (self.c() - self.a())
            .angle_between(self.b() - self.a())
            .abs()
    }

    pub fn b_angle(&self) -> f32 {
        (self.c() - self.b())
            .angle_between(self.a() - self.b())
            .abs()
    }

    pub fn c_angle(&self) -> f32 {
        (self.a() - self.c())
            .angle_between(self.b() - self.c())
            .abs()
    }

    pub fn a_angle_bisector(&self) -> Line2 {
        Line2::new_from_point_axis(
            self.a(),
            Vec2::from_angle(
                (Vec2::X.angle_between(self.b() - self.a())
                    + Vec2::X.angle_between(self.c() - self.a()))
                    * 0.5,
            ),
        )
    }

    pub fn b_angle_bisector(&self) -> Line2 {
        Line2::new_from_point_axis(
            self.b(),
            Vec2::from_angle(
                (Vec2::X.angle_between(self.c() - self.b())
                    + Vec2::X.angle_between(self.a() - self.b()))
                    * 0.5,
            ),
        )
    }

    pub fn c_angle_bisector(&self) -> Line2 {
        Line2::new_from_point_axis(
            self.c(),
            Vec2::from_angle(
                (Vec2::X.angle_between(self.b() - self.c())
                    + Vec2::X.angle_between(self.a() - self.c()))
                    * 0.5,
            ),
        )
    }

    pub fn edges(&self) -> [LineSegment2; 3] {
        triangle_2_validity_check!(self);
        [self.ab(), self.bc(), self.ca()]
    }

    pub fn ab(&self) -> LineSegment2 {
        triangle_2_validity_check!(self);
        LineSegment2::new(self.a(), self.b())
    }

    pub fn bc(&self) -> LineSegment2 {
        triangle_2_validity_check!(self);
        LineSegment2::new(self.b(), self.c())
    }

    pub fn ca(&self) -> LineSegment2 {
        triangle_2_validity_check!(self);
        LineSegment2::new(self.c(), self.a())
    }

    pub fn circumcenter(&self) -> Option<Vec2> {
        triangle_2_validity_check!(self);
        let bisector1 = self.ab().perpendicular_bisector();
        let bisector2 = self.bc().perpendicular_bisector();
        bisector1.intersection_point(bisector2)
    }

    pub fn circumcircle(&self) -> Option<Circle> {
        triangle_2_validity_check!(self);
        let circumcenter = self.circumcenter()?;
        Some(Circle::new(
            circumcenter,
            circumcenter.distance(self.a()) * 2.,
        ))
    }

    pub fn centroid(&self) -> Option<Vec2> {
        triangle_2_validity_check!(self);
        let line1 = Line2::new_from_points(self.a(), self.bc().center());
        let line2 = Line2::new_from_points(self.b(), self.ca().center());
        line1.intersection_point(line2)
    }

    pub fn scale(&mut self, scale: f32) {
        if let Some(centroid) = self.centroid() {
            self.vertices[0] = centroid + (self.vertices[0] - centroid) * scale;
            self.vertices[1] = centroid + (self.vertices[1] - centroid) * scale;
            self.vertices[2] = centroid + (self.vertices[2] - centroid) * scale;
        }
    }

    pub fn scaled(&self, scale: f32) -> Triangle2 {
        let mut triangle = *self;
        triangle.scale(scale);
        triangle
    }

    pub fn contains_point(&self, point: Vec2) -> bool {
        fn sign(p1: Vec2, p2: Vec2, p3: Vec2) -> f32 {
            (p1.x - p3.x) * (p2.y - p3.y) - (p2.x - p3.x) * (p1.y - p3.y)
        }
        let d1 = sign(point, self.a(), self.b());
        let d2 = sign(point, self.b(), self.c());
        let d3 = sign(point, self.c(), self.a());
        let has_neg = (d1 < 0.) || (d2 < 0.) || (d3 < 0.);
        let has_pos = (d1 > 0.) || (d2 > 0.) || (d3 > 0.);
        !(has_neg && has_pos)
    }
}
