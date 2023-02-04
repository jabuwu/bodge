use bevy::prelude::*;
use bevy_egui::{
    egui::{self, ComboBox},
    EguiContext,
};
use bodge::{
    bevy::{DebugDraw, DebugDrawStyle, Label},
    geometry::{Circle, LineSegment2},
    prelude::{Colliding, DebugDrawable},
};
use lazy_static::lazy_static;
use strum::IntoEnumIterator;
use strum_macros::{EnumDiscriminants, EnumIter};

use crate::{util::Draggable, AppScene};

const SCENE: AppScene = AppScene::Colliding;

lazy_static! {
    static ref STYLE_VERTEX: DebugDrawStyle = DebugDrawStyle {
        color: Color::WHITE,
        thickness: 2.,
        depth: 1.,
        ..Default::default()
    };
    static ref STYLE_VERTEX_COLLIDING: DebugDrawStyle = DebugDrawStyle {
        color: Color::DARK_GREEN,
        thickness: 2.,
        depth: 1.,
        ..Default::default()
    };
    static ref STYLE_SHAPE: DebugDrawStyle = DebugDrawStyle {
        color: Color::GRAY,
        thickness: 2.,
        depth: 0.,
        ..Default::default()
    };
    static ref STYLE_SHAPE_COLLIDING: DebugDrawStyle = DebugDrawStyle {
        color: Color::LIME_GREEN,
        thickness: 2.,
        depth: 0.,
        ..Default::default()
    };
}

pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<State>();
        app.add_system_set(SystemSet::on_enter(SCENE).with_system(setup));
        app.add_system_set(SystemSet::on_update(SCENE).with_system(draw));
    }
}

#[derive(Resource)]
struct State {
    setup: bool,
    collidable_1: Collidable,
    collidable_2: Collidable,
}

impl Default for State {
    fn default() -> State {
        State {
            setup: true,
            collidable_1: Collidable::LineSegment(LineSegment2::new(Vec2::ZERO, Vec2::ZERO)),
            collidable_2: Collidable::Circle(Circle::new(Vec2::ZERO, 10.)),
        }
    }
}

#[derive(Debug, Clone, Copy, EnumDiscriminants)]
#[strum_discriminants(derive(EnumIter))]
enum Collidable {
    LineSegment(LineSegment2),
    Circle(Circle),
}

impl Collidable {
    fn colliding(&self, other: &Collidable) -> bool {
        macro_rules! match_collidables {
            ($match:expr, $name:ident, $expr:expr) => {
                match $match {
                    Collidable::LineSegment($name) => $expr,
                    Collidable::Circle($name) => $expr,
                }
            };
        }
        match_collidables!(self, a, match_collidables!(other, b, a.colliding(b)))
    }
}

fn setup(mut state: ResMut<State>) {
    *state = State::default();
}

fn draw(
    mut state: ResMut<State>,
    mut commands: Commands,
    mut debug_draw: ResMut<DebugDraw>,
    mut egui_context: ResMut<EguiContext>,
    label_query: Query<(Entity, &Label)>,
    transform_query: Query<&Transform>,
) {
    let State {
        setup,
        collidable_1,
        collidable_2,
    } = state.as_mut();

    let colliding = collidable_1.colliding(&collidable_2);
    let vertex_style = if colliding {
        *STYLE_VERTEX_COLLIDING
    } else {
        *STYLE_VERTEX
    };
    let shape_style = if colliding {
        *STYLE_SHAPE_COLLIDING
    } else {
        *STYLE_SHAPE
    };

    if *setup {
        for (entity, _) in label_query.iter() {
            commands.entity(entity).despawn_recursive();
        }
    }
    for (collidable_index, collidable) in [collidable_1, collidable_2].into_iter().enumerate() {
        let collidable_point_count = match collidable {
            Collidable::LineSegment(..) => 2,
            Collidable::Circle(..) => 2,
        };
        let mut collidable_point_names = vec![String::new(); collidable_point_count];
        for collidable_point_index in 0..collidable_point_count {
            collidable_point_names[collidable_point_index] =
                format!("{}_{}", collidable_index, collidable_point_index);
        }
        if *setup {
            for (collidable_point_index, collidable_point_name) in
                collidable_point_names.iter().enumerate()
            {
                let x = -100. + (collidable_index as f32) * 200.;
                let y = -40. + (collidable_point_index as f32) * 80.;
                commands.spawn((
                    TransformBundle::from_transform(Transform::from_xyz(x, y, 0.)),
                    Draggable::new(32.),
                    Label::new(&collidable_point_name),
                ));
            }
        } else {
            let collidable_points: Vec<Vec2> = collidable_point_names
                .iter()
                .map(|collidable_point_name| {
                    transform_query
                        .get(Label::find(&label_query, collidable_point_name))
                        .unwrap()
                        .translation
                        .truncate()
                })
                .collect();

            for collidable_point in collidable_points.iter() {
                Circle::new(*collidable_point, 10.).draw(debug_draw.as_mut(), vertex_style);
            }

            match collidable {
                Collidable::LineSegment(line_segment) => {
                    *line_segment = LineSegment2::new(collidable_points[0], collidable_points[1]);
                    line_segment.draw(debug_draw.as_mut(), shape_style);
                }
                Collidable::Circle(circle) => {
                    *circle = Circle::new(
                        collidable_points[0],
                        collidable_points[0].distance(collidable_points[1]).max(1.) * 2.,
                    );
                    circle.draw(debug_draw.as_mut(), shape_style);
                }
            }
        }
    }
    *setup = false;

    egui::Window::new("Shapes").show(egui_context.ctx_mut(), |ui| {
        let State {
            setup,
            collidable_1,
            collidable_2,
        } = state.as_mut();

        for (collidable_index, collidable) in [collidable_1, collidable_2].into_iter().enumerate() {
            let current_collidable_discriminant = CollidableDiscriminants::from(*collidable);
            let mut new_collidable_discriminant = current_collidable_discriminant;
            ComboBox::from_label(format!("Shape {}", collidable_index + 1))
                .selected_text(format!("{:?}", current_collidable_discriminant))
                .show_ui(ui, |ui| {
                    for collidable_discriminant in CollidableDiscriminants::iter() {
                        ui.selectable_value(
                            &mut new_collidable_discriminant,
                            collidable_discriminant,
                            format!("{:?}", collidable_discriminant),
                        );
                    }
                });
            if current_collidable_discriminant != new_collidable_discriminant {
                *setup = true;
                *collidable = match new_collidable_discriminant {
                    CollidableDiscriminants::LineSegment => {
                        Collidable::LineSegment(LineSegment2::new(Vec2::ZERO, Vec2::ZERO))
                    }
                    CollidableDiscriminants::Circle => {
                        Collidable::Circle(Circle::new(Vec2::ZERO, 1.))
                    }
                };
            }
        }
    });
}
