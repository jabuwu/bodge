use bevy::prelude::*;
use bevy_egui::{
    egui::{ComboBox, DragValue, Window},
    EguiContext,
};
use bodge::{
    bevy::{DebugDraw, DebugDrawStyle, Label},
    geometry::{Circle, LineSegment2, Polyline},
    prelude::*,
};
use lazy_static::lazy_static;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::{
    util::{PolygonBuilder, Stepper, StepperConfig},
    AppScene,
};

const SCENE: AppScene = AppScene::PolylineSimplification;

lazy_static! {
    static ref STYLE_POLYLINE_RESULT: DebugDrawStyle = DebugDrawStyle {
        color: Color::LIME_GREEN,
        thickness: 5.,
        depth: -1.,
        ..Default::default()
    };
    static ref STYLE_ALGORITHM_VERTEX_PREVIOUS: DebugDrawStyle = DebugDrawStyle {
        color: Color::RED,
        depth: 1.,
        ..Default::default()
    };
    static ref STYLE_ALGORITHM_VERTEX_PREVIOUS_RADIUS: DebugDrawStyle = DebugDrawStyle {
        color: Color::rgba(1., 0., 0., 0.05),
        depth: 0.,
        ..Default::default()
    };
    static ref STYLE_ALGORITHM_VERTEX_CURRENT: DebugDrawStyle = DebugDrawStyle {
        color: Color::PINK,
        depth: 1.,
        ..Default::default()
    };
    static ref STYLE_ALGORITHM_LINE_NEW: DebugDrawStyle = DebugDrawStyle {
        color: Color::GREEN,
        thickness: 5.,
        depth: -1.,
        ..Default::default()
    };
    static ref STYLE_ALGORITHM_CRUDE_LINE: DebugDrawStyle = DebugDrawStyle {
        color: Color::rgb(0.1, 0.1, 0.1),
        thickness: 1.,
        depth: 2.,
        ..Default::default()
    };
    static ref STYLE_ALGORITHM_INCOMPLETE_LINE: DebugDrawStyle = DebugDrawStyle {
        color: Color::PINK,
        thickness: 5.,
        depth: 2.,
        ..Default::default()
    };
    static ref STYLE_ALGORITHM_INCOMPLETE_LINE_SKINNY: DebugDrawStyle = DebugDrawStyle {
        color: Color::PINK,
        thickness: 1.,
        depth: 2.,
        ..Default::default()
    };
    static ref STYLE_ALGORITHM_INCOMPLETE_VERTEX: DebugDrawStyle = DebugDrawStyle {
        color: Color::PINK,
        depth: 2.,
        ..Default::default()
    };
    static ref STYLE_ALGORITHM_COMPLETE_VERTEX: DebugDrawStyle = DebugDrawStyle {
        color: Color::GREEN,
        depth: 2.,
        ..Default::default()
    };
}

#[derive(Default, Debug, EnumIter, PartialEq, Clone, Copy)]
enum Algorithm {
    #[default]
    SimpleDistance,
    DouglasPeucker,
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
        PolygonBuilder::default().as_polyline(),
        Label::new("polygon_builder"),
    ));
}

struct Draw {
    stepper_config: StepperConfig,
    tolerance: f32,
    algorithm: Algorithm,
}

impl Default for Draw {
    fn default() -> Draw {
        Draw {
            stepper_config: StepperConfig::default(),
            tolerance: 100.,
            algorithm: Algorithm::default(),
        }
    }
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

    let mut stepper = local.stepper_config.begin();
    let polyline = match local.algorithm {
        Algorithm::SimpleDistance => visual_polyline_simplification_simple_distance(
            polygon_builder.vertices(),
            local.tolerance,
            &mut stepper,
            debug_draw.as_mut(),
        ),
        Algorithm::DouglasPeucker => visual_polyline_simplification_douglas_peucker(
            polygon_builder.vertices(),
            local.tolerance,
            &mut stepper,
            debug_draw.as_mut(),
        ),
    };

    if stepper.show_step() || !stepper.is_enabled() {
        polyline.draw(debug_draw.as_mut(), *STYLE_POLYLINE_RESULT);
    }

    Window::new("Algorithm Step").show(egui_context.ctx_mut(), |ui| {
        ComboBox::from_label("Algorithm")
            .selected_text(format!("{:?}", local.algorithm))
            .show_ui(ui, |ui| {
                for algorithm in Algorithm::iter() {
                    ui.selectable_value(
                        &mut local.algorithm,
                        algorithm,
                        format!("{:?}", algorithm),
                    );
                }
            });
        ui.label("Tolerance");
        ui.add(DragValue::new(&mut local.tolerance).clamp_range(0.01..=1000.0));
        local.stepper_config.ui(ui, stepper.last_step());
    });
}

