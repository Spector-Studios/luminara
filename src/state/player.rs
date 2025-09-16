use super::animation::MoveAnimation;
use super::simulated::SimulatedManager;
use super::state_machine::{Command, Commands, GameMsg, GameState, Transition};
use crate::assets::TextureStore;
use crate::cursor::Cursor;
use crate::game::GameCtxView;
use crate::math::Point;
use crate::pathfinding::{DijkstraMap, get_manahattan_neighbours};
use crate::render::RenderCtxWithViewport;
use crate::ui::{Menu, MenuItem};
use crate::unit::{Unit, UnitId};
use crate::world::{Faction, WorldState};

use std::collections::{HashMap, HashSet, VecDeque};

use input_lib::Buttons;
use macroquad::color::{BLUE, Color, RED, WHITE};
use macroquad::prelude::warn;

const MARKER_SCALE: f32 = 0.95;

#[derive(Debug)]
pub struct PlayerSelect {
    player_units: HashMap<Point, UnitId>,
    enemy_units: HashMap<Point, UnitId>,
    cursor: Cursor,
}

#[derive(Debug)]
struct PlayerMove {
    unit: Unit,
    dijkstra_map: DijkstraMap,
    targetables: Vec<Point>,
    cursor: Cursor,
}

#[derive(Debug)]
struct PlayerAction {
    targetables: HashSet<Point>,
    unit: Unit,
    menu: Menu<PossibleActions>,
    cursor: Cursor,
}

#[derive(Debug)]
struct PlayerAttack {
    unit: Unit,
    cursor: Cursor,
    targets: Vec<(UnitId, Point)>,
    selected: usize,
}

impl PlayerSelect {
    pub fn boxed_new(world: &WorldState, texture_store: &TextureStore) -> Box<Self> {
        let pt = world
            .units
            .iter()
            .find(|(_, unit)| unit.faction == Faction::Player)
            .map_or(Point::zero(), |(_, unit)| unit.pos);

        let mut state = Self {
            player_units: HashMap::new(),
            enemy_units: HashMap::with_capacity(10),
            cursor: Cursor::new(pt, texture_store.get("cursor.png")),
        };
        state.update_data(world);

        Box::new(state)
    }
    fn update_data(&mut self, world: &WorldState) {
        self.player_units.clear();
        self.player_units = world
            .units
            .iter()
            .filter(|(_, unit)| unit.faction == Faction::Player)
            .filter(|(_, unit)| !unit.turn_complete)
            .map(|(id, unit)| (unit.pos, *id))
            .collect::<HashMap<Point, UnitId>>();

        self.enemy_units.clear();
        self.enemy_units = world
            .units
            .iter()
            .filter(|(_, unit)| unit.faction == Faction::Enemy)
            .map(|(id, unit)| (unit.pos, *id))
            .collect();
    }
}
impl GameState for PlayerSelect {
    fn on_enter(&self, _game_ctx: GameCtxView) {
        _game_ctx.viewport.set_center_on(self.cursor.get_pos());
    }
    fn update(
        &mut self,
        msg_queue: &mut VecDeque<GameMsg>,
        commands: &mut Commands,
        game_ctx: GameCtxView,
    ) -> Transition {
        // TODO May be drain this instead?
        if let Some(msg) = msg_queue.pop_front() {
            match msg {
                GameMsg::SetCursor(cursor) => {
                    self.cursor = cursor;
                }
                GameMsg::WorldUpdated => {
                    self.update_data(game_ctx.world);
                }
                GameMsg::MoveAnimationDone(_) => {
                    warn!("{} state should not receive msg: {:?}", self.name(), msg);
                }
            }
        }
        if game_ctx.viewport.is_centering() {
            return Transition::None;
        }

        self.cursor.update(game_ctx.controller, &game_ctx.world.map);
        game_ctx.viewport.set_follow(self.cursor.get_pos());

        if game_ctx.world.get_unmoved_unit(Faction::Player).is_none() {
            commands.add(Command::SetupTurn);
            return Transition::Switch(Box::new(SimulatedManager::new(Faction::Enemy)));
        }

        // TODO Show enemy range
        if game_ctx.controller.clicked(Buttons::A)
            && let Some(unit_id) = self.player_units.get(&self.cursor.get_pos())
        {
            let unit = game_ctx.world.units.get(unit_id).unwrap();
            let dijkstra_map = DijkstraMap::new(&game_ctx.world.map, unit, &game_ctx.world.units);
            return Transition::Push(PlayerMove::boxed_new(
                unit.clone(),
                dijkstra_map,
                self.cursor.clone(),
            ));
        }

        Transition::None
    }

    fn render_ui_layer(&self, render_ctx: &RenderCtxWithViewport) -> Option<()> {
        render_ctx.render_sprite(self.cursor.get_pos(), &self.cursor.texture, WHITE, 1.2);

        Some(())
    }

    fn name(&self) -> &'static str {
        "Player Select"
    }
}

