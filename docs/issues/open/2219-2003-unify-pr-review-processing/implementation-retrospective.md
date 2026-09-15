---
semantic-links:
  skill-links:
    - write-markdown-docs
  related-artifacts:
    - docs/issues/open/2219-2003-unify-pr-review-processing/ISSUE.md
    - docs/issues/open/2219-2003-unify-pr-review-processing/manual-verification-evidence.md
    - docs/issues/open/2219-2003-unify-pr-review-processing/agent-review-reports.md
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
    - docs/templates/PR-REVIEW-TEMPLATE.md
---

# Implementation Retrospective - Unify PR Review Processing

## Purpose

Record evidence-based process improvements discovered while implementing issue #2219. This is a
blameless review of the implementation approach; it does not replace acceptance-criteria
verification.

## Outcome

The repository now has a nightly formatting check in the local pre-commit gate, one GraphQL-first
pull-request review workflow, one audit location and template, advisory reviewer finding guidance,
compatibility redirects for the retired review skills, and portable deterministic references for
new review findings. Manual scenarios M1-M4, the full linter suite, focused structural checks, and
the mandatory pre-commit gate passed.

## What Went Well

1. Pinning the historical formatter violation and PR #2174 review fixture converted process claims
   into reproducible evidence.
2. Independent reviews found documentation-contract gaps before commit, and focused structural
   checks now protect the unified routing and citation rules.

## What Changed During Implementation

The local `linter` command could not select nightly rustfmt, so a green stable-formatting result did
not demonstrate CI parity. The pre-commit hook now runs the named nightly formatter step directly.

The legacy audit migration exposed that Markdown link checking does not validate YAML frontmatter
artifact paths. The migration therefore required a structural review in addition to Markdown and
link checks.

The first T6 migration left Copilot agent, prompt, and orchestration diagram routes that could be
read as a parallel workflow. The compatibility redirects, entry points, diagrams, and structural
contract test now all delegate to `process-pr-review` and the canonical pull-request audit.

T9 made the normalized finding a repository concept instead of a GitHub-only resource. The chosen
reference derives from the canonical audit filename and finding ID, allowing later ADRs and issue
specifications to cite the concern while retaining provider identifiers as source provenance.

## First Real Use: PR #2232 Review Rounds

PR #2232 exercised the unified workflow on its own pull request: one Copilot round (F1-F3, all
fixed and resolved) and one asynchronous human round (F4-F12, changes requested). The process held
up: free-prose findings normalized deterministically, each fix landed in its own signed commit,
and the reviewer independently verified the audit identifiers byte-for-byte. First use also
improved the audit format itself: the single 15-column findings table proved hard to read and was
split into a compact tracking table plus a Finding Details section (`review-finding:pr-2232-*`
records the full round).

The human round exposed four evidence-backed gaps, tracked for follow-up work:

1. Repository paths inside Markdown code spans are invisible to every existing gate; Lychee
   validates links only (root cause of `review-finding:pr-2232-f4` and
   `review-finding:pr-2232-f5`).
2. Retiring a document can silently drop the normative safeguards it carried; retirement needs an
   obligation inventory (`review-finding:pr-2232-f6`).
3. Bulk renames need mechanical purity verification: diff each renamed file against its
   merge-base original and assert that only the intended lines changed
   (`review-finding:pr-2232-f4`).
4. Test scripts must declare or avoid host dependencies so a missing interpreter module cannot
   masquerade as a content violation (`review-finding:pr-2232-f7`).

## Root Cause

The original implementation plan identified the major workflow differences but did not pin the
toolchain behavior, machine-check every metadata reference, or treat agent and prompt entry points
as part of the workflow contract. Those omitted boundaries allowed apparently complete local
documentation changes to retain conflicting behavior.

## Improvements for Future Work

1. Pin executable fixtures and their required toolchains whenever a process change claims CI or
   protocol parity.
2. Treat all invocation surfaces, including agents, prompts, diagrams, frontmatter, and helpers, as
   one workflow contract and protect them with focused structural checks.
3. Give recurring review findings a repository-controlled reference before using their audit data
   to motivate durable guardrails or architectural decisions.
4. Follow-up candidates from the PR #2232 rounds: a code-span path existence check, a document
   retirement obligation inventory, a rename-purity migration step, reviewer-side `review-pr`
   skill alignment (`review-finding:pr-2232-f12`), and tiered model routing where a
   high-capability model evaluates findings and a lower-cost model implements the bounded
   solutions under independent verification.

## Avoiding Overcorrection

This evidence does not justify making every documentation link a bespoke shell assertion or adding
a new workflow framework. Use structural checks for metadata and multi-artifact invariants that
existing linters cannot validate, while retaining normal Markdown, link, spell, and full-linter
checks for their established scope.

## Evidence

- Issue #2219: `docs/issues/open/2219-2003-unify-pr-review-processing/ISSUE.md`
- Manual scenarios: `manual-verification-evidence.md` sections V1-V4
- Independent review history: `agent-review-reports.md`
- First-real-use audit: `docs/pr-reviews/pr-2232-review.md` (findings F1-F12)
- T1-T6 commits: `41178d4e`, `06ad5d30`, `8c4bfaf2`, `04c9d42a`, `0427067b`, and `402c5033`
