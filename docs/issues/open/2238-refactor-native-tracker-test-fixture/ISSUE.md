---
doc-type: issue
issue-type: task
status: planned
priority: p2
epic: null
github-issue: 2238
spec-path: docs/issues/open/2238-refactor-native-tracker-test-fixture/ISSUE.md
branch: "2238-refactor-native-tracker-test-fixture-spec"
related-pr: null
last-updated-utc: 2026-09-16 15:40
semantic-links:
  skill-links:
    - create-issue
    - create-refactor-plan
  related-artifacts:
    - .github/skills/dev/planning/create-issue/SKILL.md
    - .github/skills/dev/planning/create-refactor-plan/SKILL.md
    - docs/refactor-plans/open/2238-refactor-native-tracker-test-fixture.md
    - tests/common/native_tracker.rs
    - tests/lifecycle/signals.rs
    - tests/configuration/cli_configuration.rs
---

<!-- skill-link: create-issue -->

# Issue #2238 - Refactor Native Tracker Test Fixture

## Goal

Make the native tracker executable fixture easier and safer to understand, review, test, and
change. Separate its private responsibilities so a maintainer, human or AI agent, can modify one
concern with a bounded context (the owning module, its colocated tests, and at most one or two
named collaborators) without loading or disturbing unrelated child-process behavior. Improve the
internal fixture API and module paths wherever that makes the test code clearer, while retaining
the executable behaviors and lifecycle guarantees the tests protect.

## Background

[`tests/common/native_tracker.rs`](../../../../tests/common/native_tracker.rs) is 1,272 lines and
currently combines normal tracker lifecycle management, temporary-workspace and command creation,
concurrent output capture and readiness probing, failed-start fixtures, Unix permission recovery,
and unit tests. It is compiled independently by the `lifecycle-signals` and `cli-configuration`
integration-test binaries.

The configuration tests need invalid CLI-source and failed-start helpers, while signal tests use
the normal running-tracker lifecycle. Keeping those separate responsibilities in one file makes
ownership invariants and future fixture changes harder to review.

The objective is not a smaller file by itself. The split should provide:

- clear ownership of running and failed-start child resources;
- local reasoning about configuration, output/readiness, and cleanup behavior;
- tests colocated with the implementation contract they protect;
- smaller review surfaces for future fixture changes;
- module-level docs that state each module's ownership and limits, so the invariants are
  Git-tracked rather than held in a contributor's or agent's working memory; and
- clear, expressive fixture calls in the two integration-test consumers, even when achieving that
  requires changing their imports or the fixture API.

This matters for AI-agent work in particular. An agent grounds a change in the code it retrieves;
a 1,272-line file forces it to load lifecycle, failure, rendering, and probing code to touch any
one of them, and hides which type owns a child or a deadline. Cohesive modules let an agent open one
module plus its tests, run one focused binary, and produce a diff a reviewer can bound. Splitting
further than one responsibility per module would reverse that benefit by scattering control flow.

Two constraints in the current code shape the implementation and are recorded in the plan: the
fixture is included by `#[path]` in both binaries, so nested module resolution must be decided
explicitly; and the two binaries use different subsets of the API, so the 21 `#[allow(dead_code)]`
attributes need a deliberate consolidation policy under `-D unused`.

The linked detailed plan is
[`docs/refactor-plans/open/2238-refactor-native-tracker-test-fixture.md`](../../../refactor-plans/open/2238-refactor-native-tracker-test-fixture.md).

## Scope

### In Scope

- Choose the module path and fixture API that best express the responsibilities used by the two
  integration-test binaries; update both consumers as part of the refactor.
- Decide and record the module layout under `#[path]` inclusion, including whether both binaries
  should continue to include one facade or import narrower fixture modules.
- Move failed-start preparation, invalid CLI sources, failure results, and permission restoration
  into a cohesive internal module.
- Move workspace/configuration creation and shared child-command construction into a cohesive
  internal module.
- Move output capture and health probing into cohesive internal modules, keeping the readiness
  loop with the running-tracker owner.
- Keep normal `NativeTracker` lifecycle and cleanup orchestration together and explicitly
  responsible for its owned child process.
- Give each module a short `//!` doc stating what it owns and what it must not do.
- Consolidate `#[allow(dead_code)]` attributes to module level only where an entire module is
  unused by one binary; keep item-level allowances elsewhere.
- Relocate unit tests with the implementation behavior they protect, preserving their coverage.
- Verify both affected executable-boundary integration test binaries and fixture unit tests.

### Out of Scope

- Changing tracker CLI configuration precedence, diagnostics, startup, health, or signal behavior.
- Altering timeout values, process signal policy, or test behavior merely as part of moving code.
- Routing the fixture through `tests/common/mod.rs` or sharing it with the in-process fixture.
- General refactoring of unrelated integration-test helpers.

## Architectural Decisions

- Related ADRs: None known.
- ADRs to create: None expected. This reorganizes internal test support and may improve its
  test-facing API. The module/API choices are implementation decisions recorded in the refactor
  plan, not ADRs. Create an ADR only if implementation identifies a durable cross-fixture design
  decision.

