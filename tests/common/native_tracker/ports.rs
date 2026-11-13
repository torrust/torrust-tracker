//! Re-exports the loopback port reservation that configuration scenarios use before the tracker
//! starts. The implementation lives in `torrust-tracker-test-helpers` so package tests share it.

#[allow(unused_imports)]
pub use torrust_tracker_test_helpers::ports::reserve_free_ports;
