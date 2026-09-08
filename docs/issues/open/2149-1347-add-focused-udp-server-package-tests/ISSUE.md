---
doc-type: issue
issue-type: task
status: open
priority: p2
epic: 1347
github-issue: 2149
spec-path: docs/issues/open/2149-1347-add-focused-udp-server-package-tests/ISSUE.md
branch: "2149-add-focused-udp-server-package-tests-spec"
related-pr: 2152
last-updated-utc: 2026-09-07 17:03
semantic-links:
  skill-links:
    - create-issue
    - write-unit-test
  related-artifacts:
    - .github/skills/dev/planning/create-issue/SKILL.md
    - .github/skills/dev/testing/write-unit-test/SKILL.md
    - docs/testing/refactoring-patterns/README.md
    - docs/issues/open/1347-overhaul-packages-testing/EPIC.md
    - docs/issues/open/1488-overhaul-tracker-shutdown/ISSUE.md
    - docs/issues/drafts/1488-si-14-migrate-udp-receive-reset-token-lifecycle/ISSUE.md
    - docs/issues/drafts/1488-si-15-define-udp-active-request-policy/ISSUE.md
    - docs/issues/drafts/1488-si-17-migrate-standalone-udp-environment/ISSUE.md
    - packages/udp-server/src/server/request_buffer.rs
    - packages/udp-server/src/server/launcher.rs
    - packages/udp-server/tests/server/contract.rs
    - docs/issues/open/2149-1347-add-focused-udp-server-package-tests/coverage-evidence.md
    - docs/issues/open/2149-1347-add-focused-udp-server-package-tests/performance-evidence.md
    - docs/issues/open/2149-1347-add-focused-udp-server-package-tests/test-refactor-plans/README.md
    - docs/issues/open/2149-1347-add-focused-udp-server-package-tests/test-refactor-plans/request-buffer-tests.md
    - docs/issues/open/2149-1347-add-focused-udp-server-package-tests/test-refactor-plans/event-tests.md
    - docs/issues/open/2149-1347-add-focused-udp-server-package-tests/test-refactor-plans/error-tests.md
    - docs/issues/open/2149-1347-add-focused-udp-server-package-tests/test-refactor-plans/bound-socket-tests.md
    - packages/udp-server/docs/adrs/20260907152707_keep_oldest_first_udp_request_eviction.md
---

<!-- skill-link: create-issue -->

# Issue #2149 - Add Focused UDP Server Package Tests

Parent EPIC: #1347 - Overhaul: Packages Testing

## Goal

Improve the maintainable package-local test safety net for
`torrust-tracker-udp-server`, concentrating on its UDP transport adaptation, packet-dispatch,
socket, and normal-operation overload contracts. Record evidence-based coverage decisions without
duplicating `udp-core`, `udp-protocol`, root composition, or pending shutdown work.

## Background

The UDP server is the BEP 15 delivery layer: it binds and receives UDP datagrams, applies
admission decisions, dispatches parsed packets to UDP-core-backed handlers, and publishes its
server facts for statistics and banning. It already has broad handler and metrics unit coverage,
plus eleven real-loopback UDP integration contracts. Important package-owned seams remain
untested, notably request-buffer capacity/cleanup, socket metadata, packet-error conversion, and
event/error classifications.

The active shutdown EPIC and its planned UDP subissues own cancellation, child-task joining,
active-request shutdown policy, and standalone environment/example lifecycle semantics. This task
must protect current normal-operation behavior without preempting that design.

## Scope

### In Scope

- Establish package-source coverage baseline and final evidence using a reproducible
  `cargo llvm-cov` command, aggregate comparison, per-file results, and prioritized uncovered
  behavior.
- Inventory current unit, real-loopback package integration, example, root integration, and
  relevant historical coverage before selecting new tests.
- Add focused, deterministic tests for package-owned transport and dispatch seams where they
  protect observable behavior: socket binding metadata, packet/error conversion, event/error
  classification, container composition, and normal-operation request-buffer capacity/cleanup.
- Establish and record a reproducible release-performance baseline before an approved production
  change to a UDP hot-path file. Compare equivalent repeated measurements after the change; do not
  require throughput measurements for test-only changes.
- Assess launcher admission behavior only where it can be tested without timing dependence,
  production refactoring, or a competing lifecycle design.
