//! Command-line arguments of the `ci-gate` binary.

use crate::github::PullRequestRef;

/// The usage text carried by a usage error.
pub const USAGE: &str = "usage: ci-gate classify --base-branch <branch>  (changed paths on stdin, NUL-separated, as `git diff --name-only -z` prints them); ci-gate verdict [--override-actor <login>]  (JSON on stdin: {\"classification\": <the classify output>, \"results\": {<group>: success|failure|cancelled|skipped}}); ci-gate github-verdict --repo <owner/name> --pull-request <number> --head-sha <40-hex-commit-id> --classification <file> [--gh <program>]  (reads the results and the override from GitHub through `gh`)";

/// A parsed command.
#[derive(Debug, Eq, PartialEq)]
pub enum Command {
    Classify(ClassifyArguments),
    Verdict(VerdictArguments),
    GithubVerdict(GithubVerdictArguments),
}

/// The arguments of `classify`.
#[derive(Debug, Eq, PartialEq)]
pub struct ClassifyArguments {
    /// The branch the pull request merges into.
    pub base_branch: String,
}

/// The arguments of `verdict`.
#[derive(Debug, Eq, PartialEq)]
pub struct VerdictArguments {
    /// The login of the actor whose maintainer override applies, if there is one.
    pub override_actor: Option<String>,
}

/// The arguments of `github-verdict`.
#[derive(Debug, Eq, PartialEq)]
pub struct GithubVerdictArguments {
    pub pull_request: PullRequestRef,
    /// The file that holds the output of `classify`.
    pub classification_path: String,
    /// The program that calls the GitHub API, `gh` unless a test says otherwise.
    pub gh_program: String,
}

/// Parses the arguments that follow the program name.
///
/// # Errors
///
/// Returns a message naming the problem when the subcommand is missing or unknown, when a required
/// option is missing, repeated, or has no value, or when an argument is not recognised.
pub fn parse_arguments<S: AsRef<str>>(arguments: &[S]) -> Result<Command, String> {
    let mut arguments = arguments.iter().map(AsRef::as_ref);

    match arguments.next() {
        Some("classify") => parse_classify(arguments).map(Command::Classify),
        Some("verdict") => parse_verdict(arguments).map(Command::Verdict),
        Some("github-verdict") => parse_github_verdict(arguments).map(Command::GithubVerdict),
        Some(other) => Err(format!("unknown subcommand `{other}`")),
        None => Err("missing subcommand".to_string()),
    }
}

fn parse_classify<'a>(mut arguments: impl Iterator<Item = &'a str>) -> Result<ClassifyArguments, String> {
    let mut base_branch: Option<String> = None;

    while let Some(argument) = arguments.next() {
        if argument != "--base-branch" {
            return Err(format!("unknown argument `{argument}`"));
        }
        let value = option_value(&mut arguments, "--base-branch", "a branch name")?;
        if base_branch.replace(value.to_string()).is_some() {
            return Err("`--base-branch` was given more than once".to_string());
        }
    }

    base_branch
        .map(|base_branch| ClassifyArguments { base_branch })
        .ok_or_else(|| "`--base-branch` is required".to_string())
}

fn parse_verdict<'a>(mut arguments: impl Iterator<Item = &'a str>) -> Result<VerdictArguments, String> {
    let mut override_actor: Option<String> = None;

    while let Some(argument) = arguments.next() {
        if argument != "--override-actor" {
            return Err(format!("unknown argument `{argument}`"));
        }
        let value = option_value(&mut arguments, "--override-actor", "a login")?;
        if override_actor.replace(value.to_string()).is_some() {
            return Err("`--override-actor` was given more than once".to_string());
        }
    }

    Ok(VerdictArguments { override_actor })
}

