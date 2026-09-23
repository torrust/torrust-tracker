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
