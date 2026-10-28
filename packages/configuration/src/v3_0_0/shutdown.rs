// adr: docs/adrs/20260723184019_separate_configuration_value_invariants_from_consistency_validation.md
// issue: #2668
//! Shutdown budgets for schema v3.
//!
//! The four budgets bound how long the tracker spends stopping after a shutdown request. They live
//! in the `[application.shutdown]` table. Every key is optional.
//!
//! Shutdown has two sequential stages. First the servers drain (HTTP, REST API and health check, and
//! the UDP request processors). Only after the servers that publish events have finished are the
//! event listeners stopped, and each of them then drains its queue. So the longest normal shutdown is
//! the slower server drain plus the listener drain, and the process deadline must cover both:
//! `max(http_drain, udp_request_drain) + event_listener_drain < deadline`. [`Shutdown`] checks that
//! relationship as a configuration consistency rule.
//!
//! The tracker cannot check the external stop timeout of Docker, systemd or Kubernetes. That timeout
//! must be at least five seconds longer than `deadline_in_secs`; see `docs/containers.md`.
use std::time::Duration;

use serde::{Deserialize, Deserializer, Serialize, Serializer, de};
use thiserror::Error;

use crate::validator::{SemanticValidationError, Validator};

/// A shutdown budget in whole seconds, from [`ShutdownBudgetInSecs::MINIMUM`] to
/// [`ShutdownBudgetInSecs::MAXIMUM`].
///
/// Zero would make a stage expire before it starts. The upper bound keeps every budget well inside
/// the range the supervisor can add to a monotonic clock without overflow, and rejects a value that is
/// almost certainly a unit mistake (for example milliseconds).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ShutdownBudgetInSecs(u64);

/// Error returned when a shutdown budget is outside the supported range.
#[derive(Debug, Error, PartialEq, Eq)]
#[error(
    "A shutdown budget must be between {minimum} and {maximum} seconds, but {value} was configured. Use a whole number of seconds within that range.",
    minimum = ShutdownBudgetInSecs::MINIMUM,
    maximum = ShutdownBudgetInSecs::MAXIMUM
)]
pub struct ShutdownBudgetOutOfRangeError {
    /// The rejected value.
    pub value: u64,
}

impl ShutdownBudgetInSecs {
    /// The smallest supported budget, in seconds.
    pub const MINIMUM: u64 = 1;

    /// The largest supported budget, in seconds (one hour).
    pub const MAXIMUM: u64 = 3600;

    /// Creates a budget after enforcing the supported range.
    ///
    /// # Errors
    ///
    /// Returns [`ShutdownBudgetOutOfRangeError`] when `value` is zero or above
    /// [`ShutdownBudgetInSecs::MAXIMUM`].
    pub const fn new(value: u64) -> Result<Self, ShutdownBudgetOutOfRangeError> {
        if value < Self::MINIMUM || value > Self::MAXIMUM {
            return Err(ShutdownBudgetOutOfRangeError { value });
        }

        Ok(Self(value))
    }

    /// Returns the validated budget in seconds.
    #[must_use]
    pub const fn get(self) -> u64 {
        self.0
    }

    /// Returns the validated budget as a [`Duration`], so a component that applies it needs no
    /// knowledge of this configuration type.
    #[must_use]
    pub const fn as_duration(self) -> Duration {
        Duration::from_secs(self.0)
    }
}

impl TryFrom<u64> for ShutdownBudgetInSecs {
    type Error = ShutdownBudgetOutOfRangeError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl From<ShutdownBudgetInSecs> for u64 {
    fn from(value: ShutdownBudgetInSecs) -> Self {
        value.get()
    }
}

impl Serialize for ShutdownBudgetInSecs {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.get().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for ShutdownBudgetInSecs {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = u64::deserialize(deserializer)?;
        Self::new(value).map_err(de::Error::custom)
    }
}

/// The `[application.shutdown]` table: how long each stage of a shutdown may take.
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct Shutdown {
    /// The process shutdown deadline, in seconds: one concurrent deadline shared by every top-level
    /// component, counted from the shutdown request. When it expires the tracker aborts what is
    /// still running and exits with a failure code.
    ///
    /// Defaults to 25. A container runtime, service manager or Kubernetes must allow at least five
    /// seconds more (30 by default) before it kills the process.
    #[serde(default = "Shutdown::default_deadline_in_secs")]
    pub deadline_in_secs: ShutdownBudgetInSecs,

    /// How long the HTTP trackers, the REST API and the health check API wait for open connections
    /// to finish after a shutdown request, in seconds. One value serves all three.
    ///
    /// Defaults to 15, so that a full drain still leaves room for the event listeners inside the
    /// default deadline.
    #[serde(default = "Shutdown::default_http_drain_in_secs")]
    pub http_drain_in_secs: ShutdownBudgetInSecs,

    /// How long UDP trackers let already accepted requests finish after a shutdown request, in
    /// seconds. The requests still running after it are aborted.
    ///
    /// Defaults to 5.
    #[serde(default = "Shutdown::default_udp_request_drain_in_secs")]
    pub udp_request_drain_in_secs: ShutdownBudgetInSecs,

    /// How long each event listener keeps processing the events already queued for it once it is
    /// stopped, in seconds. It is an admission budget: an event already taken is not cut short.
    /// One value serves every listener.
    ///
    /// Defaults to 5.
    #[serde(default = "Shutdown::default_event_listener_drain_in_secs")]
    pub event_listener_drain_in_secs: ShutdownBudgetInSecs,
}

impl Default for Shutdown {
    fn default() -> Self {
        Self {
            deadline_in_secs: Self::default_deadline_in_secs(),
            http_drain_in_secs: Self::default_http_drain_in_secs(),
            udp_request_drain_in_secs: Self::default_udp_request_drain_in_secs(),
            event_listener_drain_in_secs: Self::default_event_listener_drain_in_secs(),
        }
    }
}

impl Shutdown {
    /// The default process shutdown deadline, in seconds.
    pub const DEFAULT_DEADLINE_IN_SECS: u64 = 25;

