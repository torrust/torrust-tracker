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

Increase the maintainable test coverage of `torrust-tracker-udp-server` by going systematically
over every Rust source file in the package. For each file: review the existing tests against the
`write-unit-test` skill and refactor them when they are not clean, then analyse coverage and add
unit tests (preferred) or integration tests (when only the real UDP boundary is clearer), and
record the resulting per-file coverage before moving to the next file.

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
work may have changed since #2149. The inventory gives every `packages/udp-server/src/` module an
initial hypothesis (test candidate, probably no change, or lifecycle owned by #1488). That hypothesis
is **not** a terminal decision: every file still receives its own file test plan and review in T4.
Only the file test plan can conclude "no test change" for a file.

## Scope

### In Scope

- Produce a complete per-file module inventory for `packages/udp-server/src/` before adding or
  changing tests. Each row must record current unit-only coverage, selected package-owned behavior,
  property-test candidacy, and an initial hypothesis: test candidate, probably no change, or
  lifecycle owned by #1488. The hypothesis guides processing order; it does not skip a file.
- Process every Rust source file in the package through the per-file workflow below, one file at
  a time, creating its file test plan right before work on that file starts.
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
- Record one concise file test plan per source file under the issue-local `test-refactor-plans/`
  directory (name kept for EPIC #1347 consistency): one shared README for guardrails and roughly
  40-line per-file plans for current tests review, coverage analysis, selected contracts, ownership
  boundaries, review outcomes, and resulting coverage.
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
or teardown, the file test plan must define:

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
practical maintained contract. Record the selected boundary in the file test plan and
coverage evidence.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`, `DEFERRED`.

| ID  | Status | Task | Notes / Expected Output |
| --- | ------ | ---- | ----------------------- |
| T1  | DONE | Create refreshed coverage evidence and complete module inventory | [coverage-evidence.md](coverage-evidence.md) records clean aggregate/global, unit-only, and integration-only commands and totals. The inventory lists every `packages/udp-server/src/` file, its unit-only coverage, property-test candidacy, selected behavior, and T1 hypothesis. No tests were changed. |
| T2  | DONE | Create shared plan guidance and the first file test plan | [test-refactor-plans/README.md](test-refactor-plans/README.md) holds shared guardrails and the plan index. The first plan, [error-tests.md](test-refactor-plans/error-tests.md), was created for `error.rs`. No tests were changed. |
| T3  | DONE | Review and approve the processing order | Maintainer approved starting with `error.rs`. The processing order for the remaining files is the order column of the [ledger](#source-file-ledger); the maintainer may reorder it at any time. |
| T4  | IN_PROGRESS | Process every source file through the per-file workflow | One file at a time, in ledger order. For each file: create its file test plan, review and refactor its current tests, analyse coverage, add approved unit tests (or select an integration increment for T5), record the resulting coverage, and obtain completed-file review. Files 1-22 are done; 15 files remain. See [Per-File Workflow](#per-file-workflow). |
| T5  | TODO | Implement integration-test increments selected by file test plans | Only for files whose plan selected a 3B integration increment because the real UDP loopback boundary is clearer or necessary. One integration-test file at a time; record why the integration boundary was chosen. Close with an explicit note if no plan selected one. |
| T6  | TODO | Perform bounded mutation assessment | Sample one changed high-risk seam after test increments are complete. Record configuration, timeout, outcome, limitations, and behavior-relevant survivors in `mutation-evidence.md`. |
| T7  | TODO | Reconcile evidence and progress state | Verify that the ledger (37 rows, all `DONE`), every file test plan, coverage evidence per-file results, acceptance verification, and the EPIC tables agree. Grep for stale `status: proposed`, stray `TODO`/`IN_PROGRESS` labels in completed plans, and rebase-unstable commit SHA citations. |
| T8  | TODO | Complete verification and acceptance review | Run final automatic checks, manual verification, acceptance-criteria review, and implementation completion review. Stop for maintainer review after the final test-producing increment before final verification, committing, or opening an implementation PR. |

## Commit Points

| Task | Coherent change set | Commit policy |
| ---- | ------------------- | ------------- |
| T1 | Coverage evidence and complete module inventory | Include in the spec-only or first documentation commit after focused markdown validation and maintainer review. |
| T2 | Shared plan guidance and the first file test plan | Commit reviewed documentation separately from code/test changes when practical. |
| T3 | Approved processing order | Record approval and order as documentation-only evidence before test-producing work. |
| T4 | One source file: its file test plan, test refactor, unit-test increment, and recorded coverage | One commit per completed file after its completed-file review (see the gates table). A no-change conclusion is committed too, as documentation. Do not mix two source files in one commit. |
| T5 | One integration-test file's refactor followed by its contract increment | Complete, validate, and review the current-test refactor before adding coverage. Commit the completed file result after focused validation and recorded boundary rationale. |
| T6 | Mutation evidence | Commit only if it records a material decision, survivor, or follow-up; otherwise include in final evidence. |
| T7 | Reconciliation fixes | Commit documentation-state alignment separately when it materially changes issue evidence. |
| T8 | Final verification and completion evidence | Commit after maintainer review, final verification, acceptance review, and retrospective decision are complete. |

Use Conventional Commit messages that name the narrow affected area, for example
`test(udp-server): cover <module contract>` or `docs(issues): record udp server coverage inventory`.
All commits must remain GPG signed. Cite in-progress commits by unique Conventional Commit subject,
not branch SHA, until the branch is merged.

## Per-File Workflow

T4 applies this workflow to **every** Rust source file in `packages/udp-server/src/`, one file at a
time, in the order of the [ledger](#source-file-ledger). Only one file may be `IN_PROGRESS`.

### The tracking artifact: the file test plan

Each file is tracked by one **file test plan**: `test-refactor-plans/<module>-tests.md` (for
example `error-tests.md`, `handlers-announce-tests.md`). The directory name is kept for consistency
with EPIC #1347 and earlier subissues. Create the plan **right before** starting the file, so each
plan can apply lessons from the previous ones. Do not create plans in advance. The plan is about
40 lines and has these sections:

1. **Current state** — unit-only coverage of the file from the latest report, existing tests, and
   the T1 hypothesis.
2. **Current tests review** — each existing test checked against the `write-unit-test` skill:
   `it_should_*` naming, visible Arrange-Act-Assert, state-centred Arrange, one semantic assertion
   or one reason to fail, no production-derived expectations, no hidden Act, no fixture coupling.
   Conclusion: `refactor` (list the concrete smells) or `clean`.
3. **Coverage analysis** — the uncovered lines/regions grouped by behavior, each classified as:
   - **3A unit** — package-owned decision testable at the module's own API (preferred);
   - **3B integration** — only observable through the real UDP loopback boundary; record why;
   - **collaborator-owned / lifecycle (#1488) / platform** — not testable here; name the owner.
4. **Steps table** — `R1` refactor current tests, `R2` add unit tests, `R3` select integration
   increment (or `none`), `R4` record resulting coverage. Each with `TODO`/`DONE`/`SKIPPED`.
5. **Results** — unit-only coverage of the file before and after, tests added/refactored, and the
   completed-file review outcome.
6. **Progress log**.

### Steps for one file

| Step | Action | Gate before continuing |
| --- | --- | --- |
| 0 | Set the ledger row to `IN_PROGRESS`. Create the file test plan with sections 1-3 filled in. | **Maintainer approval** of the plan (its review conclusion, coverage classification, and proposed steps). |
| 1 | R1: refactor the current tests if section 2 said `refactor`. Run focused tests, nightly `fmt`, `git diff --check`. Prose-first AAA review. | **Maintainer review** of the refactor result before adding tests. Skip the gate if section 2 said `clean`. |
| 2 | R2: add the approved unit tests (3A). Run focused tests, nightly `fmt`, `git diff --check`. Prose-first AAA review and test-smell review. | Self-review only; the completed-file review below covers it. |
| 3 | R3: if section 3 selected a 3B integration increment, record it in the plan and in the T5 queue. Do not implement it here. | None. |
| 4 | R4: re-run the unit-only coverage report and record the file's before/after coverage in the plan and in `coverage-evidence.md`. | None. |
| 5 | Mark the plan and ledger row `DONE`. | **Maintainer completed-file review.** |
| 6 | **Commit** the file's plan, tests, and evidence updates as one signed Conventional Commit (`test(udp-server): ...` or `docs(issues): ...` for a no-change file). | Pre-commit gate passes. |

After step 6, start step 0 of the next file in ledger order. Before the first lifecycle-related
increment (socket readiness, listeners, teardown) add a design-review checkpoint as described in
[Design and Ownership Review](#design-and-ownership-review).

Helpers must hide only incidental mechanics and must be justified by meaningful named actions and
abstraction-level alignment, not by caller count.

### Approval and commit summary

- **Wait for maintainer approval**: step 0 (plan), step 1 (refactor result, when a refactor was
  needed), step 5 (completed file), and after the last file before T6-T8.
- **Commit**: once per completed file (step 6); separately for T6 mutation evidence if material;
  separately for T7 reconciliation; separately for T8 final evidence.
- **Never**: start a second file while one is `IN_PROGRESS`; add tests before R1 is reviewed; mix
  two files in one commit; conclude "no change" for a file without its own file test plan.

## Source File Ledger

This ledger is the execution-control checklist for every Rust source file in the package. The
inventory in `coverage-evidence.md` holds the coverage numbers and ownership rationale; the T1
hypothesis column below repeats it. States: `PENDING` (no plan yet), `IN_PROGRESS` (plan
created, work ongoing), `DONE` (plan concluded, completed-file review recorded, committed). A file
whose coverage step is lifecycle-owned by #1488 is still processed (its existing tests are still
reviewed); its plan records the deferral in section 3. T7 must reconcile this ledger with the
inventory and every plan.

| Order | Source module | T1 hypothesis | State | Plan |
| ---: | --- | --- | --- | --- |
| 1 | `error.rs` | Test candidate | DONE | [error-tests.md](test-refactor-plans/error-tests.md) |
| 2 | `lib.rs` | Probably no change | DONE | [lib-tests.md](test-refactor-plans/lib-tests.md) |
| 3 | `container.rs` | Probably no change | DONE | [container-tests.md](test-refactor-plans/container-tests.md) |
| 4 | `event.rs` | Probably no change | DONE | [event-tests.md](test-refactor-plans/event-tests.md) |
| 5 | `handlers/mod.rs` | Probably no change | DONE | [handlers-mod-tests.md](test-refactor-plans/handlers-mod-tests.md) |
| 6 | `handlers/connect.rs` | Probably no change | DONE | [handlers-connect-tests.md](test-refactor-plans/handlers-connect-tests.md) |
| 7 | `handlers/announce.rs` | Probably no change | DONE | [handlers-announce-tests.md](test-refactor-plans/handlers-announce-tests.md) |
| 8 | `handlers/scrape.rs` | Probably no change | DONE | [handlers-scrape-tests.md](test-refactor-plans/handlers-scrape-tests.md) |
| 9 | `handlers/error.rs` | Probably no change | DONE | [handlers-error-tests.md](test-refactor-plans/handlers-error-tests.md) |
| 10 | `server/mod.rs` | Probably no change | DONE | [server-mod-tests.md](test-refactor-plans/server-mod-tests.md) |
| 11 | `server/bound_socket.rs` | Platform boundary | DONE | [server-bound-socket-tests.md](test-refactor-plans/server-bound-socket-tests.md) |
| 12 | `server/spawner.rs` | Probably no change | DONE | [server-spawner-tests.md](test-refactor-plans/server-spawner-tests.md) |
| 13 | `server/launcher.rs` | Lifecycle (#1488) | DONE | [server-launcher-tests.md](test-refactor-plans/server-launcher-tests.md) |
| 14 | `server/receiver.rs` | Probably no change | DONE | [server-receiver-tests.md](test-refactor-plans/server-receiver-tests.md) |
| 15 | `server/processor.rs` | Probably no change | DONE | [server-processor-tests.md](test-refactor-plans/server-processor-tests.md) |
| 16 | `server/request_buffer.rs` | Lifecycle (#1488 SI-15) | DONE | [server-request-buffer-tests.md](test-refactor-plans/server-request-buffer-tests.md) |
| 17 | `server/states.rs` | Lifecycle (#1488) | DONE | [server-states-tests.md](test-refactor-plans/server-states-tests.md) |
| 18 | `banning/mod.rs` | Wiring only | DONE | [banning-mod-tests.md](test-refactor-plans/banning-mod-tests.md) |
| 19 | `banning/event/mod.rs` | Wiring only | DONE | [banning-event-mod-tests.md](test-refactor-plans/banning-event-mod-tests.md) |
| 20 | `banning/event/handler.rs` | Probably no change | DONE | [banning-event-handler-tests.md](test-refactor-plans/banning-event-handler-tests.md) |
| 21 | `banning/event/listener.rs` | Lifecycle (#1488) | DONE | [banning-event-listener-tests.md](test-refactor-plans/banning-event-listener-tests.md) |
| 22 | `statistics/mod.rs` | Probably no change | DONE | [statistics-mod-tests.md](test-refactor-plans/statistics-mod-tests.md) |
| 23 | `statistics/metrics.rs` | Probably no change | PENDING | — |
| 24 | `statistics/repository.rs` | Probably no change | PENDING | — |
| 25 | `statistics/services.rs` | Probably no change | PENDING | — |
| 26 | `statistics/event/mod.rs` | Wiring only | PENDING | — |
| 27 | `statistics/event/listener.rs` | Lifecycle (#1488) | PENDING | — |
| 28 | `statistics/event/handler/mod.rs` | Probably no change | PENDING | — |
| 29 | `statistics/event/handler/error.rs` | Probably no change | PENDING | — |
| 30 | `statistics/event/handler/request_received.rs` | Probably no change | PENDING | — |
| 31 | `statistics/event/handler/request_accepted.rs` | Probably no change | PENDING | — |
| 32 | `statistics/event/handler/request_discarded.rs` | Probably no change | PENDING | — |
| 33 | `statistics/event/handler/request_banned.rs` | Probably no change | PENDING | — |
| 34 | `statistics/event/handler/request_aborted.rs` | Probably no change | PENDING | — |
| 35 | `statistics/event/handler/response_sent.rs` | Probably no change | PENDING | — |
| 36 | `testing/mod.rs` | Wiring only | PENDING | — |
| 37 | `testing/environment.rs` | Lifecycle (#1488 SI-14/SI-17) | PENDING | — |

Wiring-only `mod.rs` files with no executable lines may conclude in a few lines; they still get a
plan so the ledger has no implicit exceptions.

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
- [x] Shared plan guidance and the first file test plan (`error.rs`) created; no tests changed.
- [x] First file test plan reviewed and approved before test-producing work.
- [ ] Every ledger row is `DONE` with its own file test plan (37 / 37; currently 22 / 37).
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
- 2026-09-22 11:00 UTC - User/maintainer - Corrected the process: the T1 inventory rows are
  hypotheses, not terminal decisions. Every Rust file must be processed through its own file test
  plan (review/refactor current tests, then analyse and add unit or integration coverage, then
  record coverage). T4 is not complete; T5-T8 wait.
- 2026-09-22 11:00 UTC - GitHub Copilot - Reopened T4 as `IN_PROGRESS`, rewrote the per-file
  workflow with explicit approval gates and commit points, and reset the ledger to 36 `PENDING`
  files in processing order. `lib.rs` is next.
- 2026-09-22 11:15 UTC - GitHub Copilot - Started `lib.rs` as the only `IN_PROGRESS` file and
  created [lib-tests.md](test-refactor-plans/lib-tests.md). The plan records the 28 / 29 unit-only
  baseline, test-support-helper review scope, unit/integration classifications, and approval gate.
  No tests or production code changed.
- 2026-09-22 12:45 UTC - User/maintainer - Approved the `lib.rs` completed-file review.
- 2026-09-22 12:45 UTC - GitHub Copilot - Completed the `lib.rs` plan as a no-change result:
  crate-root declarations and test-support helpers have no package-owned runtime decision, while
  the helper caller is owned by the future `handlers/announce.rs` plan.
- 2026-09-22 12:50 UTC - GitHub Copilot - Started `container.rs` as the only `IN_PROGRESS` file
  and created [container-tests.md](test-refactor-plans/container-tests.md). The plan records its
  complete unit-only baseline, current enabled-publication test, ownership boundaries, and
  approval gate. No tests or production code changed.
- 2026-09-22 16:25 UTC - User/maintainer - Approved the completed `container.rs` file result:
  no refactor or new test, with unchanged 100.00% unit-only line coverage. The signed
  documentation-only file increment is pending validation and commit.
- 2026-09-22 16:27 UTC - GitHub Copilot - Started `event.rs` as the only `IN_PROGRESS` file and
  created [event-tests.md](test-refactor-plans/event-tests.md). Fresh unit-only coverage found
  three untested `Error::ScrapeFailed` classification branches. The plan also identifies one
  existing test that combines display and metrics-label contracts for R1 review.
- 2026-09-22 16:38 UTC - User/maintainer - Approved the completed `event.rs` file result:
  one test refactor and four unit tests increased unit-only line coverage from 95.42% to 100.00%.
  The signed test and documentation increment is pending validation and commit.
- 2026-09-22 16:43 UTC - GitHub Copilot - Started `handlers/mod.rs` as the only `IN_PROGRESS`
  file and created [handlers-mod-tests.md](test-refactor-plans/handlers-mod-tests.md). Fresh
  unit-only coverage found the `CookieTimeValues::new` range calculation as the only uncovered
  package-owned decision. The plan proposes one direct deterministic unit test.
- 2026-09-22 17:14 UTC - User/maintainer - Approved the completed `handlers/mod.rs` file result:
  two tests increased unit-only line coverage from 85.98% to 96.40%. The signed test and
  documentation increment is pending validation and commit.
- 2026-09-22 17:16 UTC - GitHub Copilot - Started `handlers/connect.rs` as the only
  `IN_PROGRESS` file and created [handlers-connect-tests.md](test-refactor-plans/handlers-connect-tests.md).
  Fresh unit-only coverage is complete, but R1 identifies duplicate IPv4 response tests and test
  naming/AAA cleanup. No tests or production code changed.
- 2026-09-22 17:23 UTC - User/maintainer - Approved the completed `handlers/connect.rs` file
  result: one duplicate test was removed while unit-only coverage remained 100.00%. The signed
  test and documentation increment is pending validation and commit.
- 2026-09-22 17:25 UTC - GitHub Copilot - Started `handlers/announce.rs` as the only
  `IN_PROGRESS` file and created [handlers-announce-tests.md](test-refactor-plans/handlers-announce-tests.md).
  Fresh unit-only coverage found the six-line error adaptation from `UdpAnnounceError` to
  `HandlerError`; the plan proposed one strict invalid-cookie collaboration test plus two event-test
  name corrections. No Rust tests or production code changed.
- 2026-09-22 18:30 UTC - Jose Celano - Approved the completed `handlers/announce.rs` review.
  Its plan and ledger row are `DONE`; unit-only coverage improved from 781 / 800 (97.63%) to 903
  / 905 (99.78%) lines. The next eligible file is `handlers/scrape.rs` after this file's signed
  commit.
- 2026-09-23 - Jose Celano - Approved the completed `handlers/scrape.rs` review. Its plan and
  ledger row are `DONE`; unit-only coverage improved from 319 / 321 (99.38%) to 356 / 357
  (99.72%) lines. Further improvement discussion for this file is deferred until after its signed
  commit; do not begin the next ledger file yet.
- 2026-09-29 10:32 UTC - Jose Celano - Approved the completed `banning/mod.rs` review. Its plan
  and ledger row are `DONE` with no Rust change; the file has no executable entries. The next
  file is `banning/event/mod.rs`.
- 2026-09-29 11:14 UTC - Jose Celano - Approved the completed `banning/event/mod.rs` review. Its
  plan and ledger row are `DONE` with no Rust change; the file has no executable entries. The next
  file is `banning/event/handler.rs`.
- 2026-09-29 11:31 UTC - Jose Celano - Approved the completed `banning/event/handler.rs` review.
  Its plan and ledger row are `DONE` with no Rust change; the current focused contracts cover its
  package-owned orchestration. The next file is `banning/event/listener.rs`.
- 2026-09-29 11:59 UTC - Jose Celano - Approved the completed `banning/event/listener.rs` review.
  Its plan and ledger row are `DONE` with no Rust change; the current deterministic dispatch
  contracts remain sufficient while spawned lifecycle behavior is #1488-owned. The next file is
  `statistics/mod.rs`.
- 2026-09-29 13:21 UTC - Jose Celano - Approved the completed `statistics/mod.rs` review. Its
  plan and ledger row are `DONE` with no Rust change; repository initialization remains the metric
  composition's observable boundary. The next file is `statistics/metrics.rs`.

## Acceptance Criteria

- [ ] A complete `packages/udp-server/src/` module inventory is recorded with unit-only coverage,
      property-test candidacy, selected behavior, and a T1 hypothesis for every source file.
- [ ] Every Rust source file in the package has its own file test plan, processed through the
      per-file workflow, with its resulting unit-only coverage recorded in `coverage-evidence.md`.
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
| AC2 | TODO | Ledger 37 / 37 `DONE`; one plan per file; per-file results table in `coverage-evidence.md` |
| AC3 | TODO | Coverage evidence tables and command output summaries |
| AC4 | TODO | File test plans and test output |
| AC5 | TODO | Integration-boundary rationale in file test plans |
| AC6 | TODO | No-change/deferral conclusions in file test plans and `coverage-evidence.md` |
| AC7 | TODO | File test plans and focused validation evidence |
| AC8 | TODO | File test plans: R1 reviewed before R2; completed-file review recorded |
| AC9 | TODO | `mutation-evidence.md` |
| AC10 | TODO | Reconciliation progress-log entry and final diff review |
| AC11 | TODO | Nightly rustfmt output |
| AC12 | TODO | `linter all` output |
| AC13 | TODO | Package test output |
| AC14 | TODO | `manual-verification-evidence.md` |
| AC15 | TODO | Post-implementation acceptance review |
| AC16 | TODO | Documentation diff or no-change rationale |

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
