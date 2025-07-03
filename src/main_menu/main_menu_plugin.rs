use bevy::{
    app::{App, Plugin, Update},
    state::{
        app::AppExtStates,
        state::{OnEnter, OnExit},
    },
};

use crate::{
    main_menu::{
        guild_selection::{on_enter_guild_selection, on_insert_guild, register_guild_callbacks},
        main_menu_state::MainMenuState::{self, *},
        server_selection::{
            delete_server_selection, read_server_selection_button_input, spawn_server_selection,
            update_server_info,
        },
        server_selection_loading::wait_for_spacetime,
    },
    module_bindings::{Guild, RemoteTables, User, UserTableAccess, UserTableHandle},
    spacetime_db::{spacetime_channel::SpacetimeChannelPlugin, spacetime_state::SpacetimeState},
};

pub struct MainMenuPlugin;

fn user_callback(db: &RemoteTables) -> UserTableHandle {
    db.user()
}

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(SpacetimeChannelPlugin::new(|db| db.user()))
            .init_state::<MainMenuState>()
            .add_systems(
                OnEnter(SpacetimeState::Initialized),
                register_guild_callbacks,
            )
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
