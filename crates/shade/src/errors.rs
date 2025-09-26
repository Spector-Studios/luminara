use std::error::Error;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ShadeErrors {
    #[error("Macroquad error: {0:?}")]
    MacroquadError(#[from] macroquad::Error),

    #[error("Could not construct Asset")]
    AssetDecodeError(Box<str>),

    #[error("Multiple errors: {0:?}")]
    Multiple(Vec<Self>),

    #[error("External error: {0:?}")]
    External(#[from] Box<dyn Error>),
}

pub(crate) fn combine_errors(results: Vec<Result<(), ShadeErrors>>) -> Result<(), ShadeErrors> {
    let mut errors = Vec::new();

    for r in results {
        if let Err(e) = r {
            errors.push(e);
        }
    }

    match errors.len() {
        0 => Ok(()),
        1 => Err(errors.remove(0)),
        _ => Err(ShadeErrors::Multiple(errors)),
    }
}
