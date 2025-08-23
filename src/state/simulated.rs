use crate::state::GameContext;
use crate::state::GameMsg;
use crate::state::GameState;
use crate::state::Transition;
use crate::state::player::PlayerSelect;
use crate::world::Faction;

use std::collections::VecDeque;

#[derive(Debug)]
pub struct SimulatedManager {
    faction: Faction,
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
        game_ctx.world.setup_turn();
        Transition::Switch(Box::new(PlayerSelect))
    }

    fn render(&self, game_ctx: &GameContext) {
        todo!()
    }

    fn name(&self) -> &str {
        "Simulated Manager"
    }
}
