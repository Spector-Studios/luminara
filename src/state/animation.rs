use super::state_machine::{Command, Commands, GameMsg, GameState, Transition};
use crate::assets::TextureStore;
use crate::math::Point;
use crate::render::RenderContext;
use crate::unit::Unit;
use crate::world::WorldState;

use input_lib::Controller;
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
        _world: &WorldState,
        _render_ctx: &mut RenderContext,
        _controller: &Controller,
        _texture_store: &TextureStore,
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
    pub fn boxed_new(dest: impl Into<Vec2>, render_ctx: &RenderContext) -> Option<Box<Self>> {
        const MARGIN: f32 = 2.0;
        const TOLERANCE: f32 = 1e-3;

        let dest = dest.into();
        let mut target_rect = render_ctx.map_view_rect;

        let min_x = dest.x - target_rect.w + MARGIN + 1.0;
        let max_x = dest.x - MARGIN;
        target_rect.x = target_rect.x.clamp(min_x, max_x);

        let min_y = dest.y - target_rect.h + MARGIN + 1.0;
        let max_y = dest.y - MARGIN;
        target_rect.y = target_rect.y.clamp(min_y, max_y);

        target_rect = render_ctx.get_clamped_map_viewport(target_rect);
        if (target_rect.x - render_ctx.map_view_rect.x).abs() < TOLERANCE
            && (target_rect.y - render_ctx.map_view_rect.y).abs() < TOLERANCE
        {
            return None;
        }

        Some(Box::new(Self { target_rect }))
    }
}
impl GameState for ShiftMapView {
    fn update(
        &mut self,
        _msg_queue: &mut VecDeque<GameMsg>,
        _commands_buffer: &mut Commands,
        _world: &WorldState,
        render_ctx: &mut RenderContext,
        _controller: &Controller,
        _texture_store: &TextureStore,
    ) -> Transition {
        const STEP_SIZE: f32 = 0.5;
        const SNAP_TOLERANCE: f32 = 1e-3;

        let map_rect = &mut render_ctx.map_view_rect;

        let dx = self.target_rect.x - map_rect.x;
        let dy = self.target_rect.y - map_rect.y;
        if dx.abs() > 0.0 || dy.abs() > 0.0 {
            let shift_x = dx.signum() * dx.abs().min(STEP_SIZE);
            let shift_y = dy.signum() * dy.abs().min(STEP_SIZE);

            map_rect.x += shift_x;
            map_rect.y += shift_y;

            if (map_rect.x - self.target_rect.x).abs() < SNAP_TOLERANCE {
                map_rect.x = self.target_rect.x;
            }
            if (map_rect.y - self.target_rect.y).abs() < SNAP_TOLERANCE {
                map_rect.y = self.target_rect.y;
            }
            return Transition::None;
        }

        Transition::Pop
    }

    fn name(&self) -> &'static str {
        "Shift Map View"
    }
}
