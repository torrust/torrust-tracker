---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
    - docs/discussions/2429-templated-documentation/20261009-templated-documentation-design/README.md
    - docs/discussions/2429-templated-documentation/20261009-templated-documentation-design/maintainer-review-session.md
---

<!-- skill-link: process-pr-review -->

# PR #2501 Review Audit

Source: pull-request reviews and inline review threads for <https://github.com/torrust/torrust-tracker/pull/2501>.

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

Copilot review 5469176634 (round 1, state `COMMENTED`, submitted 2026-10-09 11:02 UTC on head `23fb13343`, "Balanced" effort) left three inline comments, each opening with a `[Major][F<n>]` bracket. The reviewer-provided IDs F1 to F3 do not collide with any earlier finding, so they are kept as the audit IDs. The review body is an overview ("Changes recommended" and a list of the three inline findings) with no independently actionable assertion of its own, so it creates no additional row. The thread states are recorded after the replies were posted and the threads resolved.

Cameron Garnham's review 5469401053 (round 2, state `APPROVED`, submitted 2026-10-09 11:26 UTC on head `db3974d9c`; his round 1 was not posted because the head moved) left three inline comments with `[Minor][F4]`, `[Nit][F5]`, and `[Minor][F6]` brackets; the IDs do not collide with F1 to F3, so they are kept. Its body recomputes the PR and lists the same three findings; one further assertion in its Index and hygiene paragraph ("EPIC lines 41 and 62 and I2P README line 41 run to 155–167 columns in files wrapped near 100") is independently actionable and has no inline thread, so it becomes F7 with an inferred severity and `NON_RESOLVABLE` thread state. The body's other "reported" and "checked, no finding" items carry no request. His ACK comment on the PR conversation is a verdict, not a finding.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | `review-finding:pr-2501-f1` | Copilot | Major | correctness | ORIGINAL | FIXED | RESOLVED |
| F2 | `review-finding:pr-2501-f2` | Copilot | Major | correctness | ORIGINAL | FIXED | RESOLVED |
| F3 | `review-finding:pr-2501-f3` | Copilot | Major | documentation | ORIGINAL | FIXED | RESOLVED |
| F4 | `review-finding:pr-2501-f4` | Human | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F5 | `review-finding:pr-2501-f5` | Human | Nit | correctness | ORIGINAL | FIXED | RESOLVED |
| F6 | `review-finding:pr-2501-f6` | Human | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F7 | `review-finding:pr-2501-f7` | Human | Nit (inferred) | formatting | ORIGINAL | FIXED | NON_RESOLVABLE |

## Finding Details

### F1 - Q3 presents unresolved template granularity as settled

- PR number: 2501
- Source review ID: 5469176634
- Reviewer finding ID: F1
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2501#discussion_r4229404757>
- Concern: Q3 in `docs/discussions/2429-templated-documentation/20261009-templated-documentation-design/README.md` said that, under the proposal, each converted record has its own render template, but the proposal leaves template granularity open ("how many templates, per package or per concept"); only its root-ADR subissue says each ADR becomes a template.
- Solution: Rewrote the Q3 topic text to state that granularity is open, that only the root-ADR subissue chooses one template per document, and that Q3 examines that per-document option for living documents. Jose Celano's Q3 position is unchanged; it argues against the option and does not depend on the wording.
- Current-tree verification: `grep -c 'per package or per concept'` on the README prints `1`, inside Q3; the proposal at PR #2430 head `6535ce21e` contains the same phrase in its "Engineering Choices Left to Implementation" section.
- Resolution reference: `docs(discussions): [#2429] present template granularity as open in Q3`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2501#discussion_r4229443664>

### F2 - Q8 calls the tooling phase file-addition-only

- PR number: 2501
- Source review ID: 5469176634
- Reviewer finding ID: F2
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2501#discussion_r4229404825>
- Concern: Q8 said the tooling phase "only adds files", but the proposal adds a dataset crate, and this repository registers every in-repo crate explicitly in the root workspace members list, so that phase also changes shared workspace metadata.
- Solution: Q8 now says the tooling phase converts no documents but is not isolated, because new crates are registered in the root workspace `members` list.
- Current-tree verification: `grep -c 'only adds files'` on the README prints `0`; the root `Cargo.toml` has an explicit `members = [` list under `[workspace]`, and `.github/skills/dev/maintenance/add-workspace-member/SKILL.md` requires each crate to be listed there.
- Resolution reference: `docs(discussions): [#2429] stop calling the tooling phase file-addition-only`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2501#discussion_r4229443878>

