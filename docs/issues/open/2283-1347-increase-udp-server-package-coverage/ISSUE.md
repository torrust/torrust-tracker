---
schema-version: 1
doc-type: issue
issue-type: task
status: planned
priority: p2
epic: 1347
github-issue: 2283
spec-path: docs/issues/open/2283-1347-increase-udp-server-package-coverage/ISSUE.md
branch: "2283-1347-increase-udp-server-package-coverage"
related-pr: null
last-updated-utc: "2026-09-22 07:10"
semantic-links:
  skill-links:
    - create-issue
    - write-unit-test
  related-artifacts:
    - .github/skills/dev/planning/create-issue/SKILL.md
    - .github/skills/dev/testing/write-unit-test/SKILL.md
    - docs/issues/open/1347-overhaul-packages-testing/EPIC.md
    - docs/issues/open/1347-overhaul-packages-testing/subissue-spec-guidelines.md
    - docs/issues/closed/2149-1347-add-focused-udp-server-package-tests/ISSUE.md
    - docs/issues/closed/2149-1347-add-focused-udp-server-package-tests/coverage-evidence.md
    - docs/issues/closed/2149-1347-add-focused-udp-server-package-tests/implementation-retrospective.md
    - docs/issues/closed/2149-1347-add-focused-udp-server-package-tests/mutation-evidence.md
    - docs/issues/closed/2149-1347-add-focused-udp-server-package-tests/manual-verification-evidence.md
    - docs/issues/open/2283-1347-increase-udp-server-package-coverage/coverage-evidence.md
    - packages/udp-server
---

<!-- skill-link: create-issue -->

# Issue #2283 - Increase UDP Server Package Coverage

Parent EPIC: #1347 - Overhaul: Packages Testing

## Goal

Continue improving the maintainable package-local safety net for
`torrust-tracker-udp-server` after #2149 by refreshing the complete module inventory, identifying
remaining package-owned behavior gaps, and implementing only maintainer-approved focused test
increments.

## Background

Issue #2149 added focused UDP server package tests for request buffering, socket adaptation,
dispatch, event/error classification, container composition, receiver adaptation, selected
statistics and banning event handlers, server-state notifications, and portable processor behavior.
It also recorded separate aggregate/global, unit-only, and integration-only coverage evidence and a
bounded mutation sample. That work raised aggregate package-source coverage to 97.85% lines and
unit-only coverage to 96.18% lines, but it deliberately left several areas as no-change or deferred
decisions where lifecycle, shutdown, collaborator internals, logging-only effects, platform fault
injection, or external package ownership made additional tests inappropriate at that time.

This follow-up is not a replay of #2149 and does not assume that a high percentage means the package
is complete. It starts from a fresh per-file inventory because the package and surrounding shutdown
work may have changed since #2149. The first implementation task must decide, for every
`packages/udp-server/src/` module, whether there is a valuable package-owned test increment, an
explicit no-change decision, or a deferral with an owner.

## Scope

### In Scope

- Produce a complete per-file module inventory for `packages/udp-server/src/` before adding or
  changing tests. Each row must record current unit-only coverage, selected package-owned behavior,
  property-test candidacy, and a terminal decision: test-added, no-change-with-reason, or
  deferred-with-owner.
- Measure fresh aggregate/global, unit-only, and integration-only package-source coverage using
  clean, reproducible `cargo llvm-cov` runs. Keep the three scopes in separate issue-local evidence
  tables and never use aggregate or integration coverage as proof of unit coverage.
- Review #2149's no-change and deferral decisions against the current codebase and related issues,
  especially shutdown/lifecycle work owned by #1488 and its UDP subissues.
- Add focused, deterministic package-local unit tests where the refreshed inventory identifies a
  readable package-owned seam that #2149 did not cover or that became newly testable.
- Add or adjust package integration tests only when a real UDP loopback boundary provides a clearer
  or more stable behavioral contract than a unit test. Record the rationale before implementation.
- Assess a bounded mutation-testing sample after the approved test increments are complete. Use it
  to challenge assertions and record behavior-relevant survivors; do not introduce a mutation score
  target or CI gate.
