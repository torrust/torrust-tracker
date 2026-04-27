use std::fmt;
use std::ops::Deref;
use std::path::Path;

/// A file name (base name only, no path separators).
///
/// Wraps a [`String`] and provides [`Deref`] to `str` so values can be used
/// directly wherever `&str` is expected, and [`AsRef<Path>`] so they can be
/// passed to [`Path::join`].
#[derive(Debug, Clone)]
pub(crate) struct FileName(String);

impl FileName {
    /// Creates a new [`FileName`] from any value that converts into a [`String`].
    pub(crate) fn new(name: impl Into<String>) -> Self {
        Self(name.into())
    }
}

impl Deref for FileName {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AsRef<Path> for FileName {
    fn as_ref(&self) -> &Path {
        Path::new(&self.0)
    }
}

impl fmt::Display for FileName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl From<String> for FileName {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl From<&str> for FileName {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::FileName;

    #[test]
    fn it_should_build_from_new_and_format_as_string() {
        let file_name = FileName::new("payload.bin");

        assert_eq!(&*file_name, "payload.bin");
        assert_eq!(file_name.to_string(), "payload.bin");
    }

    #[test]
    fn it_should_convert_from_string_and_str() {
        let from_string = FileName::from(String::from("a.torrent"));
        let from_str = FileName::from("b.torrent");

        assert_eq!(&*from_string, "a.torrent");
        assert_eq!(&*from_str, "b.torrent");
    }

    #[test]
    fn it_should_implement_as_ref_path() {
        let file_name = FileName::new("nested/file.txt");

        assert_eq!(file_name.as_ref(), Path::new("nested/file.txt"));
    }
}
