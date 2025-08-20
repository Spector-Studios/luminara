use crate::assets::TextureHandle;
use crate::math::Point;
use crate::math::Rect;

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
        *self.tiles.get(self.point_to_idx(pos.into())).unwrap()
    }

    pub fn get_terrain(&self, pos: impl Into<Point>) -> Terrain {
        *self.terrain.get(self.point_to_idx(pos.into())).unwrap()
    }

    pub fn get_texture_handle(&self, pos: impl Into<Point>) -> TextureHandle {
        *self.textures.get(self.point_to_idx(pos.into())).unwrap()
    }

    pub fn point_to_idx(&self, point: Point) -> usize {
        let (x, y) = (point.x, point.y);
        (TryInto::<usize>::try_into(y).unwrap() * self.width)
            + TryInto::<usize>::try_into(x).unwrap()
    }

    pub fn in_bounds(&self, pt: Point) -> bool {
        let map_rect = Rect::with_size(
            0,
            0,
            self.width.try_into().unwrap(),
            self.height.try_into().unwrap(),
        );
        map_rect.point_in_rect(pt)
    }
}

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