## Design and Ownership Review

Before moving code, record the responsibility map in the linked refactor plan and preserve these
invariants:

| Resource | Owner | Required invariant |
| --- | --- | --- |
| Running tracker child | `NativeTracker` (root module) | Normal shutdown waits for the child; drop-path cleanup kills and reaps it before its workspace is released. |
| Failed-start child | `NativeTrackerFailedStart` (`failed_start.rs`) | `wait_for_exit` reaps it and restores permissions; `Drop` remains a best-effort fallback that does not panic while unwinding. |
| Child stdout and stderr readers | `TrackerOutputCapture` (`output.rs`) | Both streams are drained concurrently and joined before contents are treated as final diagnostics. |
| Temporary workspace and configuration files | `NativeTrackerWorkspace` (`command.rs`) or the failed-start result | Paths remain available for diagnostics until the applicable child cleanup is complete. |
| Readiness operations | `NativeTracker` readiness loop (root module) | One absolute startup deadline bounds health probing and readiness retries. |
| Deadline constants | The module that owns the awaited resource | `STARTUP_DEADLINE`, `SHUTDOWN_DEADLINE`, `RETRY_INTERVAL` stay with the root; `FAILURE_DEADLINE` moves with failed start. |

The current consumer surface and the proposed per-module responsibility map are recorded in the
linked refactor plan. The current surface is inventory, not a compatibility promise: remove,
rename, or narrow items when that improves the resulting fixture and update both consumers in the
same refactor-plan item. After the first item that moves a resource owner, review that normal,
failure, and drop paths still have one clear owner each before proceeding.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

All code-refactor tasks, ordering, implementation commit points, and item statuses are owned
exclusively by the linked refactor plan. Do not duplicate them here. The issue-level tasks below
cover only planning-artifact workflow outside that plan.

| ID | Status | Task | Notes / Expected Output |
| --- | --- | --- | --- |
| I1 | DONE | Review and approve planning artifacts | Maintainer approved this issue specification and the linked refactor plan. |
| I2 | DONE | Create and link the GitHub issue | Created issue #2238, promoted both drafts to their open locations, and replaced placeholders. |
| I3 | IN_PROGRESS | Publish the planning artifacts | Preparing spec-only branch `2238-refactor-native-tracker-test-fixture-spec`; no fixture code belongs in it. |
| I4 | TODO | Close and archive the planning artifacts | After every refactor-plan item and the issue acceptance review are complete, close the GitHub issue and move both artifacts to `closed/`. |

## Commit Points

| Task | Coherent change set | Commit policy |
| --- | --- | --- |
| I1-I3 | Approved and published issue specification and refactor plan | Documentation-only spec commit after Markdown checks. |
| I4 | Closed issue specification and refactor plan | Archive only after plan completion and acceptance review. |

Implementation commit points are defined only in the linked refactor plan.

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/drafts/refactor-native-tracker-test-fixture/ISSUE.md`
- [x] Spec and refactor plan reviewed and approved by user/maintainer
- [x] GitHub issue #2238 created and issue number added to this spec
- [ ] (Optional, recommended for complex issues) Spec-only PR merged into `develop` before implementation
- [ ] Implementation completed
- [ ] Automatic verification completed (`linter all`, relevant tests, and any pre-push checks)
- [ ] Manual verification scenarios executed and recorded in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] Evidence-based implementation completion review recorded
- [ ] Reviewer validated acceptance criteria and updated checkboxes
- [ ] Independent reviewer reports recorded in issue-local `agent-review-reports.md` when reviewers received this specification
- [ ] Committer verified spec progress is up to date before commit
- [ ] Issue closed and specification moved to `docs/issues/closed/`

### Progress Log

- 2026-09-16 15:15 UTC - GitHub Copilot - Drafted the issue specification and linked refactor plan after inventorying the 1,272-line fixture, its responsibilities, and its two integration-test consumers. - `tests/common/native_tracker.rs`
- 2026-09-16 15:23 UTC - GitHub Copilot - Deepened both drafts: recorded the `#[path]` child-resolution constraint, the two-binary dead-code allowance policy, sibling naming collisions, the exact public surface, a per-module responsibility map, a maintenance task map, and a dependency-first extraction order. - `docs/refactor-plans/drafts/refactor-native-tracker-test-fixture.md`
- 2026-09-16 15:29 UTC - Maintainer and GitHub Copilot - Removed internal API/path compatibility as a constraint and made the refactor plan the sole implementation-task ledger; issue tasks now cover only planning-artifact workflow outside the plan. - This specification and linked refactor plan
- 2026-09-16 15:36 UTC - GitHub Copilot - Created GitHub issue #2238 after maintainer approval and promoted the issue specification and refactor plan to their numbered open locations. - https://github.com/torrust/torrust-tracker/issues/2238
- 2026-09-16 15:40 UTC - GitHub Copilot - Created the dedicated spec-only branch from the latest upstream `develop` and began publishing the approved planning artifacts. - `2238-refactor-native-tracker-test-fixture-spec`

## Acceptance Criteria

