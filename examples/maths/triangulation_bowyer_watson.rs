use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};
use bodge::{
    bevy::{DebugDraw, DebugDrawStyle, Label},
    geometry::{Circle, Triangle2},
    prelude::*,
};
use lazy_static::lazy_static;

use crate::{
    util::{PointListBuilder, Stepper, StepperConfig},
    AppScene,
};

const SCENE: AppScene = AppScene::TriangulationBowyerWatson;

lazy_static! {
    static ref STYLE_TRIANGLE: DebugDrawStyle = DebugDrawStyle {
        color: Color::LIME_GREEN,
        thickness: 2.,
        outline: true,
        depth: 0.,
        ..Default::default()
    };
    static ref STYLE_ALGORITHM_POINT_CURRENT: DebugDrawStyle = DebugDrawStyle {
        color: Color::PINK,
        depth: 3.,
        ..Default::default()
    };
    static ref STYLE_ALGORITHM_POINT_CURRENT_BAD: DebugDrawStyle = DebugDrawStyle {
        color: Color::RED,
        depth: 3.,
        ..Default::default()
    };
    static ref STYLE_ALGORITHM_TRIANGLE: DebugDrawStyle = DebugDrawStyle {
        color: Color::WHITE,
        thickness: 2.,
        outline: true,
        depth: 0.,
        ..Default::default()
    };
    static ref STYLE_ALGORITHM_TRIANGLE_CURRENT: DebugDrawStyle = DebugDrawStyle {
        color: Color::GREEN,
        outline: true,
        thickness: 3.,
        depth: 3.,
        ..Default::default()
    };
    static ref STYLE_ALGORITHM_TRIANGLE_CURRENT_VERTEX: DebugDrawStyle = DebugDrawStyle {
        color: Color::GREEN,
        depth: 3.,
        ..Default::default()
    };
    static ref STYLE_ALGORITHM_TRIANGLE_BAD: DebugDrawStyle = DebugDrawStyle {
        color: Color::rgba(0.45, 0., 0., 0.1),
        thickness: 3.,
        depth: -1.,
        ..Default::default()
    };
    static ref STYLE_ALGORITHM_EDGE_KEEP: DebugDrawStyle = DebugDrawStyle {
        color: Color::GREEN,
        thickness: 3.,
        depth: 4.,
        ..Default::default()
    };
    static ref STYLE_ALGORITHM_EDGE_REMOVE: DebugDrawStyle = DebugDrawStyle {
        color: Color::RED,
        thickness: 3.,
        depth: 4.,
        ..Default::default()
    };
    static ref STYLE_ALGORITHM_CIRCUMCIRCLE: DebugDrawStyle = DebugDrawStyle {
        color: Color::rgb(0.0, 0.02, 0.0),
        depth: -1.,
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
    commands.spawn((
        PointListBuilder::default(),
        Label::new("point_list_builder"),
    ));
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
    point_list_builder_query: Query<&PointListBuilder>,
) {
    let point_list_builder = point_list_builder_query
        .get(Label::find(&label_query, "point_list_builder"))
        .unwrap();

    point_list_builder.draw(debug_draw.as_mut());

    let mut stepper = local.stepper_config.begin();
    let triangles = visual_bowyer_watson(
        point_list_builder.vertices(),
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

fn visual_bowyer_watson(
    points: &Vec<Vec2>,
    stepper: &mut Stepper,
    debug_draw: &mut DebugDraw,
) -> Vec<Triangle2> {
    let mut center = Vec2::ZERO;
    for point in points.iter() {
        center += *point;
    }
    center /= points.len() as f32;
    let mut super_triangle = Triangle2::new(
        center + Vec2::new(-100., -100.),
        center + Vec2::new(-100., 100.),
        center + Vec2::new(150., 0.),
    );
    loop {
        let mut scale = false;
        for point in points.iter() {
            if !super_triangle.contains_point(*point) {
                scale = true;
                break;
            }
        }
        if scale {
            super_triangle.scale(1.1);
        } else {
            break;
        }
    }
    super_triangle.scale(1.2);

    let mut triangulation = vec![super_triangle];

    if stepper.show_step() {
        for triangle in triangulation.iter() {
            triangle.draw(
                debug_draw,
                DebugDrawStyle::new(Color::WHITE).with_outline(true),
            )
        }
    }

    for point in points.iter() {
        let mut bad_triangles = vec![];
        for (triangle_index, triangle) in triangulation.iter().enumerate() {
            let circumcircle = triangle.circumcircle().unwrap();
            if circumcircle.contains_point(*point) {
                bad_triangles.push(triangle_index);
                if stepper.show_step() {
                    Circle::new(triangle.a(), 15.)
                        .draw(debug_draw, *STYLE_ALGORITHM_TRIANGLE_CURRENT_VERTEX);
                    Circle::new(triangle.b(), 15.)
                        .draw(debug_draw, *STYLE_ALGORITHM_TRIANGLE_CURRENT_VERTEX);
                    Circle::new(triangle.c(), 15.)
                        .draw(debug_draw, *STYLE_ALGORITHM_TRIANGLE_CURRENT_VERTEX);
                    Circle::new(*point, 25.).draw(debug_draw, *STYLE_ALGORITHM_POINT_CURRENT);
                    triangle.draw(debug_draw, *STYLE_ALGORITHM_TRIANGLE_CURRENT);
                    circumcircle.draw(debug_draw, *STYLE_ALGORITHM_CIRCUMCIRCLE);
                    for other_triangle in triangulation.iter() {
                        other_triangle.draw(debug_draw, *STYLE_ALGORITHM_TRIANGLE);
                    }
                }
            }
        }
        if bad_triangles.len() > 0 {
            let mut edges = vec![];
            let mut shared_edges = vec![];
            for bad_triangle_index in bad_triangles.iter() {
                let bad_triangle = triangulation[*bad_triangle_index];
                for edge in bad_triangle.edges().iter() {
                    let mut shares_edge = false;
                    for other_bad_triangle_index in bad_triangles.iter() {
                        if *other_bad_triangle_index != *bad_triangle_index {
                            let other_bad_triangle = triangulation[*other_bad_triangle_index];
                            for other_edge in other_bad_triangle.edges().iter() {
                                if edge.is_same(*other_edge) {
                                    shares_edge = true;
                                    break;
                                }
                            }
                        }
                    }
                    if shares_edge {
                        shared_edges.push(*edge);
                    } else {
                        edges.push(*edge);
                    }
                }
            }
            if stepper.show_step() {
                Circle::new(*point, 15.).draw(debug_draw, *STYLE_ALGORITHM_POINT_CURRENT_BAD);
                for other_triangle in triangulation.iter() {
                    other_triangle.draw(debug_draw, *STYLE_ALGORITHM_TRIANGLE);
                }
                for bad_triangle_index in bad_triangles.iter() {
                    triangulation[*bad_triangle_index]
                        .draw(debug_draw, *STYLE_ALGORITHM_TRIANGLE_BAD);
                }
                for edge in edges.iter() {
                    edge.draw(debug_draw, *STYLE_ALGORITHM_EDGE_KEEP);
                }
                for shared_edge in shared_edges.iter() {
                    shared_edge.draw(debug_draw, *STYLE_ALGORITHM_EDGE_REMOVE);
                }
            }
            for bad_triangle in bad_triangles.iter().rev() {
                triangulation.remove(*bad_triangle);
            }
            for edge in edges.iter() {
                let new_triangle = Triangle2::new(edge.start, edge.end, *point);
                triangulation.push(new_triangle);
            }
        }
    }

    if stepper.show_step() {
        for triangle in triangulation.iter() {
            let mut remove = false;
            for vertex in triangle.vertices.iter() {
                for super_vertex in super_triangle.vertices.iter() {
                    if *vertex == *super_vertex {
                        remove = true;
                    }
                }
            }
            triangle.draw(debug_draw, *STYLE_ALGORITHM_TRIANGLE);
            if remove {
                triangle.draw(debug_draw, *STYLE_ALGORITHM_TRIANGLE_BAD);
            }
            Circle::new(super_triangle.a(), 15.)
                .draw(debug_draw, *STYLE_ALGORITHM_POINT_CURRENT_BAD);
            Circle::new(super_triangle.b(), 15.)
                .draw(debug_draw, *STYLE_ALGORITHM_POINT_CURRENT_BAD);
            Circle::new(super_triangle.c(), 15.)
                .draw(debug_draw, *STYLE_ALGORITHM_POINT_CURRENT_BAD);
        }
    }

    triangulation.retain(|triangle| {
        for vertex in triangle.vertices.iter() {
            for super_vertex in super_triangle.vertices.iter() {
                if *vertex == *super_vertex {
                    return false;
                }
            }
        }
        true
    });

    triangulation
}
