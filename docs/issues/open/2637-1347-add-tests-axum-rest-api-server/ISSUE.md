---
schema-version: 1
doc-type: issue
issue-type: task
status: in-progress
priority: p2
epic: 1347
github-issue: 2637
spec-path: docs/issues/open/2637-1347-add-tests-axum-rest-api-server/ISSUE.md
branch: "2637-1347-add-tests-axum-rest-api-server-routes"
related-pr: null
last-updated-utc: "2026-10-28 13:22"
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
    - docs/issues/closed/1349-1347-add-tests-http-core/ISSUE.md
    - docs/issues/closed/1349-1347-add-tests-http-core/lessons.md
    - docs/adrs/20260623200526_adopt_contract-first_architecture_for_rest_api.md
    - docs/security/analysis/reports/2026-09-04_rest-api-token-timing.md
    - docs/issues/open/2637-1347-add-tests-axum-rest-api-server/coverage-evidence.md
    - packages/axum-rest-api-server
---

<!-- skill-link: create-issue -->

# Issue #2637 - Add Tests to the Axum REST API Server Package

Parent EPIC: #1347 - Overhaul: Packages Testing

## Goal

Increase the maintainable, package-local test coverage of `torrust-tracker-axum-rest-api-server`
by going systematically over every Rust source file in the package. For each file: review the
existing tests against the `write-unit-test` skill and refactor them when they are not clean, then
analyse coverage and add unit tests (preferred) or package integration tests (only when a unit
test cannot protect the behavior at an appropriate boundary), and record the resulting per-file
coverage before moving to the next file.

Deliver the work as a sequence of small pull requests, each merged before the next slice starts,
instead of one large pull request at the end.

## Background

This specification replaces the draft `docs/issues/drafts/1347-add-tests-axum-rest-api-server/`
that was written on 2026-09-01, misfiled under #1349 (the `http-core` subissue), and left without
a GitHub issue. That draft predates several changes, so its module inventory, scope and baselines
were not reused. Since it was written:

- **The REST API became contract-first** (ADR
  `docs/adrs/20260623200526_adopt_contract-first_architecture_for_rest_api.md`). Request and
  response DTOs, error schemas and the endpoint contract moved to `rest-api-protocol`; the use cases
  and port traits (`AuthKeyPort`, `WhitelistCommandPort`, `TorrentQueryPort`, `StatsQueryPort`) to
  `rest-api-application`; the tracker-specific adapters and the container wiring to
  `rest-api-runtime-adapter`. This package keeps the Axum transport: routing, the authentication
  middleware, extractors, HTTP responses, and the server lifecycle. The old draft's "response
  serialization and error mapping" scope therefore splits: DTO serialization is owned elsewhere,
  and the status, header and body mapping stays here.
- **Handlers became testable without a socket.** Each handler takes `State<Arc<...ApiService>>`
  (or `State<Option<...>>` for the gated contexts), and each service wraps a `Box<dyn ...Port>`.
  A hand-written port fake and an in-process request reach every handler arm.
