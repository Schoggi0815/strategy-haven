use itertools::Itertools;

use crate::r#match::world::{
    world_tile_type::WorldTileType, world_tile_type_flags::WorldTileTypeFlags,
};

pub trait Pattern {
    fn get_size(&self) -> [usize; 2];
    fn get_tile_type(&self, position: [usize; 2]) -> WorldTileType;
    fn get_tile_occurances(&self, type_flags: WorldTileTypeFlags) -> Vec<[usize; 2]> {
        let size = self.get_size();

        (0..size[0])
            .cartesian_product(0..size[1])
            .filter(|(x, y)| type_flags.contains(self.get_tile_type([*x, *y]).into()))
            .map(|(x, y)| [x, y])
            .collect_vec()
    }
}
