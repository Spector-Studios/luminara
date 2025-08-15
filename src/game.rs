use crate::assets::TextureStore;
use crate::map::*;
use crate::render::Viewport;
use crate::state::*;

use bracket_pathfinding::prelude::*;
use input_lib::{ButtonState, Controller};
use macroquad::color::RED;
use macroquad::shapes::draw_rectangle;

use std::ops::Deref;
use std::ops::DerefMut;

pub struct Cursor {
    pos: Point,
}

impl Cursor {
    pub fn shift(&mut self, delta: impl Into<Point>, map: &Map) {
        self.pos += delta.into();
        self.pos.x = self.pos.x.clamp(0, (map.width - 1).try_into().unwrap());
        self.pos.y = self.pos.y.clamp(0, (map.height - 1).try_into().unwrap());
    }
}

impl Cursor {
    pub fn new() -> Self {
        Self { pos: Point::zero() }
    }
}

pub enum Faction {
    Player,
    Neutral,
    Enemy,
}

macro_rules! create_id {
    ($name: ident) => {
        #[derive(Clone, Copy, Debug)]
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
    faction: Faction,
    health: i32,
    position: Point,
    weapon: Option<Weapon>,
}

pub struct Game {
    state_machine: StateMachine,
    texture_store: TextureStore,
    units: Vec<Unit>,
    map: Map,
    viewport: Viewport,
    controller: Controller,
    cursor: Cursor,
    next_unit_id: u32,
    next_weapon_id: u32,
}

impl Game {
    pub fn new(map: Map, viewport: Viewport, texture_store: TextureStore) -> Self {
        Self {
            state_machine: StateMachine::new(),
            texture_store,
            map,
            viewport,
            controller: Controller::new(),
            cursor: Cursor::new(),
            units: Vec::new(),
            next_unit_id: 0,
            next_weapon_id: 0,
        }
    }

    pub async fn async_update(&mut self) {
        self.texture_store.update().await;
    }

    pub fn update(&mut self) {
        // INFO Pre Update
        self.controller.update();
        let input = self.controller.button_state;

        // INFO Update
        let maybe_transition = match self.state_machine.current_state() {
            GameState::Player(player_state) => self.update_player(player_state, input),
            GameState::Enemy(enemy_state) => todo!(),
            GameState::Animation(animation_state) => todo!(),
        };

        if let Some(transition) = maybe_transition {
            self.state_machine.transition(transition);
        }

        // INFO Post Update
        self.viewport.update(self.cursor.pos);
    }
    pub fn render(&self) {
        self.controller.draw(None);
        self.viewport.render(&self.map, &self.texture_store);

        draw_rectangle(
            self.viewport.screen_x(self.cursor.pos.x),
            self.viewport.screen_y(self.cursor.pos.y),
            Viewport::tile_size(),
            Viewport::tile_size(),
            RED,
        );
    }

    pub fn update_player(
        &mut self,
        player_state: PlayerState,
        input: ButtonState,
    ) -> Option<Transition> {
        let delta = (input.dpad_x, -input.dpad_y);
        self.cursor.shift(delta, &self.map);
        None
    }
}