- Record concise module decision records under an issue-local `test-refactor-plans/` directory:
  one shared README for guardrails and roughly 40-line per-module records for current state,
  selected contracts, ownership boundaries, review outcomes, and evidence.
- Collect any reusable testing or workflow lessons in issue-local `lessons.md`; do not modify
  repository-wide skills, agents, or testing guidance in this implementation branch.

### Out of Scope

- Reopening or rewriting #2149 evidence, plans, commits, or completed acceptance decisions.
- Arbitrary coverage-percentage targets or tests added only to move uncovered line totals.
- UDP shutdown, cancellation, active-request draining/deadlines, task joining, and standalone
  environment lifecycle redesign unless the approved #1488 work has already exposed a stable seam
  that this issue can test without changing ownership.
- Duplicating `udp-core`, `udp-protocol`, `tracker-core`, metrics-crate, events-crate, root
  application-composition, or container E2E responsibilities.
- Raw-socket, privileged, non-portable, sleep-based, or timeout-based absence tests when a positive
  deterministic contract is unavailable.
- Cross-cutting process, skill, pattern-catalog, or agent updates. Record candidates locally in
  `lessons.md` for a separate follow-up if needed.

## Architectural Decisions

- Related ADRs: `docs/adrs/20260527175600_keep_protocol_and_domain_types_decoupled.md`
- Package-local ADR: `packages/udp-server/docs/adrs/20260907152707_keep_oldest_first_udp_request_eviction.md`
- Related shutdown governance: `docs/issues/open/1488-overhaul-tracker-shutdown/ISSUE.md`
- ADRs to create: None known. Create one if this work identifies a durable package or
  cross-package ownership/design decision.

## Design and Ownership Review

This package owns UDP transport adaptation, server lifecycle entry points, packet dispatch,
request admission, package-local statistics and banning event handling, and test fixtures for its
own package boundary. Collaborator business rules remain owned by `udp-core`, `udp-protocol`,
`tracker-core`, `events`, `metrics`, root application composition, and the #1488 shutdown EPIC.

Before adding or changing tests that involve asynchronous I/O, child tasks, listeners, readiness,
or teardown, the module decision record must define:

- the narrow public or package-visible interface under test;
- which component owns normal, failure, cancellation, and drop-path cleanup;
- the absolute deadline that bounds every awaited readiness, receive, child-exit, and teardown
  operation; and
- whether a unit test, package integration test, root integration test, or explicit deferral is the
  narrowest maintainable boundary.

The first passing vertical slice that touches lifecycle, socket readiness, or listener ownership
requires a design-review checkpoint before another lifecycle-related increment begins.

## Bug-Fix Process

Not applicable. This is coverage and test-design work, not a known defect fix. If the inventory
discovers broken behavior, stop and either add a bug section to this specification or create a
separate bug issue using `.github/skills/dev/debugging/fix-bug/SKILL.md`.

## Regression Test Strategy

