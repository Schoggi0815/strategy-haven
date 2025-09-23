use bevy::prelude::*;
use bevy_hookup_core::sendable_component::SendableComponent;
use serde::{Deserialize, Serialize};

use crate::networking::sendables::Sendables;

#[derive(
    Debug, Reflect, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
pub struct PlayerPrivateId(u64);

impl PlayerPrivateId {
    pub fn new_random() -> Self {
        Self(rand::random())
    }
}

impl SendableComponent<Sendables> for PlayerPrivateId {
    fn to_sendable(&self) -> Sendables {
        Sendables::PlayerPrivateId(*self)
    }

    fn from_sendable(sendable: Sendables) -> Option<Self> {
        match sendable {
            Sendables::PlayerPrivateId(id) => Some(id),
            _ => None,
        }
    }
}