    /// The default HTTP, REST API and health check drain budget, in seconds.
    pub const DEFAULT_HTTP_DRAIN_IN_SECS: u64 = 15;

    /// The default UDP accepted-request drain budget, in seconds.
    pub const DEFAULT_UDP_REQUEST_DRAIN_IN_SECS: u64 = 5;

    /// The default event listener drain budget, in seconds.
    pub const DEFAULT_EVENT_LISTENER_DRAIN_IN_SECS: u64 = 5;

    fn default_deadline_in_secs() -> ShutdownBudgetInSecs {
        ShutdownBudgetInSecs::new(Self::DEFAULT_DEADLINE_IN_SECS)
            .expect("the default deadline must be within the supported range")
    }

    fn default_http_drain_in_secs() -> ShutdownBudgetInSecs {
        ShutdownBudgetInSecs::new(Self::DEFAULT_HTTP_DRAIN_IN_SECS)
            .expect("the default HTTP drain budget must be within the supported range")
    }

    fn default_udp_request_drain_in_secs() -> ShutdownBudgetInSecs {
        ShutdownBudgetInSecs::new(Self::DEFAULT_UDP_REQUEST_DRAIN_IN_SECS)
            .expect("the default UDP request drain budget must be within the supported range")
    }

    fn default_event_listener_drain_in_secs() -> ShutdownBudgetInSecs {
        ShutdownBudgetInSecs::new(Self::DEFAULT_EVENT_LISTENER_DRAIN_IN_SECS)
            .expect("the default event listener drain budget must be within the supported range")
    }

    /// The process shutdown deadline as a [`Duration`], for the component that applies it.
    #[must_use]
    pub const fn deadline(&self) -> Duration {
        self.deadline_in_secs.as_duration()
    }

    /// The HTTP, REST API and health check drain budget as a [`Duration`].
    #[must_use]
    pub const fn http_drain(&self) -> Duration {
        self.http_drain_in_secs.as_duration()
    }

    /// The UDP accepted-request drain budget as a [`Duration`].
    #[must_use]
    pub const fn udp_request_drain(&self) -> Duration {
        self.udp_request_drain_in_secs.as_duration()
    }

