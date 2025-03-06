use std::net::SocketAddr;

use serde::{Deserialize, Serialize};

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
