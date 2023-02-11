use std::collections::hash_map::DefaultHasher;
use std::ffi::OsString;
use std::fs::{self, OpenOptions};
use std::hash::{Hash, Hasher};
use std::io::{Cursor, Read, Write};
use std::path::Path;
use std::sync::Arc;

use log::{info, warn};

use super::{
    Settings, SettingsErrored, SettingsNamespace, TrackerSettings, TrackerSettingsBuilder, SETTINGS_NAMESPACE,
    SETTINGS_NAMESPACE_ERRORED,
};
use crate::config_const::{CONFIG_BACKUP_FOLDER, CONFIG_DEFAULT, CONFIG_ERROR_FOLDER, CONFIG_FOLDER, CONFIG_LOCAL, CONFIG_OLD};
use crate::errors::settings_manager::SettingsManagerError;
use crate::errors::wrappers::{IoError, SerdeJsonError};
use crate::helpers::get_file_at;
use crate::located_error::Located;
use crate::settings::{Clean, Fix};
use crate::Empty;

#[derive(PartialEq, Eq, Debug, Clone, Hash)]
pub struct SettingsManager {
    settings: Result<Settings, SettingsErrored>,
}

impl Default for SettingsManager {
    fn default() -> Self {
        Self {
            settings: Ok(Settings::default()),
        }
    }
}

impl From<Settings> for SettingsManager {
    fn from(okay: Settings) -> Self {
        Self { settings: Ok(okay) }
    }
}

impl TryFrom<SettingsManager> for Settings {
    type Error = SettingsErrored;

    fn try_from(manager: SettingsManager) -> Result<Self, Self::Error> {
        manager.settings
    }
}

impl From<SettingsErrored> for SettingsManager {
    fn from(error: SettingsErrored) -> Self {
        Self { settings: Err(error) }
    }
}

impl SettingsManager {
    #[must_use]
    pub fn empty() -> Self {
        Self {
            settings: Ok(Empty::empty()),
        }
    }

    #[must_use]
    pub fn error(errored: &SettingsErrored) -> Self {
        Self {
            settings: Err(errored.clone()),
        }
    }

    /// .
    ///
    /// # Errors
    ///
    /// This function will return an error if .
    pub fn setup() -> Result<Self, SettingsManagerError> {
        let config = Path::new(CONFIG_FOLDER);
        let backup = Path::new(CONFIG_BACKUP_FOLDER);
        let error = Path::new(CONFIG_ERROR_FOLDER);

        let default = Path::new(CONFIG_FOLDER).join(CONFIG_DEFAULT).with_extension("json");
        let old = Path::new(CONFIG_FOLDER).join(CONFIG_OLD).with_extension("toml");
        let local = Path::new(CONFIG_FOLDER).join(CONFIG_LOCAL).with_extension("json");

        Self::make_folder(config)?;

        Self::write_default(default.as_path())?;
        let manager = Self::load(old.as_path(), local.as_path(), backup, error)?;

        manager.save(local.as_path(), &Some(backup.into()))?;

        Ok(manager)
    }

    /// .
    ///
    /// # Errors
    ///
    /// This function will return an error if .
    pub fn load(old: &Path, local: &Path, backup_folder: &Path, error_folder: &Path) -> Result<Self, SettingsManagerError> {
        if let Some(res) = Self::import_old(old, backup_folder, error_folder)? {
            return Ok(res);
        }

        // If no old settings, lets try the local settings.
        let local_settings = match Self::read(local) {
            Ok(settings) => Some(settings),
            Err(err) => match err {
                SettingsManagerError::FailedToOpenFileForReading { .. } => {
                    info!("No Configuration To Load: {err}");
                    None
                }
                err => {
                    return Err(err);
                }
            },
        };

        if let Some(res) = local_settings {
            return Ok(res);
        };

        // if nothing else, lets load the default.
        Ok(SettingsManager::default())
    }

    /// .
    ///
    /// # Errors
    ///
    /// This function will return an error if .
    pub fn save(&self, to: &Path, archive_folder: &Option<Box<Path>>) -> Result<(), SettingsManagerError> {
        // lets backup the previous configuration, if we have any...
        let existing = get_file_at(to, OpenOptions::new().read(true)).ok();

        if let Some(existing) = existing {
            if let Some(archive_folder) = archive_folder {
                Self::archive(existing.0, &existing.1, archive_folder)?;
            }
        }

        let dest = get_file_at(to, OpenOptions::new().write(true).create(true).truncate(true)).map_err(|err| {
            SettingsManagerError::FailedToOpenFileForWriting {
                source: (Arc::new(err) as Arc<dyn std::error::Error + Send + Sync>).into(),
            }
        })?;

        self.write(dest.0)
    }

