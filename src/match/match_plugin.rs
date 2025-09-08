use bevy::prelude::*;

use crate::r#match::{
    match_state::MatchState,
    world::{world_plugin::WorldPlugin, world_state::WorldState},
};

pub struct MatchPlugin;

impl Plugin for MatchPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(WorldPlugin)
            .init_state::<MatchState>()
            .add_systems(OnEnter(MatchState::Setup), generate_world);
    }
}

fn generate_world(mut world_state: ResMut<NextState<WorldState>>) {
    world_state.set(WorldState::SpawningTiles);
}
