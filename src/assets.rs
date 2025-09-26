use std::collections::HashMap;

use macroquad::texture::Texture2D;
use macroquad::texture::load_texture;

#[derive(Debug)]
pub struct TextureStore {
    textures: HashMap<String, Texture2D>,
    to_load: Vec<String>,
}

impl TextureStore {
    pub fn new() -> Self {
        Self {
            textures: HashMap::new(),
            to_load: Vec::new(),
        }
    }

    pub fn schedule_load(&mut self, path: &str) {
        if self.textures.contains_key(path) {
            return;
        }

        self.to_load.push(path.to_string());
    }

    pub async fn load_all(&mut self) {
        for path in self.to_load.drain(0..) {
            // TODO Join all the futures or start seperate coroutines to run all this in parallel
            let texture = load_texture(&path).await.expect(&path);
            self.textures.insert(path, texture);
        }

        // INFO WARN Creating Atlas causes blank lines between some map tiles
        // build_textures_atlas();
    }

    #[must_use]
    pub fn get(&self, path: &str) -> Texture2D {
        self.textures.get(path).cloned().unwrap()
    }
}
