//! Import parsing utilities for the workspace coupling report.

use std::collections::BTreeSet;

use syn::visit::{self, Visit};
use syn::{Path, UseTree};

/// Parses Rust source and returns dependency paths imported from `dep_module`.
///
/// This convenience wrapper is intentionally pure and panic-free for tests and
/// callers that only need best-effort import extraction.
#[must_use]
pub fn parse_imports_from_source(source: &str, dep_module: &str) -> BTreeSet<String> {
    try_parse_imports_from_source(source, dep_module).unwrap_or_default()
}

/// Parses Rust source and returns dependency paths imported from `dep_module`.
///
/// The fallible variant is used by the binary so malformed Rust source can be
/// surfaced as a structured CLI error instead of being silently ignored.
///
/// # Errors
///
/// Returns a [`syn::Error`] when `source` is not valid Rust syntax.
pub fn try_parse_imports_from_source(source: &str, dep_module: &str) -> Result<BTreeSet<String>, syn::Error> {
    let file = syn::parse_file(source)?;
    let mut visitor = ImportVisitor {
        dep_module,
        imports: BTreeSet::new(),
    };

    visitor.visit_file(&file);

    Ok(visitor.imports)
}

struct ImportVisitor<'a> {
    dep_module: &'a str,
    imports: BTreeSet<String>,
}

impl<'ast> Visit<'ast> for ImportVisitor<'_> {
    fn visit_item_use(&mut self, node: &'ast syn::ItemUse) {
        self.collect_use_tree(&node.tree, &mut Vec::new());
    }

    fn visit_macro(&mut self, node: &'ast syn::Macro) {
        self.collect_macro_path_references(&node.tokens.to_string());
        visit::visit_macro(self, node);
    }

    fn visit_path(&mut self, node: &'ast Path) {
        self.collect_path_reference(node);
        visit::visit_path(self, node);
    }
}

impl ImportVisitor<'_> {
    fn collect_use_tree(&mut self, tree: &UseTree, prefix: &mut Vec<String>) {
        match tree {
            UseTree::Path(path) => {
                prefix.push(path.ident.to_string());
                self.collect_use_tree(&path.tree, prefix);
                prefix.pop();
            }
            UseTree::Name(name) => {
                prefix.push(name.ident.to_string());
                self.record_use_path(prefix);
                prefix.pop();
            }
            UseTree::Rename(rename) => {
                prefix.push(rename.ident.to_string());
                self.record_rename_path(prefix);
                prefix.pop();
            }
            UseTree::Glob(_) => {
                prefix.push(String::from("*"));
                self.record_use_path(prefix);
                prefix.pop();
            }
            UseTree::Group(group) => {
                for tree in &group.items {
                    self.collect_use_tree(tree, prefix);
                }
            }
        }
    }

    fn record_use_path(&mut self, path: &[String]) {
        let Some(import_path) = self.dep_import_path(path) else {
            return;
        };

        if import_path.len() < 2 {
            return;
        }

        self.imports.insert(import_path.join("::"));
    }

    fn record_rename_path(&mut self, path: &[String]) {
        let Some(import_path) = self.dep_import_path(path) else {
            return;
        };

        if import_path.is_empty() {
            return;
        }

        self.imports.insert(import_path.join("::"));
    }

    fn dep_import_path<'a>(&self, path: &'a [String]) -> Option<&'a [String]> {
        let module = path.first()?;

        if module != self.dep_module {
            return None;
        }

        let import_path = if path.last().is_some_and(|segment| segment == "self") {
            &path[..path.len().saturating_sub(1)]
        } else {
            path
        };

        Some(import_path)
    }

    fn collect_path_reference(&mut self, path: &Path) {
        self.record_path_reference_segments(path.segments.iter().map(|segment| segment.ident.to_string()));
    }

    fn collect_macro_path_references(&mut self, tokens: &str) {
        let mut search_start = 0;

        while let Some(relative_start) = tokens[search_start..].find(self.dep_module) {
            let start = search_start + relative_start;
            let after_module = start + self.dep_module.len();
            search_start = after_module;

            if !has_identifier_boundaries(tokens, start, after_module) {
                continue;
            }

            let Some(mut cursor) = consume_path_separator(tokens, after_module) else {
                continue;
            };

            let mut segments = vec![self.dep_module.to_owned()];

            while let Some((segment, after_segment)) = parse_identifier(tokens, cursor) {
                segments.push(segment);

                if segments.len() == 3 {
                    break;
                }

                let Some(after_separator) = consume_path_separator(tokens, after_segment) else {
                    break;
                };
                cursor = after_separator;
            }

            self.record_path_reference_segments(segments.into_iter());
        }
    }

    fn record_path_reference_segments<I>(&mut self, mut segments: I)
    where
        I: Iterator<Item = String>,
    {
        let Some(first) = segments.next() else {
            return;
        };

        if first != self.dep_module {
            return;
        }

        let Some(second) = segments.next() else {
            return;
        };

        let mut import_path = vec![self.dep_module.to_owned(), second];

        if let Some(third) = segments.next() {
            import_path.push(third);
        }

        self.imports.insert(import_path.join("::"));
    }
}

fn consume_path_separator(source: &str, cursor: usize) -> Option<usize> {
    let cursor = skip_whitespace(source, cursor);

    source[cursor..].starts_with("::").then_some(cursor + 2)
}

fn parse_identifier(source: &str, cursor: usize) -> Option<(String, usize)> {
    let cursor = skip_whitespace(source, cursor);
    let ident_start = source[cursor..].strip_prefix("r#").map_or(cursor, |_| cursor + 2);

    let first = source[ident_start..].chars().next()?;
    if !is_rust_identifier_start(first) {
        return None;
    }

    let mut end = ident_start + first.len_utf8();
    for ch in source[end..].chars() {
        if !is_rust_identifier_continue(ch) {
            break;
        }
        end += ch.len_utf8();
    }

    Some((source[cursor..end].to_owned(), end))
}

fn skip_whitespace(source: &str, cursor: usize) -> usize {
    let mut cursor = cursor;

    for ch in source[cursor..].chars() {
        if !ch.is_whitespace() {
            break;
        }
        cursor += ch.len_utf8();
    }

    cursor
}

fn has_identifier_boundaries(source: &str, start: usize, end: usize) -> bool {
    let before = source[..start].chars().next_back();
    let after = source[end..].chars().next();

    !is_rust_identifier_continue(before.unwrap_or('\0')) && !is_rust_identifier_continue(after.unwrap_or('\0'))
}

const fn is_rust_identifier_start(ch: char) -> bool {
    ch == '_' || ch.is_ascii_alphabetic()
}

const fn is_rust_identifier_continue(ch: char) -> bool {
    ch == '_' || ch.is_ascii_alphanumeric()
}
