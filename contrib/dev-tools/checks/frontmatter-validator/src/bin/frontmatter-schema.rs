//! Generates and verifies the tracked frontmatter v1 JSON Schema artifact.

use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::{env, fs};

use frontmatter_validator::v1_schema;

const ARTIFACT_PATH: &str = "docs/schemas/frontmatter-v1.schema.json";

/// Failures reported by the command, classified by the exit code they map to.
#[derive(Debug, Eq, PartialEq, thiserror::Error)]
enum Error {
    #[error("usage: frontmatter-schema <generate|check> [--artifact <path>]")]
    Usage,
    #[error("{0}")]
    Runtime(String),
}

impl Error {
    /// Exit codes follow the repository CLI output contract: `2` for usage errors, `1` otherwise.
    fn exit_code(&self) -> ExitCode {
        match self {
            Self::Usage => ExitCode::from(2),
            Self::Runtime(_) => ExitCode::FAILURE,
        }
    }
}

fn main() -> ExitCode {
    match Command::parse(env::args().skip(1)).and_then(|command| command.execute()) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            let mut stderr = io::stderr().lock();
            drop(writeln!(stderr, "frontmatter-schema: {error}"));
            error.exit_code()
        }
    }
}

/// A parsed invocation: the action to perform and the artifact it applies to.
#[derive(Debug, Eq, PartialEq)]
enum Command {
    Generate { artifact: PathBuf },
    Check { artifact: PathBuf },
}

impl Command {
    /// Parses the process arguments after the program name. Performs no I/O.
    fn parse(mut arguments: impl Iterator<Item = String>) -> Result<Self, Error> {
        let action = arguments.next().ok_or(Error::Usage)?;
        let artifact = artifact_path(&arguments.collect::<Vec<_>>())?;

        match action.as_str() {
            "generate" => Ok(Self::Generate { artifact }),
            "check" => Ok(Self::Check { artifact }),
            _ => Err(Error::Usage),
        }
    }

    fn execute(&self) -> Result<(), Error> {
        match self {
            Self::Generate { artifact } => write_schema(artifact),
            Self::Check { artifact } => check_schema(artifact),
        }
    }
}

fn artifact_path(arguments: &[String]) -> Result<PathBuf, Error> {
    match arguments {
        [] => Ok(repository_root()?.join(ARTIFACT_PATH)),
        [flag, path] if flag == "--artifact" => Ok(PathBuf::from(path)),
        _ => Err(Error::Usage),
    }
}

fn repository_root() -> Result<PathBuf, Error> {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(4)
        .map(Path::to_path_buf)
        .ok_or_else(|| Error::Runtime(String::from("could not determine repository root")))
}

fn schema_json() -> Result<String, Error> {
    serde_json::to_string_pretty(&v1_schema())
        .map(|schema| format!("{schema}\n"))
        .map_err(|error| Error::Runtime(format!("could not serialize schema: {error}")))
}

fn write_schema(artifact: &Path) -> Result<(), Error> {
    let parent = artifact
        .parent()
        .ok_or_else(|| Error::Runtime(format!("schema artifact has no parent directory: {}", artifact.display())))?;
    fs::create_dir_all(parent).map_err(|error| Error::Runtime(format!("could not create {}: {error}", parent.display())))?;
    fs::write(artifact, schema_json()?)
        .map_err(|error| Error::Runtime(format!("could not write {}: {error}", artifact.display())))
}

