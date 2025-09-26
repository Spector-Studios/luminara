#![warn(clippy::pedantic, clippy::all)]

pub mod assets;
pub mod core;
pub mod errors;
pub mod input;
pub mod prelude;
pub mod scene;

mod logging;
mod render;

pub(crate) mod sealed {
    pub(crate) trait Sealed {}
}
