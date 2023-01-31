use bevy::prelude::*;
use bodge::{
    bevy::{Cursor, CursorPlugin, DebugDraw, DebugDrawPlugin, DebugDrawStyle, DebugDrawable},
    geometry::Circle,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(DebugDrawPlugin)
        .add_plugin(CursorPlugin)
        .add_startup_system(setup)
        .add_system(draw)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn draw(mut debug_draw: ResMut<DebugDraw>, cursor: Res<Cursor>) {
    Circle::new(cursor.position, 100.).draw(
        debug_draw.as_mut(),
        DebugDrawStyle {
            ..Default::default()
        },
    );
}