fn parse_github_verdict<'a>(mut arguments: impl Iterator<Item = &'a str>) -> Result<GithubVerdictArguments, String> {
    let mut repository: Option<&str> = None;
    let mut number: Option<&str> = None;
    let mut head_sha: Option<&str> = None;
    let mut classification_path: Option<&str> = None;
    let mut gh_program: Option<&str> = None;

    while let Some(argument) = arguments.next() {
        let (slot, what) = match argument {
            "--repo" => (&mut repository, "a repository of the form `owner/name`"),
            "--pull-request" => (&mut number, "a pull request number"),
            "--head-sha" => (&mut head_sha, "a commit id of 40 hexadecimal digits"),
            "--classification" => (&mut classification_path, "a file name"),
            "--gh" => (&mut gh_program, "a program name"),
            other => return Err(format!("unknown argument `{other}`")),
        };
        let value = option_value(&mut arguments, argument, what)?;
        if slot.replace(value).is_some() {
            return Err(format!("`{argument}` was given more than once"));
        }
    }

    let required = |value: Option<&'a str>, option: &str| value.ok_or_else(|| format!("`{option}` is required"));
    let repository = required(repository, "--repo")?;
    let number = required(number, "--pull-request")?;
    let number: u64 = number
        .parse()
        .map_err(|_| format!("`{number}` is not a pull request number"))?;

    Ok(GithubVerdictArguments {
        pull_request: PullRequestRef::new(repository, number, required(head_sha, "--head-sha")?)?,
        classification_path: required(classification_path, "--classification")?.to_string(),
        gh_program: gh_program.unwrap_or("gh").to_string(),
    })
}

