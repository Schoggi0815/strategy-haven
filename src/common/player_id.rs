use std::fmt::Display;

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(
    Debug, Reflect, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
pub struct PlayerId(u64);

impl PlayerId {
    pub fn new_random() -> Self {
        Self(rand::random())
    }
}

impl Display for PlayerId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
