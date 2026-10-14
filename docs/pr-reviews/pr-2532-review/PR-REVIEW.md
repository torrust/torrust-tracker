---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
    - docs/issues/closed/2521-1669-remove-unused-udp-stats-traits/ISSUE.md
    - docs/issues/closed/2517-1669-encapsulate-statistics-repository-state/ISSUE.md
---

<!-- skill-link: process-pr-review -->

# PR #2532 Review Audit

Source: pull-request reviews and inline review threads for <https://github.com/torrust/torrust-tracker/pull/2532>.

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

Review 5521450385 (round 1, `copilot-pull-request-reviewer`, state `COMMENTED`, submitted
2026-10-14 16:48 UTC on the head whose tip commit was
`chore(issues): archive closed issue #2521 spec to docs/issues/closed`,
<https://github.com/torrust/torrust-tracker/pull/2532#pullrequestreview-5521450385>) left no inline comment and reports "0 open findings". Its
"Needs a closer look" sentence is nevertheless an independently actionable assertion, recorded as
`F1` with `Thread state=NON_RESOLVABLE`. The rest of the body summarizes the changes and is not a
further finding.

Review 5522263962 (round 1, `da2ce7`, state `APPROVED`, submitted 2026-10-14 17:53 UTC on the head
whose tip commit was `docs(pr-reviews): [#2521] add the PR #2532 review audit`,
<https://github.com/torrust/torrust-tracker/pull/2532#pullrequestreview-5522263962>) approved and left one inline comment bracketed `[Nit][F2]`;
the row keeps that ID. Its body reports what it checked and repeats F2; neither is a further
finding. GitHub dismissed the approval as stale when the F2 fix was pushed. Review 5523636848
(josecelano, `COMMENTED`, 19:39 UTC) is the container GitHub created for the F2 reply.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | `review-finding:pr-2532-f1` | Copilot | Nit (inferred) | link-integrity | ORIGINAL | FIXED | NON_RESOLVABLE |
| F2 | `review-finding:pr-2532-f2` | Human | Nit | correctness | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - The archived #2517 spec still points at the old open path of the #2521 spec

- PR number: 2532
- Source review ID: 5521450385
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2532#pullrequestreview-5521450385>
- Concern: "One live cross-reference in the archived #2517 specification still points to the removed open path": the Out of Scope note in `docs/issues/closed/2517-1669-encapsulate-statistics-repository-state/ISSUE.md` names `docs/issues/open/2521-1669-remove-unused-udp-stats-traits/ISSUE.md`, which this PR moves.
- Solution: The reference had been retained as a historical record, but it is a navigational "tracked by" pointer, so it now names the closed path; the #2517 spec's `last-updated-utc` and progress log record the change. Outside this audit, the two remaining mentions of the old path are `Current-tree verification` commands in the PR #2529 audit, which record the commands as they were run.
- Current-tree verification: outside this audit, whose Concern quotes the old path and whose verification names the search pattern, `rg -n 'open/2521-1669-remove-unused-udp-stats-traits'` matches only lines 132 and 145 of `docs/pr-reviews/pr-2529-review/PR-REVIEW.md`; line 133 of the #2517 spec names `docs/issues/closed/2521-1669-remove-unused-udp-stats-traits/ISSUE.md`.
- Resolution reference: `docs(issues): [#2521] point the archived #2517 spec at the closed #2521 spec`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2532#issuecomment-6173714194>

### F2 - The F1 verification claim is contradicted by this audit's own lines

- PR number: 2532
- Source review ID: 5522263962
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2532#discussion_r4274337998>
- Concern: Once this audit was committed, the F1 search for the old open path matched four lines, not two: the two PR #2529 audit lines plus this audit's own F1 Concern (which quotes the old path) and F1 verification (which names the search pattern). The F1 Solution's "The two remaining mentions" sentence had the same problem. The validator does not check this prose.
- Solution: Both F1 claims are now scoped to the files outside this audit, and the verification names why the audit's own lines match. This entry avoids quoting the old path, so it adds no further match.
- Current-tree verification: the F1 search returns lines 132 and 145 of the PR #2529 audit and, in this audit, only the F1 Concern and F1 verification lines, as the scoped F1 claims now state. They are named by role because this audit's line numbers shift as entries are added.
- Resolution reference: `docs(pr-reviews): [#2521] scope the PR #2532 audit's old-path search to the files outside it`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2532#discussion_r4275413496>

## Processing Log

- 2026-10-14 16:48 UTC - Review 5521450385 (`copilot-pull-request-reviewer`, `COMMENTED`) left no
  inline comment; its body flags the #2517 spec reference.
- 2026-10-14 16:55 UTC - Committed the F1 fix at the maintainer's request.
- 2026-10-14 16:58 UTC - Pushed the fix and posted the PR conversation response.
- 2026-10-14 16:58 UTC - Created this audit.
- 2026-10-14 17:53 UTC - Review 5522263962 (`da2ce7`, `APPROVED`) left inline comment 4274337998.
- 2026-10-14 19:37 UTC - Committed the F2 fix at the maintainer's request.
- 2026-10-14 19:39 UTC - Pushed the F2 fix, which dismissed the approval as stale, and posted the
  F2 reply.
- 2026-10-14 19:40 UTC - Added F2 to this audit.

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
