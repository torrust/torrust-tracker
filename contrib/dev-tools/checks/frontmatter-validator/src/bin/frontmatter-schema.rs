//! Generates and verifies the tracked frontmatter v1 JSON Schema artifact.

use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::{env, fs};

use frontmatter_validator::v1_schema_json;

const ARTIFACT_PATH: &str = "docs/schemas/frontmatter-v1.schema.json";

/// Failures reported by the command. Every variant except `Usage` is a runtime failure.
#[derive(Debug, thiserror::Error)]
enum Error {
    #[error("usage: frontmatter-schema <generate|check> [--artifact <path>]")]
    Usage,
    #[error("could not determine repository root")]
    RepositoryRoot,
    #[error("schema artifact has no parent directory: {}", artifact.display())]
    NoParentDirectory { artifact: PathBuf },
    #[error("could not create {}: {source}", directory.display())]
    CreateDirectory { directory: PathBuf, source: io::Error },
    #[error("could not write {}: {source}", artifact.display())]
    Write { artifact: PathBuf, source: io::Error },
    #[error("could not read {}: {source}", artifact.display())]
    Read { artifact: PathBuf, source: io::Error },
    #[error(
        "{} differs from the deterministic v1 schema output; run `cargo run --offline --package frontmatter-validator --bin frontmatter-schema -- generate --artifact {}`",
        artifact.display(),
        artifact.display()
    )]
    Drift { artifact: PathBuf },
}

