use bevy::prelude::*;
use bevy_egui::{egui, EguiContext, EguiPlugin};
use bodge::bevy::EguiBlockInputPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_plugin(EguiBlockInputPlugin)
        .add_system(test)
        .run();
}

fn test(
    mut egui_context: ResMut<EguiContext>,
    mut string: Local<String>,
    keys: Res<Input<KeyCode>>,
    mouse: Res<Input<MouseButton>>,
) {
    egui::Window::new("Hello").show(egui_context.ctx_mut(), |ui| {
        ui.text_edit_singleline(&mut *string);
    });
    if keys.just_pressed(KeyCode::A) {
        println!("pressed A!");
    }
    if mouse.just_pressed(MouseButton::Left) {
        println!("pressed left mouse button!");
    }
}
