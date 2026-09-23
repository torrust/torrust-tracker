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