- Review every test-bearing file selected by the evidence inventory. Create one file-local
  refactor plan for each concrete opportunity, then improve test readability, maintainability,
  expressiveness, or behavior coverage without reducing valuable existing protection.
- Review `tests/server/contract.rs` and add only approved real-socket contracts that cover a
  stable package transport behavior not already protected at a better boundary.
- Perform a bounded mutation-testing assessment after the evidence and incremental test plan are
  approved; retain only behavior-relevant survivors as a follow-up queue.

### Out of Scope

- Redesigning UDP receive-loop cancellation, shutdown, active-request draining/deadlines, task
  joining, or the standalone environment/example lifecycle; these are owned by the #1488
  shutdown work and its SI-14, SI-15, and SI-17 drafts.
- Root application-composition tests for multi-listener metrics, shared banning, configuration
  ordering, or REST-observable aggregation; existing root integration tests remain their proper
  boundary.
- Raw-socket or privileged source-port-zero transport tests. Portable direct processor coverage is
  already the deepest feasible automated boundary.
- Duplicating UDP protocol parser/serializer fuzzing or property tests that belong to
  `udp-protocol`, or tracker business-rule tests owned by `udp-core` and `tracker-core`.
- Arbitrary coverage targets, broad test-factory refactors, new sleep-based tests, container E2E,
  or example signal tests without separate evidence and approval.

## Architectural Decisions

- Related ADRs: `docs/adrs/20260527175600_keep_protocol_and_domain_types_decoupled.md`
- Package-local ADR: `packages/udp-server/docs/adrs/20260907152707_keep_oldest_first_udp_request_eviction.md`
- Related shutdown governance: `docs/issues/open/1488-overhaul-tracker-shutdown/ISSUE.md`
- ADRs to create: None known. Create one if this work identifies another durable package or
  cross-package ownership/design decision.

## Design and Ownership Review

Non-lifecycle unit seams may be implemented without changing ownership. Before adding, changing,
or claiming coverage for asynchronous lifecycle fixtures, launcher shutdown, or the example,
review and record the following, then obtain maintainer approval:

| Resource                                    | Current owner                                        | Required invariant for any proposed test work                                                                       |
| ------------------------------------------- | ---------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------- |
| UDP receive-loop task                       | `Launcher::run_with_graceful_shutdown`               | The owner retains the task outcome; normal, failure, cancellation, and drop paths have explicit cleanup.            |
| Request processor tasks                     | `Launcher::run_udp_server_main` and `ActiveRequests` | Normal capacity eviction remains distinct from the shutdown drain/deadline policy owned by SI-15.                   |
| UDP-core, statistics, and banning listeners | `testing::environment::Environment`                  | Each listener's cancellation and joined/aborted completion is explicit before fixture resources are released.       |
| Readiness and shutdown waits                | Package test fixture / proposed seam                 | One absolute deadline bounds every awaited readiness, retry, response-decoding, child-exit, and teardown operation. |

