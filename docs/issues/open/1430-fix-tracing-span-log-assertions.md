---
doc-type: issue
issue-type: bug
status: in_progress
priority: p2
epic: null
github-issue: 1430
spec-path: docs/issues/open/1430-fix-tracing-span-log-assertions.md
branch: "1430-fix-tracing-span-log-assertions"
related-pr: 1429
last-updated-utc: 2026-08-26 16:11
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - docs/adrs/20260826124959_use_explicit_identifiers_for_test_log_assertions.md
    - packages/test-helpers/src/logging.rs
    - packages/axum-http-server/tests/server/v1/contract/configured_as_whitelisted.rs
    - packages/axum-rest-api-server/tests/server/v1/contract/context/torrent.rs
    - .github/skills/dev/planning/create-issue/SKILL.md
---

<!-- skill-link: create-issue -->

# Issue #1430 - Document test log-assertion strategy and close span-scoping follow-up

## Goal

Document the decision to retain the repository-owned test log-capture helper and explicit,
developer-selected log-record identifiers. Close the span-scoped assertion follow-up without
changing production or test logging behavior.

## Background

The repository uses `torrust-tracker-test-helpers` to install one custom global tracing
subscriber. Its bounded shared buffer allows integration tests to search formatted log lines
through `logging::logs_contains_a_line_with`.

Existing assertions use natural identifiers such as a request ID or info hash. An earlier attempt
to identify captured records through a test-owned `tracing` span found that span context did not
appear reliably in spawned Tokio tasks, blocking work, or nested child tasks. The upstream
`tracing-test` issue documents the same limitation: automatic association of events across task
and thread boundaries is not generally possible; propagation must be applied deliberately at each
boundary.

The tracker has a highly concurrent execution model and complex nested tracing spans. Making test
scope propagation reliable would require auditing and maintaining explicit propagation across many
execution boundaries, while still leaving edge cases. It has no current unmet need for richer log
assertions: the repository-owned helper is working, customizable, and easier to inspect when a
test fails. Explicit identifiers deliberately selected by the test author are preferred to an
implicit span-based association strategy.

PR #1429 was superseded by merged PR #1735. That change simplified TLS configuration handling; it
did not introduce general tracing-context propagation or resolve this issue's assertion strategy.

## Scope

### In Scope

- Record an ADR establishing the repository-owned test logging helper and explicit identifiers as
  the current project strategy for assertions over captured logs.
- Record the limitations of `tracing` global initialization, the shared capture buffer, and
  automatic span association across asynchronous and blocking execution boundaries.
- Close GitHub issue #1430 as a documented decision rather than an implementation defect.

### Out of Scope

- Replacing the custom logging helper with the `tracing-test` crate.
- Propagating test-owned spans through Tokio tasks, `spawn_blocking`, OS threads, or nested
  execution paths.
- Changing the shared bounded capture buffer or refactoring existing request-ID and info-hash
  assertions.
- Creating a generic logging guide that duplicates the ADR without a concrete developer workflow
  requiring separate procedural documentation.

## Architectural Decisions

- Related ADRs: None known.
- ADRs to create: Document the test logging assertion strategy, its rationale, and the rejected
  automatic span-scoping alternative.
- Decision: retain `packages/test-helpers/src/logging.rs` as the test log-capture mechanism and
  use explicit developer-selected identifiers to locate expected records. Do not pursue automatic
  propagation of test-owned tracing spans through concurrent tracker execution.

## Implementation Plan

| ID  | Status | Task                                                 | Notes / Expected Output                                                                                          |
| --- | ------ | ---------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------- |
| T1  | DONE   | Review the original failure and upstream limitations | Confirmed the limitation affects spawned and blocking work and requires explicit propagation at each boundary.   |
| T2  | DONE   | Evaluate current tracker need and alternatives       | The custom helper and explicit identifiers satisfy current needs with less maintenance and better debuggability. |
| T3  | DONE   | Write the test logging strategy ADR                  | Added `docs/adrs/20260826124959_use_explicit_identifiers_for_test_log_assertions.md`.                            |
| T4  | DONE   | Validate and review the ADR                          | Maintainer approved the ADR; `linter all` passed.                                                                |
| T5  | TODO   | Merge the documentation PR and close the GitHub issue | The PR description must use `Closes #1430`; GitHub will close the issue when the PR merges.                      |

## Progress Tracking

### Workflow Checkpoints

