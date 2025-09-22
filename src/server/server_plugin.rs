use std::{fs::File, io::Read};

use bevy::prelude::*;
use bevy_hookup_core::{owner_component::Owner, sync_entity::SyncEntityOwner};

use crate::{
    r#match::world::{ms::ms_grid::MSGrid, tile_grid::TileGrid},
    networking::world_tile_position::WorldTilePosition,
    server::server_state::ServerState,
};

pub struct ServerPlugin;

impl Plugin for ServerPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<ServerState>()
            .add_systems(OnEnter(ServerState::On), spawn_world);
    }
}

fn spawn_world(mut commands: Commands) {
    let mut file = File::open("assets/wfc_preset.ron").expect("Could not open preset file!");
    let mut ron_string = String::new();
    file.read_to_string(&mut ron_string)
        .expect("Could not read file.");

    let reference: TileGrid = ron::from_str(&ron_string).expect("Could not parse file.");

    let mut ms_grid = MSGrid::from_tile_grid(&reference, 3, 3, 40, 40);

    ms_grid.collapse_grid();

    info!("Collapsed Grid");

    let tile_grid = ms_grid.to_tile_grid();

    for (x, column) in tile_grid.data.iter().enumerate() {
        for (y, tile_type) in column.iter().enumerate() {
            commands.spawn((
                SyncEntityOwner::new(),
                Owner::new(WorldTilePosition {
                    x: x as u16,
                    y: y as u16,
                }),
                Owner::new(*tile_type),
            ));
        }
    }
}
