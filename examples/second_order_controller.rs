use bevy::prelude::*;
use bevy_egui::{
    egui::{self, Slider},
    EguiContext, EguiPlugin,
};
use bodge::{
    bevy::{
        Cursor, CursorPlugin, DebugDraw, DebugDrawPlugin, DebugDrawStyle, DebugDrawable,
        EguiBlockInputPlugin,
    },
    control::SecondOrder,
    geometry::Circle,
};

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_plugin(EguiBlockInputPlugin)
        .add_plugin(DebugDrawPlugin)
        .add_plugin(CursorPlugin)
        .add_startup_system(setup)
        .add_system(second_order_controller)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

struct SecondOrderController {
    controller: SecondOrder<Vec2>,
    target: Vec2,
    frequency: f32,
    response: f32,
    damping: f32,
}

impl Default for SecondOrderController {
    fn default() -> Self {
        let frequency = 1.;
        let response = 1.;
        let damping = 1.;
        Self {
            controller: SecondOrder::new_frequency_response(
                Vec2::ZERO,
                frequency,
                response,
                damping,
            ),
            target: Vec2::ZERO,
            frequency,
            response,
            damping,
        }
    }
}

fn second_order_controller(
    mut local: Local<SecondOrderController>,
    mut debug_draw: ResMut<DebugDraw>,
    mut egui_context: ResMut<EguiContext>,
    cursor: Res<Cursor>,
    time: Res<Time>,
    mouse_buttons: Res<Input<MouseButton>>,
) {
    let SecondOrderController {
        controller,
        target,
        frequency,
        response,
        damping,
    } = &mut *local;

    if mouse_buttons.pressed(MouseButton::Left) {
        *target = cursor.position;
    }

    let position = controller.update(*target, time.delta_seconds());

    Circle::new(position, 50.).draw(debug_draw.as_mut(), DebugDrawStyle::new(Color::WHITE));

    egui::Window::new("Debug").show(egui_context.ctx_mut(), |ui| {
        ui.label("Frequency");
        ui.add(Slider::new(frequency, 0.001..=3.0));
        ui.label("Response");
        ui.add(Slider::new(response, -2.0..=2.0));
        ui.label("Damping");
        ui.add(Slider::new(damping, 0.0..=2.0));
        controller.set_frequency_response(*frequency, *response, *damping);
    });
}
