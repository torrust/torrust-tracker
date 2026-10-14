---
schema-version: 1
doc-type: issue
issue-type: task
status: planned
priority: p3
epic: 1669
github-issue: 2517
spec-path: docs/issues/open/2517-1669-encapsulate-statistics-repository-state/ISSUE.md
branch: "2517-1669-encapsulate-statistics-repository-state"
related-pr: null
last-updated-utc: "2026-10-14 12:45"
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - .github/skills/dev/planning/create-issue/SKILL.md
    - "issue #1669"
    - docs/adrs/20260629000000_adopt_independent_package_versioning.md
    - packages/udp-core/src/statistics/repository.rs
    - packages/udp-server/src/statistics/repository.rs
    - packages/http-core/src/statistics/repository.rs
    - packages/tracker-core/src/statistics/repository.rs
    - packages/swarm-coordination-registry/src/statistics/repository.rs
---

<!-- skill-link: create-issue -->

# Issue #2517 - Make the Statistics Repositories' Metrics Lock Private

Parent EPIC: #1669 (Overhaul: Packages).

## Goal

Stop exposing the `stats: Arc<RwLock<Metrics>>` field of the five statistics repositories, so every
read and write goes through the repository's methods. This is a visibility-only change: the
public read accessors keep returning a lock read guard (see Out of Scope).

## Background

Five packages define a statistics `Repository` with the same public field:

```rust
pub struct Repository {
    pub stats: Arc<RwLock<Metrics>>,
}
```

| Package                       | File                                                                |
| ----------------------------- | ------------------------------------------------------------------- |
| `udp-core`                    | `packages/udp-core/src/statistics/repository.rs`                    |
| `udp-server`                  | `packages/udp-server/src/statistics/repository.rs`                  |
| `http-core`                   | `packages/http-core/src/statistics/repository.rs`                   |
| `tracker-core`                | `packages/tracker-core/src/statistics/repository.rs`                |
| `swarm-coordination-registry` | `packages/swarm-coordination-registry/src/statistics/repository.rs` |

The repositories already offer the operations callers need: a read guard and write methods that
take the write lock and release it.

| Package                       | Read guard      | Write methods                                                                       |
| ----------------------------- | --------------- | ----------------------------------------------------------------------------------- |
| `udp-core`                    | `get_stats()`   | `increase_counter()`                                                                |
| `udp-server`                  | `get_stats()`   | `increase_counter()`, `set_gauge()`, `recalculate_udp_avg_processing_time_ns()`     |
| `http-core`                   | `get_stats()`   | `increase_counter()`                                                                |
| `tracker-core`                | `get_metrics()` | `increment_counter()`, `set_counter()`, `set_gauge()`, `increment_gauge()`, `decrement_gauge()` |
| `swarm-coordination-registry` | `get_metrics()` | `increment_counter()`, `set_gauge()`, `increment_gauge()`, `decrement_gauge()`      |

The public field lets any caller take the write lock directly and bypass those methods. It also
makes the field, with its `Arc` sharing and its lock, part of each crate's public API, so removing
it after a publish is a breaking change. This matters for EPIC #1669's goal of independently
versioned, publishable packages.

Making the field private does not remove the lock type from the public API entirely: the read
accessors return `tokio::sync::RwLockReadGuard<'_, Metrics>`, and callers outside the five crates
rely on them (for example, `rest-api-runtime-adapter` merges each repository's
`metric_collection`, and the `axum-http-server` contract tests read the HTTP counters through
`get_stats()`).
Hiding the lock completely needs a read-side redesign and is out of scope here.

### Evidence

Re-verified on `develop` at `395cd3ca0` (2026-10-14), after a first check at `48e11787f`:

- A search for `.stats` in all Rust files under `packages/`, `src/`, `tests/`, `console/`, and
  `contrib/` finds the field used only inside the five repository files. Every other match is a
  different type (`announce_data.stats`, `domain_data.stats`, `data.stats`) or a method
  (`run.stats()`).
- No code outside the five files builds a `Repository` with a struct literal
  (`Repository { stats: ... }`); all callers use `Repository::new(...)`.
