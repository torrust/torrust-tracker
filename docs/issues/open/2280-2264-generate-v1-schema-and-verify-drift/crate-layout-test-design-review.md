# Crate Layout Test Design Review

## Refactor Plan Item 1 - Crate-Owned Fixtures

### Arrange

Six profile tests load Markdown fixtures through `include_str!` from an archived issue folder
under `docs/issues/closed/`.

### Act

Move the fixture directory, including its manifest, to
`contrib/dev-tools/checks/frontmatter-validator/fixtures/` and update the six paths to
`../fixtures/...` relative to `src/profile.rs`.

### Assert

The same 44 library tests and 24 binary tests pass with identical fixture bytes, and `linter all`
accepts the new location with no ignore-file changes.

### Review

The fixtures are unchanged, so no new assertion is warranted. The proof is that compilation no
longer depends on any path under `docs/issues/`, which `grep` on `include_str!` confirms.

## Refactor Plan Item 2 - Diagnostic Module

### Arrange

`Diagnostic` and `DiagnosticCategory` are declared in `lib.rs` with doc comments that describe
them as extraction failures, although six categories are produced only by `profile.rs`.

### Act

Move both types and `Diagnostic::new` to `diagnostic.rs`, re-export them from the crate root, and
correct the doc comments to cover extraction and validation.

### Assert

All 44 library tests and 24 binary tests pass unchanged through the re-exported paths, the tracked
schema is byte-identical, and `cargo +nightly doc` resolves the new intra-doc links without
warnings.

### Review

This is a declaration move with corrected documentation and no behavior change. Every existing
test already asserts on a `DiagnosticCategory` value through the crate-root path, so the unchanged
suite is the proof that the public surface did not move.

## Refactor Plan Item 3 - Frontmatter Scalar-Style Query

### Arrange

Four Markdown documents write the same top-level field with a double-quoted value, a plain value, a
double-quoted value followed by a tab-separated YAML comment, and a double-quoted value under a
field name that merely shares the queried prefix.

### Act

Extract each document and ask `Frontmatter::has_double_quoted_scalar("last-updated-utc")`.

### Assert

Only the two exact-field, double-quoted rows return `true`; the plain scalar and the
prefix-sharing field return `false`.

### Review

The query moves scalar-style detection to the module that owns the YAML source and makes `yaml`
private; `Issue` and `Epic` now ask a named question instead of receiving source text. The strict
profile sequence takes `&Frontmatter` so three loose parameters collapse into one. Mutation:
inverting the query's quote check made seven tests fail, including the new direct test and
`it_should_reject_an_unquoted_strict_timestamp`, before the canonical check was restored.

## Refactor Plan Item 4 - Syntax Module

### Arrange

The canonical model declares four schema regex patterns, two of them still as inline attribute
literals, and ten dependency-free predicates that the Rust validator treats as authoritative for
the same languages.

### Act

Move the predicates and all four constants into `syntax.rs`, declaring each constant directly
above its predicate, reference the two newly named constants from the `SkillName` and
`RelatedArtifact` attributes, and move the UTC table test with the code it exercises.

### Assert

The schema projection test asserts the generated `SkillName` and `RelatedArtifact` patterns equal
the constants, offline drift verification reports the tracked artifact byte-identical, and the
library suite stays at 45 tests with the UTC table test now under `syntax::tests`.

### Review

`schemars_derive` 1.2.1 passes `extend(...)` values through `serde_json::json!`, so a constant
expression is a supported input; this was verified in the derive source before the change. Clippy's
`redundant_pub_crate` required plain `pub` inside the private module. Mutation: pointing the
`SkillName` attribute at `UTC_MINUTE_PATTERN` made the schema test fail on the `SkillName` pattern
assertion, and the strict `-D unused` build rejected the now-unused import, before the canonical
constant was restored.

## Refactor Plan Item 5 - Single Reference-Validation Path

### Arrange

The five existing reference-syntax rejection tests (invalid skill name, consecutive hyphens,
unapproved tagged artifact, backslash artifact path, and the predecessor's invalid-reference
fixture) each supply one bad `semantic-links` entry inside an otherwise valid strict issue.

### Act

After structural validation, deserialize `values["semantic-links"]` into `StrictSemanticLinks`
and map its failure to `InvalidReferenceSyntax`; delete `validate_reference_syntax` over the
universal envelope and the `Option<&SemanticLinks>` parameter it needed.

### Assert

All five tests still report `InvalidReferenceSyntax`, precedence after `UnknownField` is unchanged
(`it_should_prioritize_unknown_fields_before_reference_syntax_for_strict_profiles`), and the
tracked schema is byte-identical.

### Review

The `TryFrom<String>` impls on `SkillName` and `RelatedArtifact` are now the only reference-syntax
gate, so the newtypes' error branches are reachable through `validate`. The
`it_should_report_a_missing_envelope_instead_of_panicking` test was deleted: it constructed a
`Frontmatter` whose `values` held `semantic-links` while `semantic_links` was `None`, a state that
only the second path could observe. `profile.rs` no longer reads `Frontmatter::semantic_links`;
the universal envelope remains an extraction output for the future command adapter.

Message wording changed. `serde_yaml::from_value` attaches no field path, so the diagnostic now
reads `` `semantic-links` contains an invalid v1 reference: `Write-Markdown-Docs` must match
[a-z0-9]+(-[a-z0-9]+)*. `` instead of naming `skill-links` or `related-artifacts`; the field
prefix was added explicitly so the diagnostic keeps naming the offending mapping. Mutation: mapping
the failure to `WrongScalarType` made exactly the five reference-syntax tests fail before the
canonical category was restored.

### Correction - Field-Scoped Reference Diagnostics

**Arrange**: The invalid-skill-name and unapproved-tagged-artifact tests already supply one bad
entry under a known `semantic-links` field.

**Act**: Deserialize `skill-links` and `related-artifacts` separately through `Vec<SkillName>` and
`Vec<RelatedArtifact>`, prefixing the failure with `semantic-links.<field>`.

**Assert**: Both tests additionally assert the message starts with the dotted field path; all 44
library tests and 24 binary tests pass and the schema is byte-identical.

**Review**: The single typed path is kept; only the granularity of the deserialization call changed
so the field name is known at the failure point. Mutation: swapping the two field literals failed
five tests, including both extended assertions, before the mapping was restored.

## Refactor Plan Item 6 - Top-Down Ordering

### Arrange

After items 2-5, `profile.rs` still declares `validate` between private types and helpers, and
`StrictProfileKind::definition` references profile constants declared hundreds of lines later.

### Act

Reorder the 42 top-level items only: public surface, `validate` and its shared sequence, kind
recognition, definitions with the structural checks they apply, models with their enums and
invariants, envelope newtypes, then field-level helpers and tests.

### Assert

`git diff --stat` shows an equal number of inserted and deleted lines, the sorted non-blank lines
are unchanged, rustfmt makes no further edits, and all 44 library and 24 binary tests, the drift
check, Clippy, and rustdoc pass.

### Review

A scripted permutation with a lossless round-trip check was used so that no item text could
change during the move. No test was added: the change has no behavior to observe beyond the
existing suite continuing to compile and pass.
