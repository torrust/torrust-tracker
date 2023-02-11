use std::panic::Location;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("failed to parse settings {message}, {location}")]
    ParseConfig {
        location: &'static Location<'static>,
        message: String,
    },
}
