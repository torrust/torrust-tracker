---
schema-version: 1
doc-type: issue
issue-type: task
status: in-progress
priority: p2
epic: 1347
github-issue: 1348
spec-path: docs/issues/open/1348-1347-add-tests-udp-core/ISSUE.md
branch: "1348-1347-add-tests-udp-core"
related-pr: null
last-updated-utc: "2026-10-09 07:30"
semantic-links:
  skill-links:
    - create-issue
    - write-unit-test
  related-artifacts:
    - .github/skills/dev/planning/create-issue/SKILL.md
    - .github/skills/dev/testing/write-unit-test/SKILL.md
    - docs/issues/open/1347-overhaul-packages-testing/EPIC.md
    - docs/issues/open/1347-overhaul-packages-testing/subissue-spec-guidelines.md
    - docs/issues/closed/2283-1347-increase-udp-server-package-coverage/ISSUE.md
    - docs/issues/closed/2283-1347-increase-udp-server-package-coverage/coverage-evidence.md
    - packages/udp-core
---

<!-- skill-link: create-issue -->

# Issue #1348 - Add Tests to the UDP Core Package

Parent EPIC: #1347 - Overhaul: Packages Testing

## Goal

Increase the maintainable, package-local test coverage of `torrust-tracker-udp-core` by going
systematically over every Rust source file in the package. For each file: review the existing tests
against the `write-unit-test` skill and refactor them when they are not clean, then analyse coverage
and add unit tests (preferred) or package integration tests (only when a unit test cannot protect
the behavior at an appropriate boundary), and record the resulting per-file coverage before moving
to the next file.

## Background

GitHub issue #1348 was opened when the package was named `bittorrent-udp-tracker-core` under
`packages/udp-tracker-core/`. It is now `torrust-tracker-udp-core` under `packages/udp-core/`. The
coverage table in the GitHub issue body is a historical 2025 measurement of a different package
layout; it is not a baseline for this work.

The package owns the UDP-specific tracker domain logic between the protocol crate and the UDP
server: connection-cookie generation and validation, the instance cipher keys, the connect,
announce, scrape, and banning services, UDP core domain events, peer construction from announce
requests, the UDP core service container, and the UDP core statistics (metrics, repository,
services, and event handler/listener).

A preliminary unit-only measurement on branch base `474acc436` (`cargo-llvm-cov 0.6.16`, stable
Rust 1.98.1, `cargo llvm-cov -p torrust-tracker-udp-core --all-features --lib`) found 43 unit
tests and these package-source totals:

| Lines | Regions | Functions |
| ---: | ---: | ---: |
| 975 / 1,345 (72.49%) | 1,488 / 1,894 (78.56%) | 129 / 181 (71.27%) |

Four package-owned files have no unit-only coverage at all: `services/announce.rs`,
`services/scrape.rs`, `container.rs`, and `peer_builder.rs`. The announce and scrape services are
exercised indirectly by `udp-server` handler tests, but EPIC #1347 does not accept coverage from
another package, an integration test, or an end-to-end test as evidence that a feasible
package-owned unit seam is protected. The package has no `tests/` integration target.

This preliminary measurement guides the draft only. T1 replaces it with clean, reproducible
evidence before any test changes. When T1 records the baseline, the EPIC #1347 Package Coverage
Tracking tables gain a `torrust-tracker-udp-core` row in both the aggregate and unit-only tables.

## Key Package Responsibilities

Ranked from most to least critical. The ranking drives the ledger processing order.

| Rank | Responsibility | Source modules | Why it matters |
| ---: | --- | --- | --- |
| 1 | Connection-ID (cookie) issuing and validation (BEP 15) | `connection_cookie.rs`, `crypto/keys.rs`, `crypto/ephemeral_instance_keys.rs` | Gate for every announce and scrape. A defect either rejects all clients or re-enables source-address spoofing and amplification. |
| 2 | Announce service | `services/announce.rs`, `peer_builder.rs` | Main tracker use case. Applies the validation policy, whitelist authorization, delegation to `tracker-core`, error adaptation, and event publication. The peer address comes from the socket, not the request. |
| 3 | Scrape service | `services/scrape.rs` | Same validation policy and error adaptation for swarm statistics. |
| 4 | Connect service | `services/connect.rs` | Issues the connection ID that later requests must present, and publishes the connect event. |
| 5 | Invalid connection-ID banning | `services/banning.rs` | Abuse protection: counts invalid connection IDs per IP and decides bans; governed by the package ADR. |
| 6 | UDP core domain events | `event.rs` | Contract consumed by statistics and other listeners. |
| 7 | Service composition | `container.rs` | Wires services with configuration: public URL, external IP, ban threshold, event bus. |
| 8 | UDP core statistics | `statistics/event/handler.rs`, `statistics/repository.rs`, `statistics/metrics.rs`, `statistics/services.rs`, `statistics/event/listener.rs`, `statistics/mod.rs` | Observability. Incorrect metrics do not affect tracker answers. Listener lifecycle is #1488-owned. |
| 9 | Crate root and wiring | `lib.rs`, `services/mod.rs`, `crypto/mod.rs`, `statistics/event/mod.rs` | `ConnectionIdValidationPolicy`, static initialization, test support, and module declarations. |

