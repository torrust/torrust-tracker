//! Provider-neutral CI decisions for pull requests.
//!
//! Part of issue #2525. The decisions that CI makes about a pull request live in Rust tools with
//! provider-neutral inputs and outputs, and the workflows stay thin, as recorded in the ADR
//! `20261016154414_keep_ci_decision_logic_in_rust_behind_thin_provider_agnostic_workflows.md`.
//!
//! This crate currently decides which job groups a change needs ([`change_detection`]). The merge
//! verdict is added by a later task of the issue.

pub mod change_detection;
pub mod cli;
pub mod job_group;
