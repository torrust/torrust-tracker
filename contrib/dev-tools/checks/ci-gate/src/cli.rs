//! Command-line arguments of the `ci-gate` binary.

/// The usage text carried by a usage error.
pub const USAGE: &str = "usage: ci-gate classify --base-branch <branch>  (changed paths on stdin, one per line)";

/// A parsed `classify` command.
#[derive(Debug, Eq, PartialEq)]
pub struct ClassifyArguments {
    /// The branch the pull request merges into.
    pub base_branch: String,
}

/// Parses the arguments that follow the program name.
///
/// # Errors
///
/// Returns a message naming the problem when the subcommand is missing or unknown, when
/// `--base-branch` is missing, repeated, or has no value, or when an argument is not recognised.
pub fn parse_arguments<S: AsRef<str>>(arguments: &[S]) -> Result<ClassifyArguments, String> {
    let mut arguments = arguments.iter().map(AsRef::as_ref);

    match arguments.next() {
        Some("classify") => {}
        Some(other) => return Err(format!("unknown subcommand `{other}`")),
        None => return Err("missing subcommand".to_string()),
    }

    let mut base_branch: Option<String> = None;

    while let Some(argument) = arguments.next() {
        if argument != "--base-branch" {
            return Err(format!("unknown argument `{argument}`"));
        }
        let value = arguments
            .next()
            .filter(|value| !value.is_empty() && !value.starts_with("--"))
            .ok_or_else(|| "`--base-branch` needs a branch name".to_string())?;
        if base_branch.replace(value.to_string()).is_some() {
            return Err("`--base-branch` was given more than once".to_string());
        }
    }

    base_branch
        .map(|base_branch| ClassifyArguments { base_branch })
        .ok_or_else(|| "`--base-branch` is required".to_string())
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
            Ok(ClassifyArguments {
                base_branch: "develop".to_string()
            })
        );
    }

    #[rstest]
    #[case::no_arguments(&[], "missing subcommand")]
    #[case::unknown_subcommand(&["verdict"], "unknown subcommand `verdict`")]
    #[case::missing_base_branch(&["classify"], "`--base-branch` is required")]
    #[case::base_branch_without_value(&["classify", "--base-branch"], "`--base-branch` needs a branch name")]
    #[case::base_branch_followed_by_a_flag(&["classify", "--base-branch", "--other"], "`--base-branch` needs a branch name")]
    #[case::base_branch_given_twice(
        &["classify", "--base-branch", "develop", "--base-branch", "main"],
        "`--base-branch` was given more than once"
    )]
    #[case::unknown_argument(&["classify", "--head", "x"], "unknown argument `--head`")]
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
