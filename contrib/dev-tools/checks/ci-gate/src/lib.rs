//! Provider-neutral CI decisions for pull requests.
//!
//! Part of issue #2525. The decisions that CI makes about a pull request live in Rust tools with
//! provider-neutral inputs and outputs, and the workflows stay thin, as recorded in the ADR
//! `20261016154414_keep_ci_decision_logic_in_rust_behind_thin_provider_agnostic_workflows.md`.
//!
//! This crate decides which job groups a change needs ([`change_detection`]) and whether the results
//! of those groups let the pull request merge ([`verdict`]).

pub mod change_detection;
pub mod cli;
pub mod job_group;
pub mod verdict;
