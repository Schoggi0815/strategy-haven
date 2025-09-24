use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Reflect, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct PlayerName(pub String);
