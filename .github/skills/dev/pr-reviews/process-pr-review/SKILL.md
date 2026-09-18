---
name: process-pr-review
description: Process every pull-request review finding, regardless of whether it was authored by Copilot, a person, or another bot. Use when asked to process PR review feedback, resolve review threads, audit review comments, or address Copilot and maintainer review findings together.
metadata:
  author: torrust
  version: "1.1"
  semantic-links:
    related-artifacts:
      - docs/issues/closed/2219-2003-unify-pr-review-processing/ISSUE.md
      - docs/templates/PR-REVIEW-TEMPLATE.md
      - docs/templates/REVIEW-FINDINGS.md
      - .github/skills/dev/pr-reviews/fetch-review-threads/SKILL.md
      - .github/skills/dev/pr-reviews/resolve-review-threads/SKILL.md
---

# Processing Pull-Request Reviews

The PR author owns one tracked audit record at
`docs/pr-reviews/pr-<PR_NUMBER>-review.md`. Reviewers, including repository
review agents, deliver findings through GitHub only and create no repository
artifact. Process all authors through this workflow; author classification adds
context but never changes the audit or resolution requirements.

## Purpose

Process every review finding correctly, efficiently, traceably, and deterministically: handle
every actionable suggestion, reply to every resolvable thread, and record each finding's progress
through resolution. The normalized audit data also supports continuous improvement: identify
recurring errors, review-process patterns, and automation or guardrail candidates so common
feedback is prevented earlier and future review processing consumes fewer tokens.

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
   `copilot-pull-request-reviewer` as `Copilot`, human accounts as `Human`, and
   every other bot or unavailable author as `Unknown`. Split each review body into
   one row per independently actionable assertion; do not make a row for a summary
   or verdict that contains no request. Preserve the source review ID and URL.
3. **Assign and deduplicate findings.** Use a reviewer-provided finding ID when
   present and it does not collide with an existing audit finding ID. When a
   later review reuses an existing ID for a different finding, assign the next
   audit-local `F<ordinal>` and record the reviewer's original ID in the detail
   entry. Otherwise assign `F<ordinal>` in source-review and source-order order.
   Assign the immutable repository reference
   `review-finding:pr-<PR_NUMBER>-<FINDING_ID>`, lowercasing the finding ID in
   the reference. Before action, compare a new item with all earlier findings
   against the current tree. A later item requesting the same current-tree
   change is a re-raise: keep its source row and record
   `RE_RAISE_OF:<FindingId>`.
4. **Categorize for future analysis.** For every new audit row, assign exactly one
   primary category: `link-integrity`, `formatting`, `metadata`, `testing`,
   `correctness`, `documentation`, `maintainability`, `security`, or `other`.
   Categorize the concern rather than its proposed fix; use `other` only when no
   listed category fits. Do not backfill or reinterpret historical audit records.
5. **Decide against the current tree.** Re-derive every reply claim from the
   current tree with file inspection or a focused command. Record one of
   `FIXED`, `NO_ACTION`, `SUPERSEDED`, or `FOLLOW_UP`, the verification performed,
   and a concise independent summary. Never close a row with an undocumented
   disposition.
6. **Implement and validate fixes.** Make each independent fix in its own GPG
   signed Conventional Commit after focused validation. Cite the unique commit
   subject, not a branch SHA, because the branch can be rebased.
7. **Reply before resolving.** For each resolvable inline thread, reply with the
   disposition, current-tree verification, and resolution reference, then use
   `resolve-review-threads` to resolve it. If a code or documentation change fixed
   the concern, record `Disposition=FIXED` and `Thread state=RESOLVED` even when
   GitHub marks the original thread outdated after the push. For a duplicate,
   superseded, or no-change thread, reply exactly `Superseded by <FindingId>:
<reason>.`, record `Disposition=NO_ACTION` and `Thread state=SUPERSEDED`, then
   resolve it.
8. **Consolidate review bodies.** A PR conversation response may cover multiple
   review rounds only when it names every review ID and every finding ID with its
   disposition and resolution reference. Store its durable URL in each related
   row.
9. **Verify completion.** Refresh GraphQL thread data and show no unresolved
   actionable thread. Update the audit progressively and commit it separately
   from product fixes.

## Required Audit Fields

Every new normalized finding records PR number, source review ID, source URL,
author class, finding ID, review finding reference, severity, category, summary,
relationship, disposition, current-tree verification, resolution reference, reply
URL, and thread state. Record each finding as one compact tracking row (finding
ID, review finding reference, author class, severity, category, relationship,
disposition, thread state) plus one matching detail entry carrying the remaining
narrative and source-metadata fields, as laid out in the audit template. The
immutable repository reference is
`review-finding:pr-<PR_NUMBER>-<FINDING_ID>`, with a lowercase finding ID; use it
when another repository artifact needs to cite the finding. GitHub identifiers
remain source metadata, not the canonical finding reference. Never change the
reference after assigning it. Historical audit records remain unchanged and do
not need review-finding references.
Author class is `Copilot`, `Human`, or `Unknown`; category is `link-integrity`,
`formatting`, `metadata`, `testing`, `correctness`, `documentation`,
`maintainability`, `security`, or `other`. Severity is `Blocker`, `Major`,
`Minor`, `Nit`, or `Suggestion`; mark a severity inferred from free prose as
inferred. Resolution references are unique Conventional Commit subjects and/or
durable reply URLs, never branch SHAs. Historical records remain valid without
the analysis fields.

## Advisory Reviewer Finding Format

The [review finding template](../../../../../docs/templates/REVIEW-FINDINGS.md) is
advisory: never reject a review for omitting it. For a formatted inline finding, require the first
line `[<Severity>][<FindingId>] <summary>`, with severity limited to `Blocker`, `Major`, `Minor`,
`Nit`, or `Suggestion`. Each independently actionable finding gets its own inline thread. A
re-raised finding uses the original finding ID and states the re-raise in its body. Review bodies
contain only the round verdict or summary; do not duplicate detailed inline findings there.

## Retiring or Replacing Review Workflow Documents

When a PR retires, renames, replaces, or supersedes a review-workflow document, do not rely on file
movement alone as evidence that the workflow contract survived. Before resolving the review finding,
record an inventory of every normative rule in the retiring artifact. For each rule, record one of:

- `PRESERVED`: the destination document and section that now owns the rule;
- `DROPPED`: the explicit reason the rule is no longer part of the workflow contract.

Resolve the finding only after the inventory is checked against the current tree and any preserved
rule is present in its named destination. Keep historical audit records unchanged; the inventory
guards live workflow behavior, not archival prose.

## Rename Migration Verification

When a review finding concerns a rename-only or move-with-limited-edits migration, verify the rename
mechanically before accepting the migration as pure. The verification record must name:

- the selected merge base or comparison commit;
- the old repository path;
- the new repository path;
- the reviewed expected zero-context patch for that file.

Compare the actual zero-context patch with the reviewed expectation and fail the verification when
they differ. The approved patch may be empty only when the actual patch is also empty. Use a
failure-propagating comparison such as:

```sh
actual_patch=$(git diff -U0 "<base>:<old-path>" "HEAD:<new-path>")
test "$actual_patch" = "$(cat "<approved-zero-context-patch>")"
```

Record the command, base, paths, expected-patch artifact or inline expectation, and result in the
audit detail before resolving the finding.

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
