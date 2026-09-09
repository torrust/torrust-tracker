---
doc-type: issue
issue-type: task
status: in-progress
priority: p3
epic: null
github-issue: 2162
spec-path: docs/issues/open/2162-enforce-lychee-and-schedule-external-link-checks/ISSUE.md
branch: "2162-enforce-lychee-and-schedule-external-link-checks"
related-pr: null
last-updated-utc: 2026-09-08 16:35
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - .github/skills/dev/planning/create-issue/SKILL.md
    - docs/issues/open/2150-add-lychee-link-checker/ISSUE.md
    - lychee.toml
    - .github/lychee-online.toml
    - .github/workflows/external-link-check.yaml
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

| ID  | Status | Task                                                       | Notes / Expected Output                                                                                   |
| --- | ------ | ---------------------------------------------------------- | --------------------------------------------------------------------------------------------------------- |
| T1  | DONE   | Confirm `torrust/torrust-linting#3` is merged/released     | `torrust-linting` 0.2.0 provides `linter lychee` and offline Lychee in `linter all`                       |
| T2  | DONE   | Wire offline lychee linting into local and normal CI paths | Pinned 0.2.0 install; `testing.yaml` runs `linter all`, and `docs-lint.yaml` runs `linter lychee`         |
| T3  | DONE   | Add advisory external-link workflow                        | Weekly schedule plus `workflow_dispatch`; no push/pull-request triggers; Lychee failures remain red       |
| T4  | DONE   | Bound and secure online requests                           | `GITHUB_TOKEN` environment, 30-minute job timeout, and config bounds for request timeout, retry, and rate |
| T5  | DONE   | Upload readable failure report                             | Markdown artifact uploads with `if: always()` and 14-day retention                                        |
| T6  | DONE   | Document triage procedure                                  | `docs/testing.md` requires one rerun, then a URL fix or narrow documented exclusion                       |
| T7  | DONE   | Verify normal and advisory workflows                       | Local checks passed; two hosted dispatches visibly failed and each uploaded the retained Markdown report  |

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/drafts/enforce-lychee-and-schedule-external-link-checks/ISSUE.md`
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue created and issue number added to this spec
- [x] Implementation completed after `torrust/torrust-linting#3` is available
- [x] Automatic verification completed (`linter all`, relevant workflow checks)
- [x] Manual verification scenarios executed and recorded (status + evidence)
- [x] Acceptance criteria reviewed after implementation and updated with evidence
- [x] Evidence-based implementation completion review recorded
- [x] Reviewer validated acceptance criteria and updated checkboxes
- [ ] Issue closed and spec moved to `docs/issues/closed/`

### Progress Log

