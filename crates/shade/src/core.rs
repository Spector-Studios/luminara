use async_channel::Receiver;
use macroquad::{
    camera::{set_camera, set_default_camera},
    experimental::coroutines::start_coroutine,
    file::set_pc_assets_folder,
    math::{UVec2, uvec2},
    window::next_frame,
};

use crate::{
    assets::AssetServer,
    errors::{ShadeError, error_receiver},
    input::InputManager,
    render::RenderManager,
    scene::Scene,
};

#[must_use]
pub struct Engine<G: Game> {
    spec: EngineSpec,
    game: G,
    scene: Option<Box<dyn Scene<G>>>,
    render: RenderManager,
    input: InputManager,
    // TODO Seperate Global and Scene assets
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

    #[allow(clippy::missing_inline_in_public_items)]
    pub fn run(mut self) {
        macroquad::Window::new(&self.spec.window_title, async move {
            set_pc_assets_folder("assets");
            start_coroutine(error_listener(error_receiver()));

            loop {
                self.input.update();
                self.render.update();

                self.game.update();

                if let Some(scene) = &mut self.scene {
                    scene.update(&self.game, &self.input);

                    set_camera(&self.render.camera);
                    scene.render(&self.game);
                }

                set_default_camera();
                self.input.render();
                next_frame().await;
            }
        });
    }
}

async fn error_listener(receiver: Receiver<ShadeError>) {
    while let Ok(msg) = receiver.recv().await {
        macroquad::logging::error!("[Shade Engine] Error: {:?}", msg);
    }
}

#[must_use]
#[derive(Clone, Debug)]
pub struct EngineSpec {
    canvas_size: UVec2,
    window_title: Box<str>,
}

impl EngineSpec {
    #[inline]
    pub fn new() -> Self {
        Self {
            canvas_size: uvec2(800, 600),
            window_title: "Shade Engine".into(),
        }
    }

    #[inline]
    pub fn set_canvas_size(self, size: UVec2) -> Self {
        Self {
            canvas_size: size,
            ..self
        }
    }

    #[inline]
    pub fn set_title(self, title: impl Into<Box<str>>) -> Self {
        Self {
            window_title: title.into(),
            ..self
        }
    }
}

impl Default for EngineSpec {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

pub trait Game {
    fn init() -> Self;
    fn update(&mut self);
    fn render(&self);
}
