use bevy::prelude::*;
use bodge::bevy::Cursor;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub enum DraggableSystems {
    Update,
    Move,
}

pub struct DraggablePlugin;

impl Plugin for DraggablePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<DragState>()
            .add_system(draggable_update.label(DraggableSystems::Update))
            .add_system(
                draggable_move
                    .label(DraggableSystems::Move)
                    .after(DraggableSystems::Update),
            );
    }
}

#[derive(Resource, Default, Debug, Clone, Copy)]
pub struct DragState {
    pub dragging: Option<Entity>,
}

#[derive(Component, Clone, Copy, Debug, Default)]
pub struct Draggable {
    pub radius: f32,
    pub snapping: Option<f32>,
}

impl Draggable {
    pub fn new(radius: f32) -> Draggable {
        Draggable {
            radius,
            snapping: None,
        }
    }
}

fn draggable_update(
    mut drag_state: ResMut<DragState>,
    draggable_query: Query<(Entity, &Draggable, &GlobalTransform)>,
    cursor: Res<Cursor>,
    mouse_buttons: Res<Input<MouseButton>>,
) {
    if mouse_buttons.just_pressed(MouseButton::Left) {
        for (draggable_entity, draggable, draggable_transform) in draggable_query.iter() {
            let distance = draggable_transform
                .translation()
                .truncate()
                .distance(cursor.position);
            if distance < draggable.radius * 0.5 {
                drag_state.dragging = Some(draggable_entity);
                break;
            }
        }
    }
    if mouse_buttons.just_released(MouseButton::Left) {
        drag_state.dragging = None;
    }
}

fn draggable_move(
    mut draggable_query: Query<(&mut Transform, &Draggable)>,
    drag_state: Res<DragState>,
    cursor: Res<Cursor>,
) {
    if let Some(dragging) = drag_state.dragging {
        if let Some((mut draggable_transform, draggable)) = draggable_query.get_mut(dragging).ok() {
            let mut position = cursor.position;
            if let Some(snapping) = draggable.snapping {
                position = (position / snapping).round() * snapping;
            }
            draggable_transform.translation = position.extend(0.);
        }
    }
}
