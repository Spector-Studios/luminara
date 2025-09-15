use super::state_machine::{Command, Commands, GameMsg, GameState, Transition};
use crate::game::GameCtxView;
use crate::math::{Point, TileRect};
use crate::render::Viewport;
use crate::unit::Unit;

use macroquad::math::Vec2;
use macroquad::prelude::Rect;
use macroquad::time::get_frame_time;

use std::collections::VecDeque;

const TICK_TIME: f32 = 0.15;

#[derive(Debug)]
pub struct MoveAnimation {
    timer: f32,
    unit: Unit,
    path: Vec<Point>,
}

#[derive(Debug)]
pub struct ShiftMapView {
    target_rect: Rect,
    target_tilerect: TileRect,
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
        commands: &mut Commands,
        _game_ctx: GameCtxView,
    ) -> Transition {
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

impl ShiftMapView {
    pub fn boxed_new(dest: impl Into<Point>, viewport: &Viewport) -> Option<Box<Self>> {
        const MARGIN: i32 = 2;

        assert!(viewport.render_view.is_none());
        let dest = dest.into();
        let mut target_rect = viewport.map_view;

        let min_x = dest.x - target_rect.w + MARGIN + 1;
        let max_x = dest.x - MARGIN;
        target_rect.x = target_rect.x.clamp(min_x, max_x);

        let min_y = dest.y - target_rect.h + MARGIN + 1;
        let max_y = dest.y - MARGIN;
        target_rect.y = target_rect.y.clamp(min_y, max_y);

        target_rect = viewport.clamp_tilerect_to_map(target_rect);

        // TODO Move this logic to the call site
        if target_rect == viewport.map_view {
            return None;
        }

        Some(Box::new(Self {
            target_rect: target_rect.into(),
            target_tilerect: target_rect,
        }))
    }
}
impl GameState for ShiftMapView {
    fn update(
        &mut self,
        _msg_queue: &mut VecDeque<GameMsg>,
        _commands_buffer: &mut Commands,
        game_ctx: GameCtxView,
    ) -> Transition {
        const STEP_SIZE: f32 = 0.2;
        const SNAP_TOLERANCE: f32 = 1e-3;

        let mut render_rect = game_ctx.viewport.get_render_rect();

        let dx = self.target_rect.x - render_rect.x;
        let dy = self.target_rect.y - render_rect.y;
        if dx.abs() > 0.0 || dy.abs() > 0.0 {
            let shift_x = dx.signum() * dx.abs().min(STEP_SIZE);
            let shift_y = dy.signum() * dy.abs().min(STEP_SIZE);

            render_rect.x += shift_x;
            render_rect.y += shift_y;

            if (render_rect.x - self.target_rect.x).abs() < SNAP_TOLERANCE {
                render_rect.x = self.target_rect.x;
            }
            if (render_rect.y - self.target_rect.y).abs() < SNAP_TOLERANCE {
                render_rect.y = self.target_rect.y;
            }

            game_ctx.viewport.render_view = Some(render_rect);
            return Transition::None;
        }

        game_ctx.viewport.map_view = self.target_tilerect;
        game_ctx.viewport.render_view = None;
        Transition::Pop
    }

    fn name(&self) -> &'static str {
        "Shift Map View"
    }
}