- [ ] AC1: Both integration-test consumers use a narrow, expressive fixture API and module layout;
  current paths and signatures may change when the result is clearer and more maintainable.
- [ ] AC2: Normal startup, readiness, graceful shutdown, and drop-path tracker cleanup retain
      their current ownership and deadline guarantees.
- [ ] AC3: Failed-start invalid-source behavior, output diagnostics, permission restoration, and
      fallback cleanup retain their current observable behavior.
- [ ] AC4: Configuration rendering/command construction and output/health support have clear
      internal ownership boundaries, without unnecessary generic abstractions, and no item is more
      visible than it was before.
- [ ] AC5: Fixture unit tests are colocated with their protected behavior and continue to pass.
- [ ] AC6: Every module under the fixture root has a `//!` doc stating what it owns and what it
      must not do, consistent with the plan's responsibility map.
- [ ] AC7: Every row of the plan's maintenance task map holds: the named primary module and
      collaborators are sufficient to make that change.
- [ ] AC8: `#[allow(dead_code)]` is consolidated to module level only where an entire module is
      unused by one binary, and no allowance hides code made dead by the refactor.
- [ ] `linter all` exits with code `0`.
- [ ] Relevant tests pass.
- [ ] Manual verification scenarios are executed and documented in issue-local `manual-verification-evidence.md`.
- [ ] Acceptance criteria are re-reviewed after implementation and reflect actual behavior.

## Verification Plan

### Automatic Checks

- `cargo test --test lifecycle-signals`
- `cargo test --test cli-configuration`
- `cargo test --test lifecycle-signals --test cli-configuration` after every extraction
- `linter all`
- Pre-push checks when preparing the implementation PR.

Review aid, not a gate: `git diff --color-moved=dimmed-zebra develop...HEAD -- tests/` should show
each extraction as moved blocks with only visibility, `use`, and doc edits highlighted as changes.

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID | Scenario | Human-oriented command/steps | Expected Result | Status | Evidence |
| --- | --- | --- | --- | --- | --- |
| M1 | Trace a running tracker lifecycle | Starting from a signal test, navigate through its final fixture API into startup, readiness, shutdown, and drop cleanup; record the owner of each child, output reader, and workspace. | The consumer reads clearly, the complete normal lifecycle is understandable without entering failed-start implementation, and every resource has one clear owner. | TODO | `manual-verification-evidence.md` section V1 |
| M2 | Trace a failed-start lifecycle | Starting from an invalid-source test, navigate through its final fixture API into source preparation, spawn, diagnostics, permission restoration, wait, and drop fallback; record the owner and deadline at each stage. | The consumer reads clearly, the complete failure lifecycle is understandable without entering normal readiness orchestration, and cleanup responsibilities and deadlines remain explicit. | TODO | `manual-verification-evidence.md` section V2 |
| M3 | Walk the maintenance task map | For each row of the plan's task map, open only the named primary module and collaborators and confirm the described change could be made there; note any row that would require opening another module. | Every row holds; any exception is recorded and either fixed or justified. | TODO | `manual-verification-evidence.md` section V3 |

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| --- | --- | --- |
| AC1 | TODO | Pending implementation |
| AC2 | TODO | Pending implementation |
| AC3 | TODO | Pending implementation |
| AC4 | TODO | Pending implementation |
| AC5 | TODO | Pending implementation |
| AC6 | TODO | Pending implementation |
| AC7 | TODO | Pending implementation |
| AC8 | TODO | Pending implementation |

## Risks and Trade-offs

- Moving ownership-sensitive cleanup code can introduce a child or workspace lifetime regression.
  Mitigation: retain the ownership map, use internal visibility, and run both executable-boundary
  integration-test binaries after each extraction.
- A bare `mod child;` inside a `#[path]`-loaded file resolves beside that file, not under a
  same-named directory. Mitigation: resolve the layout in the refactor plan before extracting
  children and verify with a compile of both binaries.
- `-D unused` turns an unused `pub(super)` item into a compile error in the binary that does not
  use it. Mitigation: apply the allowance policy deliberately; do not spray allowances to make the
  build pass.
- Splitting too aggressively can obscure the normal fixture flow. Mitigation: keep
  `NativeTracker` startup, readiness, shutdown, and `Drop` together in the root and extract only
  the leaf collaborators and the failed-start fixture.
- A private helper made public solely for cross-module access would expand the fixture contract.
  Mitigation: use `pub(super)` only; AC4 checks no item became more visible.
- Colocated tests that build private struct literals will fail to compile if a struct moves
  without its tests. Mitigation: move tests in the same commit as their struct.

## Implementation Completion Review

After implementation, compare the result with this specification. Record reusable lessons,
material design changes, and deviations from the approved plan in an issue-local
`implementation-retrospective.md` when warranted; otherwise add a progress-log entry explaining
why no retrospective is needed.

## References

- GitHub issue: https://github.com/torrust/torrust-tracker/issues/2238
- Related refactor plan: `docs/refactor-plans/open/2238-refactor-native-tracker-test-fixture.md`
- Affected fixture: `tests/common/native_tracker.rs`
- Affected test binaries: `lifecycle-signals`, `cli-configuration`
