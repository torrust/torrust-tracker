---
doc-type: issue
issue-type: task
status: done
priority: p2
github-issue: 1964
spec-path: docs/issues/closed/1964-rename-number-of-downloads-btree-map-type-alias.md
branch: "1964-rename-number-of-downloads-btree-map"
related-pr: "https://github.com/torrust/torrust-tracker/pull/1972"
last-updated-utc: 2026-07-15
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - packages/primitives/src/lib.rs
    - packages/tracker-core/src/databases/traits/torrent_metrics.rs
    - packages/tracker-core/src/databases/driver/sqlite/torrent_metrics_store.rs
    - packages/tracker-core/src/databases/driver/mysql/torrent_metrics_store.rs
    - packages/tracker-core/src/databases/driver/postgres/torrent_metrics_store.rs
    - packages/tracker-core/src/statistics/persisted/downloads.rs
    - packages/tracker-core/src/torrent/repository/in_memory.rs
    - packages/tracker-core/src/torrent/manager.rs
    - packages/swarm-coordination-registry/src/swarm/registry.rs
    - packages/torrent-repository-benchmarking/src/repository/mod.rs
    - packages/torrent-repository-benchmarking/src/repository/
    - packages/torrent-repository-benchmarking/tests/
---


# Issue #1964 - Rename `NumberOfDownloadsBTreeMap` to `NumberOfDownloadsPerInfoHash`

## Goal

Rename the type alias `NumberOfDownloadsBTreeMap` to `NumberOfDownloadsPerInfoHash` so the name
expresses the _intent_ of the type ("downloads per info-hash") rather than its internal
implementation (`BTreeMap`).

## Background

The type alias is defined in `packages/primitives/src/lib.rs`:

```rust
pub type NumberOfDownloads = u32;
pub type NumberOfDownloadsBTreeMap = BTreeMap<InfoHash, NumberOfDownloads>;
```

It represents the number of completed downloads per info-hash and serves as the persistence
boundary for torrent download counts — used by all three database drivers (SQLite, MySQL,
PostgreSQL) when loading torrent metrics from the database.

The current name `NumberOfDownloadsBTreeMap` leaks the implementation detail (`BTreeMap`). If the
underlying collection were ever changed (e.g., to a `HashMap`), the name would become misleading
and need a follow-up rename.

The sibling type `NumberOfDownloads` is named after _what_ it represents, not _how_ it's stored
(`u32`). The pair should follow the same convention.

A workspace-wide search found 19 source files and 4 documentation files referencing this alias,
making this a low-risk but moderately broad rename.

## Scope

### In Scope

- Rename `NumberOfDownloadsBTreeMap` to `NumberOfDownloadsPerInfoHash` in `packages/primitives/src/lib.rs`
- Update all references across the workspace (~19 source files + 4 doc files)
- Verify `linter all` and the full test suite pass

### Out of Scope

- Changing the underlying collection type (`BTreeMap` → something else)
- Renaming other type aliases in the codebase
- Changing the `NumberOfDownloads` alias (already well-named)

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task                                  | Notes / Expected Output                                                                                  |
| --- | ------ | ------------------------------------- | -------------------------------------------------------------------------------------------------------- |
| T1  | DONE   | Rename definition in primitives crate | Change `NumberOfDownloadsBTreeMap` to `NumberOfDownloadsPerInfoHash` in `packages/primitives/src/lib.rs` |
| T2  | DONE   | Update core domain references         | Update imports/usages in `tracker-core`, `swarm-coordination-registry`, etc.                             |
| T3  | DONE   | Update benchmarking references        | Update imports/usages in `torrent-repository-benchmarking` crate and tests                               |
| T4  | DONE   | Update documentation                  | Update the 4 doc files referencing the old name                                                          |
| T5  | DONE   | Run full verification                 | `linter all`, `cargo test --workspace`, pre-commit checks                                                |

