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

Ten reviews were submitted so far, seven by Copilot and three by the reviewer `da2ce7`. Each body repeats its inline threads, so the bodies add
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
- Review 5602711782 read the head whose subject is "docs(pr-reviews): record the fifth review round in the PR
  #2605 audit". It marks F7 and F8 as resolved and carries two inline threads: F9, a new finding about the
  saved evidence, and F10, the claim of F4 a fourth time, recorded as a re-raise of it.
- Review 5603029640 read the head whose subject is "docs(pr-reviews): record the sixth review round in the PR
  #2605 audit". It marks F9 and F10 as resolved and carries one inline thread, F11, the claim of F4 a fifth time,
  recorded as a re-raise of it.
- Review 5604657954, by `da2ce7`, read the head whose subject is "docs(pr-reviews): add the review retrospective for
  PR #2605 and PR #2610" and was submitted on 2026-10-21 at 18:52 UTC, with an ACK of that head as an issue comment,
  which adds no row. GitHub shows its state as dismissed. It carries five inline threads, F12 to F16, each with the
  reviewer's own severity, and says that none blocks.
- Review 5619147885, by `da2ce7`, read the head whose subject is "docs(pr-reviews): record the answer to F16 and the
  closing of PR #2610 in the PR #2605 audit" and was submitted on 2026-10-22 at 18:41 UTC, with an ACK of that head as
  an issue comment, which adds no row. GitHub shows its state as dismissed. It finds F12 to F16 repaired. It
  carries five inline threads, which it labels F17, F18, F19, F20, and F22, each with its own severity, and says
  that none blocks. Its F21, in a round that was not posted, was found repaired by the commit that added the
  row of F16, and is not a row.
- Review 5619454221, by `da2ce7`, read the head whose subject is "docs(pr-reviews): record the third review of the second
  reviewer in the PR #2605 audit" and was submitted on 2026-10-22 at 19:05 UTC, with an ACK of that head as an issue
  comment, which adds no row. GitHub shows its state as dismissed. It finds F17 to F22 repaired and carries two inline
  threads, F23 and F24, both Nits, and says that none blocks.

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
| F9 | review-finding:pr-2605-f9 | Copilot | Minor (inferred) | documentation | ORIGINAL | FIXED | RESOLVED |
| F10 | review-finding:pr-2605-f10 | Copilot | Nit (inferred) | formatting | RE_RAISE_OF:F4 | NO_ACTION | SUPERSEDED |
| F11 | review-finding:pr-2605-f11 | Copilot | Nit (inferred) | formatting | RE_RAISE_OF:F4 | NO_ACTION | SUPERSEDED |
| F12 | review-finding:pr-2605-f12 | Human | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F13 | review-finding:pr-2605-f13 | Human | Minor | metadata | ORIGINAL | FIXED | RESOLVED |
| F14 | review-finding:pr-2605-f14 | Human | Nit | formatting | ORIGINAL | FIXED | RESOLVED |
| F15 | review-finding:pr-2605-f15 | Human | Nit | metadata | ORIGINAL | FIXED | RESOLVED |
| F16 | review-finding:pr-2605-f16 | Human | Suggestion | documentation | ORIGINAL | FIXED | RESOLVED |
| F17 | review-finding:pr-2605-f17 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F18 | review-finding:pr-2605-f18 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F19 | review-finding:pr-2605-f19 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F20 | review-finding:pr-2605-f20 | Human | Nit | metadata | ORIGINAL | FIXED | RESOLVED |
| F22 | review-finding:pr-2605-f22 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F23 | review-finding:pr-2605-f23 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F24 | review-finding:pr-2605-f24 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |

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
- Current-tree verification: the file was fetched from the GitHub contents API at the head of the pull request (blob `1e414fc2a`, at the head whose subject is "docs(pr-reviews): record the fourth review round in the PR #2605 audit"): `grep -c "^||"` gives 0, and every table row begins with a pipe and a space. The audit validator parses all six rows.
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

### F9 - The saved logs cannot be reconciled with the saved workflow

- PR number: 2605
- Source review ID: 5602711782
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2605#discussion_r4339141583>
- Concern: the logs of `t5-gate-logs.txt` show `ci-gate verdict` invoked with an empty `--override-actor` when
  there is no label, but the saved workflow adds that option only when the actor is not empty. Either note that
  runs 1 and 3 used an earlier revision and identify it, or save the earlier workflow next to the fixed one.
