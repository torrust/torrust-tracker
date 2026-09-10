//! Validation primitives for documented Clippy `allow` attributes.

use std::collections::BTreeSet;

use syn::parse::Parser as _;
use syn::punctuated::Punctuated;
use syn::spanned::Spanned as _;
use syn::visit::{self, Visit};
use syn::{Attribute, Meta, Token};

/// A validation error for a changed Clippy `allow` attribute.
#[derive(Debug, Eq, PartialEq)]
pub struct Violation {
    /// The one-based source line containing the relevant attribute.
    pub line: usize,
    /// The actionable reason for rejecting the attribute.
    pub message: &'static str,
}

/// Validates changed Clippy `allow` attributes in a Rust source file.
///
/// Only attributes whose source span overlaps a line in `changed_lines` are checked. This lets a
/// caller enforce a prospective policy without requiring an inventory of historical attributes.
///
/// # Errors
///
/// Returns a [`syn::Error`] when `source` is not valid Rust syntax.
pub fn validate_changed_allows(source: &str, changed_lines: &BTreeSet<usize>) -> Result<Vec<Violation>, syn::Error> {
    let file = syn::parse_file(source)?;
    let mut visitor = AllowVisitor {
        changed_lines,
        violations: Vec::new(),
    };

    visitor.visit_file(&file);
    Ok(visitor.violations)
}

struct AllowVisitor<'a> {
    changed_lines: &'a BTreeSet<usize>,
    violations: Vec<Violation>,
}

impl<'ast> Visit<'ast> for AllowVisitor<'_> {
    fn visit_attribute(&mut self, attribute: &'ast Attribute) {
        self.validate(attribute);
        visit::visit_attribute(self, attribute);
    }
}

impl AllowVisitor<'_> {
    fn validate(&mut self, attribute: &Attribute) {
        let span = attribute.span();
        let start = span.start().line;
        let end = span.end().line;

        if self.changed_lines.range(start..=end).next().is_none() || !is_clippy_allow(attribute) {
            return;
        }

        let Some(reason) = allow_reason(attribute) else {
            self.violations.push(Violation {
                line: start,
                message: "Clippy allow attributes require `reason = \"<specific rationale>\"`.",
            });
            return;
        };

        if reason.trim().is_empty() {
            self.violations.push(Violation {
                line: start,
                message: "Clippy allow reasons must not be empty.",
            });
        } else if is_temporary(&reason) && !has_temporary_removal_information(&reason) {
            self.violations.push(Violation {
                line: start,
                message: "Temporary Clippy allow reasons require an issue reference or non-empty `remove when`, `remove after`, `remove by`, or `until` condition.",
            });
        }
    }
}

fn is_clippy_allow(attribute: &Attribute) -> bool {
    let Meta::List(list) = &attribute.meta else {
        return false;
    };

    if !list.path.is_ident("allow") {
        return false;
    }

    parse_allow_items(list.tokens.clone()).is_ok_and(|items| {
        items.iter().any(
            |item| matches!(item, Meta::Path(path) if path.segments.first().is_some_and(|segment| segment.ident == "clippy")),
        )
    })
}

fn allow_reason(attribute: &Attribute) -> Option<String> {
    let Meta::List(list) = &attribute.meta else {
        return None;
    };

    let items = parse_allow_items(list.tokens.clone()).ok()?;
    items.into_iter().find_map(|item| match item {
        Meta::NameValue(name_value) if name_value.path.is_ident("reason") => match name_value.value {
            syn::Expr::Lit(expression) => match expression.lit {
                syn::Lit::Str(reason) => Some(reason.value()),
                _ => None,
            },
            _ => None,
        },
        _ => None,
    })
}

fn parse_allow_items(tokens: proc_macro2::TokenStream) -> Result<Punctuated<Meta, Token![,]>, syn::Error> {
    Punctuated::<Meta, Token![,]>::parse_terminated.parse2(tokens)
}

fn is_temporary(reason: &str) -> bool {
    reason.to_ascii_lowercase().contains("temporary")
}

fn has_temporary_removal_information(reason: &str) -> bool {
    let normalized = reason.to_ascii_lowercase();
    let has_issue = normalized
        .match_indices('#')
        .any(|(index, _)| normalized[index + 1..].chars().next().is_some_and(char::is_numeric));
    let has_condition = ["remove when ", "remove after ", "remove by ", "until "]
        .iter()
        .any(|prefix| {
            normalized.contains(prefix)
                && normalized
                    .split(prefix)
                    .nth(1)
                    .is_some_and(|suffix| !suffix.trim().is_empty())
        });

    has_issue || has_condition
}

#[cfg(test)]
mod tests {
    use super::*;

    fn changed(lines: &[usize]) -> BTreeSet<usize> {
        lines.iter().copied().collect()
    }

    #[test]
    fn it_should_reject_a_changed_allow_without_a_reason() {
        let source = "#[allow(clippy::too_many_lines)]\nfn example() {}\n";

        let violations = validate_changed_allows(source, &changed(&[1])).unwrap();

        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].line, 1);
    }

    #[test]
    fn it_should_reject_a_changed_allow_with_an_empty_reason() {
        let source = "#[allow(clippy::too_many_lines, reason = \"\")]\nfn example() {}\n";

        let violations = validate_changed_allows(source, &changed(&[1])).unwrap();

        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].message, "Clippy allow reasons must not be empty.");
    }

    #[test]
    fn it_should_accept_a_documented_item_allow() {
        let source = "#[allow(clippy::struct_field_names, reason = \"The wire schema uses external names.\")]\nstruct Schema { field_name: String }\n";

        assert_eq!(validate_changed_allows(source, &changed(&[1])).unwrap(), [] as [Violation; 0]);
    }

    #[test]
    fn it_should_accept_a_documented_crate_allow() {
        let source = "#![allow(clippy::module_name_repetitions, reason = \"The generated compatibility module is intentionally named.\")]\n";

        assert_eq!(validate_changed_allows(source, &changed(&[1])).unwrap(), [] as [Violation; 0]);
    }

    #[test]
    fn it_should_reject_a_temporary_reason_without_removal_information() {
        let source = "#[allow(clippy::too_many_arguments, reason = \"Temporary compatibility shim.\")]\nfn example() {}\n";

        let violations = validate_changed_allows(source, &changed(&[1])).unwrap();

        assert_eq!(violations.len(), 1);
    }

    #[test]
    fn it_should_accept_a_temporary_reason_with_a_removal_condition() {
        let source = "#[allow(clippy::too_many_arguments, reason = \"Temporary compatibility shim; remove when the v4 migration completes.\")]\nfn example() {}\n";

        assert_eq!(validate_changed_allows(source, &changed(&[1])).unwrap(), [] as [Violation; 0]);
    }

    #[test]
    fn it_should_accept_a_temporary_reason_with_an_issue_reference() {
        let source =
            "#[allow(clippy::too_many_arguments, reason = \"Temporary compatibility shim; see #2158.\")]\nfn example() {}\n";

        assert_eq!(validate_changed_allows(source, &changed(&[1])).unwrap(), [] as [Violation; 0]);
    }

    #[test]
    fn it_should_ignore_an_unchanged_legacy_allow() {
        let source = "#[allow(clippy::too_many_lines)]\nfn legacy() {}\n";

        assert_eq!(validate_changed_allows(source, &changed(&[2])).unwrap(), [] as [Violation; 0]);
    }
}
