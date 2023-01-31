use bevy::{ecs::schedule::StateData, prelude::*};
pub use bodge_macros::Scenes;

pub trait Scenes {
    fn init_scene(app: &mut App);
}

pub trait AddScenes {
    fn add_scenes<T: Scenes + Default + StateData>(&mut self) -> &mut Self;
}

impl AddScenes for App {
    fn add_scenes<T: Scenes + Default + StateData>(&mut self) -> &mut Self {
        T::init_scene(self);
        self
    }
}

#[derive(Component)]
pub struct Persistent;

pub fn cleanup_non_persistent_entities(
    mut commands: Commands,
    query: Query<Entity, (Without<Persistent>, Without<Parent>)>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
