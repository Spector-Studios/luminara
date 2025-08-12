use macroquad::prelude::*;

use crate::game::Game;

mod game;

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
    let mut game = Game::new();
    game.spawn_unit();
    game.update();
    game.render();

    loop {
        clear_background(RED);

        draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
        draw_circle(screen_width() - 30.0, screen_height() - 30.0, 15.0, YELLOW);
        draw_text("HELLO", 20.0, 20.0, 20.0, DARKGRAY);

        next_frame().await
    }
}