Not applicable as a bug-fix strategy. For each selected coverage increment, choose the smallest
deterministic maintained boundary that protects the package-owned behavior. Prefer unit tests at
the causal seam; use package integration only when the real UDP boundary is clearer or the only
practical maintained contract. Record the selected boundary in the module decision record and
coverage evidence.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`, `DEFERRED`.

| ID  | Status | Task | Notes / Expected Output |
| --- | ------ | ---- | ----------------------- |
| T1  | DONE | Create refreshed coverage evidence and complete module inventory | [coverage-evidence.md](coverage-evidence.md) records clean aggregate/global, unit-only, and integration-only commands and totals. The inventory lists every `packages/udp-server/src/` file, its unit-only coverage, property-test candidacy, selected behavior, and terminal decision category. No tests were changed. |
| T2  | DONE | Create shared plan guidance and module decision records | [test-refactor-plans/README.md](test-refactor-plans/README.md) contains shared guardrails, validation commands, non-goals, review gates, subsystem order, and one-file-at-a-time progress rules. The [error adapter record](test-refactor-plans/error-tests.md) records current-test review, refactor decisions, ownership boundaries, proposed contracts, status, and evidence. No tests were changed. |
| T3  | DONE | Review and approve prioritized implementation queue | Maintainer approved the one-file `error.rs` error-adapter queue, which completed its R3 review and completed-file gate. |
| T4  | DONE | Refactor and then extend approved package-local unit tests | Completed the approved `error.rs` error-adapter increment: refactored fragmented assertions, added direct announce/scrape conversion contracts, recorded 100% unit-only file coverage, and obtained completed-file review. |
| T5  | TODO | Refactor and then extend approved package integration tests, if any | Work on one integration-test file at a time. Refactor approved current-test issues before adding a real-loopback contract, and select that contract only when T1-T3 justify the integration boundary over a unit test. Record why the integration boundary is clearer or necessary. |
| T6  | TODO | Perform bounded mutation assessment | Sample one changed high-risk seam after test increments are complete. Record configuration, timeout, outcome, limitations, and behavior-relevant survivors in `mutation-evidence.md`. |
| T7  | TODO | Reconcile evidence and progress state | Verify that plan frontmatter, checklists, module records, coverage evidence, acceptance verification, and the EPIC tables agree. Grep for stale `status: proposed`, stray `TODO`/`IN_PROGRESS` labels in completed records, and rebase-unstable commit SHA citations. |
| T8  | TODO | Complete verification and acceptance review | Run final automatic checks, manual verification, acceptance-criteria review, and implementation completion review. Stop for maintainer review after the final test-producing increment before final verification, committing, or opening an implementation PR. |

## Commit Points

| Task | Coherent change set | Commit policy |
| ---- | ------------------- | ------------- |
| T1 | Coverage evidence and complete module inventory | Include in the spec-only or first documentation commit after focused markdown validation and maintainer review. |
| T2 | Shared plan guidance and module decision records | Commit reviewed documentation records separately from code/test changes when practical. |
| T3 | Approved prioritized queue | Record approval and selected queue as documentation-only evidence before test-producing work. |
| T4 | One source file's test refactor followed by its unit-test increment | Complete, validate, and review the current-test refactor before adding coverage. Commit the completed file result after focused validation and recorded test-design review; do not begin or mix another source file. |
| T5 | One integration-test file's refactor followed by its contract increment | Complete, validate, and review the current-test refactor before adding coverage. Commit the completed file result after focused validation and recorded boundary rationale. |
| T6 | Mutation evidence | Commit only if it records a material decision, survivor, or follow-up; otherwise include in final evidence. |
| T7 | Reconciliation fixes | Commit documentation-state alignment separately when it materially changes issue evidence. |
| T8 | Final verification and completion evidence | Commit after maintainer review, final verification, acceptance review, and retrospective decision are complete. |

Use Conventional Commit messages that name the narrow affected area, for example
`test(udp-server): cover <module contract>` or `docs(issues): record udp server coverage inventory`.
All commits must remain GPG signed. Cite in-progress commits by unique Conventional Commit subject,
not branch SHA, until the branch is merged.

## Test Development Loop

Apply this loop to every test-producing task:

1. Order approved files by cohesive subsystem. Complete all selected files in the current
  subsystem before starting an unrelated subsystem; do not interleave, for example, banning and
  statistics work merely because both have uncovered lines.
2. Complete and approve the target source file's decision record before changing tests. Do not
  begin a second source file while the current record is `IN_PROGRESS`.
3. Review the source file's current tests and implement every approved readability, duplication,
  fixture, assertion, or Arrange-Act-Assert refactor before adding coverage. Run focused
  validation and record the completed refactor decision.
4. Add the smallest approved behavior-focused test increment for that same source file.
5. Review the refactored and added tests before beginning the next source file. Write temporary
  prose for Arrange, Act, and Assert; refactor until the code expresses that prose; remove
  redundant prose; and record the result in the module decision record.
6. Confirm the one causal initial-state difference is visible, the production Act remains visible,
   and expected results are independently specified rather than derived through production code
   under test.
7. Run focused tests and correct failures.
8. Record test-level coverage or explicit no-change evidence, update the file status to `DONE`,
  and obtain the completed-file review before beginning another source file.
9. Commit the coherent increment after focused validation and required review.
10. After the final test-producing increment, stop for maintainer review before final verification,
   committing the final evidence, or opening an implementation pull request.

Helpers must hide only incidental mechanics and must be justified by meaningful named actions and
abstraction-level alignment, not by caller count.

## Test Refactor Plans

Use a lightweight issue-local plan layout:

- `test-refactor-plans/README.md` for shared guardrails, selected validation commands, non-goals,
  subsystem order, module approval gates, one-file-at-a-time progress rules, manual-verification
  definition, and reconciliation checklist.
- One concise module decision record per selected source file. Each record should name current
  coverage, current-test refactor review and outcome, selected contracts, rejected alternatives,
  property-test decision, ownership boundaries, `TODO`/`IN_PROGRESS`/`DONE` status, review status,
  and focused validation evidence.

Do not repeat the full issue workflow in each module record. The complete inventory in
`coverage-evidence.md` is the authoritative checklist for whether every source file reached a
terminal decision.

### Source File Refactor-Plan Ledger

This ledger is the execution-control checklist for every Rust source file in the package. The
inventory in `coverage-evidence.md` remains the authority for coverage and ownership rationale.
Before a file receives test changes, change its ledger state to `TODO`, create its decision record,
and obtain T3 approval. A selected file is complete only when its record is `DONE`, focused
validation and completed-file review are recorded, and this ledger says `DONE`. A terminal
no-change row needs no decision record; a deferred row remains visible until its named owner
resolves the boundary. T7 must reconcile this ledger with the inventory and every decision record.

| Source module | T1 decision | Refactor-plan state | Verification state |
| --- | --- | --- | --- |
| `banning/event/handler.rs` | No change | N/A | T1 terminal decision recorded |
| `banning/event/listener.rs` | Deferred (#1488) | DEFERRED | T1 owner recorded |
| `banning/event/mod.rs` | No change | N/A | T1 terminal decision recorded |
| `banning/mod.rs` | No change | N/A | T1 terminal decision recorded |
| `container.rs` | No change | N/A | T1 terminal decision recorded |
| `error.rs` | Test selected | DONE ([record](test-refactor-plans/error-tests.md)) | R1-R3, focused validation, and completed-file review recorded |
| `event.rs` | No change | N/A | T1 terminal decision recorded |
| `handlers/announce.rs` | No change | N/A | T1 terminal decision recorded |
| `handlers/connect.rs` | No change | N/A | T1 terminal decision recorded |
| `handlers/error.rs` | No change | N/A | T1 terminal decision recorded |
| `handlers/mod.rs` | No change | N/A | T1 terminal decision recorded |
| `handlers/scrape.rs` | No change | N/A | T1 terminal decision recorded |
| `lib.rs` | No change | N/A | T1 terminal decision recorded |
| `server/bound_socket.rs` | Deferred (platform boundary) | DEFERRED | T1 owner recorded |
| `server/launcher.rs` | Deferred (#1488) | DEFERRED | T1 owner recorded |
| `server/mod.rs` | No change | N/A | T1 terminal decision recorded |
| `server/processor.rs` | No change | N/A | T1 terminal decision recorded |
| `server/receiver.rs` | No change | N/A | T1 terminal decision recorded |
| `server/request_buffer.rs` | Deferred (#1488 SI-15) | DEFERRED | T1 owner recorded |
| `server/spawner.rs` | No change | N/A | T1 terminal decision recorded |
| `server/states.rs` | Deferred (#1488) | DEFERRED | T1 owner recorded |
| `statistics/event/handler/error.rs` | No change | N/A | T1 terminal decision recorded |
| `statistics/event/handler/mod.rs` | No change | N/A | T1 terminal decision recorded |
| `statistics/event/handler/request_aborted.rs` | No change | N/A | T1 terminal decision recorded |
| `statistics/event/handler/request_accepted.rs` | No change | N/A | T1 terminal decision recorded |
| `statistics/event/handler/request_banned.rs` | No change | N/A | T1 terminal decision recorded |
| `statistics/event/handler/request_discarded.rs` | No change | N/A | T1 terminal decision recorded |
| `statistics/event/handler/request_received.rs` | No change | N/A | T1 terminal decision recorded |
| `statistics/event/handler/response_sent.rs` | No change | N/A | T1 terminal decision recorded |
| `statistics/event/listener.rs` | Deferred (#1488) | DEFERRED | T1 owner recorded |
| `statistics/event/mod.rs` | No change | N/A | T1 terminal decision recorded |
| `statistics/metrics.rs` | No change | N/A | T1 terminal decision recorded |
| `statistics/mod.rs` | No change | N/A | T1 terminal decision recorded |
| `statistics/repository.rs` | No change | N/A | T1 terminal decision recorded |
| `statistics/services.rs` | No change | N/A | T1 terminal decision recorded |
| `testing/environment.rs` | Deferred (#1488 SI-14/SI-17) | DEFERRED | T1 owner recorded |
| `testing/mod.rs` | No change | N/A | T1 terminal decision recorded |

## Progress Tracking

### Workflow Checkpoints

- [x] Checked for an existing `udp-server` package-testing draft; none was present under
      `docs/issues/drafts/`.
- [x] Reviewed EPIC #1347, completed package-testing subissues #2136, #2140, and #2149, the #1347
      subissue-spec guidelines, and the `create-issue` workflow.
- [x] Folder-style draft created and moved to `docs/issues/open/2283-1347-increase-udp-server-package-coverage/ISSUE.md`.
- [x] Draft specification reviewed and approved by user/maintainer.
- [x] GitHub issue #2283 created and linked as a subissue of #1347.
- [x] Draft moved to `docs/issues/open/` using the assigned issue number.
- [x] Spec-only PR #2286 merged into `develop` before implementation.
- [x] Complete module inventory and baseline coverage evidence recorded.
- [x] Shared plan guidance and proposed error-adapter decision record created; no tests changed.
- [x] Module decision record reviewed and approved for the active `error.rs` file before test-producing work.
- [ ] Implementation completed.
- [ ] Automatic verification completed with toolchain-qualified evidence.
- [ ] Manual verification scenarios executed and recorded in issue-local `manual-verification-evidence.md`.
- [ ] Bounded mutation assessment recorded.
- [ ] Acceptance criteria reviewed after implementation and updated with evidence.
- [ ] Evidence-based implementation completion review recorded.
- [ ] Reviewer validated acceptance criteria and updated checkboxes.
- [x] Committer verified spec progress is up to date before commit.
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`.

