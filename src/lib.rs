#![warn(clippy::pedantic, clippy::all)]

mod assets;
mod cursor;
mod game;
mod map;
mod math;
mod pathfinding;
mod prelude;
mod render;
mod state;
mod ui;
mod unit;
mod world;

use crate::{assets::TextureStore, game::Engine, map::Map};

// use macroquad::experimental::animation;
use macroquad::experimental::collections::storage;
use macroquad::prelude::coroutines::start_coroutine;
use macroquad::prelude::*;

#[allow(dead_code)]
mod platform {
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

    let texture_store;
    {
        let builder = start_coroutine(async move {
            let mut texture_store = TextureStore::new();
            texture_store.schedule_load("grass1.png");
            texture_store.schedule_load("forest1.png");
            texture_store.schedule_load("unit1.png");
            texture_store.schedule_load("mage1.png");
            texture_store.schedule_load("cursor.png");

            texture_store.load_all().await;
            texture_store
        });

        let text = "Loading";
        let font_size = 200;
        let center = get_text_center(text, None, font_size, 1.0, 0.0);
        let (x, y) = (
            screen_width() / 2.0 - center.x,
            screen_height() / 2.0 - center.y,
        );
        loop {
            if builder.is_done() {
                texture_store = builder.retrieve().unwrap();
                break;
            }
            draw_text(text, x, y, font_size.into(), WHITE);
            next_frame().await;
        }
    }

    let grass = texture_store.get("grass1.png");
    let forest = texture_store.get("forest1.png");
    let map = Map::random(30, 20, &grass, &forest);

    storage::store("Global Storage");
    debug!("{:?}", *storage::get::<&str>());

    let mut game = Engine::new(map, texture_store);

    loop {
        clear_background(BLACK);

        game.update();
        game.render();

        next_frame().await;
    }
}
