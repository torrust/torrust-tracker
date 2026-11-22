---
pr-number: 2985
pr-url: https://github.com/torrust/torrust-tracker/pull/2985
last-updated-utc: "2026-11-22 09:56"
---

<!-- skill-link: process-pr-review -->

# PR #2985 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/2985.

<!-- Copied verbatim into each audit record after placeholder replacement. -->

## Ownership

The PR author owns this tracked audit record. Reviewers, including repository review agents,
deliver findings through GitHub and have no repository-artifact obligation.

- Post-merge workflow approval: N/A

<!-- Copied verbatim into each audit record. -->

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

Round 3 is da2ce7's APPROVED review `6011512347` (three inline findings, F12 to F14 in his numbering, kept here) on the head `docs(issues): [#2984] record the maintainer decisions on the open questions`.

Round 2 is da2ce7's APPROVED review `6006401735` (three inline findings, F9 to F11 in his numbering, kept here) on the head `docs(pr-reviews): [#2984] add the PR 2985 review audit`.

Round 1 is da2ce7's APPROVED review `6006228624` (six inline findings, F1 to F6 in his numbering) and Copilot's COMMENTED review `6006218464` (two inline findings, F7 and F8 here: his IDs F1 to F6 were kept, so Copilot's take the next ordinals), both on the first head, the commit `docs(issues): add issue specification for #2984`.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | `review-finding:pr-2985-f1` | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F2 | `review-finding:pr-2985-f2` | Human | Minor | testing | ORIGINAL | FIXED | RESOLVED |
| F3 | `review-finding:pr-2985-f3` | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F4 | `review-finding:pr-2985-f4` | Human | Minor | metadata | ORIGINAL | FIXED | RESOLVED |
| F5 | `review-finding:pr-2985-f5` | Human | Nit | metadata | ORIGINAL | FIXED | RESOLVED |
| F6 | `review-finding:pr-2985-f6` | Human | Suggestion | testing | ORIGINAL | FIXED | RESOLVED |
| F7 | `review-finding:pr-2985-f7` | Copilot | Minor (inferred) | formatting | ORIGINAL | NO_ACTION | SUPERSEDED |
| F8 | `review-finding:pr-2985-f8` | Copilot | Minor (inferred) | formatting | ORIGINAL | NO_ACTION | SUPERSEDED |
| F9 | `review-finding:pr-2985-f9` | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F10 | `review-finding:pr-2985-f10` | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F11 | `review-finding:pr-2985-f11` | Human | Nit | metadata | ORIGINAL | FIXED | RESOLVED |
| F12 | `review-finding:pr-2985-f12` | Human | Minor | documentation | ORIGINAL | FOLLOW_UP | OPEN |
| F13 | `review-finding:pr-2985-f13` | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F14 | `review-finding:pr-2985-f14` | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - A DNS failure has no slot in the result record the spec reports it in

- PR number: 2985
- Source review ID: 6006228624
- Reviewer finding ID: da2ce7 F1
- Source URL: https://github.com/torrust/torrust-tracker/pull/2985#discussion_r4653334347
- Concern: `udp::Checks` serializes `remote_addr: SocketAddr`, which an unresolved host cannot supply, so AC3 needs a result-shape change that Out of Scope appeared to exclude.
- Solution: Open Question 1 now lists both options and the representation the failed-check option needs (`Option<SocketAddr>` plus the configured URL, or a placeholder); Out of Scope excludes only changes other than that result-shape change.
- Current-tree verification: `grep -n "Option<SocketAddr>"` on `ISSUE.md` finds Open Question 1 and the scope bullets point to it.
- Resolution reference: `docs(issues): [#2984] name the result representation for an unresolved UDP host`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2985#discussion_r4653368111

### F2 - The `.invalid` test is not network-free

- PR number: 2985
- Source review ID: 6006228624
- Reviewer finding ID: da2ce7 F2
- Source URL: https://github.com/torrust/torrust-tracker/pull/2985#discussion_r4653334349
- Concern: `socket_addrs` uses the system resolver, so a resolver that rewrites name-not-found answers could make a `.invalid` name resolve; the no-network claim is wrong.
- Solution: The claim is removed: the test asserts only that an error is returned, and injecting the resolution step is allowed as an alternative.
- Current-tree verification: Read the Regression Test Strategy section of `ISSUE.md`: it states the limit and does not claim the test is network-free.
- Resolution reference: `docs(issues): [#2984] drop the network-free claim for the unresolved-host test`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2985#discussion_r4653368218

### F3 - The reproduction rows paraphrase stderr instead of recording it

- PR number: 2985
- Source review ID: 6006228624
- Reviewer finding ID: da2ce7 F3
- Source URL: https://github.com/torrust/torrust-tracker/pull/2985#discussion_r4653334353
- Concern: The five wrong-outcome rows summarized stderr; only the control rows quoted records.
- Solution: V1 now has one `text` block per URL with the exit code, stdout size and the `panic` and `runtime_failure` records verbatim, from a rerun on 2026-11-21 at 15:15 UTC with the same commands; the file says the control rows were not rerun.
- Current-tree verification: `grep -c '^```text'` on the evidence file prints 5, one per URL.
- Resolution reference: `docs(issues): [#2984] record the reproduction stderr verbatim`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2985#discussion_r4653368329

### F4 - The artifact under test is not identified

- PR number: 2985
- Source review ID: 6006228624
- Reviewer finding ID: da2ce7 F4
- Source URL: https://github.com/torrust/torrust-tracker/pull/2985#discussion_r4653334356
- Concern: "The commit that contains PR #2956" and "default toolchain" do not identify the build, which the like-for-like recheck needs.
- Solution: The environment section names `develop` commit `7bc30738a`, `rustc 1.100.0` and `cargo 1.100.0`, and says the toolchain of the first run (2026-11-19) was not recorded.
- Current-tree verification: Read the environment section of the evidence file; `git diff 7bc30738a -- console packages` prints nothing.
- Resolution reference: `docs(issues): [#2984] identify the build and toolchain of the reproduction`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2985#discussion_r4653368448

### F5 - `DONE (before) / TODO (after)` is not a template status value

- PR number: 2985
- Source review ID: 6006228624
- Reviewer finding ID: da2ce7 F5
- Source URL: https://github.com/torrust/torrust-tracker/pull/2985#discussion_r4653334361
- Concern: The M1 status used a value outside the template's list, and the final recheck was not its own entry.
- Solution: M1 is `DONE` as the reproduction and M2 is a separate `TODO` row for the final recheck.
- Current-tree verification: Rows M1 and M2 of the Manual Verification Scenarios table in `ISSUE.md`.
- Resolution reference: `docs(issues): [#2984] split the manual scenario into reproduction and recheck rows`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2985#discussion_r4653368570

### F6 - A wrong-scheme test cannot tell a scheme check from the port check

- PR number: 2985
- Source review ID: 6006228624
- Reviewer finding ID: da2ce7 F6
- Source URL: https://github.com/torrust/torrust-tracker/pull/2985#discussion_r4653334362
- Concern: The `udp://` prefix turns `ftp://127.0.0.1:6969` into `udp://ftp://127.0.0.1:6969`, which already fails the no-port check, so the planned test passes with no scheme check.
- Solution: The Regression Test Strategy now says the scheme check reads the raw entry before the prefix and that the test asserts the reason.
- Current-tree verification: Read the Regression Test Strategy section of `ISSUE.md`.
- Resolution reference: `docs(issues): [#2984] state that the scheme check reads the raw entry`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2985#discussion_r4653368707

### F7 - Table rows in `ISSUE.md` start with `||`

- PR number: 2985
- Source review ID: 6006218464
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2985#discussion_r4653325952
- Concern: Copilot reported that each table row starts with `||`, which renders an empty first column.
- Solution: No change: no row starts with `||`. The reply is `Superseded by F7: ...`.
- Current-tree verification: `grep -nP '^\|\|'` on `ISSUE.md` prints nothing at the head after the fixes; every table row starts with a single `|`.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2985#discussion_r4653368805
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2985#discussion_r4653368805

### F8 - Table rows in the evidence file start with `||`

- PR number: 2985
- Source review ID: 6006218464
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2985#discussion_r4653325963
- Concern: Copilot reported the same `||` row prefix in the evidence file.
- Solution: No change, for the same reason as F7. The reply is `Superseded by F8: ...`.
- Current-tree verification: `grep -nP '^\|\|'` on the evidence file prints nothing at the head after the fixes.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2985#discussion_r4653368912
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2985#discussion_r4653368912

### F9 - Reading the raw entry's scheme would reject the domain form without a scheme

- PR number: 2985
- Source review ID: 6006401735
- Reviewer finding ID: da2ce7 F9
- Source URL: https://github.com/torrust/torrust-tracker/pull/2985#discussion_r4653484238
- Concern: `localhost:6969` parses as a URL with scheme `localhost`, so a scheme check on the raw entry would break that allowed form, and AC4 named only `udp://localhost:6969`.
- Solution: The spec says the scheme is detected by the `://` separator, and AC4 lists `localhost:6969`.
- Current-tree verification: Read the Regression Test Strategy sentence and the AC4 line of `ISSUE.md`.
- Resolution reference: `docs(issues): [#2984] say how the scheme is detected and cover localhost:6969 in AC4`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2985#discussion_r4653507315

### F10 - This control row now shows a field that the first run's record left out

- PR number: 2985
- Source review ID: 6006401735
- Reviewer finding ID: da2ce7 F10
- Source URL: https://github.com/torrust/torrust-tracker/pull/2985#discussion_r4653484242
- Concern: The `source` field in the control row was restored without a rerun, while the file said the controls were not rerun.
- Solution: Both controls were rerun with the same binary at 2026-11-21 15:57 UTC and recorded verbatim; the environment section says so.
- Current-tree verification: Read the controls section and the Dates line of the evidence file.
- Resolution reference: `docs(issues): [#2984] rerun the two control cases and record them verbatim`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2985#discussion_r4653507638

### F11 - The spec's `last-updated-utc` and Progress Log did not move with the round-1 fixes

- PR number: 2985
- Source review ID: 6006401735
- Reviewer finding ID: da2ce7 F11
- Source URL: https://github.com/torrust/torrust-tracker/pull/2985#discussion_r4653484247
- Concern: The frontmatter still read 13:15 and the Progress Log had no entry for the edits or the rerun.
- Solution: `last-updated-utc` is the update time and the Progress Log has entries for the 15:15 rerun, the round 1 fixes and the round 2 fixes.
- Current-tree verification: Read frontmatter line 12 and the Progress Log of `ISSUE.md`.
- Resolution reference: `docs(issues): [#2984] bring the spec progress log and last-updated up to date`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2985#discussion_r4653507842

### F12 - The decisions cite a record that does not exist, and a session date that disagrees with the log

- PR number: 2985
- Source review ID: 6011512347
- Reviewer finding ID: da2ce7 F12
- Source URL: https://github.com/torrust/torrust-tracker/pull/2985#discussion_r4657538043
- Concern: The decisions section attributed both answers to a chat session dated 2026-11-22 as 'the session that created this issue' and named this PR's review thread as the record, but the thread held no such statement and the issue was created on 2026-11-21; `fix-bug` asks for evidence that does not rely on chat history.
- Solution: The date and source wording is corrected (answered in chat on 2026-11-22, one day after the issue was created; the chat is not a durable record; the decisions are the author's record). The durable record, a PR comment in which the maintainer restates both decisions, is the maintainer's to write; the spec will cite its URL when it exists. The thread stays open until then.
- Current-tree verification: Read the Decisions section of `ISSUE.md`: the wording is corrected and says the maintainer comment is still to be cited.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2985#discussion_r4657554361
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2985#discussion_r4657554361

### F13 - Decision 2 is not carried through the rest of the spec

- PR number: 2985
- Source review ID: 6011512347
- Reviewer finding ID: da2ce7 F13
- Source URL: https://github.com/torrust/torrust-tracker/pull/2985#discussion_r4657538044
- Concern: The Goal, T1, the manual scenarios and Risks still covered only UDP, although decision 2 needs a new HTTP reproduction before the fix and changes working configurations to exit 2.
- Solution: The Goal covers HTTP and health-check URLs; T2 plans the HTTP reproduction; tasks and commit points are renumbered with T5 rechecking V1 and V2; M3 is the HTTP reproduction scenario and M2 rechecks both; Risks names the change to working configurations.
- Current-tree verification: Read the Goal, Implementation Plan, Commit Points, Manual Verification Scenarios and Risks sections of `ISSUE.md`.
- Resolution reference: `docs(issues): [#2984] carry decision 2 through the goal, plan, scenarios and risks`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2985#discussion_r4657554482

### F14 - The PR description still describes the spec as open questions

- PR number: 2985
- Source review ID: 6011512347
- Reviewer finding ID: da2ce7 F14
- Source URL: https://github.com/torrust/torrust-tracker/pull/2985#discussion_r4657538047
- Concern: The PR description said the spec lists two open questions and covered only UDP entries, while the spec records both decisions and adds HTTP and health-check validation.
- Solution: The PR description was edited on 2026-11-22 at 09:38 UTC; its summary records the two decisions, AC5 and that the HTTP reproduction is planned.
- Current-tree verification: `gh pr view 2985 --json body` after the edit shows the new summary.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2985#discussion_r4657554587
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2985#discussion_r4657554587

## Processing Log

- 2026-11-21 15:23 UTC - Round 1: da2ce7's APPROVED review `6006228624` (15:14:29Z, six findings) and Copilot's COMMENTED review `6006218464` (15:12:25Z, two findings, both about a `||` row prefix that is not in the files). Six commits fixed F1 to F6, one per finding (pushed, exit 0, head on GitHub then `docs(issues): [#2984] identify the build and toolchain of the reproduction`). Eight replies were posted between 15:23:24Z and 15:23:40Z (after the push), and this record was written after them, not before (late start).
- 2026-11-21 15:25 UTC - Round 1, continued: the eight threads were resolved after the validator run (`failures: 0`). GraphQL: 0 unresolved.
- 2026-11-21 16:03 UTC - Round 2: da2ce7's APPROVED review `6006401735` (15:55:15Z, three findings F9 to F11). Three commits fixed them, one per finding (pushed, exit 0). Three replies were posted after the push, between 16:02:20Z and 16:02:24Z, and this record was updated after them. Threads resolved at 16:03Z after the validator run (`failures: 0`). GraphQL: 0 unresolved.
- 2026-11-22 09:40 UTC - Round 3: da2ce7's APPROVED review `6011512347` (09:33:14Z, three findings F12 to F14) on the head with the maintainer decisions. Two commits fixed F12's wording and F13 (pushed, exit 0); the PR description was edited for F14; three replies were posted after that, at 09:39Z. F12 is only partly fixed: the maintainer has to restate the decisions in a PR comment, so its thread stays open (FOLLOW_UP/OPEN). F13 and F14 threads are resolved after the validator run.
- 2026-11-22 09:56 UTC - Correction: the round 3 entry stamped 2026-11-22 09:40 UTC was rewritten in place by an earlier commit (`docs(pr-reviews): [#2984] correct the reply times in the PR 2985 audit round 3 entry`), which replaced "at 09:39Z" with "between 09:38:33Z and 09:38:37Z". That entry is restored to its first wording above. The reply times are the second wording: the three round 3 replies were posted between 09:38:33Z and 09:38:37Z, by the capture of the review comments.

<!-- Copied verbatim into each audit record. -->

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
