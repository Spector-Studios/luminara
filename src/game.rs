use crate::assets::TextureHandle;
use crate::assets::TextureStore;
use crate::cursor::Cursor;
use crate::map::*;
use crate::pathfinding::DijkstraMap;
use crate::render::RenderContext;
use crate::state::*;

use bracket_pathfinding::prelude::*;
use input_lib::Buttons;
use input_lib::Controller;
use macroquad::prelude::*;

use std::collections::HashMap;
use std::ops::Deref;
use std::ops::DerefMut;

pub enum Faction {
    Player,
    Neutral,
    Enemy,
}

macro_rules! create_id {
    ($name: ident) => {
        #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
        pub struct $name(u32);
        impl Deref for $name {
            type Target = u32;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
        impl DerefMut for $name {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
        impl $name {
            fn new(id: u32) -> Self {
                Self(id)
            }
        }
    };
}

create_id!(WeaponId);
create_id!(UnitId);

pub struct Weapon {
    id: WeaponId,
}

pub struct Unit {
    id: UnitId,
    pub movement: u32,
    faction: Faction,
    health: i32,
    pub pos: Point,
    pub render_pos: Option<Vec2>,
    pub texture_handle: TextureHandle,
    weapon: Option<Weapon>,
}

impl Unit {
    pub fn get_movement_cost(&self, terrain: Terrain) -> u32 {
        match terrain {
            Terrain::Ground => 1,
            Terrain::Forest => 2,
            Terrain::Mountain => DijkstraMap::UNREACHABLE,
            Terrain::River => DijkstraMap::UNREACHABLE,
        }
    }
}

pub struct WorldState {
    units: HashMap<UnitId, Unit>,
    map: Map,
    timer: f32,
    next_unit_id: u32,
    next_weapon_id: u32,
}

struct GameContext {
    world: WorldState,
    render_context: RenderContext,
    controller: Controller,
    cursor: Cursor,
}

impl GameContext {
    pub fn update(&mut self, state_machine: &mut StateMachine) {
        // INFO Pre Update
        self.controller.update();

        // INFO Update
        // INFO This part causes problems with dense Engine struct
        let transition = match state_machine.current_state_mut() {
            GameState::Player(player_state) => self.update_player(player_state),
            GameState::Enemy(enemy_state) => todo!(),
            GameState::Animation { timer, a_state } => self.update_animation(timer, a_state),
        };

        state_machine.transition(transition);

        // INFO Post Update
        self.render_context.update(self.cursor.get_pos());
    }

    pub fn render(&self, state_machine: &StateMachine) {
        self.controller.draw(None);

        // INFO Map
        self.render_context.render_map(&self.world.map);

        match state_machine.current_state() {
            GameState::Player(player_state) => {}
            GameState::Enemy(enemy_state) => {}
            GameState::Animation { timer, a_state } => {}
        }
        // INFO Units
        self.render_all_units(None);

        // INFO Move tiles
        if let GameState::Player(PlayerState::MoveUnit { id, dijkstra_map }) =
            state_machine.current_state()
        {
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
        self.render_context
            .render_tile_rectangle(self.cursor.get_pos(), RED);
    }

    pub fn update_player(&mut self, player_state: &mut PlayerState) -> Transition {
        self.cursor.update(&self.controller, &self.world.map);
        let transition = match player_state {
            PlayerState::SelectUnit => self.player_select_unit(),
            PlayerState::MoveUnit { id, dijkstra_map } => self.player_move_unit(*id, dijkstra_map),
            PlayerState::Action(unit_id) => self.player_action_unit(),
        };

        transition
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
                .find(|(_, unit)| unit.pos == self.cursor.get_pos())
            {
                let dijkstra_map = DijkstraMap::new(&self.world.map, unit);
                return Transition::to_player_move(*id, dijkstra_map);
            }
        }
        Transition::Stay
    }

    fn player_move_unit(&mut self, id: UnitId, dijkstra_map: &mut DijkstraMap) -> Transition {
        if self.controller.button_state().buttons.contains(Buttons::A)
            && !self.controller.last_state().buttons.contains(Buttons::A)
        {
            if let Some(point) = &dijkstra_map
                .get_reachables()
                .iter()
                .find(|pt| **pt == self.cursor.get_pos())
            {
                let path = dijkstra_map.get_path(**point);
                return Transition::to_player_action(id, path);
            }
        }

        Transition::Stay
    }

    fn player_action_unit(&mut self) -> Transition {
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
        if *timer >= 1.0 {
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

impl Engine {
    pub fn new(map: Map, render_context: RenderContext) -> Self {
        let mut units = HashMap::new();
        let unit = Unit {
            id: UnitId::new(0),
            movement: 5,
            faction: Faction::Player,
            health: 10,
            pos: (4, 3).into(),
            render_pos: None,
            texture_handle: render_context.texture_store.get_key("unit1.png"),
            weapon: None,
        };
        units.insert(UnitId(0), unit);

        Self {
            state_machine: StateMachine::new(),
            game_context: GameContext {
                world: WorldState {
                    map,
                    units,
                    timer: 0.0,
                    next_unit_id: 1,
                    next_weapon_id: 0,
                },
                render_context,
                controller: Controller::new(),
                cursor: Cursor::new(),
            },
        }
    }

    pub fn update(&mut self) {
        warn!("{:#?}", &self.state_machine);
        self.game_context.update(&mut self.state_machine);
    }

    pub fn render(&self) {
        self.game_context.render(&self.state_machine);
    }
}
