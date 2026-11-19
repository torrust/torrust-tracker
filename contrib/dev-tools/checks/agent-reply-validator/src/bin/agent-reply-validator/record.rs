//! The NDJSON records written to stderr, one JSON object per line.

use std::io::{self, Write};

use agent_reply_validator::file::Problem;
use serde::Serialize;

pub const EXIT_SUCCESS: u8 = 0;
pub const EXIT_FAILURE: u8 = 1;
pub const EXIT_USAGE: u8 = 2;

/// One line on stderr. Field order and nullability are the command's output contract.
#[derive(Debug, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum Record {
    /// One reason a reply file is invalid.
    Problem {
        path: String,
        check: String,
        field_path: Option<String>,
        message: String,
    },
    /// The totals of a run that checked files.
    Summary {
        files: usize,
        problems: usize,
    },
    UsageError {
        message: String,
        exit_code: u8,
    },
    RuntimeError {
        path: Option<String>,
        message: String,
        exit_code: u8,
    },
    Help {
        message: String,
    },
}

impl Record {
    pub fn problem(path: &str, problem: Problem) -> Self {
        Self::Problem {
            path: path.to_owned(),
            check: problem.check.to_string(),
            field_path: problem.field_path,
            message: problem.message,
        }
    }

    pub fn usage_error(message: impl Into<String>) -> Self {
        Self::UsageError {
            message: message.into(),
            exit_code: EXIT_USAGE,
        }
    }

    pub fn runtime_error(path: Option<String>, message: impl Into<String>) -> Self {
        Self::RuntimeError {
            path,
            message: message.into(),
            exit_code: EXIT_FAILURE,
        }
    }
}

pub fn emit(records: &[Record], stderr: &mut impl Write) -> io::Result<()> {
    for record in records {
        serde_json::to_writer(&mut *stderr, record)?;
        writeln!(stderr)?;
    }
    stderr.flush()
}