- **The server gained a token-aware lifecycle** (`ApiServer::start_with_cancellation`, #2309) next
  to the legacy `Halted` path. #1488 deprecates (SI-18) and removes (SI-19) the legacy path after
  #2449 migrates the test environment.
- **Test-support code is being reorganized.** `src/testing/` is compiled unconditionally today;
  the draft `docs/issues/drafts/2482-gate-axum-rest-api-server-test-environment/ISSUE.md` gates it
  behind a `testing` feature, and #2449 changes its lifecycle. Neither is this issue's work.
- **Public error enums became `#[non_exhaustive]`** (#2454), which adds wildcard arms to the
  handler `match` expressions.
- **Independent subissues set the template.** #1348 (`udp-core`) and #1349 (`http-core`) added the
  [EPIC #1347 subissue guidelines](../1347-overhaul-packages-testing/subissue-spec-guidelines.md),
  the pull-request slices, and the findings procedure used here.

The package exposes the tracker management API below `/api/v1`: statistics, torrents, the
whitelist and authentication keys, plus an unauthenticated `/api/health_check`. Every `/api/v1`
route sits behind a token authentication middleware. The auth-key routes answer `409` unless the
tracker is `private`, and the whitelist routes answer `409` unless it is `listed`. The package
holds about 2.9 thousand source lines in 31 files, 6 unit tests, and 58 integration tests under
`tests/` that start a real server and call it with `rest-api-client`.

## Baseline Coverage

Measured on 2026-10-24 at `develop` commit `abce11ec8`
(`Merge torrust/torrust-tracker#2630: docs(skills): state that a negated closing keyword still closes the issue`),
with `cargo-llvm-cov 0.6.16` and stable Rust 1.99.1, from a clean workspace for each report:

```console
cargo llvm-cov clean --workspace
cargo llvm-cov -p torrust-tracker-axum-rest-api-server --all-features --json --output-path .tmp/axum-rest-api-coverage/aggregate.json
cargo llvm-cov clean --workspace
cargo llvm-cov -p torrust-tracker-axum-rest-api-server --all-features --lib --json --output-path .tmp/axum-rest-api-coverage/unit.json
cargo llvm-cov clean --workspace
cargo llvm-cov -p torrust-tracker-axum-rest-api-server --all-features --test integration --json --output-path .tmp/axum-rest-api-coverage/integration.json
```

Totals include only files under `packages/axum-rest-api-server/src/`, test modules and the
`src/testing/` support code included. The files under `tests/` are not counted.

| Scope | Tests | Lines | Regions | Functions |
| --- | ---: | ---: | ---: | ---: |
| Aggregate (all selected test targets) | 6 unit + 58 integration | 865 / 1,014 (85.31%) | 1,278 / 1,578 (80.99%) | 118 / 149 (79.19%) |
| Unit-only (`--lib`) | 6 unit | 408 / 1,014 (40.24%) | 673 / 1,578 (42.65%) | 36 / 149 (24.16%) |
| Integration-only (`--test integration`) | 58 integration | 645 / 867 (74.39%) | 897 / 1,297 (69.16%) | 99 / 135 (73.33%) |

The three scopes differ, so they are recorded separately and are never combined. The unit-only
figure is the primary objective; the gap between 40.24% and 85.31% is the behavior that only the
network-level integration tests protect today. Per-file results, the uncovered-line groups and the
prioritized gaps are in [coverage-evidence.md](coverage-evidence.md).

An unfiltered `cargo llvm-cov` `TOTAL` line also counts the other workspace crates that the tests
execute (15.32% unit-only and 32.09% aggregate over the whole workspace at this commit); it is not
a package figure and is not used here.

The package is small enough that per-file results matter more than totals: `v1/responses.rs`, the
four `*/handlers.rs` with logic, the `*/responses.rs` files and
`v1/context/health_check/handlers.rs` have 0% unit-only coverage, while
`v1/context/stats/responses.rs` has 6 of 45 lines covered even in the aggregate. EPIC #1347 does
not accept coverage from another package, an integration test, or an end-to-end test as evidence
that a feasible package-owned unit seam is protected.

`packages/axum-rest-api-server/` changed on `develop` after this baseline (#2449 and #2653, below), so
the package was re-measured at `0b4b2d0d3` before S1; it can change again (#2482 and #1488 touch it),
so each slice re-measures it at its start and records the baseline difference, as #1349 did.

**S1 baseline (`0b4b2d0d3`)** supersedes the table above for slice work (the table is kept as history):

| Scope | Tests | Lines | Regions | Functions |
| --- | ---: | ---: | ---: | ---: |
| Aggregate (all selected test targets) | 15 unit + 58 integration | 1,091 / 1,240 (87.98%) | 1,618 / 1,924 (84.10%) | 156 / 187 (83.42%) |
| Unit-only (`--lib`) | 15 unit | 792 / 1,240 (63.87%) | 1,231 / 1,924 (63.98%) | 96 / 187 (51.34%) |
| Integration-only (`--test integration`) | 58 integration | 674 / 908 (74.23%) | 941 / 1,352 (69.60%) | 97 / 137 (70.80%) |

The unit-only rise is mostly out-of-scope test support: #2449 added 8 unit tests to
`src/testing/environment.rs` that run the real router, and #2653 added the first middleware test
and rejected an unreadable `Authorization` header. Excluding `src/testing/environment.rs`, unit-only lines
are 491 / 908 (54.07%). [coverage-evidence.md](coverage-evidence.md#re-measurement-before-slice-s1)
has the per-file differences.

## Key Package Responsibilities

Ranked from most to least critical. The ranking drives the ledger processing order.

| Rank | Responsibility | Source modules | Why it matters |
| ---: | --- | --- | --- |
| 1 | Token authentication | `v1/middlewares/auth.rs` | Decides who may call a management API that can add keys and whitelist entries. A wrong precedence, a lenient scheme match, or a comparison that depends on token content weakens it (security record `2026-09-04_rest-api-token-timing.md`). |
| 2 | Route composition and transport layers | `v1/routes.rs`, `routes.rs`, `v1/context/health_check/handlers.rs` | Capability gating: the auth-key and whitelist routes are always registered, and `v1/routes.rs` passes their handlers a service only when `private` (or `listed`) is set and persistence exists, otherwise authenticated requests get `409`; that the health check is outside the authentication layer, the request timeout, and the request-id header. |
| 3 | Shared response contract | `v1/responses.rs` | One status, `content-type` and body for every success and error, including the legacy `Unhandled rejection: ...` body kept for backward compatibility. Clients and `rest-api-client` depend on it. |
| 4 | State-changing contexts | `v1/context/auth_key/*`, `v1/context/whitelist/*` | Map every `AuthKeyError` and `WhitelistError` outcome to the right status and message, reject invalid input, and answer `409` when the capability is off. |
| 5 | Read-only contexts | `v1/context/torrent/*`, `v1/context/stats/*` | Query parsing (pagination, info-hash lists, `format`), the not-known response, and the Prometheus text of `/stats` and `/metrics`. |
| 6 | Server lifecycle | `server.rs` | Start and stop, TLS startup, listener errors, registration-failure cleanup, the token-aware drain, and the health-check job. The legacy `Halted` half is owned by #1488. |
| 7 | Test support | `testing/environment.rs`, `testing/mod.rs` | Out of scope: the EPIC does not test test code (#2482 gates it, #2449 migrates it). |
| 8 | Crate root and wiring | `lib.rs` and the nine `mod.rs` and declaration-only files | Module declarations, the clock aliases, the log target and the endpoint documentation. |

## Coverage Objectives

EPIC #1347 sets qualitative, risk-based objectives rather than a numeric target. For this package:

1. **Highest risk** - authentication: every token source and precedence combination (header only,
   query only, both, neither), the empty token, a header without the `Bearer` scheme prefix, an invalid
   token, a valid token, and the equivalence of the constant-time comparison to plain membership
   for arbitrary tokens. Each package-owned branch has a direct unit test or a recorded owner-based
   reason.
2. **High** - route composition and the response contract: the `private` and `listed` capability
   gating (service present or absent, hence the `409`), the health check outside authentication, the `408` timeout, and the status, header and
   body of every shared response; and the handlers' mapping of every service outcome (including
   each `AuthKeyError` arm, invalid info hashes and the disabled `409`) to a response.
3. **Medium** - query parsing for the read-only contexts and the Prometheus text; server startup
   failures, TLS startup, `check_fn` and the `Launcher` display.
4. **Low** - declaration-only modules, logging-only closures, the legacy `Halted` stop errors
   (owned by #1488) and test support.

The objective is met when every highest-risk and high package-owned decision is protected by a
unit test or has a recorded owner-based reason, not when a percentage is reached.

## Test-Technique Assessment

EPIC #1347 requires every package subissue to assess each test level. T1 recorded the initial
assessment in [coverage-evidence.md](coverage-evidence.md#test-technique-assessment); the summary
is:

| Technique | Initial assessment |
| --- | --- |
| Unit tests | Selected. Primary objective, in-file `#[cfg(test)]` modules. Handlers are called with a port fake, pure functions directly, and the middleware and router through an in-process request (no socket). |
| Package integration tests | Existing (58) and retained; not reviewed, moved or deleted here. A new one is added only when a file plan records why a unit seam cannot protect the behavior; TLS startup is the likely case. |
| Runnable examples | Not applicable; the package has no examples and its documentation blocks are not Rust. |
| Root `tests/` or `packages/e2e-tools/` | Not selected; not evidence for a package-owned unit seam. |
| Mutation testing | Selected as a bounded whole-package run in T6 (few functions), with `RUSTFLAGS="--cap-lints=warn"` and a reason for every unviable mutant. |
| Property-based testing | Assessed per seam: `authenticate` against a membership model, the bearer-header extraction, and weakly `parse_info_hashes`. Kept only when it protects state that examples miss; then `quickcheck` is added following the `add-rust-dependency` skill, as in #1348. Discrete branch policy uses `rstest` tables. |
| Fuzzing | Not selected; untrusted input is parsed by Axum and serde before it reaches package code. |
| Benchmarks | Out of scope. |

## Scope

### In Scope

- Keep the complete per-file module inventory and baseline in
  [coverage-evidence.md](coverage-evidence.md) (produced by T1 with this specification): one row per
  source file with its unit-only, integration-only and aggregate coverage, property-test
  candidacy, selected behavior, and an initial hypothesis. The hypothesis guides processing
  order; it does not skip a file.
- Process every Rust source file in the package through the per-file workflow below, one file at a
  time, creating its file test plan right before work on that file starts.
- Deliver the processed files through the planned [pull-request slices](#pull-request-slices),
  updating the evidence and the EPIC coverage tables in each slice.
- Measure fresh aggregate/global and unit-only package-source coverage, and integration-only
  coverage, using clean reproducible `cargo llvm-cov` runs. Keep the scopes in separate
  issue-local evidence tables.
- Add focused, deterministic unit tests for package-owned decisions, starting with the
  authentication middleware.
- Add a package integration test only when a file test plan records why a unit test cannot protect
  the behavior or why the integration boundary is demonstrably clearer. When a behavior is
  impractical to cover in this package, use the narrowest stable boundary: package integration
  first, then root `tests/` or `packages/e2e-tools/` only when the behavior is necessarily composed
  there. Record the chosen boundary and its rationale.
- Add dev-dependencies that a plan justifies (for example `rstest`, `async-trait` for port fakes,
  `quickcheck`, or the `tower` `util` feature for in-process requests), each following the
  `add-rust-dependency` skill and `cargo machete --with-metadata`.
- When a package behavior is covered outside this package, add the exact external test path to this
  specification's `semantic-links.related-artifacts`, following the Semantic Skill Link Convention.
  Do not add broad directory links.
- Documentation-comment corrections in the files under test, when a plan finds them wrong,
  including the endpoint documentation in the `mod.rs` files.
- Hand off bugs and other production findings through the
  [findings procedure](#findings-during-implementation).
- Assess a bounded mutation-testing sample after the approved test increments are complete.
- Record one concise file test plan per source file that needs one under the issue-local
  `test-refactor-plans/` directory: one shared README for guardrails and roughly 40-line per-file
  plans, each with a Prose-First AAA Review section (#1348 retrospective). A group of
  declaration-only files may share one plan.
- Collect reusable testing or workflow lessons, including candidate entries for the testing
  refactoring-pattern catalog, in issue-local `lessons.md`. Promoting them into `docs/testing/` or
  `.github/skills/` is a separate small PR after this issue closes, per the subissue guidelines.

### Out of Scope

- Production behavior changes in `packages/axum-rest-api-server/src/`. A bug fix or production
  clean-up found here goes to a separate issue through the findings procedure.
- Tracker business rules and persistence owned by `tracker-core`, use-case orchestration and port
  traits owned by `rest-api-application`, adapters and container wiring owned by
  `rest-api-runtime-adapter`, DTOs and their serialization owned by `rest-api-protocol`, the HTTP
  client owned by `rest-api-client`, and the generic server helpers (graceful shutdown, TLS
  configuration, custom acceptors) owned by `axum-server`.
- `src/testing/` and its gating or lifecycle migration (#2482 draft, #2449). The EPIC does not test
  test code, and this issue does not change it.
- Shutdown, cancellation and spawned-task lifecycle redesign, and the removal of the legacy
  `Halted` path, owned by #1488.
- Reviewing, moving or deleting the existing 58 integration tests (a separate integration-test
  review, as #2140 was for `axum-http-server`).
- Arbitrary coverage-percentage targets or tests added only to move uncovered line totals.
- Cross-cutting process, skill, pattern-catalog or agent updates. Record candidates locally in
  `lessons.md`.
- Maintaining coverage details in the GitHub issue body; it points to this specification, which is
  the source of truth.

## Architectural Decisions

- Related ADRs: `docs/adrs/20260623200526_adopt_contract-first_architecture_for_rest_api.md`
  (this package is the transport layer; the port traits are the test seam) and
  `docs/adrs/20260901113500_define_completed_download_metric_retention_names.md` (the completed
  download availability of `/stats`).
- Package-local ADRs: none exist (`packages/axum-rest-api-server/docs/adrs/` is absent).
- ADRs to create: none known. Create one if this work identifies a durable package or
  cross-package ownership or design decision, placed by decision scope as the `create-adr` skill
  describes.

## Design and Ownership Review

This package owns the HTTP-specific decisions of the management API: which token source wins,
which capabilities are enabled for a configuration (the key and whitelist routes are always
registered; the service decides the `409`), which status, header and body each outcome produces, how
query parameters are parsed, and how the server starts, listens and stops. The use-case, port,
adapter and DTO behavior belongs to the other `rest-api-*` packages and `tracker-core`.

Questions the file plans must answer, because the answer may be a finding rather than a test:

- A non-UTF-8 `Authorization` header was read as "no header", so a valid `token` query authenticated
  the request. Handed off as #2653 and fixed (rejected as `unknown token provided`, merged through
  #2656); the first middleware test of #2653 pins it.
- The crate documentation in `lib.rs` spells the section `tsl_config` in three places, but the v3 `HttpApi`
  key is `tls_config` and the type denies unknown fields, so the documented snippet does not parse.
  It is a documentation correction for the `lib.rs` plan (S6), not a production change.
- Authentication failures answer `500` with a legacy body. The module documentation says so; the
  plan decides whether the tests pin it as the documented backward-compatible contract.
- The authentication layer is added before the health-check route, so the health check is
  unauthenticated by layer order. The plan pins this ordering with a test.

Before adding or changing tests that involve asynchronous I/O, spawned tasks, TLS or listener
teardown (`server.rs`), the file test plan must define the interface under test, which component
owns normal, failure, cancellation and drop-path cleanup, the absolute deadline bounding every
awaited operation, and the narrowest maintainable boundary.

## Bug-Fix Process

Not applicable to this issue. It is coverage and test-design work, not a known defect fix. A defect
found during the work is handed off through the
[findings procedure](#findings-during-implementation), and its fix follows
`.github/skills/dev/debugging/fix-bug/SKILL.md` in its own issue.

## Regression Test Strategy

Not applicable as a bug-fix strategy. For each selected coverage increment, choose the smallest
deterministic maintained boundary that protects the package-owned behavior and record it in the file
test plan and coverage evidence. Never write a test that pins behavior a handed-off finding reports
as wrong without recording it: leave it uncovered, with the finding as its owner, until the fix
merges, unless the maintainer decides to pin it and the plan, progress log and `lessons.md` record
that decision and name the test that must change with the fix.

## Findings During Implementation

Writing tests often exposes bugs (#1348 found two, fixed by #2458 and #2505; #1349 found a counter
semantics question) and dead or misleading code. Handle every finding that needs a production
change this way:

1. **Stop the current increment** at a clean point: committed, or with an issue-local
   `interruption-note.md` (see [Commit Points](#commit-points)).
2. **Write the draft specification** for the bug under `docs/issues/drafts/<short-topic>/ISSUE.md`
   from `docs/templates/ISSUE.md`, on its own branch cut from `develop` (never on a slice branch),
   with the `Bug-Fix Process` and `Regression Test Strategy` sections that the `fix-bug` skill
   requires, the observed and expected behavior, and `epic: null` unless the maintainer names a
   parent. Put the context another agent needs that does not belong in the spec in a git-ignored
   `.tmp/<issue>-<short-topic>/HANDOFF.md`: the file and plan that found it, the evidence (a
   failing test kept out of the branch, a hand-written mutation, or a reproduction), the suspected
   root cause, the files a fix will probably touch and their overlap with remaining ledger rows, the
   recommended mode (parallel or pause), the validation commands, and a reminder not to edit this
   issue's records.
3. **Stop and tell the maintainer.** Another session handles the bug. The maintainer decides
   whether this work pauses; do not pause or continue on your own.
4. **Record it durably.** Add a progress-log entry and a `lessons.md` follow-up row that summarize
   the finding and name the draft path. When the draft becomes a GitHub issue, replace the path with
   the issue number.
5. **Recommended modes** the maintainer chooses between:
   - **Parallel** - the fix does not touch a remaining ledger row, or touches only rows already
     merged here, and does not change behavior that a remaining plan will test.
   - **Pause** - the fix will add or change code or tests in the current file or a remaining ledger
     row, or changes behavior a remaining plan will pin. Set the affected ledger rows to `BLOCKED`
     with the finding reference. Finished rows may still be delivered as an early pull request,
     because a pause is a natural [pull-request point](#pull-request-slices).
6. **Resume after a pause** once the fix merges: rebase onto `develop`, re-run the unit-only
   coverage of the affected rows, re-check their approved plans and any committed tests against the
   new code, record the result in the progress log, and set the rows back to `PENDING` or
   `IN_PROGRESS`.

Non-bug findings (dead code, misleading documentation outside the files under test, API
clean-ups) only need a `lessons.md` row unless the maintainer asks for a hand-off.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`, `DEFERRED`.

| ID | Status | Task | Notes / Expected Output |
| --- | ------ | ---- | ----------------------- |
| T1 | DONE | Create coverage evidence and complete module inventory | [coverage-evidence.md](coverage-evidence.md) records the baseline commands and totals from this spec, per-file uncovered groups, property-test candidacy, selected behavior, and an initial hypothesis for every `packages/axum-rest-api-server/src/` file. Delivered with this specification (S0). No tests changed. |
| T2 | DONE | Create shared plan guidance and the first file test plan | `test-refactor-plans/README.md` holds the shared guardrails and the plan index, including the #1348 and #1349 testing lessons that apply here (see [lessons carried](#lessons-carried-from-1348-and-1349)). The first plan is `v1-middlewares-auth-tests.md`. No tests changed. |
| T3 | DONE | Review and approve the processing order and slice boundaries | Maintainer approves the ledger order and the pull-request slices; both may change at any time. |
| T4 | TODO | Process every source file through the per-file workflow | Every ledger row has a completed plan (or a shared declaration-only plan), recorded coverage result, completed-file review, and signed per-file commit, merged through its slice. |
| T5 | TODO | Implement integration-test increments selected by file test plans | None expected; a plan may select one (TLS startup is the likely candidate). |
| T6 | TODO | Perform bounded mutation assessment | `mutation-evidence.md` records the run, the survivors and the reason for each unviable mutant. |
| T7 | TODO | Reconcile evidence and progress state | Ledger, plans, plan index, slice table, coverage evidence, acceptance rows, and both EPIC coverage tables agree. Grep for `status: proposed`, stray `TODO`/`IN_PROGRESS` labels in completed records, and stale checkpoint names. |
| T8 | TODO | Complete verification and acceptance review | Automatic checks, manual verification, the final Task Reviewer review (`agent-review-reports.md`), the acceptance review below, and the retrospective (`implementation-retrospective.md`) are recorded. |

The specification frontmatter `status` stays `planned`, and the EPIC #1347 subissue row stays
`TODO`, until the spec-only PR merges and the first implementation slice starts; both then change
to `in-progress` and `IN_PROGRESS` in that slice's first commit (the sequence #1348 and #1349
followed).

After the last T4/T5 test-producing increment, stop for maintainer review before T6-T8 or the final
pull request.

### Lessons Carried From #1348 and #1349

- Re-check an "untestable" path against exported test doubles and derives before recording it as
  untestable: a fake `...Port` reaches every service outcome, and a type's `PartialEq`/`Debug`
  derives allow whole-value assertions (#1349).
- Prove an ordering decision with an expectation-free double that fails on the first call (for
  example, authentication before the handler runs).
- Assert whole values against literals: statuses, header values, JSON and Prometheus text written
  as literals make a mutant that changes any part fail (#1349).
- Table-test the policy, not one setting: the capability gating over `private` and `listed`, the
  `AuthKeyError` arms, and the `format` values (#1349).
- Test a helper that many tests rely on (a port fake) against the mutant that breaks its documented
  contract.
- Re-measure the baseline and diff the package at the start of every slice, because `develop`
  moves under the work (#1349).
- Mutants must compile under `unused = "deny"`; revert each exactly (#1349).

## Pull-Request Slices

The implementation is merged in slices. Each slice is a pull-request point: when its rows are done,
open a PR, get it reviewed, and merge it. The spec-only PR comes first and already contains the T1
evidence, so the first implementation slice starts with the shared plan guidance.

**Decision (2026-10-24, maintainer Jose Celano):** the slices below are approved as proposed. The
ledger order inside each slice is still confirmed with the first plan (T3).

| Slice | Content | Ledger rows | Branch | PR | Status |
| --- | --- | --- | --- | --- | --- |
| S0 | This specification, the T1 coverage evidence and inventory, and the EPIC registration with the baseline | - | `2637-1347-add-tests-axum-rest-api-server-spec` | #2638 | DONE |
| S1 | T2-T3, then authentication and the shared response contract (ranks 1 and 3) | 1, 5 | `2637-1347-add-tests-axum-rest-api-server-auth` | #2667 | DONE |
| S2 | Route composition, transport layers and the health check (rank 2) | 2-4 | `2637-1347-add-tests-axum-rest-api-server-routes` | TBD | IN_PROGRESS |
| S3 | State-changing contexts: auth keys and whitelist (rank 4) | 6-11 | `2637-1347-add-tests-axum-rest-api-server-commands` | TBD | PENDING |
| S4 | Read-only contexts: torrents and statistics (rank 5) | 12-17 | `2637-1347-add-tests-axum-rest-api-server-queries` | TBD | PENDING |
| S5 | Server lifecycle (rank 6) | 18 | `2637-1347-add-tests-axum-rest-api-server-lifecycle` | TBD | PENDING |
| S6 | Crate root, wiring and test-support decisions (ranks 7-8), then T5-T8 | 19-31 | `2637-1347-add-tests-axum-rest-api-server-final` | TBD | PENDING |

Rules:

- **Pull-request point.** A slice is ready for a PR when its ledger rows are `DONE` with one signed
  commit each, the maintainer has approved the last completed-file review, the slice's evidence and
  the EPIC records are updated, and the independent Task Reviewer reviewed the slice's rows
  (recorded in `agent-review-reports.md`).
- **Each PR updates the records it delivers.** The ledger, plan index, slice table, the slice's
  coverage measurement in `coverage-evidence.md`, the EPIC "Latest" coverage values, and the
  progress log. The next slice's work starts from those merged records.
- **Issue linking.** S0-S5 use `Related to #2637`; only S6 uses `Closes #2637` (see
  the `open-pull-request` skill). Titles follow the repository convention, for example
  `test(axum-rest-api-server): [#2637] cover the token authentication middleware`.
- **Branches.** The base name `2637-1347-add-tests-axum-rest-api-server` stays unused on
  purpose, because with several slices it would suggest a branch that carries the whole
  implementation. Cut each slice branch from the latest `develop` only after the previous slice PR
  has merged. Do not stack slice branches: while a slice PR is in review, only review fixes,
  findings hand-offs, and the next file's plan draft (kept outside git until its branch exists) may
  proceed. Keep at most one open implementation PR. The frontmatter `branch` and `related-pr`
  fields name the current slice; this table keeps the full history.
- **Re-slicing.** The maintainer may split, merge, or reorder slices at any time, for example to
  deliver finished rows early when a finding pauses the work. Record each change in the progress
  log.
- **Manual verification and mutation sampling run once, in S6.** S1-S5 change only tests,
  dev-dependencies and documentation, so the built tracker behaves identically before and after
  them.
- **Check in at every PR boundary.** After opening a slice PR, report its link and wait for the
  review and merge before starting the next slice.

## Commit Points

| Task | Coherent change set | Commit policy |
| ---- | ------------------- | ------------- |
| Spec (S0) | This specification, the T1 coverage evidence, the EPIC registration with the baseline, and the PR review audit | One signed `docs(issues)` commit after maintainer approval, plus a follow-up signed commit that records the spec-only PR number once the PR exists, plus any review-fix commits, plus the review-audit commit (see the "Review round" row below); merged through the spec-only PR. |
| T2-T3 | Shared plan guidance, first plan, and approved order | Documentation-only commit before test-producing work. |
| T4 | One source file: its plan, test refactor, unit tests, and recorded coverage | One commit per completed file after its completed-file review. A no-change conclusion is committed as documentation. Never mix two source files in one commit. |
| Slice close | Slice coverage measurement, EPIC table update, reviewer report, records | One documentation commit per slice, right before its PR. |
| Review round (every PR, including this one) | `docs/pr-reviews/pr-<PR>-review/PR-REVIEW.md`, plus the review-fix commits it records | One signed `docs(pr-reviews)` commit for the audit, separate from the fixes, as `process-pr-review` requires; each independent fix is its own signed commit. |
| T5 | One integration increment | Commit after focused validation and recorded boundary rationale. |
| T6 | Mutation evidence | Commit if it records a material decision, survivor, or follow-up; otherwise include in final evidence. |
| T7 | Reconciliation fixes | Separate documentation commit when it materially changes evidence. |
| T8 | Final verification and completion evidence | Commit after maintainer review, final verification, and acceptance review. |

Use Conventional Commit messages that name the narrow affected area, for example
`test(axum-rest-api-server): cover <module contract>` or
`docs(issues): record axum rest api server coverage inventory`. All commits must be GPG signed.
Cite commits in issue-local evidence by their unique Conventional Commit subject, never by branch
SHA, until they merge.

When work stops mid-task, record a one-paragraph `interruption-note.md` in the issue folder: the
next action, the validation state at stop, and any uncommitted-work locations. Remove it when the
work resumes. It is not the findings hand-off, which is defined in
[Findings During Implementation](#findings-during-implementation).

## Per-File Workflow

T4 applies this workflow to every Rust source file in `packages/axum-rest-api-server/src/`, one file
at a time. The slice order of the [Pull-Request Slices](#pull-request-slices) takes precedence: a slice
starts only after the previous one merged, and the ledger order applies within a slice (so rows 1 and 5
are processed in S1, and rows 2-4 in S2). Only one file may be `IN_PROGRESS`.

### The File Test Plan

Each file is tracked by one file test plan: `test-refactor-plans/<module>-tests.md` (the path
with `/` replaced by `-`, for example `v1-middlewares-auth-tests.md`). Create it right before
starting the file, never in advance. The plan is about 40 lines with these sections:

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

   Every no-change or deferral decision lists its uncovered lines and answers, per group: is this
   genuinely hard to test, owned elsewhere, or merely unselected?
4. **Property-test decision** - for a flagged candidate, the selected property and the dependency
   justification, or the reason examples suffice.
5. **Steps table** - `R1` refactor, `R2` add unit tests, `R3` select integration increment or
   `none`, `R4` record resulting coverage; each `TODO`/`DONE`/`SKIPPED`.
6. **Prose-First AAA Review** - for every added or changed test, the temporary Arrange, Act, and
   Assert prose and how the code was refactored to express it.
7. **Results** - unit-only coverage before and after, tests added or refactored, mutation proofs,
   review outcome.
8. **Progress log**.

### Steps for One File

| Step | Action | Gate before continuing |
| --- | --- | --- |
| 0 | Set the ledger row to `IN_PROGRESS`; create the plan with sections 1-4. | Maintainer approval of the plan. |
| 1 | R1: refactor current tests if section 2 said `refactor`. Run focused tests, `cargo +nightly fmt --all -- --check`, `git diff --check`. Prose-first AAA review. | Self-review by default; the maintainer reviews the result at step 5. A plan that records a specific risk may add an intermediate maintainer stop here. Skipped when section 2 said `clean`. |
| 2 | R2: add the approved unit tests. Same validation plus the test-smell and prose-first AAA review, recorded in section 6. | Self-review; covered by step 5. |
| 3 | R3: record any selected 3B increment in the plan and T5 queue. | None. |
| 4 | R4: re-run unit-only coverage; record before/after in the plan and `coverage-evidence.md`. | None. |
| 5 | Mark the plan and ledger row `DONE`. | Maintainer completed-file review. |
| 6 | Commit the file's plan, tests, and evidence as one signed commit. | Pre-commit gate passes. |
| 7 | If the row closes its slice, follow the [pull-request point](#pull-request-slices) rule. | The slice PR is opened. |

The per-module approval gates are two (approve the plan before changing tests, review the
completed result), as the subissue guidelines require; intra-file increments proceed without
stopping unless the plan flags a specific risk.

Helpers must hide only incidental mechanics and are justified by a meaningful named action and
abstraction-level alignment, not by caller count. Test bootstraps construct only the collaborators
the test needs; they do not reuse `testing::environment` or production container factories where a
port fake or a minimal router suffices.

## Source File Ledger

States: `PENDING` (no plan yet), `IN_PROGRESS`, `BLOCKED` (paused by a finding; the notes name it),
`DONE`, `REMOVED` (the file no longer exists; the row keeps the history). Rows follow
[Key Package Responsibilities](#key-package-responsibilities), most critical first; the slice order
decides which rows are processed first (see the [Per-File Workflow](#per-file-workflow)). Unit-only lines
are the S1 baseline measurement at `0b4b2d0d3` (see [Baseline Coverage](#baseline-coverage)); the
declaration-only rows have no executable entries.

| Order | Source module | Rank | Slice | Unit-only lines | Tests | Initial hypothesis | State | Plan |
| ---: | --- | ---: | --- | ---: | ---: | --- | --- | --- |
| 1 | `v1/middlewares/auth.rs` | 1 | S1 | 71 / 103 (68.93%) | 4 | Property candidate | DONE | [v1-middlewares-auth-tests.md](test-refactor-plans/v1-middlewares-auth-tests.md) (done; 142 / 142 lines after, 27 test cases, 8 mutants killed, property test declined) |
| 2 | `v1/routes.rs` | 2 | S2 | 27 / 45 (60.00%) | 0 | Test candidate | DONE | [v1-routes-tests.md](test-refactor-plans/v1-routes-tests.md) (done; 111 / 111 lines after, 19 test cases, 6 mutants killed, private test_support helper) |
| 3 | `routes.rs` | 2 | S2 | 58 / 67 (86.57%) | 0 | Test candidate | PENDING | - |
| 4 | `v1/context/health_check/handlers.rs` | 2 | S2 | 4 / 4 (100.00%) | 0 | Test candidate (small) | PENDING | - |
| 5 | `v1/responses.rs` | 3 | S1 | 8 / 40 (20.00%) | 0 | Test candidate | DONE | [v1-responses-tests.md](test-refactor-plans/v1-responses-tests.md) (done; 66 / 66 lines after, 8 test cases, 8 mutants killed) |
| 6 | `v1/context/auth_key/handlers.rs` | 4 | S3 | 0 / 56 (0.00%) | 0 | Test candidate | PENDING | - |
| 7 | `v1/context/auth_key/responses.rs` | 4 | S3 | 0 / 26 (0.00%) | 0 | Test candidate | PENDING | - |
| 8 | `v1/context/auth_key/routes.rs` | 4 | S3 | 16 / 16 (100.00%) | 0 | Probably no change | PENDING | - |
| 9 | `v1/context/whitelist/handlers.rs` | 4 | S3 | 0 / 37 (0.00%) | 0 | Test candidate | PENDING | - |
| 10 | `v1/context/whitelist/responses.rs` | 4 | S3 | 0 / 9 (0.00%) | 0 | Test candidate (small) | PENDING | - |
| 11 | `v1/context/whitelist/routes.rs` | 4 | S3 | 14 / 14 (100.00%) | 0 | Probably no change | PENDING | - |
| 12 | `v1/context/torrent/handlers.rs` | 5 | S4 | 0 / 50 (0.00%) | 0 | Test candidate (weak property) | PENDING | - |
| 13 | `v1/context/torrent/responses.rs` | 5 | S4 | 0 / 9 (0.00%) | 0 | Test candidate (small) | PENDING | - |
| 14 | `v1/context/torrent/routes.rs` | 5 | S4 | 9 / 9 (100.00%) | 0 | Probably no change | PENDING | - |
| 15 | `v1/context/stats/handlers.rs` | 5 | S4 | 0 / 20 (0.00%) | 0 | Test candidate | PENDING | - |
| 16 | `v1/context/stats/responses.rs` | 5 | S4 | 0 / 45 (0.00%) | 0 | Test candidate | PENDING | - |
| 17 | `v1/context/stats/routes.rs` | 5 | S4 | 9 / 9 (100.00%) | 0 | Probably no change | PENDING | - |
| 18 | `server.rs` | 6 | S5 | 275 / 349 (78.80%) | 3 | Test candidate (TLS, errors, `check_fn`); legacy path owned by #1488 | PENDING | - |
| 19 | `testing/environment.rs` | 7 | S6 | 301 / 332 (90.66%) | 8 | Out of scope (test support; #2482, #2449) | PENDING | - |
| 20 | `testing/mod.rs` | 7 | S6 | No executable entries | 0 | Out of scope (test support) | PENDING | - |
| 21 | `lib.rs` | 8 | S6 | No executable entries | 0 | Wiring only | PENDING | - |
| 22 | `v1/mod.rs` | 8 | S6 | No executable entries | 0 | Wiring only | PENDING | - |
| 23 | `v1/context/mod.rs` | 8 | S6 | No executable entries | 0 | Wiring only | PENDING | - |
| 24 | `v1/middlewares/mod.rs` | 8 | S6 | No executable entries | 0 | Wiring only | PENDING | - |
| 25 | `v1/context/auth_key/mod.rs` | 8 | S6 | No executable entries | 0 | Wiring only | PENDING | - |
| 26 | `v1/context/health_check/mod.rs` | 8 | S6 | No executable entries | 0 | Wiring only | PENDING | - |
| 27 | `v1/context/stats/mod.rs` | 8 | S6 | No executable entries | 0 | Wiring only | PENDING | - |
| 28 | `v1/context/torrent/mod.rs` | 8 | S6 | No executable entries | 0 | Wiring only | PENDING | - |
| 29 | `v1/context/torrent/resources/mod.rs` | 8 | S6 | No executable entries | 0 | Wiring only | PENDING | - |
| 30 | `v1/context/torrent/resources/torrent.rs` | 8 | S6 | No executable entries | 0 | Wiring only | PENDING | - |
| 31 | `v1/context/whitelist/mod.rs` | 8 | S6 | No executable entries | 0 | Wiring only | PENDING | - |

The 31 rows are every `.rs` file under `packages/axum-rest-api-server/src/` (`git ls-files`), each
once. If the slicing changes, the Slice column changes with it.

## Progress Tracking

### Workflow Checkpoints

- [x] Checked for an existing `axum-rest-api-server` package-testing spec; the draft
      `docs/issues/drafts/1347-add-tests-axum-rest-api-server/ISSUE.md` existed, was outdated, and
      was replaced by this specification.
- [x] Reviewed EPIC #1347, the #1347 subissue-spec guidelines, the `create-issue` workflow, and the
      #1348 and #1349 specifications used as templates.
- [x] Reviewed the package against the contract-first split, the test-environment gating and
      migration work, and the #1488 legacy-shutdown removal.
- [x] Clean baseline coverage measured and recorded in `coverage-evidence.md` and in the EPIC tables.
- [x] Complete module inventory recorded (T1).
- [x] Specification and the proposed slicing reviewed and approved by the maintainer (2026-10-24).
- [x] GitHub issue #2637 created and linked as a subissue of #1347.
- [ ] Spec-only PR (S0) merged into `develop` before implementation.
- [ ] Shared plan guidance and the first file test plan created (T2, done); processing order and slices
      approved (T3).
- [ ] Every ledger row is `DONE` with its own file test plan, merged through its slice.
- [ ] Maintainer review after the final test-producing increment.
- [ ] Automatic verification completed with toolchain-qualified evidence.
- [ ] Manual verification recorded in `manual-verification-evidence.md`.
- [ ] Bounded mutation assessment recorded in `mutation-evidence.md`.
- [ ] Task Reviewer reports for every slice recorded in `agent-review-reports.md`.
- [ ] Acceptance criteria reviewed after implementation.
- [ ] Implementation completion review recorded (`implementation-retrospective.md`).
- [ ] Issue closed and spec moved to `docs/issues/closed/`.

### Progress Log

- 2026-10-24 19:59 UTC - GitHub Copilot - Replaced the outdated draft with this specification on
  branch `2637-1347-add-tests-axum-rest-api-server-spec`, using #1348 and #1349 as the
  templates. Measured a clean baseline at `abce11ec8` (6 unit and 58 integration tests; unit-only
  lines 40.24%, aggregate lines 85.31%, integration-only lines 74.39%, all filtered to
  `packages/axum-rest-api-server/src/`) and recorded the 31-file inventory in
  `coverage-evidence.md` (T1). The coordinator's rough figures (15% and 32%) were the unfiltered
  workspace `TOTAL`, not package figures. Proposed seven slices (S0-S6) and the findings
  procedure. No tests or production code changed.
- 2026-10-24 19:57 UTC - Jose Celano - Approved the specification and the seven slices (S0-S6) as
  proposed, with the T1 evidence delivered in the spec-only PR (S0), `src/testing/` out of scope,
  the legacy `Halted` path left to #1488, and dev-dependencies allowed when a plan justifies them.
- 2026-10-24 19:57 UTC - GitHub Copilot - Created GitHub issue #2637 (19:57:14 UTC) and linked it as a subissue of
  #1347; renamed the branch to `2637-1347-add-tests-axum-rest-api-server-spec`, moved the
  specification from `docs/issues/drafts/` to `docs/issues/open/`, and registered the package in
  the EPIC tables.
- 2026-10-24 20:00 UTC - GitHub Copilot - Opened spec-only PR #2638 (<https://github.com/torrust/torrust-tracker/pull/2638>) with `Related to #2637`.
- 2026-10-24 20:24 UTC - GitHub Copilot - Processed the Copilot review of #2638 (F1 to F4, submitted
  20:18 UTC): TLS key, capability gating, slice versus ledger order, and the refactor gate were fixed
  in four commits, rebased onto `develop`, replied to and resolved; the audit is
  `docs/pr-reviews/pr-2638-review/PR-REVIEW.md`.
- 2026-10-24 20:45 UTC - Jose Celano - Merged spec-only PR #2638 into `develop`. Cameron's (da2ce7)
  review, posted at 20:39 UTC with findings F5 to F9, had not been processed when it merged; the
  maintainer chose to fold those fixes into the S1 branch (see the audit record).
- 2026-10-24 20:53 UTC - GitHub Copilot - Started S1 on branch
  `2637-1347-add-tests-axum-rest-api-server-auth` from `develop` at the merge of #2638. The package
  and its `rest-api-*`, `axum-server` and `configuration` collaborators are unchanged since
  `abce11ec8` (only a `Cargo.lock` patch bump), so the baseline stands. Folded da2ce7's late findings
  F5 to F9 of #2638 into this branch in separate commits, as the maintainer approved; the audit
  update follows once the S1 pull request exists. Set this specification to `in-progress`.
- 2026-10-24 20:57 UTC - GitHub Copilot - T2: drafted the shared plan guidance and the first plan
  (`v1/middlewares/auth.rs`). A throwaway probe (reverted) showed that a non-UTF-8 `Authorization`
  header with a valid `token` query authenticates, a duplicate `token` answers `400`, and a lower-case
  `bearer` scheme is rejected.
- 2026-10-25 06:15 UTC - Jose Celano - Reviewed the first plan: approved `rstest` and `tower` (`util`)
  as dev-dependencies, pinning the `400` for a duplicate `token`, and no property test for
  `authenticate`; asked to hand the non-UTF-8 header behavior off as a finding and stop. The plan
  still needs explicit approval.
- 2026-10-25 06:30 UTC - GitHub Copilot - Finding handed off: the non-UTF-8 `Authorization` header
  behavior. Wrote the draft bug specification `docs/issues/drafts/rest-api-auth-ignores-unreadable-authorization-header/`
  on branch `chore/draft-rest-api-unreadable-authorization-header` and a git-ignored hand-off in
  `.tmp/2637-auth-unreadable-header/`; added `lessons.md`. The auth plan leaves the case untested with the
  finding as owner. Stopped for the maintainer to choose between pausing S1 and continuing in parallel; the fix
  edits the same file as ledger row 1.
- 2026-10-26 17:50 UTC - GitHub Copilot - The finding was fixed and merged as #2653 (PR #2656; spec
  archived in `docs/issues/closed/2653-rest-api-auth-ignores-unreadable-authorization-header/`), and #2449
  (the REST API test environment) merged too. Rebased the S1 branch onto `develop` at `0b4b2d0d3` and
  re-measured the package: unit-only lines 40.24% to 63.87% (15 unit tests), aggregate 85.31% to
  87.98%, integration-only 74.23%; the rise is mostly #2449's test-environment unit tests. The auth
  middleware is now 71 / 103 lines. Updated the S1 baseline, the ledger, the evidence, the EPIC tables
  and the auth plan (the unreadable-header case is already tested; `tower` `util` is already a
  dev-dependency). S1 resumes; the plan still needs the maintainer's approval.
- 2026-10-26 18:30 UTC - GitHub Copilot - S1 row 1: `v1/middlewares/auth.rs` is done. Unit-only lines 71 / 103
  (68.93%) to 142 / 142 (100.00%) (the counts include the test module), 27 test cases, eight named mutations
  killed, `rstest` 0.28.0 added as a dev-dependency. The maintainer approved the plan (2026-10-26, after the
  #2653 update) and the completed file (18:26 UTC).
- 2026-10-27 07:16 UTC - GitHub Copilot - S1 close: both rows are `DONE` (`v1/middlewares/auth.rs` 142 / 142 and
  `v1/responses.rs` 66 / 66 unit-only lines). The maintainer approved the second plan (2026-10-26) and the completed
  file (2026-10-27). Rebased onto `develop` (only a `torrust-server-lib` bump touched the package) and measured
  again: unit-only lines 921 / 1,305 (70.57%), aggregate 1,161 / 1,305 (88.97%), integration-only 674 / 908
  (74.23%); 46 unit tests. Updated the EPIC "Latest" values.
- 2026-10-27 08:07 UTC - GitHub Copilot - Recorded S1 pull request #2667 (<https://github.com/torrust/torrust-tracker/pull/2667>) with `Related to #2637`, opened at 07:32 UTC after both independent reviews; recorded its number in the frontmatter and the slice table after Copilot's review (finding F1 of the PR #2667 audit).
- 2026-10-27 14:58 UTC - GitHub Copilot - S1 merged through #2667 (11:47 UTC, with your ACK at 10:23 UTC;
  Copilot's one nit F1 was fixed before the merge). Closed the loop on the late #2638 findings F5 to F9: the
  fixes are on `develop`, I replied to and resolved the five threads (14:55 UTC) and set their audit rows to
  `FIXED`/`RESOLVED` in the first commit of this branch. Started S2 on branch
  `2637-1347-add-tests-axum-rest-api-server-routes` from `develop` at the merge of #2672. The package is
  byte-identical to the S1 head, so the S1 measurement is the S2 baseline (unit-only lines 921 / 1,305,
  70.57%). Row 2 (`v1/routes.rs`) is `IN_PROGRESS`; its plan is proposed and awaits the maintainer.
- 2026-10-28 12:20 UTC - Jose Celano - Approved the row 2 plan (`v1/routes.rs`) with these decisions: the
  crate-private container helper is a `#[cfg(test)] mod test_support` (ADR `20261021190318`, rule C; `src/testing/`
  is for code shared outside the crate); the cases with `private` or `listed` set but no persistence are kept as the
  second line of defense, linked to the first line (`validate_persistence_requirements` in
  `src/bootstrap/persistence.rs`, called by `setup` in `src/bootstrap/app.rs`); the design question (making that
  state unrepresentable) and a stale module comment are recorded in `lessons.md` only.
- 2026-10-28 13:22 UTC - GitHub Copilot - Jose Celano questioned whether `src/testing/` should hold only shared test
  code or all test-only code outside a test module. The repository evidence (history of the `src/environment.rs`
  moves, ADR `20261021190318` rule C, `tracker-core`'s crate-root helpers) is in the routes plan. Decision: the helper
  stays a private `#[cfg(test)] mod test_support` and its file gets an inner `#![cfg(test)]`; the convention, including
  the `test_` file prefix for files that contain only test code, goes to `lessons.md` for a separate ADR
  clarification after this issue.

## Acceptance Criteria

- [ ] AC1: A complete `packages/axum-rest-api-server/src/` module inventory is recorded with
      unit-only coverage, property-test candidacy, selected behavior, and an initial hypothesis for
      every source file, plus the final test-technique assessment.
- [ ] AC2: Every source file has its own file test plan (or a shared declaration-only plan)
      processed through the per-file workflow, with its resulting unit-only coverage recorded in
      `coverage-evidence.md`.
- [ ] AC3: Fresh aggregate/global, unit-only and integration-only coverage evidence is recorded with
      clean reproducible commands and toolchain-qualified results.
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
      a Task Reviewer report, and only the last PR closes #2637.
- [ ] AC12: Every finding that needs a production change has a draft specification, a progress-log
      entry, and a `lessons.md` row naming its draft path or issue, and no test pins its reported
      behavior without a recorded maintainer decision.
- [ ] AC13: `cargo +nightly fmt --all -- --check` passes.
- [ ] AC14: `linter all` exits with code `0`.
- [ ] AC15: `cargo test -p torrust-tracker-axum-rest-api-server` passes.
- [ ] AC16: Manual verification scenarios are executed and recorded.
- [ ] AC17: Acceptance criteria are re-reviewed after implementation.

## Verification Plan

### Automatic Checks

- `cargo llvm-cov clean --workspace`
- `cargo llvm-cov -p torrust-tracker-axum-rest-api-server --all-features --json`
- `cargo llvm-cov clean --workspace`
- `cargo llvm-cov -p torrust-tracker-axum-rest-api-server --all-features --lib --json`
- `cargo llvm-cov clean --workspace`
- `cargo llvm-cov -p torrust-tracker-axum-rest-api-server --all-features --test integration --json`
- `cargo +nightly fmt --all -- --check`
- `cargo test -p torrust-tracker-axum-rest-api-server`
- `cargo machete --with-metadata`
- `linter all`
- `TORRUST_GIT_HOOKS_LOG_DIR=.tmp ./contrib/dev-tools/git/hooks/pre-commit.sh --format=json`
- Pre-push checks before each slice PR.

Every recorded result names the toolchain that produced it, for example "nightly Rust toolchain"
for formatting and "stable Rust 1.99.1" for tests and coverage. Raw JSON reports stay in
git-ignored `.tmp/axum-rest-api-coverage/`; only concise Markdown evidence is committed.

### Manual Verification Scenarios

Manual verification is human-oriented interaction with the built tracker using an isolated
configuration. Running automated tests never satisfies it. It runs once, in S6, against the tracker
built from `develop` with every earlier slice merged, following the `use-rest-api` and
`run-tracker-locally` skills.

| ID | Scenario | Human-oriented command/steps | Expected Result | Status | Evidence |
| --- | --- | --- | --- | --- | --- |
| M1 | Public and protected routes | Start an isolated tracker. `curl` `/api/health_check` without a token, then `/api/v1/stats` without a token, with a wrong token, and with the configured token. | The health check succeeds without a token; the protected route fails without or with a wrong token and succeeds with the token. | TODO | `manual-verification-evidence.md` V1 |
| M2 | Token sources and precedence | `curl` `/api/v1/stats` with a valid `Authorization: Bearer` header and an invalid `token` query, then an invalid header and a valid query, then a header without the `Bearer` scheme prefix. | The header token decides when both are present; a header with another scheme is rejected. | TODO | `manual-verification-evidence.md` V2 |
| M3 | Configuration-gated contexts | With the default (public, unlisted) configuration, `POST` an auth key and a whitelist entry; repeat with `private = true` and with `listed = true`, using `curl`. | `409` with the disabled reason in the default mode; the key and whitelist operations succeed in their modes. | TODO | `manual-verification-evidence.md` V3 |
| M4 | Statistics formats | `curl` `/api/v1/stats` and `/api/v1/metrics` with `format=json` and `format=prometheus`. | JSON and Prometheus text bodies with the documented fields. | TODO | `manual-verification-evidence.md` V4 |
| M5 | TLS and graceful shutdown | Start the tracker with a self-signed certificate in `[http_api.tls_config]`, `curl --insecure https://...`, then stop it with `SIGINT`. | The API answers over HTTPS and the process exits cleanly. | TODO | `manual-verification-evidence.md` V5 |

### Disposable Verification Scripts

None planned. `curl` and the tracker binary cover every scenario. If a scenario needs a script,
record its issue-local path, why a maintained test does not replace it, and its retention owner
here before writing it.

### Acceptance Verification

| AC ID | Status | Evidence |
| ----- | ------ | -------- |
| AC1 | TODO | Inventory and T1 hypotheses in `coverage-evidence.md`; the final test-technique assessment. |
| AC2 | TODO | All ledger rows `DONE`; plans in `test-refactor-plans/README.md`; per-file results in `coverage-evidence.md`. |
| AC3 | TODO | Commands, toolchains and figures in the final measurement section of `coverage-evidence.md`. |
| AC4 | TODO | Test boundaries recorded in the file plans. |
| AC5 | TODO | The T5 row and the final assessment. |
| AC6 | TODO | The uncovered-line tables and wiring-only plan conclusions. |
| AC7 | TODO | Each plan's prose-first AAA review and focused validation; one independent review per slice in `agent-review-reports.md`. |
| AC8 | TODO | `mutation-evidence.md`. |
| AC9 | TODO | The T7 reconciliation and both EPIC tables. |
| AC10 | TODO | Unit tests with named mutation proofs in the plans, or recorded owners. |
| AC11 | TODO | The slice table, merged pull requests and reviewer reports. |
| AC12 | TODO | Draft specifications, progress-log entries and `lessons.md` rows, or a recorded absence of findings. |
| AC13 | TODO | `cargo +nightly fmt --all -- --check` output (nightly Rust toolchain). |
| AC14 | TODO | `linter all` output. |
| AC15 | TODO | `cargo test -p torrust-tracker-axum-rest-api-server` output (stable Rust toolchain). |
| AC16 | TODO | `manual-verification-evidence.md`. |
| AC17 | TODO | This table, after the final independent review. |

## Risks and Trade-offs

- Indirect coverage from the 58 integration tests can hide missing package-local tests; mitigate by
  using unit-only evidence for every decision.
- The `rest-api-*` ports make handlers easy to test, but a hand-written port fake becomes a test
  double that many tests depend on; mitigate by testing the fake against the mutant that breaks
  its contract (#1349 lesson) and by keeping one fake per port.
- Router-level unit tests (for the middleware and capability gating) need a container or a minimal
  router; mitigate by building only what each test needs and not reusing `testing::environment`,
  which #2482 and #2449 will change.
- #2449, #2482 and #1488 change `server.rs`, `testing/` and Cargo features while this work runs;
  mitigate by re-measuring at every slice start, keeping `server.rs` last (S5), and not testing the
  legacy `Halted` path beyond what exists.
- High line coverage in the integration run can suggest little work remains; mitigate with the
  per-file "what does this module decide?" analysis, which found #1348's two bugs in covered code.
- Several PRs mean more review and rebase overhead; mitigate by keeping slices small, not stacking
  branches, and reconciling in T7.
- TLS startup needs a certificate and an unbound port, which can make a unit test slow or flaky;
  mitigate by deciding the boundary in the `server.rs` plan (a unit seam, or one package
  integration test) with an absolute deadline.
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
- GitHub issue: https://github.com/torrust/torrust-tracker/issues/2637
- Template subissues: #1349 (`docs/issues/closed/1349-1347-add-tests-http-core/`) and #1348
  (`docs/issues/closed/1348-1347-add-tests-udp-core/`)
- Package: `packages/axum-rest-api-server/`
- REST contract-first ADR: `docs/adrs/20260623200526_adopt_contract-first_architecture_for_rest_api.md`
- Token-timing security record: `docs/security/analysis/reports/2026-09-04_rest-api-token-timing.md`
- Related open work: #2449 (migrate the REST API test environment), the draft
  `docs/issues/drafts/2482-gate-axum-rest-api-server-test-environment/ISSUE.md`, and #1488
  (legacy shutdown removal)
- Historical test-environment work: `docs/issues/closed/1903-1669-si-23-relocate-axum-rest-api-server-test-environment/ISSUE.md`
