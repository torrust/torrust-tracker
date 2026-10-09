---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
    - docs/issues/open/2264-2003-refactor-semantic-link-conventions/EPIC.md
    - docs/issues/open/2278-2003-strengthen-pr-review-author-self-audit/EPIC.md
    - docs/discussions/2003-overhaul-guardrails-and-automation/20261003-semantic-linking-knowledge-graph/README.md
    - docs/discussions/2003-overhaul-guardrails-and-automation/20261003-goals-and-boundaries/README.md
---

<!-- skill-link: process-pr-review -->

# PR #2434 Review Audit

Source: pull-request reviews and inline review threads for <https://github.com/torrust/torrust-tracker/pull/2434>.

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

Copilot review 5407866771 (round 1, state `COMMENTED`, submitted 2026-10-04 19:48 UTC on the PR's opening head, "Balanced" effort) left one inline comment. Its body is an overview ("Changes recommended") that restates the inline finding, so it creates no additional finding.

The comment opens with an explicit `[Major][F1]` bracket, so the row records `Major` from the bracket, and Copilot's ID is the audit-local ID. The overview's badge reads "Low severity"; the bracket governs, and it is what the validator checks. The record is committed after the fix and before the reply is posted: the thread state is `RESOLVED`, the value the skill prescribes for a concern a change fixed, and the Reply URL keeps the template's placeholder until the posted reply's URL is recorded.

Review 5467876260 (round 2, josecelano, state `COMMENTED`, submitted 2026-10-09 08:50 UTC on head `d26ba356c`) reports no blocking issues and left four inline comments, each opening with an explicit bracket: `[Minor][R1]`, `[Minor][R2]`, `[Nit][R3]` and `[Nit][R4]`. The rows record those severities and keep the reviewer's IDs as the audit-local IDs. Its body lists the checks it ran and restates the four findings, so it creates no additional finding. As in round 1, the rows are committed after the fixes and before the replies are posted: the thread state is `RESOLVED`, the value the threads take when they are resolved after the push, and each Reply URL keeps the template's placeholder until the posted reply's URL is recorded.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | `review-finding:pr-2434-f1` | Copilot | Major | documentation | ORIGINAL | FIXED | RESOLVED |
| R1 | `review-finding:pr-2434-r1` | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| R2 | `review-finding:pr-2434-r2` | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| R3 | `review-finding:pr-2434-r3` | Human | Nit | formatting | ORIGINAL | FIXED | RESOLVED |
| R4 | `review-finding:pr-2434-r4` | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - The source Outcomes still say nothing is recorded and lack their backlinks

- PR number: 2434
- Source review ID: 5407866771
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2434#discussion_r4179060807>
- Concern: "Recording the canonical decisions here leaves both source Outcomes contradictory and without the required backlink: the semantic-linking Outcome still says nothing has been recorded in #2264 and explicitly requires this entry to be linked, while goals-and-boundaries still says nothing has been recorded in #2264 or #2278 and requires Outcomes to link their canonical records. Update both discussion Outcomes in this PR so their status and navigation remain accurate."
- Solution: Replaced each Outcome's status sentence, which said that none of its answers was recorded in an EPIC, with one naming where they are recorded. The semantic-linking Outcome links #2264's `Decisions Recorded on This EPIC` section and names the Progress Log entry of 2026-10-04 and the order 5 and 7 cells, as its item 4 asks. The goals-and-boundaries Outcome links that section and #2278's `Decision Record`, and names the overhaul pull request #2366 for the #2003 parts, because `develop` does not yet have that section; PR #2366 replaces the clause with a link when it lands. The discussions convention allows the edit: a discussion records its outcome and links the canonical document that carries it (`docs/discussions/AGENTS.md`, step 3), and is afterwards edited only to repair links. No other text changed.
- Current-tree verification: `git grep -n -e "none has been recorded" -e "decisions-recorded-on-this-epic" -e "EPIC.md#decision-record" -- docs/discussions/2003-overhaul-guardrails-and-automation` at the branch head prints three lines, the new links at `20261003-goals-and-boundaries/README.md:239` and `:241` and `20261003-semantic-linking-knowledge-graph/README.md:234`, and no "none has been recorded"; each link resolves from its README's folder, anchor included; the fix commit's diff changes only the two status sentences.
- Resolution reference: `docs(discussions): link the #2264 and #2278 records from their Outcomes`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2434#discussion_r4179101218>

### R1 - The #2264 record calls the #2003 parts recorded before PR #2366 lands

- PR number: 2434
- Source review ID: 5467876260
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2434#discussion_r4228345308>
- Concern: "This bullet says the #2003 parts are already recorded, but they are only in the open PR #2366." On `develop`, the #2003 EPIC has no `Decisions Recorded on This EPIC` section and does not mention the four-aspect frame, while the goals-and-boundaries Outcome edited in this pull request already hedges "by its overhaul pull request #2366".
- Solution: The sentence names the pull request that records the #2003 parts, in the reviewer's wording, which matches the Outcome's hedge and stays true once PR #2366 merges. No other text changed.
- Current-tree verification: in `docs/issues/open/2264-2003-refactor-semantic-link-conventions/EPIC.md`, under "Decisions Recorded on This EPIC", the bullet "Where these decisions are recorded" reads "convention are recorded on #2003 by its overhaul pull request #2366, and the audit-record contract on #2278"; the phrase "convention are recorded on #2003, and" no longer occurs.
- Resolution reference: `docs(issues): [#2264] [#2278] hedge the #2003 record reference and cite the round-3 review`
- Follow-up PR URL: N/A
- Reply URL: <REPLY_URL_OR_NA>

### R2 - The specifications bullet and its log entry cite the round-1 review

- PR number: 2434
- Source review ID: 5467876260
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2434#discussion_r4228345322>
- Concern: "The fourth bullet comes from review 5402713128 (round 3), but this lead-in cites only review 5400754664." The specifications-and-rationale Outcome cites review 5402713128 (round 3, answer 4), and the Progress Log entry of 2026-10-04 19:18 UTC repeats the round-1 attribution.
- Solution: The fourth bullet cites review 5402713128 beside its own Outcome link, because the lead-in's review stays correct for the other three bullets, and the Progress Log entry names that review for the specifications discussion.
- Current-tree verification: in `docs/issues/open/2278-2003-strengthen-pr-review-author-self-audit/EPIC.md`, under "Decision Record", the bullet "the specifications-and-rationale outcome sends no work to this EPIC" ends "[review 5402713128](https://github.com/torrust/torrust-tracker/pull/2428#pullrequestreview-5402713128), round 3)."; under "Progress Log", the 2026-10-04 19:18 UTC entry reads "the specifications discussion (review 5402713128) sends no work to this EPIC"; `docs/discussions/2003-overhaul-guardrails-and-automation/20261003-specifications-and-rationale/README.md`, under "Outcome", cites "[review 5402713128]".
- Resolution reference: `docs(issues): [#2264] [#2278] hedge the #2003 record reference and cite the round-3 review`
- Follow-up PR URL: N/A
- Reply URL: <REPLY_URL_OR_NA>

### R3 - The #2264 Progress Log entry lacks its final period

- PR number: 2434
- Source review ID: 5467876260
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2434#discussion_r4228345351>
- Concern: "This Progress Log entry has no final period." The adjacent entries, and the matching #2278 entry added in this pull request, end with one.
- Solution: The entry ends with a period; its text is otherwise unchanged.
- Current-tree verification: in `docs/issues/open/2264-2003-refactor-semantic-link-conventions/EPIC.md`, under "Progress Log", the 2026-10-04 19:18 UTC entry ends "noted the scope of orders 5 and 7 in the subissue table.".
- Resolution reference: `docs(issues): [#2264] [#2278] hedge the #2003 record reference and cite the round-3 review`
- Follow-up PR URL: N/A
- Reply URL: <REPLY_URL_OR_NA>

### R4 - The Processing Log does not record the F1 thread's resolution

- PR number: 2434
- Source review ID: 5467876260
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2434#discussion_r4228345364>
- Concern: "The Processing Log never records the F1 thread being resolved." The 19:59 entry says the thread is resolved after the gate of record passes, the thread is resolved on GitHub, and no later entry records it.
- Solution: An appended entry records the resolution: the F1 thread was resolved immediately after reply 4179101218 was posted at 2026-10-04 19:59:32 UTC, in the same action. The entry carries the time it was recorded, and names the event's own time in its text, because the log is append-only and its entries must stay chronological.
- Current-tree verification: in this record, under "Processing Log", the entry of 2026-10-09 10:42 UTC reads "the F1 thread was resolved immediately after reply 4179101218 was posted at 2026-10-04 19:59:32 UTC".
- Resolution reference: `docs(pr-reviews): [#2264] [#2278] record the maintainer's review of PR #2434`
- Follow-up PR URL: N/A
- Reply URL: <REPLY_URL_OR_NA>

## Processing Log

- 2026-10-04 19:30 UTC - PR #2434 opened with two commits, `docs(issues): [#2264] record the semantic-linking and goals discussion outcomes` and `docs(issues): [#2278] record the audit-record decisions from the discussions`.
- 2026-10-04 19:48 UTC - Copilot review 5407866771 (`COMMENTED`, "Changes recommended") left one inline comment, 4179060807, bracketed `[Major][F1]`, on `docs/issues/open/2264-2003-refactor-semantic-link-conventions/EPIC.md` line 448; its overview restates that finding, so the review normalizes to F1 alone. The thread capture shows one thread, `resolved=false`.
- 2026-10-04 19:57 UTC - Re-derived F1 at the branch head: both Outcome sections still said none of their answers was recorded. Committed the fix as `docs(discussions): link the #2264 and #2278 records from their Outcomes`.
- 2026-10-04 19:57 UTC - Recorded this audit with F1 `FIXED`/`RESOLVED`. The reply is prepared and is posted after the fix and this record are pushed; its URL replaces the Reply URL placeholder then, and the audit validator runs in the gate of record.
- 2026-10-04 19:59 UTC - Pushed the fix and this record; replied on the F1 thread (reply 4179101218) and recorded its URL; the thread is resolved after the gate of record passes.
- 2026-10-04 20:00 UTC - The Docs Lint workflow failed on the pushed head: cspell flagged `backlink` in F1's quoted concern. Added the word to `project-words.txt`; the concern text is the reviewer's and is not reworded.
- 2026-10-09 08:50 UTC - Review 5467876260 (josecelano, `COMMENTED`, on head `d26ba356c`) reported no blocking issues and left four inline comments, bracketed `[Minor][R1]`, `[Minor][R2]`, `[Nit][R3]` and `[Nit][R4]`; its body restates them with the checks it ran, so the review normalizes to R1 to R4. The thread capture shows the four new threads `resolved=false` and the F1 thread `resolved=true`.
- 2026-10-09 10:41 UTC - Re-derived R1 to R3 at the branch head and fixed them in one commit, `docs(issues): [#2264] [#2278] hedge the #2003 record reference and cite the round-3 review`. Both EPICs' `last-updated-utc` is that minute; neither file logs corrections of its own text, so no Progress Log entry was added.
- 2026-10-09 10:42 UTC - Recorded the F1 thread's resolution, which the entry of 2026-10-04 19:59 UTC expected after the gate of record: the F1 thread was resolved immediately after reply 4179101218 was posted at 2026-10-04 19:59:32 UTC, in the same action. Recorded R1 to R4 `FIXED`/`RESOLVED`; the replies are drafted for posting after the push, and their URLs replace the Reply URL placeholders then.

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
