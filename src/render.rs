use std::f32;

use crate::map::Map;
use crate::math::{Point, clamp_rect_to_bounds};
use crate::unit::Unit;

use macroquad::prelude::vec2;
use macroquad::prelude::*;

const VIEWPORT_WIDTH: i32 = 1600;
const VIEWPORT_HEIGHT: i32 = 1200;

const VIEWPORT_TILES_WIDTH: i32 = 16;
const VIEWPORT_TILES_HEIGHT: i32 = 12;

const TILE_SIZE: i32 = VIEWPORT_WIDTH / VIEWPORT_TILES_WIDTH;

#[allow(clippy::cast_precision_loss)]
const VIEWPORT_TILES_WIDTH_F: f32 = VIEWPORT_TILES_WIDTH as f32;
#[allow(clippy::cast_precision_loss)]
const VIEWPORT_TILES_HEIGHT_F: f32 = VIEWPORT_TILES_HEIGHT as f32;
#[allow(clippy::cast_precision_loss)]
const TILE_SIZE_F: f32 = TILE_SIZE as f32;

#[derive(Debug)]
pub struct RenderContext {
    view_camera: Camera2D,
    screen_size: (f32, f32),
}

#[allow(clippy::cast_precision_loss, clippy::cast_possible_truncation)]
impl RenderContext {
    pub fn new() -> Self {
        let view_camera = Camera2D {
            offset: vec2(-1.0, 1.0),
            ..Default::default()
        };

        let mut render_context = Self {
            view_camera,
            screen_size: (1.0, 1.0),
        };

        render_context.resize_if_required();
        render_context
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

    pub fn render_map(map: &Map, viewport: &Viewport) {
        let view_rect = viewport.get_render_rect();

        let start_x = view_rect.left().floor().max(0.0) as i32;
        let start_y = view_rect.top().floor().max(0.0) as i32;

        let end_x = (view_rect.right().ceil()).min(viewport.map_width) as i32;
        let end_y = (view_rect.bottom().ceil()).min(viewport.map_height) as i32;

        for y in start_y..end_y {
            for x in start_x..end_x {
                let pt: Point = (x, y).into();
                Self::render_sprite(pt, map.get_texture_handle(pt), WHITE, 1.0, viewport);
            }
        }
    }

    pub fn render_sprite(
        pos: impl Into<Vec2>,
        texture: &Texture2D,
        color: Color,
        scale: f32,
        viewport: &Viewport,
    ) {
        // TODO Rewrite this
        let (mut x, mut y) = Self::screen_pos(pos, viewport.get_render_rect());
        let padding = ((1.0 - scale) / 2.0) * TILE_SIZE_F;
        (x, y) = (x + padding, y + padding);
        let params = DrawTextureParams {
            dest_size: Some(Vec2::splat(TILE_SIZE_F * scale)),
            ..Default::default()
        };

        draw_texture_ex(texture, x, y, color, params);
    }

    pub fn render_unit(unit: &Unit, viewport: &Viewport) {
        let color = if unit.turn_complete {
            Color::new(0.75, 0.75, 0.75, 1.0)
        } else {
            WHITE
        };
        Self::render_sprite(
            unit.render_pos.unwrap_or(unit.pos.into()),
            &unit.texture,
            color,
            1.0,
            viewport,
        );

        let (x, y) = Self::screen_pos(
            unit.render_pos.unwrap_or(unit.pos.into()),
            viewport.get_render_rect(),
        );
        let (w, h) = (TILE_SIZE_F * 0.9, TILE_SIZE_F * 0.2);
        let health_frac = (unit.curr_health as f32) / (unit.max_health as f32);
        draw_rectangle(x, y + TILE_SIZE_F, w, h, GRAY);
        draw_rectangle(x, y + TILE_SIZE_F, w * health_frac, h, RED);
    }

    pub fn render_tile_rectangle(
        pos: impl Into<Vec2>,
        color: Color,
        scale: f32,
        viewport: &Viewport,
    ) {
        let (mut x, mut y) = Self::screen_pos(pos, viewport.get_render_rect());
        let (w, h) = (TILE_SIZE_F * scale, TILE_SIZE_F * scale);

        x += (TILE_SIZE_F - w) / 2.0;
        y += (TILE_SIZE_F - h) / 2.0;
        draw_rectangle(x, y, w, h, color);
    }

    pub fn screen_pos(tile_pos: impl Into<Vec2>, render_rect: Rect) -> (f32, f32) {
        let tile_pos = tile_pos.into();
        (
            Self::screen_x(tile_pos.x, &render_rect),
            Self::screen_y(tile_pos.y, &render_rect),
        )
    }

    fn screen_x(tile_x: f32, render_rect: &Rect) -> f32 {
        (tile_x - render_rect.x) * TILE_SIZE_F
    }

    fn screen_y(tile_y: f32, render_rect: &Rect) -> f32 {
        (tile_y - render_rect.y) * TILE_SIZE_F
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

#[derive(Clone, Copy, PartialEq, Debug)]
enum ViewportMode {
    CenterOn { corner: Vec2 },
    Follow(Point),
    Free,
}
#[derive(Debug)]
pub struct Viewport {
    render_view: Rect,
    map_width: f32,
    map_height: f32,
    mode: ViewportMode,
}
impl Viewport {
    // TODO Make this a global constant from which all animation speeds are derived from
    pub const SHIFT_SPEED: f32 = 0.35;
    const MARGIN: f32 = 2.0;

    #[allow(clippy::cast_precision_loss)]
    pub fn new(map_width: i32, map_height: i32) -> Self {
        Self {
            render_view: Rect::new(0.0, 0.0, VIEWPORT_TILES_WIDTH_F, VIEWPORT_TILES_HEIGHT_F),
            map_width: map_width as f32,
            map_height: map_height as f32,
            mode: ViewportMode::Free,
        }
    }

    pub fn get_render_rect(&self) -> Rect {
        self.render_view
    }

    pub fn set_center_on(&mut self, pt: impl Into<Vec2>) {
        let pt = pt.into();
        let mut centered_rect = self.render_view;
        centered_rect.x = (pt.x - (VIEWPORT_TILES_WIDTH_F / 2.0)).round();
        centered_rect.y = (pt.y - (VIEWPORT_TILES_HEIGHT_F / 2.0)).round();
        centered_rect = clamp_rect_to_bounds(centered_rect, self.map_width, self.map_height);
        self.mode = ViewportMode::CenterOn {
            corner: centered_rect.point(),
        };
    }

    pub fn set_follow(&mut self, pt: impl Into<Point>) {
        self.mode = ViewportMode::Follow(pt.into());
    }

    pub fn update(&mut self) {
        match self.mode {
            ViewportMode::CenterOn { corner } => {
                let centered = self.center_point(corner);
                if centered {
                    self.mode = ViewportMode::Free;
                }
            }
            ViewportMode::Follow(point) => self.ensure_in_view(point),
            ViewportMode::Free => {}
        }
    }

    fn center_point(&mut self, corner: Vec2) -> bool {
        let mut delta = corner - self.render_view.point();
        delta = delta.clamp_length_max(Self::SHIFT_SPEED);
        self.render_view = self.render_view.offset(delta);

        (corner - self.render_view.point()).length_squared() < f32::EPSILON
    }

    fn ensure_in_view(&mut self, pt: Point) {
        let pt: Vec2 = pt.into();
        let center = self.render_view.center();

        let dx = if (pt.x - center.x).abs() > (VIEWPORT_TILES_WIDTH_F / 2.0 - Self::MARGIN) {
            pt.x - center.x
        } else {
            center.x.round() - center.x
        };

        let dy = if (pt.y - center.y).abs() > (VIEWPORT_TILES_HEIGHT_F / 2.0 - Self::MARGIN) {
            pt.y - center.y
        } else {
            center.y.round() - center.y
        };

        let delta = Vec2::new(dx, dy);

        if delta.length_squared() > 0.0 {
            let delta = delta.clamp_length_max(Self::SHIFT_SPEED);
            self.render_view = self.render_view.offset(delta);
            self.render_view =
                clamp_rect_to_bounds(self.render_view, self.map_width, self.map_height);
        }
    }

    pub fn is_centering(&self) -> bool {
        matches!(self.mode, ViewportMode::CenterOn { .. })
    }

    pub fn is_point_visible(&self, pt: impl Into<Vec2>) -> bool {
        let render_rect = self.get_render_rect();
        let pt = pt.into();

        pt.x >= render_rect.left().floor()
            && pt.x <= render_rect.right().ceil()
            && pt.y >= render_rect.top().floor()
            && pt.y <= render_rect.bottom().ceil()
    }
}

#[derive(Clone, Copy, Debug)]
pub struct RenderCtxWithViewport<'a> {
    pub viewport: &'a Viewport,
}
impl<'a> RenderCtxWithViewport<'a> {
    pub fn new(viewport: &'a Viewport) -> Self {
        Self { viewport }
    }

    pub fn is_tile_visible(self, pt: impl Into<Vec2>) -> bool {
        self.viewport.is_point_visible(pt)
    }

    pub fn render_tile_rectangle(self, pos: impl Into<Vec2>, color: Color, scale: f32) {
        RenderContext::render_tile_rectangle(pos, color, scale, self.viewport);
    }

    pub fn render_sprite(
        self,
        pos: impl Into<Vec2>,
        texture: &Texture2D,
        color: Color,
        scale: f32,
    ) {
        RenderContext::render_sprite(pos, texture, color, scale, self.viewport);
    }
}