fn visual_polyline_simplification_simple_distance(
    vertices: &Vec<Vec2>,
    tolerance: f32,
    stepper: &mut Stepper,
    debug_draw: &mut DebugDraw,
) -> Polyline {
    let mut polyline = Polyline::new();
    if vertices.len() > 0 {
        polyline.0.push(vertices[0]);
        for (vertex_index, vertex) in vertices.iter().enumerate().skip(1) {
            let show_step = stepper.show_step();
            if show_step {
                Circle::new(*polyline.0.last().unwrap(), 10.)
                    .draw(debug_draw, *STYLE_ALGORITHM_VERTEX_PREVIOUS);
                Circle::new(*polyline.0.last().unwrap(), tolerance * 2.)
                    .draw(debug_draw, *STYLE_ALGORITHM_VERTEX_PREVIOUS_RADIUS);
                polyline.draw(debug_draw, *STYLE_POLYLINE_RESULT);
                Circle::new(*vertex, 10.).draw(debug_draw, *STYLE_ALGORITHM_VERTEX_CURRENT);
            }
            if vertex.distance(*polyline.0.last().unwrap()) > tolerance
                || vertex_index == vertices.len() - 1
            {
                if show_step {
                    LineSegment2::new(*polyline.0.last().unwrap(), *vertex)
                        .draw(debug_draw, *STYLE_ALGORITHM_LINE_NEW);
                }
                polyline.0.push(*vertex);
            }
        }
    }
    polyline
}

fn visual_polyline_simplification_douglas_peucker(
    vertices: &Vec<Vec2>,
    tolerance: f32,
    stepper: &mut Stepper,
    debug_draw: &mut DebugDraw,
) -> Polyline {
    fn subdivide(
        line_segment: LineSegment2,
        vertices: Vec<Vec2>,
        tolerance: f32,
        stepper: &mut Stepper,
        debug_draw: &mut DebugDraw,
        show_step: bool,
    ) -> Polyline {
        let mut polyline = Polyline(vec![line_segment.start, line_segment.end]);
        if vertices.len() != 0 {
            let first_closest_point = line_segment.closest_point(vertices[0]);
            let mut furthest = (
                first_closest_point.distance(vertices[0]),
                first_closest_point,
                0,
            );
            for (vertex_index, vertex) in vertices.iter().enumerate() {
                let closest_point = line_segment.closest_point(*vertex);
                let distance = closest_point.distance(*vertex);
                if distance > furthest.0 {
                    furthest = (distance, closest_point, vertex_index);
                }
            }
            if furthest.0 > tolerance {
                if show_step {
                    line_segment.draw(debug_draw, *STYLE_ALGORITHM_INCOMPLETE_LINE);

                    let vertex = vertices[furthest.2];
                    Circle::new(vertex, 20.).draw(debug_draw, *STYLE_ALGORITHM_INCOMPLETE_VERTEX);

                    LineSegment2::new(vertex, line_segment.center())
                        .draw(debug_draw, *STYLE_ALGORITHM_INCOMPLETE_LINE_SKINNY);
                }
                let next_show_step = stepper.show_step();
                if next_show_step {
                    line_segment.draw(debug_draw, *STYLE_ALGORITHM_CRUDE_LINE);
                }
                let left_vertices = vertices[..furthest.2].to_vec();
                let right_vertices = vertices[(furthest.2 + 1)..].to_vec();
                let left_polyline = subdivide(
                    LineSegment2::new(line_segment.start, vertices[furthest.2]),
                    left_vertices,
                    tolerance,
                    stepper,
                    debug_draw,
                    next_show_step,
                );
                let mut right_polyline = subdivide(
                    LineSegment2::new(vertices[furthest.2], line_segment.end),
                    right_vertices,
                    tolerance,
                    stepper,
                    debug_draw,
                    next_show_step,
                );
                polyline = left_polyline;
                right_polyline.0.remove(0);
                polyline.0.append(&mut right_polyline.0);
            } else if show_step {
                line_segment.draw(debug_draw, *STYLE_POLYLINE_RESULT);
                for vertex in vertices.iter() {
                    Circle::new(*vertex, 10.).draw(debug_draw, *STYLE_ALGORITHM_COMPLETE_VERTEX);
                }
            }
        } else if show_step {
            line_segment.draw(debug_draw, *STYLE_POLYLINE_RESULT);
        }
        polyline
    }

    if vertices.len() > 0 {
        if vertices.len() > 1 {
            let new_vertices = vertices[1..vertices.len() - 1].to_vec();
            let next_show_step = stepper.show_step();
            subdivide(
                LineSegment2::new(*vertices.first().unwrap(), *vertices.last().unwrap()),
                new_vertices,
                tolerance,
                stepper,
                debug_draw,
                next_show_step,
            )
        } else {
            Polyline(vec![*vertices.first().unwrap()])
        }
    } else {
        Polyline::new()
    }
}
