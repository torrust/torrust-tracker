//! Finds enabled features of workspace packages in the output of `cargo tree`.
//!
//! The input is the output of
//! `cargo tree -p <shipped package> -e normal,build --prefix none -f "{p}|{f}"`: one package per
//! line, `<name> v<version>[ (<path>)][ (proc-macro)]|<enabled features>[ (*)]`. That command resolves
//! features for the selected package the way `cargo build -p <package>` does, without the
//! dev-dependencies, which `cargo metadata` cannot do.

use std::collections::BTreeSet;

/// A workspace package that has the checked feature enabled.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Violation {
    /// The package name.
    pub package: String,
    /// The package version.
    pub version: String,
    /// The enabled feature that is not allowed.
    pub feature: String,
}

/// Returns the workspace packages of `tree` that have `feature` enabled, each once and in order.
///
/// A package is a workspace package when its line carries a local path. Registry and Git
/// dependencies are not checked: a third-party crate's feature is outside the repository's rules.
#[must_use]
pub fn workspace_packages_with_feature(tree: &str, feature: &str) -> Vec<Violation> {
    let violations: BTreeSet<Violation> = tree
        .lines()
        .filter_map(|line| {
            let (package, features) = line.split_once('|')?;
            let (name, version, location) = split_package(package)?;
            (is_local_path(location) && has_feature(features, feature)).then(|| Violation {
                package: name.to_owned(),
                version: version.to_owned(),
                feature: feature.to_owned(),
            })
        })
        .collect();

    violations.into_iter().collect()
}

/// Splits `<name> v<version>[ (<location>)][ (proc-macro)]` into its name, version and location.
///
/// The location is the text inside the parentheses that follow the version, or an empty string when
/// the line has none (a registry package) or only the `(proc-macro)` marker.
fn split_package(package: &str) -> Option<(&str, &str, &str)> {
    let (name, rest) = package.trim().split_once(' ')?;
    let (version, rest) = rest.split_once(' ').unwrap_or((rest, ""));
    let rest = rest.trim_end().strip_suffix("(proc-macro)").unwrap_or(rest).trim();
    let location = rest.strip_prefix('(').and_then(|rest| rest.strip_suffix(')')).unwrap_or("");

    version.starts_with('v').then_some((name, version, location))
}

/// Whether `location` is a file system path, as opposed to the URL of a Git dependency.
fn is_local_path(location: &str) -> bool {
    let mut characters = location.chars();
    let windows_drive = matches!(
        (characters.next(), characters.next(), characters.next()),
        (Some(letter), Some(':'), Some('\\' | '/')) if letter.is_ascii_alphabetic()
    );

    location.starts_with('/') || windows_drive
}

/// Whether `feature` is one of the comma-separated features that `cargo tree` printed for a package.
fn has_feature(features: &str, feature: &str) -> bool {
    features
        .trim()
        .trim_end_matches("(*)")
        .split(',')
        .any(|enabled| enabled.trim() == feature)
}

#[cfg(test)]
mod tests {
    use super::{Violation, workspace_packages_with_feature};

    // Output of `cargo tree -p b -e normal,build --prefix none -f "{p}|{f}"` on a scratch workspace where
    // `b` depends on `a` and on `c`, and only `c` enables `a`'s `testing` feature.
    const TRANSITIVE_TREE: &str = "\
b v0.1.0 (/scratch/b)|
a v0.1.0 (/scratch/a)|testing
c v0.1.0 (/scratch/c)|
a v0.1.0 (/scratch/a)|testing
";

    // The first lines of the same command on this repository, with its path replaced by `/repo`.
    const REPOSITORY_TREE: &str = "\
torrust-tracker v3.0.0-develop (/repo)|
anyhow v1.0.104|default,std
axum-server v0.8.0|arc-swap,default,rustls,rustls-pki-types,tls-rustls-no-provider,tokio-rustls
arc-swap v1.9.3|
rustversion v1.0.23 (proc-macro)|
serde_derive v1.0.229 (proc-macro)|default
proc-macro2 v1.0.107|default,proc-macro
proc-macro2 v1.0.107|default,proc-macro (*)
";

