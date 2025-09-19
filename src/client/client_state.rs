use bevy::prelude::*;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, States, Default)]
pub enum ClientState {
    #[default]
    Off,
    Connecting,
    Connected,
}
