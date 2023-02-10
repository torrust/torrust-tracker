use std::fs::{File, OpenOptions};
use std::path::Path;

use crate::errors::wrappers::IoError;
use crate::errors::FilePathError;

pub fn get_file_at(at: &Path, mode: &OpenOptions) -> Result<(File, Box<Path>), FilePathError> {
    let file = mode.open(at).map_err(|error| FilePathError::FilePathIsNotAvailable {
        input: at.into(),
        source: IoError::from(error).into(),
    })?;

    let at = Path::new(at)
        .canonicalize()
        .map_err(|error| FilePathError::FilePathIsUnresolvable {
            input: at.into(),
            source: IoError::from(error).into(),
        })?;

    Ok((file, at.into_boxed_path()))
}