## Coverage Objectives

EPIC #1347 sets qualitative, risk-based objectives rather than a numeric target. For this package:

1. **Highest risk** - security- and abuse-relevant decisions: connection-cookie validation windows
   and fingerprints, cipher keys, connection-ID ban counting, and the `ConnectionIdValidationPolicy`
   bypass in the announce and scrape services. Each package-owned branch must have a direct unit
   test or a recorded owner-based reason.
2. **High** - service adaptation with no unit-only coverage: whitelist rejection and tracker-core
   errors mapped to `UdpAnnounceError` and `UdpScrapeError`, event publication with and without a
   sender, and the announce-request to domain-peer mapping in `peer_builder.rs`.
3. **Medium** - container composition and statistics projections, where the observable contract is
   which services and configuration values are wired.
4. **Low** - declaration-only modules, logging-only branches, and listener lifecycle owned by #1488.

The EPIC objective is met when every high-risk and high package-owned decision is protected by a
unit test or has a recorded reason, not when a percentage is reached.

## Test-Technique Assessment

EPIC #1347 requires every package subissue to assess each test level. T1 records the final
decision in `coverage-evidence.md`; the initial assessment is:

| Technique | Initial assessment |
| --- | --- |
| Unit tests | Selected. Primary objective; in-file `#[cfg(test)]` modules next to the code. |
| Package integration tests | Not selected initially. The package has no network boundary of its own; a `tests/` target is added only if a file plan records why a public-API contract is clearer there. |
| Runnable examples | Not selected initially. Existing documentation tests in `connection_cookie.rs` and `statistics/services.rs` are reviewed with their files. |
| Root `tests/` or `packages/e2e-tools/` | Not selected. Real UDP behavior is owned by `udp-server` and root tests; any coverage relied on there is linked, not duplicated. |
| Mutation testing | Selected as a bounded sample in T6 on a changed high-risk seam. |
| Property-based testing | Try where a seam has a natural property: cookie make/check round trip and validity window, cipher encrypt/decrypt round trip, and ban-threshold counting. If an attempt shows no value over example-based tests (for example, two or three enumerable cases, or no defect a property could catch that the examples miss), record that conclusion in the file plan and do not add the dependency. When kept, `quickcheck` (already a dev-dependency of `udp-protocol`) is added following the `add-rust-dependency` skill. |
| Fuzzing | Not selected. Untrusted byte parsing is owned by `udp-protocol`; this package receives typed requests. |
| Benchmarks | Out of scope. Existing Criterion benches are not tests of correctness. |

## Scope

### In Scope

- Produce a complete per-file module inventory for `packages/udp-core/src/` before adding or
  changing tests. Each row records current unit-only coverage, selected package-owned behavior,
  property-test candidacy, and an initial hypothesis. The hypothesis guides processing order; it
  does not skip a file.
- Process every Rust source file in the package through the per-file workflow below, one file at
  a time, creating its file test plan right before work on that file starts.
- Measure fresh aggregate/global and unit-only package-source coverage using clean, reproducible
  `cargo llvm-cov` runs, and integration-only coverage if a package integration target is added.
  Keep the scopes in separate issue-local evidence tables.
- Add focused, deterministic unit tests for package-owned decisions, starting with the services
  and adapters that have no unit-only coverage.
- Add a package integration test only when a file test plan records why a unit test cannot protect
  the behavior or why the integration boundary is demonstrably clearer. When a behavior is
  impractical to cover in this package, use the narrowest stable boundary: package integration
  first, then root `tests/` or `packages/e2e-tools/` only when the behavior is necessarily composed
  there. Record the chosen boundary and its rationale.
- When a package behavior is covered outside this package, add the exact external test path to this
  specification's `semantic-links.related-artifacts`, following the Semantic Skill Link Convention.
  Do not add broad directory links.