    fn violation(package: &str, version: &str) -> Violation {
        Violation {
            package: package.to_owned(),
            version: version.to_owned(),
            feature: "testing".to_owned(),
        }
    }

    #[test]
    fn it_should_report_a_workspace_package_whose_testing_feature_a_dependency_enables_once() {
        // Arrange: `a` appears twice in the tree with `testing` on, because `c` enables it.

        // Act
        let violations = workspace_packages_with_feature(TRANSITIVE_TREE, "testing");

        // Assert
        assert_eq!(violations, vec![violation("a", "v0.1.0")]);
    }

    #[test]
    fn it_should_report_nothing_when_no_package_has_the_feature_enabled() {
        // Arrange: a real tree whose lines carry `(proc-macro)` and `(*)` markers and no `testing` feature.

        // Act
        let violations = workspace_packages_with_feature(REPOSITORY_TREE, "testing");

        // Assert
        assert_eq!(violations, vec![]);
    }

    #[test]
    fn it_should_report_the_shipped_package_itself_when_it_enables_the_feature() {
        // Arrange: the root line of the tree is the shipped package.
        let tree = "shipped v1.2.3 (/repo/shipped)|testing\n";

        // Act
        let violations = workspace_packages_with_feature(tree, "testing");

        // Assert
        assert_eq!(violations, vec![violation("shipped", "v1.2.3")]);
    }

    #[test]
    fn it_should_ignore_a_registry_package_that_has_a_feature_with_the_same_name() {
        // Arrange: a line in the format of a registry package (no path) with a `testing` feature.
        let tree = "third-party v1.0.0|default,testing\n";

        // Act
        let violations = workspace_packages_with_feature(tree, "testing");

        // Assert
        assert_eq!(violations, vec![]);
    }

    #[test]
    fn it_should_ignore_a_git_package_that_has_a_feature_with_the_same_name() {
        // Arrange: a line in the format of a Git dependency, whose location is a URL and not a path.
        let tree = "forked v0.3.0 (https://example.org/forked.git#0123abc)|testing\n";

        // Act
        let violations = workspace_packages_with_feature(tree, "testing");

        // Assert
        assert_eq!(violations, vec![]);
    }

    #[test]
    fn it_should_match_the_feature_name_exactly() {
        // Arrange: features whose names contain `testing` but are not `testing`.
        let tree = "a v0.1.0 (/scratch/a)|testing-utils,db-testing\n";

        // Act
        let violations = workspace_packages_with_feature(tree, "testing");

        // Assert
        assert_eq!(violations, vec![]);
    }

    #[test]
    fn it_should_read_a_workspace_path_that_contains_spaces() {
        // Arrange: a checkout under a directory with a space in its name.
        let tree = "a v0.1.0 (/home/some user/my repo/a)|testing\n";

        // Act
        let violations = workspace_packages_with_feature(tree, "testing");

        // Assert
        assert_eq!(violations, vec![violation("a", "v0.1.0")]);
    }

    #[test]
    fn it_should_report_a_workspace_procedural_macro_crate_that_has_the_feature_enabled() {
        // Arrange: a workspace crate that is also a procedural macro carries both a path and `(proc-macro)`.
        let tree = "macros v0.1.0 (/repo/macros) (proc-macro)|testing\n";

        // Act
        let violations = workspace_packages_with_feature(tree, "testing");

        // Assert
        assert_eq!(violations, vec![violation("macros", "v0.1.0")]);
    }

    #[test]
    fn it_should_skip_lines_that_are_not_package_lines() {
        // Arrange: blank lines and a line without the `|` separator.
        let tree = "\nnot a package line\n\n";

        // Act
        let violations = workspace_packages_with_feature(tree, "testing");

        // Assert
        assert_eq!(violations, vec![]);
    }
}
