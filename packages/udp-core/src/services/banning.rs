//! Banning service for UDP tracker.
//!
//! It bans clients that send invalid connection id's.
//! It uses an exact `HashMap` to track connection-ID errors by source IP,
//! avoiding collision-driven bans. See ADR
//! `../../docs/adrs/20260829204258_use_exact_ip_counters_for_udp_banning.md`.
use std::collections::HashMap;
use std::net::IpAddr;

use crate::UDP_TRACKER_LOG_TARGET;

pub struct BanService {
    max_connection_id_errors_per_ip: u32,
    accurate_error_counter: HashMap<IpAddr, u32>,
    /// Number of IPs in `accurate_error_counter` whose count exceeds
    /// `max_connection_id_errors_per_ip`. Maintained on every change to the
    /// map so reading it never scans the map, whose size an attacker can
    /// grow by spoofing source addresses.
    banned_ips_total: usize,
}

impl BanService {
    #[must_use]
    pub fn new(max_connection_id_errors_per_ip: u32) -> Self {
        Self {
            max_connection_id_errors_per_ip,
            accurate_error_counter: HashMap::new(),
            banned_ips_total: 0,
        }
    }

    pub fn increase_counter(&mut self, ip: &IpAddr) {
        let count = self.accurate_error_counter.entry(*ip).or_insert(0);

        // Stop at `u32::MAX` instead of wrapping: a wrapped count would cross
        // the limit again and count an already banned IP twice.
        let Some(next_count) = count.checked_add(1) else {
            return;
        };
        *count = next_count;

        // An IP becomes banned exactly when its count reaches `limit + 1`.
        if Some(next_count) == self.max_connection_id_errors_per_ip.checked_add(1) {
            self.banned_ips_total += 1;
        }
    }

    #[must_use]
    pub fn get_count(&self, ip: &IpAddr) -> Option<u32> {
        self.accurate_error_counter.get(ip).copied()
    }

    /// Returns the number of IPs whose connection ID error count exceeds the
    /// limit, that is, the IPs for which [`is_banned`](Self::is_banned) is
    /// true.
    ///
    /// The service does not know the connection ID validation policy. When
    /// validation is `disabled`, the server does not enforce bans, so these
    /// IPs are "would-be banned" rather than blocked.
    #[must_use]
    pub const fn get_banned_ips_total(&self) -> usize {
        self.banned_ips_total
    }

    /// Returns true if the given ip address is banned.
    #[must_use]
    pub fn is_banned(&self, ip: &IpAddr) -> bool {
        self.get_count(ip)
            .is_some_and(|count| count > self.max_connection_id_errors_per_ip)
    }

    /// Resets the counters, which also lifts every ban.
    pub fn reset_bans(&mut self) {
        self.accurate_error_counter.clear();
        self.banned_ips_total = 0;

        tracing::info!(target: UDP_TRACKER_LOG_TARGET, "Udp::run_udp_server::loop (connection id errors filter cleared)");
    }
}

#[cfg(test)]
mod tests {
    use std::net::IpAddr;

    use quickcheck::{Arbitrary, Gen, QuickCheck, TestResult};

    use super::BanService;

    /// Sample service with one day ban duration.
    fn ban_service(counter_limit: u32) -> BanService {
        BanService::new(counter_limit)
    }

    #[test]
    fn it_should_increase_the_errors_counter_for_a_given_ip() {
        let mut ban_service = ban_service(1);

        let ip: IpAddr = "127.0.0.2".parse().unwrap();

        ban_service.increase_counter(&ip);

        assert_eq!(ban_service.get_count(&ip), Some(1));
    }

    #[test]
    fn it_should_ban_ips_with_counters_exceeding_a_predefined_limit() {
        let mut ban_service = ban_service(1);

        let ip: IpAddr = "127.0.0.2".parse().unwrap();

        ban_service.increase_counter(&ip); // Counter = 1
        ban_service.increase_counter(&ip); // Counter = 2

        assert!(ban_service.is_banned(&ip));
    }

    #[test]
    fn it_should_not_ban_ips_whose_counters_do_not_exceed_the_predefined_limit() {
        let mut ban_service = ban_service(1);

        let ip: IpAddr = "127.0.0.2".parse().unwrap();

        ban_service.increase_counter(&ip);

        assert!(!ban_service.is_banned(&ip));
    }

    #[test]
    fn it_should_not_ban_ips_without_connection_id_errors() {
        // Arrange
        let ban_service = ban_service(1);
        let ip: IpAddr = "127.0.0.2".parse().unwrap();

        // Act
        let is_banned = ban_service.is_banned(&ip);

        // Assert
        assert!(!is_banned);
    }