- Solution: both. The first revision of the workflow is saved as `t5-sandbox/exp-2525-orchestrator-first-revision.yaml`
  next to the fixed one, the header of the logs and V10 say that runs 1 to 4 ran the first revision and runs 5 to
  7 the fixed one, name the sandbox commits, and say what differs (the probe step, and a test with `-s` that is
  true for a file that holds only a newline), and the logs show which revision ran, because only the fixed one
  prints `probe:` lines. V10 had said that only run 3 still used the old workflow, and run 4 did too; that is
  corrected.
- Current-tree verification: the first revision, extracted from the sandbox commit, differs from its saved copy
  only on the first line, and from the fixed one in the probe step, the test of the actor file, and the first
  line; the saved first revision passes yamllint. The `probe:` lines are in runs 5 to 7 and not in runs 1 to 4.
- Resolution reference: docs(issues): [#2525] save the first revision of the T5 workflow and say which runs used it
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2605#discussion_r4339194479>

### F10 - The audit's table is said, a fourth time, to have a doubled leading pipe

- PR number: 2605
- Source review ID: 5602711782
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2605#discussion_r4339141684>
- Concern: the rows of the findings table are said to start with `||`, which renders an empty first column and
  contradicts the later text of F4, F6, F7, and F8. It is the concern of F4, raised for the fourth time, at line 68.
- Solution: no change, for the reason given for F4. No row of the table starts with `||`.
- Current-tree verification: the file was fetched from the GitHub contents API at the head of the pull request
  (blob `05e3cb88b`, at the head whose subject is "docs(pr-reviews): record the fifth review round in the PR #2605 audit"): `grep -c "^||"` gives 0, the 10 table rows, the header, the separator, and
  eight findings, begin with a single pipe, and line 68 is the row of F8. The audit validator parses all rows.
- Resolution reference: <https://github.com/torrust/torrust-tracker/pull/2605#discussion_r4339194835>
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2605#discussion_r4339194835>

### F11 - The audit's table is said, a fifth time, to have a doubled leading pipe

- PR number: 2605
- Source review ID: 5603029640
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2605#discussion_r4339393136>
- Concern: the rows of the findings table, header and separator included, are said to start with a doubled
  leading pipe, which renders an empty first column. It is the concern of F4, raised for the fifth time, at
  line 73.
- Solution: no change, for the reason given for F4. This entry describes the claim in words and does not quote
  the characters, to see whether the next review raises it again.
- Current-tree verification: the file was fetched from the GitHub contents API at the commit that review read
  (blob `d9528aa39`, at the commit whose subject is "docs(pr-reviews): record the sixth review round in the PR #2605 audit"): no line starts with a doubled pipe, the 12 table rows, the header, the
  separator, and ten findings, begin with a single pipe followed by a space, and line 73 is the row of F10. The
  audit validator parses all rows.
- Resolution reference: <https://github.com/torrust/torrust-tracker/pull/2605#discussion_r4339514454>
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2605#discussion_r4339514454>

### F12 - The saved listings never show the check runs that V10 describes

- PR number: 2605
- Source review ID: 5604657954
- Reviewer finding ID: F12
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2605#discussion_r4340695881>
- Concern: in none of the seven runs does the saved list of all check runs for the head commit include a `CI gate` run, a skipped run, or a run named `matrix.group`, although V10 describes them. Either the list is not every check run, or the observations come from a source that the evidence does not record; the design rests on which runs the endpoint returns. Record the command and output behind the observations, and say whether the list is filtered.
- Solution: the saved file had kept only some of the lines of each list, which was an error in saving the evidence. The logs of the seven runs are still on GitHub; each saved list was compared with the real one, every saved line was found in it, and the real lists are restored with a note in the header. V10 records the command and the real output for the head of runs 5 to 7 (15 runs, with three `CI gate` runs and two skipped `matrix.group` runs), and that the same request with `filter=all` returned the same runs.
- Current-tree verification: the restored lists are the output of `gh run view <run id> --repo josecelano/torrust-tracker --log` for each run, and the V10 listing is the output of the API request on 2026-10-22; both agree with the statements of V10. A rerun of a job inside one check suite was not tested, and V10 says so.
- Resolution reference: docs(issues): [#2525] restore the full check-run listings of the T5 logs and record the real listing
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2605#discussion_r4351877097

### F13 - Records cite commits of this branch by id

- PR number: 2605
- Source review ID: 5604657954
- Reviewer finding ID: F13
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2605#discussion_r4340695889>
- Concern: three lines of this record and one of the retrospective cite commits of this branch by id; the `open-pull-request` check prints them, and the skill says to cite the Conventional Commit subject instead.
- Solution: each of the four cites the subject now. The check, run in a clone that also holds the other pull request's branch, printed one more, in the retrospective, which is a subject now too. What it still prints are the fork's sandbox commits in the saved T4 and T5 evidence, which do not resolve in a clone without the fork's branches.
- Current-tree verification: the check prints no commit of this branch.
- Resolution reference: docs(pr-reviews): cite commit subjects, not ids, in the PR #2605 audit and retrospective
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2605#discussion_r4351877516

### F14 - The T5 gate logs end with a blank line

- PR number: 2605
- Source review ID: 5604657954
- Reviewer finding ID: F14
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2605#discussion_r4340695905>
- Concern: the file ends with a blank line, which `git diff --check` reports, and no gate catches it.
- Solution: the blank line is removed.
- Current-tree verification: `git diff --check` is clean for the file.
- Resolution reference: docs(issues): [#2525] drop the blank line at the end of the T5 gate logs
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2605#discussion_r4351877932

### F15 - Two log entries are stamped before the events they report

- PR number: 2605
- Source review ID: 5604657954
- Reviewer finding ID: F15
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2605#discussion_r4340695913>
- Concern: the 11:53 entry ends "and pushed" but the round's last commit is later, and the 11:56 entry reports the replies and the start of this record, which are later. Restamp them or append a correction entry.
- Solution: a correction entry with the true times is appended, and the two entries are left as written, so that the record shows the slip.
- Current-tree verification: the times were checked against the commit times and the reply times from the API: the last commit at 11:54:43, the replies at 11:58:17 and 11:58:20, and the first commit of this record at 12:00:31 UTC. The validator reports the order of the entries as valid.
- Resolution reference: docs(pr-reviews): correct the times of two log entries in the PR #2605 audit
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2605#discussion_r4351878322

### F16 - Half of the retrospective is about another pull request whose audit is not in this tree

- PR number: 2605
- Source review ID: 5604657954
- Reviewer finding ID: F16
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2605#discussion_r4340695920>
- Concern: the retrospective is half about #2610, whose claims cannot be checked from this tree, and it cites that pull request's audit, which does not exist here. The template places one retrospective in each pull request's review folder, so keep this pull request's part here and move the other part to the other pull request's folder when it lands, with a pointer each way.
- Solution: partly adopted. The concern is addressed, and the suggested split is not done. Pull request #2610 was closed unmerged by the maintainer's decision of 2026-10-22, so its folder will not land and a retrospective moved there would have no audit beside it. The audit of #2610 is added here as the historical copy `pr-2610-audit-record.md`, not named as an audit and without the `skill-link` marker, because the audit validator checks resolution references against the commits of the branch that holds the record and #2610's commits are not on this one. The retrospective cites that copy, and its figures for #2610 were recomputed from the API and git. The reply offers the split as a further commit if the reviewer prefers it.
- Current-tree verification: the copy and the retrospective pass the Markdown, spelling, and link checks and the pre-commit gate. The audit validator was run on the copy under its original name, where it fails for the reason given, and that is why it is not stored under that name.
- Resolution reference: docs(pr-reviews): add the PR #2610 audit as a historical record next to the retrospective
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2605#discussion_r4352032069

### F17 - The API observations behind the decision have no query or output

- PR number: 2605
- Source review ID: 5619147885
- Reviewer finding ID: F17
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2605#discussion_r4352159453>
- Concern: the observations cited for the decision (a null `pushedDate` for the commits queried, empty `pull_requests` lists, the force-push event) have no query or output, here or in the historical copy, and V10's permission responses have no transcript. Record the queries and their output beside the evidence, or say they were not kept.
- Solution: V12 of the evidence records each query and its output, re-run on 2026-10-22 against the closed pull request, and says that it is the state at that time. It also says what was not observed: no force-push happened on #2610, so the statement about the force-push event is the reviewer's, and that the GraphQL count of 195 against 59 commits is the known caveat that the type filter is ignored (added after the reviewer's F23).
- Current-tree verification: the outputs in V12 are those of the commands as they were run.
- Resolution reference: docs(issues): [#2525] record the queries behind the observations of the decision in V12
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2605#discussion_r4352225517

### F18 - The ADR is more than one bullet at odds with the decision

- PR number: 2605
- Source review ID: 5619147885
- Reviewer finding ID: F18
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2605#discussion_r4352159467>
- Concern: the decision section says that the ADR has "one bullet" at odds with it, but the ADR also rejects "Leave required checks off and rely on the maintainer" as not scaling, which this decision adopts, and draws consequences from a required check at three more places. Add a dated note to the ADR that points at the decision, or make "whether to supersede it" a tracked task.
- Solution: the decision section states what the ADR keeps and each way in which it is at odds, with its lines, and the question of annotating or superseding it is task T9. The ADR is not changed in this pull request, without the maintainer's approval.
- Current-tree verification: the five ADR lines were read before the text was written.
- Resolution reference: docs(issues): [#2525] state how the T1 ADR is at odds with the decision
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2605#discussion_r4352225838

### F19 - The follow-ups are tracked nowhere and the closing instructions still describe dropped work

- PR number: 2605
- Source review ID: 5619147885
- Reviewer finding ID: F19
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2605#discussion_r4352159474>
- Concern: "tracked in two new issues, which are not created yet" tracks them nowhere. Step 6 of the delivery sequence still says to create the T8(b) measurement issue, the acceptance verification table still lists AC1 to AC3 and AC5 to AC7 as `TODO`, no task creates the two new issues or settles the ADR question, and the evidence says "None" for follow-ups.
- Solution: task T9 covers creating the two issues, settling the ADR, and closing the specification. Step 6 is rewritten, the six rows are `DROPPED`, the decision section points to T9, and the evidence's follow-up section names T9. A dated entry of the progress log that says T8(b) remains was left, because it was true when written.
- Current-tree verification: a search for T8(b) in the specification finds only dated log entries and the statement of what is dropped.
- Resolution reference: docs(issues): [#2525] add the follow-up task T9 and mark the dropped criteria and close step
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2605#discussion_r4352226184

### F20 - The last update of the evidence file is not current

- PR number: 2605
- Source review ID: 5619147885
- Reviewer finding ID: F20
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2605#discussion_r4352159488>
- Concern: the `last-updated-utc` of the evidence file is 2026-10-20 17:52, although this pull request adds V10 and corrections of later dates; set it to the newest edit.
- Solution: the field is 2026-10-22 18:47. The edit went into the commit of F19 and not into a commit of its own, because the file edit ran before the commit command; the history was not rewritten to split it.
- Current-tree verification: the field equals the time of the last edit of the file.
- Resolution reference: docs(issues): [#2525] add the follow-up task T9 and mark the dropped criteria and close step
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2605#discussion_r4352226633

### F22 - The retrospective says that every reply followed its push

- PR number: 2605
- Source review ID: 5619147885
- Reviewer finding ID: F22
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2605#discussion_r4352159499>
- Concern: the claim no longer holds: the F16 reply cites two commits and was posted on a review tied to the earlier head, before those commits reached the pull request. Qualify the sentence or log the exception.
- Solution: the sentence states the exception. The time at which the commits reached the pull request is taken from the reviewer's capture and was not checked independently.
- Current-tree verification: the sentence was read after the edit.
- Resolution reference: docs(pr-reviews): qualify the claim that every reply followed its push in the retrospective
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2605#discussion_r4352226929

### F23 - The count of 195 against 59 has a known cause

- PR number: 2605
- Source review ID: 5619454221
- Reviewer finding ID: F23
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2605#discussion_r4352391556>
- Concern: the 59 against 195 in V12 has a known cause, the caveat in the baseline evidence of #2507 that the GraphQL `totalCount` of a typed timeline query counts every timeline item, so the 59 nodes are every commit of the closed pull request and the table reads as a sample when it is not. Cite the caveat, in place of "which was not investigated".
- Solution: V12 cites the caveat with a link and says that the 59 nodes are the whole set. The audit's F17 entry was corrected to say the same.
- Current-tree verification: the explanation was checked before it was used: `commits { totalCount }` of #2610 is 59, and `timelineItems` returns 195 with and without the type filter.
- Resolution reference: docs(issues): [#2525] explain the count of 195 in V12 with the known caveat
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2605#discussion_r4352419808

### F24 - The claim of one exception no longer holds

- PR number: 2605
- Source review ID: 5619454221
- Reviewer finding ID: F24
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2605#discussion_r4352391575>
- Concern: "with one exception" no longer holds, because the reply on F17 cites a commit but was posted on a review tied to the earlier head, and the deployment time that was cited is only a latest bound. State the cause, that a reply posted within seconds of a push can land on the earlier head, in place of a count.
- Solution: the retrospective states the cause and the two cases, and no time. Before the replies of this round, the head that GitHub reports was polled until it matched the pushed head, and a pause was added.
- Current-tree verification: the review of each reply names its commit, and the replies of this round are on the head that holds the commits they cite.
- Resolution reference: docs(pr-reviews): state the cause of a reply landing on the earlier head in the retrospective
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2605#discussion_r4352420107

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
- 2026-10-21 16:14 UTC - Copilot's sixth review arrived with two threads. F9, the saved logs against the saved
  workflow, was confirmed, and reconciling them also showed an error in my text. F10 is the claim of F4 a fourth
  time; the file was fetched again from the GitHub contents API and no line starts with a doubled pipe.
- 2026-10-21 16:21 UTC - Saved the first revision of the workflow, corrected the evidence, recorded the round in
  the specification, pushed, replied on both threads, and added F9 and F10 to this record.
- 2026-10-21 16:40 UTC - Copilot's seventh review arrived with one thread, F11, the claim of F4 a fifth time. The
  file was fetched from the GitHub contents API at the commit that review read, and no line starts with a
  doubled pipe.
- 2026-10-21 16:52 UTC - Replied `Superseded by F4: <reason>.` on the thread, added F11 to this record, and did
  not request another review of this pull request: the finding had already been answered with evidence twice
  and the maintainer was asked whether to resolve such threads without a further review.
- 2026-10-22 18:10 UTC - Correction of two entries above, found by a reviewer and checked against the commit times and the reply
  times. The 11:53 entry says that the round was pushed, but its last commit was made at 11:54:43 UTC. The 11:56 entry
  reports the replies and the start of this record, but the replies were posted at 11:58:17 and 11:58:20 UTC and
  the first commit of this record was made at 12:00:31 UTC. Both entries are left as written, so that the record
  shows the slip, and this entry gives the true times.
- 2026-10-22 18:15 UTC - Found a review that I had not seen: the second reviewer's of 2026-10-21 18:52 UTC, with five threads, F12 to
  F16, and an ACK of the head of that time, because my watchers had covered only the other pull request. Checked each
  against the evidence. F12 was a real error of mine in saving the logs, shown by comparing them with the logs still on
  GitHub. Fixed F12 to F15 in one signed commit each. F16 (whether the retrospective should be split by pull request)
  is left to the maintainer. In the same push, the decision of the maintainer on 2026-10-22 to drop the required gate
  and the override is recorded in the specification, V10 has a correction on the permission field, and the rulesets
  document no longer says the gate is planned. These commits make the ACK of the earlier head stale.
- 2026-10-22 18:28 UTC - Closed pull request #2610 unmerged by the maintainer's confirmation. Answered F16: the concern is addressed by adding
  the #2610 audit as a historical copy and by recomputing the retrospective's figures from the API and git, and the split is
  not done, for the reasons in the reply. The retrospective was updated with the final state, the decision, a classification
  of the 52 findings, and the slips of the later rounds. The second reviewer is re-requested; Copilot is not, because one of
  its false positives came back five times on this pull request.
- 2026-10-22 18:49 UTC - The second reviewer's review of 18:41 UTC and its ACK of the head of that time arrived while the
  watcher was running, with five non-blocking threads. Checked each against the tree and the sources, and fixed all five
  in separate signed commits, except that the `last-updated-utc` edit of F20 went into the commit of F19, because the file
  edit ran before the commit command and the history was not rewritten to split it. F17 needed new evidence, which was
  collected by running the queries again, with their limits stated. F18 was settled by tracking the ADR question as a
  task and not by changing the ADR without the maintainer. Pushed, replied on the five threads, and added F17 to F22 to this
  record. Each fix makes the ACK of that head stale, and the second reviewer is re-requested.
- 2026-10-22 19:08 UTC - The second reviewer's review of 19:05 UTC and its ACK of the head of that time arrived with two Nits,
  F23 and F24, both correct. F23: the explanation of the count was checked against the API and the #2507 baseline
  caveat before it was used. F24: the claim of one exception was replaced by its cause. The replies of this round were
  posted only after the head that GitHub reports matched the pushed head, and the reviews that hold them name that
  head. Added F23 and F24 to this record, and the typo of one URL in my first draft of the F24 row was fixed before
  the commit.

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
