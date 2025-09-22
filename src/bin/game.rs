use bevy::prelude::*;
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};
use bevy_panorbit_camera::PanOrbitCameraPlugin;
use bevy_ui_text_input::TextInputPlugin;
use strategy_haven::{
    main_menu::{
        address_input::AddressInput, main_menu_plugin::MainMenuPlugin,
        main_menu_state::MainMenuState,
    },
    networking::{
        connection_details::ConnectionDetails, network_state::NetworkState,
        networking_plugin::NetworkingPlugin,
    },
};

#[derive(Component)]
struct MenuCam;

#[tokio::main]
async fn main() {
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
        .add_systems(OnEnter(MainMenuState::HostLoading), start_host)
        .add_systems(OnEnter(MainMenuState::JoinLoading), start_join)
        .add_systems(OnEnter(NetworkState::None), return_to_title_screen)
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

fn start_host(
    mut menu_state: ResMut<NextState<MainMenuState>>,
    mut network_state: ResMut<NextState<NetworkState>>,
) {
    network_state.set(NetworkState::Host);
    menu_state.set(MainMenuState::Hidden);
}

fn return_to_title_screen(mut menu_state: ResMut<NextState<MainMenuState>>) {
    menu_state.set(MainMenuState::ServerSelection);
}

fn start_join(
    mut menu_state: ResMut<NextState<MainMenuState>>,
    mut network_state: ResMut<NextState<NetworkState>>,
    server_adress: Res<AddressInput>,
    mut commands: Commands,
) {
    commands.insert_resource(ConnectionDetails {
        server_id: server_adress.into_inner().0.clone(),
    });

    menu_state.set(MainMenuState::Hidden);
    network_state.set(NetworkState::Join);
}
