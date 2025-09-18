use crate::math::Point;
use crate::prelude::Bounds2D;
use crate::render::Viewport;

use input_lib::Controller;
use macroquad::math::Vec2;
use macroquad::texture::Texture2D;

#[derive(Clone, Debug)]
pub struct Cursor {
    pos: Point,
    render_pos: Vec2,
    pub texture: Texture2D,
}

impl Cursor {
    pub fn new(pos: Point, texture: Texture2D) -> Self {
        Self {
            pos,
            render_pos: pos.into(),
            texture,
        }
    }

    pub fn get_pos(&self) -> Point {
        self.pos
    }

    pub fn get_render_pos(&self) -> Vec2 {
        self.render_pos
    }

    pub fn set_pos(&mut self, pos: impl Into<Point>) {
        self.pos = pos.into();
    }

    pub fn snap_to_pos(&mut self, pos: impl Into<Point>) {
        self.pos = pos.into();
        self.render_pos = self.pos.into();
    }

    fn shift(&mut self, delta: impl Into<Point>, bounds: &Bounds2D) {
        self.pos += delta.into();

        self.pos.x = self.pos.x.clamp(bounds.0.start, bounds.0.end - 1);
        self.pos.y = self.pos.y.clamp(bounds.1.start, bounds.1.end - 1);
    }

    pub fn update(&mut self, controller: &Controller, bounds: &Bounds2D) {
        let input = controller.timed_hold();
        let delta = (input.dpad_x, -input.dpad_y);
        self.shift(delta, bounds);
        self.update_render_pos();
    }

    fn update_render_pos(&mut self) {
        let mut delta = Into::<Vec2>::into(self.pos) - self.render_pos;
        delta = delta.clamp_length_max(Viewport::SHIFT_SPEED);
        self.render_pos += delta;
    }
}
