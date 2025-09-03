use crate::Map;
use crate::math::Point;

use input_lib::Controller;
use macroquad::texture::Texture2D;

#[derive(Clone, Debug)]
pub struct Cursor {
    pos: Point,
    pub texture: Texture2D,
}

impl Cursor {
    pub fn new(pos: Point, texture: Texture2D) -> Self {
        Self { pos, texture }
    }

    pub fn get_pos(&self) -> Point {
        self.pos
    }

    fn shift(&mut self, delta: impl Into<Point>, map: &Map) {
        self.pos += delta.into();

        // TODO Implement Ord on Point to make this one line
        self.pos.x = self.pos.x.clamp(0, (map.width - 1).try_into().unwrap());
        self.pos.y = self.pos.y.clamp(0, (map.height - 1).try_into().unwrap());
    }

    // TODO store map width and height instead of requiring full map
    // TODO require button state instead of full controller
    // TODO this timing logic should be moved to controller as it will be required by menus
    pub fn update(&mut self, controller: &Controller, map: &Map) {
        let input = controller.timed_hold();
        let delta = (input.dpad_x, -input.dpad_y);
        self.shift(delta, map);
    }
}
