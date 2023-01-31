use bevy::prelude::*;
use bodge::{
    bevy::{Cursor, DebugDraw, DebugDrawStyle},
    geometry::Circle,
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
    static ref STYLE_VERTEX_HOVER: DebugDrawStyle = DebugDrawStyle {
        color: Color::YELLOW,
        depth: 0.3,
        ..Default::default()
    };
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub enum PluginBuilderSystems {
    Update,
}

pub struct PointListBuilderPlugin;

impl Plugin for PointListBuilderPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(point_list_builder_update.label(PluginBuilderSystems::Update));
    }
}

#[derive(Component)]
pub struct PointListBuilder {
    vertices: Vec<Vec2>,
    drag_index: Option<usize>,
    hover_index: Option<usize>,
}

impl Default for PointListBuilder {
    fn default() -> PointListBuilder {
        PointListBuilder {
            vertices: vec![
                Vec2::new(0., 100.),
                Vec2::new(100., -100.),
                Vec2::new(-100., -100.),
            ],
            drag_index: None,
            hover_index: None,
        }
    }
}

impl PointListBuilder {
    pub fn vertices(&self) -> &Vec<Vec2> {
        &self.vertices
    }

    pub fn draw(&self, debug_draw: &mut DebugDraw) {
        self.draw_vertices(debug_draw);
        self.draw_hovered(debug_draw);
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

    pub fn draw_hovered(&self, debug_draw: &mut DebugDraw) {
        if let Some(hover_index) = self.hover_index {
            Circle::new(self.vertices[hover_index], 11.).draw(debug_draw, *STYLE_VERTEX_HOVER);
        }
    }
}

fn point_list_builder_update(
    mut point_list_builder_query: Query<&mut PointListBuilder>,
    cursor: Res<Cursor>,
    mouse_buttons: Res<Input<MouseButton>>,
) {
    for mut point_list_builder in point_list_builder_query.iter_mut() {
        let mut hover_index: Option<usize> = None;
        if let Some(drag_index) = point_list_builder.drag_index {
            point_list_builder.vertices[drag_index] = cursor.position;
        } else {
            for (vertex_index, vertex) in point_list_builder.vertices.iter().enumerate() {
                let distance = cursor.position.distance(*vertex);
                if distance < 30. {
                    hover_index = Some(vertex_index);
                    break;
                }
            }
        }
        if mouse_buttons.just_released(MouseButton::Left) {
            point_list_builder.drag_index = None;
        } else if mouse_buttons.just_pressed(MouseButton::Left) {
            if let Some(hover_index) = hover_index {
                point_list_builder.drag_index = Some(hover_index);
            } else {
                point_list_builder.drag_index = Some(point_list_builder.vertices.len());
                point_list_builder.vertices.push(cursor.position);
            }
            hover_index = None;
        } else if mouse_buttons.just_pressed(MouseButton::Right) {
            if let Some(hover_index) = hover_index {
                if point_list_builder.vertices.len() > 3 {
                    point_list_builder.vertices.remove(hover_index);
                }
            }
            hover_index = None;
        }
        point_list_builder.hover_index = hover_index;
    }
}