impl PlayerMove {
    pub fn boxed_new(unit: Unit, dijkstra_map: DijkstraMap, cursor: Cursor) -> Box<Self> {
        let reachables = dijkstra_map.get_reachables();
        let edge_tiles: Vec<_> = reachables
            .iter()
            .copied()
            .filter(|pt| {
                DijkstraMap::DIRS
                    .iter()
                    .any(|d_point| !reachables.contains(&(*pt + *d_point)))
            })
            .collect();

        let mut targetables = HashSet::new();
        for tile in edge_tiles {
            targetables.extend(get_manahattan_neighbours(tile, unit.get_attack_range()));
        }
        targetables.retain(|pt| !reachables.contains(pt));

        Box::new(Self {
            unit,
            dijkstra_map,
            targetables: targetables.into_iter().collect(),
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
        commands: &mut Commands,
        game_ctx: GameCtxView,
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

        self.cursor.update(game_ctx.controller, &game_ctx.world.map);
        game_ctx.viewport.set_follow(self.cursor.get_pos());

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
                && game_ctx.world.is_tile_empty(self.cursor.get_pos())
            {
                return Transition::Push(MoveAnimation::boxed_new(
                    self.unit.clone(),
                    self.dijkstra_map.get_path_to(self.cursor.get_pos()),
                ));
            }
        }

        Transition::None
    }

    fn render_map_overlay(&self, render_ctx: &RenderCtxWithViewport) -> Option<()> {
        self.dijkstra_map
            .get_reachables()
            .iter()
            .filter(|pt| render_ctx.is_tile_visible(**pt))
            .for_each(|pt| {
                render_ctx.render_tile_rectangle(*pt, Color { a: 0.4, ..BLUE }, MARKER_SCALE);
            });

        self.targetables
            .iter()
            .filter(|pt| render_ctx.is_tile_visible(**pt))
            .for_each(|pt| {
                render_ctx.render_tile_rectangle(*pt, Color { a: 0.4, ..RED }, MARKER_SCALE);
            });

        Some(())
    }

    fn render_ui_layer(&self, render_ctx: &RenderCtxWithViewport) -> Option<()> {
        render_ctx.render_sprite(self.cursor.get_pos(), &self.cursor.texture, WHITE, 1.2);

        Some(())
    }

    fn name(&self) -> &'static str {
        "Player Move"
    }
}

impl PlayerAction {
    pub fn boxed_new(unit: Unit, cursor: Cursor) -> Box<Self> {
        Box::new(Self {
            targetables: HashSet::new(),
            unit,
            cursor,
            menu: Menu::new(&[
                PossibleActions::Attack,
                PossibleActions::Skill,
                PossibleActions::Wait,
            ]),
        })
    }
}

impl GameState for PlayerAction {
    fn active_unit(&self) -> Option<&Unit> {
        Some(&self.unit)
    }
    fn update(
        &mut self,
        msg_queue: &mut VecDeque<GameMsg>,
        commands: &mut Commands,
        game_ctx: GameCtxView,
    ) -> Transition {
        self.menu.update(game_ctx.controller);

        if game_ctx.controller.clicked(Buttons::B) {
            return Transition::Pop;
        }

        match self.menu.selected() {
            PossibleActions::Wait => 'wait: {
                self.targetables.clear();
                if !game_ctx.controller.clicked(Buttons::A) {
                    break 'wait;
                }
                self.unit.turn_complete = true;
                commands.add(Command::CommitUnit(self.unit.clone()));
                msg_queue.push_back(GameMsg::SetCursor(self.cursor.clone()));
                return Transition::PopAllButFirst;
            }
            PossibleActions::Attack => 'attack: {
                self.targetables.clear();
                self.targetables.extend(get_manahattan_neighbours(
                    self.unit.pos,
                    self.unit.get_attack_range(),
                ));
                if !game_ctx.controller.clicked(Buttons::A) {
                    break 'attack;
                }
                let opposing_units: Vec<(UnitId, Point)> = game_ctx
                    .world
                    .units
                    .iter()
                    .filter(|(_, unit)| unit.faction == Faction::Enemy)
                    .filter(|(_, unit)| self.targetables.contains(&unit.pos))
                    .map(|(id, unit)| (*id, unit.pos))
                    .collect();

                // TODO Attack option shouldn't be shown if this is empty
                if opposing_units.is_empty() {
                    break 'attack;
                }
                return Transition::Push(PlayerAttack::boxed_new(
                    self.unit.clone(),
                    self.cursor.clone(),
                    opposing_units,
                ));
            }
            PossibleActions::Skill => {
                // TODO Control from render() if this should render rather than clearing it
                self.targetables.clear();
            }
        }

        Transition::None
    }

    fn render_map_overlay(&self, render_ctx: &RenderCtxWithViewport) -> Option<()> {
        self.targetables
            .iter()
            .filter(|pt| render_ctx.is_tile_visible(**pt))
            .for_each(|pt| {
                render_ctx.render_tile_rectangle(*pt, Color { a: 0.4, ..RED }, MARKER_SCALE);
            });

        Some(())
    }

    fn render_ui_layer(&self, _render_ctx: &RenderCtxWithViewport) -> Option<()> {
        self.menu.render();

        Some(())
    }

    fn name(&self) -> &'static str {
        "Player Action"
    }
}

impl PlayerAttack {
    pub fn boxed_new(unit: Unit, cursor: Cursor, targets: Vec<(UnitId, Point)>) -> Box<Self> {
        Box::new(Self {
            unit,
            cursor,
            targets,
            selected: 0,
        })
    }
}
impl GameState for PlayerAttack {
    fn update(
        &mut self,
        msg_queue: &mut VecDeque<GameMsg>,
        commands_buffer: &mut Commands,
        game_ctx: GameCtxView,
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

    fn render_ui_layer(&self, render_ctx: &RenderCtxWithViewport) -> Option<()> {
        render_ctx.render_sprite(self.cursor.get_pos(), &self.cursor.texture, WHITE, 1.0);

        Some(())
    }

    fn name(&self) -> &'static str {
        "Player Attack"
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum PossibleActions {
    Attack,
    Skill,
    Wait,
}

impl MenuItem for PossibleActions {
    fn menu_label(&self) -> &str {
        match self {
            Self::Attack => "Attack",
            Self::Skill => "Skill",
            Self::Wait => "Wait",
        }
    }
}
