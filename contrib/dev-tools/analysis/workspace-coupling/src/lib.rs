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
                self.record_use_path(prefix);
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
        let Some(module) = path.first() else {
            return;
        };

        if module != self.dep_module {
            return;
        }

        let import_path = if path.last().is_some_and(|segment| segment == "self") {
            &path[..path.len().saturating_sub(1)]
        } else {
            path
        };

        if import_path.len() < 2 {
            return;
        }

        self.imports.insert(import_path.join("::"));
    }

    fn collect_path_reference(&mut self, path: &Path) {
        let mut segments = path.segments.iter();
        let Some(first) = segments.next() else {
            return;
        };

        if first.ident != self.dep_module {
            return;
        }

        let Some(second) = segments.next() else {
            return;
        };

        let mut import_path = vec![self.dep_module.to_owned(), second.ident.to_string()];

        if let Some(third) = segments.next() {
            import_path.push(third.ident.to_string());
        }

        self.imports.insert(import_path.join("::"));
    }
}