### Progress Log

- 2026-09-21 00:00 UTC - GitHub Copilot - Created this draft after reviewing EPIC #1347, the
  completed #2136, #2140, and #2149 package-testing subissues, the #1347 subissue specification
  guidelines, and the issue-creation workflow. No existing UDP-server follow-up draft was found in
  `docs/issues/drafts/`. This draft intentionally stops before GitHub issue creation because the
  repository workflow requires maintainer review and approval first.
- 2026-09-21 18:51 UTC - User/maintainer and GitHub Copilot - Approved the specification, created
  GitHub issue #2283, linked it as a subissue of #1347, and moved this specification to its
  numbered open-issue folder. The next workflow step is a spec-only PR before implementation.
- 2026-09-22 06:41 UTC - GitHub Copilot - Completed T1 without changing package code or tests.
  Clean aggregate/global, unit-only, and integration-only `cargo llvm-cov` reports at `cfb93157`
  are recorded in [coverage-evidence.md](coverage-evidence.md). The complete source inventory
  selects only `error.rs` for T2/T3 review; all other modules have an explicit no-change or
  owner-based deferral decision.
- 2026-09-22 06:59 UTC - User/maintainer - Approved the T1 queue and required subsystem-complete
  sequencing: complete related selected files before beginning an unrelated subsystem while
  retaining one-file-at-a-time status and review gates.
