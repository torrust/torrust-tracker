use std::path::Path;
use std::sync::Arc;

use thiserror::Error;

use super::wrappers::{self, IoError, TomlDeError};
use super::{settings, FilePathError};

#[derive(Error, Clone, Debug, Eq, Hash, PartialEq)]
pub enum SettingsManagerError {
    #[error("Unable to open file for reading : \".{source}\"")]
    FailedToOpenFileForReading { source: FilePathError },
    #[error("Unable to open file for writing : \".{source}\"")]
    FailedToOpenFileForWriting { source: FilePathError },
    #[error("Unable to open new file at:: {source}!")]
    FailedToCreateNewFile { source: FilePathError },

    #[error("Unable to resolve path at: \"{at}\"!")]
    FailedToResolvePath { at: Box<Path>, source: Arc<wrappers::IoError> },

    #[error("Unable to prepare directory at: \"{at}\" : {source}!")]
    FailedToPrepareDirectory { at: Box<Path>, source: Arc<wrappers::IoError> },

    #[error("Unable to resolve a directory at: \"{at}\"!")]
    FailedToResolveDirectory { at: Box<Path> },

    #[error("Unable to read file, {message}: \"{from}\" : {source}.")]
    FailedToReadFromFile {
        message: String,
        from: Box<Path>,
        source: Box<Self>,
    },
    #[error("Unable to write file, {message}: \"{to}\": {source}.")]
    FailedToWriteToFile {
        message: String,
        to: Box<Path>,
        source: Box<Self>,
    },

    #[error("Unable to read buffer: {source}")]
    FailedToReadFromBuffer { source: Arc<IoError> },
    #[error("Unable to write buffer: {source}")]
    FailedToWriteIntoBuffer { source: Arc<IoError> },

    #[error("Unable to read json, {message}: {source}")]
    FailedToSerializeIntoJson {
        message: String,
        source: Arc<wrappers::SerdeJsonError>,
    },
    #[error("Unable to write json, {message}: {source}")]
    FailedToDeserializeFromJson {
        message: String,
        source: Arc<wrappers::SerdeJsonError>,
    },

    #[error("Unable to read toml: {source}")]
    FailedToDeserializeFromToml { source: TomlDeError },

    #[error("Decoded json with unknown namespace: \"{namespace}\"")]
    FailedToMatchNamespace { namespace: String },

    #[error("Unable to process old settings from : \"{source}\"")]
    FailedToProcessOldSettings { source: Box<Self> },

    #[error("Unable to import old settings from: \"{from}\" : \"{source}\"")]
    FailedToImportOldSettings {
        from: Box<Path>,
        source: Box<settings::SettingsError>,
    },

    #[error("Unable to successfully move file from: {from} to: {to} \"{source}\"")]
    FailedToMoveFile {
        from: Box<Path>,
        to: Box<Path>,
        source: Arc<IoError>,
    },
}
