---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
    - docs/issues/closed/2867-udp-health-check-panics-on-invalid-response/ISSUE.md
---

<!-- skill-link: process-pr-review -->

# PR #2879 Review Audit

Source: pull-request reviews and inline review threads for <https://github.com/torrust/torrust-tracker/pull/2879>.

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

F1 to F3 come from the first da2ce7 review (APPROVED, three Nits, none blocking), on the head ending in `docs(pr-reviews): [#2867] record the deferred second da2ce7 review of PR 2868 as follow-ups`. F4 comes from the first Copilot review (COMMENTED), on the same head; Copilot called it F1, which collides with the audit's F1. F5 and F6 come from the second da2ce7 review (APPROVED, two Nits, none blocking), on the head ending in `docs(pr-reviews): [#2867] log the resolution of the first Copilot review of PR 2879`; da2ce7 called them F10 and F11. F7 to F9 come from the third da2ce7 review (APPROVED, one Minor, one Nit and one Suggestion, none blocking), on the head `Merge torrust/torrust-tracker develop into 2867-udp-health-check-panics-on-invalid-response`; da2ce7 called them F12 to F14.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2879-f1 | Human | Nit | metadata | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-2879-f2 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-2879-f3 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F4 | review-finding:pr-2879-f4 | Copilot | Minor | testing | ORIGINAL | FIXED | RESOLVED |
| F5 | review-finding:pr-2879-f5 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F6 | review-finding:pr-2879-f6 | Human | Nit | metadata | ORIGINAL | FIXED | RESOLVED |
| F7 | review-finding:pr-2879-f7 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F8 | review-finding:pr-2879-f8 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F9 | review-finding:pr-2879-f9 | Human | Suggestion | maintainability | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - Four timestamps are later than the commits that record them

