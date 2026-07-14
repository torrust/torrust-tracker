//! Scrape response types for the HTTP tracker.
pub mod data;
pub mod deserialization;
pub mod encoding;

pub use data::{ScrapeData, SwarmMetadata};
pub use deserialization::{BencodeParseError, File, Response, ResponseBuilder};
pub use encoding::Bencoded;
