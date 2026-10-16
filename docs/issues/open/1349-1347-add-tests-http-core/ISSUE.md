---
schema-version: 1
doc-type: issue
issue-type: task
status: planned
priority: p2
epic: 1347
github-issue: 1349
spec-path: docs/issues/open/1349-1347-add-tests-http-core/ISSUE.md
branch: "1349-1347-add-tests-http-core-spec"
related-pr: 2546
last-updated-utc: "2026-10-15 20:30"
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
files have no unit-only coverage: `container.rs` (0 / 73) and `services/error_mapping.rs` (0 / 15).
The rest have high line coverage, but #1348 showed that a covered file can still pin the wrong value
or miss a decision, so every file still gets its own plan.

The `torrust-tracker-axum-http-server` package and the root `tests/` exercise these services
indirectly. EPIC #1347 does not accept coverage from another package, an integration test, or an
end-to-end test as evidence that a feasible package-owned unit seam is protected.

T1 copies this baseline into `coverage-evidence.md`, adds per-file uncovered regions, and
re-measures only if `packages/http-core/` changed on `develop` after `a9d8c5703`. The EPIC #1347
Package Coverage Tracking tables record this baseline in both the aggregate and unit-only tables.

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

1. **Stop the current increment** at a clean point: committed, or with a `handoff.md` note.
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
| T1 | TODO | Create coverage evidence and complete module inventory | `coverage-evidence.md` records the baseline commands and totals from this spec, per-file uncovered regions, property-test candidacy, selected behavior, and initial hypothesis for every `packages/http-core/src/` file. No tests changed. |
| T2 | TODO | Create shared plan guidance and the first file test plan | `test-refactor-plans/README.md` holds the shared guardrails and the plan index; the first plan is `services-announce-tests.md`. No tests changed. |
| T3 | TODO | Review and approve the processing order and slice boundaries | Maintainer approves the ledger order and the pull-request slices; both may change at any time. |
| T4 | TODO | Process every source file through the per-file workflow | Every ledger row has a completed plan, recorded coverage result, completed-file review, and signed per-file commit, merged through its slice. |
| T5 | TODO | Implement integration-test increments selected by file test plans | Only if a plan selected a 3B increment; otherwise record why none was needed. |
| T6 | TODO | Perform bounded mutation assessment | `mutation-evidence.md` records configuration, timeout, scope, outcomes, and the reason for each unviable mutant. |
| T7 | TODO | Reconcile evidence and progress state | Ledger, plans, plan index, slice table, coverage evidence, acceptance rows, and both EPIC coverage tables agree. Grep completed records for stray `TODO`/`IN_PROGRESS` labels and stale frontmatter. |
| T8 | TODO | Complete verification and acceptance review | Automatic checks, manual verification, the final Task Reviewer review, acceptance review, and the retrospective decision are recorded. |

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
| S0 | This specification and the EPIC registration with the baseline | - | `1349-1347-add-tests-http-core-spec` | #2546 | IN_PROGRESS |
| S1 | T1-T3: coverage evidence, inventory, shared plan guidance, first plan, approved order | - | `1349-1347-add-tests-http-core-inventory` | - | TODO |
| S2 | Announce service and its error mapping (ranks 1-2) | 1-2 | `1349-1347-add-tests-http-core-announce` | - | TODO |
| S3 | Scrape service (rank 3) | 3 | `1349-1347-add-tests-http-core-scrape` | - | TODO |
| S4 | Events and composition (ranks 4-5) | 4-5 | `1349-1347-add-tests-http-core-events-container` | - | TODO |
| S5 | Statistics (rank 6) | 6-10 | `1349-1347-add-tests-http-core-statistics` | - | TODO |
| S6 | Crate root and wiring (rank 7), then T5-T8 | 11-13 | `1349-1347-add-tests-http-core-final` | - | TODO |

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
- **Branches.** Cut each slice branch from the latest `develop` only after the previous slice PR
  has merged. Do not stack slice branches (maintainer decision): while a slice PR is in review,
  only review fixes, findings hand-offs, and the next file's plan draft (kept outside git until
  its branch exists) may proceed. Keep at most one open implementation PR. The frontmatter
  `branch` and `related-pr` fields name the current slice; this table keeps the full history
  (issue frontmatter allows one PR; see draft
  `docs/issues/drafts/2264-support-multiple-related-prs-in-issue-frontmatter/`).
- **Re-slicing.** The maintainer may split, merge, or reorder slices at any time, for example to
  deliver finished rows early when a finding pauses the work. Record each change in the progress
  log.
- **Manual verification and mutation sampling run once, in S6.** S1-S5 change only tests and
  documentation, so the built tracker behaves identically before and after them.

