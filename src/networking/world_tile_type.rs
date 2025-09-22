use bevy::prelude::*;
use bevy_hookup_core::sendable_component::SendableComponent;
use serde::{Deserialize, Serialize};

use crate::networking::sendables::Sendables;

#[derive(
    Component, Reflect, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize,
)]
pub enum WorldTileType {
    Water,
    Field,
    Forest,
    Mountain,
    Beach,
    Empty,
    Uncertain,
}

impl WorldTileType {
    pub fn get_color(&self) -> Color {
        match self {
            WorldTileType::Water => Color::linear_rgb(0., 0., 1.),
            WorldTileType::Field => Color::linear_rgb(0., 1., 0.),
            WorldTileType::Forest => Color::linear_rgb(0.25, 0.75, 0.),
            WorldTileType::Mountain => Color::linear_rgb(0.75, 0.75, 0.75),
            WorldTileType::Beach => Color::linear_rgb(0.75, 0.75, 0.),
            WorldTileType::Empty => Color::linear_rgb(1., 0., 0.),
            WorldTileType::Uncertain => Color::linear_rgb(0.1, 0.1, 0.1),
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            WorldTileType::Water => "Water",
            WorldTileType::Field => "Field",
            WorldTileType::Forest => "Forst",
            WorldTileType::Mountain => "Mount",
            WorldTileType::Beach => "Beach",
            WorldTileType::Empty => "Empty",
            WorldTileType::Uncertain => "Uncer",
        }
    }
}

impl SendableComponent<Sendables> for WorldTileType {
    fn to_sendable(&self) -> Sendables {
        Sendables::WorldTileType(*self)
    }

    fn from_sendable(sendable: Sendables) -> Option<Self> {
        match sendable {
            Sendables::WorldTileType(tile) => Some(tile),
            _ => None,
        }
    }
}
