// adr: docs/adrs/20260721100000_use_newtypes_for_constrained_configuration_field_types.md
// This module is the canonical implementation of the newtype pattern for domain-constrained
// configuration fields.  Read the ADR above before adding a new constrained config field type.

//! Validated URL newtypes for `public_url` fields in v3 configuration structs.
//!
//! Each tracker-instance config struct (`HttpTracker`, `UdpTracker`, `HttpApi`) carries
//! an optional `public_url` field typed as either [`HttpUrl`] or [`UdpUrl`].  The scheme
//! constraint is encoded in the type, so consuming code never needs to re-validate:
//!
//! - [`HttpUrl`] — accepts `http://` or `https://` only (`HttpTracker`, `HttpApi`)
//! - [`UdpUrl`]  — accepts `udp://` only (`UdpTracker`)
//!
//! Both types implement [`serde::Serialize`] / [`serde::Deserialize`] as plain strings, so
//! they round-trip transparently through TOML.  Validation happens at deserialization time;
//! after that the invariant is guaranteed by the type.

use std::fmt;

use serde::{Deserialize, Deserializer, Serialize, Serializer, de};
use url::Url;

// ── HttpUrl ──────────────────────────────────────────────────────────────────

/// A URL that is guaranteed to use the `http` or `https` scheme.
///
/// Used for the `public_url` field of HTTP-based service configs
/// (`HttpTracker`, `HttpApi`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HttpUrl(Url);

impl HttpUrl {
    /// Construct an `HttpUrl` from an already-parsed [`Url`].
    ///
    /// # Errors
    ///
    /// Returns an error string if the scheme is not `http` or `https`.
    pub fn new(url: Url) -> Result<Self, String> {
        match url.scheme() {
            "http" | "https" => Ok(Self(url)),
            scheme => Err(format!("invalid scheme '{scheme}': expected 'http' or 'https'")),
        }
    }

    /// Parse a string into an `HttpUrl`, validating both structure and scheme.
    ///
    /// # Errors
    ///
    /// Returns an error string if `s` is not a valid URL or its scheme is not `http` or `https`.
    pub fn parse(s: &str) -> Result<Self, String> {
        let url = Url::parse(s).map_err(|e| format!("invalid URL '{s}': {e}"))?;
        Self::new(url)
    }

    /// Returns the URL as a `&str`.
    #[must_use]
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    /// Returns a reference to the inner [`Url`].
    #[must_use]
    pub const fn as_url(&self) -> &Url {
        &self.0
    }
}

impl fmt::Display for HttpUrl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl AsRef<str> for HttpUrl {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl AsRef<Url> for HttpUrl {
    fn as_ref(&self) -> &Url {
        self.as_url()
    }
}

impl Serialize for HttpUrl {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.0.as_str().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for HttpUrl {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        Self::parse(&s).map_err(de::Error::custom)
    }
}

// ── UdpUrl ───────────────────────────────────────────────────────────────────

/// A URL that is guaranteed to use the `udp` scheme.
///
/// Used for the `public_url` field of UDP tracker configs (`UdpTracker`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UdpUrl(Url);

impl UdpUrl {
    /// Construct a `UdpUrl` from an already-parsed [`Url`].
    ///
    /// # Errors
    ///
    /// Returns an error string if the scheme is not `udp`.
    pub fn new(url: Url) -> Result<Self, String> {
        match url.scheme() {
            "udp" => Ok(Self(url)),
            scheme => Err(format!("invalid scheme '{scheme}': expected 'udp'")),
        }
    }

    /// Parse a string into a `UdpUrl`, validating both structure and scheme.
    ///
    /// # Errors
    ///
    /// Returns an error string if `s` is not a valid URL or its scheme is not `udp`.
    pub fn parse(s: &str) -> Result<Self, String> {
        let url = Url::parse(s).map_err(|e| format!("invalid URL '{s}': {e}"))?;
        Self::new(url)
    }

