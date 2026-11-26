---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .claude/skills/process-pr-review/SKILL.md
---

<!-- skill-link: process-pr-review -->

# PR #3081 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/3081.

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
- Reviewer: the author's GitHub login, for example `da2ce7` or `coderabbitai[bot]`; `unknown`
  when GitHub shows no author
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

Review 6078267704 (`claude[bot]`) has no finding: it only says that automated review is disabled for a pull request from a fork.

| Finding ID | Review finding reference | Reviewer | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | -------- | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-3081-f1 | coderabbitai[bot] | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-3081-f2 | coderabbitai[bot] | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-3081-f3 | coderabbitai[bot] | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F4 | review-finding:pr-3081-f4 | coderabbitai[bot] | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F5 | review-finding:pr-3081-f5 | coderabbitai[bot] | Minor | documentation | ORIGINAL | FIXED | NON_RESOLVABLE |
| F6 | review-finding:pr-3081-f6 | da2ce7 | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F7 | review-finding:pr-3081-f7 | da2ce7 | Minor | documentation | RE_RAISE_OF:F5 | FIXED | RESOLVED |
| F8 | review-finding:pr-3081-f8 | da2ce7 | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F9 | review-finding:pr-3081-f9 | da2ce7 | Nit | correctness | ORIGINAL | FIXED | RESOLVED |
| F10 | review-finding:pr-3081-f10 | da2ce7 | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F11 | review-finding:pr-3081-f11 | coderabbitai[bot] | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F12 | review-finding:pr-3081-f12 | coderabbitai[bot] | Nit (inferred) | documentation | ORIGINAL | FIXED | RESOLVED |
| F13 | review-finding:pr-3081-f13 | da2ce7 | Nit | correctness | ORIGINAL | FIXED | RESOLVED |
| F14 | review-finding:pr-3081-f14 | coderabbitai[bot] | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F15 | review-finding:pr-3081-f15 | coderabbitai[bot] | Minor | maintainability | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - The reserved-key rule can still produce a duplicate key