The first passing vertical slice must be reviewed for responsibility coherence before another
lifecycle-related test increment. Do not introduce a generic fixture or new lifecycle abstraction
without evidence of a shared capability.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`, `DEFERRED`.

| ID  | Status      | Task                                        | Notes / Expected Output                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| --- | ----------- | ------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| T1  | DONE        | Record baseline and test-boundary inventory | [coverage-evidence.md](coverage-evidence.md) records the exact command, package-source scope, aggregate baseline, per-file detail, priority gaps, and external-coverage/deferral decisions.                                                                                                                                                                                                                                                                                  |
| T2  | IN_PROGRESS | Review and approve test design              | Inventory test-bearing files and create one file-local plan per concrete opportunity in [test-refactor-plans/](test-refactor-plans/README.md). Every plan now has an ordered cleanup phase for existing tests, then incremental missing-behavior tests with a design review after each addition. The request-buffer, event, and parse-error adapter plans are complete; the proposed [bound-socket plan](test-refactor-plans/bound-socket-tests.md) awaits maintainer approval. |
| T3  | DONE        | Improve request-buffer tests                | Completed the reviewed request-buffer plan: capacity-available, oldest-first eviction, and buffer-drop cleanup contracts are covered; R2 documents the intentional bounded policy and R5 defers the scheduler-dependent race guard. The current per-file comparison is recorded in [coverage-evidence.md](coverage-evidence.md). **Commit point:** completed through focused reviewed increments. |
| T4  | TODO        | Improve dispatch and classification tests   | Implement the approved plan increment(s) for `event.rs`, `error.rs`, or `handlers/mod.rs`. Keep event/error classification and packet-dispatch behavior separate from handler business rules. **Commit point:** one reviewed, coherent classification or dispatch increment plus focused validation.                                                                                                                                                                         |
| T5  | TODO        | Improve socket-adapter tests                | Implement the approved `server/bound_socket.rs` or `server/receiver.rs` plan increment for stable socket metadata, port-zero allocation, or receive adaptation. Do not assert platform-specific dual-stack defaults. **Commit point:** one reviewed socket-adapter increment plus focused validation.                                                                                                                                                                        |
| T6  | TODO        | Improve container-composition tests         | Implement a `container.rs` test-plan increment only if review identifies a package-owned composition regression not already proven indirectly. A justified no-change decision completes this task without a commit. **Commit point:** one reviewed composition increment plus focused validation, if code changes are warranted.                                                                                                                                             |
| T7  | TODO        | Improve admission or UDP contracts          | Implement one approved `server/launcher.rs` or `tests/server/contract.rs` increment only when the package integration boundary adds unique stable value. Record an infeasible seam rather than forcing a production refactor. **Commit point:** one reviewed admission or real-loopback contract increment plus focused validation.                                                                                                                                          |
| T8  | TODO        | Perform bounded mutation assessment         | Run a time-bounded sample against the completed changed/high-risk seam. Record configuration, duration, limitations, and behavior-relevant surviving mutants; do not create a score target or CI gate. **Commit point:** documentation-only commit if the evidence materially changes the tracked review queue.                                                                                                                                                              |
| T9  | TODO        | Review, verify, and complete evidence       | Stop for maintainer review after the final test increment, then run checks, manual scenarios, refreshed coverage, acceptance review, and completion review. **Commit point:** final documentation/evidence commit only after the required review and verification.                                                                                                                                                                                                           |

## Commit Points

Each test-producing task is deliberately a small, independently reviewable commit opportunity.
Finish the current file-local plan increment, its focused validation, and its required review before
starting the next task. Do not combine unrelated source/test areas merely to reduce commit count.

| Task | Coherent change set                                         | Commit policy                                                                                                                                                     |
| ---- | ----------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| T1   | Baseline coverage evidence and boundary inventory           | Included in the spec-only PR; no implementation commit.                                                                                                           |
| T2   | Test-design review and file-local refactor plans            | Included in the spec-only PR when possible. If analysis is performed after the spec-only PR, commit only the reviewed plans and evidence.                         |
| T3   | One `request_buffer.rs` test refactor or behavior increment | Commit after focused validation and review.                                                                                                                       |
| T4   | One classification or packet-dispatch file-plan increment   | Commit after focused validation and review; do not mix `event.rs`, `error.rs`, and `handlers/mod.rs` unless one approved plan proves they are inseparable.        |
| T5   | One socket-adapter file-plan increment                      | Commit after focused validation and review.                                                                                                                       |
| T6   | One container-composition increment, if warranted           | Commit after focused validation and review; record no-change decisions in the plan without an empty commit.                                                       |
| T7   | One launcher-admission or real-loopback contract increment  | Commit after focused validation and review. Do not combine with lifecycle redesign.                                                                               |
| T8   | Mutation evidence that changes the prioritized backlog      | Use a separate documentation-only commit only when it records a material decision or follow-up; otherwise include the evidence in the final documentation commit. |
| T9   | Refreshed evidence and implementation completion record     | Commit only after the maintainer review, full verification, and acceptance review are complete.                                                                   |

Use Conventional Commit messages that name the narrow changed package area, for example
`test(udp-server): cover active request eviction` or
`docs(issues): record udp server mutation evidence`. All commits must remain GPG signed.

## Test Development Loop

Apply this loop to every test-producing task after T1 and T2 are complete:

1. Complete the target file's refactor plan and obtain approval before changing its tests.
2. Add the smallest approved readability, maintainability, or behavior-focused increment.
3. Review the changed tests before the next increment. Make the one causal initial-state difference
   visible; use inline values, a readable builder, or a narrowly named scenario fixture according
   to the [test-pattern catalog](../../../testing/refactoring-patterns/README.md).
4. Run focused tests and correct failures. Record the result in the file-local plan.
5. Commit the coherent increment at its mapped commit point after focused validation and review.
6. Do not begin the next file's plan until the current plan's approved work has been reviewed and,
   when changes were made, committed.
7. After the final test-producing increment, stop for maintainer review before final verification,
   committing, or opening an implementation pull request.
8. Address feedback, then perform final verification and acceptance review.

Tests must retain the production Act and concrete expected result visibly. Helpers may encapsulate
only repeated mechanics and must not derive expected protocol outputs through production code.

## Test Refactor Plans

Test-bearing files are reviewed one at a time under
[test-refactor-plans/](test-refactor-plans/README.md). Each plan identifies the file's strengths
to preserve, maintainability problems, behavior gaps, and ordered changes from high-impact/low-
effort to lower-impact work. A plan may end in a justified no-change decision. Do not create a
cross-file helper or generic fixture unless a reviewed cross-file plan demonstrates one cohesive
responsibility.

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style draft created under `docs/issues/drafts/`.
- [x] Existing EPIC, completed #2136 and #2140 specifications, package code, tests, and related
      shutdown work reviewed.
- [x] Package-source coverage baseline recorded in issue-local evidence before test changes.
- [x] Per-file test-refactor-plan workflow established before implementation.
- [x] Draft specification reviewed and approved by user/maintainer.
- [x] GitHub issue #2149 created and linked to parent EPIC #1347 in this specification.
- [x] Draft moved to `docs/issues/open/` using the assigned issue number.
- [x] Spec-only PR #2152 opened against `develop` before implementation.
- [ ] Implementation completed.
- [ ] Automatic verification completed.
- [ ] Manual verification scenarios executed and recorded.
- [ ] Acceptance criteria reviewed after implementation and updated with evidence.
- [ ] Evidence-based implementation completion review recorded.
- [ ] Reviewer validated acceptance criteria and updated checkboxes.
- [ ] Committer verified specification progress before commit.
- [ ] Issue closed and specification moved to `docs/issues/closed/`.

### Progress Log

- 2026-09-07 08:49 UTC - GitHub Copilot - Created this draft after reviewing EPIC #1347,
  completed #2136 and #2140 work, current UDP-server source/tests, package boundaries, and the
  UDP shutdown drafts. The draft prioritizes package-owned deterministic seams and explicitly
  defers lifecycle redesign to #1488 SI-14, SI-15, and SI-17.
- 2026-09-07 09:23 UTC - GitHub Copilot - Completed T1 before any test changes. A clean
  `cargo llvm-cov` measurement at commit `2054d494` recorded the package-source baseline:
  4,814/4,965 lines (96.96%), 6,326/6,604 regions (95.79%), and 485/499 functions (97.19%).
  The reproducible command, per-file detail, priority queue, and boundary decisions are in
  [coverage-evidence.md](coverage-evidence.md).
- 2026-09-07 09:25 UTC - User/maintainer - Required the UDP-server work to use the completed
  #2136 opportunity model: review test-bearing files individually, create and approve refactor
  plans, then improve maintainability and behavior coverage incrementally rather than pursuing
  coverage alone.
- 2026-09-07 09:27 UTC - User/maintainer - Required small, coherent implementation steps with
  explicit commit points mapped directly to the implementation-plan tasks. The plan now separates
  request-buffer, dispatch/classification, socket-adapter, container, and listener-contract work.
- 2026-09-07 09:42 UTC - User/maintainer - Approved the draft specification. GitHub issue #2149
  was created and linked to parent EPIC #1347, and this folder was moved to its open-issue path
  for the spec-only PR.
- 2026-09-07 10:08 UTC - GitHub Copilot - Opened spec-only PR #2152 against `develop` from the
  fork branch `josecelano:2149-add-focused-udp-server-package-tests-spec`. The PR uses
  `Related to #2149` and does not close the implementation issue.
