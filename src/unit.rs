use crate::assets::TextureStore;
use crate::map::Terrain;
use crate::math::Point;
use crate::pathfinding::DijkstraMap;
use crate::world::Faction;

use std::ops::Deref;
use std::ops::DerefMut;

use macroquad::prelude::Vec2;
use macroquad::texture::Texture2D;

#[derive(Clone, Debug)]
pub struct Unit {
    id: UnitId,
    pub movement: u32,
    movement_class: MovementClass,
    pub turn_complete: bool,
    pub faction: Faction,
    pub curr_health: i32,
    pub max_health: i32,
    pub pos: Point,
    pub render_pos: Option<Vec2>,
    pub texture: Texture2D,
    pub weapon: Option<Weapon>,
}

impl Unit {
    pub fn from_erased(id: UnitId, erased: &ErasedUnit, texture_store: &TextureStore) -> Self {
        Self {
            id,
            movement: erased.movement,
            movement_class: erased.movement_class,
            turn_complete: erased.turn_complete,
            faction: erased.faction,
            curr_health: erased.curr_health,
            max_health: erased.max_health,
            pos: erased.pos,
            render_pos: None,
            texture: texture_store.get(&erased.texture_path),
            weapon: erased.weapon,
        }
    }

    #[allow(clippy::match_same_arms)]
    pub fn get_movement_cost(&self, terrain: Terrain) -> u32 {
        match (self.movement_class, terrain) {
            // Mounted in Forest
            (MovementClass::Mounted, Terrain::Forest) => 2,
            // Flying
            (MovementClass::Flying, _) => 1,
            // Infantary in Forest
            (MovementClass::Infantry, Terrain::Forest) => 1,
            // Normal Ground
            (_, Terrain::Ground) => 1,
            // Impassable
            (_, Terrain::Mountain | Terrain::River) => DijkstraMap::UNREACHABLE,
        }
    }

    pub fn id(&self) -> UnitId {
        self.id
    }
}

#[derive(Clone, Debug)]
pub struct ErasedUnit {
    pub movement: u32,
    pub movement_class: MovementClass,
    pub turn_complete: bool,
    pub faction: Faction,
    pub curr_health: i32,
    pub max_health: i32,
    pub pos: Point,
    pub texture_path: String,
    pub weapon: Option<Weapon>,
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

#[derive(Clone, Copy, Debug)]
pub enum MovementClass {
    Infantry,
    Mounted,
    Flying,
}
