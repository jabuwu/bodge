use bevy::prelude::*;
use bodge::{
    bevy::{Persistent, Scenes},
    prelude::*,
};

#[derive(Scenes, Default, Clone, Copy, PartialEq, Eq, Debug, Hash)]
enum AppScene {
    #[default]
    Scene1,
    Scene2,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_scenes::<AppScene>()
        .add_startup_system(setup)
        .add_system_set(SystemSet::on_enter(AppScene::Scene1).with_system(scene1_enter))
        .add_system_set(SystemSet::on_update(AppScene::Scene1).with_system(scene1_update))
        .add_system_set(SystemSet::on_enter(AppScene::Scene2).with_system(scene2_enter))
        .add_system_set(SystemSet::on_update(AppScene::Scene2).with_system(scene2_update))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), Persistent));
}

fn scene1_enter(mut commands: Commands) {
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::RED,
            custom_size: Some(Vec2::splat(100.)),
            ..Default::default()
        },
        transform: Transform::from_xyz(-100., 0., 0.),
        ..Default::default()
    });
}

fn scene1_update(mut app_state: ResMut<State<AppScene>>, mut keys: ResMut<Input<KeyCode>>) {
    if keys.just_pressed(KeyCode::Space) {
        keys.reset(KeyCode::Space);
        app_state.set(AppScene::Scene2).unwrap();
    }
}

fn scene2_enter(mut commands: Commands) {
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::BLUE,
            custom_size: Some(Vec2::splat(100.)),
            ..Default::default()
        },
        transform: Transform::from_xyz(100., 0., 0.),
        ..Default::default()
    });
}

fn scene2_update(mut app_state: ResMut<State<AppScene>>, mut keys: ResMut<Input<KeyCode>>) {
    if keys.just_pressed(KeyCode::Space) {
        keys.reset(KeyCode::Space);
        app_state.set(AppScene::Scene1).unwrap();
    }
}