- PR number: 3081
- Source review ID: 6078339285
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3081#discussion_r4711695467
- Concern: An event can have both `target` and `field_target`; renaming `target` to `field_target` then gives two equal keys.
- Solution: D4 now says the formatter remembers every key written in the record and repeats the prefix `field_` until the name is free, so no key is duplicated and no field is dropped. The D4 row of the Decisions table in `ISSUE.md` says the same.
- Current-tree verification: `grep -n "repeated until the name is" docs/issues/open/2766-2758-daemon-json-logging/decisions.md` finds the rule; `grep -c "field_<name>"` on `decisions.md` and `ISSUE.md` prints 0 for both.
- Resolution reference: docs(issues): [#2766] make the reserved-key rule of the JSON formatter collision-free
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3081#discussion_r4712471417

### F2 - A panic in main exits 101, outside the contract

- PR number: 3081
- Source review ID: 6078339285
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3081#discussion_r4711695478
- Concern: D6 kept Rust's exit code `101` for a panic in `main`, but the contract allows only `0`, `1` and `2`.
- Solution: The code is mapped instead of amending the contract: D6 says `main` catches the unwind of the application future and returns `1` after the hook wrote the record; task panics are unchanged. The combinations table, the note on reserved codes in D5, and the scope, the Decisions table and AC8 of `ISSUE.md` follow. This changes a detail of D6 that the author had added; the maintainer was told before the push.
- Current-tree verification: `grep -n "101" docs/issues/open/2766-2758-daemon-json-logging/*.md` shows three lines in `decisions.md`: two say that `101` is not used and one says that the first version of the decision kept it; `ISSUE.md` has none.
- Resolution reference: docs(issues): [#2766] exit 1, not 101, when a panic unwinds out of main
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3081#discussion_r4712471704

### F3 - The chosen D7 alternative still says trace_style is removed

- PR number: 3081
- Source review ID: 6078339285
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3081#discussion_r4711695486
- Concern: Alternative A of D7 said the "Changes in 4.0.0" section would state "JSON only, `trace_style` removed", which contradicts D1.
- Solution: The text was left from before the maintainer changed D1. It now lists `json` as the new default with the text styles still available, the records that are JSON in every style, no colour codes, and exit `2` for configuration errors.
- Current-tree verification: `grep -c "trace_style. removed" docs/issues/open/2766-2758-daemon-json-logging/decisions.md` prints 0; the row of alternative A contains "with the text styles still available".
- Resolution reference: docs(issues): [#2766] align the planned 4.0.0 changes section with the trace_style decision
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3081#discussion_r4712471990

### F4 - The risk does not say that a text style gives a mixed stream

- PR number: 3081
- Source review ID: 6078339285
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3081#discussion_r4711695497
- Concern: The first risk said `trace_style = "full"` and `2>&1` give back the old lines, without saying that some records stay JSON.
- Solution: The risk now says it is not a text-only stream, names the records that stay JSON, and says the "Changes in 4.0.0" section tells a consumer to handle both. da2ce7 noted in review 6078667320 that the next risk bullet already covered the point; the first bullet is now explicit too.
- Current-tree verification: `grep -n "not a text-only stream" docs/issues/open/2766-2758-daemon-json-logging/ISSUE.md` finds the sentence in Risks and Trade-offs.
- Resolution reference: docs(issues): [#2766] say that a text style still gives a mixed stream
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3081#discussion_r4712472301

### F5 - The EPIC risk still says the trace_style decision is pending

- PR number: 3081
- Source review ID: 6078339285
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3081#pullrequestreview-6078339285
- Concern: Outside-diff comment in the review body: the daemon-logging risk of the EPIC (line 191) said the decision was pending and that the plain styles cannot simply stay.
- Solution: The risk states the decision of 2026-11-26 and how the opt-in fits the contract: an exception for the daemon that the ADR of issue 7 records, with a default configuration that meets the contract.
- Current-tree verification: `grep -n "explicit opt-in" docs/issues/open/2758-cli-output-contract-tools-migration/EPIC.md` finds the risk; `grep -c "cannot simply stay"` on the same file prints 0.
- Resolution reference: docs(issues): [#2766] update the daemon logging risk of the EPIC to the decision
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3081#issuecomment-7201125446

### F6 - The plan edits the contract ADR table that the EPIC reserves for issues 1 and 11

- PR number: 3081
- Source review ID: 6078667320
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3081#discussion_r4711949455
- Concern: The Architectural Decisions section and the T2 note had this issue update the daemon's row in the binary table of the contract ADR; the EPIC and the close-out spec say only issues 1 and 11 edit that table.
- Solution: The table edit is dropped: both places now say this issue does not edit the table and that the close-out issue (#2770) updates the daemon's row. The EPIC and the close-out spec are unchanged.
- Current-tree verification: `grep -n "binary table" docs/issues/open/2766-2758-daemon-json-logging/ISSUE.md` shows two lines, both saying the table is not edited here.
- Resolution reference: docs(issues): [#2766] leave the binary table of the contract ADR to the close-out issue
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3081#discussion_r4712522576

### F7 - The EPIC risk still says the trace_style decision is pending

- PR number: 3081
- Source review ID: 6078667320
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3081#discussion_r4711949464
- Concern: Same concern as F5, raised inline on the new progress-log line of the EPIC.
- Solution: Fixed by the change of F5, which was committed before this review was read.
- Current-tree verification: Same verification as F5.
- Resolution reference: docs(issues): [#2766] update the daemon logging risk of the EPIC to the decision
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3081#discussion_r4712522844

### F8 - A sentence says the follow-up is filed when the maintainer confirms

- PR number: 3081
- Source review ID: 6078667320
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3081#discussion_r4711949470
- Concern: D4 is recorded as confirmed, and the plan files the follow-up in T10, so the sentence left unclear whether the issue exists.
- Solution: The sentence now says the follow-up is filed by task T10, in the fourth pull request, and does not exist yet.
- Current-tree verification: `grep -n "by task T10" docs/issues/open/2766-2758-daemon-json-logging/decisions.md` finds the sentence; `grep -c "when the maintainer confirms this decision"` prints 0.
- Resolution reference: docs(issues): [#2766] say that task T10 files the tracker_client follow-up
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3081#discussion_r4712523063

### F9 - Three example records differ from what the code writes

- PR number: 3081
- Source review ID: 6078667320
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3081#discussion_r4711949472
- Concern: The missing-file example showed a "Loading base configuration" record that the code cannot emit for a file it could not read; the `--help` message lacked clap's first line; the D6 panic example gave `location` as file:line:column.
- Solution: The missing-file example shows only the failure, and the text and the summary row say when the loading record comes first, citing `packages/configuration/src/lib.rs:196-197`; the help message starts with clap's first line; the D6 example gives file:line.
- Current-tree verification: `sed -n 195,197p packages/configuration/src/lib.rs` shows the read before the `info!`; `grep -c "42:9" docs/issues/open/2766-2758-daemon-json-logging/decisions.md` prints 0; `grep -c "Command-line arguments accepted"` on the same file prints 1.
- Resolution reference: docs(issues): [#2766] correct three example records of the decision record
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3081#discussion_r4712523287

### F10 - "already corrects them" reads as if issue #3060 were done

- PR number: 3081
- Source review ID: 6078667320
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3081#discussion_r4711949481
- Concern: The spec of issue #3060 is `planned` with no pull request, so "already corrects" overstated it.
- Solution: The sentence now says "issue #3060 (open, its spec is `planned`) corrects them".
- Current-tree verification: `grep -c "already corrects" docs/issues/open/2766-2758-daemon-json-logging/decisions.md` prints 0.
- Resolution reference: docs(issues): [#2766] say that issue #3060 is open and planned, not done
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3081#discussion_r4712523553

### F11 - The spec does not say whether the failure records bypass trace_filter

- PR number: 3081
- Source review ID: 6079316164
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3081#discussion_r4712502681
- Concern: After the configuration is loaded, `trace_filter` would apply to every record, so a runtime startup failure with the filter at `off` could write no `startup_failure` record, while a configuration failure keeps its record; the spec had to say, for `startup_failure` and `panic`, whether they are exempt and whether the panic hook writes through the subscriber.
- Solution: D1 says that `help`, `usage_error`, `startup_failure` and `panic` ignore `trace_filter`: they are not `tracing` events but records built with the formatter code and written to stderr directly; only `log` records pass through the subscriber and its filter. D6 says the panic hook writes directly. AC3 and AC8 require the records with every `trace_filter`, and manual scenario M2d repeats the failures with the filter off. The author chose the behavior from the maintainer's decision that a supervisor must always be able to read why the process stopped, and told the maintainer.
- Current-tree verification: `grep -n "ignore .trace_filter." docs/issues/open/2766-2758-daemon-json-logging/decisions.md` finds the D1 bullet; `grep -n "M2d" docs/issues/open/2766-2758-daemon-json-logging/ISSUE.md` finds the scenario; `grep -c "with every .trace_filter."` on `ISSUE.md` prints 1.
- Resolution reference: docs(issues): [#2766] write the failure, usage, help and panic records whatever the trace filter is
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3081#discussion_r4712576914

### F12 - The collision example does not say which field is renamed

- PR number: 3081
- Source review ID: 6078339285
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3081#discussion_r4711695467
- Concern: Raised in a reply on the thread of F1 (comment 4712539137, review 6079358212), so the source URL is that thread and the source review is the one that opened it. CodeRabbit accepted the rule and added that the example holds only in one order: `target` becomes `field_field_target` only if `field_target` was written first; otherwise the later `field_target` is the one renamed. The comment has no severity; `Nit` is inferred.
- Solution: D4 states the order: fields are written in the order the event records them and the later field is renamed, with the example reworded to match.
- Current-tree verification: `grep -n "the field that comes later is the one renamed" docs/issues/open/2766-2758-daemon-json-logging/decisions.md` finds the sentence.
- Resolution reference: docs(issues): [#2766] say which field is renamed when two event fields collide
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3081#discussion_r4712609299

### F13 - Read literally, the reserved-key rule renames the fields the formatter takes as its own keys

- PR number: 3081
- Source review ID: 6079585902
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3081#discussion_r4712712642
- Concern: The event's `kind` field and tracing's `message` field are "already in the record" once the formatter has written its keys, so the rule as worded would add `field_kind` and `field_message`. The reviewer said it could wait for the implementation pull request.
- Solution: D4 says the `message` key is the event's own `message` field, so that field is not renamed. `kind` is not exempt, unlike the reviewer's suggestion: since F11 the kinds other than `log` are written directly, so a record from a `tracing` event is always `log`, an event cannot set the kind, and an event field named `kind` is renamed like any other. The sentence "An event sets the kind with a `kind` field" is removed. The first reply on the thread promised the fix for the implementation pull request; the maintainer then asked for it now, and the second reply says so.
- Current-tree verification: `grep -n "so that field is not renamed" docs/issues/open/2766-2758-daemon-json-logging/decisions.md` finds the sentence; `grep -c "An event sets the kind"` on the same file prints 0; `grep -c "event's own .message. field"` on `ISSUE.md` prints 1.
- Resolution reference: docs(issues): [#2766] say that the message field keeps its key and that an event cannot set the kind
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3081#discussion_r4712896421

### F14 - D2 says initialize_global_services only sets the level

- PR number: 3081
- Source review ID: 6079935603
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3081#discussion_r4712931139
- Concern: The consequence of D2 said `initialize_global_services` only sets the level, while the Scope of `ISSUE.md` gives it the level and the style; an implementation that follows D2 alone would never select a text style.
- Solution: The consequence now says it sets the level and the style of the log records from the configuration.
- Current-tree verification: `grep -n "it sets the level and the style" docs/issues/open/2766-2758-daemon-json-logging/decisions.md` finds the sentence; `grep -c "it only sets the level"` on the same file prints 0.
- Resolution reference: docs(issues): [#2766] say that initialize_global_services sets the level and the style
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3081#discussion_r4713726048

### F15 - The test-workflow paragraph omits parts of the test-development loop

- PR number: 3081
- Source review ID: 6079935603
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3081#discussion_r4712931146
- Concern: The paragraph under Commit Points did not state the smallest behavior-focused increment, did not require the design review to be recorded in task evidence, and did not stop for maintainer review after the final increment.
- Solution: The paragraph now carries the loop of `docs/templates/ISSUE.md` for tasks T4 to T7, with the three parts named.
- Current-tree verification: `grep -c "smallest behavior-focused increments" docs/issues/open/2766-2758-daemon-json-logging/ISSUE.md` prints 1; the same paragraph contains "recorded in the task's evidence" and "stop for maintainer review".
- Resolution reference: docs(issues): [#2766] state the full test-development loop for tasks T4 to T7
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3081#discussion_r4713726263

## Processing Log

- 2026-11-26 17:20 UTC - Review 6078267704 (`claude[bot]`) arrived; it has no finding.
- 2026-11-26 17:24 UTC - The maintainer commented `ACK` on the first head.
- 2026-11-26 17:25 UTC - CodeRabbit's review 6078339285 arrived with F1 to F5 (F5 in the review body, outside the diff).
- 2026-11-26 17:55 UTC - da2ce7's review 6078667320 arrived with F6 to F10. It was not seen then: the poller had not been restarted after it reported the maintainer's comment.
- 2026-11-26 18:57 UTC - Committed the fixes for F1 to F5, after the maintainer chose to have them pushed (which makes the `ACK` stale).
- 2026-11-26 19:01 UTC - Pushed the five fix commits.
- 2026-11-26 19:02 UTC - Replied on the four CodeRabbit threads and posted the conversation response for F5 (comment 7201125446).
- 2026-11-26 19:03 UTC - Read da2ce7's review while listing the threads. Resolved the three CodeRabbit threads that were still open (F1, F2, F4); the thread of F3 was already resolved.
- 2026-11-26 19:05 UTC - Committed the fixes for F6, F8, F9 and F10. F7 is a re-raise of F5.
- 2026-11-26 19:09 UTC - Pushed the four fix commits and replied on da2ce7's five threads.
- 2026-11-26 19:09 UTC - Created this record from the template. It was started after the replies were posted, not before the first reply as `process-pr-review` requires; the late start is logged here.
- 2026-11-26 19:10 UTC - Resolved da2ce7's five threads (F6 to F10). A GraphQL listing then showed one unresolved thread, a new CodeRabbit finding from review 6079316164 on the head of the first push; it is processed next.
- 2026-11-26 19:11 UTC - CodeRabbit answered on the threads of F1, F2 and F3 (reviews 6079358212, 6079353833 and 6079343771): it confirmed the three fixes and added F12 on the thread of F1.
- 2026-11-26 19:12 UTC - Committed the fix for F11 and this record (first version, F1 to F10).
- 2026-11-26 19:15 UTC - Pushed both commits and replied on the thread of F11.
- 2026-11-26 19:16 UTC - Committed the fix for F12.
- 2026-11-26 19:17 UTC - CodeRabbit answered on the thread of F11 (review 6079426841): the spec now covers the direct output and the filter-off checks; it asks for nothing more.
- 2026-11-26 19:19 UTC - Pushed the F12 fix and replied on the thread of F1, which CodeRabbit had already resolved.
- 2026-11-26 19:20 UTC - Added F11 and F12 to this record and resolved the thread of F11; its last comment is CodeRabbit's confirmation, which asks for nothing. A GraphQL listing shows no unresolved thread.
- 2026-11-26 19:25 UTC - Pushed the spec log commit and the second version of this record.
- 2026-11-26 19:33 UTC - da2ce7's review 6079585902 arrived: approval, with F13 (a Nit that could wait), followed by his `ACK` of that head.
- 2026-11-26 19:34 UTC - Replied on the thread of F13 that the fix would come with the first implementation pull request, and left the thread open.
- 2026-11-26 19:55 UTC - The maintainer commented `ACK` on that head, after asking in the session for F13 to be fixed now.
- 2026-11-26 19:56 UTC - Committed the fix for F13.
- 2026-11-26 19:59 UTC - Pushed it, which makes both `ACK`s and the approval stale, and replied on the thread of F13 again.
- 2026-11-26 20:00 UTC - Added F13 to this record and resolved its thread. A GraphQL listing shows no unresolved thread.
- 2026-11-26 20:03 UTC - Pushed the third version of this record.
- 2026-11-26 20:04 UTC - CodeRabbit's review 6079935603 arrived with F14 and F15, on the head of the F13 fix.
- 2026-11-26 22:02 UTC - Committed the fixes for F14 and F15, after the maintainer chose a push over carrying them to the first implementation pull request.
- 2026-11-26 22:05 UTC - Pushed both commits.
- 2026-11-26 22:06 UTC - Replied on the threads of F14 and F15.
- 2026-11-26 22:06 UTC - Added F14 and F15 to this record. CodeRabbit resolved both threads itself after the replies; a GraphQL listing shows no unresolved thread. da2ce7 had commented `ACK` on the previous head at 20:10.

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
