//! Helpers shared by the command-boundary tests.
//!
//! Every git invocation, including the binaries', clears inherited `GIT_*` variables and sets a
//! ceiling directory, so a test run from a git hook can never read or change the real repository.

use std::path::Path;
use std::process::Command;

/// Inherited variables that would redirect git away from the disposable repository.
const GIT_ENVIRONMENT: &[&str] = &[
    "GIT_DIR",
    "GIT_WORK_TREE",
    "GIT_INDEX_FILE",
    "GIT_OBJECT_DIRECTORY",
    "GIT_ALTERNATE_OBJECT_DIRECTORIES",
    "GIT_COMMON_DIR",
    "GIT_PREFIX",
    "GIT_CONFIG_PARAMETERS",
    "GIT_CONFIG_COUNT",
    "GIT_LITERAL_PATHSPECS",
    "GIT_GLOB_PATHSPECS",
    "GIT_NOGLOB_PATHSPECS",
    "GIT_ICASE_PATHSPECS",
];

/// Every disposable repository lives below the temp directory, so git never searches above it.
pub fn isolated(mut command: Command, directory: &Path) -> Command {
    for variable in GIT_ENVIRONMENT {
        command.env_remove(variable);
    }
    command
        .current_dir(directory)
        .env("GIT_CEILING_DIRECTORIES", std::env::temp_dir());
    command
}