- Assess a bounded mutation-testing sample after the approved test increments are complete.
- Record one concise file test plan per source file under the issue-local `test-refactor-plans/`
  directory: one shared README for guardrails and roughly 40-line per-file plans.
- Collect reusable testing or workflow lessons, including candidate entries for the testing
  refactoring-pattern catalog, in issue-local `lessons.md`. Promoting them into `docs/testing/` or
  `.github/skills/` is a separate small PR after this issue merges, per the subissue guidelines.
- Update the EPIC #1347 Subissues and Package Coverage Tracking tables when this issue starts and
  completes.

### Out of Scope

- Tracker business rules owned by `tracker-core` (announce handling, peer selection, whitelist
  policy, scrape aggregation), protocol encoding owned by `udp-protocol`, and generic event-bus or
  metric behavior owned by `events` and `torrust-metrics`.
- UDP transport, request admission, and server lifecycle owned by `udp-server` (#2149, #2283).
- Shutdown, cancellation, and spawned-task lifecycle redesign owned by #1488.
- Arbitrary coverage-percentage targets or tests added only to move uncovered line totals.
- Benchmark changes under `packages/udp-core/benches/`.
- Cross-cutting process, skill, pattern-catalog, or agent updates. Record candidates locally in
  `lessons.md`.

## Architectural Decisions

- Related ADRs: `docs/adrs/20260527175600_keep_protocol_and_domain_types_decoupled.md`
- Package-local ADR: `packages/udp-core/docs/adrs/20260829204258_use_exact_ip_counters_for_udp_banning.md`
- ADRs to create: None known. Create one if this work identifies a durable package or
  cross-package ownership or design decision.

## Design and Ownership Review

This package owns UDP-specific domain decisions: cookie generation and validation windows, remote
fingerprints, cipher keys, connection-ID ban counting, the connection-ID validation policy, the
mapping from protocol announce requests to domain peers, service-level error adaptation, UDP core
event publication, and UDP core statistics. Collaborator rules remain owned by `tracker-core`,
`udp-protocol`, `events`, `torrust-metrics`, `udp-server`, root application composition, and #1488.

Before adding or changing tests that involve asynchronous I/O, spawned tasks, or event-listener
teardown, the file test plan must define the interface under test, which component owns normal,
failure, cancellation, and drop-path cleanup, the absolute deadline bounding every awaited
operation, and the narrowest maintainable boundary.

## Bug-Fix Process

Not applicable. This is coverage and test-design work, not a known defect fix. If the inventory
discovers broken behavior, stop and either add a bug section to this specification or create a
separate bug issue using `.github/skills/dev/debugging/fix-bug/SKILL.md`.

## Regression Test Strategy

Not applicable as a bug-fix strategy. For each selected coverage increment, choose the smallest
deterministic maintained boundary that protects the package-owned behavior and record it in the
file test plan and coverage evidence.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`, `DEFERRED`.

| ID | Status | Task | Notes / Expected Output |
| --- | ------ | ---- | ----------------------- |
| T1 | DONE | Create coverage evidence and complete module inventory | [coverage-evidence.md](coverage-evidence.md) records clean aggregate/global and unit-only commands and totals. Its inventory lists every `packages/udp-core/src/` file with unit-only coverage, property-test candidacy, selected behavior, and T1 hypothesis. No tests changed. |
| T2 | DONE | Create shared plan guidance and the first file test plan | [test-refactor-plans/README.md](test-refactor-plans/README.md) holds shared guardrails and the plan index; the first plan is [services-announce-tests.md](test-refactor-plans/services-announce-tests.md). No tests changed. |
| T3 | DONE | Review and approve the processing order | Maintainer approves the ledger order; it may be changed at any time. |
| T4 | TODO | Process every source file through the per-file workflow | Every ledger row has a completed plan, recorded coverage result, completed-file review, and signed per-file commit. |
| T5 | TODO | Implement integration-test increments selected by file test plans | Only if a plan selected a 3B increment; otherwise record why none was needed. |
| T6 | TODO | Perform bounded mutation assessment | `mutation-evidence.md` records configuration, timeout, scope, outcomes, and limitations. |
| T7 | TODO | Reconcile evidence and progress state | Ledger, plans, plan index, coverage evidence, acceptance rows, and both EPIC coverage tables agree. Grep completed records for stray `TODO`/`IN_PROGRESS` labels and stale frontmatter. |
| T8 | TODO | Complete verification and acceptance review | Automatic checks, manual verification, acceptance review, and retrospective decision are recorded. |

After the last T4/T5 test-producing increment, stop for maintainer review before T6-T8, any final
verification commit, or opening a pull request. Use the independent Task Reviewer for the final
pre-PR review.

## Commit Points

| Task | Coherent change set | Commit policy |
| ---- | ------------------- | ------------- |
| Spec | This specification and the EPIC registration | One signed `docs(issues)` commit after maintainer approval, merged through a spec-only PR. |
| T1 | Coverage evidence and module inventory | Documentation-only commit after review. |
| T2-T3 | Shared plan guidance, first plan, and approved order | Documentation-only commit before test-producing work. |
| T4 | One source file: its plan, test refactor, unit tests, and recorded coverage | One commit per completed file after its completed-file review. A no-change conclusion is committed as documentation. Never mix two source files in one commit. |
| T5 | One integration increment | Commit after focused validation and recorded boundary rationale. |
| T6 | Mutation evidence | Commit if it records a material decision, survivor, or follow-up; otherwise include in final evidence. |
| T7 | Reconciliation fixes | Separate documentation commit when it materially changes evidence. |
| T8 | Final verification and completion evidence | Commit after maintainer review, final verification, and acceptance review. |

Use Conventional Commit messages that name the narrow affected area, for example
`test(udp-core): cover <module contract>` or `docs(issues): record udp core coverage inventory`.
All commits must be GPG signed. Until the branch merges, cite in-progress commits by unique
Conventional Commit subject, never branch SHA.

When work stops mid-task, record a one-paragraph `handoff.md` in the issue folder: the next action,
the validation state at stop, and any uncommitted-work locations.

## Per-File Workflow

T4 applies this workflow to every Rust source file in `packages/udp-core/src/`, one file at a time,
in ledger order. Only one file may be `IN_PROGRESS`.

### The file test plan

Each file is tracked by one file test plan: `test-refactor-plans/<module>-tests.md`. Create it right
before starting the file, never in advance. The plan is about 40 lines with these sections:

1. **Current state** - unit-only coverage from the latest report, existing tests, T1 hypothesis.
2. **Current tests review** - each existing test checked against `write-unit-test`. Conclusion:
   `refactor` (list the smells) or `clean`.
3. **Coverage analysis** - uncovered lines or regions grouped by behavior, each classified as:
   - **3A unit** - package-owned decision testable at the module's API (preferred);
   - **3B integration** - only observable through a package integration boundary; record why;
   - **collaborator-owned / lifecycle (#1488) / platform** - not testable here; name the owner.
4. **Steps table** - `R1` refactor, `R2` add unit tests, `R3` select integration increment or
   `none`, `R4` record resulting coverage; each `TODO`/`DONE`/`SKIPPED`.
5. **Results** - unit-only coverage before and after, tests added or refactored, review outcome.
6. **Progress log**.

### Steps for one file

| Step | Action | Gate before continuing |
| --- | --- | --- |
| 0 | Set the ledger row to `IN_PROGRESS`; create the plan with sections 1-3. | Maintainer approval of the plan. |
| 1 | R1: refactor current tests if section 2 said `refactor`. Run focused tests, `cargo +nightly fmt --all -- --check`, `git diff --check`. Prose-first AAA review. | Maintainer review of the refactor result; skipped when section 2 said `clean`. |
| 2 | R2: add the approved unit tests. Same validation plus test-smell review. | Self-review; covered by step 5. |
| 3 | R3: record any selected 3B increment in the plan and T5 queue. | None. |
| 4 | R4: re-run unit-only coverage; record before/after in the plan and `coverage-evidence.md`. | None. |
| 5 | Mark the plan and ledger row `DONE`. | Maintainer completed-file review. |
| 6 | Commit the file's plan, tests, and evidence as one signed commit. | Pre-commit gate passes. |

Helpers must hide only incidental mechanics and are justified by a meaningful named action and
abstraction-level alignment, not by caller count. Test bootstraps construct only the collaborators
the test needs; they do not reuse production container factories.

## Source File Ledger

States: `PENDING` (no plan yet), `IN_PROGRESS`, `DONE`, `REMOVED` (the file no longer exists; the
row keeps the history). Critical-path files with no unit-only
coverage come first (announce, its peer mapping, scrape); the rest follow
[Key Package Responsibilities](#key-package-responsibilities), most critical first. Within a
responsibility, the module holding the decision comes before its supporting modules. Unit-only
lines are the T1 measurement; files added after T1 show their first measurement instead.
Issue #2458 (merged 2026-10-08) replaced the global cookie keys with an injected
`CookieCipher`, so the rows after `connection_cookie.rs` were re-planned on 2026-10-09.

| Order | Source module | Rank | Unit-only lines | T1 hypothesis | State | Plan |
| ---: | --- | ---: | ---: | --- | --- | --- |
| 1 | `services/announce.rs` | 2 | 0 / 101 (0.00%) | Test candidate | DONE | [services-announce-tests.md](test-refactor-plans/services-announce-tests.md) |
| 2 | `peer_builder.rs` | 2 | 0 / 14 (0.00%) | Test candidate | DONE | [peer-builder-tests.md](test-refactor-plans/peer-builder-tests.md) |
| 3 | `services/scrape.rs` | 3 | 0 / 77 (0.00%) | Test candidate | DONE | [services-scrape-tests.md](test-refactor-plans/services-scrape-tests.md) |
| 4 | `connection_cookie.rs` | 1 | 130 / 135 (96.30%) | Property candidate | DONE | [connection-cookie-tests.md](test-refactor-plans/connection-cookie-tests.md) |
| 5 | `crypto/cookie_cipher.rs` | 1 | 49 / 49 (100.00%) after #2458 | Not in T1 (added by #2458 with tests) | PENDING | - |
| 6 | `services/connect.rs` | 4 | 147 / 157 (93.63%) | Probably no change | PENDING | - |
| 7 | `services/banning.rs` | 5 | 60 / 66 (90.91%) | Property candidate | PENDING | - |
| 8 | `event.rs` | 6 | 131 / 137 (95.62%) | Probably no change | PENDING | - |
| 9 | `container.rs` | 7 | 0 / 97 (0.00%) | Test candidate (#2458 added tests) | PENDING | - |
| 10 | `statistics/event/handler.rs` | 8 | 199 / 200 (99.50%) | Probably no change | PENDING | - |
| 11 | `statistics/repository.rs` | 8 | 21 / 26 (80.77%) | Test candidate | PENDING | - |
| 12 | `statistics/metrics.rs` | 8 | 56 / 65 (86.15%) | Test candidate | PENDING | - |
| 13 | `statistics/services.rs` | 8 | 32 / 32 (100.00%) | Probably no change | PENDING | - |
| 14 | `statistics/event/listener.rs` | 8 | 124 / 145 (85.52%) | Lifecycle (#1488) | PENDING | - |
| 15 | `statistics/mod.rs` | 8 | 8 / 8 (100.00%) | Probably no change | PENDING | - |
| 16 | `lib.rs` | 9 | 5 / 10 (50.00%) | Probably no change | PENDING | - |
| 17 | `services/mod.rs` | 9 | 22 / 24 (91.67%) | Test support only | PENDING | - |
| 18 | `crypto/mod.rs` | 9 | No executable entries | Wiring only | PENDING | - |
| 19 | `statistics/event/mod.rs` | 9 | No executable entries | Wiring only | PENDING | - |
| - | `crypto/keys.rs` | 1 | 35 / 41 (85.37%) | Test candidate | REMOVED (#2458) | [crypto-keys-tests.md](test-refactor-plans/crypto-keys-tests.md) (superseded) |
| - | `crypto/ephemeral_instance_keys.rs` | 1 | 5 / 10 (50.00%) | Probably no change | REMOVED (#2458) | - |

## Progress Tracking

### Workflow Checkpoints

- [x] Checked for an existing `udp-core` package-testing spec; none was present.
- [x] Reviewed EPIC #1347, the #1347 subissue-spec guidelines, the `create-issue` workflow, and
      the #2283 specification used as the template.
- [x] Local branch `1348-1347-add-tests-udp-core-spec` created from `develop` for the spec-only PR;
      `1348-1347-add-tests-udp-core` is reserved for implementation.
- [x] Specification reviewed and approved by user/maintainer.
- [x] Spec-only PR #2407 merged into `develop` before implementation.
- [x] Complete module inventory and baseline coverage evidence recorded.
- [x] Shared plan guidance and the first file test plan created; processing order approved.
- [x] EPIC #1347 Subissues and coverage tables updated with the T1 baseline.
- [ ] Every ledger row is `DONE` with its own file test plan.
- [ ] Maintainer review after the final test-producing increment.
- [ ] Automatic verification completed with toolchain-qualified evidence.
- [ ] Manual verification recorded in `manual-verification-evidence.md`.
- [ ] Bounded mutation assessment recorded in `mutation-evidence.md`.
- [ ] Acceptance criteria reviewed after implementation.
- [ ] Implementation completion review recorded (`implementation-retrospective.md` or a
      progress-log reason); independent reviewer results in `agent-review-reports.md` when used.
- [ ] Issue closed and spec moved to `docs/issues/closed/`.

### Progress Log

- 2026-10-01 16:06 UTC - GitHub Copilot - Created this specification on branch
  `1348-1347-add-tests-udp-core`, using #2283 as the template. A preliminary unit-only measurement
  guided the ledger hypotheses. No tests or production code changed.
- 2026-10-02 11:03 UTC - User/maintainer and GitHub Copilot - Maintainer review round 1: ordered
  the ledger by package criticality (new Key Package Responsibilities section); property-based
  tests are tried where a seam has a natural property and dropped when they prove no more valuable
  than examples; manual verification adds invalid connection-ID and ban scenarios using a
  disposable low-level client.
- 2026-10-02 11:26 UTC - User/maintainer - Approved the specification. It is merged through a
  spec-only PR before implementation starts on `1348-1347-add-tests-udp-core`.
- 2026-10-02 11:42 UTC - GitHub Copilot - Opened spec-only PR #2407
  (<https://github.com/torrust/torrust-tracker/pull/2407>) with `Related to #1348`.
- 2026-10-02 17:06 UTC - GitHub Copilot - Started implementation on branch
  `1348-1347-add-tests-udp-core` after PR #2407 merged as `6eba68a1f`. Completed T1 without
  changing tests or production code: [coverage-evidence.md](coverage-evidence.md) records clean
  aggregate and unit-only source measurements, the 20-file inventory, property-test candidates,
  ownership boundaries, and prioritized gaps. The package has 43 unit tests and no package
  integration test target at baseline. T2/T3 await maintainer review of this inventory and order.
- 2026-10-02 18:34 UTC - Jose Celano / GitHub Copilot - T1 approved and committed as
  `docs(issues): record udp core coverage baseline and inventory`. Maintainer chose to start with
  `services/announce.rs` (critical and uncovered). T2 created the shared plan guidance and the
  announce plan; T3 proposes moving announce, `peer_builder.rs`, and scrape ahead of the
  cookie/crypto files. Also checked the EPIC-baseline checkpoint missed in T1. Plan and order await
  approval; no Rust tests changed.
- 2026-10-02 18:43 UTC - Jose Celano - Approved the processing order (T3) and the announce plan.
  T4 is paused: the maintainer is fixing a bug that adds tests to this package. Rebase onto
  `develop` after that PR merges, then re-check the announce plan against the new tests before
  starting R2.
- 2026-10-04 09:48 UTC - GitHub Copilot - PR #2423 merged; rebased onto `develop`. #2423 touched
  only doc comments in `udp-core`, so the announce plan was kept unchanged. Added 7 announce
  service unit tests (unit-only file lines 0 / 101 to 338 / 347). Awaiting completed-file review.
- 2026-10-04 10:35 UTC - Jose Celano - Approved the `services/announce.rs` completed-file review;
  ledger row 1 is `DONE`.
- 2026-10-05 12:56 UTC - GitHub Copilot - Rebased onto `develop` (no `udp-core` change). The
  `peer_builder.rs` plan was approved; added 5 unit tests (unit-only file lines 11 / 14 to
  79 / 79). Awaiting completed-file review.
- 2026-10-05 13:19 UTC - Jose Celano - Approved the `peer_builder.rs` completed-file review;
  ledger row 2 is `DONE`.
- 2026-10-05 18:03 UTC - GitHub Copilot - Rebased onto `develop` (no `udp-core` change). The
  `services/scrape.rs` plan was approved; added 6 unit tests (unit-only file lines 0 / 77 to
  264 / 273). Maintainer also approved reopening `services/announce.rs` for one more test (R2h,
  tracker-core failure adaptation) as a separate commit after scrape. Awaiting scrape
  completed-file review.
- 2026-10-05 18:44 UTC - Jose Celano - Approved the `services/scrape.rs` completed-file review;
  ledger row 3 is `DONE`.
- 2026-10-05 18:49 UTC - GitHub Copilot - Reopened `services/announce.rs` (ledger row 1) for the
  approved R2h test. Unit-only file lines 338 / 347 to 390 / 396. Awaiting completed-file review.
- 2026-10-05 21:06 UTC - Jose Celano - Approved R2h; ledger row 1 is `DONE` again.
- 2026-10-06 11:41 UTC - GitHub Copilot - Rebased onto `develop` (no `udp-core` change). The
  `connection_cookie.rs` plan and R1 refactor were approved; added 6 tests including the first
  `quickcheck` property and the `quickcheck` dev-dependencies (unit-only file lines 130 / 135 to
  171 / 171). Created [lessons.md](lessons.md) with four follow-up candidates. Awaiting
  completed-file review.
- 2026-10-06 11:51 UTC - Jose Celano - Approved the `connection_cookie.rs` completed-file review;
  ledger row 4 is `DONE`.
- 2026-10-06 13:38 UTC - Jose Celano / GitHub Copilot - The `crypto/keys.rs` plan (ledger row 5)
  found that the startup `check_seed()` cannot detect a zeroed cookie key in production. The
  maintainer chose to fix the design by injecting one cookie key, as a separate issue implemented
  in parallel by another agent; the draft spec and hand-off were prepared in the local, git-ignored
  `.tmp/inject-udp-cookie-cipher/`. **#1348 is paused** until that work merges. On resume: rebase,
  adapt the committed cookie, announce, and scrape tests to the injected key, and re-plan rows 5-7
  (`crypto/keys.rs`, `crypto/ephemeral_instance_keys.rs`, `services/connect.rs`) and row 10
  (`container.rs`) against the new design.
- 2026-10-07 10:59 UTC - GitHub Copilot - The hand-off became Issue #2458
  ([spec](../../closed/2458-inject-udp-cookie-cipher/ISSUE.md)), implemented by another agent and in review
  as PR #2470. #1348 stays paused until PR #2470 merges.
- 2026-10-09 07:30 UTC - GitHub Copilot - PR #2470 merged; #1348 resumed. Rebased onto `develop`
  (255 commits). The cookie file's conflicts were resolved by keeping the R1 refactor and passing
  `CookieCipher::fixed_for_testing()` explicitly; the announce and scrape fixtures now inject the
  fixed test cipher into the service constructors. 67 package unit tests pass. #2458 deleted
  `crypto/keys.rs` and `crypto/ephemeral_instance_keys.rs` (ledger rows 5-6) and added
  `crypto/cookie_cipher.rs`, with its own tests; the ledger is re-planned next.
- 2026-10-09 07:30 UTC - GitHub Copilot - Ledger re-planned after #2458 (awaiting maintainer
  approval): `crypto/keys.rs` and `crypto/ephemeral_instance_keys.rs` marked `REMOVED`, their
  plan superseded; `crypto/cookie_cipher.rs` added as order 5 (rank 1). Fresh unit-only package
  coverage: 2,026 / 2,097 lines (96.61%); `container.rs` (183 / 183) and `lib.rs` (5 / 5) are now
  fully covered by #2458's changes, but every remaining row still gets its own file plan.
