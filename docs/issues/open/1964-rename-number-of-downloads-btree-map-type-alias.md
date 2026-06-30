---
doc-type: issue
issue-type: task
status: planned
priority: p2
github-issue: 1964
spec-path: docs/issues/open/1964-rename-number-of-downloads-btree-map-type-alias.md
branch: "1964-rename-number-of-downloads-btree-map"
related-pr: null
last-updated-utc: 2026-06-30 12:00
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

<!-- skill-link: create-issue -->

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
| T1  | TODO   | Rename definition in primitives crate | Change `NumberOfDownloadsBTreeMap` to `NumberOfDownloadsPerInfoHash` in `packages/primitives/src/lib.rs` |
| T2  | TODO   | Update core domain references         | Update imports/usages in `tracker-core`, `swarm-coordination-registry`, etc.                             |
| T3  | TODO   | Update benchmarking references        | Update imports/usages in `torrent-repository-benchmarking` crate and tests                               |
| T4  | TODO   | Update documentation                  | Update the 4 doc files referencing the old name                                                          |
| T5  | TODO   | Run full verification                 | `linter all`, `cargo test --workspace`, pre-commit checks                                                |

## Progress Tracking

### Workflow Checkpoints

- [x] Spec drafted in `docs/issues/drafts/`
- [ ] Spec reviewed and approved by user/maintainer
- [ ] GitHub issue created and issue number added to this spec
- [ ] (Optional, recommended for complex issues) Spec-only PR merged into `develop` before implementation
- [ ] Implementation completed
- [ ] Automatic verification completed (`linter all`, relevant tests, and any pre-push checks)
- [ ] Manual verification scenarios executed and recorded (status + evidence)
- [ ] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] Reviewer validated acceptance criteria and updated checkboxes
- [ ] Committer verified spec progress is up to date before commit
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-06-30 12:00 UTC - Copilot - Spec draft created

## Acceptance Criteria

- [ ] AC1: `NumberOfDownloadsBTreeMap` no longer appears anywhere in the codebase
- [ ] AC2: `NumberOfDownloadsPerInfoHash` is the sole name for the type alias
- [ ] AC3: All tests pass (`cargo test --workspace`)
- [ ] AC4: `linter all` exits with code `0`
- [ ] AC5: Pre-commit checks pass
- [ ] Manual verification scenarios are executed and documented (status + evidence)
- [ ] Acceptance criteria are re-reviewed after implementation and reflect actual behavior

## Verification Plan

### Automatic Checks

- `linter all`
- `cargo test --workspace`
- Pre-commit checks (`./contrib/dev-tools/git/hooks/pre-commit.sh`)

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID  | Scenario                    | Command/Steps                                                           | Expected Result                            | Status | Evidence                     |
| --- | --------------------------- | ----------------------------------------------------------------------- | ------------------------------------------ | ------ | ---------------------------- |
| M1  | Build succeeds after rename | `cargo build --workspace`                                               | Zero errors, no warnings related to rename | TODO   | {log/output/screenshot/path} |
| M2  | grep confirms no old name   | `grep -r "NumberOfDownloadsBTreeMap" --include="*.rs" --include="*.md"` | No matches found                           | TODO   | {log/output/screenshot/path} |

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence           |
| ----- | ---------------------- | ------------------ |
| AC1   | TODO                   | {test/log/PR link} |
| AC2   | TODO                   | {test/log/PR link} |
| AC3   | TODO                   | {test/log/PR link} |
| AC4   | TODO                   | {test/log/PR link} |
| AC5   | TODO                   | {test/log/PR link} |

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
