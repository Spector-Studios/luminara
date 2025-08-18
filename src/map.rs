use crate::assets::TextureHandle;

use bracket_pathfinding::prelude::{Algorithm2D, BaseMap, Point};
use macroquad::rand::ChooseRandom;

pub struct Map {
    tiles: Vec<TileType>,
    terrain: Vec<Terrain>,
    textures: Vec<TextureHandle>,
    pub width: usize,
    pub height: usize,
}

impl Map {
    fn empty(width: u32, height: u32) -> Self {
        assert!(width * height < u32::MAX);
        let capacity = (width * height) as usize;
        Self {
            tiles: Vec::with_capacity(capacity),
            terrain: Vec::with_capacity(capacity),
            textures: Vec::with_capacity(capacity),
            width: width as usize,
            height: height as usize,
        }
    }

    pub fn filled(width: u32, height: u32, grass: TextureHandle, forest: TextureHandle) -> Self {
        let mut map = Self::empty(width, height);

        for _ in 0..width * height {
            match [TileType::Ground, TileType::Forest].choose().unwrap() {
                TileType::Ground => {
                    map.tiles.push(TileType::Ground);
                    map.terrain.push(Terrain::Ground);
                    map.textures.push(grass);
                }
                TileType::Forest => {
                    map.tiles.push(TileType::Forest);
                    map.terrain.push(Terrain::Forest);
                    map.textures.push(forest);
                }
            }
        }

        map
    }

    pub fn get(&self, pos: impl Into<Point>) -> TileType {
        *self.tiles.get(self.point2d_to_index(pos.into())).unwrap()
    }

    pub fn get_terrain(&self, pos: impl Into<Point>) -> Terrain {
        *self.terrain.get(self.point2d_to_index(pos.into())).unwrap()
    }

    pub fn get_texture_handle(&self, pos: impl Into<Point>) -> TextureHandle {
        *self
            .textures
            .get(self.point2d_to_index(pos.into()))
            .unwrap()
    }
}

impl Algorithm2D for Map {
    fn dimensions(&self) -> Point {
        (self.width as i32, self.height as i32).into()
    }
}
impl BaseMap for Map {}

#[derive(Clone, Copy, Debug)]
pub enum TileType {
    Ground,
    Forest,
}

#[derive(Clone, Copy, Debug)]
pub enum Terrain {
    Ground,
    Forest,
    Mountain,
    River,
}
