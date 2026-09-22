//! Generates and verifies the tracked frontmatter v1 JSON Schema artifact.

use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::{env, fs};

use frontmatter_validator::v1_schema;

const ARTIFACT_PATH: &str = "docs/schemas/frontmatter-v1.schema.json";

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            let mut stderr = io::stderr().lock();
            drop(writeln!(stderr, "frontmatter-schema: {error}"));
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<(), String> {
    let mut arguments = env::args().skip(1);
    let action = arguments.next().ok_or_else(usage)?;
    let artifact = artifact_path(&arguments.collect::<Vec<_>>())?;

    match action.as_str() {
        "generate" => write_schema(&artifact),
        "check" => check_schema(&artifact),
        _ => Err(usage()),
    }
}

fn usage() -> String {
    String::from("usage: frontmatter-schema <generate|check> [--artifact <path>]")
}

fn artifact_path(arguments: &[String]) -> Result<PathBuf, String> {
    match arguments {
        [] => Ok(repository_root()?.join(ARTIFACT_PATH)),
        [flag, path] if flag == "--artifact" => Ok(PathBuf::from(path)),
        _ => Err(usage()),
    }
}

fn repository_root() -> Result<PathBuf, String> {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(4)
        .map(Path::to_path_buf)
        .ok_or_else(|| String::from("could not determine repository root"))
}

fn schema_json() -> Result<String, String> {
    serde_json::to_string_pretty(&v1_schema())
        .map(|schema| format!("{schema}\n"))
        .map_err(|error| format!("could not serialize schema: {error}"))
}

fn write_schema(artifact: &Path) -> Result<(), String> {
    let parent = artifact
        .parent()
        .ok_or_else(|| format!("schema artifact has no parent directory: {}", artifact.display()))?;
    fs::create_dir_all(parent).map_err(|error| format!("could not create {}: {error}", parent.display()))?;
    fs::write(artifact, schema_json()?).map_err(|error| format!("could not write {}: {error}", artifact.display()))
}

fn check_schema(artifact: &Path) -> Result<(), String> {
    let actual = fs::read_to_string(artifact).map_err(|error| format!("could not read {}: {error}", artifact.display()))?;
    if actual == schema_json()? {
        return Ok(());
    }

    Err(format!(
        "{} differs from the deterministic v1 schema output; run `cargo run --offline --package frontmatter-validator --bin frontmatter-schema -- generate --artifact {}`",
        artifact.display(),
        artifact.display(),
    ))
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::PathBuf;

    use tempfile::TempDir;

    use super::{artifact_path, check_schema, schema_json, usage, write_schema};

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
    fn it_should_use_an_explicit_artifact_path_when_requested() {
        // Arrange: a caller requests a disposable artifact path for a schema operation.
        let arguments = vec![String::from("--artifact"), String::from(".tmp/frontmatter-v1.schema.json")];

        // Act: resolve the artifact path from the command arguments.
        let artifact = artifact_path(&arguments).unwrap();

        // Assert: the command uses the requested copy instead of the tracked artifact.
        assert_eq!(artifact, PathBuf::from(".tmp/frontmatter-v1.schema.json"));
    }

    #[test]
    fn it_should_default_to_the_tracked_artifact_in_the_repository_checkout() {
        // Arrange: the caller passes no artifact option.
        let arguments: Vec<String> = vec![];

        // Act: resolve the artifact path from the command arguments.
        let artifact = artifact_path(&arguments).unwrap();

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
    fn it_should_reject_an_artifact_option_without_a_path() {
        // Arrange: the option is present but its value is missing.
        let arguments = vec![String::from("--artifact")];

        // Act: resolve the artifact path from the command arguments.
        let error = artifact_path(&arguments).unwrap_err();

        // Assert: the command reports its usage.
        assert_eq!(error, usage());
    }

    #[test]
    fn it_should_reject_an_unknown_option() {
        // Arrange: the caller passes an option the command does not define.
        let arguments = vec![String::from("--output"), String::from("schema.json")];

        // Act: resolve the artifact path from the command arguments.
        let error = artifact_path(&arguments).unwrap_err();

        // Assert: the command reports its usage.
        assert_eq!(error, usage());
    }

    #[test]
    fn it_should_reject_extra_arguments_after_the_artifact_path() {
        // Arrange: a trailing argument follows a complete artifact option.
        let arguments = vec![String::from("--artifact"), String::from("a.json"), String::from("b.json")];

        // Act: resolve the artifact path from the command arguments.
        let error = artifact_path(&arguments).unwrap_err();

        // Assert: the command reports its usage.
        assert_eq!(error, usage());
    }

    #[test]
    fn it_should_detect_drift_from_the_deterministic_schema_output() {
        // Arrange: a disposable artifact contains content that differs from the generated schema.
        let directory = TempDir::new().unwrap();
        let artifact = directory.path().join("frontmatter-v1.schema.json");
        fs::write(&artifact, "{}\n").unwrap();

        // Act: check the artifact against the canonical schema output.
        let error = check_schema(&artifact).unwrap_err();

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
