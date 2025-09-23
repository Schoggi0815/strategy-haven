use std::fmt::Display;

use bevy::prelude::*;
use bevy_hookup_core::sendable_component::SendableComponent;
use serde::{Deserialize, Serialize};

use crate::networking::sendables::Sendables;

#[derive(
    Debug, Reflect, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
pub struct PlayerId(u64);

impl PlayerId {
    pub fn new_random() -> Self {
        Self(rand::random())
    }
}

impl SendableComponent<Sendables> for PlayerId {
    fn to_sendable(&self) -> Sendables {
        Sendables::PlayerId(*self)
    }

    fn from_sendable(sendable: Sendables) -> Option<Self> {
        match sendable {
            Sendables::PlayerId(id) => Some(id),
            _ => None,
        }
    }
}

impl Display for PlayerId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
