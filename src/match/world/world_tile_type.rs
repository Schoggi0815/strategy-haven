use bevy::prelude::*;

pub const LAST_TILE_TYPE: WorldTileType = WorldTileType::Beach;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum WorldTileType {
    Water,
    Field,
    Forest,
    Mountain,
    Beach,
}

impl WorldTileType {
    pub fn get_color(&self) -> Color {
        match self {
            WorldTileType::Water => Color::linear_rgb(0., 0., 1.),
            WorldTileType::Field => Color::linear_rgb(0., 1., 0.),
            WorldTileType::Forest => Color::linear_rgb(0.25, 0.75, 0.),
            WorldTileType::Mountain => Color::linear_rgb(0.5, 0.5, 0.5),
            WorldTileType::Beach => Color::linear_rgb(0.75, 0.75, 0.),
        }
    }
}
