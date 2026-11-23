---
pr-number: 3013
pr-url: https://github.com/torrust/torrust-tracker/pull/3013
last-updated-utc: "2026-11-23 18:36"
---

<!-- skill-link: process-pr-review -->

# PR #3013 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/3013.

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

Round 1 is Copilot's COMMENTED review `6026885441` (18:05:27Z, three inline findings, F1 to F3) on the head `docs(issues): [#2984] record the red and green evidence and mark the implementation tasks done`. Greptile's COMMENTED review `6026494247` (17:40:51Z, one inline finding, F4) came earlier on the same head, but it is not on the poller's reviewer list, so it was found only when the threads were listed before resolving; it is numbered after Copilot's findings for that reason and re-raises F2.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | `review-finding:pr-3013-f1` | Copilot | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F2 | `review-finding:pr-3013-f2` | Copilot | Minor | testing | ORIGINAL | FIXED | RESOLVED |
| F3 | `review-finding:pr-3013-f3` | Copilot | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F4 | `review-finding:pr-3013-f4` | Unknown | Minor (inferred) | testing | RE_RAISE_OF:F2 | NO_ACTION | SUPERSEDED |

## Finding Details

### F1 - The UDP scheme check misreads a query string and is case-sensitive

- PR number: 3013
- Source review ID: 6026885441
- Reviewer finding ID: F1
- Source URL: https://github.com/torrust/torrust-tracker/pull/3013#discussion_r4669911720
- Concern: The raw scheme check rejected `127.0.0.1:6969/announce?next=http://example.com` (it read part of the query as a scheme) and `UDP://127.0.0.1:6969` (case-sensitive comparison).
- Solution: A scheme is recognised only as a leading `<letter><letters, digits, +, -, .>://` token, compared case-insensitively, and the same detection decides whether `udp://` is prepended. Two regression tests cover both forms.
- Current-tree verification: `cargo test -p torrust-tracker-client --lib checker::config` passes (30 tests); with the old logic put back the two new tests fail, with the fix they pass.
- Resolution reference: `fix(tracker-client): [#2984] read the UDP URL scheme only when the entry starts with one`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3013#discussion_r4670013898

### F2 - No test covers the failed-check result or the continuation to later trackers

- PR number: 3013
- Source review ID: 6026885441
- Reviewer finding ID: F2
- Source URL: https://github.com/torrust/torrust-tracker/pull/3013#discussion_r4669911803
- Concern: The resolver-only test does not protect the new failed-check result in `run` or the continuation to later trackers.
- Solution: A deterministic test of `run` with two UDP URLs without a port asserts two failed results that each carry their URL, a null `remote_addr` and exactly one failed `Setup` check; it needs no DNS and no network.
- Current-tree verification: `cargo test -p torrust-tracker-client --lib checks::udp` passes; with the failure branch removed from `run` the new test fails (0 results instead of 2), with the branch it passes.
- Resolution reference: `test(tracker-client): [#2984] cover the failed-check result and the continuation to later trackers`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3013#discussion_r4670014434

### F3 - The test design reviews required by the specification are not recorded

- PR number: 3013
- Source review ID: 6026885441
- Reviewer finding ID: F3
- Source URL: https://github.com/torrust/torrust-tracker/pull/3013#discussion_r4669911867
- Concern: T3 is marked done, but neither the evidence nor the progress log records the prose-first Arrange/Act/Assert design reviews that the Test-development loop requires.
- Solution: Section V5 of `manual-verification-evidence.md` records the review of each increment (Arrange, Act, Assert, helper boundary, outcome), states that the reviews were done after T3 and not during it, and names one weakness left as it is. The progress log names the round.
- Current-tree verification: `grep -n "V5 - Test design reviews" docs/issues/open/2984-tracker-client-check-udp-url-validation/manual-verification-evidence.md` finds the section.
- Resolution reference: `docs(issues): [#2984] record the test design reviews of the implementation`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3013#discussion_r4670014950

### F4 - The DNS failure output is untested (Greptile)

- PR number: 3013
- Source review ID: 6026494247
- Reviewer finding ID: P2 (no ID)
- Source URL: https://github.com/torrust/torrust-tracker/pull/3013#discussion_r4669608687
- Concern: The resolver test never reaches the new branch in `run`, so it cannot catch a missing tracker result, the wrong check name or a missing `remote_addr: null`; the reviewer asks for a test through `run` that checks the failed `Setup` result and that later trackers still run.
- Solution: The same request as F2, which the test of `run` for F2 already satisfies. No further change.
- Current-tree verification: `grep -n "fn it_should_report_a_failed_setup_check_for_every_tracker" console/tracker-client/src/console/clients/checker/checks/udp.rs` finds the test; it asserts the URL, a null `remote_addr`, one failed `Setup` check and two results for two trackers.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/3013#discussion_r4670019997
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3013#discussion_r4670019997

## Processing Log

- 2026-11-23 18:20 UTC - Round 1: Copilot's COMMENTED review `6026885441` (18:05:27Z, three findings F1 to F3). Three commits fixed them, one per finding (pushed, exit 0). Three replies were posted after the push, between 18:14:13Z and 18:14:19Z, and this record was written after them, not before (late start). Greptile's review `6026494247` (17:40:51Z, one finding F4, a re-raise of F2) was found at 18:14Z while listing the threads, because the poller does not watch Greptile; its reply (`Superseded by F2`) was posted at 18:14Z. All four threads were resolved at 18:14Z. The poller's reviewer list has no Greptile entry; later rounds pass it with `--reviewer` so a Greptile review wakes the loop.
- 2026-11-23 18:36 UTC - Correction: the round 1 entry stamped 2026-11-23 18:20 UTC (and the record's `last-updated-utc`) carry a time that was typed, not read from the clock. The entry was written after the replies (18:14:13Z to 18:14:19Z) and before the validator run; the exact minute is not known. The first commit of this record failed the spell check and was redone with the reviewer's name added to the project dictionary.

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
