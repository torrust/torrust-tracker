use std::fmt;
use std::ops::Deref;
use std::path::Path;

/// A file name (base name only, no path separators).
///
/// Wraps a [`String`] and provides [`Deref`] to `str` so values can be used
/// directly wherever `&str` is expected, and [`AsRef<Path>`] so they can be
/// passed to [`Path::join`].
///
/// # Invariant
///
/// The wrapped string must not contain `/`, `\`, or the component `..`.
/// Construction fails with a panic in debug builds and returns an error via
/// the `TryFrom` impl when the invariant is violated.
#[derive(Debug, Clone)]
pub(crate) struct FileName(String);

/// Error returned when a string is not a valid base file name.
#[derive(Debug)]
pub(crate) struct InvalidFileName(String);

impl fmt::Display for InvalidFileName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "invalid file name (must not contain path separators or '..'): {:?}",
            self.0
        )
    }
}

impl std::error::Error for InvalidFileName {}

fn validate(name: &str) -> Result<(), InvalidFileName> {
    if name.contains('/') || name.contains('\\') || name == ".." || name.contains("/..") || name.contains("../") {
        return Err(InvalidFileName(name.to_string()));
    }
    Ok(())
}

impl FileName {
    /// Creates a new [`FileName`].
    ///
    /// # Panics
    ///
    /// Panics if `name` contains `/`, `\`, or the path component `..`.
    pub(crate) fn new(name: impl Into<String>) -> Self {
        let s = name.into();
        validate(&s).expect("FileName invariant violated");
        Self(s)
    }
}

impl TryFrom<String> for FileName {
    type Error = InvalidFileName;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        validate(&s)?;
        Ok(Self(s))
    }
}

impl TryFrom<&str> for FileName {
    type Error = InvalidFileName;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        validate(s)?;
        Ok(Self(s.to_string()))
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
        let from_string = FileName::try_from(String::from("a.torrent")).unwrap();
        let from_str = FileName::try_from("b.torrent").unwrap();

        assert_eq!(&*from_string, "a.torrent");
        assert_eq!(&*from_str, "b.torrent");
    }

    #[test]
    fn it_should_implement_as_ref_path() {
        let file_name = FileName::new("file.txt");

        assert_eq!(file_name.as_ref(), Path::new("file.txt"));
    }

    #[test]
    fn it_should_reject_forward_slash() {
        let result = FileName::try_from("nested/file.txt");
        assert!(result.is_err());
    }

    #[test]
    fn it_should_reject_backslash() {
        let result = FileName::try_from("nested\\file.txt");
        assert!(result.is_err());
    }

    #[test]
    fn it_should_reject_double_dot() {
        let result = FileName::try_from("..");
        assert!(result.is_err());
    }
}
