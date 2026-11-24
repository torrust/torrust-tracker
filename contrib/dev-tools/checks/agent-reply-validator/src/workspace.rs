//! The repository on disk, as the stop hook sees it.
//!
//! [`FileWorkspace`] answers the questions of [`Workspace`] from the file system and keeps the
//! count of refusals in `.agents/issues/<issue-number>/reply-guard/`, one small file per request.
//! That subtree is local workflow state: Git ignores it and the linters exclude it, and nothing
//! committed goes there (decision D17 of issue #2904). `.tmp/` is not used, because agents clean it.

use std::fs;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};

use crate::guard::{RefusalKey, ReplyFile, Workspace};

/// The folder of the repository that holds the local workflow state of each issue.
const STATE_FOLDER: &str = ".agents/issues";

/// The folder, inside an issue's state folder, that holds the counts of refusals.
const REFUSALS_FOLDER: &str = "reply-guard";

/// The repository whose top-level directory is `root`.
#[derive(Debug)]
pub struct FileWorkspace {
    root: PathBuf,
}

impl FileWorkspace {
    /// A workspace for the repository at `root`.
    #[must_use]
    pub const fn new(root: PathBuf) -> Self {
        Self { root }
    }

    /// The file that holds the count of one request: the run and the reply file name, in the
    /// folder of the issue. Both come from outside the repository, so they are reduced to
    /// characters that cannot leave the folder.
    fn refusals_file(&self, key: &RefusalKey) -> PathBuf {
        self.root
            .join(STATE_FOLDER)
            .join(key.issue_number.to_string())
            .join(REFUSALS_FOLDER)
            .join(format!(
                "{}--{}.count",
                safe_component(&key.agent_id),
                safe_component(&key.reply_file_name)
            ))
    }
}

/// The text with every character that is not an ASCII letter, a digit, `.`, `_` or `-` replaced by
/// `_`, and a `_` in front when the result is empty or starts with a dot.
fn safe_component(text: &str) -> String {
    let mut component: String = text
        .chars()
        .map(|character| {
            if character.is_ascii_alphanumeric() || matches!(character, '.' | '_' | '-') {
                character
            } else {
                '_'
            }
        })
        .collect();
    if component.is_empty() || component.starts_with('.') {
        component.insert(0, '_');
    }
    component
}

impl Workspace for FileWorkspace {
    fn specification_exists(&self, folder: &str) -> bool {
        self.root.join(folder).is_dir()
    }

    fn reply_file(&self, path: &str) -> ReplyFile {
        let file = self.root.join(path);
        match fs::symlink_metadata(&file) {
            Err(error) if error.kind() == ErrorKind::NotFound => ReplyFile::Missing,
            Err(error) => ReplyFile::Unreadable(error.to_string()),
            Ok(metadata) if metadata.file_type().is_symlink() => {
                ReplyFile::Unreadable("it is a symbolic link; a reply file must be a regular file".to_owned())
            }
            Ok(metadata) if !metadata.is_file() => ReplyFile::Unreadable("it is not a regular file".to_owned()),
            Ok(_) => match fs::read(&file).map(String::from_utf8) {
                Ok(Ok(text)) => ReplyFile::Text(text),
                Ok(Err(_)) => ReplyFile::Unreadable("it is not valid UTF-8".to_owned()),
                Err(error) => ReplyFile::Unreadable(error.to_string()),
            },
        }
    }

    fn refusals(&self, key: &RefusalKey) -> u8 {
        fs::read_to_string(self.refusals_file(key))
            .ok()
            .and_then(|text| text.trim().parse().ok())
            .unwrap_or(0)
    }

    fn set_refusals(&mut self, key: &RefusalKey, count: u8) -> Result<(), String> {
        let file = self.refusals_file(key);
        write_count(&file, count).map_err(|error| format!("{}: {error}", display_relative(&self.root, &file)))
    }

