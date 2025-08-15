use bracket_pathfinding::prelude::{Algorithm2D, Point, Rect};
use macroquad::prelude::vec2;
use macroquad::{
    color::{BLACK, WHITE},
    shapes::draw_rectangle_lines,
    text::draw_text,
    texture::{DrawTextureParams, draw_texture_ex},
    window::{screen_height, screen_width},
};

use crate::{
    assets::TextureStore,
    map::{Map, TileType},
};

// TODO Scale with window
const VIEWPORT_WIDTH: i32 = 15;
const VIEWPORT_HEIGHT: i32 = 10;

pub struct Viewport {
    view_rect: Rect,
    map_width: i32,
    map_height: i32,
}

impl Viewport {
    pub fn new(map_width: i32, map_height: i32) -> Self {
        let mut x = 0 - (VIEWPORT_WIDTH / 2);
        let mut y = 0 - (VIEWPORT_HEIGHT / 2);

        x = x.clamp(0, map_width - VIEWPORT_WIDTH);
        y = y.clamp(0, map_height - VIEWPORT_HEIGHT);

        Self {
            view_rect: Rect::with_size(x, y, VIEWPORT_WIDTH, VIEWPORT_HEIGHT),
            map_width,
            map_height,
        }
    }

    pub fn update(&mut self, cursor_pos: impl Into<Point>) {
        const MARGIN: i32 = 2;
        let cursor_pos = cursor_pos.into();

        if self.view_rect.x1 > cursor_pos.x - MARGIN {
            self.view_rect.x1 -= 1
        } else if self.view_rect.x2 < cursor_pos.x + MARGIN + 1 {
            self.view_rect.x1 += 1
        }
        if self.view_rect.y1 > cursor_pos.y - MARGIN {
            self.view_rect.y1 -= 1
        } else if self.view_rect.y2 < cursor_pos.y + MARGIN + 1 {
            self.view_rect.y1 += 1
        }

        self.view_rect.x1 = self.view_rect.x1.clamp(0, self.map_width - VIEWPORT_WIDTH);
        self.view_rect.y1 = self
            .view_rect
            .y1
            .clamp(0, self.map_height - VIEWPORT_HEIGHT);

        // INFO May be make a Rect.translate() method?
        self.view_rect.x2 = self.view_rect.x1 + VIEWPORT_WIDTH;
        self.view_rect.y2 = self.view_rect.y1 + VIEWPORT_HEIGHT;
    }

    pub fn map_view_rect(&self) -> &Rect {
        &self.view_rect
    }

    pub fn render(&self, map: &Map, texture_store: &TextureStore) {
        let params = DrawTextureParams {
            dest_size: Some(vec2(Self::tile_size(), Self::tile_size())),
            ..Default::default()
        };
        self.view_rect.for_each(|pt| {
            let texture = texture_store.get(map.get_texture_handle(pt));

            draw_texture_ex(
                texture,
                self.screen_x(pt.x),
                self.screen_y(pt.y),
                WHITE,
                params.clone(),
            );
            draw_rectangle_lines(
                self.screen_x(pt.x),
                self.screen_y(pt.y),
                Self::tile_size(),
                Self::tile_size(),
                5.0,
                BLACK,
            );
        });
    }

    pub fn screen_x(&self, tile_x: i32) -> f32 {
        ((tile_x - self.view_rect.x1) as f32) * Self::tile_size() + Self::offset_x()
    }

    pub fn screen_y(&self, tile_y: i32) -> f32 {
        ((tile_y - self.view_rect.y1) as f32) * Self::tile_size() + Self::offset_y()
    }

    fn offset_x() -> f32 {
        (screen_width() - (screen_width() * 0.99)) / 2.0
    }

    fn offset_y() -> f32 {
        (screen_height() - (screen_height() * 0.7)) / 2.0
    }

    pub fn tile_size() -> f32 {
        (screen_width() * 0.99) / VIEWPORT_WIDTH as f32
    }
}
