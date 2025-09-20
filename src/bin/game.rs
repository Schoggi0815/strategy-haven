use bevy::prelude::*;
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};
use bevy_panorbit_camera::PanOrbitCameraPlugin;
use bevy_ui_text_input::TextInputPlugin;
use strategy_haven::{
    main_menu::{main_menu_plugin::MainMenuPlugin, main_menu_state::MainMenuState},
    networking::{network_state::NetworkState, networking_plugin::NetworkingPlugin},
};

#[derive(Component)]
struct MenuCam;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            MainMenuPlugin,
            EguiPlugin::default(),
            WorldInspectorPlugin::default(),
            PanOrbitCameraPlugin,
            NetworkingPlugin,
            TextInputPlugin,
        ))
        .add_systems(Startup, spawn_main_menu_camera)
        .add_systems(OnExit(MainMenuState::Hidden), spawn_main_menu_camera)
        .add_systems(OnEnter(MainMenuState::Hidden), remove_main_menu_camera)
        .add_systems(
            OnEnter(MainMenuState::SingleplayerLoading),
            start_singleplayer,
        )
        .run();
}

fn spawn_main_menu_camera(mut commands: Commands) {
    commands.spawn((Camera2d, MenuCam));
}

fn remove_main_menu_camera(camera: Single<Entity, With<MenuCam>>, mut commands: Commands) {
    commands.entity(camera.into_inner()).despawn();
}

fn start_singleplayer(
    mut menu_state: ResMut<NextState<MainMenuState>>,
    mut network_state: ResMut<NextState<NetworkState>>,
) {
    menu_state.set(MainMenuState::Hidden);
    network_state.set(NetworkState::Singleplayer);
}
