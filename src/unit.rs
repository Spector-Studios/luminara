use crate::assets::TextureHandle;
use crate::map::Terrain;
use crate::pathfinding::DijkstraMap;
use crate::world::Faction;

use std::ops::Deref;
use std::ops::DerefMut;

use bracket_pathfinding::prelude::Point;
use macroquad::prelude::Vec2;

#[derive(Clone, Copy, Debug)]
pub struct Unit {
    pub movement: u32,
    pub faction: Faction,
    pub health: i32,
    pub pos: Point,
    pub render_pos: Option<Vec2>,
    pub texture_handle: TextureHandle,
    pub weapon: Option<Weapon>,
}

impl Unit {
    pub fn get_movement_cost(&self, terrain: Terrain) -> u32 {
        match terrain {
            Terrain::Ground => 1,
            Terrain::Forest => 2,
            Terrain::Mountain => DijkstraMap::UNREACHABLE,
            Terrain::River => DijkstraMap::UNREACHABLE,
        }
    }
}

macro_rules! create_id {
    ($name: ident) => {
        #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
        pub struct $name(u32);
        impl Deref for $name {
            type Target = u32;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
        impl DerefMut for $name {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
        impl $name {
            pub fn new(id: u32) -> Self {
                Self(id)
            }

            pub fn next(&mut self) {
                self.0 += 1;
            }
        }
    };
}

create_id!(WeaponId);
create_id!(UnitId);

#[derive(Clone, Copy, Debug)]
pub struct Weapon {
    id: WeaponId,
}
