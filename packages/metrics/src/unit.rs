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
