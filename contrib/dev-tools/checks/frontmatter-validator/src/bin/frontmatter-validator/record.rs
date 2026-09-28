//! The D9 NDJSON record catalog written to stderr.

use std::io::{self, Write};

use frontmatter_validator::{Diagnostic, DiagnosticCategory, Severity};
use serde::Serialize;

pub const EXIT_SUCCESS: u8 = 0;
pub const EXIT_FAILURE: u8 = 1;
pub const EXIT_USAGE: u8 = 2;

/// One NDJSON line on stderr. Field order and nullability are the command's output contract.
#[derive(Debug, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum Record {
    Diagnostic {
        path: String,
        severity: Severity,
        category: DiagnosticCategory,
        field_path: Option<String>,
        message: String,
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
    pub fn diagnostic(path: impl Into<String>, diagnostic: Diagnostic) -> Self {
        Self::Diagnostic {
            path: path.into(),
            severity: diagnostic.severity,
            category: diagnostic.category,
            field_path: diagnostic.field_path,
            message: diagnostic.message,
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

    pub const fn is_error(&self) -> bool {
        matches!(
            self,
            Self::Diagnostic {
                severity: Severity::Error,
                ..
            }
        )
    }
}

pub fn emit(records: &[Record], stderr: &mut impl Write) -> io::Result<()> {
    for record in records {
        serde_json::to_writer(&mut *stderr, record)?;
        writeln!(stderr)?;
    }
    stderr.flush()
}
