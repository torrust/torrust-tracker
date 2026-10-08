---
schema-version: 1
doc-type: issue
issue-type: bug
status: planned
priority: p2
epic: null
github-issue: 2499
spec-path: docs/issues/open/2499-stabilize-trivy-code-scanning-alert-identity/ISSUE.md
branch: "2499-stabilize-trivy-code-scanning-alert-identity"
related-pr: null
last-updated-utc: "2026-10-08 17:52"
semantic-links:
  skill-links:
    - create-issue
    - fix-bug
    - implement-workflow
    - triage-github-security-findings
  related-artifacts:
    - .github/workflows/security-scan.yaml
    - docs/security/analysis/github-security/README.md
    - docs/security/analysis/github-security/reviews/2026-10-08.md
    - docs/issues/open/2499-stabilize-trivy-code-scanning-alert-identity/manual-verification-evidence.md
    - "issue #2493"
---

<!-- skill-link: create-issue -->

# Issue #2499 - Stabilize Trivy Code Scanning alert identity

## Goal

Make every Trivy image finding keep one GitHub Code Scanning alert regardless of whether the
`develop` scan was triggered by the daily schedule or by a push, so alert state reflects the
vulnerability rather than the trigger that produced the scan.

## Background

The `Security Scan` workflow scans the production image with Trivy and uploads SARIF to Code
Scanning under one analysis category. The image it scans depends on the trigger:

1. A scheduled run pulls `torrust/tracker:develop`.
2. A push or pull-request run builds the image locally as `torrust-tracker:local`.
3. Trivy records the image name as the alert location path: the scheduled image becomes
   `torrust/tracker`, and the local image becomes `library/torrust-tracker`.
4. Code Scanning treats a different location as a different alert. When a push run uploads to
   `develop`, every alert located at `torrust/tracker` is absent from the new analysis, so GitHub
   marks it `fixed`, and the matching `library/torrust-tracker` alert opens or reopens.
5. The next scheduled run reverses the change.

On 2026-10-08, the push for `554c47f67` touched `security-scan.yaml`. Its upload, processed at
16:55:43Z, marked 31 `torrust/tracker` alerts `fixed` and reopened 31 `library/torrust-tracker`
alerts with the same CVE ids and packages. No package changed. The two series pair one-to-one by
CVE id and package.

This violates the expectation that a `fixed` alert means the finding is no longer present. The
impact today:

- the Security tab shows a vulnerability as fixed when it is still in the image;
- every alert number cited by a review inventory, CVE record, or issue becomes stale at the next
  trigger change, as PR #2498 review finding `review-finding:pr-2498-f10` found;
- closure criteria in follow-up issues cannot be stated as one alert number.

This is not an exploitable vulnerability. It is a CI defect in the security evidence pipeline for
Tier 1 runtime findings.

## Scope

### In Scope

- Give scheduled and push-triggered scans one stable alert identity for each CVE and package.
- Decide how the currently duplicated alert series is reconciled after the fix.
- Update the GitHub Security catalog and the triage skill once alert numbers are stable.

### Out of Scope

- Fixing or reclassifying any CVE reported by Trivy.
- Changing the scanned image contents, the Trivy severity filter, or the scan schedule.
- Workflow-permission hardening tracked by #2495 and #2496.

## Architectural Decisions

- Related ADRs: None known.
- ADRs to create: None expected.

## Design and Ownership Review

Not applicable: no child processes, asynchronous I/O, network readiness, resource cleanup, or
reusable test fixtures are involved. Follow the `implement-workflow` skill for the workflow change.

Candidate approaches to evaluate in T2:

- scan one stable image reference in both paths, for example by tagging the locally built image
  with the published name before scanning;
- set the SARIF location explicitly so both paths produce the same path.

Do not split the paths into separate SARIF categories: both series would then stay open together,
duplicating every finding.

## Bug-Fix Process

Follow [`fix-bug`](../../../../.github/skills/dev/debugging/fix-bug/SKILL.md):

1. Analysis: the location path follows the image name, and the image name depends on the trigger.
2. Reproduction: reproduced from GitHub Code Scanning state; see V1 in
   [`manual-verification-evidence.md`](manual-verification-evidence.md).
3. Regression boundary: see Regression Test Strategy.
4. Red evidence: the recorded paired alert series before the fix.
5. Fix: make the image identity independent of the trigger.
6. Green evidence and recheck: run both trigger types after merge and confirm no alert changes
   state or number.

