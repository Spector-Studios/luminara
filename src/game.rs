use crate::cursor::Cursor;
use crate::map::*;
use crate::pathfinding::DijkstraMap;
use crate::render::RenderContext;
use crate::state::*;
use crate::unit::Unit;
use crate::unit::UnitId;
use crate::world::Faction;
use crate::world::WorldState;

use bracket_pathfinding::prelude::*;
use input_lib::Buttons;
use input_lib::Controller;
use macroquad::prelude::*;
use macroquad::rand::ChooseRandom;

struct GameContext {
    world: WorldState,
    render_context: RenderContext,
    controller: Controller,
    cursor: Cursor,
}

impl GameContext {
    pub fn new(map: Map, render_context: RenderContext) -> Self {
        Self {
            world: WorldState::new(map),
            render_context,
            controller: Controller::new(),
            cursor: Cursor::new(),
        }
    }

    pub fn update(&mut self, state_machine: &mut StateMachine) {
        // INFO Pre Update
        self.controller.update();

        // INFO Update
        // INFO This part causes problems with dense Engine struct
        let transition = match state_machine.current_state_mut() {
            GameState::Player(player_state) => self.update_player(player_state),
            GameState::Enemy(enemy_state) => self.update_enemy(enemy_state),
            GameState::Animation { timer, a_state } => self.update_animation(timer, a_state),
        };

        state_machine.transition(transition);

        // INFO Post Update
        self.render_context.update(self.cursor.get_pos());
    }

    pub fn render(&self, game_state: &GameState) {
        self.controller.draw(None);

        // INFO Map
        self.render_context.render_map(&self.world.map);

        match game_state {
            GameState::Player(player_state) => {}
            GameState::Enemy(enemy_state) => {}
            GameState::Animation { timer, a_state } => {}
        }
        // INFO Units
        self.render_all_units(None);

        // INFO Move tiles
        if let GameState::Player(PlayerState::MoveUnit { id, dijkstra_map }) = game_state {
            for point in dijkstra_map
                .get_reachables()
                .iter()
                .filter(|pt| self.render_context.in_bounds(**pt))
            {
                self.render_context
                    .render_tile_rectangle(*point, Color::new(0.0, 0.0, 0.9, 0.5));
            }
        }

        // INFO Cursor
        if matches!(game_state, GameState::Player(..)) {
            self.render_context
                .render_tile_rectangle(self.cursor.get_pos(), RED);
        }
    }

    pub fn update_player(&mut self, player_state: &mut PlayerState) -> Transition {
        self.cursor.update(&self.controller, &self.world.map);

        if self.world.available_units.is_empty() {
            self.world.setup_turn(Faction::Enemy);
            return Transition::to_enemy_manager();
        }

        let transition = match player_state {
            PlayerState::SelectUnit => self.player_select_unit(),
            PlayerState::MoveUnit { id, dijkstra_map } => self.player_move_unit(*id, dijkstra_map),
            PlayerState::Action(unit_id) => self.player_action_unit(unit_id),
        };

        transition
    }

    // TODO
    pub fn update_enemy(&mut self, enemy_state: &mut EnemyState) -> Transition {
        let transition = match enemy_state {
            EnemyState::Manager => self.enemy_turn_manager(),
            EnemyState::Move { id, dijkstra_map } => self.enemy_move_unit(*id, dijkstra_map),
            EnemyState::Action(unit_id) => self.enemy_action(*unit_id),
        };

        transition
    }

    fn enemy_turn_manager(&mut self) -> Transition {
        if let Some(id) = self.world.available_units.pop() {
            let dijkstra_map = DijkstraMap::new(
                &self.world.map,
                self.world.units.get(&id).unwrap(),
                &self.world.units,
            );
            return Transition::to_enemy_move(id, dijkstra_map);
        } else {
            self.world.setup_turn(Faction::Player);
            return Transition::to_player_select();
        }
    }