- 2026-09-22 07:00 UTC - GitHub Copilot - Completed T2 without changing package code or tests.
  Added shared plan guidance and the proposed `error.rs` service-error adapter decision record.
  T3 maintainer approval is required before R1 begins.
- 2026-09-22 07:10 UTC - User/maintainer - Approved T3 for the one-file `error.rs` queue.
- 2026-09-22 07:10 UTC - GitHub Copilot - Completed `error.rs` R1 with a reviewed no-change
  decision: the current parse-error tests already expose their causal state, direct conversion Act,
  and independently specified expected fields or typed variant. R2 remains pending approval.
- 2026-09-22 07:10 UTC - User/maintainer - Approved R2 for `error.rs`.
- 2026-09-22 07:10 UTC - GitHub Copilot - Completed `error.rs` R2 by adding direct typed
  conversion tests for announce and scrape service errors. The prose-first review confirmed each
  test exposes a fixed source error, direct `Error::from` Act, and independently expected
  UDP-server error variant. Focused test, nightly formatting, and whitespace checks passed. R3 and
  the completed-file review gate remain before beginning another source file.
- 2026-09-22 07:10 UTC - User/maintainer - Approved the completed-file review for `error.rs`.
- 2026-09-22 07:10 UTC - GitHub Copilot - Completed T4 for the approved one-file error-adapter
  queue. The decision record is `DONE`; no additional package source file has started.