## Commit Points

| Task | Coherent change set | Commit policy |
| ---- | ------------------- | ------------- |
| Spec (S0) | This specification and the EPIC registration with the baseline | One signed `docs(issues)` commit after maintainer approval, merged through the spec-only PR. |
| T1 | Coverage evidence and module inventory | Documentation-only commit after review. |
| T2-T3 | Shared plan guidance, first plan, and approved order | Documentation-only commit before test-producing work. |
| T4 | One source file: its plan, test refactor, unit tests, and recorded coverage | One commit per completed file after its completed-file review. A no-change conclusion is committed as documentation. Never mix two source files in one commit. |
| Slice close | Slice coverage measurement, EPIC table update, reviewer report, records | One documentation commit per slice, right before its PR. |
| T5 | One integration increment | Commit after focused validation and recorded boundary rationale. |
| T6 | Mutation evidence | Commit if it records a material decision, survivor, or follow-up; otherwise include in final evidence. |
| T7 | Reconciliation fixes | Separate documentation commit when it materially changes evidence. |
| T8 | Final verification and completion evidence | Commit after maintainer review, final verification, and acceptance review. |

Use Conventional Commit messages that name the narrow affected area, for example
`test(http-core): cover <module contract>` or `docs(issues): record http core coverage inventory`.
All commits must be GPG signed. Cite commits in issue-local evidence by their unique Conventional
Commit subject, never by branch SHA, until they merge.

