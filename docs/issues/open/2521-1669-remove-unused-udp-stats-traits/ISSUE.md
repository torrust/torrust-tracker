---
schema-version: 1
doc-type: issue
issue-type: task
status: planned
priority: p3
epic: 1669
github-issue: 2521
spec-path: docs/issues/open/2521-1669-remove-unused-udp-stats-traits/ISSUE.md
branch: "2521-1669-remove-unused-udp-stats-traits-spec"
related-pr: null
last-updated-utc: "2026-10-14 09:44"
semantic-links:
  skill-links:
    - create-issue
    - write-unit-test
  related-artifacts:
    - .github/skills/dev/planning/create-issue/SKILL.md
    - "issue #1669"
    - "issue #2505"
    - "issue #1924"
    - "issue #1942"
    - packages/udp-core/src/statistics/repository.rs
    - packages/udp-server/src/statistics/repository.rs
    - packages/rest-api-application/src/v1/ports/stats.rs
    - packages/rest-api-runtime-adapter/src/v1/adapters/stats.rs
    - packages/udp-core/src/statistics/services.rs
    - packages/udp-server/src/statistics/services.rs
---

<!-- skill-link: create-issue -->

# Issue #2521 - Remove the Unused UDP Statistics Traits and Services

Parent EPIC: #1669 (Overhaul: Packages).

## Goal

Remove the unused public statistics API from `udp-core` and `udp-server`:

- the `UdpCoreStatsRepository` and `UdpServerStatsRepository` traits, which SI-30 added for REST
  API decoupling, together with the `async-trait` dependency they alone require; and
- the `statistics::services` modules (`TrackerMetrics` and `get_metrics()`).

Nothing outside each item's own definition and tests consumes them: the REST runtime adapter reads
the statistics repositories directly.

## Background

