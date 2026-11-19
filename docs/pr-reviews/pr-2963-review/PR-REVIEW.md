---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .claude/skills/process-pr-review/SKILL.md
---

<!-- skill-link: process-pr-review -->

# PR #2963 Review Audit

Source: pull-request reviews and inline review threads for
<https://github.com/torrust/torrust-tracker/pull/2963>.

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
- When the current identity cannot resolve a thread (`viewerCanResolve` is `false`), keep the
  disposition established from the current tree and record `Thread state=OPEN`. The reviewer or PR
  author resolves it; permission says nothing about whether the concern was fixed.

## Findings

Round 1 has two reviews at the head whose last commit is
`docs(issues): [#2904] record the task review of T4 and check AC2`:

- `5972182565` by `da2ce7` (APPROVED, 09:11:11 UTC): six inline findings, F1 to F6, with the
  reviewer's own IDs. Its body lists the same six findings and holds no other request, so it adds
  no row.
- `5972197600` by `copilot-pull-request-reviewer[bot]` (COMMENTED, 09:12:18 UTC): an overview
  with "No open issues" and no inline comment. Its note that scenario M3 is still to run restates
  the pull request's own plan and holds no request, so it adds no row.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2963-f1 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-2963-f2 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-2963-f3 | Human | Nit | metadata | ORIGINAL | FIXED | RESOLVED |
| F4 | review-finding:pr-2963-f4 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F5 | review-finding:pr-2963-f5 | Human | Nit | correctness | ORIGINAL | FIXED | RESOLVED |
| F6 | review-finding:pr-2963-f6 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - AC2 is checked on evidence that exists only outside the repository

- PR number: 2963
- Source review ID: 5972182565
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2963#discussion_r4626549863>
- Concern: V2 summarizes the transcripts' `model` values in a table without the recorded lines,
  names no command for reading the transcripts, the metadata file, the request-cache record or the
  JSON result, and does not say which variable `printenv` read or when, so AC2 cannot be
  recomputed from the tree.
- Solution: added `evidence/m1m2-run-records.md` in the specification's folder with the `model`
  lines of both transcripts, the agent's metadata file, the cited fields of the request-cache
  record, the JSON result's model list, the debug-log lines and `printenv CLAUDE_CODE_SUBAGENT_MODEL`,
  each with the command that printed it, where and when it ran, and local path parts replaced.
  V1 and V2 link the file, and V2's conclusion names the `printenv` command, its time and its exit
  code.
- Current-tree verification: `git show HEAD:docs/issues/open/2904-2003-migrate-custom-agents-to-claude-code/evidence/m1m2-run-records.md`
  holds the five sections with their commands; `grep -n "m1m2-run-records" manual-verification-evidence.md`
  finds the links in V1 and V2.
