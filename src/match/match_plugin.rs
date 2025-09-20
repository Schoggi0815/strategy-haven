use bevy::prelude::*;

use crate::r#match::match_state::MatchState;

pub struct MatchPlugin;

impl Plugin for MatchPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<MatchState>();
    }
}
