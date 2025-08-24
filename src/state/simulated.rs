use macroquad::logging::error;
use macroquad::rand::ChooseRandom;

use crate::pathfinding::DijkstraMap;
use crate::state::GameContext;
use crate::state::GameMsg;
use crate::state::GameState;
use crate::state::Transition;
use crate::state::animation::MoveAnimation;
use crate::state::player::PlayerSelect;
use crate::unit::Unit;
use crate::world::Faction;

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
        game_ctx: &mut GameContext,
    ) -> Transition {
        if let Some(msg) = msg_queue.pop_front() {
            if let GameMsg::CommitUnit(unit) = msg {
                game_ctx.world.units.insert(unit.id(), unit);
            } else {
                error!("{} state should not receive msg: {:?}", self.name(), msg);
                panic!("{} state should not receive msg: {:?}", self.name(), msg);
            }
        }
        if let Some(unit) = game_ctx.world.get_unmoved_unit(self.faction) {
            let dijkstra_map = DijkstraMap::new(&game_ctx.world.map, unit, &game_ctx.world.units);
            return Transition::Push(MoveSimulated::boxed_new(unit, dijkstra_map));
        }
        game_ctx.world.setup_turn();
        Transition::Switch(Box::new(PlayerSelect))
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
    fn active_unit(&self) -> Option<Unit> {
        Some(self.unit)
    }
    fn update(
        &mut self,
        msg_queue: &mut VecDeque<GameMsg>,
        game_ctx: &mut GameContext,
    ) -> Transition {
        if let Some(msg) = msg_queue.pop_front() {
            match msg {
                GameMsg::MoveAnimationDone(unit) => {
                    return Transition::Push(ActionSimulated::boxed_new(unit));
                }
                GameMsg::CommitUnit(_unit) => {
                    msg_queue.push_back(msg);
                    return Transition::Pop;
                }
            }
        }

        let dest = self.dijkstra_map.get_reachables().choose().unwrap();
        let path = self.dijkstra_map.get_path(*dest);

        Transition::Push(MoveAnimation::boxed_new(self.unit, path))
    }

    fn name(&self) -> &str {
        todo!()
    }
}

impl ActionSimulated {
    pub fn boxed_new(unit: Unit) -> Box<Self> {
        Box::new(Self { unit })
    }
}

impl GameState for ActionSimulated {
    fn active_unit(&self) -> Option<Unit> {
        Some(self.unit)
    }

    // TODO Maybe not give mutable access to world to states.
    // TODO States can simply pass on chages to the state machine
    // TODO which can commit them to the world.
    fn update(
        &mut self,
        msg_queue: &mut VecDeque<GameMsg>,
        game_ctx: &mut GameContext,
    ) -> Transition {
        self.unit.turn_complete = true;
        msg_queue.push_back(GameMsg::CommitUnit(self.unit));
        Transition::Pop
    }

    fn name(&self) -> &str {
        todo!()
    }
}
