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

/// Returns `source` split into lines, with comments and the content of string and character literals
/// replaced by spaces, so that text in them is never scanned and never counted as a delimiter.
///
/// Handles line comments, nested block comments, strings with escapes (also across lines), raw strings
/// and character literals; a lifetime such as `'a` is left alone.
#[must_use]
pub fn mask(source: &str) -> Vec<String> {
    #[derive(Clone, Copy)]
    enum State {
        Code,
        Block(usize),
        Str,
        Raw(usize),
    }

    let mut state = State::Code;
    let mut lines = Vec::new();

    for line in source.lines() {
        let chars: Vec<char> = line.chars().collect();
        let mut out = String::with_capacity(line.len());
        let mut i = 0;
        while i < chars.len() {
            let c = chars[i];
            let next = chars.get(i + 1).copied();
            match state {
                State::Block(depth) => {
                    out.push(' ');
                    if c == '*' && next == Some('/') {
                        out.push(' ');
                        i += 1;
                        state = if depth == 1 { State::Code } else { State::Block(depth - 1) };
                    } else if c == '/' && next == Some('*') {
                        out.push(' ');
                        i += 1;
                        state = State::Block(depth + 1);
                    }
                }
                State::Str => {
                    out.push(' ');
                    if c == '\\' && next.is_some() {
                        out.push(' ');
                        i += 1;
                    } else if c == '"' {
                        state = State::Code;
                    }
                }
                State::Raw(hashes) => {
                    out.push(' ');
                    if c == '"' && (0..hashes).all(|k| chars.get(i + 1 + k) == Some(&'#')) {
                        for _ in 0..hashes {
                            out.push(' ');
                        }
                        i += hashes;
                        state = State::Code;
                    }
                }
                State::Code => {
                    if c == '/' && next == Some('/') {
                        out.extend(std::iter::repeat_n(' ', chars.len() - i));
                        break;
                    } else if c == '/' && next == Some('*') {
                        out.push_str("  ");
                        i += 1;
                        state = State::Block(1);
                    } else if c == '"' {
                        out.push(' ');
                        state = State::Str;
                    } else if c == 'r' && raw_string_hashes(&chars, i).is_some() && !preceded_by_identifier(&chars, i) {
                        let hashes = raw_string_hashes(&chars, i).unwrap_or(0);
                        out.extend(std::iter::repeat_n(' ', hashes + 2));
                        i += hashes + 1;
                        state = State::Raw(hashes);
                    } else if c == '\'' {
                        if let Some(length) = char_literal_length(&chars, i) {
                            out.extend(std::iter::repeat_n(' ', length));
                            i += length - 1;
                        } else {
                            out.push(c);
                        }
                    } else {
                        out.push(c);
                    }
                }
            }
            i += 1;
        }
        lines.push(out);
    }

    lines
}

/// The number of `#` of a raw string opening (`r"`, `r#"`, ...) at `i`, if there is one.
fn raw_string_hashes(chars: &[char], i: usize) -> Option<usize> {
    let hashes = chars[i + 1..].iter().take_while(|c| **c == '#').count();
    (chars.get(i + 1 + hashes) == Some(&'"')).then_some(hashes)
}

fn preceded_by_identifier(chars: &[char], i: usize) -> bool {
    i > 0 && (chars[i - 1].is_alphanumeric() || chars[i - 1] == '_')
}

/// The length of the character literal that starts at `i`, or `None` for a lifetime.
fn char_literal_length(chars: &[char], i: usize) -> Option<usize> {
    match (chars.get(i + 1), chars.get(i + 2)) {
        (Some('\\'), _) => {
            let close = chars[i + 2..].iter().position(|c| *c == '\'')?;
            Some(close + 3)
        }
        (Some(_), Some('\'')) => Some(3),
        _ => None,
    }
}

