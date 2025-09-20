use bevy::prelude::*;
use bevy_hookup_core::{
    hook_session::SessionMessenger, hookup_component_plugin::HookupComponentPlugin,
    hookup_sendable_plugin::HookupSendablePlugin, owner_component::Owner, shared::Shared,
};
use bevy_hookup_messenger_self::self_session::SelfSession;

use crate::{
    client::{client_plugin::ClientPlugin, client_state::ClientState},
    networking::{
        network_state::NetworkState, sendables::Sendables, world_tile_position::WorldTilePosition,
        world_tile_type::WorldTileType,
    },
    server::{server_plugin::ServerPlugin, server_state::ServerState},
};

pub struct NetworkingPlugin;

impl Plugin for NetworkingPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Owner<WorldTileType>>()
            .register_type::<Shared<WorldTileType>>()
            .register_type::<Owner<WorldTilePosition>>()
            .register_type::<Shared<WorldTilePosition>>()
            .init_state::<NetworkState>()
            .add_plugins((
                ClientPlugin,
                ServerPlugin,
                HookupSendablePlugin::<Sendables>::default(),
                HookupComponentPlugin::<Sendables, WorldTileType>::default(),
                HookupComponentPlugin::<Sendables, WorldTilePosition>::default(),
            ))
            .add_systems(OnEnter(NetworkState::Singleplayer), start_singleplayer);
    }
}

fn start_singleplayer(
    mut client_state: ResMut<NextState<ClientState>>,
    mut server_state: ResMut<NextState<ServerState>>,
    mut commands: Commands,
) {
    client_state.set(ClientState::Connected);
    server_state.set(ServerState::On);

    commands.spawn(SelfSession::<Sendables>::new().to_session());
}
