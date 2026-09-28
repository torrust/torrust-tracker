---
schema-version: 1
doc-type: issue
issue-type: bug
status: planned
priority: p3
epic: 1347
github-issue: 2361
spec-path: docs/issues/open/2361-1347-package-coverage-summary-discovery-failure/ISSUE.md
branch: "2361-1347-package-coverage-summary-discovery-failure"
related-pr: null
last-updated-utc: "2026-09-28 10:00"
semantic-links:
  skill-links:
    - create-issue
    - fix-bug
    - implement-workflow
  related-artifacts:
    - .github/skills/dev/debugging/fix-bug/SKILL.md
    - .github/skills/dev/planning/create-issue/SKILL.md
    - .github/workflows/generate_coverage_pr.yaml
    - contrib/dev-tools/checks/package-coverage-check/src/main.rs
    - docs/issues/closed/2222-1347-package-coverage-regression-ci/ISSUE.md
    - review-finding:pr-2293-f1
---

<!-- skill-link: create-issue -->

# Issue #2361 - Keep the Report-Only Package Coverage Summary From Failing on a Discovery Failure

**Parent EPIC:** #1347 - Overhaul: Packages Testing

## Goal

When the package coverage discovery job fails or produces no output, the report-only
"Package Coverage Regression" check should not turn red with a JSON parse error. It should either
pass with a summary that says discovery was unavailable, or fail with a message that names the
real cause. The maintainer chooses which.

## Background

Issue #2222 (PR #2293) added the report-only pull-request check "Package Coverage Regression", the
`package-coverage-summary` job in `generate_coverage_pr.yaml`. It runs under
`if: ${{ always() }}` so that it reports even when earlier jobs are skipped. It declares no
`continue-on-error`.

What happens today, step by step:

1. The `discover-package-coverage` job fails, is cancelled, or is skipped.
2. Its `discovery` output then resolves to the empty string.
3. The summary step still runs (`always()`) and writes that empty string to
   `package-coverage-discovery.json` with `printf '%s' "$DISCOVERY"`.
4. `package-coverage-check summary` reads the file with `serde_json` and fails:
   `invalid JSON: EOF while parsing a value at line 1 column 0`. The process exits `1`.
5. The step fails, so the job fails, so the check "Package Coverage Regression" is red.

The expectation this violates: the check is documented as informational and non-blocking
("This report is informational and does not block merging."). An infrastructure failure upstream
appears as a red check named for a coverage regression, and its message is a JSON parse error that
does not name the cause. A failure of the summary job's download step has the same effect.

Impact today: no false result has been observed. In the 100 most recent runs of the workflow (from
2026-09-22 09:14 UTC), `Discover Package Coverage` never failed. The two failed runs failed in the
separate `Generate Coverage Report` job, and `Package Coverage Regression` passed in both. The
defect shows only when discovery fails. Then it misleads the reader about what broke, and adds a
second red check that repeats the discovery job's failure.

The comparison-artifact side already degrades gracefully: `read_comparison_artifacts` returns an
empty list for a missing directory. Discovery input has no equivalent.

