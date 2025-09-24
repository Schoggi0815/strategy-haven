use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Component, Reflect, Debug, Clone, Serialize, Deserialize)]
pub struct WorldTilePosition {
    pub x: u16,
    pub y: u16,
}
