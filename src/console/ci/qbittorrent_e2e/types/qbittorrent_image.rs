use std::fmt;
use std::ops::Deref;

/// A Docker image reference for a qBittorrent service container.
///
/// Keeping this distinct from [`TrackerImage`] turns an accidental swap of the
/// two image arguments into a compile error.
#[derive(Debug, Clone)]
pub(crate) struct QbittorrentImage(String);

impl QbittorrentImage {
    /// Creates a new [`QbittorrentImage`] from any value that converts into a [`String`].
    pub(crate) fn new(image: impl Into<String>) -> Self {
        Self(image.into())
    }

    /// Returns the image reference as a `&str`.
    pub(crate) fn as_str(&self) -> &str {
        &self.0
    }
}

impl Deref for QbittorrentImage {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl fmt::Display for QbittorrentImage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::QbittorrentImage;

    #[test]
    fn it_should_round_trip_image_string() {
        let image = QbittorrentImage::new("lscr.io/linuxserver/qbittorrent:5.1.4");

        assert_eq!(image.as_str(), "lscr.io/linuxserver/qbittorrent:5.1.4");
        assert_eq!(&*image, "lscr.io/linuxserver/qbittorrent:5.1.4");
        assert_eq!(image.to_string(), "lscr.io/linuxserver/qbittorrent:5.1.4");
    }
}
