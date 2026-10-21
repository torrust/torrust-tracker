---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
    - docs/issues/open/2525-ci-gate-required-check/evidence/t5-gate-logs.txt
    - docs/issues/open/2525-ci-gate-required-check/ISSUE.md
---

<!-- skill-link: process-pr-review -->

# PR #2605 Review Audit

Source: pull-request reviews and inline review threads for <https://github.com/torrust/torrust-tracker/pull/2605>.

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

Five reviews were submitted so far, all by Copilot. Each body repeats its inline threads, so the bodies add
no rows.

- Review 5599040224 read the head whose subject is "docs(issues): [#2525] propose the design of the override
  across runs". It carries two inline threads, F1 and F2.
- Review 5599861652 read the head whose subject is "docs(pr-reviews): add the PR #2605 review audit". It
  marks F1 and F2 as resolved and carries one inline thread, F3.
- Review 5600335776 read the head whose subject is "docs(pr-reviews): record the second review round in the PR
  #2605 audit". It marks F3 as resolved and carries two inline threads, F4 and F5.
- Review 5601400687 read the head whose subject is "docs(pr-reviews): record the third review round in the PR
  #2605 audit". It marks F4 and F5 as resolved and carries one inline thread, F6, which asks for the same
  change as F4 and is recorded as a re-raise of it.
- Review 5602414651 read the head whose subject is "docs(pr-reviews): record the fourth review round in the PR
  #2605 audit". It marks F6 as resolved and carries two inline threads, F7 and F8, which ask once more for the
  change of F4 and are recorded as re-raises of it.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2605-f1 | Copilot | Nit (inferred) | documentation | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-2605-f2 | Copilot | Minor (inferred) | documentation | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-2605-f3 | Copilot | Minor (inferred) | correctness | ORIGINAL | FIXED | RESOLVED |
| F4 | review-finding:pr-2605-f4 | Copilot | Nit (inferred) | formatting | ORIGINAL | NO_ACTION | SUPERSEDED |
| F5 | review-finding:pr-2605-f5 | Copilot | Nit (inferred) | documentation | ORIGINAL | FIXED | RESOLVED |
| F6 | review-finding:pr-2605-f6 | Copilot | Nit (inferred) | formatting | RE_RAISE_OF:F4 | NO_ACTION | SUPERSEDED |
| F7 | review-finding:pr-2605-f7 | Copilot | Nit (inferred) | formatting | RE_RAISE_OF:F4 | NO_ACTION | SUPERSEDED |
| F8 | review-finding:pr-2605-f8 | Copilot | Nit (inferred) | documentation | RE_RAISE_OF:F4 | NO_ACTION | SUPERSEDED |

## Finding Details

### F1 - The saved sandbox workflow says it was never merged upstream

