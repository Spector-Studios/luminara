use macroquad::prelude::*;

use crate::{assets::TextureStore, game::Game, map::Map, render::Viewport};

mod assets;
mod game;
mod map;
mod render;
mod state;

#[allow(dead_code)]
mod _native_glue {
    #[cfg(target_arch = "wasm32")]
    #[unsafe(no_mangle)]
    pub extern "C" fn main() {
        super::main();
    }

    #[cfg(target_os = "android")]
    #[unsafe(no_mangle)]
    pub extern "C" fn quad_main() {
        super::main();
    }
}

#[macroquad::main("BasicShapes")]
pub async fn main() {
    set_pc_assets_folder("assets");
    set_default_filter_mode(FilterMode::Nearest);

    let mut texture_store = TextureStore::new();
    let grass = texture_store.load("grass1.png");
    let forest = texture_store.load("forest1.png");

    let map = Map::filled(80, 40, grass, forest);
    let viewport = Viewport::new(
        map.width.try_into().unwrap(),
        map.height.try_into().unwrap(),
    );

    let mut game = Game::new(map, viewport, texture_store);

    loop {
        clear_background(BLACK);

        game.async_update().await;

        game.update();
        game.render();

        next_frame().await
    }
}
