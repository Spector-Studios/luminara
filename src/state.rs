use crate::pathfinding::DijkstraMap;
use crate::unit::UnitId;

use arrayvec::ArrayVec;
use bracket_pathfinding::prelude::Point;

const STACK_DEPTH: usize = 5;

#[derive(Debug)]
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

    pub fn current_state_mut(&mut self) -> &mut GameState {
        self.stack.last_mut().unwrap()
    }

    pub fn current_state(&self) -> &GameState {
        self.stack.last().unwrap()
    }

    pub fn transition(&mut self, transition: Transition) {
        match transition {
            Transition::Stay => {}
            Transition::Done => {
                self.stack.pop().unwrap();
            }
            Transition::Switch(mut game_states) => {
                self.stack.pop();
                game_states.drain(0..).for_each(|gs| self.stack.push(gs));
            }
            Transition::Push(mut game_states) => {
                game_states.drain(0..).for_each(|gs| self.stack.push(gs));
            }
        }
    }
}

#[derive(Debug)]
pub enum GameState {
    Player(PlayerState),
    Enemy(EnemyState),
    Animation { timer: f32, a_state: AnimationState },
}

// TODO Combine substates of Enemy and Player
#[derive(Debug)]
pub enum PlayerState {
    SelectUnit,
    MoveUnit {
        id: UnitId,
        dijkstra_map: DijkstraMap,
    },
    Action(UnitId),
}

#[derive(Debug)]
pub enum EnemyState {
    Manager,
    Move {
        id: UnitId,
        valid_tiles: DijkstraMap,
    },
    Attack(UnitId),
}

#[derive(Clone, Debug)]
pub enum AnimationState {
    Move { unit: UnitId, path: Vec<Point> },
    Attack { attacker: UnitId, defender: UnitId },
}

#[derive(Debug)]
pub enum Transition {
    Stay,
    Done,
    Switch(Vec<GameState>),
    Push(Vec<GameState>),
}

impl Transition {
    pub fn to_player_move(id: UnitId, dijkstra_map: DijkstraMap) -> Self {
        let vec = vec![GameState::Player(PlayerState::MoveUnit {
            id,
            dijkstra_map,
        })];
        Self::Switch(vec)
    }

    pub fn to_player_action(id: UnitId, path: Vec<Point>) -> Self {
        let vec = vec![
            GameState::Player(PlayerState::Action(id)),
            GameState::Animation {
                timer: 0.0,
                a_state: AnimationState::Move { unit: id, path },
            },
        ];
        Self::Switch(vec)
    }

    pub fn to_player_select() -> Self {
        Self::Switch(vec![GameState::Player(PlayerState::SelectUnit)])
    }

    pub fn to_enemy_turn() -> Self {
        Self::Switch(vec![GameState::Enemy(EnemyState::Manager)])
    }
}
