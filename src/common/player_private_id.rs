use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(
    Debug, Reflect, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
pub struct PlayerPrivateId(u64);

impl PlayerPrivateId {
    pub fn new_random() -> Self {
        Self(rand::random())
    }
}
