//! Validation of the JSON reply files that AI agents write.
//!
//! An agent started by the orchestrator records each reply as one JSON file in the
//! `agent-replies/` folder of the issue specification it works on. The file name says which
//! agent replied, about which subject, and in which round; the file content follows the
//! `agent-reply-v1` schema. See decision D18 of issue #2904.

pub mod file;
pub mod name;
pub mod reply;
