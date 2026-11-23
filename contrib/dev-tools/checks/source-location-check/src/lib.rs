//! Text-based detection of source locations recorded outside test code.
//!
//! The rule (ADR `20261118045443_errors_do_not_record_source_locations`): non-test code does not use
//! `Location::caller()`, `#[track_caller]` or `torrust_located_error`. A text check cannot parse Rust, so
//! test code is recognized by convention, which `rustfmt` keeps regular:
//!
//! - a `#[cfg(test)]` attribute marks the item that follows (a block ends at the first closing brace
//!   with the indentation of the attribute, a `mod name;` declaration ends on its own line);
//! - a file declared by `#[cfg(test)] mod name;` is test code, with everything below that module;
//! - files under a `tests` directory and under `packages/test-helpers/` are test code.
//!
//! Comment lines are ignored. Patterns are built with `concat!` so this crate does not match itself.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

/// The forbidden patterns, each with the rule text shown to the contributor.
pub const FORBIDDEN: [(&str, &str); 4] = [
    (concat!("Location::", "caller"), "records the caller's source location"),
    (
        concat!("track_", "caller"),
        "the track-caller attribute exists to record a source location; allowed only in test code",
    ),
    (
        concat!("torrust_located", "_error"),
        "the located-error crate records source locations",
    ),
    (
        concat!("torrust-located", "-error"),
        "the located-error crate records source locations",
    ),
];

/// Directory prefixes (repo-relative, `/`-separated) that hold test-only code.
pub const TEST_ONLY_PREFIXES: [&str; 1] = ["packages/test-helpers/"];

/// One forbidden use.
#[derive(Debug, PartialEq, Eq)]
pub struct Violation {
    pub line: usize,
    pub pattern: &'static str,
    pub reason: &'static str,
}

/// Returns the forbidden uses in the non-test part of `source`.
#[must_use]
pub fn violations_in(source: &str) -> Vec<Violation> {
    let mut violations = Vec::new();
    let mut lines = source.lines().enumerate().peekable();

    while let Some((index, line)) = lines.next() {
        let trimmed = line.trim_start();

        if trimmed == "#[cfg(test)]" {
            skip_test_item(&mut lines, indentation(line));
            continue;
        }
        if trimmed.starts_with("//") {
            continue;
        }
        for (pattern, reason) in FORBIDDEN {
            if line.contains(pattern) {
                violations.push(Violation {
                    line: index + 1,
                    pattern,
                    reason,
                });
            }
        }
    }

    violations
}

fn indentation(line: &str) -> usize {
    line.len() - line.trim_start().len()
}

/// Consumes the item that follows a `#[cfg(test)]` attribute of the given indentation.
fn skip_test_item<'a>(lines: &mut std::iter::Peekable<impl Iterator<Item = (usize, &'a str)>>, indent: usize) {
    // Further attributes and doc comments belong to the same item.
    while let Some((_, line)) = lines.peek() {
        let trimmed = line.trim_start();
        if trimmed.starts_with("#[") || trimmed.starts_with("//") {
            lines.next();
        } else {
            break;
        }
    }
    let Some((_, first)) = lines.next() else { return };
    let ends_here = first.trim_end().ends_with(';') && !first.contains('{');
    let one_liner = first.contains('{') && first.contains('}') && first.trim_end().ends_with('}');
    if ends_here || one_liner {
        return;
    }
    for (_, line) in lines.by_ref() {
        if indentation(line) == indent && line.trim_start().starts_with('}') {
            return;
        }
    }
}

/// Names declared as `#[cfg(test)] mod name;` in `source`.
#[must_use]
pub fn test_only_modules(source: &str) -> Vec<String> {
    let mut modules = Vec::new();
    let mut after_cfg_test = false;

    for line in source.lines() {
        let trimmed = line.trim();
        if trimmed == "#[cfg(test)]" {
            after_cfg_test = true;
        } else if trimmed.starts_with("#[") {
            // another attribute of the same item
        } else {
            if after_cfg_test {
                let declaration = trimmed
                    .strip_prefix("pub(crate) ")
                    .or_else(|| trimmed.strip_prefix("pub "))
                    .unwrap_or(trimmed);
                if let Some(name) = declaration.strip_prefix("mod ").and_then(|rest| rest.strip_suffix(';')) {
                    modules.push(name.trim().to_owned());
                }
            }
            after_cfg_test = false;
        }
    }

    modules
}

/// The directory in which the child modules of the file at `path` live.
fn module_directory(path: &Path) -> PathBuf {
    let parent = path.parent().unwrap_or_else(|| Path::new(""));
    match path.file_name().and_then(|name| name.to_str()) {
        Some("lib.rs" | "main.rs" | "mod.rs") => parent.to_path_buf(),
        _ => parent.join(path.file_stem().unwrap_or_default()),
    }
}

/// Repo-relative `/`-separated path prefixes and files that are test code, given every `.rs` file as
/// `(path, source)`.
#[must_use]
pub fn test_only_paths<'a>(files: impl IntoIterator<Item = (&'a Path, &'a str)>) -> BTreeSet<String> {
    let mut paths = BTreeSet::new();
    for (path, source) in files {
        let directory = module_directory(path);
        for module in test_only_modules(source) {
            paths.insert(slash(&directory.join(format!("{module}.rs"))));
            paths.insert(slash(&directory.join(&module)) + "/");
        }
    }
    paths
}

