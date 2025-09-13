pub mod main_menu;
pub mod r#match;

use bevy::prelude::*;
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
use bevy_ui_text_input::TextInputPlugin;
use itertools::Itertools;

use crate::{
    main_menu::{main_menu_plugin::MainMenuPlugin, main_menu_state::MainMenuState},
    r#match::{
        match_plugin::MatchPlugin,
        match_state::MatchState,
        world::{
            wfc::{
                pattern::Pattern, pattern_palette::PatternPalette, super_grid::SuperGrid,
                tile_grid::TileGrid,
            },
            world_tile_type::WorldTileType,
            world_tile_type_flags::WorldTileTypeFlags,
        },
    },
};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            MainMenuPlugin,
            MatchPlugin,
            EguiPlugin::default(),
            WorldInspectorPlugin::default(),
            PanOrbitCameraPlugin,
            TextInputPlugin,
        ))
        .add_systems(Startup, setup)
        .add_systems(OnEnter(MainMenuState::Hidden), start_match)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Transform::from_translation(Vec3::new(0.0, 1.5, 5.0)),
        PanOrbitCamera::default(),
    ));
}

fn start_match(mut match_state: ResMut<NextState<MatchState>>) {
    match_state.set(MatchState::Setup);
}
