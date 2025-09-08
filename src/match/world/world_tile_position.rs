use bevy::prelude::*;

#[derive(Component, Debug, PartialEq, Eq, Clone)]
pub struct WorldTilePosition {
    pub x: i16,
    pub y: i16,
}

impl WorldTilePosition {
    pub fn new(x: i16, y: i16) -> Self {
        Self { x, y }
    }

    pub fn neighbours(&self) -> [WorldTilePosition; 8] {
        [
            WorldTilePosition::new(self.x + 1, self.y),
            WorldTilePosition::new(self.x - 1, self.y),
            WorldTilePosition::new(self.x, self.y + 1),
            WorldTilePosition::new(self.x, self.y - 1),
            WorldTilePosition::new(self.x + 1, self.y + 1),
            WorldTilePosition::new(self.x - 1, self.y + 1),
            WorldTilePosition::new(self.x + 1, self.y - 1),
            WorldTilePosition::new(self.x - 1, self.y - 1),
        ]
    }
}
