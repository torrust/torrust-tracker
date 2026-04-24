//! qBittorrent end-to-end test module.
//!
//! This module drives E2E smoke tests for the Torrust tracker by orchestrating real
//! qBittorrent clients against a live tracker instance, all running inside Docker
//! Compose containers.
//!
//! # Architecture
//!
//! The entry point is the `qbittorrent_e2e_runner` binary
//! (`src/bin/qbittorrent_e2e_runner.rs`), which is a thin wrapper that delegates
//! everything to [`runner`]. All domain logic lives in this module tree.
//!
//! ## BDD-style scenarios and steps
//!
//! Tests are structured around *scenarios* — each scenario describes a complete
//! user story from the `BitTorrent` perspective. Scenarios are composed of reusable
//! *steps* (see [`scenario_steps`]) that can be shared across scenarios.
//!
//! Currently one scenario is implemented, covering the most common tracker usage:
//!
//! 1. A **seeder** qBittorrent client creates a torrent from a known payload file
//!    and starts seeding it through the tracker.
//! 2. A **leecher** qBittorrent client discovers the torrent via the tracker and
//!    downloads it from the seeder.
//! 3. After the download completes, the downloaded file is compared byte-for-byte
//!    against the original payload to assert data integrity.
//!
//! ## Infrastructure vs. scenario
//!
//! A deliberate design decision separates *infrastructure setup* from *scenario
//! execution*:
//!
//! **Infrastructure setup** (done once before any scenario runs):
//! - Prepare the tracker workspace (config file, storage directory) and start the
//!   tracker container.
//! - Prepare each qBittorrent client workspace (per-client config, downloads
//!   directory) and start the client containers.
//! - Wait until all services are reachable.
//!
//! **Scenario execution** (runs against the already-running infrastructure):
//! - Perform the actual `BitTorrent` workflow steps.
//! - Assert the expected outcome.
//!
//! The reason for this split is cost: starting containers is slow. By keeping the
//! infrastructure alive across scenarios, multiple scenarios can run against the
//! same stack without paying the startup penalty each time.
//!
//! This also opens a clear extension path: in the future we could have multiple
//! infrastructure configurations (e.g. public vs. private tracker, `SQLite` vs.
//! `MySQL`, different numbers of peers) each hosting their own suite of scenarios,
//! without changing the scenario or step code.

pub mod bencode;
pub mod client_role;
pub mod compose_stack;
pub mod poller;
pub mod qbittorrent_client;
pub mod qbittorrent_config;
pub mod runner;
pub mod scenario_steps;
pub mod scenarios;
pub mod torrent_artifacts;
pub mod workspace;
pub mod workspace_setup;
