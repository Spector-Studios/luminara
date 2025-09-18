use super::state_machine::{Commands, GameMsg, GameState, Transition};
use crate::game::GameCtxView;
use crate::math::Point;
use crate::unit::Unit;

use macroquad::math::Vec2;
use macroquad::time::get_frame_time;

use std::collections::VecDeque;

const TICK_TIME: f32 = 0.15;

#[derive(Debug)]
pub struct MoveAnimation {
    timer: f32,
    unit: Unit,
    path: Vec<Point>,
}

impl MoveAnimation {
    pub fn boxed_new(unit: Unit, path: Vec<Point>) -> Box<Self> {
        Box::new(Self {
            timer: 0.0,
            unit,
            path,
        })
    }
}

impl GameState for MoveAnimation {
    fn active_unit(&self) -> Option<&Unit> {
        Some(&self.unit)
    }

    fn update(
        &mut self,
        msg_queue: &mut VecDeque<GameMsg>,
        _commands: &mut Commands,
        game_ctx: GameCtxView,
    ) -> Transition {
        if game_ctx.viewport.is_centering() {
            return Transition::None;
        }
        self.timer += get_frame_time();

        if self.path.is_empty() {
            self.unit.render_pos = None;
            msg_queue.push_back(GameMsg::MoveAnimationDone(self.unit.clone()));
            return Transition::Pop;
        }

        let curr_pos: Vec2 = self.unit.pos.into();
        let next_pos: Vec2 = (*self.path.last().unwrap()).into();
        // Normalised
        let progress = (self.timer / TICK_TIME).min(1.0);
        self.unit.render_pos = Some(curr_pos.lerp(next_pos, progress));

        if self.timer >= TICK_TIME {
            self.unit.pos = self.path.pop().unwrap();
            if let Some(pos) = self.path.last() {
                game_ctx.viewport.set_follow(*pos);
            }
            self.timer = 0.0;
        }

        Transition::None
    }

    fn name(&self) -> &'static str {
        "Move Animation"
    }
}
