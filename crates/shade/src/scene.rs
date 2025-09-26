use crate::{core::Game, input::InputManager};

#[expect(unused_variables, reason = "Empty functions in trait definition")]
pub trait Scene<G: Game> {
    fn on_enter(&mut self, game_ctx: &G) {}
    fn update(&mut self, game_ctx: &G, input: &InputManager);
    fn render(&self, game_ctx: &G);
    fn on_exit(&self, game_ctx: &G) {}
}