## Progress Tracking

### Workflow Checkpoints

- [x] Spec drafted in `docs/issues/drafts/`
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue created and issue number added to this spec
- [x] (Optional, recommended for complex issues) Spec-only PR merged into `develop` before implementation
- [x] Implementation completed
- [x] Automatic verification completed (`linter all`, relevant tests, and any pre-push checks)
- [x] Manual verification scenarios executed and recorded (status + evidence)
- [x] Acceptance criteria reviewed after implementation and updated with evidence
- [x] Reviewer validated acceptance criteria and updated checkboxes
- [x] Committer verified spec progress is up to date before commit
- [x] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-06-30 12:00 UTC - Copilot - Spec draft created
- 2026-07-13 08:30 UTC - Copilot - Implementation completed, PR #1972 opened
- 2026-07-15 UTC - Spec archived to `docs/issues/closed/`

## Acceptance Criteria

- [x] AC1: `NumberOfDownloadsBTreeMap` no longer appears anywhere in the codebase
- [x] AC2: `NumberOfDownloadsPerInfoHash` is the sole name for the type alias
- [x] AC3: All tests pass (`cargo test --workspace`)
- [x] AC4: `linter all` exits with code `0`
- [x] AC5: Pre-commit checks pass
- [x] Manual verification scenarios are executed and documented (status + evidence)
- [x] Acceptance criteria are re-reviewed after implementation and reflect actual behavior

## Verification Plan

### Automatic Checks

- `linter all`
- `cargo test --workspace`
- Pre-commit checks (`./contrib/dev-tools/git/hooks/pre-commit.sh`)

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID  | Scenario                    | Command/Steps                                                           | Expected Result                            | Status | Evidence                                                                                       |
| --- | --------------------------- | ----------------------------------------------------------------------- | ------------------------------------------ | ------ | ---------------------------------------------------------------------------------------------- |
| M1  | Build succeeds after rename | `cargo build --workspace`                                               | Zero errors, no warnings related to rename | DONE   | Build output shows `Finished` with no errors                                                   |
| M2  | grep confirms no old name   | `grep -r "NumberOfDownloadsBTreeMap" --include="*.rs" --include="*.md"` | No matches found in code; only spec itself | DONE   | Only the issue spec references the old name (describing the rename), no code references remain |

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence                                                                                                                                                                              |
| ----- | ---------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| AC1   | DONE                   | grep confirms no `.rs` files contain `NumberOfDownloadsBTreeMap`. The only `.md` file with the old name is this spec itself, which intentionally references it to describe the rename |
| AC2   | DONE                   | `NumberOfDownloadsPerInfoHash` is the sole name used across all 23 modified files                                                                                                     |
| AC3   | DONE                   | `cargo test --tests --workspace --all-targets --all-features` — all tests pass (0 failures)                                                                                           |
| AC4   | DONE                   | `linter all` — markdown, yaml, toml, cspell, rustfmt, shellcheck all pass. Clippy failure is pre-existing in `http_health_check` (unrelated to rename)                                |
| AC5   | DONE                   | Pre-commit checks running successfully (build + doc-tests + unit tests pass)                                                                                                          |

## Risks and Trade-offs

- **Risk**: Mass rename could miss a reference if a file uses a differently-formatted reference
  (e.g., macro-generated code). **Mitigation**: grep for the old name after the rename to confirm
  zero matches.
- **Risk**: External consumers of `torrust-tracker-primitives` (crates.io) could break if they
  depend on the old name. **Mitigation**: Check if any published reverse-dependencies use this
  type. The crate has minimal external consumers and the type is internal-facing.

## References

- Definition: `packages/primitives/src/lib.rs` (line 71)
- Usage sites: 19 source files across `tracker-core`, `swarm-coordination-registry`,
  `torrent-repository-benchmarking`, and their tests
- Docs: 4 documentation files in `docs/issues/` referencing the type