- 2026-09-22 07:10 UTC - GitHub Copilot - Reconciled the completed `error.rs` queue state before
  commit: the shared plan, file decision record, issue task table, and workflow checkpoint agree.
- 2026-09-22 09:23 UTC - User/maintainer - Required an explicit source-file refactor-plan ledger
  in this specification so unstarted, active, completed, no-change, and deferred files cannot be
  confused during later issue stages.
- 2026-09-22 09:23 UTC - GitHub Copilot - Added the 37-file ledger. It records `error.rs` as the
  only completed decision record; all other rows remain explicitly terminal or deferred, with no
  pending refactor plan until a future T3-approved selection changes its row to `TODO`.

## Acceptance Criteria

- [ ] A complete `packages/udp-server/src/` module inventory is recorded with unit-only coverage,
      property-test candidacy, selected behavior, and terminal decisions for every source file.
- [ ] Fresh aggregate/global, unit-only, and integration-only coverage evidence is recorded with
      clean reproducible commands and toolchain-qualified results.
- [ ] Selected tests protect package-owned UDP server behavior at the narrowest maintainable
      boundary, prioritizing deterministic unit tests over higher-level coverage when feasible.
- [ ] Any package integration tests added by this issue have an explicit reviewed rationale for why
      the real UDP boundary is clearer or necessary.
- [ ] No-change and deferral decisions enumerate the relevant uncovered lines or behavior groups
      and identify the owner or reason.
- [ ] Test-producing increments complete the prose-first Arrange-Act-Assert review and focused
      validation before the next module begins.
- [ ] Each selected source file completes its approved current-test refactor, focused validation,
  and completed-file review before new coverage is added or another file begins.
- [ ] Bounded mutation assessment is completed without introducing a mutation score target or CI gate.
- [ ] Reconciliation confirms module records, coverage evidence, acceptance verification, and EPIC
      tracking are aligned before final verification.
- [ ] `cargo +nightly fmt --all -- --check` passes.
- [ ] `linter all` exits with code `0`.
- [ ] Relevant package tests pass.
- [ ] Manual verification scenarios are executed and documented in issue-local
      `manual-verification-evidence.md`.
- [ ] Acceptance criteria are re-reviewed after implementation and reflect actual behavior.
- [ ] Documentation is updated when behavior or workflow changes.

## Verification Plan

Define verification before implementation starts and execute it before closing the issue.

### Automatic Checks

- `cargo llvm-cov clean --workspace`
- `cargo llvm-cov -p torrust-tracker-udp-server --all-features --json`
- `cargo llvm-cov clean --workspace`
- `cargo llvm-cov -p torrust-tracker-udp-server --all-features --lib --json`
- `cargo llvm-cov clean --workspace`
- `cargo llvm-cov -p torrust-tracker-udp-server --all-features --test integration --json`
- `cargo +nightly fmt --all -- --check`
- `cargo test -p torrust-tracker-udp-server`
- `cargo test -p torrust-tracker-udp-server --test integration`
- `linter all`
- `TORRUST_GIT_HOOKS_LOG_DIR=.tmp ./contrib/dev-tools/git/hooks/pre-commit.sh --format=json`
- Pre-push checks when applicable

