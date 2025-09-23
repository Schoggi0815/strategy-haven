use bevy::prelude::*;
use bevy_hookup_core::sendable_component::SendableComponent;
use serde::{Deserialize, Serialize};

use crate::networking::sendables::Sendables;

#[derive(Debug, Reflect, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct PlayerName(pub String);

impl SendableComponent<Sendables> for PlayerName {
    fn to_sendable(&self) -> Sendables {
        Sendables::PlayerName(self.clone())
    }

    fn from_sendable(sendable: Sendables) -> Option<Self> {
        match sendable {
            Sendables::PlayerName(name) => Some(name),
            _ => None,
        }
    }
}
