use std::path::Path;

use thiserror::Error;

use crate::located_error::LocatedError;

#[derive(Error, Clone, Debug)]
pub enum SettingsManagerError {
    #[error("Unable to open file for reading : \".{source}\"")]
    FailedToOpenFileForReading {
        source: LocatedError<'static, dyn std::error::Error + Send + Sync>,
    },
    #[error("Unable to open file for writing : \".{source}\"")]
    FailedToOpenFileForWriting {
        source: LocatedError<'static, dyn std::error::Error + Send + Sync>,
    },
    #[error("Unable to open new file at:: {source}!")]
    FailedToCreateNewFile {
        source: LocatedError<'static, dyn std::error::Error + Send + Sync>,
    },

    #[error("Unable to resolve path at: \"{at}\"!")]
    FailedToResolvePath {
        at: Box<Path>,
        source: LocatedError<'static, dyn std::error::Error + Send + Sync>,
    },

    #[error("Unable to prepare directory at: \"{at}\" : {source}!")]
    FailedToPrepareDirectory {
        at: Box<Path>,
        source: LocatedError<'static, dyn std::error::Error + Send + Sync>,
    },

    #[error("Unable to resolve a directory at: \"{at}\"!")]
    FailedToResolveDirectory { at: Box<Path> },

    #[error("Unable to read file, {message}: \"{from}\" : {source}.")]
    FailedToReadFromFile {
        message: String,
        from: Box<Path>,
        source: LocatedError<'static, dyn std::error::Error + Send + Sync>,
    },
    #[error("Unable to write file, {message}: \"{to}\": {source}.")]
    FailedToWriteToFile {
        message: String,
        to: Box<Path>,
        source: LocatedError<'static, dyn std::error::Error + Send + Sync>,
    },

    #[error("Unable to read buffer: {source}")]
    FailedToReadFromBuffer {
        source: LocatedError<'static, dyn std::error::Error + Send + Sync>,
    },
    #[error("Unable to write buffer: {source}")]
    FailedToWriteIntoBuffer {
        source: LocatedError<'static, dyn std::error::Error + Send + Sync>,
    },

    #[error("Unable to read json, {message}: {source}")]
    FailedToSerializeIntoJson {
        message: String,
        source: LocatedError<'static, dyn std::error::Error + Send + Sync>,
    },
    #[error("Unable to write json, {message}: {source}")]
    FailedToDeserializeFromJson {
        message: String,
        source: LocatedError<'static, dyn std::error::Error + Send + Sync>,
    },

    #[error("Unable to read toml: {source}")]
    FailedToDeserializeFromToml {
        source: LocatedError<'static, dyn std::error::Error + Send + Sync>,
    },

    #[error("Decoded json with unknown namespace: \"{namespace}\"")]
    FailedToMatchNamespace { namespace: String },

    #[error("Unable to process old settings from : \"{source}\"")]
    FailedToProcessOldSettings {
        source: LocatedError<'static, dyn std::error::Error + Send + Sync>,
    },

    #[error("Unable to import old settings from: \"{from}\" : \"{source}\"")]
    FailedToImportOldSettings {
        from: Box<Path>,
        source: LocatedError<'static, dyn std::error::Error + Send + Sync>,
    },

    #[error("Unable to successfully move file from: {from} to: {to} \"{source}\"")]
    FailedToMoveFile {
        from: Box<Path>,
        to: Box<Path>,
        source: LocatedError<'static, dyn std::error::Error + Send + Sync>,
    },
}
