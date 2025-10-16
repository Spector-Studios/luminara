#![warn(clippy::pedantic, clippy::all)]
#![warn(clippy::missing_inline_in_public_items)]

//! Test

use std::error::Error;

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

pub type SendableError = dyn Error + Send + Sync;
