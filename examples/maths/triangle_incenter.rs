use bevy::prelude::*;
use bodge::{
    bevy::{DebugDraw, DebugDrawStyle, Label},
    geometry::{Circle, LineSegment2, Triangle2},
    prelude::*,
};
use lazy_static::lazy_static;

use crate::{util::Draggable, AppScene};

const SCENE: AppScene = AppScene::TriangleIncenter;

lazy_static! {
    static ref STYLE_TRIANGLE: DebugDrawStyle = DebugDrawStyle {
        color: Color::WHITE,
        thickness: 2.,
        outline: true,
        depth: 5.,
        ..Default::default()
    };
    static ref STYLE_ANGLE_BISECTOR_LINE: DebugDrawStyle = DebugDrawStyle {
        color: Color::DARK_GRAY,
        depth: 2.,
        ..Default::default()
    };
    static ref STYLE_INCENTER: DebugDrawStyle = DebugDrawStyle {
        color: Color::PINK,
        depth: 3.,
        ..Default::default()
    };
    static ref STYLE_INCENTER_DISTANCE_LINE: DebugDrawStyle = DebugDrawStyle {
        color: Color::PINK,
        depth: 4.,
        ..Default::default()
    };
    static ref STYLE_INCIRCLE: DebugDrawStyle = DebugDrawStyle {
        color: Color::rgb(0.02, 0.02, 0.02),
        depth: 0.,
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
        TransformBundle::from_transform(Transform::from_xyz(100., -100., 0.)),
        Draggable::new(32.),
        Label::new("a"),
    ));
    commands.spawn((
        TransformBundle::from_transform(Transform::from_xyz(-100., -100., 0.)),
        Draggable::new(32.),
        Label::new("b"),
    ));
    commands.spawn((
        TransformBundle::from_transform(Transform::from_xyz(0., 100., 0.)),
        Draggable::new(32.),
        Label::new("c"),
    ));
}

fn draw(
    mut debug_draw: ResMut<DebugDraw>,
    label_query: Query<(Entity, &Label)>,
    transform_query: Query<&Transform>,
) {
    let vertices = transform_query
        .get_many([
            Label::find(&label_query, "a"),
            Label::find(&label_query, "b"),
            Label::find(&label_query, "c"),
        ])
        .unwrap()
        .map(|transform| transform.translation.truncate());

    let triangle = Triangle2::new(vertices[0], vertices[1], vertices[2]);
    triangle.draw(debug_draw.as_mut(), *STYLE_TRIANGLE);

    let angle_bisector_a = triangle.a_angle_bisector();
    let angle_bisector_b = triangle.b_angle_bisector();
    let angle_bisector_c = triangle.c_angle_bisector();
    angle_bisector_a.draw(debug_draw.as_mut(), *STYLE_ANGLE_BISECTOR_LINE);
    angle_bisector_b.draw(debug_draw.as_mut(), *STYLE_ANGLE_BISECTOR_LINE);
    angle_bisector_c.draw(debug_draw.as_mut(), *STYLE_ANGLE_BISECTOR_LINE);

    let incenter = angle_bisector_a
        .intersection_point(angle_bisector_b)
        .unwrap();
    Circle::new(incenter, 10.).draw(debug_draw.as_mut(), *STYLE_INCENTER);

    let incenter_distance_ab = LineSegment2::new(incenter, triangle.ab().closest_point(incenter));
    let incenter_distance_bc = LineSegment2::new(incenter, triangle.bc().closest_point(incenter));
    let incenter_distance_ca = LineSegment2::new(incenter, triangle.ca().closest_point(incenter));
    incenter_distance_ab.draw(debug_draw.as_mut(), *STYLE_INCENTER_DISTANCE_LINE);
    incenter_distance_bc.draw(debug_draw.as_mut(), *STYLE_INCENTER_DISTANCE_LINE);
    incenter_distance_ca.draw(debug_draw.as_mut(), *STYLE_INCENTER_DISTANCE_LINE);

    let incircle = Circle::new(
        incenter,
        incenter.distance(triangle.ab().closest_point(incenter)) * 2.,
    );

    incircle.draw(debug_draw.as_mut(), *STYLE_INCIRCLE);
}
