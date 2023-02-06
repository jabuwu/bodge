use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};
use bodge::{
    bevy::{DebugDraw, DebugDrawStyle, Label},
    geometry::{Circle, Triangle2, VertexList2},
    prelude::*,
};
use lazy_static::lazy_static;

use crate::{
    util::{PolygonBuilder, Stepper, StepperConfig},
    AppScene,
};

const SCENE: AppScene = AppScene::TriangulationEarClipping;

lazy_static! {
    static ref STYLE_POLYGON_BAD: DebugDrawStyle = DebugDrawStyle {
        color: Color::RED,
        thickness: 3.,
        outline: true,
        depth: 2.,
        ..Default::default()
    };
    static ref STYLE_TRIANGLE: DebugDrawStyle = DebugDrawStyle {
        color: Color::LIME_GREEN,
        thickness: 2.,
        outline: true,
        depth: 1.,
        ..Default::default()
    };
    static ref STYLE_ALGORITHM_EAR_CANDIDATE_ACCEPTED: DebugDrawStyle = DebugDrawStyle {
        color: Color::rgba(0., 1., 0., 0.05),
        depth: -1.,
        ..Default::default()
    };
    static ref STYLE_ALGORITHM_EAR_CANDIDATE_REJECTED: DebugDrawStyle = DebugDrawStyle {
        color: Color::rgba(1., 0., 0., 0.05),
        depth: -1.,
        ..Default::default()
    };
    static ref STYLE_ALGORITHM_VERTEX_BAD: DebugDrawStyle = DebugDrawStyle {
        color: Color::RED,
        depth: 1.,
        ..Default::default()
    };
}

pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(SCENE).with_system(setup));
        app.add_system_set(SystemSet::on_update(SCENE).with_system(draw));
    }
}

fn setup(mut commands: Commands) {
    commands.spawn((PolygonBuilder::default(), Label::new("polygon_builder")));
}

#[derive(Default)]
struct Draw {
    stepper_config: StepperConfig,
}

fn draw(
    mut local: Local<Draw>,
    mut egui_context: ResMut<EguiContext>,
    mut debug_draw: ResMut<DebugDraw>,
    label_query: Query<(Entity, &Label)>,
    polygon_builder_query: Query<&PolygonBuilder>,
) {
    let polygon_builder = polygon_builder_query
        .get(Label::find(&label_query, "polygon_builder"))
        .unwrap();

    polygon_builder.draw(debug_draw.as_mut());

    let vertex_list = VertexList2(polygon_builder.vertices().clone());
    if !vertex_list.is_simple_polygon() {
        for edge in vertex_list.edges() {
            edge.draw(debug_draw.as_mut(), *STYLE_POLYGON_BAD);
        }
    }

    let mut stepper = local.stepper_config.begin();
    let triangles = visual_ear_clipping(
        polygon_builder.vertices(),
        &mut stepper,
        debug_draw.as_mut(),
    );

    if stepper.show_step() || !stepper.is_enabled() {
        for triangle in triangles.iter() {
            triangle.draw(debug_draw.as_mut(), *STYLE_TRIANGLE);
        }
    }

    egui::Window::new("Algorithm Step").show(egui_context.ctx_mut(), |ui| {
        local.stepper_config.ui(ui, stepper.last_step());
    });
}

fn visual_ear_clipping(
    vertices: &Vec<Vec2>,
    stepper: &mut Stepper,
    debug_draw: &mut DebugDraw,
) -> Vec<Triangle2> {
    if vertices.len() < 3 {
        return vec![];
    }

    let mut vertex_list = VertexList2(vertices.clone());
    if !vertex_list.is_simple_polygon() {
        return vec![];
    }
    vertex_list.make_counterclockwise();

    let mut triangles: Vec<Triangle2> = vec![];
    let mut vertices = vertex_list.0;
    while vertices.len() > 3 {
        for vertex_index in 0..vertices.len() {
            let triangle_vertex_indices = [
                (vertex_index + vertices.len() - 1) % vertices.len(),
                vertex_index,
                (vertex_index + 1) % vertices.len(),
            ];
            let triangle_vertices = [
                vertices[triangle_vertex_indices[0]],
                vertices[triangle_vertex_indices[1]],
                vertices[triangle_vertex_indices[2]],
            ];
            let triangle = Triangle2::new(
                triangle_vertices[0],
                triangle_vertices[1],
                triangle_vertices[2],
            );

            let mut is_ear = triangle.is_clockwise();
            if is_ear {
                let mut bad_vertices = vec![];
                for other_vertex_index in 0..vertices.len() {
                    let other_vertex = vertices[other_vertex_index];
                    if !triangle_vertex_indices.contains(&other_vertex_index) {
                        if triangle.contains_point(other_vertex) {
                            bad_vertices.push(other_vertex);
                            is_ear = false;
                        }
                    }
                }
                if stepper.show_step() {
                    for triangle in triangles.iter() {
                        triangle.draw(debug_draw, *STYLE_TRIANGLE);
                    }
                    let style = if is_ear {
                        *STYLE_ALGORITHM_EAR_CANDIDATE_ACCEPTED
                    } else {
                        *STYLE_ALGORITHM_EAR_CANDIDATE_REJECTED
                    };
                    triangle.draw(debug_draw, style);
                    for bad_vertex in bad_vertices.iter() {
                        Circle::new(*bad_vertex, 20.).draw(debug_draw, *STYLE_ALGORITHM_VERTEX_BAD);
                    }
                }
            }

            if is_ear {
                triangles.push(triangle);
                vertices.remove(vertex_index);
                break;
            }
        }
    }

    if stepper.show_step() {
        for triangle in triangles.iter() {
            triangle.draw(debug_draw, *STYLE_TRIANGLE);
        }
        Triangle2::new(vertices[0], vertices[1], vertices[2])
            .draw(debug_draw, *STYLE_ALGORITHM_EAR_CANDIDATE_ACCEPTED);
    }

    triangles.push(Triangle2::new(vertices[0], vertices[1], vertices[2]));
    triangles
}