    #[test]
    fn it_should_allow_resetting_all_the_counters() {
        // Arrange
        let mut ban_service = ban_service(1);
        let ip: IpAddr = "127.0.0.2".parse().unwrap();

        ban_service.increase_counter(&ip);
        ban_service.increase_counter(&ip);

        // Act
        ban_service.reset_bans();

        // Assert
        assert_eq!(ban_service.get_count(&ip), None);
        assert!(!ban_service.is_banned(&ip));
    }

    fn record_connection_id_errors(ban_service: &mut BanService, ip: &IpAddr, errors: u32) {
        for _ in 0..errors {
            ban_service.increase_counter(ip);
        }
    }

    #[test]
    fn it_should_not_count_an_ip_as_banned_while_its_errors_do_not_exceed_the_limit() {
        // Arrange
        let limit = 2;
        let mut ban_service = ban_service(limit);
        let ip: IpAddr = "127.0.0.2".parse().unwrap();
        record_connection_id_errors(&mut ban_service, &ip, limit);

        // Act
        let banned_ips_total = ban_service.get_banned_ips_total();

        // Assert
        assert_eq!(
            banned_ips_total, 0,
            "banned IPs total for one IP with {limit} errors and a limit of {limit}"
        );
    }

    #[test]
    fn it_should_count_an_ip_as_banned_once_its_errors_exceed_the_limit() {
        // Arrange
        let limit = 2;
        let mut ban_service = ban_service(limit);
        let ip: IpAddr = "127.0.0.2".parse().unwrap();
        record_connection_id_errors(&mut ban_service, &ip, limit + 1);

        // Act
        let banned_ips_total = ban_service.get_banned_ips_total();

        // Assert
        assert_eq!(
            banned_ips_total,
            1,
            "banned IPs total for one IP with {} errors and a limit of {limit}",
            limit + 1
        );
    }

    #[test]
    fn it_should_count_a_banned_ip_only_once_when_it_keeps_sending_invalid_connection_ids() {
        // Arrange
        let limit = 2;
        let mut ban_service = ban_service(limit);
        let ip: IpAddr = "127.0.0.2".parse().unwrap();
        record_connection_id_errors(&mut ban_service, &ip, limit + 2);

        // Act
        let banned_ips_total = ban_service.get_banned_ips_total();

        // Assert
        assert_eq!(
            banned_ips_total,
            1,
            "banned IPs total for one IP with {} errors and a limit of {limit}",
            limit + 2
        );
    }

    #[test]
    fn it_should_not_count_any_ip_as_banned_after_the_bans_are_reset() {
        // Arrange
        let limit = 1;
        let mut ban_service = ban_service(limit);
        let ip: IpAddr = "127.0.0.2".parse().unwrap();
        record_connection_id_errors(&mut ban_service, &ip, limit + 1);
        ban_service.reset_bans();

        // Act
        let banned_ips_total = ban_service.get_banned_ips_total();

        // Assert
        assert_eq!(
            banned_ips_total,
            0,
            "banned IPs total after a reset that followed one IP with {} errors and a limit of {limit}",
            limit + 1
        );
    }

    #[test]
    fn it_should_not_count_an_ip_whose_errors_after_a_reset_do_not_exceed_the_limit() {
        // Arrange
        let limit = 1;
        let mut ban_service = ban_service(limit);
        let ip: IpAddr = "127.0.0.2".parse().unwrap();
        record_connection_id_errors(&mut ban_service, &ip, limit + 1);
        ban_service.reset_bans();
        record_connection_id_errors(&mut ban_service, &ip, limit);

        // Act
        let banned_ips_total = ban_service.get_banned_ips_total();

        // Assert
        assert_eq!(
            banned_ips_total, 0,
            "banned IPs total for an IP that was banned, reset, and then sent {limit} error with a limit of {limit}"
        );
    }

    #[test]
    fn it_should_count_an_ip_banned_again_after_a_reset_only_once() {
        // Arrange
        let limit = 1;
        let mut ban_service = ban_service(limit);
        let ip: IpAddr = "127.0.0.2".parse().unwrap();
        record_connection_id_errors(&mut ban_service, &ip, limit + 1);
        ban_service.reset_bans();
        record_connection_id_errors(&mut ban_service, &ip, limit + 1);

        // Act
        let banned_ips_total = ban_service.get_banned_ips_total();

        // Assert
        assert_eq!(
            banned_ips_total, 1,
            "banned IPs total for one IP that exceeded the limit of {limit} again after a reset"
        );
    }

