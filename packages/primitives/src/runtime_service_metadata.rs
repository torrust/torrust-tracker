use crate::{ConfigurationInstanceId, ServiceRole};

/// Immutable tracker metadata attached to a started local service registration.
///
/// The registry owns neither the role nor the configuration identity; it stores
/// this tracker-owned value without assigning it application semantics.
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Hash, Clone, Copy)]
pub struct RuntimeServiceMetadata {
    configuration_instance_id: ConfigurationInstanceId,
}

impl RuntimeServiceMetadata {
    /// Creates metadata for a canonical tracker service instance.
    #[must_use]
    pub const fn new(configuration_instance_id: ConfigurationInstanceId) -> Self {
        Self {
            configuration_instance_id,
        }
    }

    /// Returns the role implemented by the started listener.
    #[must_use]
    pub const fn service_role(self) -> ServiceRole {
        self.configuration_instance_id.service_role()
    }

    /// Returns the source configuration instance for the listener.
    #[must_use]
    pub const fn configuration_instance_id(self) -> ConfigurationInstanceId {
        self.configuration_instance_id
    }
}

#[cfg(test)]
mod tests {
    use crate::{ConfigurationInstanceId, RuntimeServiceMetadata, ServiceRole};

    #[test]
    fn it_should_derive_the_role_from_the_configuration_instance_identity() {
        let configuration_instance_id = ConfigurationInstanceId::new(ServiceRole::UdpTracker, 1);
        let metadata = RuntimeServiceMetadata::new(configuration_instance_id);

        assert_eq!(metadata.service_role(), ServiceRole::UdpTracker);
        assert_eq!(metadata.configuration_instance_id(), configuration_instance_id);
    }
}
