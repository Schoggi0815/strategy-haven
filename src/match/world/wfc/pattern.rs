use crate::r#match::world::world_tile_type::WorldTileType;

pub trait Pattern {
    fn get_size(&self) -> [usize; 2];
    fn get_tile_type(&self, position: [usize; 2]) -> WorldTileType;
}