    #[test]
    fn it_should_not_count_any_ip_as_banned_when_several_ips_stay_within_the_limit() {
        // Arrange
        let limit = 1;
        let mut ban_service = ban_service(limit);
        let ips: [IpAddr; 3] = [
            "127.0.0.2".parse().unwrap(),
            "127.0.0.3".parse().unwrap(),
            "127.0.0.4".parse().unwrap(),
        ];
        for ip in &ips {
            record_connection_id_errors(&mut ban_service, ip, limit);
        }

        // Act
        let banned_ips_total = ban_service.get_banned_ips_total();

        // Assert
        assert_eq!(
            banned_ips_total,
            0,
            "banned IPs total for {} IPs with {limit} error each and a limit of {limit}",
            ips.len()
        );
    }

    /// Puts an IP's error count near `u32::MAX` directly: reaching it through
    /// `increase_counter` would take about four billion calls. The test module
    /// can set the private map because it is a child of this module.
    fn with_error_count(ban_service: &mut BanService, ip: IpAddr, count: u32) {
        ban_service.accurate_error_counter.insert(ip, count);
    }

    #[test]
    fn it_should_not_count_a_banned_ip_again_after_its_error_count_reaches_the_maximum() {
        // Arrange
        let limit = 1;
        let mut ban_service = ban_service(limit);
        let ip: IpAddr = "127.0.0.2".parse().unwrap();
        record_connection_id_errors(&mut ban_service, &ip, limit + 1);
        with_error_count(&mut ban_service, ip, u32::MAX);

        // Act
        record_connection_id_errors(&mut ban_service, &ip, limit + 2);

        // Assert
        assert_eq!(
            ban_service.get_banned_ips_total(),
            1,
            "banned IPs total after an IP banned with a limit of {limit} sends {} more errors at a count of u32::MAX",
            limit + 2
        );
    }

    #[test]
    fn it_should_count_an_ip_once_when_the_limit_is_one_below_the_maximum_count() {
        // Arrange
        let limit = u32::MAX - 1;
        let mut ban_service = ban_service(limit);
        let ip: IpAddr = "127.0.0.2".parse().unwrap();
        with_error_count(&mut ban_service, ip, limit);

        // Act
        record_connection_id_errors(&mut ban_service, &ip, 3);

        // Assert
        assert_eq!(
            ban_service.get_banned_ips_total(),
            1,
            "banned IPs total after an IP at the limit of u32::MAX - 1 sends 3 more errors"
        );
    }

    /// One step of a property-test scenario: an invalid connection ID from one
    /// of a few IPs, or a periodic reset.
    #[derive(Clone, Debug)]
    enum Operation {
        ConnectionIdError { ip_index: usize },
        ResetBans,
    }

    const PROPERTY_TEST_IPS: [&str; 4] = ["127.0.0.2", "127.0.0.3", "127.0.0.4", "127.0.0.5"];

    impl Arbitrary for Operation {
        fn arbitrary(generator: &mut Gen) -> Self {
            // About one operation in eight is a reset, so most scenarios cross
            // the limit before the counters are cleared.
            if u8::arbitrary(generator) % 8 == 0 {
                Self::ResetBans
            } else {
                Self::ConnectionIdError {
                    ip_index: usize::arbitrary(generator) % PROPERTY_TEST_IPS.len(),
                }
            }
        }
    }

    /// Applies every operation to the service and to an independent model of
    /// the per-IP error counts, and checks after each step that the banned
    /// total equals the number of model counts above the limit.
    fn banned_ips_total_matches_the_model(limit: u8, operations: Vec<Operation>) -> TestResult {
        let limit = u32::from(limit % 4);
        let ips: Vec<IpAddr> = PROPERTY_TEST_IPS.iter().map(|ip| ip.parse().unwrap()).collect();
        let mut ban_service = ban_service(limit);
        let mut model_error_counts = [0_u32; PROPERTY_TEST_IPS.len()];

        for (step, operation) in operations.into_iter().enumerate() {
            match operation {
                Operation::ConnectionIdError { ip_index } => {
                    ban_service.increase_counter(&ips[ip_index]);
                    model_error_counts[ip_index] += 1;
                }
                Operation::ResetBans => {
                    ban_service.reset_bans();
                    model_error_counts = [0; PROPERTY_TEST_IPS.len()];
                }
            }

            let expected = model_error_counts.iter().filter(|count| **count > limit).count();
            let actual = ban_service.get_banned_ips_total();

            if actual != expected {
                return TestResult::error(format!(
                    "after step {step} ({operation:?}) with a limit of {limit} and model error counts \
                     {model_error_counts:?}: banned IPs total {actual}, expected {expected}"
                ));
            }
        }

        TestResult::passed()
    }

    #[test]
    fn it_should_keep_the_banned_ips_total_equal_to_the_ips_above_the_limit_for_any_errors_and_resets() {
        QuickCheck::new()
            .tests(1_000)
            .quickcheck(banned_ips_total_matches_the_model as fn(u8, Vec<Operation>) -> TestResult);
    }
}
