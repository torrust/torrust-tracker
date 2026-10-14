---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
    - docs/pr-reviews/pr-2527-review/PR-REVIEW.md
    - docs/issues/closed/2517-1669-encapsulate-statistics-repository-state/ISSUE.md
---

<!-- skill-link: process-pr-review -->

# PR #2530 Review Audit

Source: pull-request reviews and inline review threads for <https://github.com/torrust/torrust-tracker/pull/2530>.

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

Copilot review 5520384339 (COMMENTED, submitted 2026-10-14 15:26:49 UTC, at head `77e718306`)
posted only an overview: it reports 0 open findings, summarizes the changes, and notes that the
full agentic review did not start before a timeout. None of that is a request, so it adds no row.

Human review 5520610065 by `da2ce7` (APPROVED, submitted 2026-10-14 15:43:03 UTC, at head
`87dec325a`) carries two inline findings, F1 and F2, both `Suggestion`. Its body records the
rebase-neutrality, move, reference, EPIC, and hygiene checks plus the verdict; it repeats the
inline findings rather than adding new requests, so it adds no row. The PR comment 6172232327 is
the reviewer's ACK.

F1 and F2 carry forward the remaining parts of the post-merge findings F2 and F3 on PR #2527
(audited in `docs/pr-reviews/pr-2527-review/PR-REVIEW.md`). Those are on a different pull request,
so each row here is `ORIGINAL`.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | `review-finding:pr-2530-f1` | Human | Suggestion | documentation | ORIGINAL | FIXED | RESOLVED |
| F2 | `review-finding:pr-2530-f2` | Human | Suggestion | documentation | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - The Risks bullet contradicts the Out of Scope note it cites

- PR number: 2530
- Source review ID: 5520610065
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2530#discussion_r4272979164>
- Concern: the archived #2517 spec's Risks bullet said the SI-30 traits draft was "not committed
  yet (see Out of Scope)", while Out of Scope in the same file says the work is tracked by #2521.
  #2528 fixed only the Out of Scope half, so the archived file contradicted itself.
- Solution: the Risks bullet now names #2521 and points at Out of Scope. The historical 09:06 UTC
  Progress Log entry that mentions the uncommitted draft is left as written.
- Current-tree verification: `grep -n 'not committed yet'` in the archived `ISSUE.md` matches
  only the historical Progress Log entry (:236). The Risks bullet at :373-374 reads
  "#2521 (see Out of Scope) touches the same `udp-core` and `udp-server` files".
- Resolution reference: docs(issues): [#2517] point the Risks overlap note at #2521
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2530#discussion_r4273199064>

### F2 - The stats-before reading is compared but still not stored

- PR number: 2530
- Source review ID: 5520610065
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2530#discussion_r4272979174>
- Concern: the evidence compares `stats-before.normalized.json` between the two M1 runs but stores
  only four files, so "one copy of each is stored" was one file short. The reviewer offered two
  fixes: store the file, or narrow the sentence.
- Solution: the M1 run directories were deleted after the second run, so the reading cannot be
  stored. The sentence now says that one copy of each output is stored except the stats-before
  reading, which was compared but not copied and rests only on the `diff -u` transcript.
- Current-tree verification: `manual-verification-evidence.md`:147-149 carries the narrowed
  sentence, and `evidence/` lists the four stored files: client responses, metrics before, metrics
  after, and stats after.
- Resolution reference: docs(issues): [#2517] say the stats-before reading was compared but not stored
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2530#discussion_r4273199409>

## Processing Log

- 2026-10-14 15:53 UTC - Started audit: read Copilot review 5520384339 (overview only) and
  `da2ce7` review 5520610065 (two inline Suggestions) with PR comment 6172232327 (ACK).
- 2026-10-14 16:01 UTC - Fixed F1 and F2 in separate commits, ran the pre-commit and pre-push
  gates, pushed, re-verified both claims at the pushed head, and replied on both threads. The
  threads are resolved after this audit is pushed.

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
