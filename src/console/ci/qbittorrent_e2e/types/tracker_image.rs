use std::fmt;
use std::ops::Deref;

/// A Docker image reference for the Torrust tracker service.
///
/// Keeping this distinct from [`QbittorrentImage`] turns an accidental swap of
/// the two image arguments into a compile error.
#[derive(Debug, Clone)]
pub(crate) struct TrackerImage(String);

impl TrackerImage {
    /// Creates a new [`TrackerImage`] from any value that converts into a [`String`].
    pub(crate) fn new(image: impl Into<String>) -> Self {
        Self(image.into())
    }

    /// Returns the image reference as a `&str`.
    pub(crate) fn as_str(&self) -> &str {
        &self.0
    }
}

impl Deref for TrackerImage {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl fmt::Display for TrackerImage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::TrackerImage;

    #[test]
    fn it_should_round_trip_image_string() {
        let image = TrackerImage::new("torrust/tracker:latest");

        assert_eq!(image.as_str(), "torrust/tracker:latest");
        assert_eq!(&*image, "torrust/tracker:latest");
        assert_eq!(image.to_string(), "torrust/tracker:latest");
    }
}
