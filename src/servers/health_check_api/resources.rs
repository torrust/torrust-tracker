use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum Status {
    Ok,
    Error,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Report {
    pub status: Status,
    pub message: String,
}

impl Report {
    #[must_use]
    pub fn ok() -> Report {
        Self {
            status: Status::Ok,
            message: String::new(),
        }
    }

    #[must_use]
    pub fn error(message: String) -> Report {
        Self {
            status: Status::Error,
            message,
        }
    }
}
