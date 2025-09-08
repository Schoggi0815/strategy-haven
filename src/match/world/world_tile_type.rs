use bevy::prelude::*;

#[derive(Component, Debug)]
pub enum WorldTileType {
    Water,
    Field,
    Forest,
    Mountain,
}

impl WorldTileType {
    pub fn get_color(&self) -> Color {
        match self {
            WorldTileType::Water => Color::linear_rgb(0., 0., 1.),
            WorldTileType::Field => Color::linear_rgb(0., 1., 0.),
            WorldTileType::Forest => Color::linear_rgb(0.25, 0.75, 0.),
            WorldTileType::Mountain => Color::linear_rgb(0.5, 0.5, 0.5),
        }
    }
}
