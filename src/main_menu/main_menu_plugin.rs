use bevy::{
    app::{App, Plugin, Update},
    state::{
        app::AppExtStates,
        state::{OnEnter, OnExit},
    },
};

use crate::main_menu::{
    main_menu_state::MainMenuState::{self, *},
    server_selection::{
        delete_server_selection, play_offline, read_server_selection_button_input,
        spawn_server_selection,
    },
};

pub struct MainMenuPlugin;

pub struct SpacetimeGuildPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<MainMenuState>()
            .add_systems(OnEnter(ServerSelection), spawn_server_selection)
            .add_systems(OnExit(ServerSelection), delete_server_selection)
            .add_systems(Update, (read_server_selection_button_input, play_offline));
    }
}
