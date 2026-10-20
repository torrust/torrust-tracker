//! Command-line arguments of the `ci-gate` binary.

/// The usage text carried by a usage error.
pub const USAGE: &str = "usage: ci-gate classify --base-branch <branch>  (changed paths on stdin, NUL-separated, as `git diff --name-only -z` prints them); ci-gate verdict [--override-actor <login>]  (JSON on stdin: {\"classification\": <the classify output>, \"results\": {<group>: success|failure|cancelled|skipped}})";

/// A parsed command.
#[derive(Debug, Eq, PartialEq)]
pub enum Command {
    Classify(ClassifyArguments),
    Verdict(VerdictArguments),
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
}
