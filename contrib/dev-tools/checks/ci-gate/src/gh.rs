//! The GitHub API through the `gh` command, which holds the token.

use std::process::Command;

use serde_json::Value;

use crate::github::GithubApi;

/// Reads the GitHub REST API by running `gh api --paginate`.
///
/// `gh` reads its token from the environment (`GH_TOKEN`), as it does in a workflow step.
pub struct Gh {
    program: String,
}

impl Gh {
    /// The program is `gh`, or the path of a program that behaves like it.
    #[must_use]
    pub fn new(program: &str) -> Self {
        Self {
            program: program.to_string(),
        }
    }
}

impl GithubApi for Gh {
    fn pages(&self, path: &str) -> Result<Vec<Value>, String> {
        let output = Command::new(&self.program)
            .args(["api", "--paginate", path])
            .output()
            .map_err(|error| format!("cannot run `{}`: {error}", self.program))?;

        if !output.status.success() {
            return Err(failure_message(
                &self.program,
                path,
                &output.status.to_string(),
                &output.stderr,
            ));
        }

        parse_pages(path, &output.stdout)
    }
}

/// The most characters of what `gh` printed on stderr that an error keeps.
const STDERR_LIMIT: usize = 300;

/// The error of a request that `gh` refused: the invocation as it was run, the status, and the start of what
/// `gh` printed on stderr, marked when it was cut.
fn failure_message(program: &str, path: &str, status: &str, stderr: &[u8]) -> String {
    let stderr = String::from_utf8_lossy(stderr);
    let stderr = stderr.trim();
    let kept: String = stderr.chars().take(STDERR_LIMIT).collect();
    let cut = if stderr.chars().count() > STDERR_LIMIT {
        "… (truncated)"
    } else {
        ""
    };
    format!("`{program} api --paginate {path}` failed with {status}: {kept}{cut}")
}

/// `--paginate` prints one JSON document per page, one after the other.
fn parse_pages(path: &str, stdout: &[u8]) -> Result<Vec<Value>, String> {
    serde_json::Deserializer::from_slice(stdout)
        .into_iter::<Value>()
        .collect::<Result<Vec<_>, _>>()
        .map_err(|error| format!("the response of `{path}` is not JSON: {error}"))
}

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use serde_json::json;

    use super::*;

    #[rstest]
    #[case::one_page(r#"{"a":1}"#, vec![json!({"a": 1})])]
    #[case::two_pages_back_to_back(r#"{"a":1}{"a":2}"#, vec![json!({"a": 1}), json!({"a": 2})])]
    #[case::two_pages_on_separate_lines("{\"a\":1}\n{\"a\":2}\n", vec![json!({"a": 1}), json!({"a": 2})])]
    #[case::pages_that_are_arrays("[1,2][3]", vec![json!([1, 2]), json!([3])])]
    #[case::no_output_is_no_page("", vec![])]
    fn it_should_read_one_json_document_per_page_of_the_output_of_gh(#[case] output: &str, #[case] expected: Vec<Value>) {
        // Act
        let pages = parse_pages("repos/owner/repo/x", output.as_bytes());

        // Assert
        assert_eq!(pages, Ok(expected), "the output {output:?}");
    }

    #[rstest]
    #[case::not_json("not json")]
    #[case::a_second_page_that_is_cut_off(r#"{"a":1}{"a":"#)]
    fn it_should_name_the_path_when_the_output_of_gh_is_not_json(#[case] output: &str) {
        // Act
        let pages = parse_pages("repos/owner/repo/x", output.as_bytes());

        // Assert
        let error = pages.expect_err("the output is not valid JSON");
        assert!(
            error.contains("`repos/owner/repo/x`") && error.contains("is not JSON"),
            "the error should name the path and say that the response is not JSON, but was: {error}"
        );
    }

    #[test]
    fn it_should_name_the_real_invocation_the_status_and_the_start_of_the_error_when_a_request_fails() {
        // Arrange
        let stderr = format!("gh: HTTP 403 {}", "x".repeat(1000));

        // Act
        let message = failure_message("gh", "repos/owner/repo/x", "exit status: 1", stderr.as_bytes());

        // Assert
        assert!(
            message.starts_with("`gh api --paginate repos/owner/repo/x` failed with exit status: 1: gh: HTTP 403 "),
            "the message should name the invocation with --paginate, the status, and the error, but was: {message}"
        );
        assert!(
            message.len() < 200 + "gh: HTTP 403 ".len() + 300 + " (truncated)".len(),
            "the part of the error that is kept is limited to 300 characters, but the message has {} bytes",
            message.len()
        );
    }

    #[test]
    fn it_should_say_that_the_error_was_cut_when_stderr_is_longer_than_the_limit() {
        // Arrange
        let stderr = "x".repeat(301);

        // Act
        let message = failure_message("gh", "repos/owner/repo/x", "exit status: 1", stderr.as_bytes());

        // Assert
        assert!(
            message.ends_with("… (truncated)"),
            "a reader must not take a cut error for a complete one, but the message was: {message}"
        );
    }

    #[rstest]
    #[case::exactly_at_the_limit(300)]
    #[case::shorter_than_the_limit(10)]
    fn it_should_not_say_that_the_error_was_cut_when_stderr_fits_the_limit(#[case] length: usize) {
        // Arrange
        let stderr = "x".repeat(length);

        // Act
        let message = failure_message("gh", "repos/owner/repo/x", "exit status: 1", stderr.as_bytes());

        // Assert
        assert!(
            !message.contains("truncated"),
            "an error that is complete must not be marked as cut, but the message was: {message}"
        );
        assert!(message.ends_with(&stderr), "the whole error should be kept: {message}");
    }
}
