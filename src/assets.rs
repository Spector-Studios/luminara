use std::collections::HashMap;

use macroquad::texture::{Texture2D, build_textures_atlas, load_texture};

#[derive(Clone, Copy, Debug)]
pub struct TextureHandle(usize);

#[derive(Debug)]
pub struct TextureStore {
    textures: Vec<Texture2D>,
    handles: HashMap<String, TextureHandle>,
    to_load: Vec<(String, TextureHandle)>,
}

impl TextureStore {
    pub fn new() -> Self {
        Self {
            textures: Vec::new(),
            handles: HashMap::new(),
            to_load: Vec::new(),
        }
    }

    #[must_use]
    pub fn load(&mut self, path: &str) -> TextureHandle {
        if let Some(&id) = self.handles.get(path) {
            return id;
        }

        let handle = TextureHandle(self.textures.len());
        self.textures.push(Texture2D::empty());
        self.to_load.push((path.to_string(), handle));
        self.handles.insert(path.to_string(), handle);

        handle
    }

    pub async fn update(&mut self) {
        for (path, handle) in self.to_load.drain(0..) {
            let texture = load_texture(&path).await.unwrap();
            self.textures[handle.0] = texture;
        }

        // INFO Creating Atlas causes blank lines between some map tiles
        // build_textures_atlas();
    }

    pub fn get(&self, handle: TextureHandle) -> &Texture2D {
        &self.textures[handle.0]
    }

    pub fn get_key(&self, path: &str) -> TextureHandle {
        *self.handles.get(path).unwrap()
    }
}
