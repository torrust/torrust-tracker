---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
    - docs/issues/open/2417-2411-verify-http-scrape-info-hash-limit/ISSUE.md
---

<!-- skill-link: process-pr-review -->

# PR #2444 Review Audit

Source: pull-request reviews and inline review threads for
<https://github.com/torrust/torrust-tracker/pull/2444>.

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

Copilot review 5426465610 (round 1, "Lite" effort) left three inline comments without
reviewer-provided IDs or severity brackets. Its overview rates two of them `Medium severity` and
one `Low severity`; following earlier audits, they are recorded as `Minor (inferred)` and
`Suggestion (inferred)`. The review body is an overview only and creates no additional finding.

da2ce7 review 5426677485 (round 1, CHANGES_REQUESTED at the head before this round's fixes) left
seven inline findings with reviewer-provided IDs F1-F7, kept as given. Copilot's findings take the
next audit-local IDs F8-F10 to avoid colliding with them. The review body summarizes the seven
findings and raises no other actionable assertion.

da2ce7 review 5427602231 (round 2, CHANGES_REQUESTED at the round-1 fix head, before this audit
was pushed) verified F1 and F3-F10 and left two inline findings. Its F2 re-raises F2 against the
round-1 rewrite of the M2 sentence; it is recorded as F12 with `RE_RAISE_OF:F2`. Its F11 is kept
as given. The review body summarizes these two and raises no other actionable assertion.

da2ce7 review 5427730462 (round 3, CHANGES_REQUESTED at the head that added this record's round-1
rows) raises no new finding. It confirms the round-1 rows and lists what the record still owed:
rows for F12 and F11 and log entries for rounds 2 and 3, all added in this update. F12 kept the
review blocking until its fix was pushed.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F8 | `review-finding:pr-2444-f8` | Copilot | Minor (inferred) | maintainability | ORIGINAL | FIXED | RESOLVED |
| F9 | `review-finding:pr-2444-f9` | Copilot | Minor (inferred) | security | ORIGINAL | FIXED | RESOLVED |
| F10 | `review-finding:pr-2444-f10` | Copilot | Suggestion (inferred) | maintainability | ORIGINAL | FIXED | RESOLVED |
| F1 | `review-finding:pr-2444-f1` | Human | Major | correctness | ORIGINAL | FIXED | RESOLVED |
| F2 | `review-finding:pr-2444-f2` | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F3 | `review-finding:pr-2444-f3` | Human | Minor | testing | ORIGINAL | FIXED | RESOLVED |
| F4 | `review-finding:pr-2444-f4` | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F5 | `review-finding:pr-2444-f5` | Human | Nit | metadata | ORIGINAL | FIXED | RESOLVED |
| F6 | `review-finding:pr-2444-f6` | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F7 | `review-finding:pr-2444-f7` | Human | Suggestion | correctness | ORIGINAL | FIXED | RESOLVED |
| F12 | `review-finding:pr-2444-f12` | Human | Minor | documentation | RE_RAISE_OF:F2 | FIXED | RESOLVED |
| F11 | `review-finding:pr-2444-f11` | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F8 - Assert at compile time that a packet fits a scrape request

- PR number: 2444
- Source review ID: 5426465610
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2444#discussion_r4193719121>
- Concern: `MAX_SCRAPE_INFO_HASHES` subtracts `SCRAPE_REQUEST_HEADER_SIZE` from `MAX_PACKET_SIZE`; a change to either could underflow the const expression with a hard-to-read compile error.
- Solution: added a `const _: () = assert!(...)` that a packet fits the scrape header plus at least one info hash, with an explanatory message; a limit of 0 would make scrapes useless.
- Current-tree verification: `packages/udp-protocol/src/request.rs` contains the assertion after `SCRAPE_REQUEST_HEADER_SIZE`; `cargo test -p torrust-tracker-udp-protocol --lib` passed (12).
- Resolution reference: `refactor(udp-protocol): [#2417] assert at compile time that a packet fits a scrape`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2444#discussion_r4194539258>

### F9 - Stop collecting scrape info hashes at the cap

- PR number: 2444
- Source review ID: 5426465610
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2444#discussion_r4193719186>
- Concern: the UDP scrape parser collected every info hash, then truncated and copied again, so the abuse-mitigation cap limited the output but not the parsing work or memory.
- Solution: apply `take(max_scrape_info_hashes)` before `collect`, as the HTTP parser does.
- Current-tree verification: `Request::parse_bytes` in `packages/udp-protocol/src/request.rs` collects `chunks.iter().take(max_scrape_info_hashes)`; U1, U2, and W1 passed.
- Resolution reference: `perf(udp-protocol): [#2417] stop collecting scrape info hashes at the cap`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2444#discussion_r4194539519>

### F10 - Pass the whole payload to the UDP parser

- PR number: 2444
- Source review ID: 5426465610
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2444#discussion_r4193719228>
- Concern: `&udp_request.payload[..udp_request.payload.len()]` equals `&udp_request.payload` and adds noise.
- Solution: pass `&udp_request.payload`.
- Current-tree verification: `handle_packet` in `packages/udp-server/src/handlers/mod.rs` calls `Request::parse_bytes(&udp_request.payload, MAX_SCRAPE_INFO_HASHES)`; the handler tests passed (41).
- Resolution reference: `refactor(udp-server): [#2417] pass the whole payload to the UDP parser`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2444#discussion_r4194539853>

### F1 - The client cuts UDP scrape answers above 124 entries and blames the tracker

- PR number: 2444
- Source review ID: 5426677485
- Reviewer finding ID: F1
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2444#discussion_r4193897068>
- Concern: with the client's 74-argument cap removed, `udp scrape` sends any number of hashes, but `UdpClient::receive` read replies into a 1496-byte `MAX_PACKET_SIZE` buffer. A scrape answer is 8 + 12 x N bytes, so from 125 hashes the kernel dropped the rest, the client printed 124 entries, and the warning blamed the tracker.
- Solution: the maintainer chose no client-side limit: the tracker's packet size limits what the tracker reads, not what a client may receive. `UdpClient::receive` now uses a 65,535-byte buffer (`RECEIVE_BUFFER_SIZE`). New test K9 scrapes 125 hashes from a fake tracker that answers all of them.
- Current-tree verification: K9 failed with `left: 124, right: 125` before the fix and passes after it; `cargo test --manifest-path console/tracker-client/Cargo.toml --test tracker_client` passed (8).
- Resolution reference: `fix(tracker-client): [#2417] receive UDP replies larger than the tracker packet size`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2444#discussion_r4194540107>

### F2 - Four spec statements were false

- PR number: 2444
- Source review ID: 5426677485
- Reviewer finding ID: F2
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2444#discussion_r4193897083>
- Concern: ISSUE.md said T9's fake has `silent()` (renamed `start_silent()`), that 74 is how many entries fit in a scrape response (it comes from the request), that M2 remained pending (done), and that no ADR was related (the issue created one).
- Solution: corrected all four; the dated Progress Log entry keeps the old names as history.
- Current-tree verification: ISSUE.md's UDP rationale reads "scrape request fits in the tracker's UDP packet budget: (1496 - 16) / 20", Related ADRs links 20261005124222, the Bug-Fix Process says M2 was recorded (V2), and the T9 cell names `start_silent()`.
- Resolution reference: `docs(issues): [#2417] correct four stale spec statements`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2444#discussion_r4194540364>

### F3 - No ownership, drop-path, or deadline design for the fake trackers

- PR number: 2444
- Source review ID: 5426677485
- Reviewer finding ID: F3
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2444#discussion_r4193897089>
- Concern: create-issue requires an ownership map, lifetime invariants, and deadlines for reusable socket fixtures; the spec had none, and the HTTP fake read an accepted stream with no timeout while its drop joins the thread, so a silent peer could hang a test.
- Solution: the HTTP fake reads each request under a 5 s timeout, and ISSUE.md gained a "Fake Trackers (T8, T9)" section with the ownership map, drop-path invariants, deadlines, and an honest note that no separate checkpoint was held after the first slice.
- Current-tree verification: `REQUEST_READ_TIMEOUT` is set on the accepted stream in `console/tracker-client/tests/common/fake_trackers/http.rs`; the client integration tests passed (11 and 8).
- Resolution reference: `test(tracker-client): [#2417] bound the fake HTTP tracker's request read`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2444#discussion_r4194540681>

### F4 - Recorded pre-push and Task Reviewer runs predated the head

- PR number: 2444
- Source review ID: 5426677485
- Reviewer finding ID: F4
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2444#discussion_r4193897093>
- Concern: the recorded pre-push run and the Task Reviewer pass predated the head's last commit, and the PR body claimed the suite passed on the final branch.
- Solution: reran the full pre-push suite at the head that includes every round-1 code fix and rebased on the latest `develop`; the Progress Log and the PR description record it and state that the Task Reviewer passes predate the round-1 fixes.
- Current-tree verification: pre-push exited 0 before the round-1 push; ISSUE.md's 2026-10-06 10:55 UTC Progress Log entry and the PR body's Validation section say so.
- Resolution reference: `docs(issues): [#2417] log PR #2444 review round 1 and the pre-push rerun`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2444#discussion_r4194541006>

### F5 - Implementer follow-up entries carried reviewer verdicts

- PR number: 2444
- Source review ID: 5426677485
- Reviewer finding ID: F5
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2444#discussion_r4193897097>
- Concern: the Implementation Follow-up entries in `agent-review-reports.md` recorded `REVIEW PENDING` and `REVIEW PASSED`, which read as independent verdicts.
- Solution: appended a Correction entry withdrawing both verdicts and naming the 21:07 UTC Task Reviewer REVIEW PASSED as the standing verdict; earlier entries are unchanged.
- Current-tree verification: the last entry of `agent-review-reports.md` is the 2026-10-06 10:51 UTC Correction.
- Resolution reference: `docs(issues): [#2417] withdraw the verdicts recorded in implementer follow-ups`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2444#discussion_r4194541283>

### F6 - The V4 commands could not be run as written

- PR number: 2444
- Source review ID: 5426677485
- Reviewer finding ID: F6
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2444#discussion_r4193897103>
- Concern: V4 used `${hashes[@]}` without defining it and showed one HTTP command for two probes writing the same files.
- Solution: added both hash-generation loops, one command per probe with its own output files, and the `jq` counting commands.
- Current-tree verification: V4 in `manual-verification-evidence.md` defines `hashes` before each probe and writes `udp-k.*`, `http-k.*`, and `http-74.*` separately.
- Resolution reference: `docs(issues): [#2417] make the V4 commands runnable as written`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2444#discussion_r4194541650>

### F7 - Fewer HTTP scrape files does not always mean truncation

- PR number: 2444
- Source review ID: 5426677485
- Reviewer finding ID: F7
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2444#discussion_r4193897111>
- Concern: a tracker that omits unknown info hashes from `files` makes the HTTP comparison emit `scrape_response_truncated` when nothing was truncated.
- Solution: the maintainer accepts the protocol difference (UDP answers positionally; HTTP may collapse or omit). The warning already says the tracker "may" truncate; the module docs of `unified/scrape.rs` now state that for HTTP, fewer files means truncation or omission.
- Current-tree verification: `console/tracker-client/src/console/clients/unified/scrape.rs` module docs contain "may omit unknown torrents".
- Resolution reference: `docs(tracker-client): [#2417] explain what fewer HTTP scrape files can mean`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2444#discussion_r4194541893>

### F12 - The corrected M2 sentence contradicted the spec's own record

- PR number: 2444
- Source review ID: 5427602231
- Reviewer finding ID: F2
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2444#discussion_r4194650774>
- Concern: the round-1 fix for F2 rewrote the Bug-Fix Process to say M2 was recorded before the final behavior was selected, but the spec records T2 at 12:35 UTC and M2 with the fix applied at 13:20 UTC.
- Solution: the sentence now says the draft planned M2 before the selection, that this was not followed, and that M2 was recorded after T2 with the fix applied (V2).
- Current-tree verification: ISSUE.md's Bug-Fix Process contains "M2 was recorded after T2, with the fix applied (V2)".
- Resolution reference: `docs(issues): [#2417] state that M2 ran after the behavior was selected`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2444#discussion_r4194785032>

### F11 - The Progress Log cited an audit record that did not exist

- PR number: 2444
- Source review ID: 5427602231
- Reviewer finding ID: F11
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2444#discussion_r4194650791>
- Concern: at the reviewed head, ISSUE.md's Progress Log and the PR description cited `docs/pr-reviews/pr-2444-review/PR-REVIEW.md`, which did not exist yet.
- Solution: this audit record was committed and pushed right after the reviewed head, with every round-1 row, reply URL, and fix subject, and a passing validator run.
- Current-tree verification: the file exists; `validate-audit-record.py --pr-number 2444` passes.
- Resolution reference: `docs(pr-reviews): [#2417] audit review round 1 of PR #2444`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2444#discussion_r4194785278>

## Processing Log

- 2026-10-06 10:28 UTC - Fetched Copilot review 5426465610 (three inline threads) and da2ce7 review 5426677485 (seven inline threads) after authoring a local spec commit (10:12) and held its push; restored `related-pr: null`, which that commit had set before cleanup, per the da2ce7 review body; rebased on `develop`.
- 2026-10-06 10:56 UTC - Committed the fixes for F1, F9, F8, F10, F3, F7, F2, F6, F5, and F4 separately (10:35-10:56).
- 2026-10-06 11:10 UTC - Rebased on `develop`, reran pre-push (exit 0), checked for new reviews (none), pushed, updated the PR description, and replied on all ten threads (11:10-11:11).
- 2026-10-06 11:19 UTC - Recorded this audit.
- 2026-10-06 11:35 UTC - Fetched da2ce7 review 5427602231 (round 2: F1 and F3-F10 verified, two new inline findings); resolved the three verified Copilot threads; committed the F12 fix.
- 2026-10-06 11:39 UTC - Pushed the F12 fix and replied on the F12 and F11 threads; recorded round 2.
- 2026-10-06 11:41 UTC - Fetched da2ce7 review 5427730462 (round 3, at the round-1 audit head; no new finding); recorded it.

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