- 2026-09-07 11:10 UTC - GitHub Copilot - Spec-only PR #2152 was merged into `develop`. Created
  the implementation branch from the merged commit and began T2 with the proposed
  [request-buffer test refactor plan](test-refactor-plans/request-buffer-tests.md). No test or
  production change has been made; implementation awaits maintainer approval of R1.
- 2026-09-07 11:10 UTC - User/maintainer - Identified `ActiveRequests` as a UDP hot-path concern.
  Added a performance-evidence policy requiring equivalent release throughput baseline and after
  measurements before any approved hot-path production change, while keeping focused test-only
  changes free from unnecessary benchmark work.
- 2026-09-07 11:27 UTC - User/maintainer - Approved the request-buffer test refactor plan. Commit
  all accumulated #2149 planning and performance-evidence changes before beginning the R1
  test-only implementation increment.
- 2026-09-07 15:12 UTC - GitHub Copilot - Preserved the failed R2 experiment in an ignored handoff
  while investigating whether its full-scan expectation represented a production defect or an
  intentional policy. No production change was made.
- 2026-09-07 15:27 UTC - User/maintainer - After reviewing the request-buffer history, confirmed
  that R2's observed oldest-first eviction behavior is an intentional performance trade-off, not a
  defect. Approved a package-local ADR and source-comment clarification as an independent
  documentation commit. The unsupported bug-handoff conclusion is withdrawn.