    fn enemy_move_unit(&mut self, id: UnitId, dijkstra_map: &mut DijkstraMap) -> Transition {
        let target = dijkstra_map.get_reachables().choose().unwrap();
        Transition::to_enemy_action(id, dijkstra_map.get_path(*target))
    }

    fn enemy_action(&mut self, id: UnitId) -> Transition {
        Transition::to_enemy_manager()
    }

    fn render_all_units(&self, except: Option<UnitId>) {
        self.world
            .units
            .iter()
            .filter(|(_, unit)| self.render_context.map_view_rect().point_in_rect(unit.pos))
            .filter(|(id, _)| except.is_none_or(|except_id| except_id != **id))
            .for_each(|(_, unit)| {
                self.render_context
                    .render_sprite(unit.pos, unit.texture_handle);
            });
    }

    fn player_select_unit(&mut self) -> Transition {
        let input = self.controller.button_state();
        if input.buttons.contains(Buttons::A) {
            if let Some((id, unit)) = self
                .world
                .units
                .iter()
                .filter(|(id, _)| self.world.available_units.contains(*id))
                .find(|(_, unit)| unit.pos == self.cursor.get_pos())
            {
                let dijkstra_map = DijkstraMap::new(&self.world.map, unit, &self.world.units);
                return Transition::to_player_move(*id, dijkstra_map);
            }
        }
        Transition::Stay
    }

    fn player_move_unit(&mut self, id: UnitId, dijkstra_map: &mut DijkstraMap) -> Transition {
        if self.controller.button_state().buttons.contains(Buttons::A)
            && !self.controller.last_state().buttons.contains(Buttons::A)
        {
            if dijkstra_map
                .get_reachables()
                .contains(&self.cursor.get_pos())
                && !self
                    .world
                    .units
                    .values()
                    .any(|unit| unit.pos == self.cursor.get_pos())
            {
                let path = dijkstra_map.get_path(self.cursor.get_pos());
                return Transition::to_player_action(id, path);
            }
        }

        if self.controller.button_state().buttons.contains(Buttons::B) {
            return Transition::to_player_select();
        }

        Transition::Stay
    }

    // TODO
    fn player_action_unit(&mut self, id: &mut UnitId) -> Transition {
        assert!(self.world.available_units.remove(id));
        Transition::to_player_select()
    }

    pub fn update_animation(
        &mut self,
        timer: &mut f32,
        a_state: &mut AnimationState,
    ) -> Transition {
        *timer += get_frame_time();

        let transition = match a_state {
            AnimationState::Move { unit, path } => self.move_animation(timer, *unit, path),
            AnimationState::Attack { attacker, defender } => todo!(),
        };

        transition
    }

    fn move_animation(&mut self, timer: &mut f32, id: UnitId, path: &mut Vec<Point>) -> Transition {
        if *timer >= 0.2 {
            match path.pop() {
                Some(pos) => {
                    *timer = 0.0;
                    self.world.units.get_mut(&id).unwrap().pos = pos;
                }
                None => {
                    return Transition::Done;
                }
            }
        }

        Transition::Stay
    }
}

pub struct Engine {
    state_machine: StateMachine,
    game_context: GameContext,
}

const UNITS: [(u32, Faction, i32, (i32, i32), &str); 3] = [
    (5, Faction::Player, 10, (4, 3), "unit1.png"),
    (7, Faction::Player, 20, (5, 6), "unit1.png"),
    (5, Faction::Enemy, 15, (4, 5), "mage1.png"),
];
impl Engine {
    pub fn new(map: Map, render_context: RenderContext) -> Self {
        let mut units = Vec::new();
        UNITS
            .iter()
            .for_each(|(movement, faction, health, pos, texture)| {
                units.push(Unit {
                    movement: *movement,
                    faction: *faction,
                    health: *health,
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
        engine.game_context.world.setup_turn(Faction::Player);

        engine
    }

    pub fn update(&mut self) {
        self.game_context.update(&mut self.state_machine);
    }

    pub fn render(&self) {
        self.game_context
            .render(&self.state_machine.current_state());
    }
}
