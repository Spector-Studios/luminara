use std::collections::HashMap;

use macroquad::experimental::coroutines::{Coroutine, start_coroutine};
use macroquad::file::load_file;
use macroquad::text::{Font, load_ttf_font_from_bytes};
use macroquad::texture::Texture2D;
use slotmap::{SlotMap, SparseSecondaryMap};
use ttmap::TypeMap;

use crate::assets::key::AssetKey;
use crate::errors::{ShadeErrors, combine_errors};
use crate::sealed;

mod key;

trait Asset: Send + Sync + 'static {
    fn from_bytes(bytes: Vec<u8>) -> Result<Self, ShadeErrors>
    where
        Self: Sized;
}
impl<T: GenericAsset> Asset for T {
    fn from_bytes(bytes: Vec<u8>) -> Result<T, ShadeErrors> {
        T::from_bytes(bytes)
    }
}
impl Asset for Texture2D {
    fn from_bytes(bytes: Vec<u8>) -> Result<Texture2D, ShadeErrors> {
        Ok(Texture2D::from_file_with_format(&bytes, None))
    }
}
impl Asset for Font {
    fn from_bytes(bytes: Vec<u8>) -> Result<Font, ShadeErrors> {
        Ok(load_ttf_font_from_bytes(&bytes)?)
    }
}

pub trait GenericAsset: Send + Sync + 'static {
    fn from_bytes(bytes: Vec<u8>) -> Result<Self, ShadeErrors>
    where
        Self: Sized;
}

type FileLoadingCoroutine = Coroutine<Result<Vec<u8>, macroquad::Error>>;

struct AssetStore<T: Asset> {
    store: SlotMap<AssetKey<T>, Option<T>>,
    loaded: HashMap<Box<str>, Handle<T>>,
    loading: SparseSecondaryMap<AssetKey<T>, FileLoadingCoroutine>,
}

impl<T: Asset> AssetStore<T> {
    fn new() -> Self {
        Self {
            store: SlotMap::with_key(),
            loaded: HashMap::new(),
            loading: SparseSecondaryMap::new(),
        }
    }

    fn try_get(&self, handle: Handle<T>) -> Option<&T> {
        self.store
            .get(handle.key)
            .expect("Handle should not live longer than asset")
            .as_ref()
    }

    fn load(&mut self, path: &str) -> Handle<T> {
        if let Some(handle) = self.loaded.get(path) {
            return *handle;
        }

        let new_key = self.store.insert(None);

        let boxed_path: Box<str> = Box::from(path);

        // TODO This could take care of inserting the asset but
        // then self.store needs to be Arc<RwLock<>> which makes
        // self.get() impossible
        let loading_coroutine = start_coroutine(async move { load_file(&boxed_path).await });
        self.loading.insert(new_key, loading_coroutine);

        let new_handle = Handle::from_key(new_key);
        self.loaded.insert(path.into(), new_handle);

        new_handle
    }

    fn update(&mut self) -> Result<(), ShadeErrors> {
        let mut to_remove = Vec::new();

        for (key, coroutine) in &self.loading {
            if !coroutine.is_done() {
                continue;
            }
            let bytes = coroutine.retrieve().unwrap()?;

            let asset = T::from_bytes(bytes)?;

            *self.store.get_mut(key).unwrap() = Some(asset);
            to_remove.push(key);
        }

        for key in &to_remove {
            self.loading.remove(*key);
        }

        Ok(())
    }
}

pub struct Handle<T> {
    key: AssetKey<T>,
}
impl<T> Handle<T> {
    fn from_key(key: AssetKey<T>) -> Self {
        Self { key }
    }
}
impl<T> Clone for Handle<T> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<T> Copy for Handle<T> {}

#[expect(private_bounds, reason = "Sealed trait")]
pub trait AssetCollection<T>: sealed::Sealed {
    fn try_get(&self, handle: Handle<T>) -> Option<&T>;

    fn get(&self, handle: Handle<T>) -> &T {
        self.try_get(handle).unwrap()
    }

    fn load(&mut self, path: &str) -> Handle<T>;
}

pub struct AssetServer {
    textures: AssetStore<Texture2D>,
    fonts: AssetStore<Font>,
    generics: TypeMap,
}

impl AssetServer {
    pub(crate) fn new() -> Self {
        Self {
            textures: AssetStore::new(),
            fonts: AssetStore::new(),
            generics: TypeMap::new(),
        }
    }

    pub fn register_asset_type<T: GenericAsset>(&mut self) {
        self.generics.insert::<AssetStore<T>>(AssetStore::new());
    }

    pub(crate) fn update(&mut self) -> Result<(), ShadeErrors> {
        combine_errors(vec![
            self.textures.update(),
            self.fonts.update(),
            // FIXME self.generics.iter().update?
        ])
    }
}

impl sealed::Sealed for AssetServer {}
impl AssetCollection<Texture2D> for AssetServer {
    fn try_get(&self, handle: Handle<Texture2D>) -> Option<&Texture2D> {
        self.textures.try_get(handle)
    }

    fn load(&mut self, path: &str) -> Handle<Texture2D> {
        self.textures.load(path)
    }
}

impl AssetCollection<Font> for AssetServer {
    fn try_get(&self, handle: Handle<Font>) -> Option<&Font> {
        self.fonts.try_get(handle)
    }

    fn load(&mut self, path: &str) -> Handle<Font> {
        self.fonts.load(path)
    }
}

impl<T> AssetCollection<T> for AssetServer
where
    T: GenericAsset,
{
    fn try_get(&self, handle: Handle<T>) -> Option<&T> {
        self.generics.get::<AssetStore<T>>()?.try_get(handle)
    }

    fn load(&mut self, path: &str) -> Handle<T> {
        self.generics
            .get_mut::<AssetStore<T>>()
            .expect("Asset type needs to be registered before loading.")
            .load(path)
    }
}