- 2026-09-07 12:01 UTC - Copilot - Drafted follow-up specification required by #2150 after creating torrust/torrust-linting#3 - `docs/issues/drafts/enforce-lychee-and-schedule-external-link-checks/ISSUE.md`
- 2026-09-07 14:47 UTC - Copilot - User approved the specification; created GitHub issue #2162 - https://github.com/torrust/torrust-tracker/issues/2162
- 2026-09-08 UTC - Copilot - Confirmed `torrust-linting` 0.2.0 release from the #3 completion handoff. Pinned tracker CI/setup linter installs, added offline local-link CI coverage, and added the separate weekly/manual online workflow with a dedicated configuration and retained report.
- 2026-09-08 UTC - Copilot - Installed `torrust-linting` 0.2.0 from crates.io and ran `linter lychee` successfully. The direct online Lychee preflight produced `lychee-report.md` and exited 2 for 428 visible existing external-link failures; this expected advisory result confirms failures remain actionable. It does not substitute for a GitHub Actions `workflow_dispatch` run.
- 2026-09-08 UTC - Copilot - `linter all`, including offline Lychee, and focused YAML, TOML, Markdown, spelling, and diff-whitespace checks passed. No implementation retrospective is needed: the work used the existing workflow and configuration patterns without a material design change or reusable new lesson. Independent acceptance review and GitHub Actions manual-dispatch evidence remain pending.
- 2026-09-08 16:26 UTC - Copilot - Manual dispatch [34250154466](https://github.com/torrust/torrust-tracker/actions/runs/34250154466) on `develop` completed with a visible failure at `Check External Links`; `Upload Lychee Report` succeeded. It scanned revision `e4db63d5`; the merged workflow definition subsequently confirmed the same hosted command and configuration. The retained `lychee-external-link-report` artifact was available (5,692 bytes, 442 errors), proving a failure does not hide the report.
- 2026-09-08 16:34 UTC - Copilot - Re-ran the observed failure using manual dispatch [34251836337](https://github.com/torrust/torrust-tracker/actions/runs/34251836337) on the same revision. It again visibly failed while successfully uploading its 5,818-byte report artifact, now showing 445 errors and two timeouts. The persistent, broadly distributed existing external-link failures are actionable under the documented triage policy; they do not change the deterministic offline policy. `develop` has effective GitHub ruleset requirements for creation, pull requests, deletion, and signatures, but no required-status-check rule; the only ruleset containing required status checks is disabled and does not name `External Link Check`, confirming this workflow is not a merge requirement.
- 2026-09-08 17:12 UTC - Task Reviewer - Independent completion review passed after the ruleset correction. All acceptance criteria and M1-M3 scenarios are supported by local and hosted evidence. The workflow remains advisory, although its external-link report is intentionally noisy until persistent failures are triaged. No separate implementation retrospective is needed.

## Acceptance Criteria

- [x] AC1: `torrust/torrust-linting#3` is complete and the selected shared-linter version supports
      lychee with the repository `lychee.toml` convention.
- [x] AC2: `linter all` includes deterministic offline local Markdown file and fragment validation
      in pre-commit and normal CI.
- [x] AC3: A scheduled workflow runs the online external-link check weekly and via
      `workflow_dispatch`.
- [x] AC4: The scheduled workflow is not required to merge pull requests; lychee failures remain
      visible as workflow failures rather than being hidden by `continue-on-error`.
- [x] AC5: The scheduled workflow uses `GITHUB_TOKEN` securely and has bounded timeout, retries,
      concurrency, request interval, and report-artifact retention.
- [x] AC6: Documentation defines the external-link failure triage procedure.
- [x] `linter all` exits with code `0`.
- [x] Relevant workflow lint and validation checks passed.
- [x] Manual verification scenarios are executed and documented (status + evidence).

## Verification Plan

### Automatic Checks

- `linter all`
- Relevant GitHub Actions workflow lint/validation checks

### Manual Verification Scenarios

| ID  | Scenario                 | Command/Steps                                                | Expected Result                                                                              | Status | Evidence                                                                                                                                                                                                                                                                                              |
| --- | ------------------------ | ------------------------------------------------------------ | -------------------------------------------------------------------------------------------- | ------ | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| M1  | Offline enforcement      | Run `linter lychee` locally and inspect normal CI            | Local Markdown file and fragment checks run and pass                                         | DONE   | 2026-09-08: `linter lychee` exited 0 after installing `torrust-linting` 0.2.0                                                                                                                                                                                                                         |
| M2  | Scheduled external check | Run the new workflow with `workflow_dispatch`                | Online check runs; report is available; outcome does not affect PR merge requirements        | DONE   | [Run 34250154466](https://github.com/torrust/torrust-tracker/actions/runs/34250154466) on revision `e4db63d5` failed visibly; its 5,692-byte report artifact uploaded successfully; the merged workflow retains the same command and configuration                                                    |
| M3  | Failure triage           | Re-run a deliberately observed or simulated external failure | Procedure distinguishes transient from persistent failure without hiding the initial failure | DONE   | [Run 34250154466](https://github.com/torrust/torrust-tracker/actions/runs/34250154466) reported 442 errors; [rerun 34251836337](https://github.com/torrust/torrust-tracker/actions/runs/34251836337) reported 445 errors and two timeouts; both ran revision `e4db63d5` and retained report artifacts |

## Risks and Trade-offs

- **Upstream dependency delay**: `torrust/torrust-linting#3` may not be available when this issue
  is planned. Mitigation: T1 blocks implementation; do not duplicate the linter integration here.
- **Network flakiness**: external services can be transiently unavailable. Mitigation: weekly,
  advisory execution with a manual re-run and no merge-blocking status requirement.
- **Secret exposure**: `GITHUB_TOKEN` could be exposed by unsafe logging. Mitigation: pass it only
  through the workflow environment and never print environment variables or command traces.

## Implementation Completion Review

- Retrospective: Not needed. The completed implementation followed existing repository workflow and configuration patterns, introduced no material design change, and produced no reusable lesson beyond the already documented online/offline Lychee policy.
- Create an issue-local `implementation-retrospective.md` if implementation reveals reusable
  workflow or lychee integration lessons; otherwise record why none was needed in the progress log.

## References

- Parent issue: [#2150](https://github.com/torrust/torrust-tracker/issues/2150)
- Upstream dependency: [torrust/torrust-linting#3](https://github.com/torrust/torrust-linting/issues/3)
- lychee configuration: <https://lychee.cli.rs/guides/config/>
- lychee GitHub Actions guidance: <https://lychee.cli.rs/continuous-integration/github/>