/// Returns the forbidden uses in the non-test part of `source`.
#[must_use]
pub fn violations_in(source: &str) -> Vec<Violation> {
    let masked = mask(source);
    let mut violations = Vec::new();
    let mut lines = masked.iter().enumerate();

    while let Some((index, line)) = lines.next() {
        if line.trim() == "#[cfg(test)]" {
            skip_test_item(&mut lines);
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

/// Consumes the item that follows a `#[cfg(test)]` attribute: further attributes, then everything up to
/// the `;` or the closing delimiter that ends the item, found by balancing `{}`, `()` and `[]`.
fn skip_test_item<'a>(lines: &mut impl Iterator<Item = (usize, &'a String)>) {
    let mut depth = 0_usize;
    let mut started = false;

    for (_, line) in lines.by_ref() {
        let trimmed = line.trim();
        if !started && (trimmed.is_empty() || trimmed.starts_with("#[")) {
            continue;
        }
        started = true;
        for c in line.chars() {
            match c {
                '{' | '(' | '[' => depth += 1,
                '}' | ')' | ']' => {
                    depth = depth.saturating_sub(1);
                    if depth == 0 && c == '}' {
                        return;
                    }
                }
                ';' if depth == 0 => return,
                _ => {}
            }
        }
    }
}

/// A `mod name;` declaration found in a source file.
#[derive(Debug, PartialEq, Eq)]
pub struct ModuleDeclaration {
    /// The inline modules that enclose the declaration, outermost first, then the module itself.
    pub path: Vec<String>,
    /// Whether the declaration is `#[cfg(test)]`, directly or through an enclosing `#[cfg(test)]` module.
    pub test_only: bool,
}

/// The `mod name;` declarations of `source`, each with its inline-module ancestry.
#[must_use]
pub fn module_declarations(source: &str) -> Vec<ModuleDeclaration> {
    let mut declarations = Vec::new();
    // (name, brace depth outside the module, under cfg(test))
    let mut stack: Vec<(String, usize, bool)> = Vec::new();
    let mut depth = 0_usize;
    let mut after_cfg_test = false;

    for line in mask(source) {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        if trimmed == "#[cfg(test)]" {
            after_cfg_test = true;
            continue;
        }
        if trimmed.starts_with("#[") {
            continue;
        }

        if let Some((name, inline)) = module_item(trimmed) {
            let test_only = after_cfg_test || stack.iter().any(|entry| entry.2);
            if inline {
                stack.push((name, depth, test_only));
            } else {
                let mut path: Vec<String> = stack.iter().map(|entry| entry.0.clone()).collect();
                path.push(name);
                declarations.push(ModuleDeclaration { path, test_only });
            }
        }
        after_cfg_test = false;

        for c in line.chars() {
            match c {
                '{' => depth += 1,
                '}' => {
                    depth = depth.saturating_sub(1);
                    while stack.last().is_some_and(|entry| entry.1 >= depth) {
                        stack.pop();
                    }
                }
                _ => {}
            }
        }
    }

    declarations
}

/// Parses `[pub ...] mod name;` or `[pub ...] mod name {` and returns the name and whether it is inline.
fn module_item(trimmed: &str) -> Option<(String, bool)> {
    let mut rest = trimmed;
    if let Some(after) = rest.strip_prefix("pub") {
        rest = after.trim_start();
        if rest.starts_with('(') {
            rest = rest[rest.find(')')? + 1..].trim_start();
        }
    }
    let rest = rest.strip_prefix("mod ")?.trim_start();
    let name_end = rest.find(|c: char| !(c.is_alphanumeric() || c == '_'))?;
    let (name, after) = rest.split_at(name_end);
    match after.trim_start().chars().next()? {
        ';' => Some((name.to_owned(), false)),
        '{' => Some((name.to_owned(), true)),
        _ => None,
    }
}

/// The directory in which the child modules of the file at `path` live.
fn module_directory(path: &Path) -> PathBuf {
    let parent = path.parent().unwrap_or_else(|| Path::new(""));
    match path.file_name().and_then(|name| name.to_str()) {
        Some("lib.rs" | "main.rs" | "mod.rs") => parent.to_path_buf(),
        _ => parent.join(path.file_stem().unwrap_or_default()),
    }
}

/// Repo-relative `/`-separated files and directory prefixes (ending in `/`) that are test code, given
/// every `.rs` file as `(path, source)`.
///
/// A module file counts only when every declaration that includes it is `#[cfg(test)]`: one production
/// declaration (for example in `main.rs` next to a test-only one in `lib.rs`) keeps it checked.
#[must_use]
pub fn test_only_paths<'a>(files: impl IntoIterator<Item = (&'a Path, &'a str)>) -> BTreeSet<String> {
    let mut test_only = BTreeSet::new();
    let mut production = BTreeSet::new();

    for (path, source) in files {
        let directory = module_directory(path);
        for declaration in module_declarations(source) {
            let module = directory.join(declaration.path.join("/"));
            let file = slash(&module.with_extension("rs"));
            let prefix = slash(&module) + "/";
            let target = if declaration.test_only {
                &mut test_only
            } else {
                &mut production
            };
            target.insert(file);
            target.insert(prefix);
        }
    }

    test_only.retain(|entry| !production.contains(entry));
    test_only
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
                allowed.insert(path.trim().replace('\\', "/"));
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
    fn it_should_ignore_trailing_line_comments_block_comments_and_string_literals() {
        let source = format!(
            "fn f() {{ let a = 1; // {CALLER}\n    /* {ATTRIBUTE}\n    {CALLER} */ let b = \"{CALLER}\";\n    let c = r#\"{CALLER}\"#; let d = '{{';\n}}\n"
        );

        assert_eq!(violations_in(&source), vec![]);
    }

    #[test]
    fn it_should_stop_skipping_at_the_end_of_a_test_only_import() {
        let source = format!("#[cfg(test)]\nuse std::{{fmt, io}};\n\n{ATTRIBUTE}\nfn f() {{}}\n");

        assert_eq!(violations_in(&source).iter().map(|v| v.line).collect::<Vec<_>>(), vec![4]);
    }

    #[test]
    fn it_should_skip_a_test_only_item_whose_braces_are_not_on_their_own_lines() {
        let source = format!("#[cfg(test)]\nfn t() -> Vec<u8> {{ vec![{CALLER}] }}\n{ATTRIBUTE}\nfn f() {{}}\n");

        assert_eq!(violations_in(&source).iter().map(|v| v.line).collect::<Vec<_>>(), vec![3]);
    }

    #[test]
    fn it_should_find_modules_declared_for_tests_only() {
        let source = "#[cfg(test)]\nmod a;\nmod b;\n#[cfg(test)]\n#[allow(unused)]\npub mod c;\n";

        let declarations = module_declarations(source);

        assert_eq!(
            declarations
                .iter()
                .map(|d| (d.path.join("/"), d.test_only))
                .collect::<Vec<_>>(),
            vec![("a".to_owned(), true), ("b".to_owned(), false), ("c".to_owned(), true)]
        );
    }

    #[test]
    fn it_should_keep_the_cfg_test_attribute_across_comment_lines() {
        let source = "#[cfg(test)]\n// Test helpers.\n/* more */\nmod helpers;\n";

        assert!(module_declarations(source)[0].test_only);
    }

    #[test]
    fn it_should_keep_the_inline_module_ancestry_of_a_declaration() {
        let source = "mod inner {\n    #[cfg(test)]\n    mod helpers;\n    mod other;\n}\n#[cfg(test)]\nmod outer {\n    mod nested;\n}\nmod last;\n";

        let declarations = module_declarations(source);

        assert_eq!(
            declarations
                .iter()
                .map(|d| (d.path.join("/"), d.test_only))
                .collect::<Vec<_>>(),
            vec![
                ("inner/helpers".to_owned(), true),
                ("inner/other".to_owned(), false),
                ("outer/nested".to_owned(), true),
                ("last".to_owned(), false),
            ]
        );
    }

    #[test]
    fn it_should_treat_declared_test_modules_and_tests_directories_as_test_code() {
        let lib = Path::new("packages/x/src/lib.rs");
        let paths = test_only_paths([(
            lib,
            "#[cfg(test)]\nmod helpers;\nmod inner {\n    #[cfg(test)]\n    mod deep;\n}\n",
        )]);

        assert!(is_test_file(Path::new("packages/x/src/helpers.rs"), &paths));
        assert!(is_test_file(Path::new("packages/x/src/helpers/deeper.rs"), &paths));
        assert!(is_test_file(Path::new("packages/x/src/inner/deep.rs"), &paths));
        assert!(!is_test_file(Path::new("packages/x/src/deep.rs"), &paths));
        assert!(is_test_file(Path::new("packages/x/tests/server/asserts.rs"), &paths));
        assert!(is_test_file(Path::new("packages/test-helpers/src/lib.rs"), &paths));
        assert!(!is_test_file(Path::new("packages/x/src/other.rs"), &paths));
    }

    #[test]
    fn it_should_keep_checking_a_file_that_a_production_declaration_also_includes() {
        let lib = Path::new("packages/x/src/lib.rs");
        let main = Path::new("packages/x/src/main.rs");
        let paths = test_only_paths([(lib, "#[cfg(test)]\nmod shared;\n"), (main, "mod shared;\n")]);

        assert!(!is_test_file(Path::new("packages/x/src/shared.rs"), &paths));
    }

    #[test]
    fn it_should_normalize_backslashes_of_allow_list_paths() {
        assert!(parse_allow_list("a\\b.rs: reason\n").unwrap().contains("a/b.rs"));
    }

    #[test]
    fn it_should_reject_an_allow_list_entry_without_a_reason() {
        assert!(parse_allow_list("# c\na/b.rs: legacy until #1\n").is_ok());
        assert!(parse_allow_list("a/b.rs\n").is_err());
        assert!(parse_allow_list("a/b.rs:  \n").is_err());
    }
}
