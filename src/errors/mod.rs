use std::path::Path;
use std::sync::Arc;

use thiserror::Error;

pub mod settings;
pub mod settings_manager;
pub mod wrappers;

#[derive(Error, Clone, Debug, Eq, Hash, PartialEq)]
pub enum FilePathError {
    #[error("File Path failed to Canonicalize: {input} : {source}.")]
    FilePathIsUnresolvable {
        input: Box<Path>,
        source: Arc<wrappers::IoError>,
    },

    #[error("File Path destination is not a file: {input} : {source}.")]
    FilePathIsNotAvailable {
        input: Box<Path>,
        source: Arc<wrappers::IoError>,
    },
}
