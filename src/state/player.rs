use std::collections::VecDeque;

use super::animation::MoveAnimation;
use super::simulated::SimulatedManager;
use crate::game::GameContext;
use crate::math::Point;
use crate::pathfinding::{DijkstraMap, get_targetables};
use crate::state::{GameMsg, GameState, Transition};
use crate::ui::Menu;
use crate::unit::Unit;
use crate::world::Faction;

use input_lib::Buttons;
use macroquad::color::{Color, WHITE};
use macroquad::logging::error;

#[derive(Debug)]
pub struct PlayerSelect;

#[derive(Debug)]
struct PlayerMove {
    unit: Unit,
    dijkstra_map: DijkstraMap,
}

#[derive(Debug)]
struct PlayerAction {
    targetables: Vec<Point>,
    unit: Unit,
    menu: Menu,
}

impl GameState for PlayerSelect {
    fn update(
        &mut self,
        msg_queue: &mut VecDeque<GameMsg>,
        game_ctx: &mut GameContext,
    ) -> Transition {
        game_ctx
            .cursor
            .update(&game_ctx.controller, &game_ctx.world.map);

        if let Some(msg) = msg_queue.pop_front() {
            if let GameMsg::CommitUnit(unit) = msg {
                game_ctx.world.units.insert(unit.id(), unit);
            } else {
                error!("{} state should not receive msg: {:?}", self.name(), msg);
                panic!("{} state should not receive msg: {:?}", self.name(), msg);
            }
        }

        if game_ctx.world.get_unmoved_unit(Faction::Player).is_none() {
            game_ctx.world.setup_turn();
            return Transition::Switch(Box::new(SimulatedManager::new(Faction::Enemy)));
        }

        if game_ctx.controller.clicked(Buttons::A)
            && let Some(unit) = game_ctx
                .world
                .get_unmoved_by_pos(Faction::Player, game_ctx.cursor.get_pos())
        {
            let dijkstra_map = DijkstraMap::new(&game_ctx.world.map, unit, &game_ctx.world.units);
            return Transition::Push(PlayerMove::boxed_new(unit, dijkstra_map));
        }

        Transition::None
    }

    fn render(&self, game_ctx: &GameContext) {
        game_ctx.render_context.render_sprite(
            game_ctx.cursor.get_pos(),
            game_ctx.cursor.texture,
            WHITE,
            1.2,
        );
    }

    fn name(&self) -> &'static str {
        "Player Select"
    }
}

impl PlayerMove {
    pub fn boxed_new(unit: Unit, dijkstra_map: DijkstraMap) -> Box<Self> {
        Box::new(Self { unit, dijkstra_map })
    }
}
impl GameState for PlayerMove {
    fn active_unit(&self) -> Option<Unit> {
        Some(self.unit)
    }
    fn update(
        &mut self,
        msg_queue: &mut VecDeque<GameMsg>,
        game_ctx: &mut GameContext,
    ) -> Transition {
        game_ctx
            .cursor
            .update(&game_ctx.controller, &game_ctx.world.map);

        if game_ctx.controller.clicked(Buttons::B) {
            return Transition::Pop;
        }

        if game_ctx.controller.clicked(Buttons::A) {
            if self.unit.pos == game_ctx.cursor.get_pos() {
                return Transition::Push(PlayerAction::boxed_new(self.unit));
            }
            if game_ctx
                .world
                .is_tile_empty(game_ctx.cursor.get_pos(), Some(self.unit.id()))
            {
                return Transition::Push(MoveAnimation::boxed_new(
                    self.unit,
                    self.dijkstra_map.get_path(game_ctx.cursor.get_pos()),
                ));
            }
        }

        if let Some(msg) = msg_queue.pop_front() {
            match msg {
                GameMsg::MoveAnimationDone(unit) => {
                    return Transition::Push(PlayerAction::boxed_new(unit));
                }

                GameMsg::CommitUnit(unit) => {
                    msg_queue.push_back(GameMsg::CommitUnit(unit));
                    return Transition::Pop;
                }
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
            game_ctx.cursor.get_pos(),
            game_ctx.cursor.texture,
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
    const WAIT: &str = "Wait";
    pub fn boxed_new(unit: Unit) -> Box<Self> {
        Box::new(Self {
            targetables: Vec::new(),
            unit,
            menu: Menu::new(&[Self::ATTACK, Self::WAIT]),
        })
    }
}

// TODO Targetables
impl GameState for PlayerAction {
    fn active_unit(&self) -> Option<Unit> {
        Some(self.unit)
    }
    fn update(
        &mut self,
        msg_queue: &mut VecDeque<GameMsg>,
        game_ctx: &mut GameContext,
    ) -> Transition {
        self.menu.update(game_ctx.controller.button_state());

        if game_ctx.controller.clicked(Buttons::B) {
            return Transition::Pop;
        }

        match self.menu.selected() {
            Self::WAIT => {
                self.targetables.clear();
                if game_ctx.controller.clicked(Buttons::A) {
                    self.unit.turn_complete = true;
                    msg_queue.push_back(GameMsg::CommitUnit(self.unit));
                    return Transition::Pop;
                }
            }
            Self::ATTACK => {
                self.targetables = get_targetables(self.unit);
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
