//! Banning service for UDP tracker.
//!
//! It bans clients that send invalid connection id's.
//! It uses an exact `HashMap` to track connection-ID errors by source IP,
//! avoiding collision-driven bans. See ADR
//! `../../docs/adrs/20260829204258_use_exact_ip_counters_for_udp_banning.md`.
use std::collections::HashMap;
use std::net::IpAddr;

use tokio::time::Instant;

use crate::UDP_TRACKER_LOG_TARGET;

/// Trait exposing only the banning statistics that external consumers need.
pub trait BanningStats: Send + Sync {
    /// Returns the total number of banned IPs.
    fn get_banned_ips_total(&self) -> usize;
}

pub struct BanService {
    max_connection_id_errors_per_ip: u32,
    accurate_error_counter: HashMap<IpAddr, u32>,
    last_connection_id_errors_reset: Instant,
}

impl BanService {
    #[must_use]
    pub fn new(max_connection_id_errors_per_ip: u32) -> Self {
        Self {
            max_connection_id_errors_per_ip,
            accurate_error_counter: HashMap::new(),
            last_connection_id_errors_reset: tokio::time::Instant::now(),
        }
    }

    pub fn increase_counter(&mut self, ip: &IpAddr) {
        *self.accurate_error_counter.entry(*ip).or_insert(0) += 1;
    }

    #[must_use]
    pub fn get_count(&self, ip: &IpAddr) -> Option<u32> {
        self.accurate_error_counter.get(ip).copied()
    }

    #[must_use]
    pub fn get_banned_ips_total(&self) -> usize {
        self.accurate_error_counter.len()
    }

    /// Returns true if the given ip address is banned.
    #[must_use]
    pub fn is_banned(&self, ip: &IpAddr) -> bool {
        self.get_count(ip)
            .is_some_and(|count| count > self.max_connection_id_errors_per_ip)
    }

    /// Resets the counters and updates the reset timestamp.
    pub fn reset_bans(&mut self) {
        self.accurate_error_counter.clear();

        self.last_connection_id_errors_reset = Instant::now();

        tracing::info!(target: UDP_TRACKER_LOG_TARGET, "Udp::run_udp_server::loop (connection id errors filter cleared)");
    }
}

impl BanningStats for BanService {
    fn get_banned_ips_total(&self) -> usize {
        self.accurate_error_counter.len()
    }
}

#[cfg(test)]
mod tests {
    use std::net::IpAddr;

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
}
