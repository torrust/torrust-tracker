//! Tracker API verification steps for E2E scenarios.
//!
//! Each file contains one explicit step so available actions are discoverable in the IDE tree.

mod verify_tracker_swarm;

pub(in super::super) use verify_tracker_swarm::verify_tracker_swarm;