fn option_value<'a>(arguments: &mut impl Iterator<Item = &'a str>, option: &str, what: &str) -> Result<&'a str, String> {
    arguments
        .next()
        .filter(|value| !value.is_empty() && !value.starts_with("--"))
        .ok_or_else(|| format!("`{option}` needs {what}"))
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[test]
    fn it_should_read_the_base_branch_of_the_classify_command() {
        // Arrange
        let arguments = ["classify", "--base-branch", "develop"];

        // Act
        let parsed = parse_arguments(&arguments);

        // Assert
        assert_eq!(
            parsed,
            Ok(Command::Classify(ClassifyArguments {
                base_branch: "develop".to_string()
            }))
        );
    }

    #[rstest]
    #[case::without_an_override(&["verdict"], None)]
    #[case::with_an_override(&["verdict", "--override-actor", "josecelano"], Some("josecelano"))]
    fn it_should_read_the_optional_override_actor_of_the_verdict_command(
        #[case] arguments: &[&str],
        #[case] expected_actor: Option<&str>,
    ) {
        // Act
        let parsed = parse_arguments(arguments);

        // Assert
        assert_eq!(
            parsed,
            Ok(Command::Verdict(VerdictArguments {
                override_actor: expected_actor.map(str::to_string)
            })),
            "arguments: {arguments:?}"
        );
    }

    #[rstest]
    #[case::no_arguments(&[], "missing subcommand")]
    #[case::unknown_subcommand(&["verify"], "unknown subcommand `verify`")]
    #[case::missing_base_branch(&["classify"], "`--base-branch` is required")]
    #[case::base_branch_without_value(&["classify", "--base-branch"], "`--base-branch` needs a branch name")]
    #[case::base_branch_followed_by_a_flag(&["classify", "--base-branch", "--other"], "`--base-branch` needs a branch name")]
    #[case::base_branch_given_twice(
        &["classify", "--base-branch", "develop", "--base-branch", "main"],
        "`--base-branch` was given more than once"
    )]
    #[case::unknown_argument(&["classify", "--head", "x"], "unknown argument `--head`")]
    #[case::the_override_option_belongs_to_the_verdict_command(
        &["classify", "--base-branch", "develop", "--override-actor", "x"],
        "unknown argument `--override-actor`"
    )]
    #[case::the_base_branch_option_belongs_to_the_classify_command(
        &["verdict", "--base-branch", "develop"],
        "unknown argument `--base-branch`"
    )]
    #[case::override_actor_without_value(&["verdict", "--override-actor"], "`--override-actor` needs a login")]
    #[case::override_actor_followed_by_a_flag(
        &["verdict", "--override-actor", "--other"],
        "`--override-actor` needs a login"
    )]
    #[case::empty_override_actor(&["verdict", "--override-actor", ""], "`--override-actor` needs a login")]
    #[case::override_actor_given_twice(
        &["verdict", "--override-actor", "a", "--override-actor", "b"],
        "`--override-actor` was given more than once"
    )]
    fn it_should_reject_invalid_arguments_with_a_message_naming_the_problem(
        #[case] arguments: &[&str],
        #[case] expected_message: &str,
    ) {
        // Act
        let parsed = parse_arguments(arguments);

        // Assert
        assert_eq!(parsed, Err(expected_message.to_string()), "arguments: {arguments:?}");
    }

    const SHA: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";

    #[test]
    fn it_should_tell_in_the_usage_that_the_head_commit_id_has_forty_hexadecimal_digits() {
        // Arrange
        let required_form = "--head-sha <40-hex-commit-id>";

        // Act
        let usage = USAGE;

        // Assert
        assert!(
            usage.contains(required_form),
            "the usage must show the form that `PullRequestRef::new` requires, so a user can correct it without trying: {usage}"
        );
    }

    #[rstest]
    #[case::the_default_program(
        &["github-verdict", "--repo", "owner/repo", "--pull-request", "7", "--head-sha", SHA, "--classification", "c.json"],
        "gh"
    )]
    #[case::another_program(
        &["github-verdict", "--repo", "owner/repo", "--pull-request", "7", "--head-sha", SHA, "--classification", "c.json", "--gh", "/tmp/fake-gh"],
        "/tmp/fake-gh"
    )]
    #[case::in_another_order(
        &["github-verdict", "--classification", "c.json", "--head-sha", SHA, "--pull-request", "7", "--repo", "owner/repo"],
        "gh"
    )]
    fn it_should_read_the_arguments_of_the_github_verdict_command(#[case] arguments: &[&str], #[case] expected_program: &str) {
        // Act
        let parsed = parse_arguments(arguments);

        // Assert
        assert_eq!(
            parsed,
            Ok(Command::GithubVerdict(GithubVerdictArguments {
                pull_request: PullRequestRef::new("owner/repo", 7, SHA).expect("valid"),
                classification_path: "c.json".to_string(),
                gh_program: expected_program.to_string(),
            })),
            "arguments: {arguments:?}"
        );
    }

    #[rstest]
    #[case::no_repo(
        &["github-verdict", "--pull-request", "7", "--head-sha", SHA, "--classification", "c.json"],
        "`--repo` is required"
    )]
    #[case::no_pull_request(
        &["github-verdict", "--repo", "owner/repo", "--head-sha", SHA, "--classification", "c.json"],
        "`--pull-request` is required"
    )]
    #[case::no_head_sha(
        &["github-verdict", "--repo", "owner/repo", "--pull-request", "7", "--classification", "c.json"],
        "`--head-sha` is required"
    )]
    #[case::no_classification(
        &["github-verdict", "--repo", "owner/repo", "--pull-request", "7", "--head-sha", SHA],
        "`--classification` is required"
    )]
    #[case::a_pull_request_that_is_not_a_number(
        &["github-verdict", "--repo", "owner/repo", "--pull-request", "seven", "--head-sha", SHA, "--classification", "c.json"],
        "`seven` is not a pull request number"
    )]
    #[case::a_repository_that_is_not_owner_and_name(
        &["github-verdict", "--repo", "repo", "--pull-request", "7", "--head-sha", SHA, "--classification", "c.json"],
        "`repo` is not a repository of the form `owner/name`"
    )]
    #[case::a_short_commit_id(
        &["github-verdict", "--repo", "owner/repo", "--pull-request", "7", "--head-sha", "abc", "--classification", "c.json"],
        "`abc` is not a commit id of 40 hexadecimal digits"
    )]
    #[case::an_option_given_twice(
        &["github-verdict", "--repo", "a/b", "--repo", "c/d", "--pull-request", "7", "--head-sha", SHA, "--classification", "c.json"],
        "`--repo` was given more than once"
    )]
    #[case::an_option_without_a_value(
        &["github-verdict", "--repo", "owner/repo", "--pull-request"],
        "`--pull-request` needs a pull request number"
    )]
    #[case::a_head_commit_id_without_a_value(
        &["github-verdict", "--repo", "owner/repo", "--pull-request", "7", "--head-sha"],
        "`--head-sha` needs a commit id of 40 hexadecimal digits"
    )]
    #[case::an_unknown_option(
        &["github-verdict", "--base-branch", "develop"],
        "unknown argument `--base-branch`"
    )]
    fn it_should_reject_invalid_arguments_of_the_github_verdict_command_naming_the_problem(
        #[case] arguments: &[&str],
        #[case] expected_message: &str,
    ) {
        // Act
        let parsed = parse_arguments(arguments);

        // Assert
        assert_eq!(parsed, Err(expected_message.to_string()), "arguments: {arguments:?}");
    }
}