- 2026-10-09 11:19 UTC - Jose Celano - Approved the re-planned ledger.

## Acceptance Criteria

- [ ] AC1: A complete `packages/udp-core/src/` module inventory is recorded with unit-only coverage,
      property-test candidacy, selected behavior, and a T1 hypothesis for every source file, plus
      the final test-technique assessment.
- [ ] AC2: Every source file has its own file test plan processed through the per-file workflow,
      with its resulting unit-only coverage recorded in `coverage-evidence.md`.
- [ ] AC3: Fresh aggregate/global and unit-only coverage evidence (and integration-only, if a
      target is added) is recorded with clean reproducible commands and toolchain-qualified results.
- [ ] AC4: Selected tests protect package-owned behavior at the narrowest maintainable boundary.
- [ ] AC5: Any package integration test has a reviewed rationale.
- [ ] AC6: No-change and deferral decisions enumerate uncovered lines or behavior groups and name
      the owner or reason.
- [ ] AC7: Test-producing increments complete the prose-first AAA review and focused validation
      before the next file begins.
- [ ] AC8: Bounded mutation assessment is completed without a score target or CI gate.
- [ ] AC9: Reconciliation confirms plans, evidence, acceptance rows, and EPIC tables agree; the
      EPIC aggregate and unit-only tables record this package's baseline and final measurement.