fn check_schema(artifact: &Path) -> Result<(), Error> {
    let actual = fs::read_to_string(artifact)
        .map_err(|error| Error::Runtime(format!("could not read {}: {error}", artifact.display())))?;
    if actual == schema_json()? {
        return Ok(());
    }

    Err(Error::Runtime(format!(
        "{} differs from the deterministic v1 schema output; run `cargo run --offline --package frontmatter-validator --bin frontmatter-schema -- generate --artifact {}`",
        artifact.display(),
        artifact.display(),
    )))
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::PathBuf;
    use std::process::ExitCode;

    use tempfile::TempDir;

    use super::{Command, Error, check_schema, schema_json, write_schema};

    fn parse(arguments: &[&str]) -> Result<Command, Error> {
        Command::parse(arguments.iter().map(ToString::to_string))
    }

    #[test]
    fn it_should_exit_with_code_two_for_a_usage_error() {
        // Arrange: the command was invoked with invalid arguments.
        let error = Error::Usage;

        // Act: map the error to a process exit code.
        let exit_code = error.exit_code();

        // Assert: usage errors use the CLI contract's dedicated code.
        assert_eq!(exit_code, ExitCode::from(2));
    }

    #[test]
    fn it_should_exit_with_code_one_for_a_runtime_failure() {
        // Arrange: the command failed while doing its work.
        let error = Error::Runtime(String::from("could not read schema.json"));

        // Act: map the error to a process exit code.
        let exit_code = error.exit_code();

        // Assert: runtime failures use the generic failure code.
        assert_eq!(exit_code, ExitCode::FAILURE);
    }

    #[test]
    fn it_should_write_the_canonical_schema_bytes_creating_missing_parent_directories() {
        // Arrange: the artifact path lies under directories that do not exist yet.
        let directory = TempDir::new().unwrap();
        let artifact = directory.path().join("nested").join("dir").join("frontmatter-v1.schema.json");

        // Act: generate the artifact.
        write_schema(&artifact).unwrap();

        // Assert: the file holds exactly the canonical schema bytes.
        assert_eq!(fs::read_to_string(&artifact).unwrap(), schema_json().unwrap());
    }

    #[test]
    fn it_should_accept_an_artifact_that_matches_the_canonical_schema() {
        // Arrange: the artifact was produced by the generator and not modified since.
        let directory = TempDir::new().unwrap();
        let artifact = directory.path().join("frontmatter-v1.schema.json");
        write_schema(&artifact).unwrap();

        // Act: check the artifact against the canonical schema output.
        let result = check_schema(&artifact);

        // Assert: no drift is reported.
        assert_eq!(result, Ok(()));
    }

    #[test]
    fn it_should_parse_a_generate_action_with_an_explicit_artifact_path() {
        // Arrange: a caller requests generation into a disposable artifact path.
        let arguments = ["generate", "--artifact", ".tmp/frontmatter-v1.schema.json"];

        // Act: parse the command.
        let command = parse(&arguments).unwrap();

        // Assert: the command targets the requested copy instead of the tracked artifact.
        assert_eq!(
            command,
            Command::Generate {
                artifact: PathBuf::from(".tmp/frontmatter-v1.schema.json")
            }
        );
    }

    #[test]
    fn it_should_parse_a_check_action_defaulting_to_the_tracked_artifact_in_the_repository_checkout() {
        // Arrange: the caller passes the action and no artifact option.
        let arguments = ["check"];

        // Act: parse the command.
        let Command::Check { artifact } = parse(&arguments).unwrap() else {
            panic!("expected a check command");
        };

        // Assert: the default is the tracked artifact, which must exist so the crate location
        // walk cannot silently point at a directory outside the repository.
        assert!(
            artifact.ends_with("docs/schemas/frontmatter-v1.schema.json"),
            "{}",
            artifact.display()
        );
        assert!(artifact.is_file(), "{} does not exist", artifact.display());
    }

    #[test]
    fn it_should_reject_a_missing_action() {
        // Arrange: the caller passes no arguments at all.
        let arguments: [&str; 0] = [];

        // Act: parse the command.
        let error = parse(&arguments).unwrap_err();

        // Assert: the command reports its usage.
        assert_eq!(error, Error::Usage);
    }

    #[test]
    fn it_should_reject_an_unknown_action() {
        // Arrange: the caller passes an action the command does not define.
        let arguments = ["validate"];

        // Act: parse the command.
        let error = parse(&arguments).unwrap_err();

        // Assert: the command reports its usage.
        assert_eq!(error, Error::Usage);
    }

    #[test]
    fn it_should_reject_an_artifact_option_without_a_path() {
        // Arrange: the option is present but its value is missing.
        let arguments = ["check", "--artifact"];

        // Act: parse the command.
        let error = parse(&arguments).unwrap_err();

        // Assert: the command reports its usage.
        assert_eq!(error, Error::Usage);
    }

    #[test]
    fn it_should_reject_an_unknown_option() {
        // Arrange: the caller passes an option the command does not define.
        let arguments = ["check", "--output", "schema.json"];

        // Act: parse the command.
        let error = parse(&arguments).unwrap_err();

        // Assert: the command reports its usage.
        assert_eq!(error, Error::Usage);
    }

    #[test]
    fn it_should_reject_extra_arguments_after_the_artifact_path() {
        // Arrange: a trailing argument follows a complete artifact option.
        let arguments = ["check", "--artifact", "a.json", "b.json"];

        // Act: parse the command.
        let error = parse(&arguments).unwrap_err();

        // Assert: the command reports its usage.
        assert_eq!(error, Error::Usage);
    }

    #[test]
    fn it_should_report_a_missing_artifact_when_checking() {
        // Arrange: the artifact path does not exist.
        let directory = TempDir::new().unwrap();
        let artifact = directory.path().join("missing.json");

        // Act: check the missing artifact.
        let error = check_schema(&artifact).unwrap_err().to_string();

        // Assert: the error names the read failure and the path.
        assert!(error.starts_with("could not read"), "{error}");
        assert!(error.contains("missing.json"), "{error}");
    }

    #[test]
    fn it_should_report_a_parent_that_cannot_be_created_when_generating() {
        // Arrange: the artifact's parent path is a regular file, so no directory can be created there.
        let directory = TempDir::new().unwrap();
        let blocker = directory.path().join("blocker");
        fs::write(&blocker, "").unwrap();
        let artifact = blocker.join("frontmatter-v1.schema.json");

        // Act: generate the artifact.
        let error = write_schema(&artifact).unwrap_err().to_string();

        // Assert: the error names the directory-creation failure and the blocking path.
        assert!(error.starts_with("could not create"), "{error}");
        assert!(error.contains("blocker"), "{error}");
    }

    #[test]
    fn it_should_detect_drift_from_the_deterministic_schema_output() {
        // Arrange: a disposable artifact contains content that differs from the generated schema.
        let directory = TempDir::new().unwrap();
        let artifact = directory.path().join("frontmatter-v1.schema.json");
        fs::write(&artifact, "{}\n").unwrap();

        // Act: check the artifact against the canonical schema output.
        let error = check_schema(&artifact).unwrap_err().to_string();

        // Assert: the check explains that the differing artifact must be regenerated.
        assert!(error.contains("differs from the deterministic v1 schema output"));
        assert!(error.contains("--offline"));
        assert!(error.contains("--artifact"));
    }

    #[test]
    fn it_should_generate_deterministic_schema_json() {
        // Arrange: the canonical strict profile model is unchanged between generations.

        // Act: serialize its JSON Schema projection twice.
        let first = schema_json().unwrap();
        let second = schema_json().unwrap();

        // Assert: both serializations produce identical newline-terminated artifact bytes.
        assert_eq!(first, second);
        assert!(first.ends_with('\n'));
    }
}
