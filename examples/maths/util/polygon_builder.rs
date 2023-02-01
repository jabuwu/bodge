use bevy::prelude::*;
use bodge::{
    bevy::{Cursor, DebugDraw, DebugDrawStyle},
    geometry::{Circle, LineSegment2},
    prelude::*,
};
use lazy_static::lazy_static;

lazy_static! {
    static ref STYLE_VERTEX: DebugDrawStyle = DebugDrawStyle {
        color: Color::WHITE,
        depth: 0.2,
        ..Default::default()
    };
    static ref STYLE_VERTEX_DRAGGING: DebugDrawStyle = DebugDrawStyle {
        color: Color::YELLOW,
        depth: 0.2,
        ..Default::default()
    };
    static ref STYLE_VERTEX_CLOSEST: DebugDrawStyle = DebugDrawStyle {
        color: Color::YELLOW,
        depth: 0.3,
        ..Default::default()
    };
    static ref STYLE_EDGE: DebugDrawStyle = DebugDrawStyle {
        color: Color::GRAY,
        thickness: 2.,
        depth: 0.1,
        ..Default::default()
    };
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub enum PluginBuilderSystems {
    Update,
}

pub struct PolygonBuilderPlugin;

impl Plugin for PolygonBuilderPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(polygon_builder_update.label(PluginBuilderSystems::Update));
    }
}

#[derive(Clone, Copy)]
struct Closest {
    distance: f32,
    kind: ClosestKind,
}

#[derive(Clone, Copy)]
enum ClosestKind {
    Vertex { vertex_index: usize },
    Edge { edge_index: usize, position: Vec2 },
}

#[derive(Component)]
pub struct PolygonBuilder {
    vertices: Vec<Vec2>,
    drag_index: Option<usize>,
    closest: Option<Closest>,
}

impl Default for PolygonBuilder {
    fn default() -> PolygonBuilder {
        PolygonBuilder {
            vertices: vec![
                Vec2::new(0., 100.),
                Vec2::new(100., -100.),
                Vec2::new(-100., -100.),
            ],
            drag_index: None,
            closest: None,
        }
    }
}

impl PolygonBuilder {
    pub fn vertices(&self) -> &Vec<Vec2> {
        &self.vertices
    }

    pub fn vertices_mut(&mut self) -> &mut Vec<Vec2> {
        &mut self.vertices
    }

    pub fn draw(&self, debug_draw: &mut DebugDraw) {
        self.draw_vertices(debug_draw);
        self.draw_edges(debug_draw);
        self.draw_closest(debug_draw);
    }

    pub fn draw_vertices(&self, debug_draw: &mut DebugDraw) {
        for (vertex_index, vertex) in self.vertices().iter().enumerate() {
            let dragging = if let Some(drag_index) = self.drag_index {
                vertex_index == drag_index
            } else {
                false
            };
            Circle::new(*vertex, 10.).draw(
                debug_draw,
                if dragging {
                    *STYLE_VERTEX_DRAGGING
                } else {
                    *STYLE_VERTEX
                },
            );
        }
    }

    pub fn draw_edges(&self, debug_draw: &mut DebugDraw) {
        for vertex_index in 0..self.vertices().len() {
            let next_vertex_index = (vertex_index + 1) % self.vertices.len();
            LineSegment2::new(
                self.vertices()[vertex_index],
                self.vertices()[next_vertex_index],
            )
            .draw(debug_draw, *STYLE_EDGE);
        }
    }

    pub fn draw_closest(&self, debug_draw: &mut DebugDraw) {
        if let Some(closest) = self.closest {
            match closest.kind {
                ClosestKind::Vertex { vertex_index } => {
                    Circle::new(self.vertices[vertex_index], 11.)
                        .draw(debug_draw, *STYLE_VERTEX_CLOSEST);
                }
                ClosestKind::Edge { position, .. } => {
                    Circle::new(position, 11.).draw(debug_draw, *STYLE_VERTEX_CLOSEST);
                }
            }
        }
    }
}

fn polygon_builder_update(
    mut polygon_builder_query: Query<&mut PolygonBuilder>,
    cursor: Res<Cursor>,
    mouse_buttons: Res<Input<MouseButton>>,
) {
    for mut polygon_builder in polygon_builder_query.iter_mut() {
        let mut closest: Option<Closest> = None;
        if let Some(drag_index) = polygon_builder.drag_index {
            polygon_builder.vertices[drag_index] = cursor.position;
        } else {
            for (vertex_index, vertex) in polygon_builder.vertices.iter().enumerate() {
                let distance = cursor.position.distance(*vertex);
                if distance < 30. {
                    let closest_candidate = Closest {
                        distance,
                        kind: ClosestKind::Vertex { vertex_index },
                    };
                    if let Some(current_closest) = closest {
                        if closest_candidate.distance < current_closest.distance {
                            closest = Some(closest_candidate);
                        }
                    } else {
                        closest = Some(closest_candidate);
                    }
                }
            }
            if closest.is_none() {
                for vertex_index in 0..polygon_builder.vertices.len() {
                    let next_vertex_index = (vertex_index + 1) % polygon_builder.vertices.len();
                    let edge = LineSegment2::new(
                        polygon_builder.vertices[vertex_index],
                        polygon_builder.vertices[next_vertex_index],
                    );
                    let closest_point = edge.closest_point(cursor.position);
                    let distance = cursor.position.distance(closest_point);
                    if distance < 30. {
                        let closest_candidate = Closest {
                            distance,
                            kind: ClosestKind::Edge {
                                edge_index: next_vertex_index,
                                position: closest_point,
                            },
                        };
                        if let Some(current_closest) = closest {
                            if closest_candidate.distance < current_closest.distance {
                                closest = Some(closest_candidate);
                            }
                        } else {
                            closest = Some(closest_candidate);
                        }
                    }
                }
            }
        }
        if mouse_buttons.just_released(MouseButton::Left) {
            polygon_builder.drag_index = None;
        } else if mouse_buttons.just_pressed(MouseButton::Left) {
            if let Some(closest) = closest {
                match closest.kind {
                    ClosestKind::Vertex { vertex_index } => {
                        polygon_builder.drag_index = Some(vertex_index);
                    }
                    ClosestKind::Edge { edge_index, .. } => {
                        polygon_builder.vertices.insert(edge_index, cursor.position);
                        polygon_builder.drag_index = Some(edge_index);
                    }
                }
            }
            closest = None;
        } else if mouse_buttons.just_pressed(MouseButton::Right) {
            if let Some(closest) = closest {
                match closest.kind {
                    ClosestKind::Vertex { vertex_index } => {
                        if polygon_builder.vertices.len() > 3 {
                            polygon_builder.vertices.remove(vertex_index);
                        }
                    }
                    ClosestKind::Edge { .. } => {}
                }
            }
            closest = None;
        }
        polygon_builder.closest = closest;
    }
}
