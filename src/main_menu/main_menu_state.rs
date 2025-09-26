use bevy::state::state::States;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, States, Default)]
pub enum MainMenuState {
    #[default]
    ServerSelection,
    SingleplayerLoading,
    JoinLoading,
    HostLoading,
    MatchLobby,
    Hidden,
}