- [ ] AC15: Every high-risk and high objective in Coverage Objectives is protected by a unit test
      or has a recorded owner-based reason.
- [ ] AC10: `cargo +nightly fmt --all -- --check` passes.
- [ ] AC11: `linter all` exits with code `0`.
- [ ] AC12: `cargo test -p torrust-tracker-udp-core` passes.
- [ ] AC13: Manual verification scenarios are executed and recorded.
- [ ] AC14: Acceptance criteria are re-reviewed after implementation.

## Verification Plan

### Automatic Checks

- `cargo llvm-cov clean --workspace`
- `cargo llvm-cov -p torrust-tracker-udp-core --all-features --json`
- `cargo llvm-cov clean --workspace`
- `cargo llvm-cov -p torrust-tracker-udp-core --all-features --lib --json`
- `cargo +nightly fmt --all -- --check`
- `cargo test -p torrust-tracker-udp-core`
- `linter all`
- `TORRUST_GIT_HOOKS_LOG_DIR=.tmp ./contrib/dev-tools/git/hooks/pre-commit.sh --format=json`
- Pre-push checks when applicable

If a package integration target is added, also run a clean integration-only report with
`--test <name>`. Every recorded result names the toolchain that produced it, for example
"nightly Rust toolchain" for formatting and "stable Rust 1.98.1" for coverage. Raw JSON reports stay
in git-ignored `.tmp/1348-coverage/`; only concise Markdown evidence is committed.

