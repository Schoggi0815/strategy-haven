use bevy::prelude::*;
use bevy_hookup_core::{
    hook_session::SessionMessenger, hookup_component_plugin::HookupComponentPlugin,
    hookup_sendable_plugin::HookupSendablePlugin, owner_component::Owner, shared::Shared,
};
use bevy_hookup_messenger_self::self_session::SelfSession;
use bevy_hookup_messenger_websocket::{
    websocket_client::WebsocketClient, websocket_client_plugin::WebsocketClientPlugin,
    websocket_client_state::WebsocketClientState, websocket_server::WebsocketServer,
    websocket_server_plugin::WebsocketServerPlugin,
};

use crate::{
    client::{client_plugin::ClientPlugin, client_state::ClientState},
    common::{player_id::PlayerId, player_name::PlayerName, player_private_id::PlayerPrivateId},
    game::{world_tile_position::WorldTilePosition, world_tile_type::WorldTileType},
    networking::{
        connection_details::ConnectionDetails, network_state::NetworkState, sendables::Sendables,
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
            .register_type::<Owner<PlayerId>>()
            .register_type::<Shared<PlayerId>>()
            .register_type::<Owner<PlayerPrivateId>>()
            .register_type::<Shared<PlayerPrivateId>>()
            .register_type::<Owner<PlayerName>>()
            .register_type::<Shared<PlayerName>>()
            .init_state::<NetworkState>()
            .add_plugins((
                ClientPlugin,
                ServerPlugin,
                WebsocketServerPlugin::<Sendables>::default(),
                WebsocketClientPlugin::<Sendables>::default(),
                HookupSendablePlugin::<Sendables>::default(),
                HookupComponentPlugin::<Sendables, WorldTileType>::default(),
                HookupComponentPlugin::<Sendables, WorldTilePosition>::default(),
                HookupComponentPlugin::<Sendables, PlayerId>::default(),
                HookupComponentPlugin::<Sendables, PlayerPrivateId>::default(),
                HookupComponentPlugin::<Sendables, PlayerName>::default(),
            ))
            .add_systems(OnEnter(NetworkState::Singleplayer), start_singleplayer)
            .add_systems(OnEnter(NetworkState::Host), start_host)
            .add_systems(OnEnter(NetworkState::Join), start_join)
            .add_systems(
                Update,
                client_on_connect.run_if(in_state(NetworkState::Join)),
            );
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

fn start_host(
    mut client_state: ResMut<NextState<ClientState>>,
    mut server_state: ResMut<NextState<ServerState>>,
    mut commands: Commands,
    _marker: Option<NonSend<NonSendMarker>>,
) {
    client_state.set(ClientState::Connected);
    server_state.set(ServerState::On);

    commands.spawn(SelfSession::<Sendables>::new().to_session());
    commands.spawn(WebsocketServer::<Sendables>::new("0.0.0.0:3546".into()));
}

fn start_join(
    mut client_state: ResMut<NextState<ClientState>>,
    connection_details: Res<ConnectionDetails>,
    mut commands: Commands,
    _marker: Option<NonSend<NonSendMarker>>,
) {
    client_state.set(ClientState::Connecting);

    commands.spawn((
        WebsocketClient::<Sendables>::new_with_host_and_port(
            connection_details.server_id.clone(),
            3546,
        ),
        WebsocketClientState::default(),
    ));
}

fn client_on_connect(
    ws_state: Single<(Entity, &WebsocketClientState), Changed<WebsocketClientState>>,
    mut client_state: ResMut<NextState<ClientState>>,
    mut network_state: ResMut<NextState<NetworkState>>,
    mut commands: Commands,
) {
    let (entity, ws_state) = ws_state.into_inner();

    match ws_state {
        WebsocketClientState::Connected => {
            client_state.set(ClientState::Connected);
        }
        WebsocketClientState::Failed => {
            client_state.set(ClientState::Off);
            network_state.set(NetworkState::None);

            commands.entity(entity).despawn();
        }
        _ => {}
    }
}
