use bevy::prelude::*;
use bodge::{
    bevy::{DebugDraw, DebugDrawStyle, Label},
    geometry::{Circle, Triangle2},
    prelude::*,
};
use lazy_static::lazy_static;

use crate::{util::Draggable, AppScene};

const SCENE: AppScene = AppScene::TriangleCircumcenter;

lazy_static! {
    static ref STYLE_TRIANGLE: DebugDrawStyle = DebugDrawStyle {
        color: Color::WHITE,
        thickness: 2.,
        outline: true,
        depth: 2.,
        ..Default::default()
    };
    static ref STYLE_PERPENDICULAR_BISECTOR_LINE: DebugDrawStyle = DebugDrawStyle {
        color: Color::DARK_GRAY,
        depth: 1.,
        ..Default::default()
    };
    static ref STYLE_CIRCUMCENTER: DebugDrawStyle = DebugDrawStyle {
        color: Color::PINK,
        depth: 3.,
        ..Default::default()
    };
    static ref STYLE_CIRCUMCIRCLE: DebugDrawStyle = DebugDrawStyle {
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

    let perpendicular_bisector_ab = triangle.ab().perpendicular_bisector();
    let perpendicular_bisector_bc = triangle.bc().perpendicular_bisector();
    let perpendicular_bisector_ca = triangle.ca().perpendicular_bisector();
    perpendicular_bisector_ab.draw(debug_draw.as_mut(), *STYLE_PERPENDICULAR_BISECTOR_LINE);
    perpendicular_bisector_bc.draw(debug_draw.as_mut(), *STYLE_PERPENDICULAR_BISECTOR_LINE);
    perpendicular_bisector_ca.draw(debug_draw.as_mut(), *STYLE_PERPENDICULAR_BISECTOR_LINE);

    let circumcenter = perpendicular_bisector_ab
        .intersection_point(perpendicular_bisector_bc)
        .unwrap();
    let circumcircle = Circle::new(circumcenter, circumcenter.distance(triangle.a()) * 2.);
    circumcircle.draw(debug_draw.as_mut(), *STYLE_CIRCUMCIRCLE);
    Circle::new(circumcenter, 10.).draw(debug_draw.as_mut(), *STYLE_CIRCUMCENTER);
}