## Regression Test Strategy

Alert identity is assigned by GitHub only after SARIF upload, so no repository test can observe it
directly. The maintained guard is structural: one workflow value defines the scanned image
reference for every trigger, so the paths cannot diverge again. The red/green evidence is the
like-for-like Code Scanning state after a scheduled run and a push run on `develop`, recorded in
`manual-verification-evidence.md`.

## Implementation Plan

| ID | Status | Task | Notes / Expected Output |
| --- | --- | --- | --- |
| T1 | DONE | Reproduce the alert flip | V1 records the 31 paired alerts and the run that flipped them. |
| T2 | TODO | Select the stable-identity approach | Compare the candidates with a manual run and record the SARIF location path each produces. |
| T3 | TODO | Fix the workflow | One image reference serves every trigger. |
| T4 | TODO | Recheck both trigger types | After merge, a scheduled run and a push run leave every Trivy alert number and state unchanged. |
| T5 | TODO | Reconcile the duplicate series | Record which series remains and why the other stays `fixed`. |
| T6 | TODO | Update security records | Replace the paired series in the catalog with stable numbers; drop the pairing guidance from the triage skill. |

## Commit Points

| Task | Coherent change set | Commit policy |
| --- | --- | --- |
| T2-T3 | Stable image identity in `security-scan.yaml` | Commit after workflow validation. |
| T4-T5 | Recheck and reconciliation evidence | Commit after both post-merge runs. |
| T6 | Catalog and skill updates | Commit separately from the workflow change. |

## Progress Tracking

### Workflow Checkpoints

- [x] Draft specification created
- [x] Bug reproduced and evidence recorded
- [x] Draft reviewed and approved by maintainer
- [x] GitHub issue created and issue number added
- [ ] Workflow fixed
- [ ] Automatic verification completed (`linter all`, workflow validation)
- [ ] Manual verification scenarios executed and recorded in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] Evidence-based implementation completion review recorded
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-10-08 17:49 UTC - Copilot - Drafted from PR #2498 review finding
  `review-finding:pr-2498-f10`; reproduced the alert flip from Code Scanning state (V1).
- 2026-10-08 17:52 UTC - Maintainer - Approved the draft; Copilot created GitHub issue #2499 and
  promoted the specification to the open issue catalog.

## Acceptance Criteria

- [ ] AC1: Scheduled and push-triggered scans on `develop` produce the same location path for
  every Trivy finding.
- [ ] AC2: After a scheduled run followed by a push run, no Trivy alert changes state or number
  without a change in the scanned packages.
- [ ] AC3: The duplicate alert series is reconciled and the reconciliation is recorded.
- [ ] AC4: The GitHub Security catalog cites one stable alert number per Trivy finding.
- [ ] `linter all` exits with code `0`.
- [ ] Manual verification scenarios are executed and documented in issue-local
  `manual-verification-evidence.md`.
- [ ] Acceptance criteria are re-reviewed after implementation and reflect actual behavior.

## Verification Plan

### Automatic Checks

- `linter all`
- Workflow validation required by `implement-workflow`

### Manual Verification Scenarios

| ID | Scenario | Human-oriented command/steps | Expected Result | Status | Evidence |
| --- | --- | --- | --- | --- | --- |
| M1 | Reproduce the flip | Pair `fixed` and open Trivy alerts by CVE id and package after a push run. | Two series exist; the push run fixed one and reopened the other. | DONE | `manual-verification-evidence.md` section V1 |
| M2 | Recheck after the fix | Query open and fixed Trivy alerts after a scheduled run and after a push run. | One series; no alert changes number or state. | TODO | `manual-verification-evidence.md` section V2 |

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| --- | --- | --- |
| AC1 | TODO | M2 |
| AC2 | TODO | M2 |
| AC3 | TODO | T5 record |
| AC4 | TODO | Catalog diff |

## Risks and Trade-offs

- Tagging a local build with the published name could be confused with the published image.
  Mitigation: keep the tag local to the runner and never push it.
- The surviving series loses the alert history of the other series. Mitigation: record the pairing
  in the reconciliation evidence.

## Implementation Completion Review

- Retrospective: `Not yet assessed`
- Create `implementation-retrospective.md` only if the fix changes how scanner evidence is
  recorded beyond this workflow; otherwise record why none was needed in the progress log.

## References

- Related issues: #2493, #2497
- Related PRs: #2498
- Related ADRs: None
