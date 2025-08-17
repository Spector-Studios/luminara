use crate::Map;
use bracket_pathfinding::prelude::Point;
use input_lib::ButtonState;
use input_lib::Controller;
use macroquad::prelude::get_frame_time;

const INITIAL_DELAY: f32 = 0.25;
const REPEAT_DELAY: f32 = 0.10;

pub struct Cursor {
    pos: Point,
    timer: f32,
}

impl Cursor {
    pub fn new() -> Self {
        Self {
            pos: Point::zero(),
            timer: 0.0,
        }
    }

    pub fn get_pos(&self) -> Point {
        self.pos
    }

    fn shift(&mut self, delta: impl Into<Point>, map: &Map) {
        self.pos += delta.into();
        self.pos.x = self.pos.x.clamp(0, (map.width - 1).try_into().unwrap());
        self.pos.y = self.pos.y.clamp(0, (map.height - 1).try_into().unwrap());
    }

    pub fn update(&mut self, controller: &Controller, map: &Map) {
        let input = controller.button_state();
        if input == ButtonState::default() {
            self.timer = 0.0;
            return;
        }
        let delta = (input.dpad_x, -input.dpad_y);
        if input != controller.last_state() {
            self.shift(delta, map);
            self.timer = 0.0;
            return;
        }
        self.timer += get_frame_time();

        if self.timer > INITIAL_DELAY {
            if (self.timer - INITIAL_DELAY) % REPEAT_DELAY < get_frame_time() {
                self.shift(delta, map);
            }
        }
    }
}
