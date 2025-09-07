use crate::assets::TextureStore;
use crate::map::Map;
use crate::render::RenderContext;
use crate::state::StateMachine;
use crate::unit::ErasedUnit;
use crate::unit::MovementClass;
use crate::world::Faction;
use crate::world::WorldState;

use input_lib::Controller;
use macroquad::prelude::*;

pub struct GameContext {
    pub world: WorldState,
    pub render_context: RenderContext,
    pub controller: Controller,
    pub texture_store: TextureStore,
}

impl GameContext {
    pub fn new(map: Map, render_context: RenderContext, texture_store: TextureStore) -> Self {
        Self {
            world: WorldState::new(map),
            controller: Controller::new(),
            render_context,
            texture_store,
        }
    }
}

pub struct Engine {
    state_machine: StateMachine,
    game_context: GameContext,
}

type UnitBuilder = [(u32, MovementClass, Faction, i32, (i32, i32), &'static str); 5];

#[rustfmt::skip]
const UNITS: UnitBuilder = [
    (5, MovementClass::Infantry, Faction::Player, 10, (4, 3), "unit1.png"),
    (7, MovementClass::Mounted, Faction::Player, 20, (5, 6), "unit1.png"),
    (7, MovementClass::Flying, Faction::Player, 20, (4, 6), "unit1.png"),
    (5, MovementClass::Infantry, Faction::Enemy, 15, (4, 5), "mage1.png"),
    (6, MovementClass::Mounted, Faction::Enemy, 15, (7, 4), "mage1.png"),
];
impl Engine {
    pub fn new(map: Map, render_context: RenderContext, texture_store: TextureStore) -> Self {
        let mut units = Vec::new();
        for (movement, movement_class, faction, health, pos, texture) in &UNITS {
            units.push(ErasedUnit {
                movement: *movement,
                movement_class: *movement_class,
                turn_complete: false,
                faction: *faction,
                curr_health: *health,
                max_health: *health,
                pos: (*pos).into(),
                texture_path: (*texture).to_string(),
                weapon: None,
            });
        }

        let mut game_ctx = GameContext::new(map, render_context, texture_store);
        for unit in units {
            game_ctx.world.spawn_units(&unit, &game_ctx.texture_store);
        }
        game_ctx.world.setup_turn();

        // Ensure all operations on game_ctx are done before constructing
        // the statemachine
        let state_machine = StateMachine::new(&game_ctx);

        Self {
            state_machine,
            game_context: game_ctx,
        }
    }

    pub fn update(&mut self) {
        self.state_machine.update(&mut self.game_context);
    }

    pub fn render(&self) {
        self.state_machine.render(&self.game_context);
    }
}
