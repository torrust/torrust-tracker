use std::net::SocketAddr;

use serde::{Deserialize, Serialize};

use crate::servers::registar::{Error, HeathCheckResult, Success};

#[derive(Copy, Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum Status {
    Ok,
    Error,
    None,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct CheckReport {
    pub binding: SocketAddr,
    pub info: String,
    pub result: Result<String, String>,
}

impl CheckReport {
    #[must_use]
    pub fn pass(&self) -> bool {
        self.result.is_ok()
    }
    #[must_use]
    pub fn fail(&self) -> bool {
        self.result.is_err()
    }
}

impl From<HeathCheckResult> for CheckReport {
    fn from(result: HeathCheckResult) -> Self {
        let (addr, msg, result) = match result.as_ref() {
            Ok(success) => {
                let (addr, msg) = match success {
                    Success::AllGood { addr, msg } => (addr, msg),
                };

                (addr, msg, Ok(success.to_string()))
            }
            Err(error) => {
                let (addr, msg) = match error {
                    Error::UnableToPreformSuccessfulHealthCheck { addr, msg }
                    | Error::UnableToConnectToRemote { addr, msg, .. }
                    | Error::UnableToPreformCheck { addr, msg, .. }
                    | Error::UnableToObtainGoodResponse { addr, msg, .. }
                    | Error::UnableToGetAnyResponse { addr, msg, .. } => (addr, msg),
                };

                (addr, msg, Err(error.to_string()))
            }
        };

        Self {
            binding: *addr,
            info: msg.to_string(),
            result,
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Report {
    pub status: Status,
    pub message: String,
    pub details: Vec<CheckReport>,
}

impl Report {
    #[must_use]
    pub fn none() -> Report {
        Self {
            status: Status::None,
            message: String::new(),
            details: Vec::default(),
        }
    }

    #[must_use]
    pub fn ok(details: Vec<CheckReport>) -> Report {
        Self {
            status: Status::Ok,
            message: String::new(),
            details,
        }
    }

    #[must_use]
    pub fn error(message: String, details: Vec<CheckReport>) -> Report {
        Self {
            status: Status::Error,
            message,
            details,
        }
    }
}