### Manual Verification Scenarios

Manual verification is human-oriented interaction with the built tracker using an isolated
configuration. Running automated tests never satisfies it.

| ID | Scenario | Human-oriented command/steps | Expected Result | Status | Evidence |
| --- | --- | --- | --- | --- | --- |
| M1 | UDP announce | Start an isolated tracker and run `cargo run -p torrust-tracker-client --bin tracker_client -- udp announce <url> <hash>`. | Valid announce response. | TODO | `manual-verification-evidence.md` |
| M2 | UDP scrape | Run `cargo run -p torrust-tracker-client --bin tracker_client -- udp scrape <url> <hash>` for the announced hash. | Scrape returns the announced torrent. | TODO | `manual-verification-evidence.md` |
| M3 | Invalid connection ID | With `connection_id_validation = "strict"`, use the low-level verification client (below) to send one announce and one scrape carrying an invalid connection ID. | The tracker answers each with a UDP error response naming the cookie failure. | TODO | `manual-verification-evidence.md` |
| M4 | Ban after repeated invalid IDs | With `max_connection_id_errors_per_ip = 10`, use the same client to send invalid connection IDs until the threshold is passed, then send a valid connect. | Pre-threshold requests get error responses; after the threshold the tracker stops answering this IP. | TODO | `manual-verification-evidence.md` |

