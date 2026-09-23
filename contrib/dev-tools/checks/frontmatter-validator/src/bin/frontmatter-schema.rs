//! Generates and verifies the tracked frontmatter v1 JSON Schema artifact.

use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::{env, fmt, fs};

use frontmatter_validator::v1_schema_json;
use tempfile::NamedTempFile;

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
    #[error("could not create a staging file in {}: {source}", directory.display())]
    CreateStagingFile { directory: PathBuf, source: io::Error },
    #[error("could not write the staging file for {}: {source}", artifact.display())]
    WriteStagingFile { artifact: PathBuf, source: io::Error },
    #[error("could not synchronize the staging file for {}: {source}", artifact.display())]
    SynchronizeStagingFile { artifact: PathBuf, source: io::Error },
    #[error("could not atomically replace {}: {source}", artifact.display())]
    ReplaceArtifact { artifact: PathBuf, source: io::Error },
    #[error("could not read {}: {source}", artifact.display())]
    Read { artifact: PathBuf, source: io::Error },
    #[error("{} differs from the deterministic v1 schema output; {regeneration}", artifact.display())]
    Drift {
        artifact: PathBuf,
        regeneration: RegenerationInstruction,
    },
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

/// An actionable, non-shell-formatted instruction for regenerating a schema artifact.
#[derive(Debug, Eq, PartialEq)]
enum RegenerationInstruction {
    /// The tracked artifact has one documented copy-paste command.
    TrackedArtifact,
    /// A caller-selected path must remain diagnostic data instead of shell syntax.
    ExplicitArtifact,
}

impl fmt::Display for RegenerationInstruction {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TrackedArtifact => write!(
                formatter,
                "run `cargo run --offline --package frontmatter-validator --bin frontmatter-schema -- generate`"
            ),
            Self::ExplicitArtifact => write!(
                formatter,
                "run the documented generator with `--artifact <path>` and the artifact path above"
            ),
        }
    }
}

fn main() -> ExitCode {
    let mut stderr = io::stderr().lock();
    run(env::args().skip(1), &mut stderr).unwrap_or(ExitCode::FAILURE)
}

