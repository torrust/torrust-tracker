---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
    - docs/pr-reviews/pr-2530-review/PR-REVIEW.md
    - docs/issues/closed/2517-1669-encapsulate-statistics-repository-state/ISSUE.md
---

<!-- skill-link: process-pr-review -->

# PR #2527 Review Audit

Source: pull-request reviews and inline review threads for <https://github.com/torrust/torrust-tracker/pull/2527>.

## Ownership

The PR author owns this tracked audit record. Reviewers, including repository review agents,
deliver findings through GitHub and have no repository-artifact obligation.

- Post-merge workflow approval: <https://github.com/torrust/torrust-tracker/pull/2527#issuecomment-6172511553>

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

PR #2527 merged at 2026-10-14 15:04:05 UTC as `184446a2b`. No audit existed at merge; this record
is created on the follow-up branch of PR #2530, as the post-merge workflow approval above allows.

Copilot review 5519841305 (COMMENTED, submitted 2026-10-14 14:48:06 UTC, at head `081f6e6b0`)
arrived before the merge but was not processed then. It posted only an overview that recommends
approval and reports 0 open findings, so it adds no row.

Human review 5520161573 by `da2ce7` (COMMENTED, submitted 2026-10-14 15:10:51 UTC, after the
merge, labelled `POST_MERGE_FINDINGS`) carries four inline findings: F1 (`Minor`) and F2-F4
(`Nit`). Its body records the visibility, doctest, red/green, evidence, and hygiene checks and
repeats the inline findings, so it adds no row.

Each finding was handled on PR #2530 and recorded as `FOLLOW_UP`/`OPEN` until that pull request
merged into `develop` (2026-10-14 16:28:07 UTC, as `b24550171`); every row is now
`FIXED`/`RESOLVED`. F2 and F3 were raised again on PR #2530 as its F1 and F2; see
`docs/pr-reviews/pr-2530-review/PR-REVIEW.md`.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | `review-finding:pr-2527-f1` | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F2 | `review-finding:pr-2527-f2` | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F3 | `review-finding:pr-2527-f3` | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F4 | `review-finding:pr-2527-f4` | Human | Nit | metadata | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - The doctests commit subject does not describe the commit

- PR number: 2527
- Source review ID: 5520161573
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2527#discussion_r4272604486>
- Concern: `docs(statistics): [#2517] introduce the repository privacy doctests` only adds the two
  lead-in sentences per repository file. The doctests themselves landed with each package's
  `refactor(<package>): [#2517] make the statistics repository lock private` commit, so the subject
  sends `git log` readers to the wrong commit. Since history is not reworded, the reviewer asked for
  a recorded correction.
- Solution: on PR #2530, the archived `manual-verification-evidence.md` gains a correction note
  where it lists that commit, and the archived `ISSUE.md` Progress Log gets a matching 16:00 UTC
  entry.
- Current-tree verification: on `develop` at `b24550171`, the archived
  `manual-verification-evidence.md`:20-23 carries the correction, and `ISSUE.md`:305 is the 16:00
  UTC Progress Log entry that repeats it.
- Resolution reference: docs(issues): [#2517] record that the doctests landed with each refactor commit
- Follow-up PR URL: <https://github.com/torrust/torrust-tracker/pull/2530>
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2527#discussion_r4273485516>

### F2 - Out of Scope and Risks still say the SI-30 traits draft is not committed

- PR number: 2527
- Source review ID: 5520161573
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2527#discussion_r4272604507>
- Concern: the #2517 spec's Out of Scope note and Risks bullet said the SI-30 traits draft was not
  committed yet, although #2521's spec was already on `develop`.
- Solution: #2528 fixed the Out of Scope half on `develop` with
  `docs(issues): [#2517] point the SI-30 traits out-of-scope note at #2521`. PR #2530 fixes the
  Risks half with `docs(issues): [#2517] point the Risks overlap note at #2521`, as its F1.
- Current-tree verification: on `develop` at `b24550171`, Out of Scope (:131-133) and the Risks
  bullet (:373-374) of the archived `ISSUE.md` both name #2521, and the only "not committed yet"
  left is the historical 09:06 UTC Progress Log entry (:236).
- Resolution reference: docs(issues): [#2517] point the Risks overlap note at #2521
- Follow-up PR URL: <https://github.com/torrust/torrust-tracker/pull/2530>
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2527#discussion_r4273485948>

### F3 - The stats-before reading is compared but not stored

- PR number: 2527
- Source review ID: 5520161573
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2527#discussion_r4272604529>
- Concern: the M1 evidence compares five outputs, including `stats-before.normalized.json`, but
  stores only four, so "one copy of each is stored" was one short.
- Solution: the run directory no longer exists, so the file cannot be stored. PR #2530 narrows the
  sentence with `docs(issues): [#2517] say the stats-before reading was compared but not stored`,
  as its F2.
- Current-tree verification: on `develop` at `b24550171`, the archived
  `manual-verification-evidence.md`:147-149 says the stats-before reading rests only on the
  `diff -u` transcript.
- Resolution reference: docs(issues): [#2517] say the stats-before reading was compared but not stored
- Follow-up PR URL: <https://github.com/torrust/torrust-tracker/pull/2530>
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2527#discussion_r4273486310>

### F4 - The evidence `last-updated-utc` was not bumped when the file last changed

- PR number: 2527
- Source review ID: 5520161573
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2527#discussion_r4272604541>
- Concern: the evidence record's `last-updated-utc` read 14:15 although
  `docs(issues): [#2517] record the second independent review` edited the file at 14:24 UTC.
- Solution: PR #2530's `chore(issues): archive closed issue #2517 spec to docs/issues/closed`
  bumped the stamp, and its later fix commits bump it again with each edit.
- Current-tree verification: on `develop` at `b24550171`, the archived
  `manual-verification-evidence.md` reads `last-updated-utc: 2026-10-14 16:00`, matching the time
  of its last edit.
- Resolution reference: chore(issues): archive closed issue #2517 spec to docs/issues/closed
- Follow-up PR URL: <https://github.com/torrust/torrust-tracker/pull/2530>
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2527#discussion_r4273486608>

## Processing Log

- 2026-10-14 15:53 UTC - Started post-merge triage: read Copilot review 5519841305 (overview only)
  and `da2ce7` review 5520161573 (four inline findings, submitted after the 15:04:05 UTC merge).
  Verified that every finding still applied to the current tree; made no change.
- 2026-10-14 15:56 UTC - The maintainer approved handling F1-F4 on the open PR #2530; approval
  recorded at the Ownership URL.
- 2026-10-14 16:01 UTC - Committed the F1-F3 changes on the PR #2530 branch (F4 was already
  handled by its archive commit), pushed, re-verified the claims at the pushed head, and replied on
  all four threads with `FOLLOW_UP`. The threads stay open until PR #2530 merges; then this record
  moves the rows to `FIXED` and the threads are resolved.
- 2026-10-14 16:30 UTC - PR #2530 merged into `develop` at 16:28:07 UTC as `b24550171`.
  Re-verified F1-F4 on `develop` and replied `FIXED` on each thread. Each row's Reply URL now
  cites that reply; the earlier `FOLLOW_UP` replies (r4273199747, r4273200081, r4273200471,
  r4273200938) remain on the threads. Moved F1-F4 to `FIXED`/`RESOLVED`, citing their fix commit
  subjects; the threads are resolved after this audit is pushed.

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
