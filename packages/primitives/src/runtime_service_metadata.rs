use crate::{ConfigurationInstanceId, ServiceRole};

/// Immutable tracker metadata attached to a started local service registration.
///
/// The registry owns neither the role nor the configuration identity; it stores
/// this tracker-owned value without assigning it application semantics.
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Hash, Clone, Copy)]
pub struct RuntimeServiceMetadata {
    service_role: ServiceRole,
    configuration_instance_id: ConfigurationInstanceId,
}

impl RuntimeServiceMetadata {
    /// Creates metadata for a canonical tracker service instance.
    #[must_use]
    pub const fn new(service_role: ServiceRole, configuration_instance_id: ConfigurationInstanceId) -> Self {
        Self {
            service_role,
            configuration_instance_id,
        }
    }

    /// Returns the role implemented by the started listener.
    #[must_use]
    pub const fn service_role(self) -> ServiceRole {
        self.service_role
    }

    /// Returns the source configuration instance for the listener.
    #[must_use]
    pub const fn configuration_instance_id(self) -> ConfigurationInstanceId {
        self.configuration_instance_id
    }
}
