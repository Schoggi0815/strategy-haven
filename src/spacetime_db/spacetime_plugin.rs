use bevy::{
    app::{App, Plugin},
    state::{app::AppExtStates, state::OnEnter},
};

use crate::spacetime_db::{
    spacetime_server::init_spacetime_server, spacetime_state::SpacetimeState,
};

pub struct SpacetimePlugin;

impl Plugin for SpacetimePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<SpacetimeState>().add_systems(
            OnEnter(SpacetimeState::Uninitialized),
            init_spacetime_server,
        );
    }
}