- [x] Specification drafted for the existing GitHub issue
- [x] Specification reviewed and clarified with user/maintainer
- [x] Current need and alternatives assessed
- [x] ADR written and accepted
- [x] Documentation checks completed
- [x] Acceptance criteria reviewed after documentation implementation and updated with evidence
- [ ] Documentation PR opened and reviewed
- [ ] GitHub issue closed by the merged documentation PR
- [ ] Issue specification moved to `docs/issues/closed/` after PR merge

### Progress Log

- 2026-08-26 UTC - GitHub Copilot - Created the local implementation branch and drafted the
  source-of-truth repository specification from GitHub issue #1430.
- 2026-08-26 UTC - josecelano - Decided not to pursue automatic test-span propagation. The
  repository-owned helper and explicit developer-selected identifiers meet current needs and are
  more maintainable for the tracker's concurrent execution model. Requested an ADR and closure.
- 2026-08-26 UTC - GitHub Copilot - Drafted ADR
  `20260826124959_use_explicit_identifiers_for_test_log_assertions.md` and registered it in the
  ADR index. The ADR awaits review before it can be treated as accepted.
- 2026-08-26 UTC - GitHub Copilot - `linter all` passed for the ADR, index, and issue
  specification updates.
- 2026-08-26 UTC - josecelano - Approved the ADR and the documented decision to retain explicit
  identifiers for test log assertions.
- 2026-08-26 UTC - GitHub Copilot - Reopened GitHub issue #1430 after correcting the lifecycle:
  the documentation PR, rather than an issue state reason, will close it when merged.

## Acceptance Criteria

- [x] AC1: An ADR documents the repository-owned capture helper as the current strategy for test
      log assertions.
- [x] AC2: The ADR records explicit developer-selected identifiers as the preferred method for
      associating an assertion with an expected captured log record.
- [x] AC3: The ADR explains why automatic test-span propagation is not pursued: global tracing
      initialization, shared output, concurrent nested execution, and maintenance cost.
- [x] AC4: The ADR documents `tracing-test` and automatic span propagation as alternatives that
      may be reassessed if future test requirements justify their complexity.
- [x] AC5: `linter all` exits with code `0`.
- [x] AC6: The ADR is reviewed and accepted before closing #1430.

## Verification Plan

### Automatic Checks

- `linter all`
- Manual review of the ADR against the current helper and the linked issue history

### Manual Verification Scenarios

| ID  | Scenario                         | Command/Steps                                                                                                               | Expected Result                                                             | Status | Evidence                         |
| --- | -------------------------------- | --------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------- | ------ | -------------------------------- |
| M1  | ADR strategy review              | Compare the ADR against `packages/test-helpers/src/logging.rs`, #1147, #1148, #1149, and upstream `tracing-test` issue #23. | The ADR accurately describes the current helper, limitations, and decision. | DONE   | Maintainer approval (2026-08-26) |
| M2  | Future reopening criteria review | Review the ADR's conditions for reconsidering `tracing-test` or automatic span propagation.                                 | The decision remains reversible when concrete requirements change.          | DONE   | Maintainer approval (2026-08-26) |

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence                         |
| ----- | ---------------------- | -------------------------------- |
| AC1   | DONE                   | ADR 20260826124959               |
| AC2   | DONE                   | ADR 20260826124959               |
| AC3   | DONE                   | ADR 20260826124959               |
| AC4   | DONE                   | ADR 20260826124959               |
| AC5   | DONE                   | `linter all` (2026-08-26)        |
| AC6   | DONE                   | Maintainer approval (2026-08-26) |

## Risks and Trade-offs

- **Documentation drift**: a generic guide would repeat the ADR without serving a current
  workflow. Keep the ADR as the single source of truth; add procedural documentation only when a
  future contributor needs it.
- **Future requirements**: richer cross-task correlation may eventually justify a spike with the
  current `tracing-test` ecosystem or a targeted propagation design. The ADR must state these
  reopening criteria rather than presenting the decision as permanent.

## References

- GitHub issue: #1430
- Related PRs: #1147, #1148, #1149, #1429, #1735
- Upstream limitation: <https://github.com/dbrgn/tracing-test/issues/23>
- Existing helper: `packages/test-helpers/src/logging.rs`
- Existing log assertions: `packages/axum-http-server/tests/server/v1/contract/configured_as_whitelisted.rs`
  and `packages/axum-rest-api-server/tests/server/v1/contract/context/torrent.rs`
