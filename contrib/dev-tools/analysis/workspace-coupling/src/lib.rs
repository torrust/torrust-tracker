//! Import parsing and module-name resolution utilities for the workspace coupling report.

use std::collections::{BTreeMap, BTreeSet, HashSet};
use std::fmt;

use serde::Deserialize;
use syn::visit::{self, Visit};
use syn::{Path, UseTree};

/// The subset of `cargo metadata` output the coupling report needs.
#[derive(Deserialize)]
pub struct Metadata {
    pub workspace_root: String,
    pub workspace_members: Vec<String>,
    pub packages: Vec<Package>,
}

/// A package entry of `cargo metadata`.
#[derive(Deserialize)]
pub struct Package {
    pub id: String,
    pub name: String,
    pub manifest_path: String,
    pub dependencies: Vec<Dep>,
    #[serde(default)]
    pub targets: Vec<Target>,
    /// The `[features]` table: each feature with the items it enables.
    #[serde(default)]
    pub features: BTreeMap<String, Vec<String>>,
}

/// A dependency declaration of a package in `cargo metadata`.
#[derive(Deserialize)]
pub struct Dep {
    /// The name of the dependency package, not the name used in Rust code.
    pub name: String,
    pub kind: Option<String>,
    /// The key used in `Cargo.toml` when the dependency is renamed with `package = "..."`.
    #[serde(default)]
    pub rename: Option<String>,
    /// Whether the dependency is declared with `optional = true`.
    #[serde(default)]
    pub optional: bool,
}

/// A build target of a package in `cargo metadata`.
#[derive(Deserialize)]
pub struct Target {
    pub name: String,
    pub kind: Vec<String>,
}

/// Returns the module name that Rust code uses to import the dependency `dep`.
///
/// Resolution order: the `rename` set in the dependent's `Cargo.toml`, otherwise the name of the
/// dependency package's library target, otherwise the dependency package name. Hyphens become
/// underscores, as Cargo does. `dep_package` is the package the dependency resolves to, when known.
#[must_use]
pub fn resolve_module_name(dep: &Dep, dep_package: Option<&Package>) -> String {
    let name = dep
        .rename
        .as_deref()
        .or_else(|| dep_package.and_then(Package::library_target_name))
        .unwrap_or(&dep.name);

    name.replace('-', "_")
}

/// How a package depends on another one. The variants are declared in report order.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum EdgeKind {
    /// A required normal dependency.
    Normal,
    /// An optional normal dependency with the sorted features that enable it, possibly none.
    Optional(Vec<String>),
    Dev,
    Build,
}

impl fmt::Display for EdgeKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Normal => f.write_str("normal"),
            Self::Optional(features) if features.is_empty() => f.write_str("optional"),
            Self::Optional(features) => write!(f, "optional ({})", features.join(", ")),
            Self::Dev => f.write_str("dev"),
            Self::Build => f.write_str("build"),
        }
    }
}

impl Package {
    /// The dependencies of this package on other workspace packages, without a dependency on itself.
    ///
    /// A package depends on itself through the path-only self dev-dependency that enables its own
    /// `testing` feature for its tests. That is not coupling to another package.
    #[must_use]
    pub fn workspace_dependencies(&self, ws_names: &HashSet<&str>) -> Vec<&Dep> {
        self.dependencies
            .iter()
            .filter(|dep| dep.name != self.name && ws_names.contains(dep.name.as_str()))
            .collect()
    }

    /// Classifies how this package depends on `dep`.
    ///
    /// An optional normal dependency lists the features of this package that enable it. The
    /// `optional` field of the dependency is not enough because it does not say what enables it.
    #[must_use]
    pub fn edge_kind(&self, dep: &Dep) -> EdgeKind {
        match dep.kind.as_deref() {
            Some("dev") => EdgeKind::Dev,
            Some("build") => EdgeKind::Build,
            _ if dep.optional => EdgeKind::Optional(self.features_enabling(dep)),
            _ => EdgeKind::Normal,
        }
    }

    /// The sorted names of the features that enable the optional dependency `dep`.
    fn features_enabling(&self, dep: &Dep) -> Vec<String> {
        let key = dep.rename.as_deref().unwrap_or(&dep.name);
        let explicit = format!("dep:{key}");
        let with_feature = format!("{key}/");

        self.features
            .iter()
            .filter(|(_, enables)| {
                enables
                    .iter()
                    .any(|item| item == key || *item == explicit || item.starts_with(&with_feature))
            })
            .map(|(feature, _)| feature.clone())
            .collect()
    }

    /// The name of the target other crates import: the first library, `rlib`, `dylib` or
    /// procedural macro target.
    fn library_target_name(&self) -> Option<&str> {
        const IMPORTABLE_KINDS: [&str; 4] = ["lib", "rlib", "dylib", "proc-macro"];

        self.targets
            .iter()
            .find(|target| target.kind.iter().any(|kind| IMPORTABLE_KINDS.contains(&kind.as_str())))
            .map(|target| target.name.as_str())
    }
}

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
