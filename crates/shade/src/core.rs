use macroquad::{
    camera::{set_camera, set_default_camera},
    math::{UVec2, uvec2},
    window::next_frame,
};

use crate::{assets::AssetServer, input::InputManager, render::RenderManager, scene::Scene};

#[must_use]
pub struct Engine<G: Game> {
    spec: EngineSpec,
    game: G,
    scene: Option<Box<dyn Scene<G>>>,
    render: RenderManager,
    input: InputManager,
    pub assets: AssetServer,
}

impl<G: Game + 'static> Engine<G> {
    #[inline]
    pub fn new(engine_spec: EngineSpec) -> Self {
        Self {
            render: RenderManager::new(engine_spec.canvas_size),
            input: InputManager::new(),
            assets: AssetServer::new(),
            spec: engine_spec,
            game: G::init(),
            scene: None,
        }
    }

    pub fn run(mut self) {
        macroquad::Window::new(&self.spec.window_title, async move {
            let err = self.assets.update();
            self.render.update();

            self.game.update();

            if let Some(scene) = &mut self.scene {
                scene.update(&self.game, &self.input);

                set_camera(&self.render.camera);
                scene.render(&self.game);
            }

            set_default_camera();
            next_frame().await;
        });
    }
}

#[must_use]
#[derive(Clone, Debug)]
pub struct EngineSpec {
    canvas_size: UVec2,
    window_title: Box<str>,
}

impl EngineSpec {
    pub fn new() -> Self {
        Self {
            canvas_size: uvec2(800, 600),
            window_title: "Shade Engine".into(),
        }
    }

    pub fn set_canvas_size(self, size: UVec2) -> Self {
        Self {
            canvas_size: size,
            ..self
        }
    }

    pub fn set_title(self, title: impl Into<Box<str>>) -> Self {
        Self {
            window_title: title.into(),
            ..self
        }
    }
}

impl Default for EngineSpec {
    fn default() -> Self {
        Self::new()
    }
}

pub trait Game {
    fn init() -> Self;
    fn update(&mut self);
    fn render(&self);
}
