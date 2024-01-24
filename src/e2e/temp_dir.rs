//! Temp dir which is automatically removed when it goes out of scope.
use std::path::PathBuf;
use std::{env, io};

use tempfile::TempDir;

pub struct Handler {
    pub temp_dir: TempDir,
    pub original_dir: PathBuf,
}

impl Handler {
    /// Creates a new temporary directory and remembers the current working directory.
    ///
    /// # Errors
    ///
    /// Will error if:
    ///
    /// - It can't create the temp dir.
    /// - It can't get the current dir.
    pub fn new() -> io::Result<Self> {
        let temp_dir = TempDir::new()?;
        let original_dir = env::current_dir()?;

        Ok(Handler { temp_dir, original_dir })
    }

    /// Changes the current working directory to the temporary directory.
    ///
    /// # Errors
    ///
    /// Will error if it can't change the current di to the temp dir.
    pub fn change_to_temp_dir(&self) -> io::Result<()> {
        env::set_current_dir(self.temp_dir.path())
    }

    /// Changes the current working directory back to the original directory.
    ///
    /// # Errors
    ///
    /// Will error if it can't revert the current dir to the original one.
    pub fn revert_to_original_dir(&self) -> io::Result<()> {
        env::set_current_dir(&self.original_dir)
    }
}

impl Drop for Handler {
    /// Ensures that the temporary directory is deleted when the struct goes out of scope.
    fn drop(&mut self) {
        // The temporary directory is automatically deleted when `TempDir` is dropped.
        // We can add additional cleanup here if necessary.
    }
}
