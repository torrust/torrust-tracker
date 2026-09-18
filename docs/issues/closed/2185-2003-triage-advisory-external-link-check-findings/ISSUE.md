---
doc-type: issue
issue-type: task
status: done
priority: p3
epic: 2003
github-issue: 2185
spec-path: docs/issues/closed/2185-2003-triage-advisory-external-link-check-findings/ISSUE.md
branch: "2185-2003-triage-advisory-external-link-check-findings"
related-pr: 2258
last-updated-utc: 2026-09-18 12:50
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - .github/skills/dev/planning/create-issue/SKILL.md
    - docs/issues/open/2003-overhaul-guardrails-and-automation/EPIC.md
    - docs/issues/closed/2162-enforce-lychee-and-schedule-external-link-checks/ISSUE.md
    - docs/issues/closed/2185-2003-triage-advisory-external-link-check-findings/external-link-baseline.md
    - docs/issues/closed/2185-2003-triage-advisory-external-link-check-findings/agent-review-reports.md
    - issue #2264
    - docs/issues/open/2264-2003-refactor-semantic-link-conventions/external-link-check-residual-failures-2026-09-18.md
    - .github/workflows/external-link-check.yaml
    - .github/lychee-online.toml
    - docs/testing.md
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

| ID  | Status      | Task                                      | Notes / Expected Output                                                                                          |
| --- | ----------- | ----------------------------------------- | ---------------------------------------------------------------------------------------------------------------- |
| T1  | DONE        | Preserve and classify the baseline        | `external-link-baseline.md` maps all 461 report errors to nine recurring categories and dispositions.            |
| T2  | DONE        | Repair clearly stale references           | C3-C5 and the C6 Docker repairs are hosted-verified. C7-C8 were rerun and probed: none is a stale reference, so no repair applies. |
| T3  | DONE        | Add justified narrow exclusions           | C1/C9, C2, C6, and C10 exact online-only rules are hosted-verified.                                              |
| T4  | DONE        | Revalidate hosted signal                  | Hosted C1/C9, C2, C6, and C10 runs retain unrelated failures and their report artifacts.                         |
| T5  | DONE        | Document operations and review completion | `docs/testing.md` keeps the rerun-then-repair-or-narrow-exclusion policy; residual cases and insights are handed to the semantic-link EPIC draft. |

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
- [x] Implementation completed
- [x] Automatic verification completed (`linter all`, relevant tests, and any pre-push checks)
- [x] Manual verification scenarios executed and recorded (status + evidence)
- [x] Acceptance criteria reviewed after implementation and updated with evidence
- [x] Evidence-based implementation completion review recorded: issue-local retrospective created for material discoveries, or progress log states why none was needed
- [x] Reviewer validated acceptance criteria and updated checkboxes
- [x] Independent reviewer reports recorded in issue-local `agent-review-reports.md` when reviewers received this folder-style specification
- [x] Committer verified spec progress is up to date before commit
- [x] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

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
- 2026-09-11 16:45 UTC - Copilot - Verified the 14 C3 package manifests each inherit `documentation = https://docs.rs/crate/torrust-tracker/`; that target returned HTTP 200 and redirects to `latest`. Replaced only the 14 unavailable package-specific docs.rs links with the shared tracker documentation target. Hosted workflow verification remains pending.
- 2026-09-14 09:51 UTC - Copilot - After PR #2208 merged, [run 34829466145](https://github.com/torrust/torrust-tracker/actions/runs/34829466145) completed on merged revision `952911af`: it visibly failed with 31 unrelated errors, retained a 1,357-byte report artifact through 2026-09-28, and contained none of C3's 14 replaced package-specific docs.rs URLs. The lower count than C2's 44 is not directly comparable because PR #2207 archived issue specifications from the checked document set.
- 2026-09-14 12:00 UTC - Copilot - Repaired C4's three repository-controlled `404` links: the IPv6 ADR now references the research file's archived location, and the integration-test coverage draft now references the current `tests/scaffold.rs` global-stats example and `tests/metrics/` targets. Git and GitHub verified each replacement exists on `develop`; focused Markdown, spelling, and local-link checks passed. Hosted workflow verification remains pending.
- 2026-09-14 12:51 UTC - Copilot - After PR #2212 merged, [run 34843399874](https://github.com/torrust/torrust-tracker/actions/runs/34843399874) completed on merged revision `618723d4`: it visibly failed with 28 unrelated errors, retained a 1,191-byte report artifact through 2026-09-28, and contained none of C4's three replaced repository-controlled URLs. The count is not directly comparable to C3's 31 errors because intervening merged changes modified the checked document set.
- 2026-09-15 07:06 UTC - Copilot - Replaced C5's retired Caddy HTTP/3 documentation URL with Caddy's current server-options reference. The authoritative target returned HTTP 200 and documents the `protocols` option, including HTTP/3 as `h3`; the standalone online Lychee probe reached the target but reported unrelated missing fragments and cached failures within Caddy's documentation. Focused Markdown, spelling, and local-link checks passed; hosted workflow verification remains pending.
- 2026-09-15 10:12 UTC - Copilot - Hosted [run 34953081017](https://github.com/torrust/torrust-tracker/actions/runs/34953081017) on merged revision `a1ddcaa0` visibly failed with 38 errors and 2 timeouts, uploaded a 1,342-byte report artifact through 2026-09-29, and contained no occurrence of C5's retired URL. However, it reported the replacement `#servers` fragment as missing, so C5 is not hosted-verified. Replaced the fragment with the same authoritative Caddy server-options page without a fragment, which returned HTTP 200; focused validation and a new hosted run remain pending.
- 2026-09-15 12:52 UTC - Copilot - After PR #2225 merged, [run 34971438822](https://github.com/torrust/torrust-tracker/actions/runs/34971438822) ran on merged revision `bbb58fa8`: it visibly failed with 37 unrelated errors, no timeouts, and a retained `lychee-external-link-report` artifact. The report contains neither C5's retired URL nor the current Caddy options URL without a fragment as an error, while Docker missing fragments, third-party `403` responses, FSF transport errors, GitHub comment/review fragments, and the Star History fragment remain visible. This hosted-verifies C5 without adding an exclusion.
- 2026-09-15 15:00 UTC - Copilot - Replaced C6's two retired Docker Cloud ACI fragment links. Both Docker page URLs now redirect to Docker's retired-page notice. Azure's current Azure Files documentation describes mount-path content obscuring, replacing the obsolete single-file/subfolder assertion; Azure's troubleshooting documentation confirms that ACI does not support Docker-style port mapping. Focused validation and hosted workflow verification remain pending; the GitHub-comment and Star History fragments remain separate C6 investigations.
- 2026-09-16 17:33 UTC - Copilot - After PR #2231 merged, [run 35128890381](https://github.com/torrust/torrust-tracker/actions/runs/35128890381) ran on merged revision `6e1e9d29`: it visibly failed with 33 errors and 4 timeouts, uploaded a 1,229-byte `lychee-external-link-report` artifact through 2026-09-30, and contained no errors for either retired Docker ACI URL. Unrelated third-party `403`, FSF, GitHub comment/review fragment, Star History fragment, and timeout failures remained visible. This hosted-verifies the C6 Docker repair without adding an exclusion.
- 2026-09-17 12:00 UTC - Copilot - Verified that the two remaining GitHub issue-comment fragments resolve to their intended comments through GitHub's issue-comment API, while the Star History fragment selects the repository through client-side routing and its page-level URL does not preserve that view. Added three exact online-only exclusions; hosted boundary verification remains pending.
- 2026-09-17 13:13 UTC - Copilot - After PR #2251 merged, [run 35224905794](https://github.com/torrust/torrust-tracker/actions/runs/35224905794) ran on merged revision `3bad98d1`: it visibly failed with 30 errors and no timeouts, uploaded a retained 1,039-byte `lychee-external-link-report` artifact through 2026-10-01, and contained none of the two C6 GitHub issue-comment anchors or the Star History project selector. Unrelated GitHub fragments, third-party `403` responses, and FSF transport failures remained visible. This hosted-verifies the C6 dynamic-fragment exclusions.
- 2026-09-17 15:18 UTC - Copilot - Rerun-first check for C7-C8: [run 35238419294](https://github.com/torrust/torrust-tracker/actions/runs/35238419294) on merged revision `37c0bea5` failed visibly with 30 errors and 2 timeouts and retained its report. Every C7 `403` and C8 FSF failure persisted from run 35224905794, so neither category is transient; the two `martinfowler.com` timeouts were new and are the transient case the policy expects. Direct probes established the causes: Medium returns `403` to a browser user agent as well; the Stack Overflow short permalink redirects to the full question URL, which also returns `403` while the answer still exists; `www.fsf.org` returns HTTP 200 through `curl` and `openssl` but offers only finite-field `DHE` cipher suites, which rustls-based Lychee cannot negotiate. None of C7-C8 is a stale reference. The remaining 23 GitHub errors were `#issuecomment-<id>` and `#pullrequestreview-<id>` anchors on pull-request URLs in `docs/pr-reviews/`, a dynamic-anchor class (C10) added to the checked set after the baseline.
- 2026-09-17 16:44 UTC - Copilot - Added two exact online-only C10 patterns for pull-request `#issuecomment-<id>` and `#pullrequestreview-<id>` anchors to `.github/lychee-online.toml`. A five-link boundary test excluded the three dynamic pull-request anchor forms and the exact C6 issue-comment anchor while retaining `https://github.com/torrust/torrust-tracker/pull/123/files`. Merged as [PR #2255](https://github.com/torrust/torrust-tracker/pull/2255).
- 2026-09-18 06:43 UTC - Copilot - After PR #2255 merged, [run 35315382956](https://github.com/torrust/torrust-tracker/actions/runs/35315382956) ran on merged revision `e6dd8918`: it visibly failed with 7 errors and 4 timeouts, excluded 886 links, and retained a 782-byte `lychee-external-link-report` artifact through 2026-10-02. The report contains no GitHub fragment error of any kind. The residual failures are two Medium `403` responses, one Stack Overflow `403`, four FSF transport failures, and four first-time `https://www.gnu.org/licenses/` timeouts. This hosted-verifies C10 and confirms C7-C8 as persistent, non-stale cases.
- 2026-09-18 07:10 UTC - Copilot - Closure decision with the maintainer: the residual C7-C8 cases and the new GNU timeouts are license boilerplate or background reading that cannot be repaired, and every candidate treatment (replacing citations, excluding hosts, relaxing TLS) is a policy choice outside this task's scope. The verbatim closing report is preserved in `docs/issues/open/2264-2003-refactor-semantic-link-conventions/external-link-check-residual-failures-2026-09-18.md`, and the triage insights are recorded in EPIC #2264's "Handoff from Issue #2185" section. No issue-local retrospective is needed because the EPIC handoff is the retrospective. The issue closes when this PR merges.
- 2026-09-18 09:05 UTC - Copilot - PR [#2258](https://github.com/torrust/torrust-tracker/pull/2258) merged into `develop` and closed GitHub issue [#2185](https://github.com/torrust/torrust-tracker/issues/2185) as completed. Moved this issue specification from `docs/issues/open/` to `docs/issues/closed/`.

## Acceptance Criteria

- [x] AC1: An issue-local baseline records the exact hosted run, revision, summary counts, and a disposition for every distinct failing URL or recurring failure pattern.
- [x] AC2: Each repair changes only a verified stale reference and records why its replacement target is correct. C3, C4, C5, and C6 Docker repairs are target-verified and hosted-verified; C7-C8 were probed and are not stale, so no further repair applies.
- [x] AC3: Each added exclusion is online-only, narrowly scoped to a documented durable false-positive category, and does not suppress unrelated external-link failures.
- [x] AC4: The advisory workflow remains scheduled/manual, visibly fails for remaining external-link failures, and continues to upload its Markdown report on failure.
- [x] AC5: At least one hosted rerun after each remediation slice records the resulting counts and explains material differences from the prior run.
- [x] AC6: Documentation explains any permanent exclusion rationale and preserves the existing rerun-then-repair-or-narrow-exclusion triage policy. Each exclusion group in `.github/lychee-online.toml` carries its rationale comment, and `docs/testing.md` keeps the rerun-first policy unchanged.
- [x] `linter all` exits with code `0`.
- [x] Relevant tests pass or their documented non-applicability is reviewed. The changes are Markdown and Lychee configuration only; no Rust test applies.
- [x] Manual verification scenarios are executed and documented (status + evidence).
- [x] Acceptance criteria are re-reviewed after implementation and reflect actual behavior.
- [x] Documentation is updated when behavior or workflow changes.

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
| M3  | Verify reference repairs       | Check each changed target using the appropriate authoritative source, then run local validation.              | Replacement reference is correct and offline local-link validation remains clean.                                                               | DONE   | C3: Cargo metadata and docs.rs confirmed the shared target; [run 34829466145](https://github.com/torrust/torrust-tracker/actions/runs/34829466145) contains none of the 14 replaced URLs. C4: [run 34843399874](https://github.com/torrust/torrust-tracker/actions/runs/34843399874) contains none of the three replaced URLs. C5: [run 34971438822](https://github.com/torrust/torrust-tracker/actions/runs/34971438822) contains neither the retired URL nor the replacement without a fragment as an error. C6 Docker: [run 35128890381](https://github.com/torrust/torrust-tracker/actions/runs/35128890381) contains neither retired Docker ACI URL as an error. All completed slices retain unrelated errors. C7-C8 probes found no stale reference to repair. |
| M4  | Verify exclusion boundaries    | Dispatch the hosted workflow after adding a proposed exclusion.                                               | The intended durable false-positive category is absent, while representative unrelated external failures remain visible and the report uploads. | DONE | C1/C9: [run 34578523069](https://github.com/torrust/torrust-tracker/actions/runs/34578523069) on `427b0c93` excluded 476 links and retained its report. C2: [run 34616458439](https://github.com/torrust/torrust-tracker/actions/runs/34616458439) on `f6df96bf` excluded 495 links, left 44 errors, retained a 1,533-byte artifact, and contained no loopback URLs. C6: [run 35224905794](https://github.com/torrust/torrust-tracker/actions/runs/35224905794) on `3bad98d1` excluded 751 links, retained 30 unrelated errors, and uploaded a 1,039-byte artifact. C10: [run 35315382956](https://github.com/torrust/torrust-tracker/actions/runs/35315382956) on `e6dd8918` excluded 886 links, left no GitHub fragment error, retained 7 unrelated errors and 4 timeouts, and uploaded a 782-byte artifact. |
| M5  | Distinguish transient failures | Re-run a newly observed timeout, 403, or other potentially transient result once.                             | The record distinguishes a persistent failure from a transient response before an exclusion or repair decision.                                 | DONE   | Persistent: every C7 `403` and C8 FSF failure appears in [run 35224905794](https://github.com/torrust/torrust-tracker/actions/runs/35224905794), [run 35238419294](https://github.com/torrust/torrust-tracker/actions/runs/35238419294), and [run 35315382956](https://github.com/torrust/torrust-tracker/actions/runs/35315382956). Transient: the `martinfowler.com` timeouts in run 35238419294 are absent from run 35315382956. The GNU licenses timeouts first appear in run 35315382956 and are handed to the EPIC as a rerun-first candidate. |

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence                                                           |
| ----- | ---------------------- | ------------------------------------------------------------------ |
| AC1   | DONE                   | `external-link-baseline.md` from run 34347690674.                  |
| AC2   | DONE                   | C3-C5 and C6 Docker target and hosted verification are recorded; C7-C8 probes show no stale reference remains. |
| AC3   | DONE                   | C1/C9, C2, C6, and C10 hosted runs show each exact exclusion remains narrow. |
| AC4   | DONE                   | Run 34578523069 failed visibly and uploaded its report artifact.   |
| AC5   | DONE                   | Runs 34578523069, 34616458439, 34829466145, 34843399874, 34971438822, 35128890381, 35224905794, 35238419294, and 35315382956 record each remediation slice and the C7-C8 rerun. |
| AC6   | DONE                   | `.github/lychee-online.toml` comments state each exclusion rationale; `docs/testing.md` keeps the rerun-first policy; residual cases are documented in the semantic-link EPIC draft. |

## Closure Decision

The task scope was to clean broken links. The triage showed that most reported failures were not
broken links, that the genuinely stale references were few and are repaired, and that the residual
failures depend on a policy the repository does not have yet: how much a link matters, who owns its
target, and what the checker can physically reach. Deciding that here would exceed this task and
EPIC #2003's current decision stage.

The issue therefore closes with:

- all repository-owned stale references repaired and hosted-verified;
- exact, commented, online-only exclusions in `.github/lychee-online.toml` for dynamic GitHub
  anchors, loopback examples, and the Star History selector;
- the advisory workflow unchanged and still failing visibly on the residual cases;
- the closing report preserved in the semantic-link EPIC draft folder together with the insights
  from this triage, so the follow-up can start from evidence rather than re-deriving it.

## Risks and Trade-offs

- **Excessive suppression:** Excluding an entire host or fragment class could hide genuine link rot. Mitigation: classify first, use the narrowest matching rule, and prove remaining representative failures still appear in a hosted rerun.
- **Network variance:** `403`, rate limits, and temporary outages can make counts fluctuate. Mitigation: rerun newly observed potentially transient failures once before deciding they are permanent.
- **Unbounded historical repair:** Hundreds of review artifact links can obscure higher-value stale reference repairs. Mitigation: prioritize by durable category and owner, record a deferred disposition where a separate issue is warranted.
- **Premature architecture selection:** Changes to shared runner behavior or broad enforcement would exceed this task and EPIC #2003's current decision stage. Mitigation: retain the existing workflow and propose a separate design action if a wider change is required.

## Implementation Completion Review

After implementation, compare the result with this specification. Record invalidated assumptions, material design changes, unexpected validation findings, and reusable lessons.

- Retrospective: `The semantic-link EPIC handoff is the durable completion review for this task. Issue #2185 found no additional code or workflow behavior to change after the final hosted validation; its reusable lessons are the S13 policy questions and residual-failures artifact now recorded in the EPIC draft.`
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