    /// Returns the URL as a `&str`.
    #[must_use]
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    /// Returns a reference to the inner [`Url`].
    #[must_use]
    pub const fn as_url(&self) -> &Url {
        &self.0
    }
}

impl fmt::Display for UdpUrl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl AsRef<str> for UdpUrl {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl AsRef<Url> for UdpUrl {
    fn as_ref(&self) -> &Url {
        self.as_url()
    }
}

impl Serialize for UdpUrl {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.0.as_str().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for UdpUrl {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        Self::parse(&s).map_err(de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use serde::Deserialize;

    use super::{HttpUrl, UdpUrl};

    #[derive(Debug, Deserialize)]
    struct HttpFixture {
        #[serde(default)]
        public_url: Option<HttpUrl>,
    }

    #[derive(Debug, Deserialize)]
    struct UdpFixture {
        #[serde(default)]
        public_url: Option<UdpUrl>,
    }

    // ── HttpUrl ──────────────────────────────────────────────────────────────

    #[test]
    fn it_should_accept_http_url_when_scheme_is_http() {
        // Arrange
        let toml = r#"public_url = "http://tracker.example.com/announce""#; // DevSkim: ignore DS137138

        // Act
        let fixture: HttpFixture = toml::from_str(toml).expect("http:// should be accepted");

        // Assert
        assert_eq!(
            fixture.public_url.as_ref().map(HttpUrl::as_str),
            Some("http://tracker.example.com/announce") // DevSkim: ignore DS137138
        );
    }

    #[test]
    fn it_should_accept_http_url_when_scheme_is_https() {
        // Arrange
        let toml = r#"public_url = "https://tracker.example.com/announce""#;

        // Act
        let fixture: HttpFixture = toml::from_str(toml).expect("https:// should be accepted");

        // Assert
        assert_eq!(
            fixture.public_url.as_ref().map(HttpUrl::as_str),
            Some("https://tracker.example.com/announce")
        );
    }

    #[test]
    fn it_should_default_to_none_when_http_url_field_is_absent() {
        // Arrange
        let toml = "";

        // Act
        let fixture: HttpFixture = toml::from_str(toml).expect("absent field should default to None");

        // Assert
        assert!(fixture.public_url.is_none());
    }

    #[test]
    fn it_should_reject_http_url_when_scheme_is_udp() {
        // Arrange
        let toml = r#"public_url = "udp://tracker.example.com:6969""#;

        // Act
        let result = toml::from_str::<HttpFixture>(toml);

        // Assert
        let err = result.expect_err("udp:// scheme should be rejected for HttpUrl");
        assert!(
            err.to_string().contains("invalid scheme"),
            "expected scheme error, got: {err}"
        );
    }

    #[test]
    fn it_should_reject_http_url_when_value_is_not_a_valid_url() {
        // Arrange
        let toml = r#"public_url = "not-a-url""#;

        // Act
        let result = toml::from_str::<HttpFixture>(toml);

        // Assert
        let err = result.expect_err("malformed URL should be rejected");
        assert!(err.to_string().contains("invalid URL"), "expected parse error, got: {err}");
    }

    #[test]
    fn it_should_round_trip_http_url_through_toml_serialization() {
        // Arrange
        #[derive(serde::Serialize, serde::Deserialize)]
        struct Wrapper {
            public_url: HttpUrl,
        }
        let original = Wrapper {
            public_url: HttpUrl::parse("https://tracker.example.com/announce").unwrap(),
        };

        // Act
        let toml_str = toml::to_string(&original).unwrap();
        let parsed: Wrapper = toml::from_str(&toml_str).unwrap();

        // Assert
        assert_eq!(original.public_url, parsed.public_url);
    }

    // ── UdpUrl ───────────────────────────────────────────────────────────────

    #[test]
    fn it_should_accept_udp_url_when_scheme_is_udp() {
        // Arrange
        let toml = r#"public_url = "udp://tracker.example.com:6969""#;

        // Act
        let fixture: UdpFixture = toml::from_str(toml).expect("udp:// should be accepted");

        // Assert
        assert_eq!(
            fixture.public_url.as_ref().map(UdpUrl::as_str),
            Some("udp://tracker.example.com:6969")
        );
    }

    #[test]
    fn it_should_default_to_none_when_udp_url_field_is_absent() {
        // Arrange
        let toml = "";

        // Act
        let fixture: UdpFixture = toml::from_str(toml).expect("absent field should default to None");

        // Assert
        assert!(fixture.public_url.is_none());
    }

    #[test]
    fn it_should_reject_udp_url_when_scheme_is_http() {
        // Arrange
        let toml = r#"public_url = "https://tracker.example.com/announce""#;

        // Act
        let result = toml::from_str::<UdpFixture>(toml);

        // Assert
        let err = result.expect_err("https:// scheme should be rejected for UdpUrl");
        assert!(
            err.to_string().contains("invalid scheme"),
            "expected scheme error, got: {err}"
        );
    }

    #[test]
    fn it_should_reject_udp_url_when_value_is_not_a_valid_url() {
        // Arrange
        let toml = r#"public_url = "not-a-url""#;

        // Act
        let result = toml::from_str::<UdpFixture>(toml);

        // Assert
        let err = result.expect_err("malformed URL should be rejected");
        assert!(err.to_string().contains("invalid URL"), "expected parse error, got: {err}");
    }

    #[test]
    fn it_should_round_trip_udp_url_through_toml_serialization() {
        // Arrange
        #[derive(serde::Serialize, serde::Deserialize)]
        struct Wrapper {
            public_url: UdpUrl,
        }
        let original = Wrapper {
            public_url: UdpUrl::parse("udp://tracker.example.com:6969").unwrap(),
        };

        // Act
        let toml_str = toml::to_string(&original).unwrap();
        let parsed: Wrapper = toml::from_str(&toml_str).unwrap();

        // Assert
        assert_eq!(original.public_url, parsed.public_url);
    }
}
