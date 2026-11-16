---
schema-version: 1
doc-type: issue
issue-type: bug
status: draft
priority: p2
epic: 2392
github-issue: null
spec-path: docs/issues/drafts/2392-remove-leaked-test-temporary-files/ISSUE.md
branch: "{issue-number}-2392-remove-leaked-test-temporary-files"
related-pr: null
last-updated-utc: "2026-11-16 17:30"
semantic-links:
  skill-links:
    - create-issue
    - fix-bug
    - write-unit-test
  related-artifacts:
    - .github/skills/dev/debugging/fix-bug/SKILL.md
    - .github/skills/dev/planning/create-issue/SKILL.md
    - docs/issues/open/2392-test-isolation/EPIC.md
    - docs/issues/open/2394-2392-audit-shared-test-resources/inventory.md
    - packages/test-helpers/src/configuration.rs
    - packages/configuration/src/v2_0_0/mod.rs
    - packages/configuration/src/v3_0_0/mod.rs
    - packages/axum-server/src/tls.rs
---

<!-- skill-link: create-issue -->

# Issue #[To be assigned] - Remove the Temporary Files Tests Leave Behind

Parent EPIC: #2392 - Test Isolation (`docs/issues/open/2392-test-isolation/EPIC.md`)

## Goal

Tests remove every temporary file they create, on success, failure and drop paths (EPIC rule 1).
A test run leaves the system temporary directory as it found it.

## Background

The shared-resource audit (#2394, finding F2 in its `inventory.md`) found three test helpers that
write files to `std::env::temp_dir()` with unique names and never remove them:

| Helper | File | Removed |
| --- | --- | --- |
| `torrust_tracker_test_helpers::configuration::ephemeral_sqlite_database` (used by every `ephemeral*` configuration and the `tracker-core` SQLite tests) | `data_<16 random characters>.db` | Never |
| The configuration-file tests in `packages/configuration/src/{v2_0_0,v3_0_0}/mod.rs` | `test_config_<uuid>.toml` | Never |
| `make_temp_file` in `packages/axum-server/src/tls.rs` | `<prefix>-<nanoseconds>.pem` | Only when the test passes |

The names are unique, so no two tests or runs collide; the problem is accumulation. On the
maintainer's development machine on 2026-11-16, `/tmp` held 232,229 `data_*.db` files (about
10.6 GiB, all created since 2026-11-13) and 1,590 `test_config_*.toml` files. Several agents run
the full suite many times a day, so the directory grows until the disk or the inode table fills,
and then unrelated tests fail.

## Scope

### In Scope

- Make each helper's file owned by a guard that removes it on drop (for example a
  `tempfile::TempDir` or `NamedTempFile` kept alive as long as the configuration or test that uses
  the path).
- Adjust the callers of `ephemeral_sqlite_database` and the `ephemeral*` configurations so the
  guard outlives the tracker that opens the database.

### Out of Scope

- Deleting the files already leaked on any machine (a one-off cleanup for the machine's owner).
- Examples that use fixed temporary paths; no test runs them (#2394 F7).
- Any other resource class.

## Architectural Decisions

- Related ADRs: none.
- ADRs to create: none expected. Changing the return type of public `torrust-tracker-test-helpers`
  functions is a test-support API change; record the chosen shape in this spec.

## Design and Ownership Review

The `ephemeral*` functions return a `Configuration` that carries only a path string, and many
callers (51 files) keep the configuration, not a guard. Decide before implementation:

- who owns the guard: the configuration factory's caller, a wrapper type returned with the
  configuration, or the test environment that starts the tracker;
- that the guard is dropped only after the tracker and its database handles are dropped;
- the failure path: a test that panics still drops the guard.

## Bug-Fix Process

The defect is in test-support code. Follow `.github/skills/dev/debugging/fix-bug/SKILL.md`:

1. Real-artifact reproduction: count `data_*.db` and `test_config_*.toml` in an empty temporary
   directory (`TMPDIR`) before and after one `cargo test --workspace`.
2. Regression boundary: unit tests of each helper that assert the file is gone after the guard is
   dropped.
3. Red before the change, green after; final like-for-like recheck of step 1.

## Regression Test Strategy

One unit test per helper: create the file through the helper, drop the guard, assert the path no
longer exists. The step-1 count is the end-to-end recheck.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID | Status | Task | Notes / Expected Output |
| --- | --- | --- | --- |
| T1 | TODO | Reproduce with a fresh `TMPDIR` | Before-counts recorded (V1). |
| T2 | TODO | Decide the guard ownership | Design recorded in this spec, approved by the maintainer. |
| T3 | TODO | SQLite helper and its callers | Unit test red then green. |
| T4 | TODO | Configuration-file tests | Files removed on drop. |
| T5 | TODO | TLS test files | Files removed on failure too. |
| T6 | TODO | Recheck | Fresh `TMPDIR` is empty after the suite (V2); `linter all` passes. |

## Commit Points

| Task | Coherent change set | Commit policy |
| --- | --- | --- |
| T1-T2 | Evidence and design | Documentation-only commit after maintainer review. |
| T3 | SQLite helper and callers | Commit after focused validation and review. |
| T4 | Configuration-file tests | Commit after focused validation and review. |
| T5 | TLS test files | Commit after focused validation and review. |
| T6 | Recheck evidence | Documentation-only commit. |

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/drafts/2392-remove-leaked-test-temporary-files/ISSUE.md`
- [ ] Spec reviewed and approved by user/maintainer
- [ ] GitHub issue created and issue number added to this spec
- [ ] Implementation completed
- [ ] Automatic verification completed (`linter all`, relevant tests, and any pre-push checks)
- [ ] Manual verification scenarios executed and recorded in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] Evidence-based implementation completion review recorded
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-11-16 17:30 UTC - Claude Code (Claude Opus 5.5), #2394 session - Drafted from audit finding
  F2 after the maintainer classified it as a new problem; counted the leaked files in `/tmp`.

## Acceptance Criteria

- [ ] AC1: After `cargo test --workspace` with a fresh `TMPDIR`, no `data_*.db`,
  `test_config_*.toml` or test PEM file remains.
- [ ] AC2: Each helper has a unit test proving its file is removed when the guard is dropped.
- [ ] AC3: A failing TLS test also removes its files.
- [ ] `linter all` exits with code `0`

## Verification Plan

### Automatic Checks

- `linter all`
- `cargo test --workspace`

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID | Scenario | Human-oriented command/steps | Expected Result | Status | Evidence |
| --- | --- | --- | --- | --- | --- |
| V1 | Reproduce | `TMPDIR=$(mktemp -d) cargo test --workspace`, then count the files | Leaked files counted | TODO | `manual-verification-evidence.md` |
| V2 | Recheck | Same command after the change | No leaked files | TODO | `manual-verification-evidence.md` |

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| --- | --- | --- |
| AC1 | TODO | V2 |
| AC2 | TODO | Unit tests |
| AC3 | TODO | Unit test |

## Risks and Trade-offs

- **API churn.** Many callers use the `ephemeral*` configurations; returning a guard touches all of
  them. A wrapper that keeps the guard next to the configuration may limit the change.
- **Early drop.** A guard dropped before the tracker stops would delete an open database; SQLite on
  Linux keeps working on an unlinked file, which can hide the mistake. The unit tests must check
  the order.

## Implementation Completion Review

- Retrospective: `Not yet assessed`

## References

- Parent EPIC: #2392
- Audit: #2394, `docs/issues/open/2394-2392-audit-shared-test-resources/inventory.md` (F2)
