use std::path::Path;

use thiserror::Error;

use crate::located_error::LocatedError;

pub mod settings;
pub mod settings_manager;
pub mod wrappers;

#[derive(Error, Clone, Debug)]
pub enum FilePathError {
    #[error("File Path failed to Canonicalize: {input} : {source}.")]
    FilePathIsUnresolvable {
        input: Box<Path>,
        source: LocatedError<'static, dyn std::error::Error + Send + Sync>,
    },

    #[error("File Path destination is not a file: {input} : {source}.")]
    FilePathIsNotAvailable {
        input: Box<Path>,
        source: LocatedError<'static, dyn std::error::Error + Send + Sync>,
    },
}
