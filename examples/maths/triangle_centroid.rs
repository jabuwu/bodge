use bevy::prelude::*;
use bodge::{
    bevy::{DebugDraw, DebugDrawStyle, Label},
    geometry::{Circle, LineSegment2, Triangle2},
    prelude::*,
};
use lazy_static::lazy_static;

use crate::{util::Draggable, AppScene};

const SCENE: AppScene = AppScene::TriangleCentroid;

lazy_static! {
    static ref STYLE_TRIANGLE: DebugDrawStyle = DebugDrawStyle {
        color: Color::GRAY,
        thickness: 2.,
        outline: true,
        depth: 1.,
        ..Default::default()
    };
    static ref STYLE_MEDIAN_LINE: DebugDrawStyle = DebugDrawStyle {
        color: Color::DARK_GRAY,
        depth: 0.,
        ..Default::default()
    };
    static ref STYLE_CENTROID: DebugDrawStyle = DebugDrawStyle {
        color: Color::PINK,
        depth: 2.,
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

    let median_a = LineSegment2::new(triangle.a(), triangle.bc().center());
    let median_b = LineSegment2::new(triangle.b(), triangle.ca().center());
    let median_c = LineSegment2::new(triangle.c(), triangle.ab().center());
    median_a.draw(debug_draw.as_mut(), *STYLE_MEDIAN_LINE);
    median_b.draw(debug_draw.as_mut(), *STYLE_MEDIAN_LINE);
    median_c.draw(debug_draw.as_mut(), *STYLE_MEDIAN_LINE);

    let centroid = median_a.line().intersection_point(median_b.line()).unwrap();
    Circle::new(centroid, 10.).draw(debug_draw.as_mut(), *STYLE_CENTROID);
}
