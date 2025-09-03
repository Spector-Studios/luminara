use super::player::PlayerSelect;
use std::collections::VecDeque;
use std::fmt::Debug;

use crate::cursor::Cursor;
use crate::game::GameContext;
use crate::math::Point;
use crate::unit::{Unit, UnitId};

#[derive(Debug)]
pub struct StateMachine {
    stack: Vec<Box<dyn GameState>>,
    msg_queue: VecDeque<GameMsg>,
    commands_buffer: Commands,
}
impl StateMachine {
    pub fn new(game_ctx: &GameContext) -> Self {
        Self {
            stack: vec![PlayerSelect::boxed_new(game_ctx)],
            msg_queue: VecDeque::new(),
            commands_buffer: Commands::new(),
        }
    }

    pub fn update(&mut self, game_ctx: &mut GameContext) {
        loop {
            game_ctx.controller.update();
            let transition = match self.stack.last_mut() {
                Some(state) => {
                    state.update(&mut self.msg_queue, game_ctx, &mut self.commands_buffer)
                }
                None => break,
            };

            self.commands_buffer
                .drain()
                .for_each(|command| match command {
                    Command::CommitUnit(unit) => {
                        game_ctx.world.units.insert(unit.id(), unit);
                    }
                    Command::SetupTurn => game_ctx.world.setup_turn(),
                    Command::FocusView(pt) => {
                        // TODO Animate this. Wait for it to finish before proceeding
                        // TODO May push an animation state?
                        game_ctx.render_context.update(pt);
                    }
                    Command::DamageUnit(id, dmg) => {
                        let unit = game_ctx.world.units.get_mut(&id).unwrap();
                        unit.curr_health -= dmg;
                    }
                });

            match transition {
                Transition::None => break,
                _ => self.apply_transition(transition),
            }
        }
    }

    pub fn render(&self, game_ctx: &GameContext) {
        // TODO Make a better API for following
        game_ctx.render_context.render_map(&game_ctx.world.map);
        game_ctx.controller.draw(None);

        let mut operating_unit = None;
        for state in self.stack.iter().rev() {
            if let Some(unit) = state.active_unit() {
                operating_unit = Some(unit);
                break;
            }
        }

        game_ctx
            .world
            .units
            .iter()
            .filter(|(_, unit)| game_ctx.render_context.in_bounds(unit.pos))
            .filter(|(id, _)| operating_unit.is_none_or(|unit| unit.id() != **id))
            .for_each(|(_, unit)| game_ctx.render_context.render_unit(unit));
        if let Some(unit) = operating_unit
            && game_ctx.render_context.in_bounds(unit.pos)
        {
            game_ctx.render_context.render_unit(unit);
        }

        self.stack.last().unwrap().render(game_ctx);
    }

    fn apply_transition(&mut self, transition: Transition) {
        match transition {
            Transition::None => {}
            Transition::Pop => {
                self.stack.pop();
            }
            Transition::Push(game_state) => self.stack.push(game_state),
            Transition::Switch(game_state) => {
                self.stack.pop();
                self.stack.push(game_state);
            }
            Transition::PopAllButFirst => {
                self.stack.truncate(1);
            }
        }
    }
}

#[derive(Debug)]
pub enum GameMsg {
    MoveAnimationDone(Unit),
    SetCursor(Cursor),
    SkillSelected,
}

#[derive(Debug)]
pub enum Command {
    CommitUnit(Unit),
    DamageUnit(UnitId, i32),
    SetupTurn,
    FocusView(Point),
}

pub trait GameState: Debug {
    fn update(
        &mut self,
        msg_queue: &mut VecDeque<GameMsg>,
        game_ctx: &GameContext,
        commands_buffer: &mut Commands,
    ) -> Transition;
    fn name(&self) -> &'static str;

    fn render(&self, _game_ctx: &GameContext) {}
    fn active_unit(&self) -> Option<&Unit> {
        None
    }
}

#[derive(Debug)]
pub enum Transition {
    None,
    Pop,
    Push(Box<dyn GameState>),
    Switch(Box<dyn GameState>),
    PopAllButFirst,
}

#[derive(Debug)]
pub struct Commands {
    queue: VecDeque<Command>,
}

impl Commands {
    pub fn new() -> Self {
        Self {
            queue: VecDeque::with_capacity(5),
        }
    }

    pub fn add(&mut self, command: Command) {
        self.queue.push_back(command);
    }

    fn drain(&mut self) -> impl Iterator<Item = Command> {
        self.queue.drain(0..)
    }
}