SI-30 ([#1924](https://github.com/torrust/torrust-tracker/issues/1924)) defined three traits so the
REST API could depend on narrow interfaces instead of concrete UDP types:

| Trait                      | Package                                   | Method                     | Implemented by                       |
| -------------------------- | ----------------------------------------- | -------------------------- | ------------------------------------ |
| `BanningStats`             | `udp-core` (`services/banning.rs`)        | `get_banned_ips_total()`   | `BanService`                         |
| `UdpCoreStatsRepository`   | `udp-core` (`statistics/repository.rs`)   | `get_metrics_collection()` | `statistics::repository::Repository` |
| `UdpServerStatsRepository` | `udp-server` (`statistics/repository.rs`) | `get_metrics_collection()` | `statistics::repository::Repository` |

SI-30 deferred the REST-side wiring. The contract-first migration (SI-33
[#1930](https://github.com/torrust/torrust-tracker/issues/1930), then EPIC #1938 and its SI-4
[#1942](https://github.com/torrust/torrust-tracker/issues/1942)) decoupled the REST API differently:
`rest-api-application` defines ports (`StatsQueryPort` and others), and `rest-api-runtime-adapter`'s
`TrackerStatsAdapter` implements them over the **concrete** repositories
(`get_stats().await.metric_collection`). The traits were never wired. Issues #1924, #1930, #1938,
and #1942 are closed, so the traits are leftovers, not an unfinished refactor.

`BanningStats` is **not** in this issue's scope: the banned-IPs metric bug
[#2505](https://github.com/torrust/torrust-tracker/issues/2505) already owns its removal (that
spec's In Scope and AC4, decided by the maintainer on 2026-10-13), and that spec leaves the two
repository traits to a separate issue. This is that issue.

Evidence on `develop` at `cbdbce9c1` (2026-10-14):

- Outside their definitions and impls, no code names `UdpCoreStatsRepository` or
  `UdpServerStatsRepository`, except two `udp-server` repository unit tests that call
  `UdpServerStatsRepository::get_metrics_collection(&repo)`:
  - `it_should_initialize_metrics_when_constructed_with_default` checks that `Repository::default()`
    initializes the described metrics; the trait call is only its read path.
  - `it_should_return_a_snapshot_of_the_initialized_metrics_collection` exercises only the trait
    method.
- `TrackerStatsAdapter` holds `Arc<torrust_tracker_udp_core::statistics::repository::Repository>`
  and `Arc<torrust_tracker_udp_server::statistics::repository::Repository>` and reads
  `get_stats().await.metric_collection`.
- Both traits carry `#[async_trait::async_trait]` and a documented `clippy::double_must_use`
  allowance. They are the only `async_trait` uses in `udp-core` and `udp-server`, so removing them
  leaves the `async-trait` dependency unused in both packages. The `MetricCollection` import in
  both repository files is used only by the traits.
- Neither `torrust-tracker-udp-core` nor `torrust-tracker-udp-server` is published on crates.io, so
  removing the public traits has no external semver impact.
- The only current documentation naming these traits, outside closed specs, is the EPIC #1669
  SI-30 table row and the #2505 spec's Out of Scope note.

### Unused statistics services

Both packages also keep a `statistics::services` module with a public `TrackerMetrics` struct
(aggregate swarm metadata plus a copy of the package's `Metrics`) and a public
`async fn get_metrics(in_memory_torrent_repository, stats_repository) -> TrackerMetrics`. They date
from the 2025 split of the tracker statistics into per-protocol packages; the REST API no longer
calls them, because `TrackerStatsAdapter` reads the repositories directly and takes swarm metadata
from `tracker-core`.

Evidence on `develop` at `d1bf013ea` (2026-10-14):

- In `udp-core` and `udp-server`, `statistics/services.rs` is the only file that names
  `TrackerMetrics` or `statistics::services::get_metrics`. Each file's only reference is its own
  test, `the_statistics_service_should_return_the_tracker_metrics`, which tests only the removed
  function. No other package names either module.
- No other package has a `statistics/services.rs` module.
- Removing the modules changes no workspace dependency edge: both packages still use
  `torrust_tracker_core` (for example `udp-core`'s `container.rs` and `services/announce.rs`) and
  `torrust_tracker_primitives` in other files.
- Each `services.rs` carries the only module-level documentation of the package's statistics
  event flow (event sender, listener, handler, repository). Part of it is stale: its example shows a
  `pub udp4_connections_handled: u64` field on `Metrics`, which is now a method.

Reported by the AI agent working on issue #1348, after this specification was first drafted; the
maintainer chose to fold it into this issue on 2026-10-14.

Found while planning `udp-core` tests in issue #1348 (branch `1348-1347-add-tests-udp-core`, plan
`test-refactor-plans/statistics-repository-tests.md`, not yet on `develop`).

## Scope

### In Scope

- Remove `UdpCoreStatsRepository` and its `impl` for `udp-core`'s statistics `Repository`.
- Remove `UdpServerStatsRepository` and its `impl` for `udp-server`'s statistics `Repository`.
- Remove the two `allow(clippy::double_must_use)` attributes with their comments, and any import
  left unused (`MetricCollection`).
- Remove the `async-trait` dependency from `udp-core` and `udp-server` (confirm with
  `cargo machete --with-metadata`).
- Adapt `it_should_initialize_metrics_when_constructed_with_default` to read through the concrete
  `get_stats()` path, keeping its intent. Delete
  `it_should_return_a_snapshot_of_the_initialized_metrics_collection`, which only tests the removed
  method; `it_should_be_initialized_with_described_metrics` already covers the initialized
  collection through `get_stats()`.
- Add a note to the EPIC #1669 SI-30 table row that the two repository traits were removed by this
  issue and `BanningStats` by #2505. Closed specs are historical and stay unchanged.
- Remove `statistics/services.rs` (`TrackerMetrics`, `get_metrics()`, and its test) and the
  `pub mod services;` declaration from `udp-core` and from `udp-server`.
- Keep the still-accurate statistics event-flow explanation by moving it to the `statistics/mod.rs`
  module documentation of each package, dropping the stale `Metrics` field example and any
  reference to the removed service.

### Out of Scope

- `BanningStats`: removed by #2505.
- What `get_banned_ips_total()` counts: #2505.
- The public `stats` lock field on the statistics repositories (#2517, a separate #1669
  sub-issue).
- Changing the REST API ports or the runtime adapter design.
- Statistics services in other packages: none exist today.

## Architectural Decisions

- Related: SI-30 (#1924), SI-33 (#1930), and SI-4 (#1942) specs; ADR
  `docs/adrs/20260527175600_keep_protocol_and_domain_types_decoupled.md` for context only.
- Decision (maintainer, 2026-10-14): remove the traits rather than wire `rest-api-runtime-adapter`
  through them. The `rest-api-application` ports already own the REST boundary.
- ADRs to create: none expected; this removes unused items and adds no design. If an important
  architectural decision emerges, record it in `docs/adrs/` (it spans `udp-core`, `udp-server`, and
  the REST packages) and link it here.

## Design and Ownership Review

Not applicable: removal of unused public items; no I/O, lifetimes, or fixtures.

Package boundaries (EPIC #1669 subissue architecture checklist): no workspace dependency edge
changes.

- Current and target workspace edges are identical: `rest-api-runtime-adapter -> udp-core` and
  `rest-api-runtime-adapter -> udp-server` remain, because `TrackerStatsAdapter` still uses the
  concrete `statistics::repository::Repository` types. These runtime-adapter edges respect the
  layer model; the change removes no forbidden edge and adds none.
- The only dependency change is the external `async-trait` crate leaving `udp-core` and
  `udp-server`; it is not a layer in the model. Removing `statistics::services` drops no edge,
  because both packages still use `tracker-core` and `primitives` elsewhere.
- Verification: compare `cargo tree -e normal --depth 1 -p torrust-tracker-udp-core` and the same
  for `torrust-tracker-udp-server` before and after the change. The only difference is the
  removed `async-trait` line.

## Bug-Fix Process

Not applicable: not a bug. The traits and services are dead code, not incorrect behavior.

## Regression Test Strategy

No new behavior, so no new regression test. Existing tests must keep passing. The adapted
`udp-server` test keeps its intent (default construction initializes the described metrics). The
two `the_statistics_service_should_return_the_tracker_metrics` tests are deleted with the function
they test (T4, T5); deleting a test of removed code needs no test-design review.

Test-change gates for T3 (and for T2 if #1348's trait-calling test has landed by then):

1. Use the `write-unit-test` skill. Make the test change as the smallest behavior-focused
   increment and run focused validation:
   `cargo test -p torrust-tracker-udp-server --lib statistics::repository` (stable Rust toolchain).
2. After it passes, perform and record the mandatory prose-first Arrange-Act-Assert design review
   in the progress log: write temporary prose for each section, refactor until the code expresses
   it, and remove the redundant prose. Confirm that the test exposes the one causal initial state
   (default construction), its fixture owns only incidental mechanics, and the production Act and
   the independently specified expected result (the described metric is present) stay visible.
3. Stop for maintainer review of the test change before final verification, commit, or pull
   request.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task                                                       | Notes / Expected Output                                                                                                                                                       |
| --- | ------ | ---------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| T1  | TODO   | Re-verify the evidence on current `develop`                | Search confirms no consumers of the traits or services; record the commit. Check whether #1348 has merged tests calling `UdpCoreStatsRepository` or `get_metrics()` (see Risks). |
| T2  | TODO   | Remove `UdpCoreStatsRepository` from `udp-core`            | Trait, impl, allow, unused import, and `async-trait` dependency removed; any test calling the trait adapted to `get_stats()` or deleted if it only tests the trait.          |
| T3  | TODO   | Remove `UdpServerStatsRepository` from `udp-server`        | Trait, impl, allow, unused import, and `async-trait` dependency removed; two tests handled as described in Scope.                                                            |
| T4  | TODO   | Remove `statistics::services` from `udp-core`              | `services.rs` and `pub mod services;` removed; the accurate event-flow docs moved to `statistics/mod.rs` without the stale `Metrics` field example.                          |
| T5  | TODO   | Remove `statistics::services` from `udp-server`            | Same as T4 for `udp-server`.                                                                                                                                                  |
| T6  | TODO   | Update the EPIC #1669 SI-30 row                            | Note added; no remaining references in current docs except historical records and #2505's Out of Scope note.                                                                |
| T7  | TODO   | Verification                                               | See Verification Plan.                                                                                                                                                        |

## Commit Points

| Task | Coherent change set                                                          | Commit policy                                        |
| ---- | ---------------------------------------------------------------------------- | ---------------------------------------------------- |
| T1   | No separate commit: the re-verification progress-log entry rides with T2     | Recorded in the spec before starting T2.             |
| T2   | `refactor(udp-core): [#2521] remove unused UdpCoreStatsRepository trait`     | Commit after focused validation and `cargo machete`; if a test changed, also after the recorded test-design review and maintainer review. |
| T3   | `refactor(udp-server): [#2521] remove unused UdpServerStatsRepository trait` | Commit after focused validation, `cargo machete`, the recorded test-design review, and maintainer review. |
| T4   | `refactor(udp-core): [#2521] remove unused statistics services`              | Commit after focused validation, `cargo machete`, and `cargo +nightly doc` for the package. |
| T5   | `refactor(udp-server): [#2521] remove unused statistics services`            | Commit after focused validation, `cargo machete`, and `cargo +nightly doc` for the package. |
| T6   | `docs(issues): [#2521] note the SI-30 trait removal in EPIC #1669`           | Commit after `linter all`.                           |
| T7   | `docs(issues): [#2521] record verification evidence`: issue-local `manual-verification-evidence.md` (M1, M2), Acceptance Verification evidence, and spec progress updates | Commit after all checks pass and the evidence is reviewed. |

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/drafts/1669-remove-unused-udp-stats-traits/ISSUE.md`
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue created and issue number added to this spec
- [ ] Spec-only PR merged into `develop` before implementation
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

- 2026-10-14 06:50 UTC - GitHub Copilot (#1348 session) - Drafted from the
  `statistics/repository.rs` test plan finding; maintainer asked for it as an EPIC #1669
  sub-issue.
- 2026-10-14 07:15 UTC - GitHub Copilot (spec session) - Re-verified on `develop` at `395cd3ca0`.
  Found that the merged #2505 spec already owns removing `BanningStats`; the maintainer narrowed
  this issue to the two repository traits and chose removal over wiring. Corrected the history
  (SI-4 #1942 built the adapter over concrete repositories). Confirmed the crates are unpublished.
- 2026-10-14 08:14 UTC - GitHub Copilot (spec session) - Maintainer approved the spec. Re-checked
  the evidence at `cbdbce9c1` (unchanged). Created #2521, linked it under EPIC #1669, and moved the
  spec to `docs/issues/open/`. The sibling draft is now #2517.
- 2026-10-14 09:13 UTC - GitHub Copilot (spec session) - Processed PR #2522 review round 1
  (reviews 5515225239 and 5515320310): split the REST verification into M1 (`/api/v1/stats`) and
  M2 (`/api/v1/metrics`), recorded the package-boundary review, made the test-change gates
  explicit, restored the template checkpoints and documentation AC, mapped T1 and T5 to commits,
  corrected the evidence commit, and switched issue-spec links to `issue #NNNN`. Audit:
  `docs/pr-reviews/pr-2522-review/PR-REVIEW.md`.
- 2026-10-14 09:44 UTC - GitHub Copilot (spec session) - The #1348 agent reported that
  `statistics::services` (`TrackerMetrics`, `get_metrics()`) is also unused in both packages;
  verified at `d1bf013ea`, and the maintainer folded it into this issue. Added T4 and T5 (EPIC row
  and verification became T6 and T7), AC5 and AC6, and retitled the issue.

## Acceptance Criteria

- [ ] AC1: `UdpCoreStatsRepository` and `UdpServerStatsRepository` no longer exist, and no code
      references them.
- [ ] AC2: `cargo machete --with-metadata` reports no unused dependency in `udp-core` or
      `udp-server`, and neither package depends on `async-trait`.
- [ ] AC3: Both REST surfaces are unchanged, in JSON and Prometheus formats: `/api/v1/stats` still
      reports the UDP server fields (read from the `udp-server` repository), and `/api/v1/metrics`
      still reports `udp_tracker_core_requests_received_total` (the `udp-core` repository) and the
      `udp_tracker_server_*` metrics, with unchanged names (M1 and M2).
- [ ] AC4: The adapted `udp-server` test still verifies that default construction initializes the
      described metrics.
- [ ] AC5: `statistics::services`, `TrackerMetrics`, and `statistics::services::get_metrics()` no
      longer exist in `udp-core` or `udp-server`, and no code references them.
- [ ] AC6: Each package's `statistics/mod.rs` documents the statistics event flow without the
      stale `Metrics` field example, and `cargo +nightly doc --no-deps` for both packages reports
      no broken intra-doc link.
- [ ] `linter all` exits with code `0`
- [ ] Relevant tests pass
- [ ] Manual verification scenarios are executed and documented in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria are re-reviewed after implementation and reflect actual behavior
- [ ] Documentation is updated when behavior/workflow changes

## Verification Plan

### Automatic Checks

- `linter all`
- `cargo test -p torrust-tracker-udp-core -p torrust-tracker-udp-server -p torrust-tracker-rest-api-runtime-adapter --all-features`
  (stable Rust toolchain)
- `cargo machete --with-metadata`
- `cargo +nightly doc --no-deps -p torrust-tracker-udp-core -p torrust-tracker-udp-server`
  (nightly Rust toolchain; no broken intra-doc links after the module docs move)
- `cargo tree -e normal --depth 1` for `torrust-tracker-udp-core` and `torrust-tracker-udp-server`,
  before and after (see Design and Ownership Review)
- `./contrib/dev-tools/git/hooks/pre-commit.sh` before each commit and
  `./contrib/dev-tools/git/hooks/pre-push.sh` before pushing (nightly Clippy and formatting, full
  stable test suite)

Record the toolchain for each command result.

### Manual Verification Scenarios

The two REST surfaces read different repositories: `TrackerStatsAdapter::get_stats()` serves
`/api/v1/stats` from the `udp-server` repository only, while `get_labeled_stats()` merges both the
`udp-core` and `udp-server` metric collections and serves `/api/v1/metrics`. Verify each surface
separately. For both scenarios, run the tracker with the default development configuration, send
one UDP announce with `tracker_client`, and capture the output before the change (baseline, on
`develop`) and after it.

| ID  | Scenario                                         | Human-oriented command/steps                                                                                                                                                   | Expected Result                                                                                                                                                                                                       | Status | Evidence                                     |
| --- | ------------------------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------ | -------------------------------------------- |
| M1  | `/api/v1/stats` still reports UDP server fields  | `curl "http://127.0.0.1:1212/api/v1/stats?token=MyAccessToken&format=json"` and `curl "http://127.0.0.1:1212/api/v1/stats?token=MyAccessToken&format=prometheus"`              | Both formats show the same UDP fields as the baseline (for example `udp4_requests`, `udp4_announces_handled`, `udp_banned_ips_total`), and the announce is counted in `udp4_announces_handled`                        | TODO   | `manual-verification-evidence.md` section M1 |
| M2  | `/api/v1/metrics` still reports UDP core metrics | `curl "http://127.0.0.1:1212/api/v1/metrics?token=MyAccessToken&format=json"` and `curl "http://127.0.0.1:1212/api/v1/metrics?token=MyAccessToken&format=prometheus"`          | Both formats contain `udp_tracker_core_requests_received_total` (from `udp-core`) and the `udp_tracker_server_*` metrics (from `udp-server`) with the same names and labels as the baseline, and the announce is counted | TODO   | `manual-verification-evidence.md` section M2 |

### Disposable Verification Scripts

None planned.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| ----- | ---------------------- | -------- |
| AC1   | TODO                   |          |
| AC2   | TODO                   |          |
| AC3   | TODO                   |          |
| AC4   | TODO                   |          |
| AC5   | TODO                   |          |
| AC6   | TODO                   |          |

## Risks and Trade-offs

- **Overlap with #1348.** Branch `1348-1347-add-tests-udp-core` adds a `udp-core` repository test
  that calls `UdpCoreStatsRepository::get_metrics_collection()`. Whichever lands second adapts it:
  read through `get_stats()` if the test protects repository behavior, or delete it if it only
  tests the trait. Any test #1348 adds for `statistics::services::get_metrics()` is deleted with
  the module. Do not edit `docs/issues/open/1348-*` from this issue.
- **Overlap with #2505.** #2505 edits `packages/udp-core/src/services/banning.rs` and removes
  `BanningStats`; this issue does not touch that file, so no conflict is expected.
- **Overlap with #2517 (make the statistics repositories' metrics lock private).** It changes the same two
  repository files (the `stats` field). Scopes are disjoint; the second to land rebases.
- **Future need for the traits or services.** If a consumer later needs a narrow interface or an
  aggregated view, it should be defined at the consumer's boundary (as the `rest-api-application`
  ports are), not restored here.

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
- Related issues: #1924 (SI-30), #1930 (SI-33), #1938, #1942 (SI-4), #2505, #2517, #1348
- Related ADRs: `docs/adrs/20260527175600_keep_protocol_and_domain_types_decoupled.md`