- No same-crate caller outside the repository file uses the field, so `pub(crate)` is not needed.

### Semver impact

None of the five crates is published on crates.io under its current name
(`torrust-tracker-udp-core`, `torrust-tracker-udp-server`, `torrust-tracker-http-core`,
`torrust-tracker-core`, `torrust-tracker-swarm-coordination-registry`), nor under its former
names found in git history (`bittorrent-udp-tracker-core`, `torrust-tracker-udp-tracker-core`,
`torrust-udp-tracker-server`, `bittorrent-http-tracker-core`,
`torrust-tracker-http-tracker-core`, `bittorrent-tracker-core`). The only published predecessor,
`torrust-tracker-torrent-repository` v3.0.0 (2024-10-03, the former name of
`swarm-coordination-registry`), has no statistics module. Making the field private therefore breaks
no published API. Doing it before the first publish avoids a later semver-major bump for this
field. The read guards remain public API and keep their own semver exposure.

### Origin

Found while adding tests to `udp-core` (#1348, branch `1348-1347-add-tests-udp-core`, not yet
merged). That branch's statistics repository test plan writes through `stats` from the test
module inside the same file, which keeps access to a private field.

## Scope

### In Scope

- Make `stats` private in the five repositories.
- Add one `compile_fail,E0616` doctest per repository that pins the field as private (see
  Regression Test Strategy).
- Adapt any test, bench, or example that breaks; prefer the repository methods.

### Out of Scope

- Changing the repositories' method signatures or the `Metrics` types.
- Removing the lock type from the public read accessors (`get_stats()` / `get_metrics()` return
  `RwLockReadGuard<'_, Metrics>`). A possible follow-up: replace the guards with per-metric
  accessors or a cheap snapshot. Cloning the whole `Metrics` on every read is not acceptable on
  event-handler paths, so this needs its own design. File it as a separate #1669 sub-issue if the
  maintainer wants it before the first publish.
- Renaming the field.
- Unifying the five repositories into a shared generic type.
- Removing the unused SI-30 statistics traits (`UdpCoreStatsRepository`,
  `UdpServerStatsRepository`) in `udp-core` and `udp-server`. A separate #1669 sub-issue draft for
  them exists but is not committed yet; the only in-tree note on their possible leftover status is
  the Out of Scope of the #2505 spec (`docs/issues/open/2505-udp-banned-ips-metric/ISSUE.md`).
- Changelog entries: none of the five packages, nor the repository root, keeps a changelog.

## Architectural Decisions

- Related ADRs: `docs/adrs/20260629000000_adopt_independent_package_versioning.md` (the field is
  public API; removing it after a publish would be breaking).
- ADRs to create: None known.

## Design and Ownership Review

Not applicable: visibility change only; lock ownership stays inside each repository.

## Bug-Fix Process

Not applicable: not a bug.

## Regression Test Strategy

Not a bug, so there is no defect to reproduce.

The change is still guarded by a test. A doctest compiles as a separate crate, so a
`rust,compile_fail,E0616` doctest on each `Repository` that reads `repository.stats` from outside
the crate fails to compile only while the field is private. This tree already uses the pattern in
one of the five crates: the `UdpTrackerCoreServices` docs in `packages/udp-core/src/container.rs`
pin the private `cookie_cipher` field. For example, for `udp-core`:

