use crate::game::UnitId;
use arrayvec::ArrayVec;
use bracket_pathfinding::prelude::Point;

const STACK_DEPTH: usize = 5;

pub struct StateMachine {
    stack: ArrayVec<GameState, STACK_DEPTH>,
}
impl StateMachine {
    pub fn new() -> Self {
        let mut state = Self {
            stack: ArrayVec::new(),
        };
        state.stack.push(GameState::Player(PlayerState::SelectUnit));
        state
    }

    pub fn current_state(&self) -> GameState {
        *self.stack.last().unwrap()
    }

    pub fn transition(&mut self, transition: Transition) {
        match transition {
            Transition::Push(game_state) => self
                .stack
                .try_push(game_state)
                .expect("Attempting to push past max states"),
            Transition::Pop => {
                if self.stack.len() <= 1 {
                    panic!("Attempting to pop base state")
                }
                self.stack.pop().unwrap();
            }
            Transition::Switch(game_state) => *self.stack.last_mut().unwrap() = game_state,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum GameState {
    Player(PlayerState),
    Enemy(EnemyState),
    Animation(AnimationState),
}

#[derive(Clone, Copy, Debug)]
pub enum PlayerState {
    SelectUnit,
    MoveUnit(UnitId),
    Attack(UnitId),
}

#[derive(Clone, Copy, Debug)]
pub enum EnemyState {
    Manager,
    Move(UnitId),
    Attack(UnitId),
}

#[derive(Clone, Copy, Debug)]
pub enum AnimationState {
    Move { unit: UnitId, dest: Point },
    Attack { attacker: UnitId, defender: UnitId },
}

#[derive(Clone, Copy, Debug)]
pub enum Transition {
    Push(GameState),
    Pop,
    Switch(GameState),
}
