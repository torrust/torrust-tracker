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