    /// .
    ///
    /// # Errors
    ///
    /// This function will return an error if .
    pub fn write_default(to: &Path) -> Result<(), SettingsManagerError> {
        let dest = get_file_at(to, OpenOptions::new().write(true).create(true).truncate(true)).map_err(|err| {
            SettingsManagerError::FailedToOpenFileForWriting {
                source: (Arc::new(err) as Arc<dyn std::error::Error + Send + Sync>).into(),
            }
        })?;

        Self::default().write(dest.0)
    }

    /// .
    ///
    /// # Errors
    ///
    /// This function will return an error if .
    pub fn read(from: &Path) -> Result<Self, SettingsManagerError> {
        let source =
            get_file_at(from, OpenOptions::new().read(true)).map_err(|err| SettingsManagerError::FailedToOpenFileForReading {
                source: (Arc::new(err) as Arc<dyn std::error::Error + Send + Sync>).into(),
            })?;

        Self::read_json(source.0)
    }

    /// .
    ///
    /// # Errors
    ///
    /// This function will return an error if .
    pub fn write(&self, writer: impl Write) -> Result<(), SettingsManagerError> {
        self.write_json(writer)
    }

    /// .
    ///
    /// # Errors
    ///
    /// This function will return an error if .
    pub fn read_json<R>(mut rdr: R) -> Result<Self, SettingsManagerError>
    where
        R: Read,
    {
        let data: &mut Vec<u8> = &mut Vec::default();

        rdr.read_to_end(data)
            .map_err(|err| SettingsManagerError::FailedToReadFromBuffer {
                source: (Arc::new(err) as Arc<dyn std::error::Error + Send + Sync>).into(),
            })?;

        let settings = serde_json::from_reader::<Cursor<&mut Vec<u8>>, SettingsNamespace>(Cursor::new(data)).map_err(|err| {
            SettingsManagerError::FailedToDeserializeFromJson {
                message: "(read as \"SettingsNamespace\")".to_string(),
                source: (Arc::new(err) as Arc<dyn std::error::Error + Send + Sync>).into(),
            }
        })?;
        {
            match settings.namespace.as_str() {
                SETTINGS_NAMESPACE => serde_json::from_reader::<Cursor<&mut Vec<u8>>, Settings>(Cursor::new(data))
                    .map_err(|err| SettingsManagerError::FailedToDeserializeFromJson {
                        message: "(read as \"Settings\")".to_string(),
                        source: (Arc::new(err) as Arc<dyn std::error::Error + Send + Sync>).into(),
                    })
                    .map(SettingsManager::from),

                SETTINGS_NAMESPACE_ERRORED => serde_json::from_reader::<Cursor<&mut Vec<u8>>, SettingsErrored>(Cursor::new(data))
                    .map_err(|err| SettingsManagerError::FailedToDeserializeFromJson {
                        message: "(read as \"SettingsErrored\")".to_string(),
                        source: (Arc::new(err) as Arc<dyn std::error::Error + Send + Sync>).into(),
                    })
                    .map(SettingsManager::from),

                namespace => Err(SettingsManagerError::FailedToMatchNamespace {
                    namespace: namespace.to_string(),
                }),
            }
        }
    }

    /// .
    ///
    /// # Errors
    ///
    /// This function will return an error if .
    pub fn write_json<W>(&self, writer: W) -> Result<(), SettingsManagerError>
    where
        W: Write,
    {
        match &self.settings {
            Ok(okay) => {
                serde_json::to_writer_pretty(writer, okay).map_err(|err| SettingsManagerError::FailedToDeserializeFromJson {
                    message: "(read as \"Settings\")".to_string(),
                    source: (Arc::new(err) as Arc<dyn std::error::Error + Send + Sync>).into(),
                })
            }
            Err(err) => {
                serde_json::to_writer_pretty(writer, err).map_err(|err| SettingsManagerError::FailedToDeserializeFromJson {
                    message: "(read as \"SettingsErrored\")".to_string(),
                    source: (Arc::new(err) as Arc<dyn std::error::Error + Send + Sync>).into(),
                })
            }
        }
    }

