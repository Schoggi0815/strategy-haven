use bevy::prelude::*;
use bevy_hookup_core::sendable_component::SendableComponent;
use serde::{Deserialize, Serialize};

use crate::networking::sendables::Sendables;

#[derive(Component, Reflect, Debug, Clone, Serialize, Deserialize)]
pub struct WorldTilePosition {
    pub x: u16,
    pub y: u16,
}

impl SendableComponent<Sendables> for WorldTilePosition {
    fn to_sendable(&self) -> Sendables {
        Sendables::WorldTilePosition(self.clone())
    }

    fn from_sendable(sendable: Sendables) -> Option<Self> {
        match sendable {
            Sendables::WorldTilePosition(position) => Some(position),
            _ => None,
        }
    }
}