Source: `review-finding:pr-2293-f1` from the post-merge review of PR #2293, routed here by #2347
(<https://github.com/torrust/torrust-tracker/issues/2347#issuecomment-5867552349>). The hosted
rollout review of the same feature is owned separately by #2301.

## Scope

### In Scope

- Decide the intended outcome when discovery is unavailable:
  - **Report**: the summary renders an explicit "discovery unavailable" report, naming the
    discovery job's result, and exits `0`. The discovery job's own red status remains the failure
    signal.
  - **Fail clearly**: the summary exits non-zero with a message naming the missing discovery
    input instead of a parse error.
- Implement that outcome in `package-coverage-check` and keep the workflow a thin adapter, for
  example by passing `needs.discover-package-coverage.result` to the tool. Follow the
  `implement-workflow` skill.
- Cover the empty-discovery case with a maintained test in the tool.
- Decide the same question for a failed results download.

### Out of Scope

- The hosted rollout review, the warning threshold, and the #2222 M2/M3 evidence (owned by #2301).
- Least-privilege `permissions:` for the workflow (declined in #2347).
- Making the check required.

## Architectural Decisions

- Related ADRs: `docs/adrs/20260821172000_establish_ai_agent_context_capability_and_portability_governance.md`
  (keep logic in repository-owned tools, with the workflow as an adapter).
- ADRs to create: `None known`.

## Design and Ownership Review

Not applicable. The change is argument handling and output in a synchronous command-line tool,
plus a workflow input; it adds no process, asynchronous I/O, readiness, or fixture ownership.

## Bug-Fix Process

Follow [`fix-bug`](../../../../.github/skills/dev/debugging/fix-bug/SKILL.md):

1. Analysis: the causal decision is `summary` in `package-coverage-check/src/main.rs`, which calls
   `read_json::<SummaryInput>` on the discovery path and propagates the parse error. Nothing
   distinguishes absent discovery input from malformed input.
2. Reproduction: **Reproduced** at the tool level on `develop` at `478516cf`, recorded in
   `manual-verification-evidence.md` V0. An empty discovery file exits `1` with the parse error;
   a valid empty discovery renders the informational report and exits `0`.
3. Regression-test boundary: see below.
4. Red: the maintained test fails against the current tool.
5. Fix: the smallest change that implements the approved outcome.
6. Green and recheck: the test passes, and V0's command is re-run like-for-like.

## Regression Test Strategy

The boundary is an integration test in `contrib/dev-tools/checks/package-coverage-check/tests/`,
next to `summary.rs`. It runs the `summary` subcommand with an empty discovery file and a missing
results directory, and asserts the approved exit status and output. The observable contract is
the command's exit status and rendered text, which a unit test of `render_summary` would not
cover. If the fix extracts a pure function for discovery-input handling, add a unit test there as
well. The hosted red check itself is not reproduced; the workflow semantics (`always()` and no
`continue-on-error`) make the tool's exit status decisive.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID | Status | Task | Notes / Expected Output |
| -- | ------ | ---- | ----------------------- |
| T1 | TODO | Maintainer decision on the outcome | Report or Fail clearly, for missing discovery and for a failed download; recorded in the progress log. |
| T2 | TODO | Add the regression test and prove it red | Test committed only after its failing run against the current tool is recorded in `manual-verification-evidence.md`. |
| T3 | TODO | Fix the tool and adapt the workflow | Smallest change implementing T1; workflow passes only the needed inputs. |
| T4 | TODO | Green and like-for-like recheck | Test passes; V0's command re-run with the recorded output; `linter all` and the tool's tests pass. |

## Commit Points

| Task | Coherent change set | Commit policy |
| ---- | ------------------- | ------------- |
| T2 + T3 | Regression test with the fix | One signed `fix(ci)` commit, after the red run is recorded. |
| T3 | Workflow input change, if any | One signed `ci(coverage)` commit. |
| T4 | Evidence and tracking | One signed `docs(issues)` commit. |

For the test, use the `write-unit-test` skill and record the prose-first Arrange-Act-Assert
review in the evidence file before committing.

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/drafts/1347-package-coverage-summary-discovery-failure/ISSUE.md`
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue created, linked as a sub-issue of #1347, and issue number added to this spec
- [ ] Implementation completed
- [ ] Automatic verification completed (`linter all`, relevant tests, and any pre-push checks)
- [ ] Manual verification scenarios executed and recorded in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] Evidence-based implementation completion review recorded: issue-local retrospective created for material discoveries, or progress log states why none was needed
- [ ] Reviewer validated acceptance criteria and updated checkboxes
- [ ] Independent reviewer reports recorded in issue-local `agent-review-reports.md` when reviewers received this folder-style specification
- [ ] Committer verified spec progress is up to date before commit
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-09-28 09:56 UTC - GitHub Copilot - Drafted as follow-up of #2347 for `review-finding:pr-2293-f1`, as the T3 amendment approves. Reproduced the bug at the tool level on `develop` at `478516cf` (`manual-verification-evidence.md` V0).
- 2026-09-28 10:00 UTC - GitHub Copilot - Maintainer approved the specification. Created GitHub issue #2361 with the `bug` label, linked it as a sub-issue of #1347 (`parent_issue_url` verified), registered it as order 9 in the #1347 EPIC, and moved this specification to `docs/issues/open/`.

## Acceptance Criteria

- [ ] AC1: With an empty discovery file and a missing results directory, `package-coverage-check
      summary` shows the approved outcome. With Report, it exits `0` and names that discovery was
      unavailable. With Fail clearly, it exits non-zero with a message naming the missing
      discovery input rather than a JSON parse error.
- [ ] AC2: A valid empty discovery still renders "No directly changed workspace package was
      selected." and exits `0`.
- [ ] AC3: A maintained test covers AC1 and was observed failing before the fix.
- [ ] AC4: The failed-download case follows the approved decision.
- [ ] `linter all` exits with code `0`
- [ ] Relevant tests pass
- [ ] Manual verification scenarios are executed and documented in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria are re-reviewed after implementation and reflect actual behavior
- [ ] Documentation is updated when behavior/workflow changes

## Verification Plan

### Automatic Checks

- `linter all`
- `cargo test -p package-coverage-check` (stable Rust toolchain)
- Pre-push checks

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID | Scenario | Human-oriented command/steps | Expected Result | Status | Evidence |
| -- | -------- | ---------------------------- | --------------- | ------ | -------- |
| M0 | Reproduce before the fix | V0's commands | Exit `1` with the JSON parse error | DONE | `manual-verification-evidence.md` section V0 |
| M1 | Like-for-like recheck after the fix | V0's commands on the fixed tool | The approved outcome (AC1); the control is unchanged (AC2) | TODO | `manual-verification-evidence.md` section V1 |

No disposable verification script is planned.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| ----- | ---------------------- | -------- |
| AC1 | TODO | M1; regression test |
| AC2 | TODO | M1 control |
| AC3 | TODO | Red and green runs in the evidence file |
| AC4 | TODO | Progress log decision; test or recorded rationale |

## Risks and Trade-offs

- With Report, the check stays green while discovery is broken. Mitigation: the discovery job
  remains red, and the summary names its result.
- With Fail clearly, two checks stay red for one cause, but the message is accurate.

## Implementation Completion Review

- Retrospective: `Not yet assessed`. Record in the progress log why none is needed, unless the fix
  shows a reusable lesson about `always()` summary jobs.

## References

- Source finding: `review-finding:pr-2293-f1` (<https://github.com/torrust/torrust-tracker/pull/2293#discussion_r4076532667>).
- Routing: #2347 and its T3 amendment.
- Related: #2222 (feature), #2301 (rollout review).