    fn backup(&self, to: &Path, folder: &Path) -> Result<(), SettingsManagerError> {
        let ext = match to.extension().map(std::ffi::OsStr::to_os_string) {
            Some(mut ext) => {
                ext.push(".json");
                ext
            }
            None => OsString::from("json"),
        };

        let data: &mut Vec<u8> = &mut Vec::default();

        self.write_json(data.by_ref())
            .map_err(|err| SettingsManagerError::FailedToWriteToFile {
                message: "(backup)".to_string(),
                to: to.into(),

                source: (Arc::new(err) as Arc<dyn std::error::Error + Send + Sync>).into(),
            })?;

        Self::archive(Cursor::new(data), &to.with_extension(ext), folder)?;
        Ok(())
    }

    fn archive(mut rdr: impl Read, from: &Path, to_folder: &Path) -> Result<(), SettingsManagerError> {
        Self::make_folder(to_folder)?;

        let to_folder = to_folder
            .canonicalize()
            .map_err(|err| SettingsManagerError::FailedToResolvePath {
                at: to_folder.into(),
                source: (Arc::new(err) as Arc<dyn std::error::Error + Send + Sync>).into(),
            })?;

        let mut hasher: DefaultHasher = DefaultHasher::default();
        let data: &mut Vec<u8> = &mut Vec::default();

        // todo: lock and stream the file instead of loading the full file into memory.
        let _size = rdr
            .read_to_end(data)
            .map_err(|err| SettingsManagerError::FailedToReadFromBuffer {
                source: (Arc::new(err) as Arc<dyn std::error::Error + Send + Sync>).into(),
            })
            .map_err(|err| SettingsManagerError::FailedToReadFromFile {
                message: "(archive, read into)".to_string(),
                from: from.into(),
                source: (Arc::new(err) as Arc<dyn std::error::Error + Send + Sync>).into(),
            })?;

        data.hash(&mut hasher);

        let ext = match from.extension() {
            Some(ext) => {
                let mut ostr = OsString::from(format!("{}.", hasher.finish()));
                ostr.push(ext);
                ostr
            }
            None => OsString::from(hasher.finish().to_string()),
        };

        let to = to_folder.join(from.file_name().unwrap()).with_extension(ext);

        // if we do not have a backup already, lets make one.
        if to.canonicalize().is_err() {
            let mut dest = get_file_at(&to, OpenOptions::new().write(true).create_new(true)).map_err(|err| {
                SettingsManagerError::FailedToCreateNewFile {
                    source: (Arc::new(err) as Arc<dyn std::error::Error + Send + Sync>).into(),
                }
            })?;

            dest.0.write_all(data).map_err(|a| {
                let b = SettingsManagerError::FailedToWriteIntoBuffer {
                    source: (Arc::new(a) as Arc<dyn std::error::Error + Send + Sync>).into(),
                };

                SettingsManagerError::FailedToWriteToFile {
                    to: dest.1,
                    message: "(archive, making backup)".to_string(),
                    source: (Arc::new(b) as Arc<dyn std::error::Error + Send + Sync>).into(),
                }
            })?;
        };

        Ok(())
    }

