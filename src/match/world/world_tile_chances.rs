use crate::r#match::world::world_tile_type_flags::WorldTileTypeFlags;

pub struct WorldTileChances {
    pub water: f32,
    pub field: f32,
    pub forest: f32,
    pub mountain: f32,
    pub beach: f32,
}

impl WorldTileChances {
    pub fn get_matching_chance(&self, flag: WorldTileTypeFlags) -> f32 {
        match flag {
            WorldTileTypeFlags::Water => self.water,
            WorldTileTypeFlags::Field => self.field,
            WorldTileTypeFlags::Forest => self.forest,
            WorldTileTypeFlags::Mountain => self.mountain,
            WorldTileTypeFlags::Beach => self.beach,
            _ => 0.,
        }
    }

    pub fn reduce_matching(&mut self, flag: WorldTileTypeFlags) {
        match flag {
            WorldTileTypeFlags::Water => self.water *= 0.9,
            WorldTileTypeFlags::Field => self.field *= 0.95,
            WorldTileTypeFlags::Forest => self.forest *= 0.8,
            WorldTileTypeFlags::Mountain => self.mountain *= 0.8,
            WorldTileTypeFlags::Beach => self.beach *= 0.9,
            _ => {}
        }
    }
}
