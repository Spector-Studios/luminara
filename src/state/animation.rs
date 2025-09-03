use super::state_machine::{Command, Commands, GameMsg, GameState, Transition};
use crate::game::GameContext;
use crate::math::Point;
use crate::unit::Unit;

use macroquad::time::get_frame_time;

use std::collections::VecDeque;

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
        _game_ctx: &GameContext,
        commands: &mut Commands,
    ) -> Transition {
        self.timer += get_frame_time();

        if self.timer >= 0.2 {
            if let Some(pos) = self.path.pop() {
                self.timer = 0.0;
                self.unit.pos = pos;
                commands.add(Command::FocusView(pos));
            } else {
                msg_queue.push_back(GameMsg::MoveAnimationDone(self.unit.clone()));
                return Transition::Pop;
            }
        }

        Transition::None
    }

    fn render(&self, _game_ctx: &GameContext) {}

    fn name(&self) -> &'static str {
        "Move Animation"
    }
}
