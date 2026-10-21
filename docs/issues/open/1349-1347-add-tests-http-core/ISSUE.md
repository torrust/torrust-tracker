---
schema-version: 1
doc-type: issue
issue-type: task
status: in-progress
priority: p2
epic: 1347
github-issue: 1349
spec-path: docs/issues/open/1349-1347-add-tests-http-core/ISSUE.md
branch: "1349-1347-add-tests-http-core-final"
related-pr: null
last-updated-utc: "2026-10-21 15:29"
semantic-links:
  skill-links:
    - create-issue
    - write-unit-test
    - fix-bug
  related-artifacts:
    - .github/skills/dev/planning/create-issue/SKILL.md
    - .github/skills/dev/testing/write-unit-test/SKILL.md
    - .github/skills/dev/debugging/fix-bug/SKILL.md
    - docs/issues/open/1347-overhaul-packages-testing/EPIC.md
    - docs/issues/open/1347-overhaul-packages-testing/subissue-spec-guidelines.md
    - docs/issues/closed/1348-1347-add-tests-udp-core/ISSUE.md
    - docs/issues/closed/1348-1347-add-tests-udp-core/implementation-retrospective.md
    - docs/issues/closed/1348-1347-add-tests-udp-core/lessons.md
    - docs/pr-reviews/pr-2546-review/PR-REVIEW.md
    - docs/pr-reviews/pr-2558-review/PR-REVIEW.md
    - docs/issues/open/1349-1347-add-tests-http-core/coverage-evidence.md
    - docs/issues/open/1349-1347-add-tests-http-core/test-refactor-plans/README.md
    - packages/http-core
---

<!-- skill-link: create-issue -->

# Issue #1349 - Add Tests to the HTTP Core Package

Parent EPIC: #1347 - Overhaul: Packages Testing

## Goal

Increase the maintainable, package-local test coverage of `torrust-tracker-http-core` by going
systematically over every Rust source file in the package. For each file: review the existing tests
against the `write-unit-test` skill and refactor them when they are not clean, then analyse coverage
and add unit tests (preferred) or package integration tests (only when a unit test cannot protect
the behavior at an appropriate boundary), and record the resulting per-file coverage before moving
to the next file.

Deliver the work as a sequence of small pull requests, each merged before the next slice starts,
instead of one large pull request at the end.

## Background

GitHub issue #1349 was opened when the package was named `bittorrent-http-tracker-core` under
`packages/http-tracker-core/`. It is now `torrust-tracker-http-core` under `packages/http-core/`.
The coverage table in the GitHub issue body is a historical 2025 measurement of a different package
layout (it still lists `statistics/keeper.rs`, `statistics/services.rs`, `statistics/setup.rs`, and
the then-vendored `contrib/bencode`); it is not a baseline for this work.

The package owns the HTTP-specific tracker domain logic between the HTTP protocol crate and the
Axum HTTP server: the announce and scrape application services (authentication in private mode,
whitelist authorization, client-IP resolution behind a reverse proxy, peer-IP selection from the
BEP 3 `ip` parameter and the configured external IP, mapping protocol requests to domain
announcements, and error adaptation to bencoded failure responses), HTTP core domain events and
their metric labels, the HTTP core service container, and the HTTP core statistics (metrics,
repository, event handler, and listener).

