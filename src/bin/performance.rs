use std::{fs::File, io::Read, time::Instant};

use strategy_haven::r#match::world::{
    tile_grid::TileGrid,
    wfc::{pattern_palette::PatternPalette, super_grid::SuperGrid},
    world_tile_type_flags::WorldTileTypeFlags,
};

fn main() {
    let mut file = File::open("assets/wfc_preset.ron").expect("Could not open preset file!");
    let mut ron_string = String::new();
    file.read_to_string(&mut ron_string)
        .expect("Could not read file.");

    let reference: TileGrid = ron::from_str(&ron_string).expect("Could not parse file.");
    println!("{}", reference);

    let patterns = reference.get_patterns::<3, 3>();

    patterns.iter().enumerate().for_each(|(i, p)| {
        println!(
            "Pattern {} occured {} times:\n{}",
            i,
            p.occurrence_count,
            p.to_grid()
        )
    });

    let pattern_palette = PatternPalette::new(patterns);
    let mut super_grid = SuperGrid::new_empty(pattern_palette, [50, 50]);
    super_grid.set(10, 10, WorldTileTypeFlags::Beach);

    let now = Instant::now();
    super_grid.collapse_grid();
    println!("Collapsed grid in {}ms", now.elapsed().as_millis());

    let new_grid = super_grid.to_tile_grid();
    println!("{}", new_grid);
}