### F3 - The session file is labelled source material although it was edited

- PR number: 2501
- Source review ID: 5469176634
- Reviewer finding ID: F3
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2501#discussion_r4229404897>
- Concern: `maintainer-review-session.md` was labelled "Source material", which `docs/discussions/AGENTS.md` requires to be kept as written (only disclosed linter edits), yet its assistant replies are condensed and the README called it the "full session". The reviewer asked to either preserve the original exchange or present the file consistently as an authored summary.
- Solution: Chose the second option, because no verbatim transcript was kept: the file is now labelled as Jose Celano's edited session record written for the opening round, "not source material and not policy", and states that it is not a verbatim transcript. The README describes it the same way and no longer says "full session".
- Current-tree verification: `grep -c 'Source material, not policy'` on the session file prints `0` and `grep -c 'full session'` on the README prints `0`; both files state that the record is not a verbatim transcript.
- Resolution reference: `docs(discussions): [#2429] present the session record as an edited record`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2501#discussion_r4229444092>

### F4 - Q6 quotes "committed check results" as the proposal's wording

- PR number: 2501
- Source review ID: 5469401053
- Reviewer finding ID: F4
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2501#discussion_r4229592072>
- Concern: Q6 put "committed check results" in quotation marks, but the proposal at `6535ce21e` says the data generator reads "(frontmatter, paths, links, committed captures and check results)"; the quoted string appears nowhere in the proposal. The session record quoted the same phrase. Non-blocking: the attribution to the proposal stands.
- Solution: Q6 now quotes the proposal's own wording, and the session record drops the quotation marks.
- Current-tree verification: `grep -c '"committed check results"'` prints `0` for both the README and the session record; the README contains "links, committed captures and check results" inside Q6.
- Resolution reference: `docs(discussions): [#2429] quote the proposal's own words for check results`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2501#discussion_r4237435459>

### F5 - Q9 omits one destination named by the discussions guide

- PR number: 2501
- Source review ID: 5469401053
- Reviewer finding ID: F5
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2501#discussion_r4229592078>
- Concern: Q9 said `docs/discussions/AGENTS.md` records conclusions in an issue, an EPIC specification, or an ADR, but the guide says "in the owning issue or EPIC specification, an ADR, or another canonical document".
- Solution: Q9 now restates the guide's full list.
- Current-tree verification: `grep -c 'or another canonical document'` on the README prints `1`, inside Q9; `docs/discussions/AGENTS.md` contains the same phrase.
- Resolution reference: `docs(discussions): [#2429] list every destination the discussions guide names`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2501#discussion_r4237435519>

### F6 - The last round 1 Processing Log entry is stamped after its commit

- PR number: 2501
- Source review ID: 5469401053
- Reviewer finding ID: F6
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2501#discussion_r4229592084>
- Concern: The entry stamped 2026-10-09 11:09 UTC is later than the commit that carries it (authored at 11:08:50 UTC), so the stamp was derived rather than read; the validator checks only the order of stamps, so it passed.
- Solution: Appended a correction that names the 11:09 entry, keeps it as written, and records that later stamps are read from `date -u` or taken from commit and GitHub timestamps. The correction notes that the audit commit's committer date changed when the branch was rebased onto `develop` on 2026-10-10, so its author date carries the original time.
- Current-tree verification: the Processing Log contains the entry starting "Correction: the entry stamped 2026-10-09 11:09 UTC"; `git log --format='%aI' --grep='add the PR #2501 review audit record'` prints `2026-10-09T12:08:50+01:00`.
- Resolution reference: `docs(pr-reviews): [#2429] correct the late stamp in the PR #2501 audit log`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2501#discussion_r4237435582>

### F7 - The milestone links made three I2P lines overlong

