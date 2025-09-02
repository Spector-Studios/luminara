use crate::map::Map;
use crate::math::Point;
use crate::math::TileRect;
use crate::unit::Unit;

use macroquad::prelude::vec2;
use macroquad::prelude::*;

// TODO Scale with window
const VIEWPORT_WIDTH: i32 = 15;
const VIEWPORT_HEIGHT: i32 = 10;

#[derive(Debug)]
pub struct RenderContext {
    view_rect: TileRect,
    map_width: i32,
    map_height: i32,
    tile_size: f32,
    offset_x: f32,
    offset_y: f32,
    screen_size: (f32, f32),
}

#[allow(clippy::cast_precision_loss)]
impl RenderContext {
    pub fn new(map_width: i32, map_height: i32) -> Self {
        let mut render_context = Self {
            view_rect: TileRect::with_size(0, 0, VIEWPORT_WIDTH, VIEWPORT_HEIGHT),
            map_width,
            map_height,
            tile_size: 0.0,
            offset_x: 0.0,
            offset_y: 0.0,
            screen_size: (1.0, 1.0),
        };

        render_context.update((0, 0));
        render_context
    }

    pub fn update(&mut self, cursor_pos: impl Into<Point>) {
        const MARGIN: i32 = 2;
        let cursor_pos = cursor_pos.into();

        if self.view_rect.x > cursor_pos.x - MARGIN {
            self.view_rect.x -= 1;
        } else if self.view_rect.x + self.view_rect.w < cursor_pos.x + MARGIN + 1 {
            self.view_rect.x += 1;
        }
        if self.view_rect.y > cursor_pos.y - MARGIN {
            self.view_rect.y -= 1;
        } else if self.view_rect.y + self.view_rect.h < cursor_pos.y + MARGIN + 1 {
            self.view_rect.y += 1;
        }

        self.view_rect.x = self.view_rect.x.clamp(0, self.map_width - VIEWPORT_WIDTH);
        self.view_rect.y = self.view_rect.y.clamp(0, self.map_height - VIEWPORT_HEIGHT);

        if (self.screen_size.0 - screen_width()).abs() > 1.0
            || (self.screen_size.1 - screen_height()).abs() > 1.0
        {
            self.resize();
        }
    }

    fn resize(&mut self) {
        const PADDING: f32 = 0.01;
        info!("Resize viewport");

        let sw = screen_width();
        let sh = screen_height();
        self.screen_size = (sw, sh);

        let drawable_w = sw * (1.0 - 2.0 * PADDING);
        let drawable_h = sh * (1.0 - 2.0 * PADDING);

        let tile_size_w = drawable_w / VIEWPORT_WIDTH as f32;
        let tile_size_h = drawable_h / VIEWPORT_HEIGHT as f32;
        self.tile_size = f32::min(tile_size_w, tile_size_h);

        let viewport_width_px = self.tile_size * VIEWPORT_WIDTH as f32;
        let viewport_height_px = self.tile_size * VIEWPORT_HEIGHT as f32;

        self.offset_x = (sw - viewport_width_px) / 2.0;

        if sw < sh {
            self.offset_y = sw * 0.3;
        } else {
            self.offset_y = (sh - viewport_height_px) / 2.0;
        }
    }

    pub fn map_view_rect(&self) -> &TileRect {
        &self.view_rect
    }

    pub fn view_size(&self) -> (f32, f32) {
        (
            VIEWPORT_WIDTH as f32 * self.tile_size,
            VIEWPORT_HEIGHT as f32 * self.tile_size,
        )
    }

    pub fn render_map(&self, map: &Map) {
        self.view_rect.for_each(|pt| {
            self.render_sprite(pt, map.get_texture_handle(pt), WHITE, 1.0);
        });
    }

    pub fn render_sprite(
        &self,
        pos: impl Into<Point>,
        texture: &Texture2D,
        color: Color,
        scale: f32,
    ) {
        // TODO Rewrite this
        let (mut x, mut y) = self.screen_pos(pos);
        let padding = ((1.0 - scale) / 2.0) * self.tile_size;
        (x, y) = (x + padding, y + padding);
        let params = DrawTextureParams {
            dest_size: Some(vec2(self.tile_size * scale, self.tile_size * scale)),
            ..Default::default()
        };

        draw_texture_ex(texture, x, y, color, params);
    }

    pub fn render_unit(&self, unit: &Unit) {
        let color = if unit.turn_complete { GRAY } else { WHITE };
        self.render_sprite(unit.pos, &unit.texture, color, 1.0);

        let (x, y) = self.screen_pos(unit.pos);
        let (w, h) = (self.tile_size * 0.9, self.tile_size * 0.2);
        let health_frac = (unit.curr_health as f32) / (unit.max_health as f32);
        draw_rectangle(x, y + self.tile_size, w, h, GRAY);
        draw_rectangle(x, y + self.tile_size, w * health_frac, h, RED);

        if unit.turn_complete {
            self.render_tile_rectangle(unit.pos, Color::new(0.2, 0.2, 0.2, 0.6));
        }
    }

    pub fn render_tile_rectangle(&self, pos: impl Into<Point>, color: Color) {
        let (x, y) = self.screen_pos(pos);
        let (w, h) = (self.tile_size, self.tile_size);
        draw_rectangle(x, y, w, h, color);
    }

    pub fn in_bounds(&self, pt: impl Into<Point>) -> bool {
        self.view_rect.point_in_rect(pt.into())
    }

    pub fn screen_pos(&self, tile_pos: impl Into<Point>) -> (f32, f32) {
        let tile_pos = tile_pos.into();
        (self.screen_x(tile_pos.x), self.screen_y(tile_pos.y))
    }

    fn screen_x(&self, tile_x: i32) -> f32 {
        ((tile_x - self.view_rect.x) as f32) * self.tile_size + self.offset_x
    }

    fn screen_y(&self, tile_y: i32) -> f32 {
        ((tile_y - self.view_rect.y) as f32) * self.tile_size + self.offset_y
    }

    pub fn offsets(&self) -> (f32, f32) {
        (self.offset_x, self.offset_y)
    }

    pub fn view_rect(&self) -> Rect {
        Rect {
            x: self.offset_x,
            y: self.offset_y,
            w: self.tile_size * VIEWPORT_WIDTH as f32,
            h: self.tile_size * VIEWPORT_HEIGHT as f32,
        }
    }
}