    /// The event listener drain budget as a [`Duration`].
    #[must_use]
    pub const fn event_listener_drain(&self) -> Duration {
        self.event_listener_drain_in_secs.as_duration()
    }
}

impl Validator for Shutdown {
    fn validate(&self) -> Result<(), SemanticValidationError> {
        let (slowest_drain_key, slowest_drain) = if self.udp_request_drain_in_secs > self.http_drain_in_secs {
            ("udp_request_drain_in_secs", self.udp_request_drain_in_secs)
        } else {
            ("http_drain_in_secs", self.http_drain_in_secs)
        };

        if slowest_drain.get() + self.event_listener_drain_in_secs.get() >= self.deadline_in_secs.get() {
            return Err(SemanticValidationError::ShutdownBudgetsExceedDeadline {
                slowest_drain_key,
                slowest_drain_in_secs: slowest_drain.get(),
                event_listener_drain_in_secs: self.event_listener_drain_in_secs.get(),
                deadline_in_secs: self.deadline_in_secs.get(),
            });
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{Shutdown, ShutdownBudgetInSecs};
    use crate::validator::{SemanticValidationError, Validator};

    fn shutdown_with(deadline: u64, http_drain: u64, udp_request_drain: u64, event_listener_drain: u64) -> Shutdown {
        let toml = [
            ("deadline_in_secs", deadline),
            ("http_drain_in_secs", http_drain),
            ("udp_request_drain_in_secs", udp_request_drain),
            ("event_listener_drain_in_secs", event_listener_drain),
        ]
        .map(|(key, value)| format!("{key} = {value}"))
        .join("\n");

        toml::from_str(&toml).expect("the budgets of the fixture should be within the supported range")
    }

    #[test]
    fn it_should_default_to_the_documented_budgets() {
        // Arrange

        // Act
        let shutdown = Shutdown::default();

        // Assert
        assert_eq!(
            (
                shutdown.deadline_in_secs.get(),
                shutdown.http_drain_in_secs.get(),
                shutdown.udp_request_drain_in_secs.get(),
                shutdown.event_listener_drain_in_secs.get()
            ),
            (25, 15, 5, 5),
            "deadline, HTTP drain, UDP request drain and event listener drain defaults, in seconds"
        );
    }

    #[test]
    fn it_should_accept_the_default_budgets_as_consistent() {
        // Arrange
        let shutdown = Shutdown::default();

        // Act
        let result = shutdown.validate();

        // Assert
        assert!(result.is_ok(), "the defaults must satisfy their own chain rule: {result:?}");
    }

    #[test]
    fn it_should_use_the_defaults_for_every_omitted_key() {
        // Arrange
        let only_the_deadline = "deadline_in_secs = 30";

        // Act
        let shutdown: Shutdown = toml::from_str(only_the_deadline).expect("a partial table should deserialize");

        // Assert
        assert_eq!(shutdown.deadline_in_secs.get(), 30, "the configured deadline");
        assert_eq!(
            (
                shutdown.http_drain_in_secs.get(),
                shutdown.udp_request_drain_in_secs.get(),
                shutdown.event_listener_drain_in_secs.get()
            ),
            (15, 5, 5),
            "the omitted keys keep their defaults"
        );
    }

    #[test]
    fn it_should_default_each_key_that_is_omitted_while_the_other_keys_are_set() {
        // Arrange
        // Each key has a default, and each configured value below differs from every default.
        let defaults = [
            ("deadline_in_secs", 25),
            ("http_drain_in_secs", 15),
            ("udp_request_drain_in_secs", 5),
            ("event_listener_drain_in_secs", 5),
        ];
        let configured = [
            ("deadline_in_secs", 100),
            ("http_drain_in_secs", 31),
            ("udp_request_drain_in_secs", 32),
            ("event_listener_drain_in_secs", 33),
        ];

        for (omitted_key, default) in defaults {
            let toml = configured
                .iter()
                .filter(|(key, _)| *key != omitted_key)
                .map(|(key, value)| format!("{key} = {value}"))
                .collect::<Vec<_>>()
                .join("\n");

            // Act
            let shutdown: Shutdown = toml::from_str(&toml).expect("a table without one key should deserialize");

            // Assert
            let serialized = toml::Table::try_from(&shutdown).expect("the budgets should serialize");
            assert_eq!(
                serialized[omitted_key].as_integer(),
                Some(default),
                "`{omitted_key}` is omitted, so it should take its default"
            );
        }
    }

    #[test]
    fn it_should_convert_a_budget_to_a_duration_of_whole_seconds() {
        // Arrange
        let budget = ShutdownBudgetInSecs::new(7).expect("7 s is within the supported range");

        // Act
        let duration = budget.as_duration();

        // Assert
        assert_eq!(duration, std::time::Duration::from_secs(7), "7 configured seconds");
    }

    #[test]
    fn it_should_expose_each_budget_as_a_duration_under_its_own_name() {
        // Arrange
        // Four different values, so an accessor that returns another budget's value is visible.
        let shutdown = shutdown_with(40, 30, 20, 10);

        // Act
        let durations = (
            shutdown.deadline(),
            shutdown.http_drain(),
            shutdown.udp_request_drain(),
            shutdown.event_listener_drain(),
        );

        // Assert
        let seconds = std::time::Duration::from_secs;
        assert_eq!(
            durations,
            (seconds(40), seconds(30), seconds(20), seconds(10)),
            "deadline, HTTP drain, UDP request drain, and event listener drain, in that order"
        );
    }

    #[test]
    fn it_should_round_trip_through_toml() {
        // Arrange
        let original = shutdown_with(40, 20, 7, 3);

        // Act
        let serialized = toml::to_string(&original).expect("the budgets should serialize");
        let deserialized: Shutdown = toml::from_str(&serialized).expect("the serialized budgets should deserialize");

        // Assert
        assert_eq!(deserialized, original);
    }

    #[test]
    fn it_should_accept_a_budget_at_each_end_of_the_supported_range() {
        // Arrange
        let (minimum, maximum) = (ShutdownBudgetInSecs::MINIMUM, ShutdownBudgetInSecs::MAXIMUM);

        // Act
        let (at_minimum, at_maximum) = (ShutdownBudgetInSecs::new(minimum), ShutdownBudgetInSecs::new(maximum));

        // Assert
        assert_eq!(
            (
                at_minimum.map(ShutdownBudgetInSecs::get),
                at_maximum.map(ShutdownBudgetInSecs::get)
            ),
            (Ok(1), Ok(3600))
        );
    }

    #[test]
    fn it_should_reject_a_zero_budget_with_the_supported_range_in_the_message() {
        // Arrange
        let zero = 0;

        // Act
        let error = ShutdownBudgetInSecs::new(zero).expect_err("zero must be rejected");

        // Assert
        assert_eq!(
            error.to_string(),
            "A shutdown budget must be between 1 and 3600 seconds, but 0 was configured. Use a whole number of seconds within that range."
        );
    }

    #[test]
    fn it_should_reject_a_budget_above_the_maximum() {
        // Arrange
        let one_above_the_maximum = ShutdownBudgetInSecs::MAXIMUM + 1;

        // Act
        let error = ShutdownBudgetInSecs::new(one_above_the_maximum).expect_err("a value above the maximum must be rejected");

        // Assert
        assert_eq!(error.value, 3601);
    }

    #[test]
    fn it_should_reject_a_zero_budget_during_deserialization() {
        // Arrange
        let zero_deadline = "deadline_in_secs = 0";

        // Act
        let error = toml::from_str::<Shutdown>(zero_deadline).expect_err("a zero deadline must be rejected");

        // Assert
        assert!(
            error.to_string().contains("between 1 and 3600 seconds"),
            "the error should state the supported range: {error}"
        );
    }

    #[test]
    fn it_should_reject_an_unknown_key() {
        // Arrange
        let misspelled_key = "drain_in_secs = 5";

        // Act
        let result = toml::from_str::<Shutdown>(misspelled_key);

        // Assert
        assert!(result.is_err(), "a key that is not a shutdown budget must be rejected");
    }

    #[test]
    fn it_should_accept_a_chain_that_is_one_second_below_the_deadline() {
        // Arrange
        let shutdown = shutdown_with(25, 19, 5, 5);

        // Act
        let result = shutdown.validate();

        // Assert
        assert!(result.is_ok(), "19 + 5 = 24 is below 25: {result:?}");
    }

    #[test]
    fn it_should_reject_a_chain_that_equals_the_deadline_naming_the_http_drain() {
        // Arrange
        let shutdown = shutdown_with(25, 20, 5, 5);

        // Act
        let error = shutdown.validate().expect_err("20 + 5 is not below 25");

        // Assert
        assert!(
            matches!(
                error,
                SemanticValidationError::ShutdownBudgetsExceedDeadline {
                    slowest_drain_key: "http_drain_in_secs",
                    slowest_drain_in_secs: 20,
                    event_listener_drain_in_secs: 5,
                    deadline_in_secs: 25,
                }
            ),
            "unexpected error: {error:?}"
        );
    }

    #[test]
    fn it_should_name_the_udp_request_drain_when_it_is_the_longer_server_drain() {
        // Arrange
        let shutdown = shutdown_with(25, 15, 21, 5);

        // Act
        let error = shutdown.validate().expect_err("max(15, 21) + 5 = 26 is not below 25");

        // Assert
        assert!(
            matches!(
                error,
                SemanticValidationError::ShutdownBudgetsExceedDeadline {
                    slowest_drain_key: "udp_request_drain_in_secs",
                    slowest_drain_in_secs: 21,
                    ..
                }
            ),
            "unexpected error: {error:?}"
        );
    }

    #[test]
    fn it_should_name_the_http_drain_when_both_server_drains_are_equal() {
        // Arrange
        let shutdown = shutdown_with(25, 20, 20, 5);

        // Act
        let error = shutdown.validate().expect_err("20 + 5 is not below 25");

        // Assert
        assert!(
            matches!(
                error,
                SemanticValidationError::ShutdownBudgetsExceedDeadline {
                    slowest_drain_key: "http_drain_in_secs",
                    ..
                }
            ),
            "unexpected error: {error:?}"
        );
    }

    #[test]
    fn it_should_tell_the_operator_which_keys_and_values_to_change() {
        // Arrange
        let shutdown = shutdown_with(25, 20, 5, 5);

        // Act
        let message = shutdown.validate().expect_err("20 + 5 is not below 25").to_string();

        // Assert
        for expected in [
            "application.shutdown.http_drain_in_secs (20 s)",
            "application.shutdown.event_listener_drain_in_secs (5 s)",
            "application.shutdown.deadline_in_secs (25 s)",
            "Lower the drain budgets or raise the deadline",
        ] {
            assert!(
                message.contains(expected),
                "the message should contain `{expected}`: {message}"
            );
        }
    }
}