- 2026-09-07 17:03 UTC - User/maintainer - Reviewed and approved completion of the request-buffer
  plan. Its current package-source measurement is 92.31% lines, 87.89% regions, and 95.65%
  functions for `server/request_buffer.rs`; the issue-local evidence records the baseline comparison.
- 2026-09-08 08:13 UTC - GitHub Copilot - Began the next file-local planning step after the
  completed request-buffer review. The proposed [event test plan](test-refactor-plans/event-tests.md)
  targets deterministic internal-error classification and request-kind metric representations without
  duplicating event emission, consumer, protocol, or tracker-core coverage.
- 2026-09-08 11:31 UTC - GitHub Copilot - Began the next file-local planning step after completing
  the event plan. The proposed [parse-error adapter plan](test-refactor-plans/error-tests.md) targets
  `RequestParseError` to server response-routing-metadata conversion without duplicating protocol
  parsing, event classification, or error-response serialization.
- 2026-09-08 11:38 UTC - User/maintainer - Required all subsequent file-local plans to first clean
  existing test code, then add missing behavior tests one at a time with a post-test design review.
  The shared plan guidance and proposed [bound-socket plan](test-refactor-plans/bound-socket-tests.md)
  now record this two-phase sequence.

## Acceptance Criteria

- [ ] Coverage evidence records reproducible package-source baseline/final measurements, scope,
      aggregate comparison, per-file detail, and prioritized gaps.
- [ ] The current unit, package integration, example, root/E2E, mutation, property, and fuzz
      evidence is assessed, with selected, deferred, and inapplicable levels justified.
- [ ] Every selected test-bearing file has a reviewed file-local refactor plan that records
      strengths, concrete problems, ordered improvements, guardrails, validation, and justified
      no-change decisions where applicable.
- [ ] Approved tests protect high-value UDP-server transport, dispatch, socket, event/error, or
      normal-operation overload behavior without duplicating lower or higher package ownership.
- [ ] Approved test refactors improve readability, maintainability, or expressiveness without
      reducing existing behavior coverage or hiding causal state, the production Act, or expected
      output in generic helpers.
- [ ] Request-buffer tests distinguish current normal-operation capacity/cleanup behavior from
      the shutdown policy owned by SI-15.
- [ ] Any approved production change to a UDP hot-path file has reproducible before/after release
      performance evidence with equivalent workload and environment details.
- [ ] Any asynchronous fixture or lifecycle test change completes the Design and Ownership Review,
      uses bounded absolute deadlines, and has a post-vertical-slice review.
- [ ] Package integration tests are added only when the actual loopback UDP boundary provides
      unique regression value; no sleep-based or privileged raw-socket test is added.
- [ ] `linter all` exits with code `0`.
- [ ] Relevant package tests pass.
- [ ] Manual verification scenarios are executed and documented.
- [ ] Acceptance criteria are re-reviewed after implementation and reflect observed behavior.
- [ ] Documentation is updated when behavior or workflow changes.

## Verification Plan

### Automatic Checks

