use crate::math::Point;
use crate::math::TileRect;
use crate::prelude::Bounds2D;

use macroquad::rand::ChooseRandom;
use macroquad::texture::Texture2D;

pub struct Map {
    terrain: Vec<Terrain>,
    textures: Vec<Texture2D>,
    pub width: usize,
    pub height: usize,
}

impl Map {
    fn empty(width: u32, height: u32) -> Self {
        assert!(width * height < u32::MAX);
        let capacity = (width * height) as usize;
        Self {
            terrain: Vec::with_capacity(capacity),
            textures: Vec::with_capacity(capacity),
            width: width as usize,
            height: height as usize,
        }
    }

    pub fn random(width: u32, height: u32, grass: &Texture2D, forest: &Texture2D) -> Self {
        let mut map = Self::empty(width, height);

        for _ in 0..width * height {
            match [Terrain::Ground, Terrain::Forest].choose().unwrap() {
                Terrain::Ground => {
                    map.terrain.push(Terrain::Ground);
                    map.textures.push(grass.clone());
                }
                Terrain::Forest => {
                    map.terrain.push(Terrain::Forest);
                    map.textures.push(forest.clone());
                }

                _ => {}
            }
        }

        for i in 0..width {
            let idx = map.point_to_idx((i, height - 1));
            map.textures[idx] = forest.clone();
        }

        map
    }

    pub fn get_terrain(&self, pos: impl Into<Point>) -> Terrain {
        *self.terrain.get(self.point_to_idx(pos.into())).unwrap()
    }

    pub fn get_texture_handle(&self, pos: impl Into<Point>) -> &Texture2D {
        self.textures.get(self.point_to_idx(pos.into())).unwrap()
    }

    pub fn get_bounds(&self) -> Bounds2D {
        (
            0..self.width.try_into().unwrap(),
            0..self.height.try_into().unwrap(),
        )
    }

    pub fn point_to_idx(&self, point: impl Into<Point>) -> usize {
        let point = point.into();
        let (x, y) = (point.x, point.y);
        (TryInto::<usize>::try_into(y).unwrap() * self.width)
            + TryInto::<usize>::try_into(x).unwrap()
    }

    pub fn in_bounds(&self, pt: Point) -> bool {
        let map_rect = TileRect::with_size(
            0,
            0,
            self.width.try_into().unwrap(),
            self.height.try_into().unwrap(),
        );
        map_rect.point_in_rect(pt)
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Terrain {
    Ground,
    Forest,
    Mountain,
    River,
}
