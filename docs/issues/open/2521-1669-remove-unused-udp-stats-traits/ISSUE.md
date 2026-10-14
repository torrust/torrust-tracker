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
last-updated-utc: "2026-10-14 08:14"
semantic-links:
  skill-links:
    - create-issue
    - write-unit-test
  related-artifacts:
    - .github/skills/dev/planning/create-issue/SKILL.md
    - docs/issues/open/1669-overhaul-packages/EPIC.md
    - docs/issues/open/2505-udp-banned-ips-metric/ISSUE.md
    - docs/issues/closed/1924-1669-si-30-decouple-rest-api-core-from-udp-internals/ISSUE.md
    - docs/issues/closed/1942-1938-si-4-migrate-stats-context/ISSUE.md
    - packages/udp-core/src/statistics/repository.rs
    - packages/udp-server/src/statistics/repository.rs
    - packages/rest-api-application/src/v1/ports/stats.rs
    - packages/rest-api-runtime-adapter/src/v1/adapters/stats.rs
---

<!-- skill-link: create-issue -->

# Issue #2521 - Remove the Unused UDP Statistics Repository Traits Left by SI-30

Parent EPIC: #1669 (Overhaul: Packages).

## Goal

Remove the `UdpCoreStatsRepository` and `UdpServerStatsRepository` traits, which SI-30 added for
REST API decoupling and which nothing consumes, together with the `async-trait` dependency they
alone require.

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

Evidence on `develop` at `395cd3ca0` (2026-10-14):

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

### Out of Scope

- `BanningStats`: removed by #2505.
- What `get_banned_ips_total()` counts: #2505.
- The public `stats` lock field on the statistics repositories (#2517, a separate #1669
  sub-issue).
- Changing the REST API ports or the runtime adapter design.

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
  `udp-server`; it is not a layer in the model.
- Verification: compare `cargo tree -e normal --depth 1 -p torrust-tracker-udp-core` and the same
  for `torrust-tracker-udp-server` before and after the change. The only difference is the
  removed `async-trait` line.

## Bug-Fix Process

Not applicable: not a bug. The traits are dead code, not incorrect behavior.

## Regression Test Strategy

No new behavior, so no new regression test. Existing tests must keep passing. The adapted
`udp-server` test keeps its intent (default construction initializes the described metrics). Follow
the `write-unit-test` skill for the adaptation and record a brief Arrange-Act-Assert review of the
adapted test in the progress log before committing.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task                                                | Notes / Expected Output                                                                                                                                             |
| --- | ------ | --------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| T1  | TODO   | Re-verify the evidence on current `develop`         | Search confirms no consumers; record the commit. Check whether #1348 has merged a test calling `UdpCoreStatsRepository::get_metrics_collection()` (see Risks).     |
| T2  | TODO   | Remove `UdpCoreStatsRepository` from `udp-core`     | Trait, impl, allow, unused import, and `async-trait` dependency removed; any test calling the trait adapted to `get_stats()` or deleted if it only tests the trait. |
| T3  | TODO   | Remove `UdpServerStatsRepository` from `udp-server` | Trait, impl, allow, unused import, and `async-trait` dependency removed; two tests handled as described in Scope.                                                   |
| T4  | TODO   | Update the EPIC #1669 SI-30 row                     | Note added; no remaining references in current docs except historical records and #2505's Out of Scope note.                                                       |
| T5  | TODO   | Verification                                        | See Verification Plan.                                                                                                                                              |

## Commit Points

| Task | Coherent change set                                                                    | Commit policy                                        |
| ---- | -------------------------------------------------------------------------------------- | ---------------------------------------------------- |
| T2   | `refactor(udp-core): [#2521] remove unused UdpCoreStatsRepository trait`     | Commit after focused validation and `cargo machete`. |
| T3   | `refactor(udp-server): [#2521] remove unused UdpServerStatsRepository trait` | Commit after focused validation and `cargo machete`. |
| T4   | `docs(issues): [#2521] note the SI-30 trait removal in EPIC #1669`           | Commit after `linter all`.                           |

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
- [ ] Evidence-based implementation completion review recorded
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
- [ ] `linter all` exits with code `0`
- [ ] Relevant tests pass
- [ ] Manual verification scenarios are executed and documented in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria are re-reviewed after implementation and reflect actual behavior

## Verification Plan

### Automatic Checks

- `linter all`
- `cargo test -p torrust-tracker-udp-core -p torrust-tracker-udp-server -p torrust-tracker-rest-api-runtime-adapter --all-features`
  (stable Rust toolchain)
- `cargo machete --with-metadata`
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

## Risks and Trade-offs

- **Overlap with #1348.** Branch `1348-1347-add-tests-udp-core` adds a `udp-core` repository test
  that calls `UdpCoreStatsRepository::get_metrics_collection()`. Whichever lands second adapts it:
  read through `get_stats()` if the test protects repository behavior, or delete it if it only
  tests the trait. Do not edit `docs/issues/open/1348-*` from this issue.
- **Overlap with #2505.** #2505 edits `packages/udp-core/src/services/banning.rs` and removes
  `BanningStats`; this issue does not touch that file, so no conflict is expected.
- **Overlap with #2517 (make the statistics repositories' metrics lock private).** It changes the same two
  repository files (the `stats` field). Scopes are disjoint; the second to land rebases.
- **Future need for the traits.** If a consumer later needs a narrow interface, it should be
  defined at the consumer's boundary (as the `rest-api-application` ports are), not restored here.

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
