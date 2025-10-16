use std::{any::type_name, error::Error, sync::LazyLock};

use async_channel::{Receiver, Sender};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ShadeError {
    #[error("File load error:\nPath: {path:?}\nError: {error:?}")]
    FSError {
        error: macroquad::miniquad::fs::Error,
        path: Box<str>,
    },

    #[error("Could not construct asset: {0:?}")]
    AssetDecodeError(#[from] AssetDecodeError),

    #[error("Multiple errors: {0:?}")]
    Multiple(Vec<Self>),
}

#[derive(Error, Debug)]
#[error("Asset type: {asset_kind}\nFile path: {path}\nAdditional Info: {info:?}")]
pub struct AssetDecodeError {
    pub asset_kind: &'static str,
    pub path: Box<str>,
    pub info: Option<Box<dyn Error + Send + Sync>>,
}

impl AssetDecodeError {
    pub(crate) fn new<T>(path: &str, info: Option<Box<dyn Error + Send + Sync>>) -> Self {
        Self {
            asset_kind: type_name::<T>(),
            path: path.into(),
            info,
        }
    }
}

impl From<macroquad::Error> for ShadeError {
    #[inline]
    fn from(value: macroquad::Error) -> Self {
        match value {
            macroquad::Error::FileError { kind, path } => Self::FSError {
                error: kind,
                path: path.into(),
            },
            _ => todo!(),
        }
    }
}

pub(crate) fn combine_errors(results: Vec<Result<(), ShadeError>>) -> Result<(), ShadeError> {
    let mut errors = Vec::new();

    for r in results {
        if let Err(e) = r {
            errors.push(e);
        }
    }

    match errors.len() {
        0 => Ok(()),
        1 => Err(errors.remove(0)),
        _ => Err(ShadeError::Multiple(errors)),
    }
}

static ERROR_CHANNEL: LazyLock<(Sender<ShadeError>, Receiver<ShadeError>)> =
    LazyLock::new(async_channel::unbounded);

pub(crate) fn error_sender() -> Sender<ShadeError> {
    ERROR_CHANNEL.0.clone()
}

pub(crate) fn error_receiver() -> Receiver<ShadeError> {
    debug_assert_eq!(
        ERROR_CHANNEL.1.receiver_count(),
        1,
        "Only one receiver should be used, more requested."
    );
    ERROR_CHANNEL.1.clone()
}
