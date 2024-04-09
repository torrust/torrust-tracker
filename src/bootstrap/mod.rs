//! Tracker application bootstrapping.
//!
//! This module includes all the functions to build the application, its dependencies, and run the jobs.
//!
//! Jobs are tasks executed concurrently. Some of them are concurrent because of the asynchronous nature of the task,
//! like cleaning torrents, and other jobs because they can be enabled/disabled depending on the configuration.
//! For example, you can have more than one UDP and HTTP tracker, each server is executed like a independent job.
pub mod app;
pub mod config;
pub mod jobs;
pub mod tracing;
