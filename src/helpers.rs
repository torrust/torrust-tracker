use std::fs::{File, OpenOptions};
use std::path::Path;
use std::sync::Arc;

use crate::errors::FilePathError;

/// .
///
/// # Errors
///
/// This function will return an error if .
pub fn get_file_at(at: &Path, mode: &OpenOptions) -> Result<(File, Box<Path>), FilePathError> {
    let file = mode.open(at).map_err(|err| FilePathError::FilePathIsNotAvailable {
        input: at.into(),
        source: (Arc::new(err) as Arc<dyn std::error::Error + Send + Sync>).into(),
    })?;

    let at = Path::new(at)
        .canonicalize()
        .map_err(|err| FilePathError::FilePathIsUnresolvable {
            input: at.into(),
            source: (Arc::new(err) as Arc<dyn std::error::Error + Send + Sync>).into(),
        })?;

    Ok((file, at.into_boxed_path()))
}