```rust,ignore
/// ```rust,compile_fail,E0616
/// use torrust_tracker_udp_core::statistics::repository::Repository;
///
/// fn lock_of(repository: &Repository) {
///     let _lock = repository.stats.clone();
/// }
/// ```
```

Red/green: added before the field becomes private, the doctest fails (the snippet compiles); after
the change it passes with error `E0616`. It runs under the existing `cargo test --doc --workspace`.
It does not catch `pub(crate)` or `pub(super)`, which compile only inside the crate; the AC1 grep
covers those.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task                                                | Notes / Expected Output                                                                                     |
| --- | ------ | --------------------------------------------------- | ----------------------------------------------------------------------------------------------------------- |
| T1  | TODO   | Re-verify the evidence on current `develop`         | Repeat the `.stats` search and crates.io check; record the commit in the progress log.                     |
| T2  | TODO   | Make `stats` private in `udp-core`                  | `compile_fail,E0616` doctest on `Repository` (red first); `cargo test -p torrust-tracker-udp-core` passes, including doctests. |
| T3  | TODO   | Make `stats` private in `udp-server`                | `compile_fail,E0616` doctest on `Repository` (red first); `cargo test -p torrust-tracker-udp-server` passes, including doctests. |
| T4  | TODO   | Make `stats` private in `http-core`                 | `compile_fail,E0616` doctest on `Repository` (red first); `cargo test -p torrust-tracker-http-core` passes, including doctests. |
| T5  | TODO   | Make `stats` private in `tracker-core`              | `compile_fail,E0616` doctest on `Repository` (red first); `cargo test -p torrust-tracker-core` passes, including doctests. |
| T6  | TODO   | Make `stats` private in `swarm-coordination-registry` | `compile_fail,E0616` doctest on `Repository` (red first); `cargo test -p torrust-tracker-swarm-coordination-registry` passes, including doctests. |
| T7  | TODO   | Verification                                        | See Verification Plan.                                                                                      |

## Commit Points

| Task  | Coherent change set                                                                       | Commit policy                    |
| ----- | ----------------------------------------------------------------------------------------- | -------------------------------- |
| T2-T6 | `refactor(<package>): [#2517] make the statistics repository lock private`, one per package, each with its `compile_fail` doctest | Commit after focused validation and the test-design review. |
| T7    | Manual verification evidence and spec progress updates                                    | Commit after verification.       |

Every commit is GPG signed. Write each doctest with the `write-unit-test` skill and record a short
design review in the task evidence before committing: the snippet must fail only because of the
private field (the pinned `E0616`), with no unrelated reason to fail compilation.

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/drafts/1669-encapsulate-statistics-repository-state/ISSUE.md`
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue created and issue number added to this spec
- [x] Spec-only PR merged into `develop` before implementation
- [ ] Implementation completed
- [ ] Automatic verification completed (`linter all`, relevant tests, and any pre-push checks)
- [ ] Manual verification scenarios executed and recorded in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] Evidence-based implementation completion review recorded: issue-local retrospective created for material discoveries, or progress log states why none was needed
- [ ] Reviewer validated acceptance criteria and updated checkboxes
- [ ] Independent reviewer reports recorded in issue-local `agent-review-reports.md` when reviewers received this folder-style specification
- [ ] Committer verified spec progress is up to date before commit
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-10-14 06:50 UTC - GitHub Copilot (#1348 session) - Drafted from the `udp-core` statistics
  repository test plan finding; maintainer asked for it as an EPIC #1669 sub-issue.
- 2026-10-14 07:11 UTC - GitHub Copilot (spec session) - Took over the draft. Re-verified the
  evidence on `develop` at `395cd3ca0`, checked crates.io for current and former crate names
  (none published with the field), split T2 into one task per package, and completed the
  remaining template sections.
- 2026-10-14 07:15 UTC - GitHub Copilot (spec session) - Maintainer approved the spec. Created
  #2517, linked it as a sub-issue of #1669, moved the spec to `docs/issues/open/`, and added it
  to the EPIC quick list and specs index on branch
  `2517-1669-encapsulate-statistics-repository-state-spec`.
- 2026-10-14 08:14 UTC - GitHub Copilot (spec session) - PR #2520 review: narrowed the goal to a
  visibility-only change (the public read accessors still return `RwLockReadGuard`), recorded the
  read-side redesign as a possible follow-up, and cited EPIC #1669 as `issue #1669` in the
  frontmatter.
- 2026-10-14 09:06 UTC - GitHub Copilot (spec session) - PR #2520 review round 2: planned one
  `compile_fail,E0616` doctest per repository, made the AC1 grep catch restricted visibility, named
  the AC2 checks, and said the SI-30 traits draft is not committed yet.
- 2026-10-14 12:45 UTC - Maintainer - Confirmed that M1 should be performed against both the
  `develop` baseline and the implementation branch, comparing UDP and HTTP announce metrics through
  the REST API.

## Acceptance Criteria

- [ ] AC1: None of the five statistics repositories has a `pub` metrics lock field.
- [ ] AC2: Code outside each repository file reads and writes metrics only through the
  repository's methods.
- [ ] AC3: Metric values reported by the REST API are unchanged for the same announces.
- [ ] `linter all` exits with code `0`
- [ ] Relevant tests pass
- [ ] Manual verification scenarios are executed and documented in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria are re-reviewed after implementation and reflect actual behavior
- [ ] Documentation is updated when behavior/workflow changes

## Verification Plan

### Automatic Checks

- `linter all`
- `cargo test --tests --benches --examples --workspace --all-targets --all-features` (stable Rust
  toolchain)
- `cargo test --doc --workspace` (stable Rust toolchain)
- Pre-push checks (`./contrib/dev-tools/git/hooks/pre-push.sh`)
- AC1 check: `grep -nE 'pub(\([^)]*\))? +stats *:' packages/*/src/statistics/repository.rs`
  prints nothing. The pattern also matches `pub(crate)` and `pub(super)`, so it passes only for a
  private field.
- AC2 checks: the workspace builds and all tests pass with the field private (no caller outside a
  repository file can compile against it), and the five `compile_fail,E0616` doctests pass under
  `cargo test --doc --workspace`.

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID  | Scenario             | Human-oriented command/steps                                                                                                                                                         | Expected Result                                                                                      | Status | Evidence                                     |
| --- | -------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | ---------------------------------------------------------------------------------------------------- | ------ | -------------------------------------------- |
| M1  | Metrics still update | Run the tracker with the default development config (`run-tracker-locally` skill), send one UDP and one HTTP announce with `tracker_client`, then `curl` the REST API stats and metrics endpoints | The UDP, HTTP, tracker-core, and swarm counters reflect the announces, as they do on `develop` | TODO   | `manual-verification-evidence.md` section V1 |

Notes:

- Run M1 once on `develop` before the change and once after, and compare the reported values.
- Create `manual-verification-evidence.md` from `docs/templates/MANUAL-VERIFICATION-EVIDENCE.md`
  and record the actual commands, output, and relevant tracker logs.

### Disposable Verification Scripts

None planned.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence           |
| ----- | ---------------------- | ------------------ |
| AC1   | TODO                   | {test/log/PR link} |
| AC2   | TODO                   | {test/log/PR link} |
| AC3   | TODO                   | {test/log/PR link} |

## Risks and Trade-offs

- **Merge conflicts with #1348**: the unmerged branch `1348-1347-add-tests-udp-core` edits
  `packages/udp-core/src/statistics/repository.rs`. Its test lives in the file's own test module,
  so it keeps compiling after the change. Rebase on whichever lands second.
- **Overlap with the SI-30 statistics traits work**: its draft (not committed yet; see Out of
  Scope) touches the same `udp-core` and `udp-server` files. Keep each change in its own issue and
  rebase as needed.
- **Hidden future need for direct lock access**: if a caller later needs an operation the
  methods do not offer, add a repository method instead of reopening the field.

## Implementation Completion Review

After implementation, compare the result with this specification. Record
invalidated assumptions, material design changes, unexpected validation
findings, and reusable lessons.

- Retrospective: `Not yet assessed`
- If needed, create `implementation-retrospective.md` from the repository
  template at `docs/templates/IMPLEMENTATION-RETROSPECTIVE.md` in this issue
  specification's directory.
- If no retrospective is needed, add a concise progress-log entry explaining
  why the work had no material discovery.
- When an independent reviewer receives this folder-style specification, it
  records its result in `agent-review-reports.md` using
  `docs/templates/AGENT-REVIEW-REPORTS.md`.

## References

- Parent EPIC: #1669
- Related issues: #1348 (origin of the finding)
- Related ADRs: `docs/adrs/20260629000000_adopt_independent_package_versioning.md`
