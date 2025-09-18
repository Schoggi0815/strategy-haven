use std::{fs::File, io::Read, time::Instant};

use strategy_haven::r#match::world::{ms::ms_grid::MSGrid, tile_grid::TileGrid};

pub fn main() {
    let mut file = File::open("assets/wfc_preset.ron").expect("Could not open preset file!");
    let mut ron_string = String::new();
    file.read_to_string(&mut ron_string)
        .expect("Could not read file.");

    let reference: TileGrid = ron::from_str(&ron_string).expect("Could not parse file.");

    let mut ms_grid = MSGrid::from_tile_grid(&reference, 3, 3, 60, 60);

    let now = Instant::now();
    ms_grid.collapse_grid();
    println!("Grid collapsed in {} ms", now.elapsed().as_millis());
    println!("{}", ms_grid.to_tile_grid());
}
