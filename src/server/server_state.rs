use bevy::prelude::*;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, States, Default)]
pub enum ServerState {
    #[default]
    Off,
    Singleplayer,
    Host,
}