- PR number: 2879
- Source review ID: 5918351027
- Reviewer finding ID: F1
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2879#discussion_r4584471537>
- Concern: The ISSUE.md `last-updated-utc` and progress-log entry (09:55), the evidence file's `last-updated-utc` (09:50) and the PR 2868 audit's last processing-log entry (09:58) name times after the commits that carry them (09:44:30Z and 09:48:27Z).
- Solution: Set the progress-log entry and the evidence stamp to 09:44 and the PR 2868 audit entry to 09:48; added a progress-log entry that records the correction.
- Current-tree verification: `grep -n "09:55\|09:50\|09:58" docs/issues/open/2867-udp-health-check-panics-on-invalid-response/*.md docs/pr-reviews/pr-2868-review/PR-REVIEW.md` prints nothing.
- Resolution reference: docs(issues): [#2867] correct four timestamps written ahead of their commits
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2879#discussion_r4584500325>

### F2 - The T5 red log's test-file line does not match the committed test

- PR number: 2879
- Source review ID: 5918351027
- Reviewer finding ID: F2
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2879#discussion_r4584471560>
- Concern: `evidence/t5-red-run.log` places the probe's panic at `contract.rs:663:34`, but the committed `panic!` is on line 666; the evidence row did not say which test-file state the red run used.
- Solution: The T5 row now says the red run happened before T3's assertion was tightened from five lines to eight, and names both lines.
- Current-tree verification: `grep -n "663" docs/issues/open/2867-udp-health-check-panics-on-invalid-response/manual-verification-evidence.md` prints the T5 row.
- Resolution reference: docs(issues): [#2867] say which test-file state the T5 red run used
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2879#discussion_r4584500687>

### F3 - The retrospective approval record states no original approval time

- PR number: 2879
- Source review ID: 5918351027
- Reviewer finding ID: F3
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2879#discussion_r4584471565>
- Concern: The PR 2868 audit called its approval comment a retrospective record and dated the chat decision only as before the merge, which the process-pr-review skill does not allow here.
- Solution: The Ownership line no longer says retrospective; it states the decision window (07:35:29 to 08:03:34 UTC), the merge time and the comment's creation time.
- Current-tree verification: `grep -n "Post-merge workflow approval" docs/pr-reviews/pr-2868-review/PR-REVIEW.md` prints the line with the window.
- Resolution reference: docs(pr-reviews): [#2867] state when the maintainer approved deferring the PR 2868 findings
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2879#discussion_r4584501060>

### F4 - Release the junk-answering peer when the test ends

- PR number: 2879
- Source review ID: 5918382473
- Reviewer finding ID: F1
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2879#discussion_r4584495261>
- Concern: T3's junk-answering peer ran on a detached native thread blocked in `recv_from`, so the thread and its UDP port lived until the test process exited; neither `env.stop()` nor the runtime's shutdown stopped it.
- Solution: The peer is a `tokio::net::UdpSocket` served by a `tokio::spawn` task of the test's runtime, which its shutdown drops. T3's red (mutate-then-restore) and green runs were repeated, and the evidence says the stored logs are those runs.
- Current-tree verification: `grep -n "std::thread::spawn" packages/axum-health-check-api-server/tests/server/contract.rs` prints nothing; `cargo test -p torrust-tracker-axum-health-check-api-server --test integration` passes.
- Resolution reference: test(axum-health-check-api-server): [#2867] run the T3 junk-answering peer as a task of the test runtime
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2879#discussion_r4584563973>

### F5 - The evidence still describes T3's overwritten runs in three places

- PR number: 2879
- Source review ID: 5918615659
- Reviewer finding ID: F10
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2879#discussion_r4584671897>
- Concern: After T3's runs were redone, the V4 excerpt still quoted the old green time, the build-line note dated the stopped-UDP run to "the T3 green run", and V4 called T3's peer the same socket as V1's; the T5 row gave only the line at the T5 commit.
- Solution: The V4 excerpt quotes the redone green run and names the first run; the build-line note dates the stopped-UDP run to the first T3 green run; V4's initial state says the peer is a runtime task; the T5 row also gives line 670.
- Current-tree verification: `grep -n "finished in 1.01s" docs/issues/open/2867-udp-health-check-panics-on-invalid-response/manual-verification-evidence.md` prints nothing; `grep -n "line 670" docs/issues/open/2867-udp-health-check-panics-on-invalid-response/manual-verification-evidence.md` prints the T5 row.
- Resolution reference: docs(issues): [#2867] align the evidence with the redone T3 runs
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2879#discussion_r4584685534>

### F6 - F4's reviewed head is misattributed

- PR number: 2879
- Source review ID: 5918615659
- Reviewer finding ID: F11
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2879#discussion_r4584671907>
- Concern: The findings paragraph and the processing-log entry named a later head for the Copilot review (taken from the comment's current `commit_id`) and gave the comment's time, not the review's.
- Solution: Both now name the review's `commit_id` head, the one ending in `docs(pr-reviews): [#2867] record the deferred second da2ce7 review of PR 2868 as follow-ups`, and the review time 10:20:41Z.
- Current-tree verification: `grep -n "at 10:20:41Z" docs/pr-reviews/pr-2879-review/PR-REVIEW.md` prints the Copilot processing-log entry, which names the review's head.
- Resolution reference: docs(pr-reviews): [#2867] log the resolution of the second da2ce7 review of PR 2879
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2879#discussion_r4584685917>

### F7 - The merged `develop` removed the stopped-UDP test that V4, AC4 and the description cite

- PR number: 2879
- Source review ID: 5921882653
- Reviewer finding ID: F12
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2879#discussion_r4587180897>
- Concern: The #2393 rewrite merged with `develop` replaced `it_should_return_error_when_udp_service_was_stopped_after_registration` with `it_should_return_error_when_the_registered_udp_service_never_answers_the_check`, but V3, V4 step 2, the AC4 row and the description still named the removed test as maintained, so its filter now matches no test.
- Solution: A new V4 subsection names the replacement as AC4's coverage from the merge on, with a new run log (`evidence/never-answering-udp-run.log`). V3 and V4 step 2 speak of the removed test in the past tense, the AC4 row names the replacement, and the description's Validation bullet was updated.
- Current-tree verification: `grep -n "never_answers_the_check" docs/issues/open/2867-udp-health-check-panics-on-invalid-response/*.md` prints the V3 conclusion, the V4 subsection and the AC4 row; the new log shows `1 passed`.
- Resolution reference: docs(issues): [#2867] record the replacement stopped-UDP test as AC4 coverage after the develop merge
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2879#discussion_r4587243057>

### F8 - The line references moved again with the merge

- PR number: 2879
- Source review ID: 5921882653
- Reviewer finding ID: F13
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2879#discussion_r4587180913>
- Concern: The merge added shared helpers above the tests, so T3's red-run line (520) and T5's `panic!` line (670), given for the head, no longer held there.
- Solution: Both numbers are tied to the commit where they hold (the T3 fixture-change commit) instead of to the head.
- Current-tree verification: `grep -n "520\|line 670" docs/issues/open/2867-udp-health-check-panics-on-invalid-response/manual-verification-evidence.md` prints both, each next to its commit.
- Resolution reference: docs(issues): [#2867] record the replacement stopped-UDP test as AC4 coverage after the develop merge
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2879#discussion_r4587243504>

### F9 - T3 could register through the shared helper the merge brings in

- PR number: 2879
- Source review ID: 5921882653
- Reviewer finding ID: F14
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2879#discussion_r4587180924>
- Concern: T3's helper repeated `registar_with_one_service`, and T3's and T5's `Started::new` calls repeated `start_health_check_api`, both added by the #2393 work.
- Solution: T3 registers through `registar_with_one_service` and starts the API with `start_health_check_api`; its helper only binds the peer (`bind_a_junk_answering_udp_peer`). T5 starts its API with `start_health_check_api` and keeps `register` for its second service. Unused imports were removed.
- Current-tree verification: `cargo clippy -p torrust-tracker-axum-health-check-api-server --tests -- -D warnings` passes; `cargo test -p torrust-tracker-axum-health-check-api-server --test integration` reports 13 passed. A mutate-then-restore run with the `check` fix reverted failed T3 at its cause assertion (`contract.rs:541`); its log was not stored.
- Resolution reference: test(axum-health-check-api-server): [#2867] register T3 and start T5's API through the shared helpers
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2879#discussion_r4587243893>

## Processing Log

- 2026-11-16 10:21 UTC - Started audit. da2ce7 reviewed the head ending in `docs(pr-reviews): [#2867] record the deferred second da2ce7 review of PR 2868 as follow-ups` at 10:18:14Z (APPROVED, three inline threads, all Nits, none blocking), recorded as F1 to F3. Fixed each in its own commit and replied on all three threads before the push.
- 2026-11-16 10:28 UTC - Copilot reviewed the head ending in `docs(pr-reviews): [#2867] record the deferred second da2ce7 review of PR 2868 as follow-ups` at 10:20:41Z (COMMENTED, one inline thread), recorded as F4. Fixed the fixture in its own commit, repeated T3's red and green runs in a docs commit, and replied before the push.
- 2026-11-16 10:40 UTC - da2ce7 reviewed the head ending in `docs(pr-reviews): [#2867] log the resolution of the first Copilot review of PR 2879` at 10:39:03Z (APPROVED, two inline threads, both Nits), recorded as F5 and F6. F5 was fixed in its own commit; F6 is fixed by this audit commit. Replied on both threads before the push.
- 2026-11-16 14:50 UTC - da2ce7 reviewed the head `Merge torrust/torrust-tracker develop into 2867-udp-health-check-panics-on-invalid-response` at 14:44:35Z (APPROVED, three inline threads: one Minor, one Nit, one Suggestion), recorded as F7 to F9. F9 was fixed in a test commit and F7 and F8 in one docs commit; the description's Validation bullet was updated. Replied on all three threads before the push.

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
