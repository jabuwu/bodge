use glam::Vec2;

// ax + by = c
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Line2 {
    pub a: f32,
    pub b: f32,
    pub c: f32,
}

macro_rules! line_2_validity_check {
    ($line_2:expr) => {
        bodge_assert!($line_2.a != 0. || $line_2.b != 0.);
    };
}

impl Line2 {
    pub fn new(a: f32, b: f32, c: f32) -> Line2 {
        let line = Line2 { a, b, c };
        line_2_validity_check!(line);
        line
    }

    pub fn new_from_points(point1: Vec2, point2: Vec2) -> Line2 {
        let a = point2.y - point1.y;
        let b = point1.x - point2.x;
        let c = a * point1.x + b * point1.y;
        let line = Line2 { a, b, c };
        line_2_validity_check!(line);
        line
    }

    pub fn new_from_point_axis(point: Vec2, axis: Vec2) -> Line2 {
        bodge_assert!(axis.length_squared() > 0.);
        Line2::new_from_points(point, point + axis)
    }

    pub fn x(&self, y: f32) -> Option<f32> {
        line_2_validity_check!(self);
        if self.a != 0.0 {
            Some((self.c - self.b * y) / self.a)
        } else {
            None
        }
    }

    pub fn y(&self, x: f32) -> Option<f32> {
        line_2_validity_check!(self);
        if self.b != 0.0 {
            Some((self.c - self.a * x) / self.b)
        } else {
            None
        }
    }

    pub fn axis(&self) -> Vec2 {
        line_2_validity_check!(self);
        Vec2::new(-self.b, self.a).normalize()
    }

    pub fn closest_point(&self, point: Vec2) -> Vec2 {
        line_2_validity_check!(self);
        // TODO: what happens if they are the same line?
        let a = self.b;
        let b = -self.a;
        let c = a * point.x + b * point.y;
        self.intersection_point(Line2 { a, b, c }).unwrap()
    }

    pub fn intersection_point(&self, other: Line2) -> Option<Vec2> {
        line_2_validity_check!(self);
        let determinant = self.a * other.b - other.a * self.b;
        if determinant != 0. {
            Some(Vec2::new(
                (other.b * self.c - self.b * other.c) / determinant,
                (self.a * other.c - other.a * self.c) / determinant,
            ))
        } else {
            None
        }
    }
}