    fn clear_refusals(&mut self, key: &RefusalKey) {
        // A count that is already gone is what this wants; any other failure leaves a count that the
        // next valid or refused attempt overwrites.
        fs::remove_file(self.refusals_file(key)).ok();
    }
}

fn write_count(file: &Path, count: u8) -> std::io::Result<()> {
    if let Some(folder) = file.parent() {
        fs::create_dir_all(folder)?;
    }
    fs::write(file, format!("{count}\n"))
}

/// The path relative to the root, for a message; the path itself when it is not under it.
fn display_relative(root: &Path, file: &Path) -> String {
    file.strip_prefix(root).unwrap_or(file).display().to_string()
}

#[cfg(test)]
mod tests {
    use std::fs;

    use rstest::rstest;
    use tempfile::TempDir;

    use super::{FileWorkspace, safe_component};
    use crate::guard::{RefusalKey, ReplyFile, Workspace};

    /// The issue number of the keys below; the expected paths in the tests spell it out.
    const ISSUE_NUMBER: u64 = 2904;

    fn key(agent_id: &str, reply_file_name: &str) -> RefusalKey {
        RefusalKey {
            issue_number: ISSUE_NUMBER,
            agent_id: agent_id.to_owned(),
            reply_file_name: reply_file_name.to_owned(),
        }
    }

    /// A workspace in a disposable folder; `TempDir` removes it on drop.
    struct Disposable {
        directory: TempDir,
        workspace: FileWorkspace,
    }

    impl Disposable {
        fn new() -> Self {
            let directory = TempDir::new().unwrap();
            let workspace = FileWorkspace::new(directory.path().to_path_buf());
            Self { directory, workspace }
        }

        fn write(&self, path: &str, content: &[u8]) {
            let file = self.directory.path().join(path);
            fs::create_dir_all(file.parent().unwrap()).unwrap();
            fs::write(file, content).unwrap();
        }
    }

    #[test]
    fn it_should_find_the_specification_when_its_folder_exists() {
        // Arrange
        let disposable = Disposable::new();
        fs::create_dir_all(disposable.directory.path().join("docs/issues/open/2904-slug")).unwrap();

        // Act
        let exists = disposable.workspace.specification_exists("docs/issues/open/2904-slug");

        // Assert
        assert!(exists);
    }

    #[test]
    fn it_should_not_find_the_specification_when_nothing_is_at_its_path() {
        // Arrange
        let disposable = Disposable::new();

        // Act
        let exists = disposable.workspace.specification_exists("docs/issues/open/2904-slug");

        // Assert
        assert!(!exists);
    }

    #[test]
    fn it_should_not_take_a_file_for_the_specification_folder() {
        // Arrange
        let disposable = Disposable::new();
        disposable.write("docs/issues/open/2904-slug", b"not a folder");

        // Act
        let exists = disposable.workspace.specification_exists("docs/issues/open/2904-slug");

        // Assert
        assert!(!exists, "a file at the path is not a specification folder");
    }

    #[test]
    fn it_should_read_the_text_of_the_reply_file() {
        // Arrange
        let disposable = Disposable::new();
        disposable.write("docs/reply.json", b"{\"a\": 1}");

        // Act
        let file = disposable.workspace.reply_file("docs/reply.json");

        // Assert
        assert_eq!(file, ReplyFile::Text("{\"a\": 1}".to_owned()));
    }

    #[test]
    fn it_should_report_the_reply_file_as_missing_when_nothing_is_at_its_path() {
        // Arrange
        let disposable = Disposable::new();

        // Act
        let file = disposable.workspace.reply_file("docs/reply.json");

        // Assert
        assert_eq!(file, ReplyFile::Missing);
    }

