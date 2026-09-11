---
doc-type: issue
issue-type: task
status: planned
priority: p3
epic: 2003
github-issue: 2185
spec-path: docs/issues/open/2185-2003-triage-advisory-external-link-check-findings/ISSUE.md
branch: "2185-2003-triage-advisory-external-link-check-findings"
related-pr: null
last-updated-utc: 2026-09-11 15:38
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - .github/skills/dev/planning/create-issue/SKILL.md
    - docs/issues/open/2003-overhaul-guardrails-and-automation/EPIC.md
    - docs/issues/closed/2162-enforce-lychee-and-schedule-external-link-checks/ISSUE.md
    - docs/issues/open/2185-2003-triage-advisory-external-link-check-findings/external-link-baseline.md
    - docs/issues/open/2185-2003-triage-advisory-external-link-check-findings/agent-review-reports.md
    - .github/workflows/external-link-check.yaml
    - .github/lychee-online.toml
---

<!-- skill-link: create-issue -->

# Issue #2185 - Triage advisory external-link check findings

**Parent EPIC:** [#2003 - Overhaul: Automation Tools and AI Agent Guardrails](https://github.com/torrust/torrust-tracker/issues/2003)

## Goal

Classify the persistent failures reported by the advisory External Link Check, repair a small, evidence-selected set of genuinely stale references, and document narrowly scoped exclusions only where a URL is intentionally uncheckable by automated online validation.

Preserve the weekly/manual workflow as a visible advisory signal. This task must reduce actionable noise without broadly suppressing external-link monitoring or changing the deterministic offline local-link policy.

## Background

Issue [#2162](https://github.com/torrust/torrust-tracker/issues/2162) added the online-only External Link Check workflow. The workflow deliberately fails when Lychee detects external-link failures and retains a Markdown report artifact, while remaining outside merge-required status checks.

Manual run [34347690674](https://github.com/torrust/torrust-tracker/actions/runs/34347690674) ran the merged workflow on `develop` revision `7abc30b2b9fb85b235e7b2ef2a40f5d5f7ed1555`. Its retained `lychee-external-link-report` artifact reported 1,677 total checks, 1,216 successes, 24 redirects, 461 errors, and no timeouts. The errors were 440 generic `ERROR` entries, 18 `404` responses, and 3 `403` responses.

The current report shows at least four materially different categories:

- GitHub pull-request review-comment fragment links, which account for most generic errors because the referenced dynamic fragments are not reliably available to Lychee.
- Intentionally non-routable local example endpoints such as `localhost` and `127.0.0.1`, which cannot be available on hosted runners.
- Genuine stale external references, including `404` links to moved repository paths, removed documentation pages, and unavailable docs.rs crate pages.
- Third-party access or content limitations, including `403` responses and missing fragments in externally hosted documentation.

The parent EPIC permits low-risk, additive, independently verifiable guardrail work before its architecture decision when it preserves the current integration point and does not select a shared runner, cache, enforcement platform, or broad migration. This task satisfies that exception: it investigates and incrementally improves the signal from an existing workflow without redesigning it.

## Scope

### In Scope

- Preserve a concise issue-local Markdown baseline that records the run URL, revision, summary counts, representative failures, and the classification method used.
- Categorize every distinct failing URL or recurring failure pattern in the baseline as repair, intentional exclusion candidate, transient/retry candidate, or unresolved investigation.
- Repair a bounded, reviewed set of clearly stale repository-controlled or third-party documentation references, with evidence that replacement targets are appropriate.
- Define only narrowly scoped, documented online exclusions for durable false-positive categories that cannot be meaningfully checked by the advisory workflow, such as dynamic GitHub review-comment fragments or local service examples.
- Keep local file and fragment validation in `lychee.toml` unchanged unless a separate local-link defect is independently found.
- Re-run the hosted workflow after each logical remediation slice; distinguish durable improvements from network variance and retain report evidence.
- Update `docs/testing.md` or the online configuration comments only when the implemented exclusion policy needs user-facing operational documentation.

### Out of Scope

- Broadly excluding `github.com`, `docs.rs`, all external URLs, or all URL fragments.
- Making the external-link workflow required for pull requests or masking it with `continue-on-error`.
- Replacing Lychee, changing the shared `torrust-linting` integration, or selecting the automation architecture under EPIC #2003.
- Repairing every historical review artifact in one unbounded change without a classification and prioritization decision.
- Closing issue #2162 or moving its issue specification; completed-issue cleanup remains a separate batch lifecycle process.

## Architectural Decisions

- Related ADRs: `None`
- ADRs to create: `None known`

This task uses the existing advisory workflow and configuration. Stop and propose an ADR if triage reveals a repository-wide policy change with meaningful long-term consequences, such as a new externally observable validation contract or a new permanent exclusion class beyond the current workflow's configuration.

## Design and Ownership Review

Not applicable. This is evidence-driven documentation maintenance and configuration refinement around an existing hosted workflow; it introduces no child-process lifecycle, network-readiness, resource-cleanup, or reusable-fixture abstraction.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task                                      | Notes / Expected Output                                                                                          |
| --- | ------ | ----------------------------------------- | ---------------------------------------------------------------------------------------------------------------- |
| T1  | DONE   | Preserve and classify the baseline        | `external-link-baseline.md` maps all 461 report errors to nine recurring categories and dispositions.            |
| T2  | TODO   | Repair clearly stale references           | Small, reviewable repairs replace or remove only references confirmed stale, with replacement-target evidence.   |
| T3  | DONE   | Add justified narrow exclusions           | C1/C9 and C2 online-only rules are verified on merged upstream hosted runs.                                      |
| T4  | DONE   | Revalidate hosted signal                  | Hosted run 34616458439 excluded C1/C9 and C2 while retaining visible unrelated failures and its report artifact. |
| T5  | TODO   | Document operations and review completion | Triage procedure, residual risks, acceptance evidence, and independent review are updated from observed results. |

## Commit Points

| Task  | Coherent change set                                                 | Commit policy                                                                                         |
| ----- | ------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------- |
| T1    | Add issue-local baseline and classification evidence only.          | Commit after `linter markdown`, `linter cspell`, and independent review of the classification.        |
| T2    | Repair one coherent category of confirmed stale references.         | Commit after focused local validation and a review of replacement-target evidence.                    |
| T3    | Add one documented, narrow online exclusion category, if justified. | Commit only after a hosted rerun proves the category is suppressed without hiding unrelated failures. |
| T4-T5 | Record hosted evidence and completion review.                       | Commit after focused documentation validation and independent review.                                 |

A category that needs no repository change is recorded in issue-local evidence without an empty commit.

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/drafts/2003-triage-advisory-external-link-check-findings/ISSUE.md`
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue created and issue number added to this spec
- [x] (Optional, recommended for complex issues) Spec-only PR merged into `develop` before implementation
- [x] Baseline classification independently reviewed
- [ ] Implementation completed
- [ ] Automatic verification completed (`linter all`, relevant tests, and any pre-push checks)
- [ ] Manual verification scenarios executed and recorded (status + evidence)
- [ ] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] Evidence-based implementation completion review recorded: issue-local retrospective created for material discoveries, or progress log states why none was needed
- [ ] Reviewer validated acceptance criteria and updated checkboxes
- [x] Independent reviewer reports recorded in issue-local `agent-review-reports.md` when reviewers received this folder-style specification
- [ ] Committer verified spec progress is up to date before commit
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-09-09 12:00 UTC - Copilot - Drafted a proposed EPIC #2003 subissue from External Link Check [run 34347690674](https://github.com/torrust/torrust-tracker/actions/runs/34347690674), which scanned merged `develop` revision `7abc30b2` and retained a report with 461 errors for classification before any remediation.
- 2026-09-09 12:10 UTC - GitHub Operator - Created [issue #2185](https://github.com/torrust/torrust-tracker/issues/2185) with the `task` label and linked it as a subissue of [EPIC #2003](https://github.com/torrust/torrust-tracker/issues/2003) after maintainer approval.
- 2026-09-09 15:15 UTC - Copilot - Downloaded the retained report from [run 34347690674](https://github.com/torrust/torrust-tracker/actions/runs/34347690674), classified all 461 errors in `external-link-baseline.md`, and selected only GitHub pull-request review-comment anchors as the first proposed remediation slice. No production configuration or link was changed.
- 2026-09-10 07:09 UTC - Task Reviewer - Independently re-parsed the retained report and passed the corrected classification: $416+7+14+3+1+5+3+4+8=461$, with C1/C9 covering 424 exact GitHub pull-request review-comment-anchor failures. No broad exclusion is proposed; no configuration or link changed in this evidence-only slice.
- 2026-09-10 07:25 UTC - Copilot - Added the exact C1/C9 URL-pattern exclusion to `.github/lychee-online.toml`. A two-link Lychee boundary test excluded a pull-request review-comment anchor while retaining a GitHub issue-comment anchor as a visible error. Hosted rerun evidence remains pending.
- 2026-09-11 08:27 UTC - Copilot - After PR #2197 merged, [run 34577938048](https://github.com/torrust/torrust-tracker/actions/runs/34577938048) was cancelled while its Lychee step was still in progress. Its upload step retained an empty, unusable report artifact, so it provides no usable Lychee output. Replacement [run 34578523069](https://github.com/torrust/torrust-tracker/actions/runs/34578523069) completed on merged revision `427b0c93`: it visibly failed with 52 remaining errors, excluded 476 links, uploaded a 1,743-byte report artifact, and contained no matching C1/C9 `#discussion_r` URLs. Unrelated `404`, `403`, loopback, issue-comment, other-fragment, and rate-limit failures remained visible.
- 2026-09-11 08:32 UTC - Task Reviewer - Independently reviewed the replacement hosted run and report artifact. The evidence supports T3/T4, AC3-AC5, and M4; the issue remains open for the remaining C2-C8 work. See `agent-review-reports.md`.
- 2026-09-11 09:25 UTC - Copilot - Added online-only `exclude_loopback = true` for C2. With explicit online configuration, a three-link boundary test excluded `127.0.0.1` and `localhost` while checking `https://www.rust-lang.org/` successfully. Hosted verification remains pending.
- 2026-09-11 15:38 UTC - Copilot - After PR #2202 merged, [run 34616458439](https://github.com/torrust/torrust-tracker/actions/runs/34616458439) completed on merged revision `f6df96bf`: it visibly failed with 44 remaining errors, excluded 495 links, uploaded a retained 1,533-byte report artifact, and contained no loopback (`localhost` or `127.0.0.1`) or C1/C9 `#discussion_r` URLs. Unrelated `404`, `403`, FSF transport, issue-comment, pull-request-review, and other missing-fragment failures remained visible.

## Acceptance Criteria

- [x] AC1: An issue-local baseline records the exact hosted run, revision, summary counts, and a disposition for every distinct failing URL or recurring failure pattern.
- [ ] AC2: Each repair changes only a verified stale reference and records why its replacement target is correct.
- [x] AC3: Each added exclusion is online-only, narrowly scoped to a documented durable false-positive category, and does not suppress unrelated external-link failures.
- [x] AC4: The advisory workflow remains scheduled/manual, visibly fails for remaining external-link failures, and continues to upload its Markdown report on failure.
- [x] AC5: At least one hosted rerun after each remediation slice records the resulting counts and explains material differences from the prior run.
- [ ] AC6: Documentation explains any permanent exclusion rationale and preserves the existing rerun-then-repair-or-narrow-exclusion triage policy.
- [ ] `linter all` exits with code `0`.
- [ ] Relevant tests pass or their documented non-applicability is reviewed.
- [ ] Manual verification scenarios are executed and documented (status + evidence).
- [ ] Acceptance criteria are re-reviewed after implementation and reflect actual behavior.
- [ ] Documentation is updated when behavior or workflow changes.

## Verification Plan

### Automatic Checks

- `linter all`
- `git diff --check`
- Relevant configuration/workflow validation for every changed YAML or TOML file
- Pre-push checks when a branch is prepared for review

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID  | Scenario                       | Command/Steps                                                                                                 | Expected Result                                                                                                                                 | Status | Evidence                                                                                                                                                                                                                                                                                                                                                             |
| --- | ------------------------------ | ------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------- | ------ | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| M1  | Reproduce baseline             | Manually dispatch `external-link-check.yaml` on `develop`; download `lychee-external-link-report`.            | Report is available even when Lychee fails; baseline counts and categories can be reviewed.                                                     | DONE   | [Run 34347690674](https://github.com/torrust/torrust-tracker/actions/runs/34347690674), revision `7abc30b2`, 461 errors, 0 timeouts.                                                                                                                                                                                                                                 |
| M2  | Classify durable failures      | Review all report entries and group by URL/pattern, response type, owning document, and proposed disposition. | Every baseline failure has a traceable disposition; no broad host-level suppression is proposed.                                                | DONE   | `external-link-baseline.md`; independent reconciliation passed on 2026-09-10.                                                                                                                                                                                                                                                                                        |
| M3  | Verify reference repairs       | Check each changed target using the appropriate authoritative source, then run local validation.              | Replacement reference is correct and offline local-link validation remains clean.                                                               | TODO   | Focused commands and review evidence.                                                                                                                                                                                                                                                                                                                                |
| M4  | Verify exclusion boundaries    | Dispatch the hosted workflow after adding a proposed exclusion.                                               | The intended durable false-positive category is absent, while representative unrelated external failures remain visible and the report uploads. | DONE   | C1/C9: [run 34578523069](https://github.com/torrust/torrust-tracker/actions/runs/34578523069) on `427b0c93` excluded 476 links and retained its report. C2: [run 34616458439](https://github.com/torrust/torrust-tracker/actions/runs/34616458439) on `f6df96bf` excluded 495 links, left 44 errors, retained a 1,533-byte artifact, and contained no loopback URLs. |
| M5  | Distinguish transient failures | Re-run a newly observed timeout, 403, or other potentially transient result once.                             | The record distinguishes a persistent failure from a transient response before an exclusion or repair decision.                                 | TODO   | Pair of hosted-run URLs and comparison.                                                                                                                                                                                                                                                                                                                              |

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence                                                           |
| ----- | ---------------------- | ------------------------------------------------------------------ |
| AC1   | DONE                   | `external-link-baseline.md` from run 34347690674.                  |
| AC2   | TODO                   | Reviewed reference-repair commits and target evidence.             |
| AC3   | DONE                   | C1/C9 and C2 hosted runs show each exact exclusion remains narrow. |
| AC4   | DONE                   | Run 34578523069 failed visibly and uploaded its report artifact.   |
| AC5   | DONE                   | Runs 34578523069 and 34616458439 record each exclusion slice.      |
| AC6   | TODO                   | Updated documentation and reviewer confirmation.                   |

## Risks and Trade-offs

- **Excessive suppression:** Excluding an entire host or fragment class could hide genuine link rot. Mitigation: classify first, use the narrowest matching rule, and prove remaining representative failures still appear in a hosted rerun.
- **Network variance:** `403`, rate limits, and temporary outages can make counts fluctuate. Mitigation: rerun newly observed potentially transient failures once before deciding they are permanent.
- **Unbounded historical repair:** Hundreds of review artifact links can obscure higher-value stale reference repairs. Mitigation: prioritize by durable category and owner, record a deferred disposition where a separate issue is warranted.
- **Premature architecture selection:** Changes to shared runner behavior or broad enforcement would exceed this task and EPIC #2003's current decision stage. Mitigation: retain the existing workflow and propose a separate design action if a wider change is required.

## Implementation Completion Review

After implementation, compare the result with this specification. Record invalidated assumptions, material design changes, unexpected validation findings, and reusable lessons.

- Retrospective: `Not yet assessed`
- If needed, create `implementation-retrospective.md` from the repository template at `docs/templates/IMPLEMENTATION-RETROSPECTIVE.md` in this issue specification's directory.
- If no retrospective is needed, add a concise progress-log entry explaining why the work had no material discovery.
- When an independent reviewer receives this folder-style specification, it records its result in `agent-review-reports.md` using `docs/templates/AGENT-REVIEW-REPORTS.md`.

## References

- Parent EPIC: [#2003](https://github.com/torrust/torrust-tracker/issues/2003)
- Enabling issue: [#2162](https://github.com/torrust/torrust-tracker/issues/2162)
- Current baseline run: [34347690674](https://github.com/torrust/torrust-tracker/actions/runs/34347690674)
- External workflow: [`.github/workflows/external-link-check.yaml`](../../../../.github/workflows/external-link-check.yaml)
- Online configuration: [`.github/lychee-online.toml`](../../../../.github/lychee-online.toml)
- Triage procedure: [`docs/testing.md`](../../../testing.md)