Every recorded command result must identify the Rust toolchain or external runtime that produced
it when that can affect behavior.

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID  | Scenario | Human-oriented command/steps | Expected Result | Status | Evidence |
| --- | -------- | ---------------------------- | --------------- | ------ | -------- |
| M1 | UDP tracker announce contract | Start or target an isolated tracker instance and announce with `cargo run -p torrust-tracker-client --bin tracker_client -- udp announce <url> <hash>`. | The UDP tracker accepts the announce and returns a valid tracker response for the selected scenario. | TODO | `manual-verification-evidence.md` section V1 |
| M2 | UDP tracker scrape or failure-path contract | Exercise the selected behavior affected by this issue through the built tracker and `tracker_client`, or record why no user-facing UDP behavior changed. | Observable UDP behavior remains consistent with the selected contract and no regression is observed. | TODO | `manual-verification-evidence.md` section V2 |

Manual verification means real human-oriented artifact interaction. Running automated tests alone
does not satisfy it.

### Disposable Verification Scripts

No disposable verification script is currently planned. If one becomes necessary, record its
issue-local path, the reason a maintained Rust test is unsuitable, its removal or retention owner,
and why any non-Rust implementation is justified.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| ----- | ---------------------- | -------- |
| AC1 | TODO | Complete inventory in `coverage-evidence.md` |
| AC2 | TODO | Coverage evidence tables and command output summaries |
| AC3 | TODO | Module decision records and test output |
| AC4 | TODO | Integration-boundary rationale in module decision records |
| AC5 | TODO | No-change/deferral rows in `coverage-evidence.md` |
| AC6 | TODO | Module decision records and focused validation evidence |
| AC7 | TODO | `mutation-evidence.md` |
| AC8 | TODO | Reconciliation progress-log entry and final diff review |
| AC9 | TODO | Nightly rustfmt output |
| AC10 | TODO | `linter all` output |
| AC11 | TODO | Package test output |
| AC12 | TODO | `manual-verification-evidence.md` |
| AC13 | TODO | Post-implementation acceptance review |
| AC14 | TODO | Documentation diff or no-change rationale |

## Risks and Trade-offs

- High prior coverage can hide untested behavior or encourage percentage-only work. Mitigate this
  with per-file ownership decisions and behavior-first selection.
- Re-testing #2149 decisions can become churn. Mitigate this by starting from #2149 evidence but
  requiring each current module to reach a fresh terminal decision.
- UDP lifecycle tests can accidentally encode unstable shutdown ownership. Mitigate this by
  deferring #1488-owned behavior unless a stable approved seam already exists.
- Integration tests can be slower and more fixture-heavy than unit tests. Mitigate this by requiring
  an explicit boundary rationale before selecting them.
- Mutation testing can grow into a tool-driven backlog. Mitigate this with a bounded sample and no
  score target.
- Documentation records can become stale during long test work. Mitigate this with the explicit
  reconciliation task before final verification.

## Implementation Completion Review

After implementation, compare the result with this specification. Record invalidated assumptions,
material design changes, unexpected validation findings, and reusable lessons.

- Retrospective: `Not yet assessed`
- Create `implementation-retrospective.md` from `docs/templates/IMPLEMENTATION-RETROSPECTIVE.md`
  if this issue produces material design discoveries, changes the approved queue, or creates
  reusable lessons beyond `lessons.md`.
- If no retrospective is needed, add a concise progress-log entry explaining why the work had no
  material discovery.
- Record independent reviewer results in `agent-review-reports.md` when reviewers receive this
  folder-style specification.

## References

- Parent EPIC: https://github.com/torrust/torrust-tracker/issues/1347
- Prior UDP-server package-testing issue: https://github.com/torrust/torrust-tracker/issues/2149
- Prior implementation PR: https://github.com/torrust/torrust-tracker/pull/2174
- Package: `packages/udp-server/`
- EPIC specification: `docs/issues/open/1347-overhaul-packages-testing/EPIC.md`
- Subissue guidelines: `docs/issues/open/1347-overhaul-packages-testing/subissue-spec-guidelines.md`
- Prior coverage evidence: `docs/issues/closed/2149-1347-add-focused-udp-server-package-tests/coverage-evidence.md`
- Prior retrospective: `docs/issues/closed/2149-1347-add-focused-udp-server-package-tests/implementation-retrospective.md`
