use bevy::{
    app::{App, Plugin, Update},
    ecs::system::Res,
    state::{
        app::AppExtStates,
        state::{OnEnter, OnExit},
    },
};
use spacetimedb_sdk::Table;
use strategy_haven_derive::{SpacetimeChannelPlugin, SpacetimeChannelRegisterer};

use crate::{
    main_menu::{
        guild_selection::{on_enter_guild_selection, on_insert_guild},
        main_menu_state::MainMenuState::{self, *},
        server_selection::{
            delete_server_selection, read_server_selection_button_input, spawn_server_selection,
            update_server_info,
        },
        server_selection_loading::wait_for_spacetime,
    },
    module_bindings::GuildUserTableAccess,
    spacetime_db::spacetime_channel::{SpacetimeChannel, SpacetimeChannelRegisterer},
};

pub struct MainMenuPlugin;

#[derive(SpacetimeChannelPlugin, SpacetimeChannelRegisterer)]
#[entity(entity = "GuildUser", entity_db = "guild_user")]
pub struct SpacetimeGuildPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(SpacetimeGuildPlugin)
            .init_state::<MainMenuState>()
            .add_systems(OnEnter(ServerSelection), spawn_server_selection)
            .add_systems(OnExit(ServerSelection), delete_server_selection)
            .add_systems(
                Update,
                (
                    update_server_info,
                    read_server_selection_button_input,
                    wait_for_spacetime,
                ),
            )
            .add_systems(OnEnter(GuildSelection), on_enter_guild_selection)
            .add_systems(Update, on_insert_guild);
    }
}
