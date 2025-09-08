use bevy::prelude::*;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, States, Default)]
pub enum WorldState {
    #[default]
    None,
    SpawningTiles,
    GeneratingTerrain,
    CleanupTerrain,
    ReplaceTiles,
    PlacingPlayers,
    Ready,
}
