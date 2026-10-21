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
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!(
                "`{} api --paginate {path}` failed with {}: {}",
                self.program,
                output.status,
                stderr.trim().chars().take(300).collect::<String>()
            ));
        }

        // `--paginate` prints one JSON document per page, one after the other.
        serde_json::Deserializer::from_slice(&output.stdout)
            .into_iter::<Value>()
            .collect::<Result<Vec<_>, _>>()
            .map_err(|error| format!("the response of `{path}` is not JSON: {error}"))
    }
}