- PR number: 2501
- Source review ID: 5469401053
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2501#pullrequestreview-5469401053>
- Concern: The review body reports that `docs/issues/open/2491-i2p-peer-support/EPIC.md` lines 41 and 62 and the I2P discussion README line 41 run to 155–167 columns in files wrapped near 100 (MD013 is off, so no linter flags them). The lines came from this PR's change to the milestone links.
- Solution: Rewrapped the three sentences; the only wording change is "see the" before each milestone link.
- Current-tree verification: outside tables, no line in either file touched by this PR exceeds 110 columns; the one longer line in the EPIC (line 50, 112 columns) predates this PR.
- Resolution reference: `docs(issues): [#2491] rewrap the v4.0.0 milestone lines`
- Follow-up PR URL: N/A
- Reply URL: N/A

## Processing Log

- 2026-10-09 11:03 UTC - Started audit for Copilot review 5469176634 from the GraphQL thread list (`github-review-threads fetch`), the REST review, and the REST review comments: three unresolved inline threads opened by review comments 4229404757 (F1), 4229404825 (F2), and 4229404897 (F3), no human review, and a review body with no independently actionable assertion.
- 2026-10-09 11:04 UTC - Re-derived the three claims against the branch: all hold. Committed one fix per finding, each with the pre-commit hook passing.
- 2026-10-09 11:06 UTC - Pushed the fixes together with the five drift-fix commits prepared before the review (sccache ADR row, two missing templates, the semantic-link convention field lists, the I2P v4.0.0 references, and the Evidence note); the pre-push hook passed.
- 2026-10-09 11:07 UTC - Posted the replies at 11:07:12 (F1), 11:07:14 (F2), and 11:07:16 UTC (F3), each on its own source thread, then resolved the three threads; a GraphQL mutation response reports each `isResolved=true`.
- 2026-10-09 11:09 UTC - Ran `validate-audit-record.py --pr-number 2501 --base torrust/develop` against the fresh review-comment capture, replies included: `{"status": "ok", "rows": 3, "log_entries": 5, "failures": 0}`.
- 2026-10-10 11:15 UTC - Correction: the entry stamped 2026-10-09 11:09 UTC above is later than the commit that carries it (`docs(pr-reviews): [#2429] add the PR #2501 review audit record`, authored at 2026-10-09 11:08:50 UTC; its committer date changed when the branch was rebased onto `develop` on 2026-10-10), so the validator run it records happened at 11:08 UTC or earlier; its stamp was derived, not read from a clock. The entry is kept as written. Stamps from this entry on are read from `date -u` when the event is recorded.
- 2026-10-10 11:18 UTC - Recorded round 2 in one entry, after the correction above, so the log stays append-only and in order; each event's own time is given inline. Fetched round 2 at or before 11:12 UTC (GraphQL threads via `github-review-threads fetch`, REST reviews and review comments, PR conversation comments): review 5469401053 by `da2ce7` (`APPROVED`) with three unresolved inline threads opened by review comments 4229592072 (F4), 4229592078 (F5), and 4229592084 (F6), plus one actionable review-body assertion (F7); all four hold against the branch. Rebased the branch onto `develop` `f960c78d7` (clean, no overlapping files; the rebased commits carry committer dates of 11:12:56–57 UTC). Committed the F4 fix at 11:13:07, the F5 fix at 11:13:46, the F7 rewrap at 11:14:26, and the F6 correction at 11:16:07 UTC (committer dates read with `git log`), each with the pre-commit hook passing. Pushed with `--force-with-lease`, completing at 11:17:08 UTC (pre-push hook passed). Posted the replies at 11:17:20 (F4), 11:17:21 (F5), and 11:17:23 UTC (F6), each on its own source thread (GitHub `created_at`), then resolved the three threads; the GraphQL mutation responses report each `isResolved=true`, and `date -u` read 11:17:32 UTC after the last one. F7 has no thread to reply to or resolve.
- 2026-10-10 11:18 UTC - Ran `validate-audit-record.py --pr-number 2501 --base torrust/develop` against a fresh review-comment capture, replies included; `date -u` read 11:18:45 UTC when it finished: `{"status": "ok", "rows": 7, "log_entries": 7, "failures": 0}`.

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
