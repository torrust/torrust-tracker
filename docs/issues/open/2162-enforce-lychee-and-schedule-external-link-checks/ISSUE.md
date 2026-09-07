---
doc-type: issue
issue-type: task
status: open
priority: p3
epic: null
github-issue: 2162
spec-path: docs/issues/open/2162-enforce-lychee-and-schedule-external-link-checks/ISSUE.md
branch: "2162-enforce-lychee-and-schedule-external-link-checks"
related-pr: null
last-updated-utc: 2026-09-07 14:47
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - .github/skills/dev/planning/create-issue/SKILL.md
    - docs/issues/open/2150-add-lychee-link-checker/ISSUE.md
    - lychee.toml
---

<!-- skill-link: create-issue -->

# Issue #2162 - Enforce local Markdown links and schedule advisory external link checks

## Goal

After `torrust-linting` adds lychee support, enforce deterministic local Markdown file and
fragment checks through the existing pre-commit and normal CI linting paths. Add a separate,
advisory weekly workflow for external URLs so link rot is detected without making network failures
a merge blocker.

## Background

Tracker issue [#2150](https://github.com/torrust/torrust-tracker/issues/2150) established
`lychee.toml` with `offline = true` and `include_fragments = "full"`, repaired the maintained
Markdown local-link baseline, and opened [torrust/torrust-linting#3](https://github.com/torrust/torrust-linting/issues/3)
to add lychee to the shared `linter` CLI.

Local links and anchors are deterministic and fast, so they are appropriate for pre-commit and
normal CI. External links need network requests and may fail due to transient network failures,
rate limits, or a third party being unavailable. They must therefore run only in a scheduled or
manually dispatched workflow that is not a required PR status check. Lychee should fail normally
in that workflow so failures remain visible and actionable; the non-blocking property comes from
not making the workflow a merge requirement, not from `continue-on-error`.

## Scope

### In Scope

- Block implementation on completion of `torrust/torrust-linting#3`.
- Wire the shared offline `linter lychee` check into this repository's normal linting path so
  `lychee.toml` validates maintained local Markdown file and fragment links in pre-commit and
  normal CI.
- Add a dedicated weekly GitHub Actions workflow with `workflow_dispatch` for online lychee checks.
- Configure the external check to use the repository `GITHUB_TOKEN` without logging it, bounded
  timeouts/retries/concurrency, and an artifact containing a readable lychee report.
- Document the workflow's advisory status and triage procedure: re-run once; fix a persistently
  broken URL or add a narrowly scoped documented exclusion when justified.

### Out of Scope

- Modifying `torrust-linting`; that work belongs to `torrust/torrust-linting#3`.
- Making the scheduled external check a required branch-protection/PR status check.
- Treating temporary external failures as a reason to change the offline local-link policy.
- Broadly excluding external hosts or paths without documented, specific rationale.

## Architectural Decisions

- Related ADRs: `None`
- ADRs to create: `None known`

This is workflow configuration using the existing shared linter and lychee configuration; no
architecture decision is anticipated.

## Design and Ownership Review

Not applicable — this work configures existing CLI execution in GitHub Actions and does not
introduce child-process lifecycle, network-readiness, or reusable-fixture abstractions.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status  | Task                                                       | Notes / Expected Output                                                                                       |
| --- | ------- | ---------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------- |
| T1  | BLOCKED | Confirm `torrust/torrust-linting#3` is merged/released     | Record the shared linter version and any CLI/configuration contract changes                                   |
| T2  | TODO    | Wire offline lychee linting into local and normal CI paths | `linter all` performs deterministic local file/fragment validation using `lychee.toml`                        |
| T3  | TODO    | Add advisory external-link workflow                        | Weekly schedule plus `workflow_dispatch`; not required for PR merges; lychee failures remain red              |
| T4  | TODO    | Bound and secure online requests                           | Use `GITHUB_TOKEN` without exposing it; configure bounded timeout, retries, concurrency, and request interval |
| T5  | TODO    | Upload readable failure report                             | Retain report artifact on failure with documented retention period                                            |
| T6  | TODO    | Document triage procedure                                  | Re-run once, then fix persistent URLs or add a narrow documented exclusion                                    |
| T7  | TODO    | Verify normal and advisory workflows                       | Record successful local/CI offline execution plus manual scheduled-workflow result                            |

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/drafts/enforce-lychee-and-schedule-external-link-checks/ISSUE.md`
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue created and issue number added to this spec
- [ ] Implementation completed after `torrust/torrust-linting#3` is available
- [ ] Automatic verification completed (`linter all`, relevant workflow checks)
- [ ] Manual verification scenarios executed and recorded (status + evidence)
- [ ] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] Evidence-based implementation completion review recorded
- [ ] Reviewer validated acceptance criteria and updated checkboxes
- [ ] Issue closed and spec moved to `docs/issues/closed/`

### Progress Log

- 2026-09-07 12:01 UTC - Copilot - Drafted follow-up specification required by #2150 after creating torrust/torrust-linting#3 - `docs/issues/drafts/enforce-lychee-and-schedule-external-link-checks/ISSUE.md`
- 2026-09-07 14:47 UTC - Copilot - User approved the specification; created GitHub issue #2162 - https://github.com/torrust/torrust-tracker/issues/2162

## Acceptance Criteria

- [ ] AC1: `torrust/torrust-linting#3` is complete and the selected shared-linter version supports
      lychee with the repository `lychee.toml` convention.
- [ ] AC2: `linter all` includes deterministic offline local Markdown file and fragment validation
      in pre-commit and normal CI.
- [ ] AC3: A scheduled workflow runs the online external-link check weekly and via
      `workflow_dispatch`.
- [ ] AC4: The scheduled workflow is not required to merge pull requests; lychee failures remain
      visible as workflow failures rather than being hidden by `continue-on-error`.
- [ ] AC5: The scheduled workflow uses `GITHUB_TOKEN` securely and has bounded timeout, retries,
      concurrency, request interval, and report-artifact retention.
- [ ] AC6: Documentation defines the external-link failure triage procedure.
- [ ] `linter all` exits with code `0`.
- [ ] Relevant tests pass.
- [ ] Manual verification scenarios are executed and documented (status + evidence).

## Verification Plan

### Automatic Checks

- `linter all`
- Relevant GitHub Actions workflow lint/validation checks

### Manual Verification Scenarios

| ID  | Scenario                 | Command/Steps                                                | Expected Result                                                                              | Status | Evidence             |
| --- | ------------------------ | ------------------------------------------------------------ | -------------------------------------------------------------------------------------------- | ------ | -------------------- |
| M1  | Offline enforcement      | Run `linter all` locally and inspect normal CI               | Local Markdown file and fragment checks run and pass                                         | TODO   | Command/CI URL       |
| M2  | Scheduled external check | Run the new workflow with `workflow_dispatch`                | Online check runs; report is available; outcome does not affect PR merge requirements        | TODO   | Workflow URL         |
| M3  | Failure triage           | Re-run a deliberately observed or simulated external failure | Procedure distinguishes transient from persistent failure without hiding the initial failure | TODO   | Workflow URLs/report |

## Risks and Trade-offs

- **Upstream dependency delay**: `torrust/torrust-linting#3` may not be available when this issue
  is planned. Mitigation: T1 blocks implementation; do not duplicate the linter integration here.
- **Network flakiness**: external services can be transiently unavailable. Mitigation: weekly,
  advisory execution with a manual re-run and no merge-blocking status requirement.
- **Secret exposure**: `GITHUB_TOKEN` could be exposed by unsafe logging. Mitigation: pass it only
  through the workflow environment and never print environment variables or command traces.

## Implementation Completion Review

- Retrospective: `Not yet assessed`
- Create an issue-local `implementation-retrospective.md` if implementation reveals reusable
  workflow or lychee integration lessons; otherwise record why none was needed in the progress log.

## References

- Parent issue: [#2150](https://github.com/torrust/torrust-tracker/issues/2150)
- Upstream dependency: [torrust/torrust-linting#3](https://github.com/torrust/torrust-linting/issues/3)
- lychee configuration: <https://lychee.cli.rs/guides/config/>
- lychee GitHub Actions guidance: <https://lychee.cli.rs/continuous-integration/github/>
