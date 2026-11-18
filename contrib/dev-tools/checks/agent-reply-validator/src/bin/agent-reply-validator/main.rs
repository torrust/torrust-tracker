//! Checks agent reply files and reports NDJSON records on stderr.
//!
//! Output class `no-stdout-result`: stdout stays empty; exit codes are `0` when every file is
//! valid, `1` for an invalid file or a runtime failure, and `2` for invalid invocation.

mod git;
mod record;

use std::ffi::OsString;
use std::io::{self, Write};
use std::path::{Component, Path, PathBuf};
use std::process::ExitCode;
use std::{env, fs};

use agent_reply_validator::file::{check_reply_file, is_reply_file_candidate};
use clap::error::ErrorKind;
use clap::{ArgGroup, Parser};
use git::Git;
use record::{EXIT_FAILURE, EXIT_SUCCESS, EXIT_USAGE, Record, emit};

/// Check agent reply files: their place, name and content, and that the name agrees with the
/// content.
#[derive(Debug, Parser)]
#[command(
    name = "agent-reply-validator",
    group(ArgGroup::new("mode").required(true).args(["paths", "all"]))
)]
struct Arguments {
    /// Reply files to check as they are on disk, whatever their names.
    paths: Vec<PathBuf>,
    /// Check, as staged, every file in the index that is in an `agent-replies` folder or is
    /// named like a reply file.
    #[arg(long)]
    all: bool,
}

fn main() -> ExitCode {
    let working_directory = env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    let mut stderr = io::stderr().lock();
    ExitCode::from(run(env::args_os(), &working_directory, &mut stderr))
}

/// Runs the command with the program name as the first argument and returns its exit code.
fn run(arguments: impl IntoIterator<Item = OsString>, working_directory: &Path, stderr: &mut impl Write) -> u8 {
    let (records, exit_code) = match Arguments::try_parse_from(arguments) {
        Ok(arguments) => match Git::discover(working_directory) {
            Ok(git) if arguments.all => check_index(&git),
            Ok(git) => check_paths(&git, working_directory, &arguments.paths),
            Err(message) => (vec![Record::runtime_error(None, message)], EXIT_FAILURE),
        },
        Err(error) if error.kind() == ErrorKind::DisplayHelp => (
            vec![Record::Help {
                message: error.to_string().trim_end().to_owned(),
            }],
            EXIT_SUCCESS,
        ),
        Err(error) => (vec![Record::usage_error(error.to_string().trim_end())], EXIT_USAGE),
    };

    match emit(&records, stderr) {
        Ok(()) => exit_code,
        Err(_) => EXIT_FAILURE,
    }
}

/// Checks the staged content of every candidate in the index, so a commit is checked as it will
/// be made. A candidate with merge conflicts has no staged content, so the run stops there with
/// a runtime error.
fn check_index(git: &Git) -> (Vec<Record>, u8) {
    let paths = match git.tracked_paths() {
        Ok(paths) => paths,
        Err(message) => return (vec![Record::runtime_error(None, message)], EXIT_FAILURE),
    };
    let mut checked = Vec::new();
    for bytes in paths {
        let path = match String::from_utf8(bytes) {
            Ok(path) if is_reply_file_candidate(&path) => path,
            Ok(_) => continue,
            Err(error) => {
                let lossy = String::from_utf8_lossy(error.as_bytes()).into_owned();
                if !is_reply_file_candidate(&lossy) {
                    continue;
                }
                return (
                    vec![Record::runtime_error(Some(lossy), "the path is not valid UTF-8")],
                    EXIT_FAILURE,
                );
            }
        };
        match git.index_content(&path).map(String::from_utf8) {
            Ok(Ok(text)) => checked.push((path, text)),
            Ok(Err(_)) => {
                return (
                    vec![Record::runtime_error(Some(path), "the file is not valid UTF-8")],
                    EXIT_FAILURE,
                );
            }
            Err(message) => return (vec![Record::runtime_error(Some(path), message)], EXIT_FAILURE),
        }
    }
    report(&checked)
}

/// Checks the given files as they are on disk; every one is checked, whatever its name.
///
/// A symbolic link is rejected: git stores a link as its target's path, so `--all` would read
/// that path, not the content behind it, and the two modes would disagree.
fn check_paths(git: &Git, working_directory: &Path, paths: &[PathBuf]) -> (Vec<Record>, u8) {
    let root = match fs::canonicalize(git.root()) {
        Ok(root) => root,
        Err(error) => return (vec![Record::runtime_error(None, error.to_string())], EXIT_FAILURE),
    };
    let mut checked = Vec::new();
    for path in paths {
        let shown = Some(path.display().to_string());
        let file = working_directory.join(path);
        if fs::symlink_metadata(&file).is_ok_and(|metadata| metadata.file_type().is_symlink()) {
            let message = "the file is a symbolic link; a reply file must be a regular file";
            return (vec![Record::runtime_error(shown, message)], EXIT_FAILURE);
        }
        let relative = match file.parent().zip(file.file_name()) {
            Some((folder, name)) => fs::canonicalize(folder)
                .map_err(|error| error.to_string())
                .and_then(|folder| repository_relative(&root, &folder.join(name)).map_err(str::to_owned)),
            None => Err("the path does not name a file".to_owned()),
        };
        let relative = match relative {
            Ok(relative) => relative,
            Err(message) => return (vec![Record::runtime_error(shown, message)], EXIT_FAILURE),
        };
        match fs::read(&file).map(String::from_utf8) {
            Ok(Ok(text)) => checked.push((relative, text)),
            Ok(Err(_)) => {
                return (
                    vec![Record::runtime_error(shown, "the file is not valid UTF-8")],
                    EXIT_FAILURE,
                );
            }
            Err(error) => return (vec![Record::runtime_error(shown, error.to_string())], EXIT_FAILURE),
        }
    }
    report(&checked)
}

