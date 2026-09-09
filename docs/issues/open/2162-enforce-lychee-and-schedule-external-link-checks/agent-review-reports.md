---
semantic-links:
  related-artifacts:
    - docs/issues/open/2162-enforce-lychee-and-schedule-external-link-checks/ISSUE.md
    - .github/workflows/external-link-check.yaml
    - .github/lychee-online.toml
---

# Agent Review Reports - Issue #2162

> Append one completed independent-review entry at a time. Do not modify, reorder, or remove
> earlier entries. A correction is a new entry that names the earlier conclusion.

## Reports

### 2026-09-08 12:53 UTC - Task Reviewer

- Invocation scope: Pre-push, read-only review of the uncommitted implementation for issue #2162.
- Inputs: `ISSUE.md`; all tracked and untracked implementation diff files; repository workflow-action policy; official Lychee GitHub Actions and configuration documentation; published `torrust-linting` 0.2.0 metadata; and local focused checks.
- Evidence: `linter lychee` passed with installed `linter 0.2.0`; the direct Lychee command accepted `.github/lychee-online.toml` and completed the declared inputs offline; `linter yaml` and `git diff --check` passed. The workflow has a Monday 06:00 UTC schedule and `workflow_dispatch`, no `push` or `pull_request` trigger, a 30-minute timeout, read-only contents permission, `GITHUB_TOKEN` only in the Lychee step environment, bounded request settings, and `if: always()` report upload with 14-day retention. `cargo info torrust-linting@0.2.0` confirmed the released crates.io package and metadata.
- Findings:
  - PENDING: No GitHub Actions manual-dispatch run or rerun evidence exists for M2/M3, so the online execution, failure report artifact, and triage process have not been observed on GitHub-hosted runners.
  - PENDING: The branch-protection configuration was not available for review. The workflow's lack of PR triggers proves it cannot produce a PR-event check, but does not independently prove it is absent from required-status policy.
  - WARN: The repository action-maintenance policy requires an organization allowlist check before PR creation. The direct CLI design avoids adding `lycheeverse/lychee-action`, and `actions/checkout@v7` and `actions/upload-artifact@v7` are already used elsewhere, but organization policy evidence is absent.
  - WARN: The issue completion-review section still says `Not yet assessed`, although its progress log contains a concise no-retrospective rationale. Reconcile the section after the pending workflow evidence is obtained.
- Verdict: REVIEW FAILED
- Follow-up actions:
  - Dispatch the External Link Check workflow from this branch, retain the run URL and artifact URL in M2, then rerun a failed or simulated-failure case and record both URLs/report evidence in M3.
  - Confirm branch-protection does not require this scheduled workflow, and record the evidence.
  - Before opening a PR, have an organization administrator confirm the configured allowed-actions policy permits the exact action references.
  - Update the Implementation Completion Review section and acceptance/progress checkboxes only after the required evidence is verified.

### 2026-09-08 16:35 UTC - Follow-up to Task Reviewer report

- Correction: The pending branch-protection finding is resolved. The legacy GitHub REST branch-protection endpoint returned `404`; this does not by itself establish why. Authenticated `gh` API calls using a token with `repo` scope checked `/repos/torrust/torrust-tracker/rules/branches/develop` and `/repos/torrust/torrust-tracker/rulesets`. The effective rules for `develop` require creation, pull requests, deletion, and signatures but contain no required-status-check rule. The repository ruleset named `(branches) [ * ; ! dependabot, ! staging ] Deny Force Push; Require Status Checks` is disabled and does not name `External Link Check`. This workflow is therefore not a merge requirement.
- Hosted evidence: [run 34250154466](https://github.com/torrust/torrust-tracker/actions/runs/34250154466) failed at the online Lychee step and uploaded its report artifact; [rerun 34251836337](https://github.com/torrust/torrust-tracker/actions/runs/34251836337) did the same. The reports contain 442 and 445 errors, respectively, so the observed condition persisted across the required single rerun.
- Tracking correction: `ISSUE.md` now records M2/M3 evidence, marks the supported acceptance criteria complete, and documents why no separate implementation retrospective is needed. Both hosted runs scanned pre-merge revision `e4db63d5`; they validate the deployed workflow behavior, while the merged workflow has the same command and configuration.

### 2026-09-08 17:12 UTC - Task Reviewer completion review

- Scope: Independent read-only review of merged commit `8dba10b4` and the updated hosted-validation evidence for issue #2162.
- Evidence: `torrust-linting` 0.2.0 supplies local offline checking through `linter all`; the hosted workflow has the Monday schedule and manual dispatch trigger, no pull-request or push trigger, secure bounded online requests, and `if: always()` Markdown report upload. [Run 34250154466](https://github.com/torrust/torrust-tracker/actions/runs/34250154466) and [rerun 34251836337](https://github.com/torrust/torrust-tracker/actions/runs/34251836337) visibly failed at online Lychee and each uploaded an unexpired readable report. Effective `develop` rules contain no required-status-check rule.
- Conclusion: All acceptance criteria and M1-M3 scenarios pass. The records correctly identify that the hosted runs scanned pre-merge revision `e4db63d5`, while the merged workflow preserves the same command and configuration. The no-retrospective rationale is adequate.
- Residual risk: The advisory report contains hundreds of existing external-link failures and will remain noisy until individually triaged under the documented rerun-and-repair-or-narrow-exclusion policy.
- Verdict: REVIEW PASSED. The reviewer-validation checkpoint may be marked complete; retain the issue in its current open location until its separate closure workflow is intentionally performed.