When work stops mid-task, record a one-paragraph `handoff.md` in the issue folder: the next action,
the validation state at stop, and any uncommitted-work locations. Remove it when the work resumes.

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
lines are the [baseline](#baseline-coverage) measurement.

| Order | Source module | Rank | Slice | Unit-only lines | Tests | Initial hypothesis | State | Plan |
| ---: | --- | ---: | --- | ---: | ---: | --- | --- | --- |
| 1 | `services/announce.rs` | 1-2 | S2 | 675 / 727 (92.85%) | 8 | Property candidate (peer-IP selection) | PENDING | - |
| 2 | `services/error_mapping.rs` | 2 | S2 | 0 / 15 (0.00%) | 0 | Test candidate | PENDING | - |
| 3 | `services/scrape.rs` | 3 | S3 | 507 / 540 (93.89%) | 8 | Test candidate (error paths) | PENDING | - |
| 4 | `event.rs` | 4 | S4 | 174 / 187 (93.05%) | 7 | Probably small change | PENDING | - |
| 5 | `container.rs` | 5 | S4 | 0 / 73 (0.00%) | 0 | Test candidate | PENDING | - |
| 6 | `statistics/event/handler.rs` | 6 | S5 | 159 / 160 (99.38%) | 5 | Probably no change | PENDING | - |
| 7 | `statistics/event/listener.rs` | 6 | S5 | 133 / 148 (89.86%) | 4 | Lifecycle (#1488) | PENDING | - |
| 8 | `statistics/metrics.rs` | 6 | S5 | 40 / 49 (81.63%) | 0 | Test candidate | PENDING | - |
| 9 | `statistics/repository.rs` | 6 | S5 | 20 / 23 (86.96%) | 0 (2 doc tests) | Test candidate | PENDING | - |
| 10 | `statistics/mod.rs` | 6 | S5 | 8 / 8 (100.00%) | 0 | Probably small change (metric-name contract) | PENDING | - |
| 11 | `lib.rs` | 7 | S6 | 27 / 27 (100.00%) | 0 | Test support only | PENDING | - |
| 12 | `services/mod.rs` | 7 | S6 | No executable entries | 0 | Wiring only | PENDING | - |
| 13 | `statistics/event/mod.rs` | 7 | S6 | No executable entries | 0 | Wiring only | PENDING | - |

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
- [ ] Spec-only PR (S0) merged into `develop` before implementation.
- [ ] Complete module inventory and coverage evidence recorded (T1).
- [ ] Shared plan guidance and the first file test plan created; processing order and slices
      approved (T2-T3).
- [ ] Every ledger row is `DONE` with its own file test plan, merged through its slice.
- [ ] Maintainer review after the final test-producing increment.
- [ ] Automatic verification completed with toolchain-qualified evidence.
- [ ] Manual verification recorded in `manual-verification-evidence.md`.
- [ ] Bounded mutation assessment recorded in `mutation-evidence.md`.
- [ ] Task Reviewer reports for every slice recorded in `agent-review-reports.md`.
- [ ] Acceptance criteria reviewed after implementation.
- [ ] Implementation completion review recorded (`implementation-retrospective.md` or a
      progress-log reason).
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

## Acceptance Criteria

- [ ] AC1: A complete `packages/http-core/src/` module inventory is recorded with unit-only
      coverage, property-test candidacy, selected behavior, and an initial hypothesis for every
      source file, plus the final test-technique assessment.
- [ ] AC2: Every source file has its own file test plan processed through the per-file workflow,
      with its resulting unit-only coverage recorded in `coverage-evidence.md`.
- [ ] AC3: Fresh aggregate/global and unit-only coverage evidence (and integration-only, if a target
      is added) is recorded with clean reproducible commands and toolchain-qualified results.
- [ ] AC4: Selected tests protect package-owned behavior at the narrowest maintainable boundary.
- [ ] AC5: Any package integration test has a reviewed rationale.
- [ ] AC6: No-change and deferral decisions enumerate uncovered lines or behavior groups and name
      the owner or reason.
- [ ] AC7: Test-producing increments record the prose-first AAA review and focused validation in
      their plan before the next file begins.
- [ ] AC8: Bounded mutation assessment is completed without a score target or CI gate.
- [ ] AC9: Reconciliation confirms plans, evidence, the slice table, acceptance rows, and EPIC
      tables agree; the EPIC aggregate and unit-only tables record this package's baseline and
      final measurement.
- [ ] AC10: Every highest-risk and high objective in Coverage Objectives is protected by a unit test
      or has a recorded owner-based reason.
- [ ] AC11: The work is merged through the planned slices (or their recorded re-slicing), each with
      a Task Reviewer report, and only the last PR closes #1349.
- [ ] AC12: Every finding that needs a production change has a hand-off draft, a progress-log entry,
      and a `lessons.md` row naming its draft path or issue, and no test pins its reported
      behavior.
- [ ] AC13: `cargo +nightly fmt --all -- --check` passes.
- [ ] AC14: `linter all` exits with code `0`.
- [ ] AC15: `cargo test -p torrust-tracker-http-core` passes.
- [ ] AC16: Manual verification scenarios are executed and recorded.
- [ ] AC17: Acceptance criteria are re-reviewed after implementation.

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
| M1 | HTTP announce | Start an isolated public tracker and run `cargo run -p torrust-tracker-client --bin tracker_client -- http announce <url> <hash>`. | Valid bencoded announce response. | TODO | `manual-verification-evidence.md` V1 |
| M2 | HTTP scrape | Run `cargo run -p torrust-tracker-client --bin tracker_client -- http scrape <url> <hash>` for the announced hash. | Scrape returns the announced torrent. | TODO | `manual-verification-evidence.md` V2 |
| M3 | Peer IP override policy | With `use_ip_from_query_string = false`, send an announce with `ip=<literal>` using `curl`; repeat with the option `true`, then list the torrent's peers through the REST API. | Disabled: bencoded failure reason naming the disabled override. Enabled: the stored peer uses the supplied IP. | TODO | `manual-verification-evidence.md` V3 |
| M4 | Private mode | With `private = true`, announce and scrape with `curl` without a key, then with a key created through the REST API. | Announce without a key fails with an authentication failure reason; scrape without a key returns zeroed data; both succeed with the key. | TODO | `manual-verification-evidence.md` V4 |
| M5 | HTTP core statistics | After M1 and M2, read the REST API statistics. | The TCP4 announce and scrape counters reflect the requests made. | TODO | `manual-verification-evidence.md` V5 |

### Disposable Verification Scripts

None planned. `tracker_client`, `curl`, and the REST API cover every scenario. If a scenario needs
a script, record its issue-local path, why a maintained test does not replace it, and its retention
owner here before writing it.

### Acceptance Verification

| AC ID | Status | Evidence |
| ----- | ------ | -------- |
| AC1-AC17 | TODO | Filled in during T8. |

## Risks and Trade-offs

- Indirect coverage from `axum-http-server` can hide missing package-local tests; mitigate by using
  unit-only evidence for every decision.
- Service tests need real `tracker-core` collaborators; mitigate by constructing only the needed
  in-memory services rather than reusing production container factories.
- High baseline line coverage can suggest little work remains; mitigate with the per-file "what
  does this module decide?" analysis, which found #1348's two bugs in covered code.
- Several PRs mean more review and rebase overhead, and a slice may land with records that the next
  slice must update; mitigate by keeping each slice's records current at its PR point and
  reconciling in T7.
- Waiting for each slice to merge before starting the next can leave idle time during review;
  accepted by the maintainer in exchange for no stacked-branch rebases or record conflicts. Keep
  slices small so reviews are quick.
- A paused issue can go stale while a finding is fixed; mitigate with `handoff.md` and the resume
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