This specification uses the completed #1348 (`udp-core`) specification as its template and applies
its [implementation retrospective](../../closed/1348-1347-add-tests-udp-core/implementation-retrospective.md)
and the [EPIC #1347 subissue guidelines](../1347-overhaul-packages-testing/subissue-spec-guidelines.md).
It adds two changes requested by the maintainer after #1348:

1. **Progressive delivery.** #1348 took about a week and merged as a single large PR (#2538) that
   was hard to review. This issue keeps one GitHub issue but splits the implementation into
   several [pull-request slices](#pull-request-slices), each reviewed and merged on its own. The
   work is test-only, so merging a partial slice is safe: it adds tests to modules without changing
   production behavior.
2. **A defined procedure for findings.** Adding tests to #1348 and earlier packages often found
   bugs. This issue defines the [findings procedure](#findings-during-implementation): a draft
   specification plus a hand-off document in a local `.tmp/` folder, passed to another agent, with
   an explicit decision to continue in parallel or pause and rebase after the fix merges.

## Baseline Coverage

Measured on 2026-10-15 at `develop` commit `a9d8c5703`
(`Merge torrust/torrust-tracker#2541: chore(issues): archive closed issue #1348 spec`), with
`cargo-llvm-cov 0.6.16` and stable Rust 1.99.0, from a clean workspace:

```console
cargo llvm-cov clean --workspace
cargo llvm-cov -p torrust-tracker-http-core --all-features --json --output-path .tmp/1349-coverage/aggregate.json
cargo llvm-cov clean --workspace
cargo llvm-cov -p torrust-tracker-http-core --all-features --lib --json --output-path .tmp/1349-coverage/unit.json
```

Totals include only files under `packages/http-core/src/`, test modules included, because unit
tests live in the same files. Benches under `packages/http-core/benches/` are not measured.

| Scope | Tests | Lines | Regions | Functions |
| --- | ---: | ---: | ---: | ---: |
| Aggregate (all selected test targets) | 32 unit | 1,743 / 1,957 (89.06%) | 2,337 / 2,582 (90.51%) | 157 / 186 (84.41%) |
| Unit-only (`--lib`) | 32 unit | 1,743 / 1,957 (89.06%) | 2,337 / 2,582 (90.51%) | 157 / 186 (84.41%) |

The two scopes are identical because the package has no `tests/` integration target.
Integration-only coverage does not apply unless a file plan adds a target.

Per-file unit-only lines are in the [Source File Ledger](#source-file-ledger). Two package-owned
files have no unit-only coverage: `container.rs` (0 / 73) and `services/error_mapping.rs` (0 / 18).
The rest have high line coverage, but #1348 showed that a covered file can still pin the wrong value
or miss a decision, so every file still gets its own plan.

The `torrust-tracker-axum-http-server` package and the root `tests/` exercise these services
indirectly. EPIC #1347 does not accept coverage from another package, an integration test, or an
end-to-end test as evidence that a feasible package-owned unit seam is protected.

`packages/http-core/` changed on `develop` after `a9d8c5703`, so the package was re-measured twice:
before S1 (T1: the #2536 refactor merged through #2545, and the #2244 refactor merged through #2552)
and again before S2, at `develop` `87690b15e` (the merge of S1, #2558), after `non_exhaustive` on the
public error enums (#2454, merged through #2568) and the exact counter sum from `torrust-metrics`
0.2.0 (#2560, merged through #2561). The S2 baseline is lines 1,751 / 1,970 (88.88%), regions
2,357 / 2,607 (90.41%), and functions 161 / 191 (84.29%), from 33 unit tests, with unit-only equal
to aggregate. It supersedes the table above; [coverage-evidence.md](coverage-evidence.md) holds the
per-file results and the differences. The Source File Ledger and the EPIC #1347 Package Coverage
Tracking tables use the S2 baseline.

## Key Package Responsibilities

Ranked from most to least critical. The ranking drives the ledger processing order.

| Rank | Responsibility | Source modules | Why it matters |
| ---: | --- | --- | --- |
| 1 | Announce peer-address decision | `services/announce.rs` (`select_peer_ip_with_policy`, `external_ip_for_loopback_connection`, `PeerIpSelectionPolicy`, `HttpTrackerPolicy`) | Decides which IP other peers receive. Trusting a client-supplied `ip` when the policy forbids it lets a client register an arbitrary address in a swarm; rejecting it wrongly breaks clients that need it. |
| 2 | Announce admission and adaptation | `services/announce.rs` (`authenticate`, `authorize`, `peer_from_request`, `peers_wanted`, `HttpAnnounceError`), `services/error_mapping.rs` | Private-mode key checks, whitelist authorization, the protocol-to-domain announcement mapping, and the client-visible failure reasons. |
| 3 | Scrape service | `services/scrape.rs` | Private-mode zeroing of unauthenticated scrapes, reverse-proxy client-IP resolution, error adaptation, and event publication. |
| 4 | HTTP core domain events | `event.rs` | Contract consumed by statistics and other listeners, including metric labels such as the client IP family and type. |
| 5 | Service composition | `container.rs` | Wires the announce and scrape services with the per-instance HTTP tracker policy, instance ID, and the event bus. No unit-only coverage. |
| 6 | HTTP core statistics | `statistics/event/handler.rs`, `statistics/event/listener.rs`, `statistics/metrics.rs`, `statistics/repository.rs`, `statistics/mod.rs` | Observability. Incorrect metrics do not change tracker answers. Listener lifecycle is owned by #1488. |
| 7 | Crate root and wiring | `lib.rs`, `services/mod.rs`, `statistics/event/mod.rs` | Clock aliases, the log target, shared test samples, and module declarations. |

## Coverage Objectives

EPIC #1347 sets qualitative, risk-based objectives rather than a numeric target. For this package:

1. **Highest risk** - the peer-address decision: every `PeerIp` variant against both
   `use_ip_from_query_string` values, the loopback external-IP substitution for IPv4 and IPv6, and
   the reverse-proxy resolution mode taken from configuration. Each package-owned branch must have
   a direct unit test or a recorded owner-based reason.
2. **High** - admission and adaptation: private-mode authentication (missing, invalid, and valid
   key) for announce and scrape, whitelist rejection, every `HttpAnnounceError` and
   `HttpScrapeError` conversion to its failure reason (including `error_mapping.rs`), and the
   protocol-to-domain announcement mapping, including defaults for absent byte counters and every
   event variant.
3. **Medium** - event labels, container composition, and statistics projections, where the
   observable contract is the wired configuration values and the produced labels and counters.
4. **Low** - declaration-only modules, logging-only branches, and listener lifecycle owned by #1488.

The objective is met when every highest-risk and high package-owned decision is protected by a unit
test or has a recorded reason, not when a percentage is reached.

## Test-Technique Assessment

EPIC #1347 requires every package subissue to assess each test level. T1 records the final decision
in `coverage-evidence.md`; the initial assessment is:

| Technique | Initial assessment |
| --- | --- |
| Unit tests | Selected. Primary objective; in-file `#[cfg(test)]` modules next to the code. |
| Package integration tests | Not selected initially. The package has no network boundary of its own; a `tests/` target is added only if a file plan records why a public-API contract is clearer there. |
| Runnable examples | Not selected initially. The existing compile and compile-fail documentation tests in `statistics/repository.rs` are reviewed with their file. |
| Root `tests/` or `packages/e2e-tools/` | Not selected. Real HTTP behavior is owned by `axum-http-server` (#2136, #2140) and root tests; any coverage relied on there is linked, not duplicated. |
| Mutation testing | Selected as a bounded sample in T6 on a changed highest-risk seam, run with `RUSTFLAGS="--cap-lints=warn"` and a stated reason for every unviable mutant (#1348 lesson). |
| Property-based testing | Try where a seam has a natural property: the peer-IP selection decision over arbitrary `PeerIp` values, policies, and connection addresses (for example, "with the override disabled, the selected IP is never a client-supplied literal"), and the protocol-to-domain announcement mapping. If an attempt shows no value over example-based tests, record that in the file plan and do not add the dependency. When kept, `quickcheck` is added following the `add-rust-dependency` skill, as in #1348. |
| Fuzzing | Not selected. Untrusted query-string and header parsing is owned by `http-protocol`; this package receives typed requests. |
| Benchmarks | Out of scope. The existing Criterion benches are not tests of correctness. |

## Scope

### In Scope

- Produce a complete per-file module inventory for `packages/http-core/src/` before changing
  tests. Each row records current unit-only coverage, selected package-owned behavior,
  property-test candidacy, and an initial hypothesis. The hypothesis guides processing order; it
  does not skip a file.
- Process every Rust source file in the package through the per-file workflow below, one file at a
  time, creating its file test plan right before work on that file starts.
- Deliver the processed files through the planned [pull-request slices](#pull-request-slices),
  updating the evidence and the EPIC coverage tables in each slice.
- Measure fresh aggregate/global and unit-only package-source coverage using clean, reproducible
  `cargo llvm-cov` runs, and integration-only coverage if a package integration target is added.
  Keep the scopes in separate issue-local evidence tables.
- Add focused, deterministic unit tests for package-owned decisions, starting with the
  highest-risk peer-address decision.
- Add a package integration test only when a file test plan records why a unit test cannot protect
  the behavior or why the integration boundary is demonstrably clearer. When a behavior is
  impractical to cover in this package, use the narrowest stable boundary: package integration
  first, then root `tests/` or `packages/e2e-tools/` only when the behavior is necessarily composed
  there. Record the chosen boundary and its rationale.
- When a package behavior is covered outside this package, add the exact external test path to this
  specification's `semantic-links.related-artifacts`, following the Semantic Skill Link Convention.
  Do not add broad directory links.
- Documentation-comment corrections in the files under test, when a plan finds them wrong.
- Hand off bugs and other production findings through the
  [findings procedure](#findings-during-implementation).
- Assess a bounded mutation-testing sample after the approved test increments are complete.
- Record one concise file test plan per source file under the issue-local `test-refactor-plans/`
  directory: one shared README for guardrails and roughly 40-line per-file plans, each with a
  Prose-First AAA Review section (#1348 retrospective).
- Collect reusable testing or workflow lessons, including candidate entries for the testing
  refactoring-pattern catalog, in issue-local `lessons.md`. Promoting them into `docs/testing/` or
  `.github/skills/` is a separate small PR after this issue closes, per the subissue guidelines.

### Out of Scope

- Production behavior changes in `packages/http-core/src/`. A bug fix or production clean-up found
  here goes to a separate issue through the findings procedure.
- Tracker business rules owned by `tracker-core` (announce handling, peer selection, whitelist
  policy, key validation, scrape aggregation), request parsing and `ip`/header classification owned
  by `http-protocol` (including `resolve_remote_client_addr`), and generic event-bus or metric
  behavior owned by `events` and `torrust-metrics`.
- HTTP transport, routing, extractors, and server lifecycle owned by `axum-http-server` (#2136,
  #2140).
- Shutdown, cancellation, and spawned-task lifecycle redesign owned by #1488.
- Arbitrary coverage-percentage targets or tests added only to move uncovered line totals.
- Benchmark changes under `packages/http-core/benches/`.
- Cross-cutting process, skill, pattern-catalog, or agent updates. Record candidates locally in
  `lessons.md`.
- Maintaining coverage details in the GitHub issue #1349 body; it points to this specification,
  which is the source of truth.

## Architectural Decisions

- Related ADRs: `docs/adrs/20260527175600_keep_protocol_and_domain_types_decoupled.md` (the
  announce request-to-domain mapping is an intentional adapter boundary) and
  `docs/adrs/20260721000000_make_network_configuration_per_tracker_instance.md` (the reverse-proxy
  mode and external IP come from each HTTP tracker instance).
- Package-local ADRs: none exist (`packages/http-core/docs/adrs/` is absent).
- ADRs to create: none known. Create one if this work identifies a durable package or
  cross-package ownership or design decision, placed by decision scope as the `create-adr` skill
  describes.

## Design and Ownership Review

This package owns the HTTP-specific application decisions: whether a private-mode request is
authenticated, the order of authentication, authorization, and client-IP resolution, which peer IP
is stored for an announce under the instance policy, the mapping from protocol announce requests to
domain announcements, the service-level error adaptation and failure-reason text, HTTP core event
publication and labels, and HTTP core statistics. Collaborator rules remain owned by
`tracker-core`, `http-protocol`, `events`, `torrust-metrics`, `axum-http-server`, root application
composition, and #1488.

Before adding or changing tests that involve asynchronous I/O, spawned tasks, or event-listener
teardown, the file test plan must define the interface under test, which component owns normal,
failure, cancellation, and drop-path cleanup, the absolute deadline bounding every awaited
operation, and the narrowest maintainable boundary.

## Bug-Fix Process

Not applicable to this issue. It is coverage and test-design work, not a known defect fix. A defect
found during the work is handed off through the
[findings procedure](#findings-during-implementation), and its fix follows
`.github/skills/dev/debugging/fix-bug/SKILL.md` in its own issue.

## Regression Test Strategy

Not applicable as a bug-fix strategy. For each selected coverage increment, choose the smallest
deterministic maintained boundary that protects the package-owned behavior and record it in the file
test plan and coverage evidence. Never write a test that pins behavior a handed-off finding reports
as wrong; leave it uncovered, with the finding as its owner, until the fix merges.

## Findings During Implementation

Writing tests often exposes bugs (#1348 found two, fixed by #2458 and #2505) and dead or misleading
code (removed by #2517 and #2521). Handle every finding that needs a production change this way:

1. **Stop the current increment** at a clean point: committed, or with an issue-local
   `interruption-note.md` (see [Commit Points](#commit-points)).
2. **Write the hand-off** in a new git-ignored folder `.tmp/1349-<short-topic>/` with two files:
   - `ISSUE.md` - a draft specification from `docs/templates/ISSUE.md`. A bug draft includes the
     `Bug-Fix Process` and `Regression Test Strategy` sections that the `fix-bug` skill requires,
     and the observed and expected behavior. Leave `github-issue: null` and `epic: null` unless
     the maintainer names a parent.
   - `HANDOFF.md` - the context the other agent needs that does not belong in the spec: the file
     and plan that found it, the evidence (a failing test kept out of the branch, a hand-written
     mutation, or a reproduction), the suspected root cause, the files the fix will probably touch
     and their overlap with remaining ledger rows, the recommended mode (parallel or pause, below),
     the validation commands, and a reminder not to edit this issue's records.
3. **Record it durably.** `.tmp/` is not committed, so add a progress-log entry and a `lessons.md`
   follow-up row that summarize the finding and name the `.tmp/` path. When the draft becomes a
   GitHub issue, replace the path with the issue number.
4. **Recommend a mode and let the maintainer decide:**
   - **Parallel** - the fix does not touch a remaining ledger row of this issue, or touches only
     rows already merged here, and does not change behavior that a remaining plan will test.
     Continue with the next file.
   - **Pause** - the fix will add or change code or tests in the current file or a remaining ledger
     row, or changes behavior a remaining plan will pin. Set the affected ledger rows to `BLOCKED`
     with the finding reference. Finished rows may still be delivered as an early pull request,
     because a pause is a natural [pull-request point](#pull-request-slices).
5. **Resume after a pause** once the fix merges: rebase onto `develop`, re-run the unit-only
   coverage of the affected rows, re-check their approved plans and any committed tests against the
   new code, record the result in the progress log, and set the rows back to `PENDING` or
   `IN_PROGRESS`.

Non-bug findings (dead code, misleading documentation outside the files under test, API
clean-ups) only need a `lessons.md` row unless the maintainer asks for a hand-off.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`, `DEFERRED`.

| ID | Status | Task | Notes / Expected Output |
| --- | ------ | ---- | ----------------------- |
| T1 | DONE | Create coverage evidence and complete module inventory | `coverage-evidence.md` records the baseline commands and totals from this spec, per-file uncovered regions, property-test candidacy, selected behavior, and initial hypothesis for every `packages/http-core/src/` file. No tests changed. |
| T2 | DONE | Create shared plan guidance and the first file test plan | `test-refactor-plans/README.md` holds the shared guardrails and the plan index, including the two #1348 testing lessons that apply here: re-check an assumed-unreachable error path against `tracker-core`'s exported store doubles before recording it untestable, and prove an ordering decision (for example, authentication before authorization before client-IP resolution) with an expectation-free double that fails on the first call. The first plan is `services-announce-tests.md`. No tests changed. |
| T3 | DONE | Review and approve the processing order and slice boundaries | Maintainer approves the ledger order and the pull-request slices; both may change at any time. |
| T4 | DONE | Process every source file through the per-file workflow | Every ledger row has a completed plan, recorded coverage result, completed-file review, and signed per-file commit, merged through its slice. All 13 rows are `DONE`; S1-S5 are merged and S6 is this PR. |
| T5 | DONE | Implement integration-test increments selected by file test plans | None: no plan selected a 3B increment, because the package has no network boundary of its own (see the final test-technique assessment in `coverage-evidence.md`). |
| T6 | DONE | Perform bounded mutation assessment | `mutation-evidence.md` records the whole-package run (96 mutants: 50 caught, 1 missed, 1 timeout, 44 unviable) and the reason for each unviable mutant. |
| T7 | DONE | Reconcile evidence and progress state | Ledger, plans, plan index, slice table, coverage evidence, acceptance rows, and both EPIC coverage tables agree (the S6 independent review found the EPIC tables stale; fixed). The only `IN_PROGRESS` label left is the S6 slice row, until its pull request merges. |
| T8 | DONE | Complete verification and acceptance review | Automatic checks, manual verification, the final Task Reviewer review (`agent-review-reports.md`), the acceptance review below, and the retrospective (`implementation-retrospective.md`) are recorded. |

The specification frontmatter `status` stays `planned`, and the EPIC #1347 subissue row stays
`TODO`, until the spec-only PR merges and T1 starts; both then change to `in-progress` and
`IN_PROGRESS` in the T1 commit (the same sequence #1348 followed).

After the last T4/T5 test-producing increment, stop for maintainer review before T6-T8 or the final
pull request.

## Pull-Request Slices

The implementation is merged in slices. Each slice is a pull-request point: when its rows are done,
open a PR, get it reviewed, and merge it. The spec-only PR comes first.

| Slice | Content | Ledger rows | Branch | PR | Status |
| --- | --- | --- | --- | --- | --- |
| S0 | This specification and the EPIC registration with the baseline | - | `1349-1347-add-tests-http-core-spec` | #2546 | DONE |
| S1 | T1-T3: coverage evidence, inventory, shared plan guidance, first plan, approved order | - | `1349-1347-add-tests-http-core-inventory` | #2558 | DONE |
| S2 | Announce service and its error mapping (ranks 1-2) | 1-2 | `1349-1347-add-tests-http-core-announce` | #2575 | DONE |
| S3 | Scrape service (rank 3) | 3 | `1349-1347-add-tests-http-core-scrape` | #2586 | DONE |
| S4 | Events and composition (ranks 4-5) | 4-5 | `1349-1347-add-tests-http-core-events-container` | #2593 | DONE |
| S5 | Statistics (rank 6) | 6-10 | `1349-1347-add-tests-http-core-statistics` | #2608 | DONE |
| S6 | Crate root and wiring (rank 7), then T5-T8 | 11-13 | `1349-1347-add-tests-http-core-final` | - | IN_PROGRESS |

Rules:

- **Pull-request point.** A slice is ready for a PR when its ledger rows are `DONE` with one signed
  commit each, the maintainer has approved the last completed-file review, the slice's evidence and
  the EPIC tables are updated, the pre-push checks pass, and the independent Task Reviewer has
  reviewed the slice's rows (recorded in `agent-review-reports.md`). Running the reviewer per slice
  replaces #1348's single late review.
- **Each PR updates the records it delivers.** The ledger, plan index, slice table, the slice's
  coverage measurement in `coverage-evidence.md`, the EPIC "Latest" coverage values, and the
  progress log. The next slice's work starts from those merged records.
- **Issue linking.** S0-S5 use `Related to #1349`; only S6 uses `Closes #1349` (see the
  `open-pull-request` skill). Titles follow the repository convention, for example
  `test(http-core): [#1349] cover announce peer-IP selection`.
- **Branches.** The `create-issue` skill reserves the base name `1349-1347-add-tests-http-core` for
  a single implementation PR. With several slices, each PR needs its own branch, so the slice
  branches below replace that reserved name on purpose, and the base name stays unused rather than
  naming S1 (which would suggest it carries the whole implementation). Cut each slice branch from
  the latest `develop` only after the previous slice PR
  has merged. Do not stack slice branches (maintainer decision): while a slice PR is in review,
  only review fixes, findings hand-offs, and the next file's plan draft (kept outside git until
  its branch exists) may proceed. Keep at most one open implementation PR. The frontmatter
  `branch` and `related-pr` fields name the current slice; this table keeps the full history
  (issue frontmatter allows one PR; see draft
  `docs/issues/drafts/2264-support-multiple-related-prs-in-issue-frontmatter/ISSUE.md`).
- **Re-slicing.** The maintainer may split, merge, or reorder slices at any time, for example to
  deliver finished rows early when a finding pauses the work. Record each change in the progress
  log.
- **Manual verification and mutation sampling run once, in S6.** S1-S5 change only tests and
  documentation, so the built tracker behaves identically before and after them.

## Commit Points

| Task | Coherent change set | Commit policy |
| ---- | ------------------- | ------------- |
| Spec (S0) | This specification, the EPIC registration with the baseline, and the PR review audit | One signed `docs(issues)` commit after maintainer approval, plus a follow-up signed commit that records the spec-only PR number once the PR exists, plus any review-fix commits, plus the review-audit commit (see the "Review round" row below); merged through the spec-only PR. |
| T1 | Coverage evidence and module inventory | Documentation-only commit after review. |
| T2-T3 | Shared plan guidance, first plan, and approved order | Documentation-only commit before test-producing work. |
| T4 | One source file: its plan, test refactor, unit tests, and recorded coverage | One commit per completed file after its completed-file review. A no-change conclusion is committed as documentation. Never mix two source files in one commit. |
| Slice close | Slice coverage measurement, EPIC table update, reviewer report, records | One documentation commit per slice, right before its PR. |
| Review round (every PR, including this one) | `docs/pr-reviews/pr-<PR>-review/PR-REVIEW.md`, plus the review-fix commits it records | One signed `docs(pr-reviews)` commit for the audit, separate from the fixes, as `process-pr-review` requires; each independent fix is its own signed commit. |
| T5 | One integration increment | Commit after focused validation and recorded boundary rationale. |
| T6 | Mutation evidence | Commit if it records a material decision, survivor, or follow-up; otherwise include in final evidence. |
| T7 | Reconciliation fixes | Separate documentation commit when it materially changes evidence. |
| T8 | Final verification and completion evidence | Commit after maintainer review, final verification, and acceptance review. |

Use Conventional Commit messages that name the narrow affected area, for example
`test(http-core): cover <module contract>` or `docs(issues): record http core coverage inventory`.
All commits must be GPG signed. Cite commits in issue-local evidence by their unique Conventional
Commit subject, never by branch SHA, until they merge.

When work stops mid-task, record a one-paragraph `interruption-note.md` in the issue folder: the
next action, the validation state at stop, and any uncommitted-work locations. Remove it when the
work resumes. It is not the findings hand-off: that is `HANDOFF.md` in a git-ignored `.tmp/` folder,
defined in [Findings During Implementation](#findings-during-implementation).

## Per-File Workflow

T4 applies this workflow to every Rust source file in `packages/http-core/src/`, one file at a time,
in ledger order. Only one file may be `IN_PROGRESS`.

### The File Test Plan

Each file is tracked by one file test plan: `test-refactor-plans/<module>-tests.md`. Create it right
before starting the file, never in advance. The plan is about 40 lines with these sections:

1. **Current state** - unit-only coverage from the latest report, existing tests, initial
   hypothesis.
2. **Current tests review** - each existing test checked against `write-unit-test`. Conclusion:
   `refactor` (list the smells) or `clean`.
3. **Coverage and decision analysis** - what the module decides, and the uncovered lines or regions
   grouped by behavior, each classified as:
   - **3A unit** - package-owned decision testable at the module's API (preferred);
   - **3B integration** - only observable through a package integration boundary; record why;
   - **collaborator-owned / lifecycle (#1488) / platform** - not testable here; name the owner;
   - **finding** - looks wrong; handed off through the findings procedure, not tested here.
4. **Steps table** - `R1` refactor, `R2` add unit tests, `R3` select integration increment or
   `none`, `R4` record resulting coverage; each `TODO`/`DONE`/`SKIPPED`.
5. **Prose-First AAA Review** - for every added or changed test, the temporary Arrange, Act, and
   Assert prose and how the code was refactored to express it.
6. **Results** - unit-only coverage before and after, tests added or refactored, mutation proofs,
   review outcome.
7. **Progress log**.

### Steps for One File

| Step | Action | Gate before continuing |
| --- | --- | --- |
| 0 | Set the ledger row to `IN_PROGRESS`; create the plan with sections 1-3. | Maintainer approval of the plan. |
| 1 | R1: refactor current tests if section 2 said `refactor`. Run focused tests, `cargo +nightly fmt --all -- --check`, `git diff --check`. Prose-first AAA review. | Maintainer review of the refactor result; skipped when section 2 said `clean`. |
| 2 | R2: add the approved unit tests. Same validation plus the test-smell and prose-first AAA review, recorded in section 5. | Self-review; covered by step 5. |
| 3 | R3: record any selected 3B increment in the plan and T5 queue. | None. |
| 4 | R4: re-run unit-only coverage; record before/after in the plan and `coverage-evidence.md`. | None. |
| 5 | Mark the plan and ledger row `DONE`. | Maintainer completed-file review. |
| 6 | Commit the file's plan, tests, and evidence as one signed commit. | Pre-commit gate passes. |
| 7 | If the row closes its slice, follow the [pull-request point](#pull-request-slices) rule. | The slice PR is opened. |

Helpers must hide only incidental mechanics and are justified by a meaningful named action and
abstraction-level alignment, not by caller count. Test bootstraps construct only the collaborators
the test needs; they do not reuse production container factories.

## Source File Ledger

States: `PENDING` (no plan yet), `IN_PROGRESS`, `BLOCKED` (paused by a finding; the notes name it),
`DONE`, `REMOVED` (the file no longer exists; the row keeps the history). Rows follow
[Key Package Responsibilities](#key-package-responsibilities), most critical first; within a
responsibility, the module holding the decision comes before its supporting modules. Unit-only
lines are the S2 baseline measurement (see [Baseline Coverage](#baseline-coverage)).

| Order | Source module | Rank | Slice | Unit-only lines | Tests | Initial hypothesis | State | Plan |
| ---: | --- | ---: | --- | ---: | ---: | --- | --- | --- |
| 1 | `services/announce.rs` | 1-2 | S2 | 675 / 727 (92.85%) | 8 | Property candidate (peer-IP selection) | DONE | [services-announce-tests.md](test-refactor-plans/services-announce-tests.md) (done; 843 / 851 lines after) |
| 2 | `services/error_mapping.rs` | 2 | S2 | 0 / 18 (0.00%) | 0 | Test candidate | DONE | [services-error-mapping-tests.md](test-refactor-plans/services-error-mapping-tests.md) (done; 26 / 29 lines after, wildcard arm untestable) |
| 3 | `services/scrape.rs` | 3 | S3 | 507 / 540 (93.89%) | 8 | Test candidate (error paths) | DONE | [services-scrape-tests.md](test-refactor-plans/services-scrape-tests.md) (done; 533 / 537 lines after) |
| 4 | `event.rs` | 4 | S4 | 174 / 187 (93.05%) | 7 | Probably small change | DONE | [event-tests.md](test-refactor-plans/event-tests.md) (done; 135 / 135 lines after) |
| 5 | `container.rs` | 5 | S4 | 0 / 73 (0.00%) | 0 | Test candidate | DONE | [container-tests.md](test-refactor-plans/container-tests.md) (done; 27 tests, all 73 production lines covered) |
| 6 | `statistics/event/handler.rs` | 6 | S5 | 158 / 161 (98.14%) | 5 | Probably no change | DONE | [statistics-handler-tests.md](test-refactor-plans/statistics-handler-tests.md) (done; 10 cases, only the two decided lines uncovered) |
| 7 | `statistics/event/listener.rs` | 6 | S5 | 133 / 148 (89.86%) | 4 | Lifecycle (#1488) | DONE | [statistics-listener-tests.md](test-refactor-plans/statistics-listener-tests.md) (done; 308 / 310 lines after) |
| 8 | `statistics/metrics.rs` | 6 | S5 | 48 / 57 (84.21%) | 1 | Test candidate | DONE | [statistics-metrics-tests.md](test-refactor-plans/statistics-metrics-tests.md) (done; 141 / 141 lines after) |
| 9 | `statistics/repository.rs` | 6 | S5 | 21 / 24 (87.50%) | 0 (3 doc tests) | Test candidate | DONE | [statistics-repository-tests.md](test-refactor-plans/statistics-repository-tests.md) (done; 108 / 108 lines after) |
| 10 | `statistics/mod.rs` | 6 | S5 | 8 / 8 (100.00%) | 0 | Probably small change (metric-name contract) | DONE | [statistics-mod-tests.md](test-refactor-plans/statistics-mod-tests.md) (done; 32 / 32 lines after, name and exposition pinned) |
| 11 | `lib.rs` | 7 | S6 | 27 / 27 (100.00%) | 0 | Test support only | DONE | [lib-tests.md](test-refactor-plans/lib-tests.md) (done; 97 / 97 lines after, log target and `RecordingSender` pinned) |
| 12 | `services/mod.rs` | 7 | S6 | No executable entries | 0 | Wiring only | DONE | [services-mod-tests.md](test-refactor-plans/services-mod-tests.md) (done; no test, module comment corrected to "does not depend on the Axum web server") |
| 13 | `statistics/event/mod.rs` | 7 | S6 | No executable entries | 0 | Wiring only | DONE | [statistics-event-mod-tests.md](test-refactor-plans/statistics-event-mod-tests.md) (done; no change) |

## Progress Tracking

### Workflow Checkpoints

- [x] Checked for an existing `http-core` package-testing spec; none was present. The former
      `axum-rest-api-server` draft misfiled under #1349 was already moved to
      `docs/issues/drafts/1347-add-tests-axum-rest-api-server/`.
- [x] Reviewed EPIC #1347, the #1347 subissue-spec guidelines, the `create-issue` workflow, and the
      #1348 specification, retrospective, and lessons used as the template.
- [x] Local branch `1349-1347-add-tests-http-core-spec` created from `develop` for the spec-only PR.
- [x] Clean baseline coverage measured and recorded in this spec and the EPIC tables.
- [x] Specification reviewed and approved by the maintainer, Jose Celano (2026-10-15).
- [x] GitHub issue #1349 title and body refreshed to the current package name and this spec.
- [x] Spec-only PR (S0) merged into `develop` before implementation (#2546).
- [x] Complete module inventory and coverage evidence recorded (T1).
- [x] Shared plan guidance and the first file test plan created; processing order and slices
      approved (T2-T3).
- [x] Every ledger row is `DONE` with its own file test plan, merged through its slice (S6 through the final pull request).
- [x] Maintainer review after the final test-producing increment (the maintainer approved the completed-file reviews
      of S6 and the T6-T8 plan on 2026-10-21).
- [x] Automatic verification completed with toolchain-qualified evidence.
- [x] Manual verification recorded in `manual-verification-evidence.md`.
- [x] Bounded mutation assessment recorded in `mutation-evidence.md`.
- [x] Task Reviewer reports for every slice recorded in `agent-review-reports.md`.
- [x] Acceptance criteria reviewed after implementation.
- [x] Implementation completion review recorded (`implementation-retrospective.md`).
- [ ] Issue closed and spec moved to `docs/issues/closed/`.

### Progress Log

- 2026-10-15 16:18 UTC - GitHub Copilot - Created this specification on branch
  `1349-1347-add-tests-http-core-spec`, using #1348 as the template. Measured a clean baseline at
  `a9d8c5703` (32 unit tests; unit-only and aggregate lines 89.06%) and recorded it here and in the
  EPIC tables. Added the pull-request slices and the findings procedure the maintainer asked for
  after #1348. No tests or production code changed.
- 2026-10-15 20:27 UTC - Jose Celano - Approved the specification with these decisions: keep the
  inventory as its own docs-only slice (S1), run the Task Reviewer before every slice PR, do not
  stack slice branches (start each slice after the previous one merges), and refresh the stale
  GitHub issue title and body to point to this specification.
- 2026-10-15 20:30 UTC - GitHub Copilot - Refreshed the GitHub issue #1349 title and body and
  opened spec-only PR #2546 (<https://github.com/torrust/torrust-tracker/pull/2546>) with
  `Related to #1349`.
- 2026-10-16 07:04 UTC - GitHub Copilot - Processed the PR #2546 review round (Copilot F5-F6 and
  da2ce7 F1-F4); all six findings are fixed and their threads resolved, recorded in
  [the review audit](../../../pr-reviews/pr-2546-review/PR-REVIEW.md). Re-stamped the earlier
  entries from the real event times (F1).
- 2026-10-16 07:28 UTC - GitHub Copilot - da2ce7 approved PR #2546 at the head after the first
  round's fixes and left two `Nit` findings (F7, F8 in the audit); both are fixed and resolved. The
  Commit Points table now covers the review-audit commit for this and every later slice PR.
- 2026-10-16 08:14 UTC - GitHub Copilot - da2ce7 approved the head again and left one `Nit`
  finding (F9 in the audit): the S0 Commit Points row now names the `Review round` row instead of
  calling it the next row. Fixed and resolved.
- 2026-10-16 08:35 UTC - GitHub Copilot - da2ce7 approved the head with no finding. Copilot's
  re-review left one `Minor (inferred)` finding (F10 in the audit): the PR description still said the EPIC row was
  `IN_PROGRESS`. The description was corrected and the thread resolved; no repository file changed
  for it.
- 2026-10-16 10:23 UTC - GitHub Copilot - da2ce7 approved the head again and left one `Minor`
  finding (F11 in the audit): Copilot's review had marked F10 "Medium severity", so F10 is
  `Minor (inferred)`. Fixed in the audit and in the entry above, and resolved.
- 2026-10-16 10:30 UTC - GitHub Copilot - Copilot's third re-review left two findings (F12 and F13
  in the audit): the interruption note is now `interruption-note.md`, distinct from the findings
  `HANDOFF.md`, and the 2264 draft is linked by its file. Both fixed and resolved.
- 2026-10-16 10:55 UTC - GitHub Copilot - Copilot's fourth review has no finding. da2ce7 approved
  the head again with one `Minor` finding (F14 in the audit): the audit named the wrong round for
  one review's head. Fixed and resolved.
- 2026-10-16 11:35 UTC - Jose Celano - Merged spec-only PR #2546 into `develop` (merge commit
  subject `Merge torrust/torrust-tracker#2546: docs(issues): [#1349] add issue specification for
  http-core package tests`).
- 2026-10-16 15:42 UTC - GitHub Copilot - Started S1 on branch `1349-1347-add-tests-http-core-inventory` and
  set the specification to `in-progress`. T1: `packages/http-core/` had changed since the spec
  baseline (#2536 through #2545), so the package was re-measured at `04013e7f1` (stable Rust 1.99.0,
  `cargo-llvm-cov 0.6.16`): lines 1,746 / 1,962 (88.99%), 32 unit tests, unit-only equal to
  aggregate; [coverage-evidence.md](coverage-evidence.md) records the 13-file inventory, the
  uncovered-line groups, and the regions that line coverage hides (the early-return paths of
  `handle_announce` and `handle_scrape`). Three dead-code candidates (the `ConnectionContext`
  getters, `ClientConnectionContext::port`, and `Metrics::set_gauge`) await their file plans. T2:
  created [the shared plan guidance](test-refactor-plans/README.md) and the proposed
  [announce plan](test-refactor-plans/services-announce-tests.md). T3 awaits the maintainer's
  approval of the announce plan, the ledger order and the slice boundaries. No test or production
  code changed.
- 2026-10-16 16:02 UTC - Jose Celano - Approved the announce plan, the ledger order, slices S1-S6,
  and adding `rstest` as a dev-dependency in S2. T3 is `DONE`. S1 awaits the independent Task
  Reviewer, the pre-push checks and its PR.
- 2026-10-17 08:22 UTC - GitHub Copilot - The independent Task Reviewer failed S1 with three
  `MEDIUM` and four `LOW` findings ([agent-review-reports.md](agent-review-reports.md)): a
  misclassified listener region, a wrongly described scrape region, the announce ledger row left
  `PENDING`, a stale doc-test count, a missing test-name smell, and an unexplained missed-line
  difference. All six blocking items were fixed in separate commits (the ledger row is now
  `IN_PROGRESS`), and the re-review passed. Next: the pre-push checks and the S1 pull request.
- 2026-10-17 08:25 UTC - GitHub Copilot - The pre-PR check found `develop` 64 commits ahead,
  including #2244 (merged through #2552), which changed `statistics/metrics.rs` again. Rebased S1
  onto `develop` (no conflicts, signatures kept) and re-measured at `a3810b813`: lines 1,737 / 1,953
  (88.94%), regions 90.40%, functions 84.21%, 32 unit tests, unit-only equal to aggregate. Only
  `statistics/metrics.rs` changed (lines 34 / 43, 79.07%, after the new `whole_total` helper); the
  uncovered lines and every other file are unchanged. Updated the evidence, the baseline paragraph,
  the ledger row and the EPIC rows.
- 2026-10-17 09:23 UTC - GitHub Copilot - The Task Reviewer failed the re-measurement commit with one
  `MEDIUM` and two `LOW` findings (a stale base commit, reproduction commands naming the superseded
  reports, and one sentence) and passed after the fixes
  ([agent-review-reports.md](agent-review-reports.md)). Next: the pre-push checks and the S1 pull
  request.
- 2026-10-17 09:28 UTC - GitHub Copilot - Opened the S1 pull request #2558
  (<https://github.com/torrust/torrust-tracker/pull/2558>) with `Related to #1349`.
- 2026-10-17 12:23 UTC - GitHub Copilot - Copilot's review of PR #2558 found one `Minor` issue (F1 in
  the audit): the plan's precedence test never gives the service an external IP. Fixed in the
  announce plan and resolved; the review audit is [PR #2558 review audit](../../../pr-reviews/pr-2558-review/PR-REVIEW.md).
- 2026-10-19 11:31 UTC - GitHub Copilot - da2ce7 approved the S1 head on 2026-10-18 with three
  findings (F2-F4 in the [audit](../../../pr-reviews/pr-2558-review/PR-REVIEW.md)): the announce plan
  gains R2k (the instance public URL in the event) and a Named Mutations table, and the evidence
  now says that `ClientConnectionContext::port` is dead only together with `client_port`. All three
  are fixed and resolved. The announce plan is amended a third time; the maintainer confirms the
  amendments at the R1 and R2 reviews.
- 2026-10-19 12:48 UTC - GitHub Copilot - Copilot's third review raised one `Minor` finding (F5 in
  the audit): mutation M5 does not compile under the workspace `unused` denial. Verified by applying
  it and fixed in the plan (M5 runs with `--cap-lints=warn`); M1-M4 compile by default. Resolved.
- 2026-10-19 15:36 UTC - GitHub Copilot - da2ce7 approved the head on 2026-10-19 13:05 UTC with one
  `Nit` (F6 in the audit): F5's `Reviewer finding ID` should be `N/A`. Fixed and resolved.
- 2026-10-19 16:01 UTC - GitHub Copilot - Copilot's fourth review raised one `Nit` (F7 in the
  audit): the audit recorded the wrong source finding ID for F6. Fixed, with every entry
  cross-checked against its source comment, and resolved.
- 2026-10-19 16:30 UTC - GitHub Copilot - Copilot's fifth review raised four threads (F8-F11 in the
  audit): the announce plan now lists its five post-approval amendments, each pending the
  maintainer's confirmation at the R1 or R2 review, and the specification links the PR #2558
  audit. Three of the threads were one comment on three lines. All are resolved.
- 2026-10-20 12:19 UTC - GitHub Copilot - The S1 pull request #2558 was merged (`87690b15e`) at the
  maintainer's request, before da2ce7 acknowledged the last head; all 11 review threads were
  resolved. S1 is `DONE`.
- 2026-10-20 12:30 UTC - GitHub Copilot - S2 started on `1349-1347-add-tests-http-core-announce`.
  `develop` had changed `packages/http-core/` again (#2454 and #2560), so the package was
  re-measured at `87690b15e`: lines 1,751 / 1,970 (88.88%), regions 90.41%, functions 84.29%, 33
  unit tests, unit-only equal to aggregate. `services/error_mapping.rs` gained a wildcard arm
  (0 / 18 lines) that no test can reach while `TrackerCoreError` has four variants, and
  `statistics/metrics.rs` gained its first test (48 / 57 lines). The evidence, the ledger and the
  EPIC rows use the new baseline.
- 2026-10-20 12:58 UTC - GitHub Copilot - `services/announce.rs` is `DONE` (S2): R1 and the completed
  file were approved by the maintainer. 48 cases replace 8 tests, `rstest = "0.28.0"` became a
  dev-dependency, every named mutation fails its test, and unit-only line coverage of the file went from
  92.85% to 99.06% (package 88.88% to 90.86%; the totals include the test module). See the
  [plan](test-refactor-plans/services-announce-tests.md) and
  [coverage evidence](coverage-evidence.md#per-file-results). No finding needed a hand-off. Next:
  `services/error_mapping.rs`, which closes S2.
- 2026-10-20 14:56 UTC - GitHub Copilot - `services/error_mapping.rs` is `DONE` (S2): the maintainer
  approved the plan and the completed file. One `rstest` table of four cases pins the failure-reason
  prefix of each `TrackerCoreError` variant; mutations E1 and E2 each fail their own case. The
  wildcard arm added by #2454 cannot be reached by a test and stays uncovered (three lines), owned by
  `tracker-core`. Interim unit-only package coverage after S2: lines 91.34%, regions 92.44%,
  functions 89.59% (totals include the test modules). Both ledger rows of S2 are `DONE`. Next: the
  independent Task Reviewer, the pre-push checks and the S2 pull request.
- 2026-10-20 16:05 UTC - GitHub Copilot - The independent Task Reviewer passed S2 with one `MEDIUM`
  and two `LOW` findings ([agent-review-reports.md](agent-review-reports.md)). The `MEDIUM` was a
  surviving mutant (an empty `ip=` skipping the loopback substitution) that contradicted the
  "no mutation survives" claim; it is fixed with a two-case test, so `services/announce.rs` has 50
  cases and the package 79 unit tests. The mutation E2 wording and a duplicated failure-reason
  literal were fixed too. Final S2 package coverage: lines 1,919 / 2,101 (91.34%), regions 2,511 /
  2,716 (92.45%), functions 198 / 221 (89.59%). Next: the pre-push checks and the S2 pull request.
- 2026-10-20 16:18 UTC - GitHub Copilot - The pre-push checks passed and the S2 pull request #2575
  (<https://github.com/torrust/torrust-tracker/pull/2575>) was opened with `Related to #1349`. It
  was not rebased onto the latest `develop`, which changed only outside `packages/http-core`;
  `git merge-tree` reports no conflict.
- 2026-10-20 16:47 UTC - GitHub Copilot - The S2 pull request #2575 was merged (`fe70cf908`). Copilot
  reviewed the last head with no open issue and da2ce7 did not reply within the agreed 15 minutes, so
  the maintainer's ACK was posted on the head and no review round was needed. S2 is `DONE`.
- 2026-10-20 16:50 UTC - GitHub Copilot - S3 started on `1349-1347-add-tests-http-core-scrape`, cut from
  `develop` at the merge of #2575. `packages/http-core` changed on `develop` only through S2, so the
  baseline is the final S2 measurement: lines 1,919 / 2,101 (91.34%), regions 2,511 / 2,716 (92.45%),
  functions 198 / 221 (89.59%), 79 unit tests; no re-measurement was needed.
- 2026-10-20 17:09 UTC - GitHub Copilot - `services/scrape.rs` is `DONE` (S3): the maintainer approved the
  plan (including a shared `RecordingSender` moved into `crate::tests`), the R1 refactor and the
  completed file. The 8 tests became 19 cases; the real database is replaced by `MockTorrentMetricsStore`;
  mutations M1-M4 and two probes each fail only their tests; unit-only line coverage of the file went
  from 93.89% to 99.19%. One planned test (a public tracker ignoring a presented key) was dropped
  because no mutation can fail it. Package after S3: lines 1,900 / 2,053 (92.55%), regions 2,492 / 2,669
  (93.37%), functions 228 / 244 (93.44%), 90 unit tests. No finding needed a hand-off. Next: the
  independent Task Reviewer, the pre-push checks and the S3 pull request.
- 2026-10-20 17:48 UTC - GitHub Copilot - The independent Task Reviewer passed S3 with one `MEDIUM` and six
  `LOW` findings ([agent-review-reports.md](agent-review-reports.md)). The `MEDIUM` was a surviving mutant:
  the zeroed branch of `handle_scrape` returning early skipped the client-IP resolution and the event, and
  no test covered the private-tracker path. It is fixed with a private-tracker event test and a private
  case in the unresolvable client-IP table. `RecordingSender` now records when the future returned by
  `send` is polled, one test pins that a tracker not behind a reverse proxy ignores a forwarded address,
  and the no-sender test asserts the data. The scrape file now has 22 cases and the package 93 unit tests.
  Final S3 package coverage: lines 1,949 / 2,102 (92.72%), regions 2,566 / 2,743 (93.55%), functions
  233 / 249 (93.57%). R1 and R2 were coded together although the plan gates R1 on a maintainer review; the
  maintainer approved R1 and the completed file together on 2026-10-20 at 17:09 UTC, which satisfies the
  gate. Next: the pre-push checks and the S3 pull request.
- 2026-10-20 17:54 UTC - GitHub Copilot - The pre-push checks passed and the S3 pull request #2586
  (<https://github.com/torrust/torrust-tracker/pull/2586>) was opened with `Related to #1349`. It was
  not rebased onto the latest `develop`, which changed only outside `packages/http-core`;
  `git merge-tree` reports no conflict.
- 2026-10-20 19:49 UTC - GitHub Copilot - The S3 pull request #2586 was merged (`fefefdd9e`). Copilot
  reviewed the last head with no open issue, the one cancelled CI job was re-run and passed, and
  da2ce7 did not reply within the agreed 15 minutes, so the maintainer's ACK was posted on the head.
  S3 is `DONE`.
- 2026-10-20 19:50 UTC - GitHub Copilot - S4 started on `1349-1347-add-tests-http-core-events-container`,
  cut from `develop` at the merge of #2586. `packages/http-core` did not change on `develop` after the
  S3 head, so the baseline is the final S3 measurement: lines 1,949 / 2,102 (92.72%), regions
  2,566 / 2,743 (93.55%), functions 233 / 249 (93.57%), 93 unit tests; no re-measurement was needed.
- 2026-10-20 20:41 UTC - GitHub Copilot - Correction to the 19:50 UTC entry: the claim that
  `packages/http-core` did not change after the S3 head was wrong. The S3 merge brought #2414, which changed
  `statistics/event/listener.rs` (production and tests). The S4 baseline was measured again at `fefefdd9e`:
  lines 2,040 / 2,197 (92.85%), regions 2,664 / 2,845 (93.64%), functions 244 / 260 (93.85%), 95 unit tests
  (see `coverage-evidence.md`). The S5 listener plan must start from the changed file.
- 2026-10-20 20:41 UTC - GitHub Copilot - `event.rs` is `DONE` (S4): the maintainer approved R1 and the
  completed file. The 7 tests are now 27 cases in 10 test functions, the dead `announce_events_match` helper
  and `pub mod test` are gone, and the four unused accessors are pinned as public API. Coverage of the file
  is 135 / 135 lines, 184 / 184 regions and 22 / 22 functions; the package has 115 unit tests. Mutations
  L1-L4 and six extra probes were killed.
- 2026-10-21 07:00 UTC - GitHub Copilot - `container.rs` is `DONE` (S4): the plan was approved on
  2026-10-20 at 20:49 UTC with D1 (hand-built tracker-core container) and D2 (test `initialize`), and the
  maintainer approved the completed file. The file went from 0 to 20 tests and all 73 production lines are
  covered; the package has 135 unit tests and 97.89% of lines. Mutations M1-M7 and eight extra probes were
  killed; one survived (the swarm registry's usage-statistics flag in `initialize`, not observable from this
  package) and is recorded in the plan with its owner.
- 2026-10-21 07:25 UTC - GitHub Copilot - The independent Task Reviewer passed S4 with four `LOW` findings and
  no required change: two label-table mutants survived in `event.rs` (the protocol and the address-type
  labels hard-coded, plus an unvaried port), one expected value was computed with a `match`, three
  `container.rs` tests had several reasons to fail or a split Act, and the `initialize` panic test used a
  fixed missing directory. All are fixed in the review-round commit (`event.rs` has 29 cases, `container.rs`
  23 tests) and the report is in `agent-review-reports.md`. The package has 140 unit tests: lines 2,616 /
  2,672 (97.90%), regions 3,397 / 3,457 (98.26%), functions 316 / 322 (98.14%). The slice awaits its
  pre-push checks and pull request.
- 2026-10-21 07:34 UTC - GitHub Copilot - The pre-push checks passed and the S4 pull request #2593
  (<https://github.com/torrust/torrust-tracker/pull/2593>) was opened with `Related to #1349`. It was
  not rebased onto the latest `develop`, which changed only documentation outside `packages/http-core`;
  `git merge-tree` reports no conflict.
- 2026-10-21 07:55 UTC - GitHub Copilot - Copilot reviewed the S4 pull request head and raised one finding:
  the event-delivery test never disabled usage statistics, so a bus gated on that policy would have passed.
  It is fixed by turning four `container.rs` tests over the policy into tables (the file has 27 tests and the
  package 144), with three new mutations killed only by the disabled cases. The review is audited in
  `docs/pr-reviews/pr-2593-review/PR-REVIEW.md`. The package now reads lines 2,536 / 2,592 (97.84%), regions
  3,264 / 3,324 (98.19%), functions 309 / 315 (98.10%).
- 2026-10-21 09:27 UTC - GitHub Copilot - The S4 pull request #2593 was merged (`b5ab35ae9`). Copilot had
  reviewed the last head with no open issue, CI was green, and da2ce7 did not reply within the agreed 15
  minutes, so the maintainer's ACK was posted on the head. S4 is `DONE`.
- 2026-10-21 09:30 UTC - GitHub Copilot - S5 started on `1349-1347-add-tests-http-core-statistics`, cut from
  `develop` at the merge of #2593. `packages/http-core` is identical to the S4 head, so the baseline is the
  final S4 measurement, re-run on this branch: lines 2,536 / 2,592 (97.84%), regions 3,264 / 3,324 (98.19%),
  functions 309 / 315 (98.10%), 144 unit tests. The statistics files measure `handler.rs` 158 / 161,
  `listener.rs` 215 / 239 (it changed after the inventory: #2414 added the drain), `metrics.rs` 48 / 57,
  `repository.rs` 21 / 24 and `mod.rs` 8 / 8 lines.
- 2026-10-21 09:42 UTC - GitHub Copilot - `statistics/event/handler.rs` is `DONE` (S5): the plan was approved
  at 09:34 UTC and the maintainer approved R1 and the completed file. The 5 tests are now 10 cases in 4 test
  functions; only the two lines the plan decides not to test are uncovered. Mutations H1-H4 and a probe were
  killed. The package has 149 unit tests: lines 2,521 / 2,577 (97.83%), regions 3,224 / 3,284 (98.17%),
  functions 314 / 320 (98.12%). The plan notes a question for the `metrics.rs` plan: the `tcp4`/`tcp6` counters
  select on the server binding's family while their comments say "peers". The S4-merge and S5-start log
  entries above were missing from the S5-start commit (a parallel edit raced with staging) and ship with this
  commit.
- 2026-10-21 09:59 UTC - GitHub Copilot - `statistics/event/listener.rs` is `DONE` (S5): the plan was approved
  at 09:46 UTC with D1 (test the two public entry points) and the maintainer approved the completed file. The
  six existing tests are unchanged and eight cases were added: the metrics policy for announce events, every
  queued event drained after cancellation, and the two entry points. The file is at 308 / 310 lines. Mutations
  K1-K5 and a probe were killed; the drain mutant that handles only the first event survived all six old
  tests. The package has 157 unit tests: lines 2,614 / 2,648 (98.72%), regions 3,339 / 3,380 (98.79%),
  functions 329 / 332 (99.10%).
- 2026-10-21 10:23 UTC - GitHub Copilot - `statistics/metrics.rs`, `statistics/repository.rs` and
  `statistics/mod.rs` are `DONE` (S5): the three plans were approved together at 10:03 UTC, with the
  family-semantics question (D1) answered "pin the behavior": the `tcp4`/`tcp6` getters select on the server
  binding's family, a test says so, and the "peers" wording of their comments is recorded in the plans with no
  hand-off. The maintainer approved the three completed files together. They add 17 tests; mutations G1-G5,
  B1-B4 and D1-D3 were killed, and the metric-name mutation fails only the new tests. The package has 174 unit
  tests: lines 2,818 / 2,840 (99.23%), regions 3,679 / 3,709 (99.19%), functions 359 / 360 (99.72%). All
  ledger rows of S5 are `DONE`; the slice awaits the independent review, the pre-push checks and its pull
  request.
- 2026-10-21 11:12 UTC - GitHub Copilot - The independent Task Reviewer passed S5 with one `MEDIUM` and four `LOW`
  findings. The `MEDIUM` was a false claim of mine in three plans: that the recorded sample time and the metric
  unit could not be observed through the package. `Metrics` is `PartialEq`, so they can, and seven mutants
  survived (an added 7 seconds in the handler, the listener, the repository and the metrics wrapper, and a wrong
  unit). All are fixed with whole-value tests and killed, together with a wiring survivor in the spawned listener
  and an error-swallowing survivor in the repository; the plans carry the corrections. The package has 183 unit
  tests: lines 2,974 / 2,996 (99.27%), regions 3,875 / 3,905 (99.23%), functions 372 / 373 (99.73%). The reviewer
  also asks that the `tcp4`/`tcp6` "peers" wording question (the getters select on the server binding's family)
  be carried into `lessons.md` in the retrospective, so it is not lost.
- 2026-10-21 11:18 UTC - GitHub Copilot - The pre-push checks passed and the S5 pull request #2608
  (<https://github.com/torrust/torrust-tracker/pull/2608>) was opened with `Related to #1349`. It was not
  rebased onto the latest `develop`, which changed only documentation outside `packages/http-core`;
  `git merge-tree` reports no conflict.
- 2026-10-21 12:25 UTC - GitHub Copilot - The S5 pull request #2608 was merged at 12:07 UTC (merge commit
  `889a6dc36`) after Copilot's round 2 was clean and Cameron did not reply within 15 minutes. S6 started on
  `1349-1347-add-tests-http-core-final`, cut from `889a6dc36`. `packages/http-core` is byte-identical to the S5
  head (`git diff af69b317c 889a6dc36 -- packages/http-core` is empty), and a clean unit-only measurement
  reproduced the S5 figures: 183 tests, lines 2,974 / 2,996 (99.27%), regions 3,875 / 3,905 (99.23%), functions
  372 / 373 (99.73%). `lib.rs` is 41 / 41 lines. Ledger rows 11-13 are next, then T5-T8.
- 2026-10-21 14:27 UTC - GitHub Copilot - `lib.rs` is `DONE` (S6): the three S6 plans were approved (including the
  comment correction of `services/mod.rs`), and `lib.rs` gained 8 tests (97 / 97 lines, 150 / 150 regions). All
  mutations were killed by the new tests only, including a sender that records at call time instead of on poll,
  which no earlier test noticed. Correction: the S6-start entry above is stamped 12:25 UTC but its commit was made
  at 13:21 UTC (the real time was not read when it was written).
- 2026-10-21 14:46 UTC - GitHub Copilot - `services/mod.rs` and `statistics/event/mod.rs` are `DONE` (S6), approved
  by the maintainer: no test for either (wiring only, no executable entry). The module comment of `services/mod.rs`
  said the services "does depend on the Axum web server" although `http-core` has no Axum dependency; it now says
  "does not depend". All 13 ledger rows are `DONE`; the independent review, the pre-push checks and T5-T8 remain.
- 2026-10-21 15:18 UTC - GitHub Copilot - T5-T8 evidence recorded after the maintainer approved the plan. T5: no
  integration increment was selected. T6: `cargo-mutants` ran the whole package (96 mutants: 50 caught; 1 missed, the
  debug log decided in the handler plan; 1 timeout, the listener's drain stop condition and the known #1488 limit;
  44 unviable, each with a stated reason); a first run was interrupted by the environment during its baseline and
  was rerun. Manual verification M1-M5 passed against a built tracker (a wrong first scrape URL is recorded in V2).
  Final measurement: 191 tests, lines 3,030 / 3,052 (99.28%), regions 3,983 / 4,013 (99.25%), functions 384 / 385
  (99.74%), identical for the aggregate and the unit-only runs. The `tcp4`/`tcp6` finding became a git-ignored draft
  with a hand-off, `.tmp/1349-tcp-family-counter-docs/`, which also covers the UDP counters that #1348 had recorded
  without an owner; the maintainer chose a draft in addition to the `lessons.md` row, and the recommended mode is
  parallel (no remaining ledger row touches the files). `lessons.md` and `implementation-retrospective.md` were
  written.
- 2026-10-21 15:29 UTC - GitHub Copilot - The independent Task Reviewer returned `REVIEW FAILED` for S6 and T5-T8 on
  one blocking finding: the EPIC tables and row still carried the S5 interim figures. It also asked for the AC12
  note and fixed-wording items; no code or measurement changed. All findings were handled in the commit
  `docs(issues): [#1349] address the S6 independent review` (see `agent-review-reports.md`); the EPIC tables now
  read lines 99.28%, regions 99.25% and functions 99.74%, T7 and T8 are `DONE`, and AC1-AC17 are verified in the
  Acceptance Verification table. Process slips disclosed: the final section of `coverage-evidence.md` was appended
  with a shell heredoc, and Python replacements were used instead of the edit tool for the slice-table status flips
  at the S6 start and for the two EPIC table rows; their content was reviewed afterwards. The pre-push checks and the
  pull request follow.

## Acceptance Criteria

- [x] AC1: A complete `packages/http-core/src/` module inventory is recorded with unit-only
      coverage, property-test candidacy, selected behavior, and an initial hypothesis for every
      source file, plus the final test-technique assessment.
- [x] AC2: Every source file has its own file test plan processed through the per-file workflow,
      with its resulting unit-only coverage recorded in `coverage-evidence.md`.
- [x] AC3: Fresh aggregate/global and unit-only coverage evidence (and integration-only, if a target
      is added) is recorded with clean reproducible commands and toolchain-qualified results.
- [x] AC4: Selected tests protect package-owned behavior at the narrowest maintainable boundary.
- [x] AC5: Any package integration test has a reviewed rationale.
- [x] AC6: No-change and deferral decisions enumerate uncovered lines or behavior groups and name
      the owner or reason.
- [x] AC7: Test-producing increments record the prose-first AAA review and focused validation in
      their plan before the next file begins.
- [x] AC8: Bounded mutation assessment is completed without a score target or CI gate.
- [x] AC9: Reconciliation confirms plans, evidence, the slice table, acceptance rows, and EPIC
      tables agree; the EPIC aggregate and unit-only tables record this package's baseline and
      final measurement.
- [x] AC10: Every highest-risk and high objective in Coverage Objectives is protected by a unit test
      or has a recorded owner-based reason.
- [x] AC11: The work is merged through the planned slices (or their recorded re-slicing), each with
      a Task Reviewer report, and only the last PR closes #1349.
- [x] AC12: Every finding that needs a production change has a hand-off draft, a progress-log entry,
      and a `lessons.md` row naming its draft path or issue, and no test pins its reported
      behavior.
- [x] AC13: `cargo +nightly fmt --all -- --check` passes.
- [x] AC14: `linter all` exits with code `0`.
- [x] AC15: `cargo test -p torrust-tracker-http-core` passes.
- [x] AC16: Manual verification scenarios are executed and recorded.
- [x] AC17: Acceptance criteria are re-reviewed after implementation.

## Verification Plan

### Automatic Checks

- `cargo llvm-cov clean --workspace`
- `cargo llvm-cov -p torrust-tracker-http-core --all-features --json`
- `cargo llvm-cov clean --workspace`
- `cargo llvm-cov -p torrust-tracker-http-core --all-features --lib --json`
- `cargo +nightly fmt --all -- --check`
- `cargo test -p torrust-tracker-http-core`
- `linter all`
- `TORRUST_GIT_HOOKS_LOG_DIR=.tmp ./contrib/dev-tools/git/hooks/pre-commit.sh --format=json`
- Pre-push checks before each slice PR.

If a package integration target is added, also run a clean integration-only report with
`--test <name>`. Every recorded result names the toolchain that produced it, for example "nightly
Rust toolchain" for formatting and "stable Rust 1.99.0" for coverage. Raw JSON reports stay in
git-ignored `.tmp/1349-coverage/`; only concise Markdown evidence is committed.

### Manual Verification Scenarios

Manual verification is human-oriented interaction with the built tracker using an isolated
configuration. Running automated tests never satisfies it. It runs once, in S6, against the tracker
built from `develop` with every earlier slice merged.

| ID | Scenario | Human-oriented command/steps | Expected Result | Status | Evidence |
| --- | --- | --- | --- | --- | --- |
| M1 | HTTP announce | Start an isolated public tracker and run `cargo run -p torrust-tracker-client --bin tracker_client -- http announce <url> <hash>`. | Valid bencoded announce response. | DONE | `manual-verification-evidence.md` V1 |
| M2 | HTTP scrape | Run `cargo run -p torrust-tracker-client --bin tracker_client -- http scrape <url> <hash>` for the announced hash. The `<url>` is the tracker's base URL. | Scrape returns the announced torrent. | DONE | `manual-verification-evidence.md` V2 |
| M3 | Peer IP override policy | With `use_ip_from_query_string = false`, send an announce with `ip=<literal>` using `curl`; repeat with the option `true`, then list the torrent's peers through the REST API. | Disabled: bencoded failure reason naming the disabled override. Enabled: the stored peer uses the supplied IP. | DONE | `manual-verification-evidence.md` V3 |
| M4 | Private mode | With `private = true`, announce and scrape with `curl` without a key, then with a key created through the REST API. | Announce without a key fails with an authentication failure reason; scrape without a key returns zeroed data; both succeed with the key. | DONE | `manual-verification-evidence.md` V4 |
| M5 | HTTP core statistics | After M1 and M2, read the REST API statistics. | The TCP4 announce and scrape counters reflect the requests made. | DONE | `manual-verification-evidence.md` V5 |

### Disposable Verification Scripts

None planned. `tracker_client`, `curl`, and the REST API cover every scenario. If a scenario needs
a script, record its issue-local path, why a maintained test does not replace it, and its retention
owner here before writing it.

### Acceptance Verification

| AC ID | Status | Evidence |
| ----- | ------ | -------- |
| AC1 | DONE | Inventory and T1 hypotheses in `coverage-evidence.md`; the final test-technique assessment ("Final Test-Technique Assessment"). |
| AC2 | DONE | All 13 ledger rows `DONE`; 13 plans in `test-refactor-plans/README.md`; per-file results in `coverage-evidence.md`. |
| AC3 | DONE | Commands, toolchain (stable Rust 1.99.1, nightly for formatting) and figures in "Final Measurement (T8)"; aggregate equals unit-only (no `tests/` directory); the S6 reviewer reproduced them with a clean run. |
| AC4 | DONE | 191 unit tests at module boundaries; no test crosses a package boundary. |
| AC5 | DONE | No package integration test was added; the rationale is the T5 row and the final assessment. |
| AC6 | DONE | The "22 uncovered lines" table names an owner or reason for each; the wiring-only plans record their conclusions. |
| AC7 | DONE | Each plan carries its prose-first AAA review and focused validation; one independent review per slice in `agent-review-reports.md`. |
| AC8 | DONE | `mutation-evidence.md`: the whole-package run, with a reason for every missed, timed-out and unviable mutant, and no score gate. |
| AC9 | DONE | Reconciled in T7; both EPIC tables carry the final figures. |
| AC10 | DONE | Every highest-risk and high objective is covered by unit tests with named mutation proofs in its plan, or has a recorded owner (error-mapping wildcard arm, the handler's failure path, the drain's stop condition under #1488). |
| AC11 | DONE | Seven pull requests: #2546, #2558, #2575, #2586, #2593 and #2608 are merged, each with a Task Reviewer report from S1 on; S6 is the final pull request, with this slice's report, and the only one with `Closes #1349`. |
| AC12 | DONE | The one finding that needs a decision outside this issue, the `tcp4`/`tcp6` counter meaning, has a draft hand-off (`.tmp/1349-tcp-family-counter-docs/`, git-ignored), a progress-log entry and a `lessons.md` row. By the maintainer's S5 decision (D1, "pin the behavior") the test `it_should_count_by_the_family_of_the_server_binding_and_not_of_the_client` pins the behavior as it is: the finding concerns the documented meaning, and the hand-off names the test as one that must change if the decision changes the behavior. |
| AC13 | DONE | `cargo +nightly fmt --all -- --check` exits 0 (nightly Rust toolchain). |
| AC14 | DONE | `linter all` exits 0, as part of every pre-commit run. |
| AC15 | DONE | `cargo test -p torrust-tracker-http-core`: 191 unit tests and 3 doc tests passed. |
| AC16 | DONE | `manual-verification-evidence.md` V1-V5. |
| AC17 | DONE | This table, after the S6 independent review and the fixes it asked for. |

## Risks and Trade-offs

- Indirect coverage from `axum-http-server` can hide missing package-local tests; mitigate by using
  unit-only evidence for every decision.
- Service tests need `tracker-core` collaborators; mitigate by constructing only the needed
  in-memory services rather than reusing production container factories. Where a path needs a
  failing or call-recording collaborator, use `tracker-core`'s exported `#[automock]` store doubles
  (`packages/tracker-core/src/databases/traits/`: keys, whitelist, torrent metrics) instead of
  recording the path as untestable.
- High baseline line coverage can suggest little work remains; mitigate with the per-file "what
  does this module decide?" analysis, which found #1348's two bugs in covered code.
- Several PRs mean more review and rebase overhead, and a slice may land with records that the next
  slice must update; mitigate by keeping each slice's records current at its PR point and
  reconciling in T7.
- Waiting for each slice to merge before starting the next can leave idle time during review;
  accepted by the maintainer in exchange for no stacked-branch rebases or record conflicts. Keep
  slices small so reviews are quick.
- A paused issue can go stale while a finding is fixed; mitigate with `interruption-note.md` and the resume
  steps in the findings procedure.
- Mutation testing can grow into a backlog; mitigate with a bounded sample and no score target.
- Adding a property-testing dependency increases maintenance cost; mitigate by requiring a recorded
  justification and preferring example-based tests for two or three enumerable cases.

## Implementation Completion Review

After implementation, compare the result with this specification, including whether the slices
reduced review cost compared with #1348's single PR. Create `implementation-retrospective.md` if
the work produces material design discoveries, slice or queue changes, or meaningful deviations
from the plan; otherwise add a progress-log entry explaining why none was needed.

## References

- Parent EPIC: https://github.com/torrust/torrust-tracker/issues/1347
- GitHub issue: https://github.com/torrust/torrust-tracker/issues/1349
- Template subissue: #1348 (`docs/issues/closed/1348-1347-add-tests-udp-core/`)
- Package: `packages/http-core/`