    /// .
    ///
    /// # Panics
    ///
    /// Panics if .
    ///
    /// # Errors
    ///
    /// This function will return an error if .
    #[allow(clippy::too_many_lines)]
    pub fn import_old(from: &Path, backup_folder: &Path, error_folder: &Path) -> Result<Option<Self>, SettingsManagerError> {
        let import_error_folder = error_folder.join("import");

        let Ok(mut file) = get_file_at(from, OpenOptions::new().read(true)) else { return Ok(None) };

        let data: &mut Vec<u8> = &mut Vec::default();

        let _ = file.0.read_to_end(data).map_err(|a| {
            let b = SettingsManagerError::FailedToReadFromBuffer {
                source: (Arc::new(a) as Arc<dyn std::error::Error + Send + Sync>).into(),
            };
            let c = SettingsManagerError::FailedToReadFromFile {
                message: "(old_file)".to_string(),
                from: file.1.clone(),
                source: (Arc::new(b) as Arc<dyn std::error::Error + Send + Sync>).into(),
            };

            SettingsManagerError::FailedToProcessOldSettings {
                source: (Arc::new(c) as Arc<dyn std::error::Error + Send + Sync>).into(),
            }
        })?;

        let parsed = toml::de::from_slice(data.as_slice()).map_err(|a| {
            let b = SettingsManagerError::FailedToDeserializeFromToml {
                source: (Arc::new(a) as Arc<dyn std::error::Error + Send + Sync>).into(),
            };

            let c = SettingsManagerError::FailedToReadFromFile {
                message: "(old settings toml)".to_string(),
                from: file.1.clone(),
                source: (Arc::new(b) as Arc<dyn std::error::Error + Send + Sync>).into(),
            };

            SettingsManagerError::FailedToProcessOldSettings {
                source: (Arc::new(c) as Arc<dyn std::error::Error + Send + Sync>).into(),
            }
        })?;

        let mut builder = TrackerSettingsBuilder::empty();

        // Attempt One
        let test_builder = builder.clone().import_old(&parsed);
        {
            if let Err(err) = TryInto::<TrackerSettings>::try_into(test_builder.clone()) {
                Self::make_folder(error_folder)?;
                Self::make_folder(&import_error_folder)?;
                let test = "First";

                warn!(
                    "{} import attempt failed: {}\nWith Error: {}",
                    test,
                    import_error_folder.to_string_lossy(),
                    err
                );

                let broken = Self::error(&SettingsErrored::new(&test_builder.tracker_settings, &err));

                let ext = match file.1.extension().map(std::ffi::OsStr::to_os_string) {
                    Some(mut ext) => {
                        ext.push(format!(".{}", test.to_lowercase()));
                        ext
                    }
                    None => OsString::from(test.to_lowercase()),
                };

                broken.backup(&file.1.with_extension(ext), import_error_folder.as_path())?;
            }

            // Replace broken with default, and remove everything else.

            builder = test_builder.tracker_settings.empty_fix().into();
        }

        // Attempt with Defaults
        let test_builder = builder.clone().import_old(&parsed);
        {
            if let Err(err) = TryInto::<TrackerSettings>::try_into(test_builder.clone()) {
                Self::make_folder(error_folder)?;
                Self::make_folder(&import_error_folder)?;
                let test = "Second";

                warn!(
                    "{} import attempt failed: {}\nWith Error: {}",
                    test,
                    import_error_folder.to_string_lossy(),
                    err
                );

                let broken = Self::error(&SettingsErrored::new(&test_builder.tracker_settings, &err));

                let ext = match file.1.extension().map(std::ffi::OsStr::to_os_string) {
                    Some(mut ext) => {
                        ext.push(format!(".{}", test.to_lowercase()));
                        ext
                    }
                    None => OsString::from(test.to_lowercase()),
                };

                broken.backup(&file.1.with_extension(ext), import_error_folder.as_path())?;
            }

            builder = test_builder.tracker_settings.clean().into();
        }

        // Final Attempt
        let settings = match TryInto::<TrackerSettings>::try_into(builder.clone()) {
            Ok(tracker) => Self {
                settings: Ok(tracker.into()),
            },

            Err(err) => {
                Self::make_folder(error_folder)?;
                Self::make_folder(&import_error_folder)?;
                let test = "Final";

                warn!(
                    "{} import attempt failed: {}\nWith Error: {}",
                    test,
                    import_error_folder.to_string_lossy(),
                    err
                );

                let broken = Self::error(&SettingsErrored::new(&builder.tracker_settings, &err));

                let ext = match file.1.extension().map(std::ffi::OsStr::to_os_string) {
                    Some(mut ext) => {
                        ext.push(format!(".{}", test.to_lowercase()));
                        ext
                    }
                    None => OsString::from(test.to_lowercase()),
                };

                broken.backup(&file.1.with_extension(ext), import_error_folder.as_path())?;

                return Err(SettingsManagerError::FailedToImportOldSettings {
                    from: file.1,
                    source: (Arc::new(err) as Arc<dyn std::error::Error + Send + Sync>).into(),
                });
            }
        };

        let ext = match file.1.extension() {
            Some(ext) => {
                let mut ostr = OsString::from("old.");
                ostr.push(ext);
                ostr
            }
            None => OsString::from("old"),
        };

        // import was successful, lets rename the extension to ".toml.old".
        let backup = backup_folder.join(file.1.file_name().unwrap()).with_extension(ext);
        Self::make_folder(backup_folder)?;

        match fs::rename(&file.1, &backup) {
            Ok(_) => {
                info!(
                    "\nOld Settings Was Successfully Imported!\n And moved from: \"{}\", to: \"{}\".\n",
                    file.1.display(),
                    backup.display()
                );
                Ok(Some(settings))
            }
            Err(err) => Err(SettingsManagerError::FailedToMoveFile {
                from: file.1,
                to: backup.into_boxed_path(),
                source: (Arc::new(err) as Arc<dyn std::error::Error + Send + Sync>).into(),
            }),
        }
    }