- `cargo llvm-cov -p torrust-tracker-udp-server --all-features --json`
- `cargo test -p torrust-tracker-udp-server`
- `cargo test -p torrust-tracker-udp-server --test integration`
- `linter all`
- `TORRUST_GIT_HOOKS_LOG_DIR=.tmp ./contrib/dev-tools/git/hooks/pre-commit.sh`
- Pre-push checks when applicable

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID  | Scenario                       | Command/Steps                                                                                           | Expected Result                                                                                                | Status | Evidence                                            |
| --- | ------------------------------ | ------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------- | ------ | --------------------------------------------------- |
| M1  | Focused UDP transport contract | Manually invoke the selected real-loopback scenarios in `packages/udp-server/tests/server/contract.rs`. | The UDP client receives the documented BEP 15 response and observable side effect for every selected contract. | TODO   | Test names and output recorded after plan approval. |
| M2  | Full package regression        | Run `cargo test -p torrust-tracker-udp-server` after the final increment.                               | Unit and package integration tests pass together without timing-dependent teardown failures.                   | TODO   | Command output recorded after implementation.       |

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence                                                  |
| ----- | ---------------------- | --------------------------------------------------------- |
| AC1   | TODO                   | Issue-local `coverage-evidence.md`                        |
| AC2   | TODO                   | Test-design review and coverage evidence                  |
| AC3   | TODO                   | File-local plans in `test-refactor-plans/`                |
| AC4   | TODO                   | Approved refactor increments and focused validation       |
| AC5   | TODO                   | Focused test paths and test output                        |
| AC6   | TODO                   | Request-buffer tests and SI-15 deferral record            |
| AC7   | TODO                   | `performance-evidence.md` and any required result report  |
| AC8   | TODO                   | Design and Ownership Review or explicit non-applicability |
| AC9   | TODO                   | Approved real-loopback contract evidence                  |
| AC10  | TODO                   | `linter all` output                                       |
| AC11  | TODO                   | Package test output                                       |
| AC12  | TODO                   | Manual-verification table                                 |
| AC13  | TODO                   | Post-implementation acceptance review                     |
| AC14  | TODO                   | Documentation diff and completion review                  |

## Risks and Trade-offs

- UDP receive-loop and request-task behavior is concurrent and potentially timing-sensitive.
  Prefer isolated deterministic seams; defer shutdown semantics to the already planned lifecycle
  subissues rather than encoding accidental behavior in a test.
- Package coverage includes test code and can hide low-value framework or fixture coverage. Use it
  to navigate per-file gaps, while selecting tests by observable risk and ownership.
- Socket behavior varies by host IPv6 and dual-stack support. Test port-zero and endpoint metadata
  invariants, and retain existing availability guards rather than asserting a universal dual-stack
  default.
- Mutation testing can be slow and generate a tool-specific backlog. Keep it bounded and use only
  behavior-relevant surviving mutants to challenge assertions.
- A production hot-path refactor can cause a throughput regression even when its tests pass.
  Mitigate this with the conditional, reproducible baseline policy in
  [performance-evidence.md](performance-evidence.md), not with a single noisy benchmark run.
- A coverage increment can uncover a production defect outside its intended delivery scope.
  Mitigate this by preserving a reproducible handoff, fixing the defect on an independent branch,
  then rebasing this branch before resuming dependent coverage work.

## Implementation Completion Review

After implementation, compare the result with this specification. Record invalidated assumptions,
material design changes, unexpected verification results, and reusable test-design lessons.

- Retrospective: `Not yet assessed`
- Create `implementation-retrospective.md` from
  `docs/templates/IMPLEMENTATION-RETROSPECTIVE.md` for a material discovery, design change, or
  deviation. Otherwise record in the progress log why a separate retrospective was unnecessary.

## References

- Parent EPIC: https://github.com/torrust/torrust-tracker/issues/1347
- GitHub issue: https://github.com/torrust/torrust-tracker/issues/2149
- Spec-only PR: https://github.com/torrust/torrust-tracker/pull/2152
- Completed package-testing predecessors: #2136 and #2140
- Package: `packages/udp-server/`
- Current real-loopback contracts: `packages/udp-server/tests/server/contract.rs`
- Performance measurement policy: [performance-evidence.md](performance-evidence.md)
- Canonical benchmarking guide: `docs/benchmarking.md`
- Shutdown EPIC: `docs/issues/open/1488-overhaul-tracker-shutdown/ISSUE.md`
- UDP receive-loop lifecycle draft: `docs/issues/drafts/1488-si-14-migrate-udp-receive-reset-token-lifecycle/ISSUE.md`
- Active-request policy draft: `docs/issues/drafts/1488-si-15-define-udp-active-request-policy/ISSUE.md`
- Standalone environment draft: `docs/issues/drafts/1488-si-17-migrate-standalone-udp-environment/ISSUE.md`
- Portable source-port-zero rationale: `docs/issues/closed/1450-discard-udp-requests-from-clients-with-port-0/ISSUE.md`
