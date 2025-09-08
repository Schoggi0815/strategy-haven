use bevy::prelude::*;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, States, Default)]
pub enum MatchState {
    #[default]
    None,
    Setup,
    Running,
    FInished,
}
