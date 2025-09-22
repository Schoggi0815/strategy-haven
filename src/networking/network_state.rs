use bevy::prelude::*;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, States, Default)]
pub enum NetworkState {
    #[default]
    None,
    Singleplayer,
    Host,
    Join,
}