    #[test]
    fn it_should_report_the_reason_when_the_reply_file_is_not_utf8() {
        // Arrange
        let disposable = Disposable::new();
        disposable.write("docs/reply.json", b"\xff\xfe");

        // Act
        let file = disposable.workspace.reply_file("docs/reply.json");

        // Assert
        assert_eq!(file, ReplyFile::Unreadable("it is not valid UTF-8".to_owned()));
    }

    #[test]
    fn it_should_report_the_reason_when_the_reply_file_is_a_folder() {
        // Arrange
        let disposable = Disposable::new();
        fs::create_dir_all(disposable.directory.path().join("docs/reply.json")).unwrap();

        // Act
        let file = disposable.workspace.reply_file("docs/reply.json");

        // Assert
        assert_eq!(file, ReplyFile::Unreadable("it is not a regular file".to_owned()));
    }

    #[cfg(unix)]
    #[test]
    fn it_should_report_the_reason_when_the_reply_file_is_a_symbolic_link() {
        // Arrange
        let disposable = Disposable::new();
        disposable.write("docs/target.json", b"{}");
        std::os::unix::fs::symlink(
            disposable.directory.path().join("docs/target.json"),
            disposable.directory.path().join("docs/reply.json"),
        )
        .unwrap();

        // Act
        let file = disposable.workspace.reply_file("docs/reply.json");

        // Assert
        assert_eq!(
            file,
            ReplyFile::Unreadable("it is a symbolic link; a reply file must be a regular file".to_owned())
        );
    }

    #[test]
    fn it_should_count_no_refusal_when_none_was_kept() {
        // Arrange
        let disposable = Disposable::new();

        // Act
        let refusals = disposable.workspace.refusals(&key("agent", "reply.json"));

        // Assert
        assert_eq!(refusals, 0);
    }

    #[test]
    fn it_should_read_back_the_count_that_was_kept_for_the_same_request() {
        // Arrange
        let mut disposable = Disposable::new();
        disposable.workspace.set_refusals(&key("agent", "reply.json"), 2).unwrap();

        // Act
        let refusals = disposable.workspace.refusals(&key("agent", "reply.json"));

        // Assert
        assert_eq!(refusals, 2);
    }

    #[test]
    fn it_should_replace_the_count_that_was_kept_for_the_same_request() {
        // Arrange
        let mut disposable = Disposable::new();
        disposable.workspace.set_refusals(&key("agent", "reply.json"), 1).unwrap();

        // Act
        disposable.workspace.set_refusals(&key("agent", "reply.json"), 2).unwrap();

        // Assert
        let file = disposable
            .directory
            .path()
            .join(".agents/issues/2904/reply-guard/agent--reply.json.count");
        assert_eq!(
            fs::read_to_string(file).unwrap(),
            "2\n",
            "the second count must replace the first, not follow it"
        );
    }

    #[rstest]
    #[case::another_run(key("other", "reply.json"))]
    #[case::another_reply_file(key("agent", "other.json"))]
    #[case::another_issue(RefusalKey { issue_number: 2905, agent_id: "agent".to_owned(), reply_file_name: "reply.json".to_owned() })]
    fn it_should_count_no_refusal_for_another_request_than_the_one_that_was_kept(#[case] other: RefusalKey) {
        // Arrange
        let mut disposable = Disposable::new();
        disposable.workspace.set_refusals(&key("agent", "reply.json"), 3).unwrap();

        // Act
        let refusals = disposable.workspace.refusals(&other);

        // Assert
        assert_eq!(refusals, 0);
    }

    #[test]
    fn it_should_keep_the_count_in_the_reply_guard_folder_of_the_issue() {
        // Arrange
        let mut disposable = Disposable::new();

        // Act
        disposable
            .workspace
            .set_refusals(&key("a423", "task-reviewer-t5-r1.json"), 1)
            .unwrap();

        // Assert
        let file = disposable
            .directory
            .path()
            .join(".agents/issues/2904/reply-guard/a423--task-reviewer-t5-r1.json.count");
        assert_eq!(fs::read_to_string(file).unwrap(), "1\n");
    }

