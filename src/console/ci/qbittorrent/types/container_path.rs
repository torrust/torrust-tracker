use std::fmt;
use std::ops::Deref;

/// An absolute path inside a Docker container (e.g. `"/downloads"`).
///
/// Distinct from host [`PathBuf`]s: a `ContainerPath` is always a
/// Linux-style absolute path that exists only within the container
/// file-system, never on the host.
///
/// [`PathBuf`]: std::path::PathBuf
#[derive(Debug, Clone)]
pub(crate) struct ContainerPath(String);

impl ContainerPath {
    /// Creates a new [`ContainerPath`] from any value that converts into a [`String`].
    pub(crate) fn new(path: impl Into<String>) -> Self {
        Self(path.into())
    }
}

impl Deref for ContainerPath {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl fmt::Display for ContainerPath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl From<String> for ContainerPath {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl From<&str> for ContainerPath {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::ContainerPath;

    #[test]
    fn it_should_build_from_new_and_format_as_string() {
        let path = ContainerPath::new("/downloads");

        assert_eq!(&*path, "/downloads");
        assert_eq!(path.to_string(), "/downloads");
    }

    #[test]
    fn it_should_convert_from_string_and_str() {
        let from_string = ContainerPath::from(String::from("/a"));
        let from_str = ContainerPath::from("/b");

        assert_eq!(&*from_string, "/a");
        assert_eq!(&*from_str, "/b");
    }
}
