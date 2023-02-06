use bevy::prelude::*;
use bodge::{
    bevy::{Cursor, DebugDraw, DebugDrawStyle, Label},
    geometry::{Aabb, Circle, LineSegment2},
    prelude::*,
};
use lazy_static::lazy_static;

use crate::{util::Draggable, AppScene};

const SCENE: AppScene = AppScene::ClosestPoint;

lazy_static! {
    static ref STYLE_VERTEX: DebugDrawStyle = DebugDrawStyle {
        color: Color::RED,
        depth: 0.7,
        ..Default::default()
    };
    static ref STYLE_SHAPE: DebugDrawStyle = DebugDrawStyle {
        color: Color::WHITE,
        depth: 0.,
        thickness: 2.,
        ..Default::default()
    };
    static ref STYLE_SHAPE_INSIDE: DebugDrawStyle = DebugDrawStyle {
        color: Color::LIME_GREEN,
        depth: 0.5,
        thickness: 2.,
        ..Default::default()
    };
    static ref STYLE_CLOSEST_POINT: DebugDrawStyle = DebugDrawStyle {
        color: Color::DARK_GREEN,
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
    commands.spawn((
        TransformBundle::from_transform(Transform::from_xyz(100., 100., 0.)),
        Draggable::new(32.),
        Label::new("line_segment_a"),
    ));
    commands.spawn((
        TransformBundle::from_transform(Transform::from_xyz(300., 200., 0.)),
        Draggable::new(32.),
        Label::new("line_segment_b"),
    ));

    commands.spawn((
        TransformBundle::from_transform(Transform::from_xyz(-300., 100., 0.)),
        Draggable::new(32.),
        Label::new("circle_position"),
    ));
    commands.spawn((
        TransformBundle::from_transform(Transform::from_xyz(-300., 150., 0.)),
        Draggable::new(32.),
        Label::new("circle_radius"),
    ));

    commands.spawn((
        TransformBundle::from_transform(Transform::from_xyz(-300., -100., 0.)),
        Draggable::new(32.),
        Label::new("aabb_a"),
    ));
    commands.spawn((
        TransformBundle::from_transform(Transform::from_xyz(-100., -300., 0.)),
        Draggable::new(32.),
        Label::new("aabb_b"),
    ));
}

fn draw(
    mut debug_draw: ResMut<DebugDraw>,
    label_query: Query<(Entity, &Label)>,
    transform_query: Query<&Transform>,
    cursor: Res<Cursor>,
) {
    let vertices = transform_query
        .get_many([
            Label::find(&label_query, "line_segment_a"),
            Label::find(&label_query, "line_segment_b"),
            Label::find(&label_query, "circle_position"),
            Label::find(&label_query, "circle_radius"),
            Label::find(&label_query, "aabb_a"),
            Label::find(&label_query, "aabb_b"),
        ])
        .unwrap()
        .map(|transform| transform.translation.truncate());

    for vertex in vertices.iter() {
        Circle::new(*vertex, 8.).draw(debug_draw.as_mut(), *STYLE_VERTEX);
    }

    let [line_segment_a, line_segment_b, circle_position, circle_radius, aabb_a, aabb_b] = vertices;

    macro_rules! draw_shape {
        ($shape:expr) => {
            $shape.draw(debug_draw.as_mut(), *STYLE_SHAPE);
        };
    }

    macro_rules! draw_shape_contains {
        ($shape:expr) => {
            if $shape.contains_point(cursor.position) {
                $shape.draw(debug_draw.as_mut(), *STYLE_SHAPE_INSIDE);
            } else {
                $shape.draw(debug_draw.as_mut(), *STYLE_SHAPE);
            }
        };
    }

    macro_rules! draw_closest_point {
        ($shape:expr) => {
            Circle::new($shape.closest_point(cursor.position), 10.)
                .draw(debug_draw.as_mut(), *STYLE_CLOSEST_POINT);
        };
    }

    let line_segment = LineSegment2::new(line_segment_a, line_segment_b);
    draw_shape!(line_segment);
    draw_closest_point!(line_segment);

    let circle = Circle::new(
        circle_position,
        circle_position.distance(circle_radius) * 2.,
    );
    draw_shape_contains!(circle);
    draw_closest_point!(circle);

    let aabb = Aabb::new((aabb_a + aabb_b) / 2., (aabb_a - aabb_b).abs());
    draw_shape_contains!(aabb);
    draw_closest_point!(aabb);
}
