# Profile Test Design Review

## Refactor Plan Item 1 - Module-Owned Profile Tests

### Arrange

The existing test suite separates into extraction-owned tests in `lib.rs` and schema/profiling
tests whose production decisions live in `profile.rs`.

### Act

Move the schema-projection and strict-profile tests into `profile.rs` without changing fixture
inputs, Acts, or assertions.

### Assert

The package still runs 41 library tests. Test paths show schema and strict-profile behavior under
`profile::tests`, while delimiter, YAML, mapping-root, and universal-envelope behavior remains
under `lib.rs` tests.

### Review

This reorganization changes ownership visibility only. Existing tests already state the causal
document fixtures and observable diagnostic categories, so adding parallel coverage would create
duplication rather than confidence. The focused package run proves each test still compiles at its
new private-module boundary.
