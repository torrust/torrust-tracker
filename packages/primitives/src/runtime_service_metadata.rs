use url::Url;

use crate::{ConfigurationInstanceId, ServiceRole};

/// Immutable listener-specific metadata attached to a started service registration.
///
/// It combines the identity of the source configuration entry with configured
/// observability data that describes the same listener. The registry stores
/// this tracker-owned value without assigning it application semantics.
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Hash, Clone)]
pub struct RuntimeServiceMetadata {
    /// Identifies the source configuration entry for this listener.
    configuration_instance_id: ConfigurationInstanceId,
    /// Configured, operator-declared external endpoint for this listener.
    ///
    /// This does not identify the local bind address or its post-bind service
    /// binding.
    public_url: Option<Url>,
}

impl RuntimeServiceMetadata {
    /// Creates metadata for a canonical tracker service instance.
    #[must_use]
    pub const fn new(configuration_instance_id: ConfigurationInstanceId) -> Self {
        Self {
            configuration_instance_id,
            public_url: None,
        }
    }

    /// Adds the configured public URL for the listener.
    #[must_use]
    pub fn with_public_url(mut self, public_url: Option<Url>) -> Self {
        self.public_url = public_url;
        self
    }

    /// Returns the role implemented by the started listener.
    #[must_use]
    pub const fn service_role(&self) -> ServiceRole {
        self.configuration_instance_id.service_role()
    }

    /// Returns the source configuration instance for the listener.
    #[must_use]
    pub const fn configuration_instance_id(&self) -> ConfigurationInstanceId {
        self.configuration_instance_id
    }

    /// Returns the configured public URL for the listener, when present.
    #[must_use]
    pub fn public_url(&self) -> Option<&Url> {
        self.public_url.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use url::Url;

    use crate::{ConfigurationInstanceId, RuntimeServiceMetadata, ServiceRole};

    #[test]
    fn it_should_derive_the_role_from_the_configuration_instance_identity() {
        let configuration_instance_id = ConfigurationInstanceId::new(ServiceRole::UdpTracker, 1);
        let metadata = RuntimeServiceMetadata::new(configuration_instance_id);

        assert_eq!(metadata.service_role(), ServiceRole::UdpTracker);
        assert_eq!(metadata.configuration_instance_id(), configuration_instance_id);
        assert_eq!(metadata.public_url(), None);
    }

    #[test]
    fn it_should_store_an_optional_configured_public_url() {
        let metadata = RuntimeServiceMetadata::new(ConfigurationInstanceId::new(ServiceRole::HttpTracker, 0))
            .with_public_url(Some(Url::parse("https://tracker.example.test/announce").unwrap()));

        assert_eq!(
            metadata.public_url().map(Url::as_str),
            Some("https://tracker.example.test/announce")
        );
    }
}
