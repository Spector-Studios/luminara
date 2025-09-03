use super::animation::MoveAnimation;
use super::simulated::SimulatedManager;
use super::state_machine::{Command, Commands, GameMsg, GameState, Transition};
use crate::cursor::Cursor;
use crate::game::GameContext;
use crate::math::Point;
use crate::pathfinding::{DijkstraMap, get_targetables};
use crate::ui::Menu;
use crate::unit::{Unit, UnitId};
use crate::world::Faction;

use std::collections::{HashSet, VecDeque};

use input_lib::Buttons;
use macroquad::color::{Color, WHITE};
use macroquad::logging::error;
use macroquad::prelude::warn;

#[derive(Debug)]
pub struct PlayerSelect {
    cursor: Cursor,
}

#[derive(Debug)]
struct PlayerMove {
    unit: Unit,
    dijkstra_map: DijkstraMap,
    cursor: Cursor,
}

#[derive(Debug)]
struct PlayerAction {
    targetables: HashSet<Point>,
    unit: Unit,
    menu: Menu,
    cursor: Cursor,
}

#[derive(Debug)]
struct PlayerAttck {
    unit: Unit,
    cursor: Cursor,
    targets: Vec<(UnitId, Point)>,
    selected: usize,
}

impl PlayerSelect {
    pub fn boxed_new(game_ctx: &GameContext) -> Box<Self> {
        let pt = game_ctx
            .world
            .units
            .iter()
            .find(|(_, unit)| unit.faction == Faction::Player)
            .map_or(Point::zero(), |(_, unit)| unit.pos);

        Box::new(Self {
            cursor: Cursor::new(pt, game_ctx.texture_store.get("cursor.png")),
        })
    }
}
impl GameState for PlayerSelect {
    fn update(
        &mut self,
        msg_queue: &mut VecDeque<GameMsg>,
        game_ctx: &GameContext,
        commands: &mut Commands,
    ) -> Transition {
        self.cursor
            .update(&game_ctx.controller, &game_ctx.world.map);
        commands.add(Command::FocusView(self.cursor.get_pos()));

        if let Some(msg) = msg_queue.pop_front() {
            match msg {
                GameMsg::SetCursor(cursor) => {
                    self.cursor = cursor;
                }
                _ => {
                    warn!("{} state should not receive msg: {:?}", self.name(), msg);
                }
            }
        }

        if game_ctx.world.get_unmoved_unit(Faction::Player).is_none() {
            commands.add(Command::SetupTurn);
            return Transition::Switch(Box::new(SimulatedManager::new(Faction::Enemy)));
        }

        if game_ctx.controller.clicked(Buttons::A)
            && let &Some(unit) = &game_ctx
                .world
                .get_unmoved_by_pos(Faction::Player, self.cursor.get_pos())
        {
            let dijkstra_map = DijkstraMap::new(&game_ctx.world.map, unit, &game_ctx.world.units);
            return Transition::Push(PlayerMove::boxed_new(
                unit.clone(),
                dijkstra_map,
                self.cursor.clone(),
            ));
        }

        Transition::None
    }

    fn render(&self, game_ctx: &GameContext) {
        game_ctx.render_context.render_sprite(
            self.cursor.get_pos(),
            &self.cursor.texture,
            WHITE,
            1.2,
        );
    }

    fn name(&self) -> &'static str {
        "Player Select"
    }
}

impl PlayerMove {
    pub fn boxed_new(unit: Unit, dijkstra_map: DijkstraMap, cursor: Cursor) -> Box<Self> {
        Box::new(Self {
            unit,
            dijkstra_map,
            cursor,
        })
    }
}
impl GameState for PlayerMove {
    fn active_unit(&self) -> Option<&Unit> {
        Some(&self.unit)
    }
    fn update(
        &mut self,
        msg_queue: &mut VecDeque<GameMsg>,
        game_ctx: &GameContext,
        commands: &mut Commands,
    ) -> Transition {
        if let Some(msg) = msg_queue.pop_front() {
            match msg {
                GameMsg::MoveAnimationDone(unit) => {
                    return Transition::Push(PlayerAction::boxed_new(unit, self.cursor.clone()));
                }
                _ => {
                    warn!("{} state should not receive msg: {:?}", self.name(), msg);
                }
            }
        }

        self.cursor
            .update(&game_ctx.controller, &game_ctx.world.map);
        commands.add(Command::FocusView(self.cursor.get_pos()));

        if game_ctx.controller.clicked(Buttons::B) {
            return Transition::Pop;
        }

        if game_ctx.controller.clicked(Buttons::A) {
            if self.unit.pos == self.cursor.get_pos() {
                return Transition::Push(PlayerAction::boxed_new(
                    self.unit.clone(),
                    self.cursor.clone(),
                ));
            }
            if self
                .dijkstra_map
                .get_reachables()
                .contains(&self.cursor.get_pos())
                && game_ctx
                    .world
                    .is_tile_empty(self.cursor.get_pos(), Some(self.unit.id()))
            {
                return Transition::Push(MoveAnimation::boxed_new(
                    self.unit.clone(),
                    self.dijkstra_map.get_path(self.cursor.get_pos()),
                ));
            }
        }

        Transition::None
    }

