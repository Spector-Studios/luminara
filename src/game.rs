use crate::cursor::Cursor;
use crate::map::*;
use crate::render::RenderContext;
use crate::state::*;
use crate::unit::ErasedUnit;
use crate::world::Faction;
use crate::world::WorldState;

use input_lib::Controller;
use macroquad::prelude::*;

pub struct GameContext {
    pub world: WorldState,
    pub render_context: RenderContext,
    pub controller: Controller,
    pub cursor: Cursor,
}

impl GameContext {
    pub fn new(map: Map, render_context: RenderContext) -> Self {
        Self {
            world: WorldState::new(map),
            controller: Controller::new(),
            cursor: Cursor::new(render_context.texture_store.get_key("cursor.png")),
            render_context,
        }
    }
}

pub struct Engine {
    state_machine: StateMachine,
    game_context: GameContext,
}

const UNITS: [(u32, Faction, i32, (i32, i32), &str); 5] = [
    (5, Faction::Player, 10, (4, 3), "unit1.png"),
    (7, Faction::Player, 20, (5, 6), "unit1.png"),
    (7, Faction::Player, 20, (4, 6), "unit1.png"),
    (5, Faction::Enemy, 15, (4, 5), "mage1.png"),
    (6, Faction::Enemy, 15, (7, 4), "mage1.png"),
];
impl Engine {
    pub fn new(map: Map, render_context: RenderContext) -> Self {
        let mut units = Vec::new();
        UNITS
            .iter()
            .for_each(|(movement, faction, health, pos, texture)| {
                units.push(ErasedUnit {
                    movement: *movement,
                    turn_complete: false,
                    faction: *faction,
                    curr_health: *health,
                    max_health: *health,
                    pos: (*pos).into(),
                    render_pos: None,
                    texture_handle: render_context.texture_store.get_key(&texture),
                    weapon: None,
                })
            });

        let mut engine = Self {
            state_machine: StateMachine::new(),
            game_context: GameContext::new(map, render_context),
        };
        engine.game_context.world.spawn_units(&units);
        engine.game_context.world.setup_turn();

        engine
    }

    pub fn update(&mut self) {
        self.state_machine.update(&mut self.game_context);
    }

    pub fn render(&self) {
        self.state_machine.render(&self.game_context);
    }
}
