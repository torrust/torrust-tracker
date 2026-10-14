---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
    - docs/pr-reviews/pr-2506-review/PR-REVIEW.md
---

<!-- skill-link: process-pr-review -->

# PR #2528 Review Audit

Source: pull-request reviews and inline review threads for <https://github.com/torrust/torrust-tracker/pull/2528>.

## Ownership

The PR author owns this tracked audit record. Reviewers, including repository review agents,
deliver findings through GitHub and have no repository-artifact obligation.

- Post-merge workflow approval: N/A

## Status Values

- Relationship: `ORIGINAL`, `RE_RAISE_OF:<FindingId>`
- Disposition: `FIXED`, `NO_ACTION`, `SUPERSEDED`, `FOLLOW_UP`
- Thread state: `OPEN`, `RESOLVED`, `NON_RESOLVABLE`, `SUPERSEDED`. `OPEN` applies prospectively
  to audits created or updated for approved follow-up work; historical audits remain valid without
  bulk migration.
- Severity: `Blocker`, `Major`, `Minor`, `Nit`, `Suggestion`; append `(inferred)` when derived
  from free prose.
- Author class: `Copilot`, `Human`, `Unknown`
- Category: `link-integrity`, `formatting`, `metadata`, `testing`, `correctness`,
  `documentation`, `maintainability`, `security`, `other`
- An outdated thread whose concern was fixed is `FIXED`/`RESOLVED`, even when GitHub marks the
  original thread outdated after the push. For in-PR feedback, use `NO_ACTION`/`SUPERSEDED` only
  for a duplicate, superseded, or no-change concern. A post-merge `NO_ACTION` requires maintainer
  approval to decline the follow-up work.

## Findings

Copilot review 5519889203 (COMMENTED, submitted 2026-10-14 14:51:30 UTC, at head `d056de4f9`) left
one inline finding without a `[<Severity>][<FindingId>]` header; its overview rates it medium,
recorded here as `Minor (inferred)`. The overview also notes that Copilot could not run its full
agentic review before a timeout and summarizes the changes; neither is a request, so neither adds
a row.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | `review-finding:pr-2528-f1` | Copilot | Minor (inferred) | metadata | ORIGINAL | NO_ACTION | SUPERSEDED |

## Finding Details

### F1 - Use commit SHAs or URLs instead of commit subjects as resolution references

- PR number: 2528
- Source review ID: 5519889203
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2528#discussion_r4272378898>
- Concern: the PR #2506 audit's F7-F14 entries replace reply URLs with plain commit subjects as
  resolution references, which the reviewer considers hard to verify; it suggested a commit SHA or
  a GitHub commit URL, optionally with the subject.
- Solution: no change. The `process-pr-review` skill requires unique Conventional Commit subjects
  as `FIXED` resolution references and forbids branch SHAs, because a rebase changes them (PR
  #2524, whose fix commits these rows cite, was rebased twice). The audit validator rejects a
  `FIXED` reference that is not a commit subject on the branch. The subjects stay verifiable, and
  the audit already names the merge commit `8045bd252`.
- Current-tree verification: at this PR's head, each of the 12 distinct `FIXED` subjects in
  `docs/pr-reviews/pr-2506-review/PR-REVIEW.md` matches exactly one commit subject on `develop`
  (`git log --format=%s` with an exact-line match), and `validate-audit-record.py --pr-number 2506`
  exits 0 (14 rows, 0 failures).
- Resolution reference: <https://github.com/torrust/torrust-tracker/pull/2528#discussion_r4272424399>
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2528#discussion_r4272424399>

## Processing Log

- 2026-10-14 14:53 UTC - Started audit: read Copilot review 5519889203 (one inline finding).
- 2026-10-14 14:55 UTC - Verified the rule and the subjects against the current tree and replied on
  the thread with `NO_ACTION`; the thread is resolved after this audit is pushed.

## Completion Rules

- Re-derive the reply claim against the current tree before replying or resolving a thread.
- Reply on every resolvable thread before resolving it.
- For an outdated thread whose concern was fixed, record `Disposition=FIXED` and
  `Thread state=RESOLVED`, even if GitHub marks the thread outdated after the push. For a
  duplicate, superseded, or no-change in-PR thread, reply exactly
  `Superseded by <FindingId>: <reason>.`, record `Disposition=NO_ACTION` and
  `Thread state=SUPERSEDED`, then resolve it. A post-merge `NO_ACTION` requires maintainer
  approval to decline the follow-up work.
- A consolidated PR conversation response may cover multiple review rounds only when it names
  every review ID and every finding ID with its disposition and resolution reference. Record its
  durable URL in each related row.
- Cite a fix by its unique Conventional Commit subject or durable reply URL, never by a branch SHA
  that can change after a rebase.
- Refresh review threads using GraphQL and confirm that no unresolved actionable thread remains.
