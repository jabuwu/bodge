use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};
use bodge::{
    bevy::{DebugDraw, DebugDrawStyle, Label},
    geometry::LineSegment2,
    prelude::DebugDrawable,
};

use crate::{util::PolygonBuilder, AppScene};

const SCENE: AppScene = AppScene::PolygonIsClockwise;

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

fn draw(
    mut egui_context: ResMut<EguiContext>,
    mut debug_draw: ResMut<DebugDraw>,
    mut polygon_builder_query: Query<&mut PolygonBuilder>,
    label_query: Query<(Entity, &Label)>,
) {
    let mut polygon_builder = polygon_builder_query
        .get_mut(Label::find(&label_query, "polygon_builder"))
        .unwrap();

    polygon_builder.draw(debug_draw.as_mut());

    for vertex_index in 0..polygon_builder.vertices().len() {
        let vertex = polygon_builder.vertices()[vertex_index];
        let next_vertex =
            polygon_builder.vertices()[(vertex_index + 1) % polygon_builder.vertices().len()];
        let direction = (next_vertex - vertex).normalize();
        let offset_position = next_vertex - direction * 10.;
        LineSegment2::new(
            offset_position,
            offset_position + Vec2::from_angle(Vec2::X.angle_between(direction) + PI * 0.8) * 20.,
        )
        .draw(
            debug_draw.as_mut(),
            DebugDrawStyle::new(Color::PINK)
                .with_thickness(5.)
                .with_depth(2.),
        );
        LineSegment2::new(
            offset_position,
            offset_position + Vec2::from_angle(Vec2::X.angle_between(direction) - PI * 0.8) * 20.,
        )
        .draw(
            debug_draw.as_mut(),
            DebugDrawStyle::new(Color::PINK)
                .with_thickness(5.)
                .with_depth(2.),
        );
    }

    let mut area = 0.;
    for i in 0..polygon_builder.vertices().len() {
        let j = (i + 1) % polygon_builder.vertices().len();
        area += polygon_builder.vertices()[i].x * polygon_builder.vertices()[j].y;
        area -= polygon_builder.vertices()[j].x * polygon_builder.vertices()[i].y;
    }
    let is_clockwise = area / 2. < 0.;

    egui::Window::new("Debug").show(egui_context.ctx_mut(), |ui| {
        ui.label(format!("Is Clockwise: {}", is_clockwise));
        if ui.button("Reverse").clicked() {
            let reversed_vertices: Vec<Vec2> = polygon_builder
                .vertices()
                .iter()
                .map(|vec| *vec)
                .rev()
                .collect();
            *polygon_builder.vertices_mut() = reversed_vertices;
        }
    });
}
