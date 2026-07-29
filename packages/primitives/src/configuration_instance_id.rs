use serde::Serialize;

use crate::ServiceRole;

/// Identifies one configured tracker service instance for a process lifetime.
///
/// Equality includes the tracker [`ServiceRole`] and the zero-based index in
/// that role's configuration-entry list. The identifier deliberately excludes
/// configured and final socket addresses, because repeated port-zero bindings
/// are valid. It is neither user supplied nor persistent across configuration
/// reordering.
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Hash, Clone, Copy, Serialize)]
pub struct ConfigurationInstanceId {
    service_role: ServiceRole,
    instance_index: usize,
}

impl ConfigurationInstanceId {
    /// Creates an identifier for a role-qualified configuration entry.
    #[must_use]
    pub const fn new(service_role: ServiceRole, instance_index: usize) -> Self {
        Self {
            service_role,
            instance_index,
        }
    }

    /// Returns the tracker role configured for this instance.
    #[must_use]
    pub const fn service_role(self) -> ServiceRole {
        self.service_role
    }

    /// Returns the zero-based index in the role's configuration-entry list.
    #[must_use]
    pub const fn instance_index(self) -> usize {
        self.instance_index
    }
}

#[cfg(test)]
mod tests {
    use crate::{ConfigurationInstanceId, ServiceRole};

    #[test]
    fn it_should_identify_equal_role_and_index_as_the_same_instance() {
        // Arrange
        let first_instance = ConfigurationInstanceId::new(ServiceRole::HttpTracker, 0);
        let same_instance = ConfigurationInstanceId::new(ServiceRole::HttpTracker, 0);

        // Act
        let are_equal = first_instance == same_instance;

        // Assert
        assert!(are_equal);
    }

    #[test]
    fn it_should_distinguish_instances_with_the_same_role_and_different_indices() {
        // Arrange
        let first_instance = ConfigurationInstanceId::new(ServiceRole::HttpTracker, 0);
        let second_instance = ConfigurationInstanceId::new(ServiceRole::HttpTracker, 1);

        // Act
        let are_equal = first_instance == second_instance;

        // Assert
        assert!(!are_equal);
    }

    #[test]
    fn it_should_distinguish_instances_with_the_same_index_and_different_roles() {
        // Arrange
        let http_instance = ConfigurationInstanceId::new(ServiceRole::HttpTracker, 0);
        let udp_instance = ConfigurationInstanceId::new(ServiceRole::UdpTracker, 0);

        // Act
        let are_equal = http_instance == udp_instance;

        // Assert
        assert!(!are_equal);
    }

    #[test]
    fn it_should_expose_its_role_and_zero_based_index() {
        // Arrange
        let instance_id = ConfigurationInstanceId::new(ServiceRole::UdpTracker, 1);

        // Act
        let service_role = instance_id.service_role();
        let instance_index = instance_id.instance_index();

        // Assert
        assert_eq!(service_role, ServiceRole::UdpTracker);
        assert_eq!(instance_index, 1);
    }

    #[test]
    fn it_should_serialize_the_role_and_instance_index() {
        // Arrange
        let instance_id = ConfigurationInstanceId::new(ServiceRole::HttpTracker, 0);

        // Act
        let serialized = serde_json::to_string(&instance_id).unwrap();

        // Assert
        assert_eq!(serialized, r#"{"service_role":"http_tracker","instance_index":0}"#);
    }
}
