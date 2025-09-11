use itertools::Itertools;

use crate::r#match::world::{
    wfc::wfc_pattern::WfcPattern, world_tile_type_flags::WorldTileTypeFlags,
};

pub struct SuperGrid<const X_SIZE: usize, const Y_SIZE: usize> {
    grid: [[WorldTileTypeFlags; Y_SIZE]; X_SIZE],
    patterns: Vec<WfcPattern>,
}

impl<const X_SIZE: usize, const Y_SIZE: usize> SuperGrid<X_SIZE, Y_SIZE> {
    pub fn new_empty(patterns: Vec<WfcPattern>) -> Self {
        Self {
            grid: [[WorldTileTypeFlags::all(); Y_SIZE]; X_SIZE],
            patterns,
        }
    }

    pub fn get(&self, x: usize, y: usize) -> Option<WorldTileTypeFlags> {
        self.grid.get(x).and_then(|column| column.get(y)).copied()
    }

    pub fn collapse_grid(&mut self) {}

    fn recalculate_super_position(&mut self, position: [usize; 2]) {
        let allowed = WorldTileTypeFlags::empty();

        for (x_offset, y_offset) in (-1..2).cartesian_product(-1..2) {
            let x = position[0] as i32 + x_offset;
            let y = position[1] as i32 + y_offset;
            if x < 0 || x >= X_SIZE as i32 || y < 0 || y >= Y_SIZE as i32 {
                continue;
            }
            let x = x as usize;
            let y = y as usize;

            let slice_1 = &self.grid[x - 1][y - 1..y + 1];
            let slice_2 = &self.grid[x][y - 1..y + 1];
            let slice_3 = &self.grid[x + 1][y - 1..y + 1];
            let slices = &[slice_1, slice_2, slice_3];

            let pattern_x = (x_offset + 1) as usize;
            let pattern_y = (y_offset + 1) as usize;

            let allowed = self
                .patterns
                .iter()
                .filter(|pattern| pattern.pattern_fits(slices))
                .fold(WorldTileTypeFlags::empty(), |acc, pattern| {
                    acc | pattern.get_type_at(pattern_x, pattern_y).into()
                });
        }
    }
}
