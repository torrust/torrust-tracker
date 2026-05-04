//! This module defines the `Unit` enum, which represents various units of
//! measurement.
//!
//! The `Unit` enum is used to specify the unit of measurement for metrics.
//!
//! They were copied from the `metrics` crate, to allow future compatibility.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Unit {
    Count,
    Percent,
    Seconds,
    Milliseconds,
    Microseconds,
    Nanoseconds,
    Tebibytes,
    Gibibytes,
    Mebibytes,
    Kibibytes,
    Bytes,
    TerabitsPerSecond,
    GigabitsPerSecond,
    MegabitsPerSecond,
    KilobitsPerSecond,
    BitsPerSecond,
    CountPerSecond,
}

#[cfg(test)]
mod tests {
    use super::Unit;

    #[test]
    fn it_should_serialize_count_to_snake_case() {
        let json = serde_json::to_string(&Unit::Count).unwrap();
        assert_eq!(json, r#""count""#);
    }

    #[test]
    fn it_should_deserialize_count_from_snake_case() {
        let unit: Unit = serde_json::from_str(r#""count""#).unwrap();
        assert_eq!(unit, Unit::Count);
    }

    #[test]
    fn it_should_round_trip_all_variants() {
        let variants = [
            Unit::Count,
            Unit::Percent,
            Unit::Seconds,
            Unit::Milliseconds,
            Unit::Microseconds,
            Unit::Nanoseconds,
            Unit::Tebibytes,
            Unit::Gibibytes,
            Unit::Mebibytes,
            Unit::Kibibytes,
            Unit::Bytes,
            Unit::TerabitsPerSecond,
            Unit::GigabitsPerSecond,
            Unit::MegabitsPerSecond,
            Unit::KilobitsPerSecond,
            Unit::BitsPerSecond,
            Unit::CountPerSecond,
        ];

        for variant in variants {
            let json = serde_json::to_string(&variant).unwrap();
            let deserialized: Unit = serde_json::from_str(&json).unwrap();
            assert_eq!(deserialized, variant);
        }
    }

    #[test]
    fn it_should_implement_clone_copy_eq_hash_debug() {
        let u = Unit::Count;
        let c = u;
        assert_eq!(u, c);
        let s = format!("{u:?}");
        assert!(!s.is_empty());
        let mut set = std::collections::HashSet::new();
        set.insert(u);
        assert!(set.contains(&Unit::Count));
    }
}
