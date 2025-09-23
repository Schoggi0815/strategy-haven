use std::{fs::File, io::Read};

use bevy::prelude::*;
use bevy_hookup_core::{
    owner_component::Owner, session::Session, session_filter::SessionFilter, shared::Shared,
    sync_entity::SyncEntityOwner,
};

use crate::{
    common::{
        player_id::PlayerId, player_name::PlayerName, player_private_id::PlayerPrivateId,
        session_component::SessionComponent,
    },
    r#match::world::{ms::ms_grid::MSGrid, tile_grid::TileGrid},
    networking::{sendables::Sendables, world_tile_position::WorldTilePosition},
    server::server_state::ServerState,
};

pub struct ServerPlugin;

impl Plugin for ServerPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<ServerState>()
            // .add_systems(OnEnter(ServerState::On), spawn_world)
            .add_systems(
                Update,
                (add_player, rename_player).run_if(in_state(ServerState::On)),
            );
    }
}

fn add_player(
    sessions: Query<&Session<Sendables>, Added<Session<Sendables>>>,
    mut commands: Commands,
) {
    for session in sessions {
        let player_id = PlayerId::new_random();
        let player_private_id = PlayerPrivateId::new_random();
        info!("New player joined: {:?}", player_id);

        commands.spawn((
            SyncEntityOwner::new()
                .with_write_filter(SessionFilter::Whitelist(vec![session.get_session_id()])),
            Owner::new(player_id),
            Owner::new(PlayerName(format!("Player {}", player_id))),
            Owner::new(player_private_id)
                .with_read_filter(SessionFilter::Whitelist(vec![session.get_session_id()])),
            SessionComponent {
                session_id: session.get_session_id(),
            },
        ));
    }
}

fn rename_player(
    names: Query<
        (&Shared<PlayerName>, &mut Owner<PlayerName>),
        (Changed<Shared<PlayerName>>, With<SyncEntityOwner>),
    >,
) {
    for (new_name, mut name) in names {
        name.0 = new_name.0.clone();
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
