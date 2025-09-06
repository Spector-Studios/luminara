use super::state_machine::{Command, Commands, GameMsg, GameState, Transition};
use crate::game::GameContext;
use crate::pathfinding::DijkstraMap;
use crate::state::animation::MoveAnimation;
use crate::state::player::PlayerSelect;
use crate::unit::Unit;
use crate::world::Faction;

use macroquad::logging::warn;
use macroquad::rand::ChooseRandom;

use std::collections::VecDeque;

#[derive(Debug)]
pub struct SimulatedManager {
    faction: Faction,
}

#[derive(Debug)]
pub struct MoveSimulated {
    unit: Unit,
    dijkstra_map: DijkstraMap,
}

#[derive(Debug)]
pub struct ActionSimulated {
    unit: Unit,
}

impl SimulatedManager {
    pub fn new(faction: Faction) -> Self {
        Self { faction }
    }
}

impl GameState for SimulatedManager {
    fn update(
        &mut self,
        msg_queue: &mut VecDeque<GameMsg>,
        game_ctx: &GameContext,
        commands: &mut Commands,
    ) -> Transition {
        if let Some(msg) = msg_queue.pop_front() {
            warn!("{} state should not receive msg: {:?}", self.name(), msg);
        }
        if let Some(unit) = game_ctx.world.get_unmoved_unit(self.faction) {
            let dijkstra_map = DijkstraMap::new(&game_ctx.world.map, unit, &game_ctx.world.units);
            commands.add(Command::FocusView(unit.pos));
            return Transition::Push(MoveSimulated::boxed_new(unit.clone(), dijkstra_map));
        }

        commands.add(Command::SetupTurn);
        Transition::Switch(PlayerSelect::boxed_new(game_ctx))
    }

    fn name(&self) -> &'static str {
        "Simulated Manager"
    }
}

impl MoveSimulated {
    pub fn boxed_new(unit: Unit, dijkstra_map: DijkstraMap) -> Box<Self> {
        Box::new(Self { unit, dijkstra_map })
    }
}

impl GameState for MoveSimulated {
    fn active_unit(&self) -> Option<&Unit> {
        Some(&self.unit)
    }
    fn update(
        &mut self,
        msg_queue: &mut VecDeque<GameMsg>,
        _game_ctx: &GameContext,
        _commands: &mut Commands,
    ) -> Transition {
        if let Some(msg) = msg_queue.pop_front() {
            match msg {
                // TODO May be pass the next state to Animation and have it handle transition?
                GameMsg::MoveAnimationDone(unit) => {
                    return Transition::Push(ActionSimulated::boxed_new(unit));
                }
                GameMsg::SetCursor(_) => {
                    warn!("{} state should not receive msg: {:?}", self.name(), msg);
                }
            }
        }

        // TODO May be use IndexSet for more efficient random choosing
        let vec_reachables = self
            .dijkstra_map
            .get_reachables()
            .iter()
            .collect::<Vec<_>>();
        let dest = vec_reachables.choose().unwrap();
        let path = self.dijkstra_map.get_path(**dest);

        Transition::Push(MoveAnimation::boxed_new(self.unit.clone(), path))
    }

    fn name(&self) -> &'static str {
        "Move Simulated"
    }
}

impl ActionSimulated {
    pub fn boxed_new(unit: Unit) -> Box<Self> {
        Box::new(Self { unit })
    }
}

impl GameState for ActionSimulated {
    fn active_unit(&self) -> Option<&Unit> {
        Some(&self.unit)
    }

    fn update(
        &mut self,
        _msg_queue: &mut VecDeque<GameMsg>,
        _game_ctx: &GameContext,
        commands: &mut Commands,
    ) -> Transition {
        self.unit.turn_complete = true;
        commands.add(Command::CommitUnit(self.unit.clone()));
        Transition::PopAllButFirst
    }

    fn name(&self) -> &'static str {
        "Action Simulated"
    }
}