/// The records of every problem in the checked files, a summary, and the exit code.
fn report(files: &[(String, String)]) -> (Vec<Record>, u8) {
    let mut records: Vec<Record> = files
        .iter()
        .flat_map(|(path, text)| {
            check_reply_file(path, text)
                .into_iter()
                .map(|problem| Record::problem(path, problem))
        })
        .collect();
    let problems = records.len();
    records.push(Record::Summary {
        files: files.len(),
        problems,
    });
    (records, if problems == 0 { EXIT_SUCCESS } else { EXIT_FAILURE })
}

/// The path of `file` relative to `root`, with `/` separators. `root` is canonical, and so is
/// the folder of `file`.
fn repository_relative(root: &Path, file: &Path) -> Result<String, &'static str> {
    let inside = file.strip_prefix(root).map_err(|_| "the file is not inside the repository")?;
    inside
        .components()
        .map(|component| match component {
            Component::Normal(part) => part.to_str().ok_or("the path is not valid UTF-8"),
            _ => Err("the file is not inside the repository"),
        })
        .collect::<Result<Vec<_>, _>>()
        .map(|parts| parts.join("/"))
}

#[cfg(test)]
mod tests {
    use std::io::{self, Write};
    use std::path::Path;

    use super::{EXIT_FAILURE, EXIT_SUCCESS, Record, report, repository_relative, run};

    /// A stderr that refuses every write.
    struct FailingWriter;

    impl Write for FailingWriter {
        fn write(&mut self, _: &[u8]) -> io::Result<usize> {
            Err(io::Error::other("stderr is closed"))
        }

        fn flush(&mut self) -> io::Result<()> {
            Err(io::Error::other("stderr is closed"))
        }
    }

    #[test]
    fn it_should_exit_one_when_the_records_cannot_be_written() {
        // Act: `--help` succeeds on its own, so only the failed write can make the run fail.
        let exit_code = run(
            ["agent-reply-validator", "--help"].map(Into::into),
            Path::new("."),
            &mut FailingWriter,
        );

        // Assert
        assert_eq!(exit_code, EXIT_FAILURE);
    }

    #[test]
    fn it_should_report_only_an_empty_summary_and_succeed_when_no_file_is_checked() {
        // Act
        let (records, exit_code) = report(&[]);

        // Assert
        assert_eq!(exit_code, EXIT_SUCCESS);
        assert!(matches!(records.as_slice(), [Record::Summary { files: 0, problems: 0 }]));
    }

    #[test]
    fn it_should_report_each_problem_then_the_summary_and_fail_when_a_file_is_invalid() {
        // Arrange
        let files = [("notes.md".to_owned(), "{}".to_owned())];

        // Act
        let (records, exit_code) = report(&files);

        // Assert: the location, the name and the content of `notes.md` are all wrong.
        assert_eq!(exit_code, EXIT_FAILURE);
        assert!(matches!(
            records.as_slice(),
            [
                Record::Problem { .. },
                Record::Problem { .. },
                Record::Problem { .. },
                Record::Summary { files: 1, problems: 3 }
            ]
        ));
    }

    #[test]
    fn it_should_join_the_parts_with_slashes_when_the_file_is_inside_the_root() {
        // Act
        let relative = repository_relative(Path::new("/repo"), Path::new("/repo/docs/agent-replies/x.json"));

        // Assert
        assert_eq!(relative, Ok("docs/agent-replies/x.json".to_owned()));
    }

    #[test]
    fn it_should_reject_the_file_when_it_is_outside_the_root() {
        // Act
        let relative = repository_relative(Path::new("/repo"), Path::new("/other/x.json"));

        // Assert
        assert_eq!(relative, Err("the file is not inside the repository"));
    }

    #[cfg(unix)]
    #[test]
    fn it_should_reject_the_file_when_a_part_of_its_path_is_not_utf8() {
        // Arrange
        use std::ffi::OsStr;
        use std::os::unix::ffi::OsStrExt;
        let file = Path::new("/repo").join(OsStr::from_bytes(b"bad-\xff.json"));

        // Act
        let relative = repository_relative(Path::new("/repo"), &file);

        // Assert
        assert_eq!(relative, Err("the path is not valid UTF-8"));
    }
}
