use super::responses;
use crate::tracker::error::Error;

pub mod announce;
pub mod common;
pub mod scrape;

impl From<Error> for responses::error::Error {
    fn from(err: Error) -> Self {
        responses::error::Error {
            failure_reason: format!("Tracker error: {err}"),
        }
    }
}