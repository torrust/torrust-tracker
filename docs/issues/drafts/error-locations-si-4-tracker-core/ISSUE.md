---
schema-version: 1
doc-type: issue
issue-type: bug
status: draft
priority: p1
epic: null
github-issue: null
spec-path: docs/issues/drafts/error-locations-si-4-tracker-core/ISSUE.md
branch: "{issue-number}-error-locations-si-4-tracker-core"
related-pr: null
last-updated-utc: "2026-11-11 17:39"
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - .github/skills/dev/debugging/fix-bug/SKILL.md
    - .github/skills/dev/planning/create-issue/SKILL.md
    - docs/issues/drafts/remove-source-locations-from-errors/EPIC.md
    - docs/issues/open/2804-location-caller-track-caller-audit/inventory.md
    - packages/tracker-core/src/error.rs
    - packages/tracker-core/src/databases/error.rs
    - packages/tracker-core/src/authentication/key/mod.rs
    - packages/tracker-core/src/authentication/handler.rs
    - packages/http-core/src/services/announce.rs
    - packages/udp-server/src/handlers/error.rs
---

<!-- skill-link: create-issue -->

# Issue #[To be assigned] - Remove source locations from the errors of `torrust-tracker-core`

Parent EPIC: [Remove source locations from errors](../remove-source-locations-from-errors/EPIC.md) (draft, GitHub issue not
created yet, so `epic` is `null` and this folder carries no EPIC number). Sub-issue SI-4 of the EPIC.

## Goal

Remove the `location` fields, the `LocatedError` sources and their text from the errors of
`packages/tracker-core`, and update the packages that build or format them, so that no UDP error
message, REST API error text, HTTP failure reason or event payload produced from a tracker-core
error holds a source path.

## Background

The tracker-core errors reach three kinds of client (`docs/issues/open/2804-location-caller-track-caller-audit/inventory.md`, T2 table).

Variants that print `{location}` (7):

| File | Variants | Built at (inventory row) |
| --- | --- | --- |
| `tracker-core/src/error.rs:104` | `WhitelistError::TorrentNotWhitelisted` | `whitelist/authorization.rs:61` (P12) |
| `tracker-core/src/databases/error.rs:47`, `:56`, `:66` | `databases::error::Error::{InsertFailed, UpdateFailed, DeleteFailed}` | 12 driver sites in `databases/driver/{sqlite,mysql,postgres}/{whitelist_store,auth_key_store}.rs` (P15); no production site was found for `UpdateFailed` |
| `tracker-core/src/authentication/key/mod.rs:180`, `:187`, `:191` | `authentication::key::Error::{UnableToReadKey, KeyExpired, MissingAuthKey}` | `authentication/service.rs:98` (P13), `key/mod.rs:158` (P14), and `http-core/src/services/announce.rs:308` (P11) |

`LocatedError` uses:

- `databases/error.rs`: the `#[track_caller]` conversions `From<(SqlxError, Driver)>` (`:112`) and
  `From<(MigrateError, Driver)>` (`:139`). The location there is the driver's `map_err` line, which is
  correct for the chain and still a library path in a client reply.
- `error.rs:132`, `:138`: `PeerKeyError::InvalidKey` and `DatabaseError` sources, built with
  `Located(..).into()` at `authentication/handler.rs:103`, `:108`, `:120`, `:125`, `:141`, `:150`.
- `authentication/key/mod.rs:198`: `From<sqlx::Error>` without `#[track_caller]`.

Who reads the text:

- **UDP clients**: a non-whitelisted announce (`udp-core/src/services/announce.rs:109`, then
  `ErrorResponse { message: error.to_string() }` in `udp-server/src/handlers/error.rs:57-59`), and a
  database failure when `core.tracker_policy.persistent_torrent_completed_stat` loads downloads.
- **HTTP clients**: `"Tracker authentication error: {source}"` and
  `"Tracker whitelist error: {source}"` (`http-core/src/services/error_mapping.rs`).
- **REST API clients**: `failed to whitelist torrent: {e}`, `failed to add key: {e}` and the other
  messages built from database and `PeerKeyError` text
  (`rest-api-runtime-adapter/.../adapters/auth_key.rs:74-85` and the response modules).
