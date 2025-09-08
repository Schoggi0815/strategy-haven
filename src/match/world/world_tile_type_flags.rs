use bitflags::bitflags;

use crate::r#match::world::world_tile_type::WorldTileType;

bitflags! {
    #[derive(PartialEq, Eq, Clone, Copy)]
    pub struct WorldTileTypeFlags: u8 {
        const Water = 1 << 2;
        const Field = 1 << 3;
        const Forest = 1 << 4;
        const Mountain = 1 << 5;
    }
}

impl WorldTileTypeFlags {
    pub fn get_tile_type(&self) -> WorldTileType {
        match *self {
            WorldTileTypeFlags::Water => WorldTileType::Water,
            WorldTileTypeFlags::Field => WorldTileType::Field,
            WorldTileTypeFlags::Mountain => WorldTileType::Mountain,
            WorldTileTypeFlags::Forest => WorldTileType::Forest,
            _ => WorldTileType::Water,
        }
    }

    pub fn get_random(&self) -> Self {
        let all = self.iter().collect::<Vec<_>>();
        all[rand::random_range(0..all.len())]
    }

    pub fn restrict(&mut self, other: &Self) {
        match *other {
            WorldTileTypeFlags::Water => {
                self.remove(WorldTileTypeFlags::Forest | WorldTileTypeFlags::Mountain)
            }
            WorldTileTypeFlags::Forest => self.remove(WorldTileTypeFlags::Water),
            WorldTileTypeFlags::Mountain => self.remove(WorldTileTypeFlags::Water),
            _ => {}
        }
    }

    pub fn get_allowed(&self) -> WorldTileTypeFlags {
        self.iter()
            .map(|single| match single {
                WorldTileTypeFlags::Water => WorldTileTypeFlags::Water | WorldTileTypeFlags::Field,
                WorldTileTypeFlags::Field => WorldTileTypeFlags::all(),
                WorldTileTypeFlags::Forest => {
                    WorldTileTypeFlags::Field
                        | WorldTileTypeFlags::Forest
                        | WorldTileTypeFlags::Mountain
                }
                WorldTileTypeFlags::Mountain => {
                    WorldTileTypeFlags::Field
                        | WorldTileTypeFlags::Forest
                        | WorldTileTypeFlags::Mountain
                }
                _ => WorldTileTypeFlags::empty(),
            })
            .fold(WorldTileTypeFlags::empty(), |acc, x| acc | x)
    }
}