impl Error {
    /// Exit codes follow the repository CLI output contract: `2` for usage errors, `1` otherwise.
    fn exit_code(&self) -> ExitCode {
        match self {
            Self::Usage => ExitCode::from(2),
            _ => ExitCode::FAILURE,
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
    Generate(SchemaArtifact),
    Check(SchemaArtifact),
}

impl Command {
    /// Parses the process arguments after the program name. Performs no I/O.
    fn parse(mut arguments: impl Iterator<Item = String>) -> Result<Self, Error> {
        let action = arguments.next().ok_or(Error::Usage)?;
        let artifact = SchemaArtifact::from_arguments(&arguments.collect::<Vec<_>>())?;

        match action.as_str() {
            "generate" => Ok(Self::Generate(artifact)),
            "check" => Ok(Self::Check(artifact)),
            _ => Err(Error::Usage),
        }
    }

    fn execute(&self) -> Result<(), Error> {
        match self {
            Self::Generate(artifact) => artifact.write(),
            Self::Check(artifact) => artifact.verify_current(),
        }
    }
}

/// The on-disk JSON Schema file whose bytes must equal `v1_schema_json()`.
#[derive(Debug, Eq, PartialEq)]
struct SchemaArtifact {
    path: PathBuf,
}

impl SchemaArtifact {
    fn from_arguments(arguments: &[String]) -> Result<Self, Error> {
        match arguments {
            [] => Ok(Self::tracked()?),
            [flag, path] if flag == "--artifact" => Ok(Self::at(PathBuf::from(path))),
            _ => Err(Error::Usage),
        }
    }

    /// The artifact committed in the repository that contains this crate.
    fn tracked() -> Result<Self, Error> {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .ancestors()
            .nth(4)
            .map(|root| Self::at(root.join(ARTIFACT_PATH)))
            .ok_or(Error::RepositoryRoot)
    }

    const fn at(path: PathBuf) -> Self {
        Self { path }
    }

    /// Writes the canonical bytes, creating missing parent directories.
    fn write(&self) -> Result<(), Error> {
        let directory = self.path.parent().ok_or_else(|| Error::NoParentDirectory {
            artifact: self.path.clone(),
        })?;
        fs::create_dir_all(directory).map_err(|source| Error::CreateDirectory {
            directory: directory.to_path_buf(),
            source,
        })?;
        fs::write(&self.path, v1_schema_json()).map_err(|source| Error::Write {
            artifact: self.path.clone(),
            source,
        })
    }

    /// Succeeds only when the file's bytes equal the canonical bytes.
    fn verify_current(&self) -> Result<(), Error> {
        let actual = fs::read_to_string(&self.path).map_err(|source| Error::Read {
            artifact: self.path.clone(),
            source,
        })?;
        if actual == v1_schema_json() {
            Ok(())
        } else {
            Err(Error::Drift {
                artifact: self.path.clone(),
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::PathBuf;
    use std::process::ExitCode;

    use frontmatter_validator::v1_schema_json;
    use tempfile::TempDir;

    use super::{Command, Error, SchemaArtifact};

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
        let error = Error::Drift {
            artifact: PathBuf::from("schema.json"),
        };

        // Act: map the error to a process exit code.
        let exit_code = error.exit_code();

        // Assert: runtime failures use the generic failure code.
        assert_eq!(exit_code, ExitCode::FAILURE);
    }

    #[test]
    fn it_should_write_the_canonical_schema_bytes_creating_missing_parent_directories() {
        // Arrange: the artifact path lies under directories that do not exist yet.
        let directory = TempDir::new().unwrap();
        let path = directory.path().join("nested").join("dir").join("frontmatter-v1.schema.json");

        // Act: generate the artifact.
        SchemaArtifact::at(path.clone()).write().unwrap();

        // Assert: the file holds exactly the canonical schema bytes.
        assert_eq!(fs::read_to_string(&path).unwrap(), v1_schema_json());
    }

    #[test]
    fn it_should_accept_an_artifact_that_matches_the_canonical_schema() {
        // Arrange: the artifact was produced by the generator and not modified since.
        let directory = TempDir::new().unwrap();
        let artifact = SchemaArtifact::at(directory.path().join("frontmatter-v1.schema.json"));
        artifact.write().unwrap();

        // Act: check the artifact against the canonical schema output.
        let result = artifact.verify_current();

        // Assert: no drift is reported.
        assert!(result.is_ok(), "{result:?}");
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
            Command::Generate(SchemaArtifact::at(PathBuf::from(".tmp/frontmatter-v1.schema.json")))
        );
    }

    #[test]
    fn it_should_parse_a_check_action_defaulting_to_the_tracked_artifact_in_the_repository_checkout() {
        // Arrange: the caller passes the action and no artifact option.
        let arguments = ["check"];

        // Act: parse the command.
        let Command::Check(artifact) = parse(&arguments).unwrap() else {
            panic!("expected a check command");
        };

        // Assert: the default is the tracked artifact, which must exist so the crate location
        // walk cannot silently point at a directory outside the repository.
        assert!(
            artifact.path.ends_with("docs/schemas/frontmatter-v1.schema.json"),
            "{}",
            artifact.path.display()
        );
        assert!(artifact.path.is_file(), "{} does not exist", artifact.path.display());
    }

    #[test]
    fn it_should_reject_a_missing_action() {
        // Arrange: the caller passes no arguments at all.
        let arguments: [&str; 0] = [];

        // Act: parse the command.
        let error = parse(&arguments).unwrap_err();

        // Assert: the command reports its usage.
        assert!(matches!(error, Error::Usage), "{error:?}");
    }

    #[test]
    fn it_should_reject_an_unknown_action() {
        // Arrange: the caller passes an action the command does not define.
        let arguments = ["validate"];

        // Act: parse the command.
        let error = parse(&arguments).unwrap_err();

        // Assert: the command reports its usage.
        assert!(matches!(error, Error::Usage), "{error:?}");
    }

    #[test]
    fn it_should_reject_an_artifact_option_without_a_path() {
        // Arrange: the option is present but its value is missing.
        let arguments = ["check", "--artifact"];

        // Act: parse the command.
        let error = parse(&arguments).unwrap_err();

        // Assert: the command reports its usage.
        assert!(matches!(error, Error::Usage), "{error:?}");
    }

    #[test]
    fn it_should_reject_an_unknown_option() {
        // Arrange: the caller passes an option the command does not define.
        let arguments = ["check", "--output", "schema.json"];

        // Act: parse the command.
        let error = parse(&arguments).unwrap_err();

        // Assert: the command reports its usage.
        assert!(matches!(error, Error::Usage), "{error:?}");
    }

    #[test]
    fn it_should_reject_extra_arguments_after_the_artifact_path() {
        // Arrange: a trailing argument follows a complete artifact option.
        let arguments = ["check", "--artifact", "a.json", "b.json"];

        // Act: parse the command.
        let error = parse(&arguments).unwrap_err();

        // Assert: the command reports its usage.
        assert!(matches!(error, Error::Usage), "{error:?}");
    }

    #[test]
    fn it_should_report_a_missing_artifact_when_checking() {
        // Arrange: the artifact path does not exist.
        let directory = TempDir::new().unwrap();
        let path = directory.path().join("missing.json");

        // Act: check the missing artifact.
        let error = SchemaArtifact::at(path.clone()).verify_current().unwrap_err();

        // Assert: the error is a read failure naming the artifact.
        assert!(
            matches!(&error, Error::Read { artifact, .. } if *artifact == path),
            "{error:?}"
        );
    }

    #[test]
    fn it_should_report_a_parent_that_cannot_be_created_when_generating() {
        // Arrange: the artifact's parent path is a regular file, so no directory can be created there.
        let directory = TempDir::new().unwrap();
        let blocker = directory.path().join("blocker");
        fs::write(&blocker, "").unwrap();

        // Act: generate the artifact.
        let error = SchemaArtifact::at(blocker.join("frontmatter-v1.schema.json"))
            .write()
            .unwrap_err();

        // Assert: the error is a directory-creation failure naming the blocking path.
        assert!(
            matches!(&error, Error::CreateDirectory { directory, .. } if *directory == blocker),
            "{error:?}"
        );
    }

    #[test]
    fn it_should_detect_drift_from_the_deterministic_schema_output() {
        // Arrange: a disposable artifact contains content that differs from the generated schema.
        let directory = TempDir::new().unwrap();
        let path = directory.path().join("frontmatter-v1.schema.json");
        fs::write(&path, "{}\n").unwrap();

        // Act: check the artifact against the canonical schema output.
        let error = SchemaArtifact::at(path.clone()).verify_current().unwrap_err();

        // Assert: drift is reported for that artifact and the message names offline regeneration.
        assert!(matches!(&error, Error::Drift { artifact } if *artifact == path), "{error:?}");
        let message = error.to_string();
        assert!(message.contains("--offline"), "{message}");
        assert!(message.contains("--artifact"), "{message}");
    }
}
