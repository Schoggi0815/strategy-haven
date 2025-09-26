use bevy::{
    app::{App, Plugin, Update},
    ecs::schedule::IntoScheduleConfigs,
    state::{
        app::AppExtStates,
        condition::in_state,
        state::{OnEnter, OnExit},
    },
};

use crate::main_menu::{
    main_menu_state::MainMenuState::{self, *},
    match_lobby::{
        despawn_match_lobby_ui, extend_with_player, extend_with_player_self, spawn_match_lobby_ui,
        update_player_name, update_player_name_self,
    },
    menu_resource::MenuResource,
    server_selection::{
        delete_server_selection, play_host, play_offline, read_server_selection_button_input,
        spawn_server_selection,
    },
};

pub struct MainMenuPlugin;

pub struct SpacetimeGuildPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<MainMenuState>()
            .init_resource::<MenuResource>()
            .add_systems(OnEnter(ServerSelection), spawn_server_selection)
            .add_systems(OnExit(ServerSelection), delete_server_selection)
            .add_systems(OnEnter(MatchLobby), spawn_match_lobby_ui)
            .add_systems(OnExit(MatchLobby), despawn_match_lobby_ui)
            .add_systems(
                Update,
                (read_server_selection_button_input, play_offline, play_host),
            )
            .add_systems(
                Update,
                (
                    extend_with_player,
                    extend_with_player_self,
                    update_player_name_self,
                    update_player_name,
                )
                    .run_if(in_state(MatchLobby)),
            );
    }
}
