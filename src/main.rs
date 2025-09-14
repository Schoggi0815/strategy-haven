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
    let mut reference = TileGrid::<9, 11>::new_filled(WorldTileType::Water);
    for (x, y) in (2..9).cartesian_product(4..11) {
        reference.set(x, y, WorldTileType::Beach);
    }
    for (x, y) in (3..9).cartesian_product(5..11) {
        reference.set(x, y, WorldTileType::Field);
    }
    for (x, y) in (6..9).cartesian_product(7..11) {
        reference.set(x, y, WorldTileType::Forest);
    }
    for (x, y) in (6..9).cartesian_product(0..5) {
        reference.set(x, y, WorldTileType::Beach);
    }
    for (x, y) in (7..9).cartesian_product(1..6) {
        reference.set(x, y, WorldTileType::Field);
    }
    for (x, y) in (5..6).cartesian_product(9..11) {
        reference.set(x, y, WorldTileType::Forest);
    }
    // println!("{}", reference);
    // return;
    let patterns = reference.get_patterns_square::<3>();
    // patterns
    //     .iter()
    //     .enumerate()
    //     .for_each(|(i, p)| println!("Pattern {}:\n{}", i, p.to_grid()));
    // return;
    let pattern_palette = PatternPalette::new(
        patterns
            .into_iter()
            .map(|pattern| -> Box<dyn Pattern> { Box::new(pattern) })
            .collect(),
    );
    let mut super_grid = SuperGrid::<30, 60>::new_empty(pattern_palette);
    super_grid.set(3, 3, WorldTileTypeFlags::Beach);
    super_grid.collapse_grid();
    let new_grid = super_grid.to_tile_grid();
    println!("{}", new_grid);
}

fn main2() {
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
