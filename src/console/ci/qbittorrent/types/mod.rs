//! Small domain types shared across the `qBittorrent` E2E module.
//!
//! Most types here follow the newtype pattern: a thin wrapper around a primitive
//! that gives the value a precise, self-documenting type at every call site.

mod compose_project_name;
mod container_path;
mod deadline;
mod file_name;
mod payload_size;
mod piece_length;
mod poll_interval;
mod qbittorrent_image;
mod tracker_image;

pub(crate) use compose_project_name::ComposeProjectName;
pub(crate) use container_path::ContainerPath;
pub(crate) use deadline::Deadline;
pub(crate) use file_name::FileName;
pub(crate) use payload_size::PayloadSize;
pub(crate) use piece_length::PieceLength;
pub(crate) use poll_interval::PollInterval;
pub(crate) use qbittorrent_image::QbittorrentImage;
pub(crate) use tracker_image::TrackerImage;
