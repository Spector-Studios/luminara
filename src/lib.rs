#![warn(clippy::pedantic, clippy::all)]

mod assets;
mod cursor;
mod game;
mod map;
mod math;
mod pathfinding;
mod render;
mod state;
mod ui;
mod unit;
mod world;

use crate::{assets::TextureStore, game::Engine, map::Map, render::RenderContext};

// use macroquad::experimental::animation;
use macroquad::experimental::collections::storage;
use macroquad::prelude::*;

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

#[macroquad::main("Luminara")]
pub async fn main() {
    set_pc_assets_folder("assets");
    set_default_filter_mode(FilterMode::Nearest);
    std::panic::set_hook(Box::new(|info| error!("{:?}", info)));

    let mut texture_store = TextureStore::new();
    texture_store.load("grass1.png");
    texture_store.load("forest1.png");
    texture_store.load("unit1.png");
    texture_store.load("mage1.png");
    texture_store.load("cursor.png");

    texture_store.update().await;

    let grass = texture_store.get("grass1.png");
    let forest = texture_store.get("forest1.png");
    let map = Map::filled(30, 20, &grass, &forest);
    let render_context = RenderContext::new(
        map.width.try_into().unwrap(),
        map.height.try_into().unwrap(),
    );

    storage::store("Global Storage");
    debug!("{:?}", *storage::get::<&str>());

    let mut game = Engine::new(map, render_context, texture_store);

    loop {
        clear_background(BLACK);

        game.update();
        game.render();

        draw_multiline_text("Test\nNew Line", 100.0, 100.0, 50.0, None, WHITE);

        next_frame().await;
    }
}
