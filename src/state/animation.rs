use super::state_machine::{Command, Commands, GameMsg, GameState, Transition};
use crate::game::GameContext;
use crate::math::Point;
use crate::unit::Unit;

use macroquad::math::Vec2;
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
        let step_time = 0.15;
        self.timer += get_frame_time();

        if self.path.is_empty() {
            self.unit.render_pos = None;
            msg_queue.push_back(GameMsg::MoveAnimationDone(self.unit.clone()));
            return Transition::Pop;
        }

        let curr_pos: Vec2 = self.unit.pos.into();
        let next_pos: Vec2 = (*self.path.last().unwrap()).into();
        // Normalised
        let progress = (self.timer / step_time).min(1.0);
        self.unit.render_pos = Some(curr_pos.lerp(next_pos, progress));

        if self.timer >= step_time {
            self.unit.pos = self.path.pop().unwrap();
            if let Some(pos) = self.path.last() {
                commands.add(Command::FocusView(*pos));
            }
            self.timer = 0.0;
        }

        Transition::None
    }

    fn name(&self) -> &'static str {
        "Move Animation"
    }
}