    /// .
    ///
    /// # Errors
    ///
    /// This function will return an error if .
    pub fn make_folder(folder: &Path) -> Result<(), SettingsManagerError> {
        if let Ok(path) = folder.canonicalize() {
            if path.is_dir() {
                return Ok(());
            }
            return Err(SettingsManagerError::FailedToResolveDirectory { at: folder.into() });
        }
        match fs::create_dir(folder) {
            Ok(_) => Ok(()),
            Err(err) => Err(SettingsManagerError::FailedToPrepareDirectory {
                at: folder.into(),
                source: (Arc::new(err) as Arc<dyn std::error::Error + Send + Sync>).into(),
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::env;
    use std::fs::OpenOptions;
    use std::io::{Seek, Write};

    use thiserror::Error;
    use uuid::Uuid;

    use super::SettingsManager;
    use crate::helpers::get_file_at;
    use crate::settings::old_settings::OLD_DEFAULT;
    use crate::settings::{Settings, SettingsErrored, TrackerSettings};

    #[test]
    fn it_should_attempt_the_default_setup() {
        SettingsManager::setup().unwrap();
    }

    #[test]
    fn it_should_import_the_old_default_settings() {
        let tmp_dir = env::temp_dir();
        let old = tmp_dir.join(format!("old_default_{}.json", Uuid::new_v4()));
        let backup = tmp_dir.join("backup");
        let error = tmp_dir.join("error");

        let mut dest = get_file_at(old.as_path(), OpenOptions::new().write(true).create_new(true)).unwrap();

        dest.0.write_all(OLD_DEFAULT.as_bytes()).unwrap();

        SettingsManager::import_old(old.as_path(), backup.as_path(), error.as_path()).unwrap();
    }

    #[test]
    fn it_should_write_and_read_the_default() {
        let temp = env::temp_dir().as_path().join(format!("test_config_{}.json", Uuid::new_v4()));

        assert!(!temp.exists());

        SettingsManager::write_default(&temp).unwrap();

        assert!(temp.is_file());

        let manager = SettingsManager::read(&temp).unwrap();

        assert_eq!(manager, SettingsManager::default());
    }

    #[test]
    fn it_should_make_config_folder() {
        let temp = env::temp_dir().as_path().join(format!("test_config_{}", Uuid::new_v4()));

        assert!(!temp.exists());

        SettingsManager::make_folder(&temp).unwrap();

        assert!(temp.is_dir());
    }

    #[test]
    fn it_should_write_and_read_errored_settings() {
        #[derive(Error, Debug)]
        enum TestErrors {
            #[error("Test Error!")]
            Error,
        }

        let path = env::temp_dir().as_path().join(format!("test_errored.{}", Uuid::new_v4()));
        let mut file_rw = get_file_at(&path, OpenOptions::new().write(true).read(true).create_new(true)).unwrap();

        let errored: SettingsManager = SettingsErrored::new(&TrackerSettings::default(), &TestErrors::Error).into();

        errored.write_json(std::io::Write::by_ref(&mut file_rw.0)).unwrap();
        file_rw.0.rewind().unwrap();

        let error_returned = SettingsManager::read_json(file_rw.0).unwrap();

        assert_eq!(errored, error_returned);
    }

    #[test]
    fn it_should_write_and_read_settings() {
        let path = env::temp_dir().as_path().join(format!("test_errored.{}", Uuid::new_v4()));
        let mut file_rw = get_file_at(&path, OpenOptions::new().write(true).read(true).create_new(true)).unwrap();

        let settings: SettingsManager = Settings::default().into();

        settings.write_json(std::io::Write::by_ref(&mut file_rw.0)).unwrap();
        file_rw.0.rewind().unwrap();

        let settings_returned = SettingsManager::read_json(file_rw.0).unwrap();

        assert_eq!(settings, settings_returned);
    }
}
