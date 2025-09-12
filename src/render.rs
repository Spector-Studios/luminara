use crate::map::Map;
use crate::math::Point;
use crate::math::TileRect;
use crate::unit::Unit;

use macroquad::prelude::vec2;
use macroquad::prelude::*;

const VIEWPORT_WIDTH: i32 = 1600;
const VIEWPORT_HEIGHT: i32 = 1200;

const VIEWPORT_TILES_WIDTH: i32 = 16;
const VIEWPORT_TILES_HEIGHT: i32 = 12;

const TILE_SIZE: i32 = VIEWPORT_WIDTH / VIEWPORT_TILES_WIDTH;
#[allow(clippy::cast_precision_loss)]
const TILE_SIZE_F: f32 = TILE_SIZE as f32;

#[derive(Debug)]
pub struct RenderContext {
    view_camera: Camera2D,
    map_view_rect: TileRect,
    map_width: i32,
    map_height: i32,
    screen_size: (f32, f32),
}

#[allow(clippy::cast_precision_loss, clippy::cast_possible_truncation)]
impl RenderContext {
    pub fn new(map_width: i32, map_height: i32) -> Self {
        let view_camera = Camera2D {
            offset: vec2(-1.0, 1.0),
            ..Default::default()
        };

        let mut render_context = Self {
            view_camera,
            map_view_rect: TileRect::with_size(0, 0, VIEWPORT_TILES_WIDTH, VIEWPORT_TILES_HEIGHT),
            map_width,
            map_height,
            screen_size: (1.0, 1.0),
        };

        render_context.resize_if_required();
        render_context.shift_viewport((0, 0));
        render_context
    }

    pub fn shift_viewport(&mut self, cursor_pos: impl Into<Point>) {
        const MARGIN: i32 = 2;
        let cursor_pos = cursor_pos.into();

        if self.map_view_rect.x > cursor_pos.x - MARGIN {
            self.map_view_rect.x -= 1;
        } else if self.map_view_rect.x + self.map_view_rect.w < cursor_pos.x + MARGIN + 1 {
            self.map_view_rect.x += 1;
        }
        if self.map_view_rect.y > cursor_pos.y - MARGIN {
            self.map_view_rect.y -= 1;
        } else if self.map_view_rect.y + self.map_view_rect.h < cursor_pos.y + MARGIN + 1 {
            self.map_view_rect.y += 1;
        }

        self.map_view_rect.x = self
            .map_view_rect
            .x
            .clamp(0, self.map_width - VIEWPORT_TILES_WIDTH);
        self.map_view_rect.y = self
            .map_view_rect
            .y
            .clamp(0, self.map_height - VIEWPORT_TILES_HEIGHT);
    }

    pub fn resize_if_required(&mut self) {
        const PADDING_FACTOR: f32 = 0.05;
        const Y_OFFSET: f32 = -0.2;

        let sw = screen_width();
        let sh = screen_height();
        if (self.screen_size.0 - sw).abs() < 1.0 && (self.screen_size.1 - sh).abs() < 1.0 {
            return;
        }

        info!("Resizing viewport");
        self.screen_size = (sw, sh);

        let scale =
            (1.0 - PADDING_FACTOR) * (sw / VIEWPORT_WIDTH as f32).min(sh / VIEWPORT_HEIGHT as f32);
        let rect_w = (VIEWPORT_WIDTH as f32 * scale) as i32;
        let rect_h = (VIEWPORT_HEIGHT as f32 * scale) as i32;
        let rect_x = (sw as i32 - rect_w) / 2;

        let desired_y = (sh as i32 - rect_h) / 2 - (sh * Y_OFFSET) as i32;
        let rect_y = desired_y.clamp(0, sh as i32 - rect_h);

        self.view_camera.viewport = Some((rect_x, rect_y, rect_w, rect_h));
        self.view_camera.zoom = vec2(2.0 / VIEWPORT_WIDTH as f32, 2.0 / VIEWPORT_HEIGHT as f32);
    }

    pub fn camera_ref(&self) -> &Camera2D {
        &self.view_camera
    }

    pub fn render_map(&self, map: &Map) {
        self.map_view_rect.for_each(|pt| {
            self.render_sprite(pt, map.get_texture_handle(pt), WHITE, 1.0);
        });
    }

    pub fn render_sprite(
        &self,
        pos: impl Into<Vec2>,
        texture: &Texture2D,
        color: Color,
        scale: f32,
    ) {
        // TODO Rewrite this
        let (mut x, mut y) = self.screen_pos(pos);
        let padding = ((1.0 - scale) / 2.0) * TILE_SIZE_F;
        (x, y) = (x + padding, y + padding);
        let params = DrawTextureParams {
            dest_size: Some(Vec2::splat(TILE_SIZE_F * scale)),
            ..Default::default()
        };

        draw_texture_ex(texture, x, y, color, params);
    }

    pub fn render_unit(&self, unit: &Unit) {
        let color = if unit.turn_complete {
            Color::new(0.75, 0.75, 0.75, 1.0)
        } else {
            WHITE
        };
        self.render_sprite(
            unit.render_pos.unwrap_or(unit.pos.into()),
            &unit.texture,
            color,
            1.0,
        );

        let (x, y) = self.screen_pos(unit.render_pos.unwrap_or(unit.pos.into()));
        let (w, h) = (TILE_SIZE_F * 0.9, TILE_SIZE_F * 0.2);
        let health_frac = (unit.curr_health as f32) / (unit.max_health as f32);
        draw_rectangle(x, y + TILE_SIZE_F, w, h, GRAY);
        draw_rectangle(x, y + TILE_SIZE_F, w * health_frac, h, RED);
    }

    pub fn render_tile_rectangle(&self, pos: impl Into<Vec2>, color: Color, scale: f32) {
        let (mut x, mut y) = self.screen_pos(pos);
        let (w, h) = (TILE_SIZE_F * scale, TILE_SIZE_F * scale);

        x += (TILE_SIZE_F - w) / 2.0;
        y += (TILE_SIZE_F - h) / 2.0;
        draw_rectangle(x, y, w, h, color);
    }

    pub fn in_bounds(&self, pt: impl Into<Point>) -> bool {
        self.map_view_rect.point_in_rect(pt.into())
    }

    pub fn screen_pos(&self, tile_pos: impl Into<Vec2>) -> (f32, f32) {
        let tile_pos = tile_pos.into();
        (self.screen_x(tile_pos.x), self.screen_y(tile_pos.y))
    }

    fn screen_x(&self, tile_x: f32) -> f32 {
        (tile_x - self.map_view_rect.x as f32) * TILE_SIZE_F
    }

    fn screen_y(&self, tile_y: f32) -> f32 {
        (tile_y - self.map_view_rect.y as f32) * TILE_SIZE_F
    }

    pub fn screen_view_rect() -> Rect {
        Rect {
            x: 0.0,
            y: 0.0,
            w: VIEWPORT_WIDTH as f32,
            h: VIEWPORT_HEIGHT as f32,
        }
    }
}
