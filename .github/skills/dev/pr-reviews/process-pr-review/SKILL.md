---
name: process-pr-review
description: Process every pull-request review finding, regardless of whether it was authored by Copilot, a person, or another bot. Use when asked to process PR review feedback, resolve review threads, audit review comments, address Copilot and maintainer findings together, or triage review feedback submitted after merge.
metadata:
  author: torrust
  version: "1.2"
  semantic-links:
    related-artifacts:
      - docs/issues/closed/2219-2003-unify-pr-review-processing/ISSUE.md
      - docs/templates/PR-REVIEW-TEMPLATE.md
      - docs/templates/REVIEW-FINDINGS.md
      - .github/skills/dev/pr-reviews/fetch-review-threads/SKILL.md
      - .github/skills/dev/pr-reviews/resolve-review-threads/SKILL.md
      - .github/skills/dev/pr-reviews/process-pr-review/scripts/validate-audit-record.py
---

# Processing Pull-Request Reviews

The PR author owns one tracked audit record at
`docs/pr-reviews/pr-<PR_NUMBER>-review/PR-REVIEW.md`. Reviewers, including
repository review agents, deliver findings through GitHub only and create no
repository artifact. Process all authors through this workflow; author
classification adds context but never changes the audit or resolution requirements.

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
9. **Verify completion.** Run `scripts/validate-audit-record.py` (see
   [Validation Script](#validation-script)) and fix every reported failure. Refresh GraphQL
   thread data and show no unresolved
   actionable thread. Update the audit progressively and commit it separately
   from product fixes.

## Reviews Submitted After Merge

A merged pull request is immutable delivery history. Review feedback submitted after merge is not
authorization to create a branch, modify `develop`, or open a follow-up pull request.

When a review body, inline thread, or comment arrives after the pull request merged:

1. **Stop after read-only triage.** Verify the merge time, review submission time, current target
   branch, thread state, and whether each finding remains live on the current target branch. Do not
   edit, branch, commit, push, reply, or resolve yet.
2. **Ask the maintainer.** Report the post-merge workflow gap and propose explicit dispositions: no
   action, audit-only recording, an existing issue/branch handoff, or a new follow-up branch and
   pull request. Obtain approval before choosing one. A pre-merge review-processing request does
   not authorize post-merge remediation. Record approval as a durable GitHub issue or pull-request
   comment URL in the original audit before any mutating action; chat-only approval is insufficient
   as the long-term evidence for this gate. When this rule is introduced after an approved action
   has already started, a retrospective approval record may be used only if it states the original
   approval time and the durable comment's creation time.
3. **Preserve the original audit.** Normalize every late finding into the merged PR's existing
   audit, including independently actionable review-body assertions. Use collision-safe audit IDs
   and retain reviewer-provided IDs in detail entries when reassigned.
4. **Implement only the approved follow-up.** Branch from the latest target branch, not from the
   merged PR head. Keep independent fixes and audit updates in coherent signed commits. Link the
   follow-up PR to the merged PR and to the issue that owns the remaining work; use issue-closing
   keywords only when the follow-up fully resolves that issue.
5. **Reply with follow-up evidence.** Reply on the merged PR's resolvable threads with the approved
   disposition and follow-up PR or merged-commit reference. Do not claim `FIXED` on the target
   branch before the follow-up merges; record `Disposition=FOLLOW_UP` and `Thread state=OPEN` while
   it remains open. A review-body finding without a thread remains `NON_RESOLVABLE`.
6. **Close the loop after merge.** After the follow-up merges, update the original audit with the
   durable merge reference, change completed dispositions to `FIXED`, reply if needed, then resolve
   addressed threads. Refresh GraphQL and record zero unresolved actionable threads.

If the maintainer declines follow-up work, record `NO_ACTION` with the reason only when the
maintainer approved that audit update. Do not silently turn a late review into a new project.

## Required Audit Fields

Every new normalized finding records PR number, source review ID, source URL,
author class, finding ID, review finding reference, severity, category, summary,
relationship, disposition, current-tree verification, resolution reference, reply
URL, optional reviewer finding ID when reassigned, follow-up PR URL, and thread
state. Record a post-merge approval URL once in the audit's Ownership section,
rather than repeating it in each detail entry. Record each finding as one compact
tracking row (finding ID, review finding reference, author class, severity,
category, relationship, disposition, thread state) plus one matching detail entry
carrying the remaining narrative and source-metadata fields, as laid out in the
audit template. The immutable repository reference is
`review-finding:pr-<PR_NUMBER>-<FINDING_ID>`, with a lowercase finding ID; use it
when another repository artifact needs to cite the finding. GitHub identifiers
remain source metadata, not the canonical finding reference. Never change the
reference after assigning it. Historical audit records remain unchanged and do
not need review-finding references.
Author class is `Copilot`, `Human`, or `Unknown`; category is `link-integrity`,
`formatting`, `metadata`, `testing`, `correctness`, `documentation`,
`maintainability`, `security`, or `other`. Severity is `Blocker`, `Major`,
`Minor`, `Nit`, or `Suggestion`; mark a severity inferred from free prose as
inferred. Resolution references are unique Conventional Commit subjects or
durable reply URLs, never branch SHAs. Record a follow-up pull request in the
separate Follow-up PR URL field, using `N/A` when it does not apply. Historical
records remain valid without the analysis fields.

## Validation Script

`scripts/validate-audit-record.py` mechanically checks the audit record against GitHub review
comments and branch history. Run it before every audit commit and before replying to or
resolving any thread:

```bash
python3 .github/skills/dev/pr-reviews/process-pr-review/scripts/validate-audit-record.py \
  --pr-number <PR_NUMBER>
```

It fetches review comments with `gh` unless `--comments-file` is given, and compares commit
subjects against `<base>..HEAD` (`--base` defaults to `develop`). It fails when:

- a tracking row has no detail entry, or a detail entry has no tracking row;
- a row's `Source review ID` does not match the review that owns its `Source URL`;
- a discussion-anchored row cites zero or several reply URLs, cites a reply that does not exist,
  or cites a reply posted on a different thread than its source comment;
- a `Resolution reference` names a commit subject that is not on the branch;
- a row's `Severity` differs from the `[Severity]` bracket of its source comment; or
- Processing Log entries are not in chronological order.

Every failure is a claim in the merged record that the bytes contradict, so treat a non-zero exit
as blocking. The script does not verify prose claims such as `Current-tree verification`
sentences, nor that a log stamp is later than the commit that carries it; check those by hand.

The script is a Python prototype. Like the other Python developer tools in this repository, it is
intended to be migrated to Rust; keep its behaviour as the reference when doing so.

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
- [ ] `scripts/validate-audit-record.py` exits `0` against the committed audit
- [ ] Every resolvable thread replied to before resolution
- [ ] Every superseded thread has the prescribed reply and audit state
- [ ] Consolidated responses name every covered review and finding
- [ ] Final GraphQL fetch reports no unresolved actionable thread
- [ ] Audit committed separately from product fixes
- [ ] For feedback submitted after merge: maintainer approval recorded before any mutating action
- [ ] For an approved post-merge follow-up: branch based on the current target branch and original
      audit updated through follow-up merge
