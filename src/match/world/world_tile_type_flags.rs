use bitflags::bitflags;

use crate::r#match::world::{world_tile_chances::WorldTileChances, world_tile_type::WorldTileType};

bitflags! {
    #[derive(PartialEq, Eq, Clone, Copy)]
    pub struct WorldTileTypeFlags: u8 {
        const Water = 1 << 2;
        const Field = 1 << 3;
        const Forest = 1 << 4;
        const Mountain = 1 << 5;
        const Beach = 1 << 6;
    }
}

impl WorldTileTypeFlags {
    pub fn get_tile_type(&self) -> WorldTileType {
        match *self {
            WorldTileTypeFlags::Water => WorldTileType::Water,
            WorldTileTypeFlags::Field => WorldTileType::Field,
            WorldTileTypeFlags::Mountain => WorldTileType::Mountain,
            WorldTileTypeFlags::Forest => WorldTileType::Forest,
            WorldTileTypeFlags::Beach => WorldTileType::Beach,
            _ => WorldTileType::Water,
        }
    }

    pub fn get_random(&self, chances: &WorldTileChances) -> Self {
        let all = self.iter().collect::<Vec<_>>();
        let chances = all
            .iter()
            .map(|flag| chances.get_matching_chance(*flag))
            .collect::<Vec<_>>();
        let total_chance: f32 = chances.iter().sum();
        let mut random = rand::random_range(0.0..total_chance);

        for (flag, chance) in all.iter().zip(chances.iter()) {
            if random < *chance {
                return *flag;
            }

            random -= chance;
        }

        return WorldTileTypeFlags::Water;
    }

    pub fn get_allowed(&self) -> WorldTileTypeFlags {
        self.iter()
            .map(|single| match single {
                WorldTileTypeFlags::Water => WorldTileTypeFlags::Water | WorldTileTypeFlags::Beach,
                WorldTileTypeFlags::Field => {
                    WorldTileTypeFlags::Field
                        | WorldTileTypeFlags::Forest
                        | WorldTileTypeFlags::Mountain
                        | WorldTileTypeFlags::Beach
                }
                WorldTileTypeFlags::Forest => {
                    WorldTileTypeFlags::Field | WorldTileTypeFlags::Forest
                }
                WorldTileTypeFlags::Mountain => {
                    WorldTileTypeFlags::Field | WorldTileTypeFlags::Mountain
                }
                WorldTileTypeFlags::Beach => {
                    WorldTileTypeFlags::Water
                        | WorldTileTypeFlags::Beach
                        | WorldTileTypeFlags::Field
                }
                _ => WorldTileTypeFlags::empty(),
            })
            .fold(WorldTileTypeFlags::empty(), |acc, x| acc | x)
    }
}

impl From<WorldTileType> for WorldTileTypeFlags {
    fn from(value: WorldTileType) -> Self {
        match value {
            WorldTileType::Water => Self::Water,
            WorldTileType::Field => Self::Field,
            WorldTileType::Forest => Self::Forest,
            WorldTileType::Mountain => Self::Mountain,
            WorldTileType::Beach => Self::Beach,
        }
    }
}