- Resolution reference: docs(issues): [#2904] store the records of the M1 and M2 run in the repository
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2963#discussion_r4628055234>

### F2 - The Progress Log ends before the task review

- PR number: 2963
- Source review ID: 5972182565
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2963#discussion_r4626549877>
- Concern: the last Progress Log entry (08:39) predates the task review recorded at 08:44 and the
  commits it produced; no entry names the review, its verdict or the AC2 check.
- Solution: appended a Progress Log entry stamped 09:18 that names the review of 08:44, its
  verdict and findings, the changes made before the pull request, AC2, what was not changed, and
  that the entry was added in this round.
- Current-tree verification: `grep -n "2026-11-19 09:18 UTC" ISSUE.md` finds the entry as the last
  line of the Progress Log.
- Resolution reference: docs(issues): [#2904] log the task review of T4 in the Progress Log
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2963#discussion_r4628055518>

### F3 - `last-updated-utc` predates the last change to the evidence file

- PR number: 2963
- Source review ID: 5972182565
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2963#discussion_r4626549885>
- Concern: the evidence file's `last-updated-utc` read `2026-11-19 08:38`, but a commit changed the
  file at 08:45.
- Solution: set `last-updated-utc` to `2026-11-19 09:19`, after the round's other changes to the
  file.
- Current-tree verification: the file's `last-updated-utc` reads `2026-11-19 09:19`, and
  `git log -1 --format=%cI` on the file prints `2026-11-19T09:19:07Z`, the commit that sets it.
- Resolution reference: docs(issues): [#2904] update last-updated-utc of the manual verification evidence
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2963#discussion_r4628055944>

### F4 - V1 does not say that the probed agent file changed after the run

- PR number: 2963
- Source review ID: 5972182565
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2963#discussion_r4626549894>
- Concern: the run used the agent file of the first T4 commit, and a later commit changed its
  `description` value and body; V1 does not say so.
- Solution: V1's initial state names the later commit, what it changed, that the keys and the
  values of `name`, `model`, `tools` and `skills` did not change, and that the run did not observe
  the `description`.
- Current-tree verification: lines 52 to 55 of the evidence file hold the added sentences.
- Resolution reference: docs(issues): [#2904] say in V1 that the agent file changed after the run
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2963#discussion_r4628056394>

### F5 - The inventory note on loading `AGENTS.md` is too broad

- PR number: 2963
- Source review ID: 5972182565
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2963#discussion_r4626549907>
- Concern: the note said every agent Claude Code starts loads `AGENTS.md`, while the
  specification's own facts say the built-in Explore and Plan agents skip the instruction files.
- Solution: the note now says that `pr-review-handler`, a project agent, loads `AGENTS.md` through
  the root `CLAUDE.md`, as scenario M4 showed for a started agent.
- Current-tree verification: the `Follow AGENTS.md.` row of `removed-profile-rules.md` reads as
  above; `grep -n "Every agent Claude Code starts" removed-profile-rules.md` finds nothing.
- Resolution reference: docs(issues): [#2904] narrow the inventory note on loading AGENTS.md to the handler
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2963#discussion_r4628056779>

### F6 - The description gives too few rows as not already held

- PR number: 2963
- Source review ID: 5972182565
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2963#discussion_r4626549912>
- Concern: the pull-request description said every rule but two was already held, while the
  inventory has five rows already held, three in the agent file and two dropped.
- Solution: edited the pull-request description at 09:24:08 UTC to give the ten rows as five
  already held, three in the agent file (identity, tools, inputs) and two dropped. The fix is
  outside the repository tree, so its reference is the pull-request comment that states it.
- Current-tree verification: `gh-bot pr view 2963 --json body` shows the corrected sentence; the
  description's edit history shows an edit by `josecelano-bot` at 2026-11-19T09:24:08Z; the
  inventory table has ten rows with those destinations.
- Resolution reference: <https://github.com/torrust/torrust-tracker/pull/2963#issuecomment-7007780542>
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2963#discussion_r4628057104>

## Processing Log

- 2026-11-19 11:28 UTC - Started the audit for review `5972182565` (`da2ce7`, APPROVED) and the
  Copilot overview `5972197600`, at the head whose last commit is
  `docs(issues): [#2904] record the task review of T4 and check AC2`. The fixes of
  F1 to F5 were committed at 09:16 to 09:19 UTC by earlier runs of the `pr-review-handler` agent
  on this event, which stopped before pushing; the description edit of F6 was made at 09:24 UTC.
  The maintainer posted an ACK of that head at 09:24:49 UTC; the
  contributor then asked for the fixes to be pushed. The first push attempt failed in the pre-push
  hook (`clippy-allow-reasons` test `it_should_accept_supported_documented_attribute_shapes`:
  `git init` of a fixture reported "File exists"); `cargo test -p clippy-allow-reasons --test cli`
  then passed (10 of 10), and the second push moved the fork branch at 11:27 UTC to the head whose
  last commit is `docs(issues): [#2904] update last-updated-utc of the manual verification evidence`
  (`git ls-remote`). The F6 comment was posted at 11:28 UTC.
- 2026-11-19 11:29 UTC - Replied on the six threads of F1 to F6 (11:28:52 to 11:29:04 UTC, the
  replies' `created_at`), recorded their URLs, and `validate-audit-record.py` exited 0 (6 rows, 0
  failures). Resolved the six threads at 11:29 UTC; each mutation returned `isResolved: true`, and a
  GraphQL refresh counted 0 unresolved threads.
- 2026-11-19 11:53 UTC - Correction for finding F11 of round 2: rewrote in place the Findings introduction
  and the first log entry above. They named the round-1 head by its full commit id, quoted the
  maintainer's ACK with that id, and named the pushed head by its abbreviated id; they now name each
  head by the subject of its last commit. The six round-1 replies, which cannot be edited, name the
  pushed head by that abbreviated id: it is the head whose last commit is
  `docs(issues): [#2904] update last-updated-utc of the manual verification evidence`.

## Completion Rules

- Re-derive the reply claim against the current tree before replying or resolving a thread.
- Reply on every resolvable thread before resolving it.
- For an outdated thread whose concern was fixed, record `Disposition=FIXED` and
  `Thread state=RESOLVED`, even if GitHub marks the thread outdated after the push. For a
  duplicate, superseded, or no-change in-PR thread, reply exactly
  `Superseded by <FindingId>: <reason>.`, record `Disposition=NO_ACTION` and
  `Thread state=SUPERSEDED`, then resolve it. A post-merge `NO_ACTION` requires maintainer
  approval to decline the follow-up work. When the current identity cannot resolve a thread
  (`viewerCanResolve` is `false`), still reply, keep the disposition established from the current
  tree, and record `Thread state=OPEN`; the reviewer or PR author resolves it.
- A consolidated PR conversation response may cover multiple review rounds only when it names
  every review ID and every finding ID with its disposition and resolution reference. Record its
  durable URL in each related row.
- Cite a fix by its unique Conventional Commit subject or durable reply URL, never by a branch SHA
  that can change after a rebase.
- Refresh review threads using GraphQL and confirm that no unresolved actionable thread remains.
