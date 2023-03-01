use super::responses;
use crate::tracker::error::Error;

pub mod announce;
pub mod auth;
pub mod scrape;
pub mod status;

impl From<Error> for responses::error::Error {
    fn from(err: Error) -> Self {
        responses::error::Error {
            failure_reason: format!("Tracker error: {err}"),
        }
    }
}