- PR number: 2605
- Source review ID: 5599040224
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2605#discussion_r4336296087>
- Concern: the saved workflow `exp-2525-orchestrator.yaml` is now part of the upstream repository, under `docs/`, so its first-line comment "Never merged upstream" is misleading. Reword it to say that it is not meant for production and is stored as evidence.
- Solution: the first line of the saved workflow says "Stored here as evidence, not as a workflow of this repository". The five saved workflows of the T4 evidence had the same wording and got the same change, and the evidence notes in V8 and V10 that only that comment was reworded.
- Current-tree verification: no saved file contains "Never merged" any more, and all six pass yamllint.
- Resolution reference: docs(issues): [#2525] say that the saved sandbox workflows are evidence, not unmerged workflows
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2605#discussion_r4336492199>

### F2 - A log line in the evidence is cut off with an ellipsis

- PR number: 2605
- Source review ID: 5599040224
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2605#discussion_r4336296176>
- Concern: the first classification line of `t5-gate-logs.txt` ends with `...`, which makes the evidence not verbatim. Add a note that the output was abbreviated, or include the full line.
- Solution: the full line, 1,093 characters, was restored from the log of the run itself, and checked to be valid JSON, so no note is needed.
- Current-tree verification: the file has no line ending in an ellipsis, and the restored line begins with the text of the truncated one.
- Resolution reference: docs(issues): [#2525] restore the full classification line in the T5 gate logs
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2605#discussion_r4336492533>

### F3 - The saved workflow does not paginate the check suites

- PR number: 2605
- Source review ID: 5599861652
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2605#discussion_r4336946940>
- Concern: the `check-suites` call of the saved sandbox workflow is not paginated, so the first-seen time, and
  with it the stale-override decision, can be wrong when the head commit has more than 100 check suites. Use
  `gh api --paginate` and take the minimum across pages, as is done for the check runs.
- Solution: the concern is right for that script, but the file is evidence of what ran in the sandbox, so it
  was not changed. The limit is stated in V10, and the code that replaces it, the adapter of T6a in pull
  request 2610, reads every page of every response; no test covered the pagination of the check suites, so
  that pull request gained one, with the earliest suite on the second page. The limit stated in V10 is the fix
  of this pull request, so the disposition is `FIXED`.
- Current-tree verification: V10 states the limit and points to the adapter. In the branch of pull request
  2610 the test fails when only the first page, or the first listed suite, is used (two mutants), and
  `cargo test -p ci-gate` passes with 199 unit and 18 command-line tests.
- Resolution reference: docs(issues): [#2525] record that the saved T5 workflow does not paginate the check suites
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2605#discussion_r4337084993>

### F4 - The review audit's table is said to have a doubled leading pipe

- PR number: 2605
- Source review ID: 5600335776
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2605#discussion_r4337308235>
- Concern: each row of the audit's table is said to start with `||`, which renders an empty first column.
  Start each row with a single `|`.
- Solution: no change. No row of the table starts with `||`.
- Current-tree verification: `grep -c "^||"` on the audit gives 0, every table row starts with a single `|`,
  as in the earlier audits, and the audit validator parses all three rows then and now. The doubled pipe may
  come from how a diff shows an added table row, with the diff marker before the first pipe.
- Resolution reference: <https://github.com/torrust/torrust-tracker/pull/2605#discussion_r4337403647>
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2605#discussion_r4337403647>

### F5 - The sandbox folder in the log header is ambiguous

- PR number: 2605
- Source review ID: 5600335776
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2605#discussion_r4337308336>
- Concern: the header of `t5-gate-logs.txt` names the saved sandbox files as `t5-sandbox/.`, which is ambiguous
  from the repository root. Use the full repository-relative path.
- Solution: the header gives `docs/issues/open/2525-ci-gate-required-check/evidence/t5-sandbox/`. The T4
  transcript named `t4-sandbox/` the same way and got the same change.
- Current-tree verification: both transcripts name the full path, and the path exists.
- Resolution reference: docs(issues): [#2525] give the sandbox folders in the evidence transcripts as repository paths
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2605#discussion_r4337404053>

### F6 - The audit's table is again said to have a doubled leading pipe

- PR number: 2605
- Source review ID: 5601400687
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2605#discussion_r4338131023>
- Concern: the rows of the audit's table are said to start with `||`, which renders an empty first column, and
  should start with a single `|`, header and separator rows included. It is the concern of F4, raised again
  on line 59.
- Solution: no change, for the reason given for F4. No row of the table starts with `||`.
- Current-tree verification: at the head commit `grep -c "^||"` on the audit gives 0, line 59 is the row of F5
  and begins with a single `|`, as does every other table row, and the audit validator parses all five rows.
- Resolution reference: <https://github.com/torrust/torrust-tracker/pull/2605#discussion_r4338446820>
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2605#discussion_r4338446820>

### F7 - The audit's table is said, a third time, to have a doubled leading pipe

- PR number: 2605
- Source review ID: 5602414651
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2605#discussion_r4338910344>
- Concern: the rows of the audit's table are said to start with `||`, which renders an empty first column, on line 63. It is the concern of F4, raised for the third time.
- Solution: no change, for the reason given for F4. No row of the table starts with `||`.
- Current-tree verification: the file was fetched from the GitHub contents API at the head of the pull request (blob `1e414fc2a`, head `74d85083b`): `grep -c "^||"` gives 0, and every table row begins with a pipe and a space. The audit validator parses all six rows.
- Resolution reference: <https://github.com/torrust/torrust-tracker/pull/2605#discussion_r4338938099>
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2605#discussion_r4338938099>

### F8 - The audit's table is said, a third time, to have a doubled leading pipe

- PR number: 2605
- Source review ID: 5602414651
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2605#discussion_r4338910408>
- Concern: line 125 is said to conflict with the table, whose rows are said to begin with `||`; either change the rows to start with a single pipe or change the write-up to match the file. It is the concern of F4 again.
- Solution: no change, for the reason given for F4. No row of the table starts with `||`.
- Current-tree verification: line 125 is the text of the verification of F4, not a table row. At the same blob `grep -c "^||"` gives 0 and all 8 table rows, the header, the separator, and six findings, begin with a single pipe, so the write-up already matches the file. The reply first said 12 rows and was corrected to 8.
- Resolution reference: <https://github.com/torrust/torrust-tracker/pull/2605#discussion_r4338938439>
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2605#discussion_r4338938439>

## Processing Log

- 2026-10-21 11:36 UTC - Copilot's review arrived with two threads, F1 and F2. Both were checked against
  the tree and confirmed; F2 was my own truncation.
- 2026-10-21 11:53 UTC - Fixed each finding in its own signed commit, recorded the round in the
  specification, ran the pre-push checks, and pushed.
- 2026-10-21 11:56 UTC - Replied on the two threads after the push, and started this record.
- 2026-10-21 12:46 UTC - Copilot's second review arrived with one thread, F3. It was checked and confirmed for
  the saved script; the file is evidence, so the limit was documented, and the missing pagination test of
  the check suites was added in the pull request of the code that replaces it.
- 2026-10-21 12:59 UTC - Pushed, replied on the thread, and added F3 to this record.
- 2026-10-21 13:20 UTC - Copilot's third review arrived with two threads, F4 and F5. F5 was confirmed and fixed.
  F4 does not match the file: no table row starts with a doubled pipe.
- 2026-10-21 13:29 UTC - Pushed, replied on both threads, and added F4 and F5 to this record. The reply to F4
  was rewritten into the prescribed form `Superseded by F4: <reason>.`, and F3 was changed from `FOLLOW_UP` to
  `FIXED`, because the skill gives a follow-up a reply URL as its reference and this finding was fixed here.
- 2026-10-21 14:34 UTC - Copilot's fourth review arrived with one thread, F6, the concern of F4 raised again.
  Checked at the head commit: no row starts with a doubled pipe.
- 2026-10-21 15:05 UTC - Replied `Superseded by F4: <reason>.` on the thread and added F6 to this record.
- 2026-10-21 15:52 UTC - Copilot's fifth review arrived with two threads, F7 and F8, the claim of F4 a third time.
  Fetched the file from the GitHub contents API at the head: no line starts with a doubled pipe.
- 2026-10-21 15:55 UTC - Replied `Superseded by F4: <reason>.` on both threads, corrected the row count in the
  reply to F8 from 12 to 8, and added F7 and F8 to this record.

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
