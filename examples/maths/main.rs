use bevy::prelude::*;
use bevy_egui::{egui, EguiContext, EguiPlugin};
use bodge::{
    bevy::{CursorPlugin, DebugDrawPlugin, EguiBlockInputPlugin, Persistent, Scenes},
    prelude::*,
};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use util::{DraggablePlugin, PointListBuilderPlugin, PolygonBuilderPlugin};

#[derive(Scenes, Default, Clone, Copy, PartialEq, Eq, Debug, Hash, EnumIter)]
enum AppScene {
    #[default]
    Menu,

    TriangleCentroid,
    TriangleCircumcenter,
    TriangleIncenter,

    TriangulationBowyerWatson,

    PolygonIsClockwise,
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_plugin(EguiBlockInputPlugin)
        .add_plugin(CursorPlugin)
        .add_plugin(DebugDrawPlugin)
        .add_plugin(DraggablePlugin)
        .add_plugin(PointListBuilderPlugin)
        .add_plugin(PolygonBuilderPlugin)
        .add_plugin(triangle_centroid::Plugin)
        .add_plugin(triangle_circumcenter::Plugin)
        .add_plugin(triangle_incenter::Plugin)
        .add_plugin(triangulation_bowyer_watson::Plugin)
        .add_plugin(polygon_is_clockwise::Plugin)
        .add_scenes::<AppScene>()
        .add_startup_system(setup)
        .add_system_set(SystemSet::on_update(AppScene::Menu).with_system(menu_update))
        .add_system(menu_on_escape)
        .add_system(window_title)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), Persistent));
}

#[derive(Default)]
struct MenuUpdate {
    next_scene: Option<AppScene>,
}

fn menu_update(
    mut local: Local<MenuUpdate>,
    mut app_scene: ResMut<State<AppScene>>,
    mut egui_context: ResMut<EguiContext>,
) {
    if let Some(next_scene) = local.next_scene {
        app_scene.set(next_scene).unwrap();
        local.next_scene = None;
    }
    egui::Window::new("Scenes").show(egui_context.ctx_mut(), |ui| {
        for scene in AppScene::iter() {
            if scene != AppScene::Menu {
                if ui.button(format!("{:?}", scene)).clicked() {
                    local.next_scene = Some(scene);
                }
            }
        }
    });
}

fn menu_on_escape(mut app_scene: ResMut<State<AppScene>>, mut keys: ResMut<Input<KeyCode>>) {
    if keys.just_pressed(KeyCode::Escape) {
        keys.reset(KeyCode::Space);
        let _ = app_scene.set(AppScene::Menu);
    }
}

fn window_title(mut windows: ResMut<Windows>, app_scene: Res<State<AppScene>>) {
    let window = windows.primary_mut();
    window.set_title(format!("{:?}", app_scene.current()));
}

mod polygon_is_clockwise;
mod triangle_centroid;
mod triangle_circumcenter;
mod triangle_incenter;
mod triangulation_bowyer_watson;
mod util;