    #[rstest]
    #[case::not_a_number("many")]
    #[case::negative("-1")]
    #[case::too_large("256")]
    #[case::empty("")]
    fn it_should_count_no_refusal_when_the_kept_count_is_not_a_number_of_refusals(#[case] content: &str) {
        // Arrange
        let disposable = Disposable::new();
        disposable.write(".agents/issues/2904/reply-guard/agent--reply.json.count", content.as_bytes());

        // Act
        let refusals = disposable.workspace.refusals(&key("agent", "reply.json"));

        // Assert
        assert_eq!(refusals, 0);
    }

    #[test]
    fn it_should_forget_the_count_when_it_is_cleared() {
        // Arrange
        let mut disposable = Disposable::new();
        disposable.workspace.set_refusals(&key("agent", "reply.json"), 2).unwrap();

        // Act
        disposable.workspace.clear_refusals(&key("agent", "reply.json"));

        // Assert
        assert_eq!(disposable.workspace.refusals(&key("agent", "reply.json")), 0);
    }

    #[test]
    fn it_should_clear_a_count_that_was_never_kept_without_failing() {
        // Arrange
        let mut disposable = Disposable::new();

        // Act
        disposable.workspace.clear_refusals(&key("agent", "reply.json"));

        // Assert: the test exists for the absence of a panic.
        assert_eq!(
            disposable.workspace.refusals(&key("agent", "reply.json")),
            0,
            "clearing a count that was never kept must leave no count"
        );
    }

    #[test]
    fn it_should_report_the_file_and_the_reason_when_the_count_cannot_be_kept() {
        // Arrange: a file where the state folder must be.
        let mut disposable = Disposable::new();
        disposable.write(".agents", b"not a folder");

        // Act
        let result = disposable.workspace.set_refusals(&key("agent", "reply.json"), 1);

        // Assert
        let reason = result.unwrap_err();
        let prefix = ".agents/issues/2904/reply-guard/agent--reply.json.count: ";
        assert!(
            reason.starts_with(prefix),
            "the reason must name the file relative to the repository: {reason}"
        );
        assert!(reason.len() > prefix.len(), "the reason must say what failed: {reason}");
    }

    #[rstest]
    #[case::letters_and_digits("a423cd4370c2ebae8", "a423cd4370c2ebae8")]
    #[case::dots_dashes_and_underscores("task-reviewer_t5-r1.json", "task-reviewer_t5-r1.json")]
    #[case::slash("../../etc/passwd", "_.._.._etc_passwd")]
    #[case::backslash("a\\b", "a_b")]
    #[case::space_and_colon("a b:c", "a_b_c")]
    #[case::non_ascii("é", "_")]
    #[case::empty("", "_")]
    #[case::leading_dot(".hidden", "_.hidden")]
    #[case::only_dots("..", "_..")]
    fn it_should_reduce_a_name_to_characters_that_cannot_leave_the_folder(#[case] name: &str, #[case] expected: &str) {
        // Act
        let component = safe_component(name);

        // Assert
        assert_eq!(component, expected);
    }

    #[test]
    fn it_should_keep_the_count_inside_the_folder_when_the_run_or_the_file_name_is_a_path() {
        // Arrange
        let mut disposable = Disposable::new();

        // Act
        disposable
            .workspace
            .set_refusals(&key("../../escape", "../../../x.json"), 1)
            .unwrap();

        // Assert
        let folder = disposable.directory.path().join(".agents/issues/2904/reply-guard");
        let names: Vec<_> = fs::read_dir(folder)
            .unwrap()
            .map(|entry| entry.unwrap().file_name().to_string_lossy().into_owned())
            .collect();
        assert_eq!(
            names,
            ["_.._.._escape--_.._.._.._x.json.count"],
            "the count file must be the only one in the folder, named with the path reduced to safe characters"
        );
    }
}