    fn render(&self, game_ctx: &GameContext) {
        self.dijkstra_map
            .get_reachables()
            .iter()
            .filter(|pt| game_ctx.render_context.in_bounds(**pt))
            .for_each(|pt| {
                game_ctx
                    .render_context
                    .render_tile_rectangle(*pt, Color::new(0.0, 0.0, 1.0, 0.4));
            });
        game_ctx.render_context.render_sprite(
            self.cursor.get_pos(),
            &self.cursor.texture,
            WHITE,
            1.2,
        );
    }

    fn name(&self) -> &'static str {
        "Player Move"
    }
}

impl PlayerAction {
    const ATTACK: &str = "Attack";
    const SKILL: &str = "Skill";
    const WAIT: &str = "Wait";

    pub fn boxed_new(unit: Unit, cursor: Cursor) -> Box<Self> {
        Box::new(Self {
            targetables: HashSet::new(),
            unit,
            cursor,
            menu: Menu::new(&[Self::ATTACK, Self::WAIT, Self::SKILL]),
        })
    }
}

// TODO Targetables
impl GameState for PlayerAction {
    fn active_unit(&self) -> Option<&Unit> {
        Some(&self.unit)
    }
    fn update(
        &mut self,
        msg_queue: &mut VecDeque<GameMsg>,
        game_ctx: &GameContext,
        commands: &mut Commands,
    ) -> Transition {
        self.menu.update(&game_ctx.controller);

        if game_ctx.controller.clicked(Buttons::B) {
            return Transition::Pop;
        }

        match self.menu.selected() {
            Self::WAIT => {
                self.targetables.clear();
                if game_ctx.controller.clicked(Buttons::A) {
                    self.unit.turn_complete = true;
                    commands.add(Command::CommitUnit(self.unit.clone()));
                    msg_queue.push_back(GameMsg::SetCursor(self.cursor.clone()));
                    return Transition::PopAllButFirst;
                }
            }
            Self::ATTACK => {
                get_targetables(&self.unit, &mut self.targetables);
                if game_ctx.controller.clicked(Buttons::A) {
                    let opposing_units: Vec<_> = game_ctx
                        .world
                        .units
                        .iter()
                        .filter(|(_, unit)| unit.faction == Faction::Enemy)
                        .filter(|(_, unit)| self.targetables.contains(&unit.pos))
                        .map(|(id, unit)| (*id, unit.pos))
                        .collect();
                    if !opposing_units.is_empty() {
                        return Transition::Push(PlayerAttck::boxed_new(
                            self.unit.clone(),
                            self.cursor.clone(),
                            opposing_units,
                        ));
                    }
                }
            }
            Self::SKILL => {
                // TODO Control from render() if this should render rather than clearing it
                self.targetables.clear();
            }
            _ => error!(
                "Unrecognised option: {} in state: {}",
                self.menu.selected(),
                self.name()
            ),
        }

        Transition::None
    }

    fn render(&self, game_ctx: &GameContext) {
        self.targetables
            .iter()
            .filter(|pt| game_ctx.render_context.in_bounds(**pt))
            .for_each(|pt| {
                game_ctx
                    .render_context
                    .render_tile_rectangle(*pt, Color::new(1.0, 0.0, 0.0, 0.4));
            });
        self.menu.render(&game_ctx.render_context);
    }

    fn name(&self) -> &'static str {
        "Player Action"
    }
}

impl PlayerAttck {
    pub fn boxed_new(unit: Unit, cursor: Cursor, targets: Vec<(UnitId, Point)>) -> Box<Self> {
        Box::new(Self {
            unit,
            cursor,
            targets,
            selected: 0,
        })
    }
}
impl GameState for PlayerAttck {
    fn update(
        &mut self,
        msg_queue: &mut VecDeque<GameMsg>,
        game_ctx: &GameContext,
        commands_buffer: &mut Commands,
    ) -> Transition {
        if game_ctx.controller.clicked(Buttons::B) {
            return Transition::Pop;
        }
        if game_ctx.controller.clicked(Buttons::A) {
            self.unit.turn_complete = true;
            commands_buffer.add(Command::DamageUnit(self.targets[self.selected].0, 3));
            commands_buffer.add(Command::CommitUnit(self.unit.clone()));

            self.cursor.set_pos(self.unit.pos);
            msg_queue.push_back(GameMsg::SetCursor(self.cursor.clone()));
            return Transition::PopAllButFirst;
        }
        let input = game_ctx.controller.timed_hold();

        if input.dpad_x > 0 || input.dpad_y > 0 {
            self.selected = (self.selected + 1) % self.targets.len();
        } else if input.dpad_x < 0 || input.dpad_y < 0 {
            self.selected = (self.selected + self.targets.len() - 1) % self.targets.len();
        }

        self.cursor.set_pos(self.targets[self.selected].1);
        Transition::None
    }

    fn render(&self, game_ctx: &GameContext) {
        game_ctx.render_context.render_sprite(
            self.cursor.get_pos(),
            &self.cursor.texture,
            WHITE,
            1.0,
        );
    }

    fn name(&self) -> &'static str {
        "Player Attack"
    }
}
