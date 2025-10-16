use std::collections::HashMap;
use std::sync::{Arc, OnceLock};

use macroquad::experimental::coroutines::start_coroutine;
use macroquad::logging::error;
use macroquad::prelude::load_file;
use macroquad::text::{Font, load_ttf_font_from_bytes};
use macroquad::texture::Texture2D;
use ttmap::TypeMap;

use crate::errors::{AssetDecodeError, error_sender};
use crate::{SendableError, sealed};

mod key;

const PLACEHOLDER_TEXTURE: [u8; 16] = [
    255, 0, 255, 255, //Magenta
    255, 255, 255, 255, //Black
    255, 255, 255, 255, //Black
    255, 0, 255, 255, //Magenta
];

trait Asset: Send + Sync + 'static {
    fn from_bytes(bytes: Vec<u8>) -> Result<Self, Box<SendableError>>
    where
        Self: Sized;

    fn place_holder() -> Self
    where
        Self: Sized;
}
impl<T: GenericAsset> Asset for T {
    fn from_bytes(bytes: Vec<u8>) -> Result<T, Box<SendableError>> {
        T::from_bytes(bytes)
    }
    fn place_holder() -> Self {
        T::place_holder()
    }
}

impl Asset for Texture2D {
    fn from_bytes(bytes: Vec<u8>) -> Result<Texture2D, Box<SendableError>> {
        Ok(Texture2D::from_file_with_format(&bytes, None))
    }

    fn place_holder() -> Self {
        Texture2D::from_rgba8(2, 2, &PLACEHOLDER_TEXTURE)
    }
}
impl Asset for Font {
    fn from_bytes(bytes: Vec<u8>) -> Result<Font, Box<SendableError>> {
        Ok(load_ttf_font_from_bytes(&bytes)?)
    }

    fn place_holder() -> Self {
        load_ttf_font_from_bytes(include_bytes!("ProggyClean.ttf"))
            .expect("This is the default font")
    }
}

pub trait GenericAsset: Send + Sync + 'static {
    fn from_bytes(bytes: Vec<u8>) -> Result<Self, Box<SendableError>>
    where
        Self: Sized;

    fn place_holder() -> Self
    where
        Self: Sized;
}

struct AssetStore<T: Asset> {
    store: HashMap<Box<str>, Handle<T>>,
}

impl<T: Asset> AssetStore<T> {
    fn new() -> Self {
        Self {
            store: HashMap::new(),
        }
    }

    fn load(&mut self, path: &str) -> Handle<T> {
        if let Some(handle) = self.store.get(path) {
            return handle.clone();
        }

        let new_handle = Handle::new();
        self.store.insert(path.into(), new_handle.clone());

        let boxed_path: Box<str> = Box::from(path);

        let cloned_handle = new_handle.clone();
        let error_tx = error_sender();
        let _loading_coroutine = start_coroutine(async move {
            let file = match load_file(&boxed_path).await {
                Ok(bytes) => Some(bytes),
                Err(err) => {
                    error_tx.send(err.into()).await.unwrap();
                    None
                }
            };

            let asset = match file {
                Some(bytes) => match T::from_bytes(bytes) {
                    Ok(t) => t,
                    Err(err) => {
                        let decode_err = AssetDecodeError::new::<T>(&boxed_path, Some(err));
                        error_tx.send(decode_err.into()).await.unwrap();
                        T::place_holder()
                    }
                },
                None => T::place_holder(),
            };

            cloned_handle
                .inner
                .set(asset)
                // .expect() without requiring Debug trait bound on Asset
                .unwrap_or_else(|_| {
                    // TODO make this reusable
                    error!(
                        "[ShadeEngine {}:{}] {}",
                        file!(),
                        line!(),
                        "This should be the only place this OnceLock is set"
                    );
                    panic!()
                });
        });

        new_handle
    }
}

// TODO Make this some kind of dense Arc
pub struct Handle<T> {
    inner: Arc<OnceLock<T>>,
}
impl<T> Handle<T> {
    fn new() -> Self {
        Self {
            inner: Arc::new(OnceLock::new()),
        }
    }

    #[inline]
    #[must_use]
    pub fn get(&self) -> Option<&T> {
        self.inner.get()
    }
}
impl<T> Clone for Handle<T> {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            inner: Arc::clone(&self.inner),
        }
    }
}

#[expect(private_bounds, reason = "Sealed trait")]
pub trait AssetCollection<T>: sealed::Sealed {
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

    #[inline]
    pub fn register_asset_type<T: GenericAsset>(&mut self) {
        self.generics.insert::<AssetStore<T>>(AssetStore::new());
    }
}

impl crate::sealed::Sealed for AssetServer {}

impl AssetCollection<Texture2D> for AssetServer {
    #[inline]
    fn load(&mut self, path: &str) -> Handle<Texture2D> {
        self.textures.load(path)
    }
}

impl AssetCollection<Font> for AssetServer {
    #[inline]
    fn load(&mut self, path: &str) -> Handle<Font> {
        self.fonts.load(path)
    }
}

impl<T: GenericAsset> AssetCollection<T> for AssetServer {
    #[inline]
    fn load(&mut self, path: &str) -> Handle<T> {
        self.generics
            .get_mut::<AssetStore<T>>()
            .expect("Asset type should be registered before use.")
            .load(path)
    }
}
