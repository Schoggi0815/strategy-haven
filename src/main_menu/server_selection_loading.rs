use bevy::prelude::*;

use crate::{
    main_menu::main_menu_state::MainMenuState, spacetime_db::spacetime_server::ServerConnection,
};

pub fn wait_for_spacetime(
    server_connection: Option<Res<ServerConnection>>,
    mut main_menu_state: ResMut<NextState<MainMenuState>>,
) {
    if server_connection.is_some() {
        main_menu_state.set(MainMenuState::GuildSelection);
    }
}
