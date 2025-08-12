use std::ops::{Deref, DerefMut};

use arrayvec::ArrayVec;
use bracket_pathfinding::prelude::*;

const STACK_DEPTH: usize = 5;

pub struct Cursor {
    pos: Point,
}

pub enum GameState {
    Player {
        cursor: Cursor,
        p_state: PlayerState,
    },
    Enemy(EnemyState),
    Animation(AnimationState),
}

pub enum PlayerState {
    SelectUnit,
    MoveUnit(UnitId),
    Attack(UnitId),
}

pub enum EnemyState {
    Manager,
    Move(UnitId),
    Attack(UnitId),
}

pub enum AnimationState {
    Move { unit: UnitId, dest: Point },
    Attack { attacker: UnitId, defender: UnitId },
}

pub enum Faction {
    Player,
    Neutral,
    Enemy,
}

macro_rules! create_id {
    ($name:ident) => {
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
    state_stack: ArrayVec<GameState, STACK_DEPTH>,
    units: Vec<Unit>,
    next_unit_id: u32,
    next_weapon_id: u32,
}

impl Game {
    pub fn new() -> Self {
        Self {
            state_stack: ArrayVec::new(),
            units: Vec::new(),
            next_unit_id: 0,
            next_weapon_id: 0,
        }
    }
    pub fn update(&mut self) {
        match self.state_stack.last_mut().unwrap() {
            GameState::Player { cursor, p_state } => todo!(),
            GameState::Enemy(enemy_state) => todo!(),
            GameState::Animation(animation_state) => todo!(),
        }
    }
    pub fn render(&self) {}
    pub fn spawn_unit(&mut self) {
        let unitid = UnitId::new(self.next_unit_id);
        self.next_unit_id += 1;
    }

    pub fn update_player(cursor: Cursor, state: PlayerState) -> Option<u32> {
        todo!()
    }
}
