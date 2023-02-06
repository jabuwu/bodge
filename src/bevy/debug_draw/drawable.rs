use std::f32::consts::TAU;

use bevy::prelude::*;

use crate::geometry::{Aabb, Circle, Line2, LineRay2, LineSegment2, Polyline, Triangle2};

use super::{draw::DebugDrawMesh, DebugDraw, DebugDrawStyle, DebugDrawVertex};

const INFINITE_LENGTH: f32 = 10000.;

pub trait DebugDrawable {
    fn draw(&self, debug_draw: &mut DebugDraw, style: DebugDrawStyle) {
        debug_draw.draw_mesh(self.to_mesh(style));
    }

    fn to_mesh(&self, style: DebugDrawStyle) -> DebugDrawMesh;
}

impl DebugDrawable for Aabb {
    fn to_mesh(&self, style: DebugDrawStyle) -> DebugDrawMesh {
        if !style.visible {
            return DebugDrawMesh::new();
        }
        DebugDrawMesh {
            vertices: vec![
                DebugDrawVertex {
                    position: self.position + self.size * Vec2::new(0.5, 0.5),
                    color: style.color,
                },
                DebugDrawVertex {
                    position: self.position + self.size * Vec2::new(-0.5, 0.5),
                    color: style.color,
                },
                DebugDrawVertex {
                    position: self.position + self.size * Vec2::new(0.5, -0.5),
                    color: style.color,
                },
                DebugDrawVertex {
                    position: self.position + self.size * Vec2::new(-0.5, -0.5),
                    color: style.color,
                },
            ],
            indices: vec![0, 1, 2, 3, 2, 1],
            depth: style.depth,
        }
    }
}

impl DebugDrawable for Circle {
    fn to_mesh(&self, style: DebugDrawStyle) -> DebugDrawMesh {
        if !style.visible {
            return DebugDrawMesh::new();
        }
        let mut vertices = vec![];
        let mut indices = vec![];
        for segment in 0..style.segments {
            let angle = segment as f32 / style.segments as f32 * TAU;
            vertices.push(DebugDrawVertex {
                position: self.center + Vec2::from_angle(angle) * self.radius * 0.5,
                color: style.color,
            });
            indices.push(style.segments as u32);
            indices.push(segment as u32);
            indices.push((segment as u32 + 1) % style.segments as u32);
        }
        vertices.push(DebugDrawVertex {
            position: self.center,
            color: style.color,
        });
        DebugDrawMesh {
            vertices,
            indices,
            depth: style.depth,
        }
    }
}

impl DebugDrawable for Line2 {
    fn to_mesh(&self, style: DebugDrawStyle) -> DebugDrawMesh {
        if !style.visible {
            return DebugDrawMesh::new();
        }
        let closest_to_zero = self.closest_point(Vec2::ZERO);
        let axis = self.axis();
        LineSegment2::new(
            closest_to_zero - axis * INFINITE_LENGTH,
            closest_to_zero + axis * INFINITE_LENGTH,
        )
        .to_mesh(style)
    }
}

impl DebugDrawable for LineRay2 {
    fn to_mesh(&self, style: DebugDrawStyle) -> DebugDrawMesh {
        if !style.visible {
            return DebugDrawMesh::new();
        }
        self.segment(INFINITE_LENGTH).to_mesh(style)
    }
}

impl DebugDrawable for LineSegment2 {
    fn to_mesh(&self, style: DebugDrawStyle) -> DebugDrawMesh {
        if self.start == self.end || !style.visible {
            DebugDrawMesh::new()
        } else {
            let orthogonal = (self.start - self.end).normalize().perp() * style.thickness * 0.5;
            DebugDrawMesh {
                vertices: vec![
                    DebugDrawVertex {
                        position: self.start - orthogonal,
                        color: style.color,
                    },
                    DebugDrawVertex {
                        position: self.start + orthogonal,
                        color: style.color,
                    },
                    DebugDrawVertex {
                        position: self.end - orthogonal,
                        color: style.color,
                    },
                    DebugDrawVertex {
                        position: self.end + orthogonal,
                        color: style.color,
                    },
                ],
                indices: vec![0, 1, 2, 3, 2, 1],
                depth: style.depth,
            }
        }
    }
}

impl DebugDrawable for Triangle2 {
    fn to_mesh(&self, style: DebugDrawStyle) -> DebugDrawMesh {
        if !style.visible {
            return DebugDrawMesh::new();
        }
        if style.outline {
            let mut lines = DebugDrawMesh::new();
            lines.depth = style.depth;
            for edge in self.edges().iter() {
                lines.merge_with(&edge.to_mesh(style));
            }
            lines
        } else {
            let a = self.vertices[0];
            let b = self.vertices[1];
            let c = self.vertices[2];
            let clockwise = b.x * a.y + c.x * b.y + a.x * c.y > a.x * b.y + b.x * c.y + c.x * a.y;
            let vertices = vec![
                DebugDrawVertex {
                    position: a,
                    color: style.color,
                },
                DebugDrawVertex {
                    position: b,
                    color: style.color,
                },
                DebugDrawVertex {
                    position: c,
                    color: style.color,
                },
            ];
            let indices = if clockwise {
                vec![0, 2, 1]
            } else {
                vec![0, 1, 2]
            };
            DebugDrawMesh {
                vertices,
                indices,
                depth: style.depth,
            }
        }
    }
}

impl DebugDrawable for Polyline {
    fn to_mesh(&self, style: DebugDrawStyle) -> DebugDrawMesh {
        if !style.visible {
            return DebugDrawMesh::new();
        }
        let mut lines = DebugDrawMesh::new();
        lines.depth = style.depth;
        for line_segment in self.line_segments().iter() {
            lines.merge_with(&line_segment.to_mesh(style));
        }
        lines
    }
}