/// Executes the command and emits its human-readable diagnostics to stderr.
fn run(arguments: impl Iterator<Item = String>, stderr: &mut impl Write) -> io::Result<ExitCode> {
    match Command::parse(arguments).and_then(|command| command.execute()) {
        Ok(()) => Ok(ExitCode::SUCCESS),
        Err(error) => {
            writeln!(stderr, "frontmatter-schema: {error}")?;
            Ok(error.exit_code())
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
    origin: ArtifactOrigin,
}

/// How a schema artifact path was selected.
#[derive(Debug, Eq, PartialEq)]
enum ArtifactOrigin {
    /// The artifact tracked in the repository.
    Tracked,
    /// A path supplied by the caller through `--artifact`.
    Explicit,
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
            .map(|root| Self {
                path: root.join(ARTIFACT_PATH),
                origin: ArtifactOrigin::Tracked,
            })
            .ok_or(Error::RepositoryRoot)
    }

    /// An artifact at a caller-chosen path.
    const fn at(path: PathBuf) -> Self {
        Self {
            path,
            origin: ArtifactOrigin::Explicit,
        }
    }

    /// The safe regeneration instruction for this artifact.
    const fn regeneration_instruction(&self) -> RegenerationInstruction {
        match self.origin {
            ArtifactOrigin::Tracked => RegenerationInstruction::TrackedArtifact,
            ArtifactOrigin::Explicit => RegenerationInstruction::ExplicitArtifact,
        }
    }

    /// Atomically replaces the artifact with canonical bytes, creating missing parent directories.
    fn write(&self) -> Result<(), Error> {
        let directory = self.path.parent().ok_or_else(|| Error::NoParentDirectory {
            artifact: self.path.clone(),
        })?;
        fs::create_dir_all(directory).map_err(|source| Error::CreateDirectory {
            directory: directory.to_path_buf(),
            source,
        })?;

        let mut staging_file = NamedTempFile::new_in(directory).map_err(|source| Error::CreateStagingFile {
            directory: directory.to_path_buf(),
            source,
        })?;
        staging_file
            .write_all(v1_schema_json().as_bytes())
            .map_err(|source| Error::WriteStagingFile {
                artifact: self.path.clone(),
                source,
            })?;
        staging_file
            .as_file_mut()
            .sync_all()
            .map_err(|source| Error::SynchronizeStagingFile {
                artifact: self.path.clone(),
                source,
            })?;
        staging_file.persist(&self.path).map_err(|error| Error::ReplaceArtifact {
            artifact: self.path.clone(),
            source: error.error,
        })?;

        Ok(())
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
                regeneration: self.regeneration_instruction(),
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::io::{self, Write};
    #[cfg(unix)]
    use std::os::unix::fs::symlink;
    use std::path::PathBuf;
    use std::process::ExitCode;

    use frontmatter_validator::v1_schema_json;
    use tempfile::TempDir;

    use super::{Command, Error, RegenerationInstruction, SchemaArtifact, run};

    fn parse(arguments: &[&str]) -> Result<Command, Error> {
        Command::parse(arguments.iter().map(ToString::to_string))
    }

    struct FailingWriter;

    impl Write for FailingWriter {
        fn write(&mut self, _buffer: &[u8]) -> io::Result<usize> {
            Err(io::Error::other("intentional write failure"))
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
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
            regeneration: RegenerationInstruction::TrackedArtifact,
        };

        // Act: map the error to a process exit code.
        let exit_code = error.exit_code();

        // Assert: runtime failures use the generic failure code.
        assert_eq!(exit_code, ExitCode::FAILURE);
    }

    #[test]
    fn it_should_report_no_stderr_for_a_successful_process_run() {
        // Arrange: generation writes to an explicit disposable artifact.
        let directory = TempDir::new().unwrap();
        let arguments = vec![
            String::from("generate"),
            String::from("--artifact"),
            directory.path().join("frontmatter-v1.schema.json").display().to_string(),
        ];
        let mut stderr = Vec::new();

        // Act: execute the process adapter.
        let exit_code = run(arguments.into_iter(), &mut stderr).unwrap();

        // Assert: success writes no diagnostic and returns the success code.
        assert_eq!(exit_code, ExitCode::SUCCESS);
        assert!(stderr.is_empty(), "{}", String::from_utf8_lossy(&stderr));
    }

    #[test]
    fn it_should_report_a_usage_error_to_stderr_with_code_two() {
        // Arrange: the process arguments omit the required action.
        let mut stderr = Vec::new();

        // Act: execute the process adapter.
        let exit_code = run(std::iter::empty(), &mut stderr).unwrap();

        // Assert: usage failures follow the CLI's one-line stderr contract.
        assert_eq!(exit_code, ExitCode::from(2));
        assert_eq!(
            String::from_utf8(stderr).unwrap(),
            "frontmatter-schema: usage: frontmatter-schema <generate|check> [--artifact <path>]\n"
        );
    }

    #[test]
    fn it_should_report_a_runtime_error_to_stderr_with_code_one() {
        // Arrange: the named artifact differs from the canonical schema output.
        let directory = TempDir::new().unwrap();
        let path = directory.path().join("drifted.json");
        fs::write(&path, "{}\n").unwrap();
        let arguments = vec![String::from("check"), String::from("--artifact"), path.display().to_string()];
        let mut stderr = Vec::new();

        // Act: execute the process adapter.
        let exit_code = run(arguments.into_iter(), &mut stderr).unwrap();

        // Assert: runtime failures use one prefixed line and the generic failure code.
        assert_eq!(exit_code, ExitCode::FAILURE);
        assert_eq!(
            String::from_utf8(stderr).unwrap(),
            format!(
                "frontmatter-schema: {} differs from the deterministic v1 schema output; run the documented generator with `--artifact <path>` and the artifact path above\n",
                path.display()
            )
        );
    }

    #[test]
    fn it_should_return_an_error_when_stderr_cannot_be_written() {
        // Arrange: malformed arguments require a diagnostic, but the output writer always fails.
        let mut stderr = FailingWriter;

        // Act: execute the process adapter.
        let error = run(std::iter::empty(), &mut stderr).unwrap_err();

        // Assert: the output failure reaches the process boundary instead of being discarded.
        assert_eq!(error.kind(), io::ErrorKind::Other);
        assert_eq!(error.to_string(), "intentional write failure");
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
    fn it_should_replace_an_existing_artifact_without_leaving_a_temporary_file() {
        // Arrange: the destination contains stale bytes before generation.
        let directory = TempDir::new().unwrap();
        let path = directory.path().join("frontmatter-v1.schema.json");
        fs::write(&path, "stale schema\n").unwrap();

        // Act: replace the existing artifact with canonical bytes.
        SchemaArtifact::at(path.clone()).write().unwrap();

        // Assert: the completed artifact is canonical and no staging filename remains.
        assert_eq!(fs::read_to_string(&path).unwrap(), v1_schema_json());
        assert_eq!(
            fs::read_dir(directory.path())
                .unwrap()
                .map(Result::unwrap)
                .map(|entry| entry.file_name())
                .collect::<Vec<_>>(),
            vec!["frontmatter-v1.schema.json"]
        );
    }

    #[cfg(unix)]
    #[test]
    fn it_should_replace_a_destination_symlink_without_modifying_its_referent() {
        // Arrange: the artifact path is a link to a separate sentinel file with stale bytes.
        let directory = TempDir::new().unwrap();
        let referent = directory.path().join("schema-referent.json");
        let artifact = directory.path().join("frontmatter-v1.schema.json");
        fs::write(&referent, "stale schema\n").unwrap();
        symlink(&referent, &artifact).unwrap();

        // Act: generate the selected artifact path.
        SchemaArtifact::at(artifact.clone()).write().unwrap();

        // Assert: generation replaced the link, leaving its former referent unchanged.
        assert_eq!(fs::read_to_string(&artifact).unwrap(), v1_schema_json());
        assert!(!fs::symlink_metadata(&artifact).unwrap().file_type().is_symlink());
        assert_eq!(fs::read_to_string(&referent).unwrap(), "stale schema\n");
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

        // Assert: drift is reported for that artifact with the hint for regenerating that same copy.
        assert!(
            matches!(&error, Error::Drift { artifact, .. } if *artifact == path),
            "{error:?}"
        );
        let message = error.to_string();
        assert!(
            message.ends_with("`--artifact <path>` and the artifact path above"),
            "{message}"
        );
    }

    #[test]
    fn it_should_suggest_the_documented_default_command_for_the_tracked_artifact() {
        // Arrange: the artifact is the tracked default, selected without `--artifact`.
        let artifact = SchemaArtifact::tracked().unwrap();

        // Act: render the drift diagnostic.
        let message = Error::Drift {
            artifact: artifact.path.clone(),
            regeneration: artifact.regeneration_instruction(),
        }
        .to_string();

        // Assert: the hint is exactly the command documented in docs/schemas/README.md.
        assert_eq!(
            message,
            format!(
                "{} differs from the deterministic v1 schema output; run `cargo run --offline --package frontmatter-validator --bin frontmatter-schema -- generate`",
                artifact.path.display()
            )
        );
    }

    #[test]
    fn it_should_suggest_regenerating_the_same_copy_for_an_explicit_artifact() {
        // Arrange: the caller named a disposable copy with `--artifact`.
        let artifact = SchemaArtifact::at(PathBuf::from(".tmp/copy.json"));

        // Act: render the drift diagnostic.
        let message = Error::Drift {
            artifact: artifact.path.clone(),
            regeneration: artifact.regeneration_instruction(),
        }
        .to_string();

        // Assert: the copy is named as data and guidance remains safe shell-independent text.
        assert_eq!(
            message,
            ".tmp/copy.json differs from the deterministic v1 schema output; run the documented generator with `--artifact <path>` and the artifact path above"
        );
    }

    #[test]
    fn it_should_select_a_regeneration_instruction_from_the_artifact_origin() {
        // Arrange: one artifact is tracked and the other was named explicitly by the caller.
        let tracked = SchemaArtifact::tracked().unwrap();
        let explicit = SchemaArtifact::at(PathBuf::from(".tmp/copy.json"));

        // Act: select the regeneration instruction for each artifact.
        let tracked_instruction = tracked.regeneration_instruction();
        let explicit_instruction = explicit.regeneration_instruction();

        // Assert: each closed origin has exactly one matching instruction.
        assert_eq!(tracked_instruction, RegenerationInstruction::TrackedArtifact);
        assert_eq!(explicit_instruction, RegenerationInstruction::ExplicitArtifact);
    }

    #[test]
    fn it_should_not_embed_an_explicit_artifact_path_in_a_shell_command() {
        // Arrange: the explicit path contains text that a shell would interpret if pasted into a command.
        let directory = TempDir::new().unwrap();
        let path = directory.path().join("schema with spaces $(unexpected).json");
        fs::write(&path, "{}\n").unwrap();

        // Act: check the non-canonical artifact and render its diagnostic.
        let message = SchemaArtifact::at(path.clone()).verify_current().unwrap_err().to_string();

        // Assert: the path remains data, not part of an executable shell command.
        assert!(message.contains(&path.display().to_string()), "{message}");
        assert!(message.contains("--artifact <path>"), "{message}");
        assert!(!message.contains(&format!("--artifact {}", path.display())), "{message}");
    }
}