- **Event consumers**: `ErrorKind::Whitelist(String)` from `source.to_string()`
  (`udp-server/src/event.rs:135`, `:145`).

`torrust-tracker-core` and the packages around it are unpublished (0.1.0), so the change adds no
release step if it lands before the first publish (#2583).

## Scope

### In Scope

- Remove `location` from the 7 variants and their formats, and from every site that builds them,
  including `http-core/src/services/announce.rs:308`.
- Replace the `LocatedError` sources in `databases::error::Error` and `PeerKeyError` with plain
  sources; remove the two `#[track_caller]` conversions' location capture and the `From<sqlx::Error>`
  one; remove the six `Located(..).into()` calls in `authentication/handler.rs`.
- Remove the `torrust-located-error` dependency from `packages/tracker-core/Cargo.toml`.
- Add a `tracing` event at the driver sites where the lost line was the only way to tell which
  query failed, if the message and the variant do not already say it.
- Update the consumers so the workspace compiles and their tests pass: `http-core`, `udp-core`,
  `udp-server` (`event.rs`, `handlers/error.rs` tests), `rest-api-runtime-adapter`.
- Decide, with evidence, whether the unused `UpdateFailed` variant stays.

### Out of Scope

- The two `udp-server` variants `Internal` and `AuthRequired` (SI-5).
- Errors defined in `http-protocol` (SI-3).
- Changing database behavior, retry policy or the REST API response shape beyond the error text.
- Whether REST and UDP replies should carry database error details at all: note it as a follow-up
  if the review finds the remaining text too detailed.

## Architectural Decisions

- Related ADRs: the ADR created by sub-issue SI-1 of the parent EPIC (errors do not record source locations); `docs/adrs/20261005145329_return_result_only_for_concretely_fallible_public_apis.md`
  (`#[non_exhaustive]` policy for public error enums).
- ADRs to create: None known

During implementation, stop and create an ADR when a decision affects project
architecture or design patterns, selects an approach among meaningful
alternatives, or has consequences future contributors need to understand. Do not
create ADRs for routine implementation details or style choices already governed
by project conventions.

## Design and Ownership Review

Not applicable: the work changes error types and their text. It adds no child process,
asynchronous I/O, network readiness, resource cleanup or reusable test fixture.

## Bug-Fix Process

Removing a location that a client can read is substantively a bug fix (misleading output), so this
issue follows `.github/skills/dev/debugging/fix-bug/SKILL.md`:

- analysis: the call paths and audiences are in the audit inventory
  (`docs/issues/open/2804-location-caller-track-caller-audit/inventory.md`, T2 table); re-check them against the tree before the first change;
- real-artifact reproduction: run a listed tracker and send a UDP announce for a non-whitelisted info-hash; send the same over HTTP; on a private tracker announce with an expired key; and call the REST API to add a key or whitelist a torrent in a way that fails in the database (for example a duplicate); record each reply;
- regression-test boundary: see Regression Test Strategy;
- red regression-test evidence, production fix, green test evidence, and the same reproduction run
  again after the last production change, all recorded in `manual-verification-evidence.md`.

## Regression Test Strategy

- **Smallest boundary**: a unit test at the error's constructor or `FromStr`/`TryFrom` seam that
  asserts the rendered `Display` holds no source path and still names what failed. It needs no
  server. Write it first and record its red output.
- **Trait-dispatch case**: where the error is built in a `FromStr` or `TryFrom` impl, call through
  `str::parse::<T>()` or `try_into()`, the call shape real callers use.
- **Portable path check**: a helper forbids these substrings: `.rs:`, `packages/`, `packages\`,
  `src/`, `src\`, `.cargo/registry` and `rustlib/src`, so workspace paths, registry checkouts, CI
  checkout paths and Windows paths are all caught.
- **Real boundary**: one test per protocol with the existing test servers (`packages/udp-server`, `packages/axum-http-server/tests`, the REST API server tests) asserting the reply holds no source path. The database case uses a driver error the tests can force (a duplicate insert).

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID | Status | Task | Notes / Expected Output |
| --- | --- | --- | --- |
| T1 | TODO | Reproduce and record | UDP, HTTP and REST replies with a library path, in `manual-verification-evidence.md`. |
| T2 | TODO | Red tests | Unit tests for the 7 variants and the `PeerKeyError`/database sources; protocol tests for UDP, HTTP and REST; red output recorded. |
| T3 | TODO | Whitelist and key errors | `location` removed from `WhitelistError` and `authentication::key::Error` and their sites, including `http-core`. |
| T4 | TODO | Database errors | `location` removed from the three variants and 12 driver sites; `LocatedError` sources replaced; `tracing` event added where needed. |
| T5 | TODO | `PeerKeyError` sources and dependency | Plain sources; six `Located(..).into()` removed; `torrust-located-error` removed from `tracker-core/Cargo.toml`. |
| T6 | TODO | Consumers | `udp-core`, `udp-server`, `http-core`, `rest-api-runtime-adapter` compile and their tests are updated. |
| T7 | TODO | Green and recheck | Green output, then the T1 requests repeated against the running tracker. |

## Commit Points

Map implementation-plan tasks that change code, tests, configuration, or documentation to small,
coherent commit opportunities. A commit point completes one independently reviewable behavior,
refactor, or evidence increment; do not group unrelated changes merely to reduce commit count.

| Task | Coherent change set | Commit policy |
| --- | --- | --- |
| T2 | The red regression tests | Commit after focused validation and required review. |
| T3 | Whitelist and key errors | Commit after focused validation and required review. |
| T4 | Database errors | Commit after focused validation and required review. |
| T5 | `PeerKeyError` sources and the dependency removal | Commit after focused validation and required review. |
| T6 | Consumer updates not already needed by T3 to T5 to compile | Commit after focused validation and required review. |
| T1, T7 | Evidence | Commit after focused validation and required review. |

Record a justified no-change decision in the task's evidence without creating an empty commit. For
test-producing work, use the `write-unit-test` skill and complete an explicit design review after
each passing test increment, before maintainer review and commit. Confirm that the test exposes the
one causal initial-state difference; its fixture owns only incidental mechanics; and the production
Act plus independently specified expected result remain visible. The review must use the mandatory
prose-first Arrange-Act-Assert comparison: write temporary prose for each section, refactor until
the code expresses it, remove redundant prose, and retain only irreducible context. Record this
review in task evidence or a file-local test plan. Assess helper boundaries by meaningful named
actions and abstraction-level alignment, not caller count: a single-use helper is valid when it
keeps the test readable and hides only incidental mechanics. Commit each reviewed test-design
increment before starting the next planned file or behavior area. Keep final verification and
completion evidence separate when it improves reviewability. Use a Conventional Commit message
with the narrow affected scope, and sign every commit with GPG.

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/drafts/error-locations-si-4-tracker-core/ISSUE.md`
- [ ] Spec reviewed and approved by user/maintainer
- [ ] GitHub issue created and issue number added to this spec
- [ ] (Optional, recommended for complex issues) Spec-only PR merged into `develop` before implementation
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

Append one line per meaningful update. When an AI agent did the work, name its model after the
agent, for example `GitHub Copilot (Claude Opus 5.5)`, so later reviews can relate outcomes to
the model that produced them.

- 2026-11-11 17:05 UTC - Claude Code (Fable 5.2) - Drafted from the audit of issue #2804 (`docs/issues/open/2804-location-caller-track-caller-audit/inventory.md`, `options.md`) as a sub-issue of the EPIC draft. Not yet approved; line numbers are those of `develop` at `4f0af379a` and must be re-checked before implementation.

## Acceptance Criteria

- [ ] AC1: No error type in `packages/tracker-core` has a `location` field or a `LocatedError` source.
- [ ] AC2: `packages/tracker-core/src` and `http-core/src/services/announce.rs` have no `Location::caller()`, `#[track_caller]` or `Located(..).into()` outside test code.
- [ ] AC3: A UDP reply to a non-whitelisted announce, an HTTP failure reason for whitelist and key errors, and a REST API error for a failed key or whitelist write hold no source path, each proven by a test through the protocol server.
- [ ] AC4: The `ErrorKind::Whitelist` event payload holds no source path.
- [ ] AC5: `packages/tracker-core/Cargo.toml` no longer depends on `torrust-located-error`.
- [ ] `linter all` exits with code `0`
- [ ] Relevant tests pass
- [ ] Manual verification scenarios are executed and documented in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria are re-reviewed after implementation and reflect actual behavior
- [ ] Documentation is updated when behavior/workflow changes

## Verification Plan

Define verification before implementation starts and execute it before closing the issue.

### Automatic Checks

- `linter all`
- Relevant tests for changed components
- Pre-push checks (when applicable)

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID | Scenario | Human-oriented command/steps | Expected Result | Status | Evidence |
| --- | --- | --- | --- | --- | --- |
| M1 | UDP whitelist rejection | Start a listed tracker; send a UDP announce for an info-hash that is not whitelisted (tracker client or a script) | The error message says the torrent is not whitelisted and holds no path | TODO | `manual-verification-evidence.md` section V1 |
| M2 | HTTP key and whitelist errors | Announce over HTTP for the same info-hash, and on a private tracker with an expired key | The failure reasons hold no path | TODO | `manual-verification-evidence.md` section V2 |
| M3 | REST database failure | Add the same whitelist entry or key twice through the REST API | The error text names the failed operation and holds no path, including no driver file | TODO | `manual-verification-evidence.md` section V3 |

Notes:

- Manual verification is mandatory even when automated tests pass. It is a
  real human-oriented use of the feature or reproduction of the bug fix, not a
  simulated result and not merely running automated tests.
- Every recorded validation command result must identify the toolchain or
  runtime that produced it when one can affect behavior. For example, record
  `cargo +nightly fmt --all -- --check` as `nightly Rust toolchain`, rather than
  the ambiguous `cargo fmt --all -- --check`.
- Create `manual-verification-evidence.md` from
  `docs/templates/MANUAL-VERIFICATION-EVIDENCE.md` when executing these
  scenarios. Record actual prerequisites, actions, commands, program output,
  relevant tracker logs, and outcomes there.
- If a scenario fails, record the failure and diagnosis in the progress log before proceeding.

### Disposable Verification Scripts

Temporary scripts may automate an issue-local verification scenario, but they
are neither maintained automatic tests nor manual-verification evidence. Before
creating one, record in the issue specification:

- why temporary automation is better for this concrete scenario than a
  maintained Rust automatic test;
- the script's issue-local path, what it verifies, and its intended removal or
  retention owner; and
- when using Python instead of Rust, why Rust is not suitable for that specific
  script.

Keep the script in the issue-specification folder so later reviewers can inspect
the verification performed. Promote durable product-behavior checks into Rust
automatic tests when practical, then remove the disposable script.

None planned for this issue.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| --- | --- | --- |
| AC1 | TODO | {test/log/PR link} |
| AC2 | TODO | {test/log/PR link} |
| AC3 | TODO | {test/log/PR link} |
| AC4 | TODO | {test/log/PR link} |
| AC5 | TODO | {test/log/PR link} |

## Risks and Trade-offs

- **Largest diff of the EPIC**: 12 driver sites in three drivers, plus consumers in four packages.
  Mitigation: commit points T3 to T6 keep each change reviewable.
- **Lost diagnostic for database failures.** The driver line told which statement failed.
  Mitigation: the `tracing` event in T4 and the variant's own context.
- **Depends on SI-3 ordering only for `http-core` tests**, which assert failure reasons.
  Mitigation: the EPIC runs SI-3 first; if run in parallel, the later one rebases.

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
  `docs/templates/AGENT-REVIEW-REPORTS.md`. Do not create that artifact for a
  legacy standalone specification or when no folder-style specification was
  supplied.

## References

- Parent EPIC draft: `docs/issues/drafts/remove-source-locations-from-errors/EPIC.md`
- Audit: `docs/issues/open/2804-location-caller-track-caller-audit/ISSUE.md`, `inventory.md`, `options.md`, `experiments.md` (issue #2804)
- Related issues: #2804 (audit), #384 (external report), #2583 (first publish of the workspace crates)