The `tracker_client` CLI always performs a valid connect first and has no option to supply a
connection ID. The `torrust-tracker-client` library exposes a lower-level client,
`udp::client::UdpTrackerClient::send(Request)`, that sends any typed request, including an announce
or scrape with an arbitrary `ConnectionId`. `packages/test-helpers/src/udp.rs` already builds such
requests (`send_invalid_connection_ids_until_banned`).

### Disposable Verification Scripts

M3 and M4 use a small disposable Rust program built on `UdpTrackerClient`, reusing the request
construction from `packages/test-helpers/src/udp.rs` where practical.

- Path: `docs/issues/open/1348-1347-add-tests-udp-core/manual-verification/invalid-connection-id/`,
  a standalone Cargo package with an empty `[workspace]` table and path dependencies, so it is
  not a workspace member. Build output goes to `.tmp/`.
- Why not a maintained test: the scenarios verify the built tracker binary from a client's point
  of view; the same behaviors are protected by maintained unit and `udp-server` integration tests.
  `tracker_client` option through a separate issue.
  `tracker_client` option through a separate issue.
- If building the client proves impractical, mark M3 and M4 `BLOCKED`, record the attempted
  commands and constraint, and define another human-oriented interaction before final
  verification. The `udp-server` integration tests are supporting evidence only; they do not
  complete manual verification.

### Acceptance Verification

| AC ID | Status | Evidence |
| ----- | ------ | -------- |
| AC1-AC15 | TODO | Filled in during T8. |

## Risks and Trade-offs

- Indirect coverage from `udp-server` can hide missing package-local tests; mitigate by using
  unit-only evidence for every decision.
- Service tests need real `tracker-core` collaborators; mitigate by constructing only the needed
  in-memory services rather than reusing production container factories.
- Mutation testing can grow into a backlog; mitigate with a bounded sample and no score target.
- Adding a property-testing dependency increases maintenance cost; mitigate by requiring a recorded
  justification and preferring example-based tests for two or three enumerable cases.
- Long per-file work can leave records stale; mitigate with T7 reconciliation and handoff notes.

## Implementation Completion Review

After implementation, compare the result with this specification. Create
`implementation-retrospective.md` if the work produces material design discoveries or queue
changes; otherwise add a progress-log entry explaining why none was needed.

## References

- Parent EPIC: https://github.com/torrust/torrust-tracker/issues/1347
- GitHub issue: https://github.com/torrust/torrust-tracker/issues/1348
- Template subissue: #2283
- Package: `packages/udp-core/`
