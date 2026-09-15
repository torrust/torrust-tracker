---
name: process-pr-review
description: Process every pull-request review finding, regardless of whether it was authored by Copilot, a person, or another bot. Use when asked to process PR review feedback, resolve review threads, audit review comments, or address Copilot and maintainer review findings together.
metadata:
  author: torrust
  version: "1.0"
  semantic-links:
    related-artifacts:
      - docs/issues/open/2219-2003-unify-pr-review-processing/ISSUE.md
      - .github/skills/dev/pr-reviews/fetch-review-threads/SKILL.md
      - .github/skills/dev/pr-reviews/resolve-review-threads/SKILL.md
---

# Processing Pull-Request Reviews

The PR author owns one tracked audit record at
`docs/pr-reviews/pr-<PR_NUMBER>-review.md`. Reviewers, including repository
review agents, deliver findings through GitHub only and create no repository
artifact. Process all authors through this workflow; author classification adds
context but never changes the audit or resolution requirements.

## Prerequisites

- Target pull request number and permission to push its branch and reply to its
  review threads.
- GitHub CLI (`gh`) and the `fetch-review-threads` and
  `resolve-review-threads` helper skills.
- Ability to run focused validation for any proposed fix.

## Workflow

1. **Fetch the source of truth.** Use `fetch-review-threads`' GraphQL scripts to
   collect all threads, including resolved and outdated threads. Also fetch each
   submitted review and its review-specific comments by review ID. REST comment
   responses are supplementary: GraphQL thread data is authoritative for thread
   identity and resolution state.
2. **Classify and normalize.** Classify `github-copilot[bot]` and
   `copilot-pull-request-reviewer` as Copilot, human accounts as human, and every
   other bot or unavailable author as `unknown`. Split each review body into one
   row per independently actionable assertion; do not make a row for a summary
   or verdict that contains no request. Preserve the source review ID and URL.
3. **Assign and deduplicate findings.** Use a reviewer-provided finding ID when
   present. Otherwise assign `F<ordinal>` in source-review and source-order
   order. Before action, compare a new item with all earlier findings against the
   current tree. A later item requesting the same current-tree change is a
   re-raise: keep its source row and record `RE_RAISE_OF:<FindingId>`.
4. **Decide against the current tree.** Re-derive every reply claim from the
   current tree with file inspection or a focused command. Record one of
   `FIXED`, `NO_ACTION`, `SUPERSEDED`, or `FOLLOW_UP`, the verification performed,
   and a concise independent summary. Never close a row with an undocumented
   disposition.
5. **Implement and validate fixes.** Make each independent fix in its own GPG
   signed Conventional Commit after focused validation. Cite the unique commit
   subject, not a branch SHA, because the branch can be rebased.
6. **Reply before resolving.** For each resolvable inline thread, reply with the
   disposition, current-tree verification, and resolution reference, then use
   `resolve-review-threads` to resolve it. For an outdated or superseded thread,
   reply exactly `Superseded by <FindingId>: <reason>.`, record
   `Disposition=NO_ACTION` and `Thread state=SUPERSEDED`, then resolve it.
7. **Consolidate review bodies.** A PR conversation response may cover multiple
   review rounds only when it names every review ID and every finding ID with its
   disposition and resolution reference. Store its durable URL in each related
   row.
8. **Verify completion.** Refresh GraphQL thread data and show no unresolved
   actionable thread. Update the audit progressively and commit it separately
   from product fixes.

## Required Audit Fields

Every normalized finding row records PR number, source review ID, source URL,
finding ID, severity, summary, relationship, disposition, current-tree
verification, resolution reference, reply URL, and thread state. Severity is
`Blocker`, `Major`, `Minor`, `Nit`, or `Suggestion`; mark a severity inferred
from free prose as inferred. Resolution references are unique Conventional
Commit subjects and/or durable reply URLs, never branch SHAs.

## Completion Checklist

- [ ] GraphQL data collected for all review threads
- [ ] Review bodies split into independent findings
- [ ] Re-raises mapped to their original finding before action
- [ ] Every action verified against the current tree, validated, and committed
- [ ] Every resolvable thread replied to before resolution
- [ ] Every superseded thread has the prescribed reply and audit state
- [ ] Consolidated responses name every covered review and finding
- [ ] Final GraphQL fetch reports no unresolved actionable thread
- [ ] Audit committed separately from product fixes