fn slash(path: &Path) -> String {
    path.to_string_lossy().replace('\\', "/")
}

/// Whether the file at `path` is test code wholesale.
#[must_use]
pub fn is_test_file(path: &Path, test_only: &BTreeSet<String>) -> bool {
    let path = slash(path);
    path.split('/').rev().skip(1).any(|part| part == "tests")
        || TEST_ONLY_PREFIXES.iter().any(|prefix| path.starts_with(prefix))
        || test_only.contains(&path)
        || test_only
            .iter()
            .any(|entry| entry.ends_with('/') && path.starts_with(entry.as_str()))
}

/// Parses the allow-list: `<path>: <reason>` lines. Returns the allowed paths or the first bad line.
///
/// # Errors
///
/// Returns a message naming the line when an entry has no reason.
pub fn parse_allow_list(text: &str) -> Result<BTreeSet<String>, String> {
    let mut allowed = BTreeSet::new();
    for (index, line) in text.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        match line.split_once(':') {
            Some((path, reason)) if !path.trim().is_empty() && !reason.trim().is_empty() => {
                allowed.insert(path.trim().to_owned());
            }
            _ => {
                return Err(format!(
                    "allow-list line {}: expected `<path>: <reason>` with a non-empty reason",
                    index + 1
                ));
            }
        }
    }
    Ok(allowed)
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::*;

    const CALLER: &str = concat!("Location::", "caller()");
    const ATTRIBUTE: &str = concat!("#[track_", "caller]");

    #[test]
    fn it_should_flag_a_location_in_production_code() {
        let source = format!("fn new() -> Self {{\n    Self {{ location: {CALLER} }}\n}}\n");

        assert_eq!(violations_in(&source).len(), 1);
        assert_eq!(violations_in(&source)[0].line, 2);
    }

    #[test]
    fn it_should_flag_the_track_caller_attribute_and_the_located_error_crate() {
        let source = format!("use torrust_located_{}::DynError;\n{ATTRIBUTE}\nfn f() {{}}\n", "error");

        assert_eq!(violations_in(&source).len(), 2);
    }

    #[test]
    fn it_should_ignore_a_cfg_test_module() {
        let source = format!("fn f() {{}}\n\n#[cfg(test)]\nmod tests {{\n    {ATTRIBUTE}\n    fn helper() {{ {CALLER}; }}\n}}\n");

        assert_eq!(violations_in(&source), vec![]);
    }

    #[test]
    fn it_should_flag_code_after_a_cfg_test_module() {
        let source = format!("#[cfg(test)]\nmod tests {{\n    fn helper() {{}}\n}}\n\n{ATTRIBUTE}\nfn f() {{}}\n");

        assert_eq!(violations_in(&source).iter().map(|v| v.line).collect::<Vec<_>>(), vec![6]);
    }

    #[test]
    fn it_should_ignore_a_nested_cfg_test_block_and_keep_checking_after_it() {
        let source = format!(
            "impl A {{\n    #[cfg(test)]\n    {ATTRIBUTE}\n    fn t() {{\n    }}\n\n    {ATTRIBUTE}\n    fn p() {{}}\n}}\n"
        );

        assert_eq!(violations_in(&source).iter().map(|v| v.line).collect::<Vec<_>>(), vec![7]);
    }

    #[test]
    fn it_should_ignore_comment_lines() {
        let source = format!("// records {CALLER}\n/// uses {ATTRIBUTE}\nfn f() {{}}\n");

        assert_eq!(violations_in(&source), vec![]);
    }

    #[test]
    fn it_should_find_modules_declared_for_tests_only() {
        let source = "#[cfg(test)]\nmod a;\nmod b;\n#[cfg(test)]\n#[allow(unused)]\npub mod c;\n";

        assert_eq!(test_only_modules(source), vec!["a", "c"]);
    }

    #[test]
    fn it_should_treat_declared_test_modules_and_tests_directories_as_test_code() {
        let lib = Path::new("packages/x/src/lib.rs");
        let paths = test_only_paths([(lib, "#[cfg(test)]\nmod helpers;\n")]);

        assert!(is_test_file(Path::new("packages/x/src/helpers.rs"), &paths));
        assert!(is_test_file(Path::new("packages/x/src/helpers/deep.rs"), &paths));
        assert!(is_test_file(Path::new("packages/x/tests/server/asserts.rs"), &paths));
        assert!(is_test_file(Path::new("packages/test-helpers/src/lib.rs"), &paths));
        assert!(!is_test_file(Path::new("packages/x/src/other.rs"), &paths));
    }

    #[test]
    fn it_should_reject_an_allow_list_entry_without_a_reason() {
        assert!(parse_allow_list("# c\na/b.rs: legacy until #1\n").is_ok());
        assert!(parse_allow_list("a/b.rs\n").is_err());
        assert!(parse_allow_list("a/b.rs:  \n").is_err());
    }
}
