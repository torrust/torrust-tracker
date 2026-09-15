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
and compatibility redirects for the retired review skills. Manual scenarios M1-M3, the full linter
suite, focused structural checks, and the mandatory pre-commit gate passed.

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

## Avoiding Overcorrection

This evidence does not justify making every documentation link a bespoke shell assertion or adding
a new workflow framework. Use structural checks for metadata and multi-artifact invariants that
existing linters cannot validate, while retaining normal Markdown, link, spell, and full-linter
checks for their established scope.

## Evidence

- Issue #2219: `docs/issues/open/2219-2003-unify-pr-review-processing/ISSUE.md`
- Manual scenarios: `manual-verification-evidence.md` sections V1-V3
- Independent review history: `agent-review-reports.md`
- T1-T6 commits: `41178d4e`, `06ad5d30`, `8c4bfaf2`, `04c9d42a`, `0427067b`, and `402c5033`
