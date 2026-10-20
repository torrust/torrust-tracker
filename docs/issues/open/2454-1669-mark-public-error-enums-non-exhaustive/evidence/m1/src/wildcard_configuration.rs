use torrust_tracker_configuration::Error;

fn describe(error: &Error) -> &'static str {
    match error {
        Error::UnableToLoadExplicitConfigFile { .. } => "unable to load explicit file",
        Error::UnableToProcessExplicitConfigFile { .. } => "unable to process explicit file",
        Error::UnableToLoadFromEnvironmentVariable { .. } => "unable to load from env",
        Error::UnableToLoadFromConfigFile { .. } => "unable to load from file",
        Error::ConfigError { .. } => "config error",
        Error::UnsupportedVersion { .. } => "unsupported version",
        Error::MissingMandatoryOption { .. } => "missing mandatory option",
        _ => "unknown",
    }
}

fn main() {
    let _ = describe;
}
