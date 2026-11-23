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
                    } else if c == 'r' && raw_string_hashes(&chars, i).is_some() && raw_prefix_allowed(&chars, i) {
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

/// Whether the `r` at `i` starts a raw string: it must not end an identifier, except as the `r` of a `br` prefix.
fn raw_prefix_allowed(chars: &[char], i: usize) -> bool {
    !preceded_by_identifier(chars, i) || (chars[i - 1] == 'b' && !preceded_by_identifier(chars, i - 1))
}

/// The length of the character literal that starts at `i`, or `None` for a lifetime.
fn char_literal_length(chars: &[char], i: usize) -> Option<usize> {
    match (chars.get(i + 1), chars.get(i + 2)) {
        (Some('\\'), _) => {
            // `\` and the escaped character come first, so `'\''` closes at the fourth character.
            let close = chars.get(i + 3..)?.iter().position(|c| *c == '\'')?;
            Some(close + 4)
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

    while let Some((first_index, first_line)) = lines.next() {
        let (mut index, mut text) = (first_index, first_line.as_str());
        // An excluded item can end in the middle of a line; the rest of that line is scanned too.
        while let Some(after_attribute) = text.trim_start().strip_prefix(CFG_TEST) {
            let Some((end_index, rest)) = skip_test_item(index, after_attribute, &mut lines) else {
                text = "";
                break;
            };
            (index, text) = (end_index, rest);
        }
        for (pattern, reason) in FORBIDDEN {
            if text.contains(pattern) {
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

const CFG_TEST: &str = "#[cfg(test)]";

/// Skips the attributes at the start of `text`, which may continue from an earlier line (`depth` is the
/// number of unclosed `[`), and returns the text after them. Returns an empty string while an attribute
/// is still open at the end of `text`.
fn skip_attributes<'a>(text: &'a str, depth: &mut usize) -> &'a str {
    let mut rest = text;
    'attributes: loop {
        let trimmed = rest.trim_start();
        let body = if *depth > 0 {
            trimmed
        } else if let Some(after) = trimmed.strip_prefix("#[") {
            *depth = 1;
            after
        } else {
            return trimmed;
        };
        for (index, c) in body.char_indices() {
            match c {
                '[' => *depth += 1,
                ']' => {
                    *depth -= 1;
                    if *depth == 0 {
                        rest = &body[index + 1..];
                        continue 'attributes;
                    }
                }
                _ => {}
            }
        }
        return "";
    }
}

/// Consumes the item that follows a `#[cfg(test)]` attribute: `first` is the text after the attribute on
/// line `index`, then further attributes (also on the item's line, or over several lines), then everything up
/// to the `;` or the closing delimiter that ends the item, found by balancing `{}`, `()` and `[]`.
///
/// Returns the line on which the item ends and the text of that line after the item, or `None` when the
/// source ends first.
fn skip_test_item<'a>(
    index: usize,
    first: &'a str,
    lines: &mut impl Iterator<Item = (usize, &'a String)>,
) -> Option<(usize, &'a str)> {
    let mut depth = 0_usize;
    let mut attribute_depth = 0_usize;
    let mut started = false;

    let mut next = Some((index, first));
    while let Some((line_index, text)) = next {
        let item = if started {
            text
        } else {
            skip_attributes(text, &mut attribute_depth)
        };
        if started || !item.trim().is_empty() {
            started = true;
            for (offset, c) in item.char_indices() {
                match c {
                    '{' | '(' | '[' => depth += 1,
                    '}' | ')' | ']' => {
                        depth = depth.saturating_sub(1);
                        if depth == 0 && c == '}' {
                            return Some((line_index, &item[offset + 1..]));
                        }
                    }
                    ';' if depth == 0 => return Some((line_index, &item[offset + 1..])),
                    _ => {}
                }
            }
        }
        next = lines.next().map(|(next_index, line)| (next_index, line.as_str()));
    }

    None
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

    let mut attribute_depth = 0_usize;

    for line in mask(source) {
        if attribute_depth == 0 && line.trim_start().starts_with(CFG_TEST) {
            after_cfg_test = true;
        }
        let item = skip_attributes(&line, &mut attribute_depth);
        if item.is_empty() {
            continue;
        }

        if let Some((name, inline)) = module_item(item) {
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

        for c in item.chars() {
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

/// The files and directory prefixes (repo-relative, `/`-separated, prefixes end in `/`) that module
/// declarations include, split by whether the declaration is `#[cfg(test)]`.
#[derive(Debug, Default, PartialEq, Eq)]
pub struct ModulePaths {
    pub test_only: BTreeSet<String>,
    pub production: BTreeSet<String>,
}

/// Collects the module paths of every `.rs` file, given as `(path, source)`.
///
/// A file counts as test code only when every declaration that includes it is test-only: a `#[cfg(test)]`
/// declaration, or any declaration inside a file that is itself test code (so the modules of a test-only
/// file are test-only). One production declaration (for example in `main.rs` next to a test-only one in
/// `lib.rs`) keeps a file checked, and a production declaration below a test-only directory prefix
/// overrides that prefix (see [`is_test_file`]).
#[must_use]
pub fn test_only_paths<'a>(files: impl IntoIterator<Item = (&'a Path, &'a str)>) -> ModulePaths {
    struct Declared {
        declaring_file: String,
        module: PathBuf,
        test_only: bool,
    }

    let mut declared = Vec::new();
    for (path, source) in files {
        let directory = module_directory(path);
        for declaration in module_declarations(source) {
            declared.push(Declared {
                declaring_file: slash(path),
                module: directory.join(declaration.path.join("/")),
                test_only: declaration.test_only,
            });
        }
    }

    // Declarations in test code are test-only; which files are test code depends on the declarations, so
    // repeat until nothing changes. The test-only set only grows, so this ends.
    let mut paths = ModulePaths::default();
    loop {
        let mut next = ModulePaths::default();
        for declaration in &declared {
            let test_only = declaration.test_only || is_test_file(Path::new(&declaration.declaring_file), &paths);
            let target = if test_only {
                &mut next.test_only
            } else {
                &mut next.production
            };
            target.insert(slash(&declaration.module.with_extension("rs")));
            target.insert(slash(&declaration.module) + "/");
        }
        next.test_only.retain(|entry| !next.production.contains(entry));
        if next == paths {
            return paths;
        }
        paths = next;
    }
}

fn slash(path: &Path) -> String {
    path.to_string_lossy().replace('\\', "/")
}

/// The length of the longest entry of `entries` that is `path` itself or a directory prefix of it.
fn longest_match(path: &str, entries: &BTreeSet<String>) -> Option<usize> {
    entries
        .iter()
        .filter(|entry| path == entry.as_str() || (entry.ends_with('/') && path.starts_with(entry.as_str())))
        .map(String::len)
        .max()
}

/// Whether the file at `path` is test code wholesale. Of a test-only and a production module path that
/// both match, the more specific (longer) one decides.
#[must_use]
pub fn is_test_file(path: &Path, paths: &ModulePaths) -> bool {
    let path = slash(path);
    if path.split('/').rev().skip(1).any(|part| part == "tests")
        || TEST_ONLY_PREFIXES.iter().any(|prefix| path.starts_with(prefix))
    {
        return true;
    }
    match (
        longest_match(&path, &paths.test_only),
        longest_match(&path, &paths.production),
    ) {
        (Some(test), Some(production)) => test > production,
        (Some(_), None) => true,
        (None, _) => false,
    }
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
    fn it_should_mask_a_raw_byte_string_and_an_escaped_apostrophe() {
        let raw_byte = format!("let a = br#\"\"\"#;\n{CALLER};\n");
        let apostrophe = format!("fn t() {{ let _ = ['\\'', '{{']; }}\n{CALLER};\n");

        assert_eq!(violations_in(&raw_byte).iter().map(|v| v.line).collect::<Vec<_>>(), vec![2]);
        assert_eq!(violations_in(&apostrophe).iter().map(|v| v.line).collect::<Vec<_>>(), vec![2]);
    }

    #[test]
    fn it_should_skip_an_item_that_shares_its_line_with_another_attribute() {
        let source = format!("#[cfg(test)]\n#[allow(dead_code)] fn helper() {{}}\n{ATTRIBUTE}\nfn production() {{}}\n");

        assert_eq!(violations_in(&source).iter().map(|v| v.line).collect::<Vec<_>>(), vec![3]);
    }

    #[test]
    fn it_should_keep_the_cfg_test_attribute_across_a_multiline_attribute() {
        let source = "#[cfg(test)]\n#[allow(\n    dead_code\n)]\nmod helpers;\n";

        assert!(module_declarations(source)[0].test_only);
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
    fn it_should_let_a_production_declaration_override_a_test_only_directory_prefix() {
        let lib = Path::new("packages/x/src/lib.rs");
        let main = Path::new("packages/x/src/main.rs");
        let paths = test_only_paths([
            (lib, "#[cfg(test)]\nmod shared;\n"),
            (main, "mod shared {\n    mod child;\n}\n"),
        ]);

        assert!(!is_test_file(Path::new("packages/x/src/shared/child.rs"), &paths));
        assert!(!is_test_file(Path::new("packages/x/src/shared/child/deeper.rs"), &paths));
    }

    #[test]
    fn it_should_resume_scanning_after_an_excluded_item_that_ends_mid_line() {
        let source = format!(
            "#[cfg(test)]\n#[allow(dead_code)] fn helper() {{ {ATTRIBUTE} fn inner() {{}} }} {ATTRIBUTE} fn production() {{}}\n"
        );

        assert_eq!(violations_in(&source).iter().map(|v| v.line).collect::<Vec<_>>(), vec![2]);
    }

    #[test]
    fn it_should_treat_modules_declared_in_a_test_only_file_as_test_only() {
        let lib = Path::new("packages/x/src/lib.rs");
        let helpers = Path::new("packages/x/src/helpers.rs");
        let paths = test_only_paths([(lib, "#[cfg(test)]\nmod helpers;\n"), (helpers, "mod nested;\n")]);

        assert!(is_test_file(Path::new("packages/x/src/helpers/nested.rs"), &paths));
    }

    #[test]
    fn it_should_keep_the_delimiter_depth_across_adjacent_character_literals_in_a_test_only_function() {
        let source = format!("#[cfg(test)]\nfn t() {{ let _ = ['\\'','{{']; }}\n{ATTRIBUTE}\nfn production() {{}}\n");

        assert_eq!(violations_in(&source).iter().map(|v| v.line).collect::<Vec<_>>(), vec![3]);
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
