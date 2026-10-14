---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
    - docs/issues/open/2505-udp-banned-ips-metric/ISSUE.md
    - docs/issues/open/2505-udp-banned-ips-metric/manual-verification-evidence.md
    - docs/issues/open/2505-udp-banned-ips-metric/performance-evidence.md
    - packages/udp-core/src/services/banning.rs
    - packages/udp-server/src/banning/mod.rs
    - packages/udp-server/src/banning/event/handler.rs
---

<!-- skill-link: process-pr-review -->

# PR #2524 Review Audit

Source: pull-request reviews and inline review threads for <https://github.com/torrust/torrust-tracker/pull/2524>.

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

Copilot review 5515830780 (submitted 2026-10-14 09:29:47 UTC) left two inline findings headed
`[<Severity>][F<n>]`; they keep their IDs F1 and F2. Its overview repeats them and adds no request.
Cameron's review 5516040147 (`da2ce7`, CHANGES_REQUESTED, submitted 2026-10-14 09:46:44 UTC, at
the head that was then pushed) left four inline findings headed F1-F4. Those IDs collide with
Copilot's, so they are recorded as audit F3-F6 with his original IDs in the detail entries. His
review body lists the same four findings and adds no other request. It also notes that the PR
description's test table called all of R1 red on `develop`, while two of its four cases are; the
description was corrected without an audit row because the body asks for no change. The body's
references to findings F7-F14 belong to his separate post-merge review of PR #2506, recorded in
`docs/pr-reviews/pr-2506-review/PR-REVIEW.md`.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | `review-finding:pr-2524-f1` | Copilot | Major | correctness | ORIGINAL | FIXED | RESOLVED |
| F2 | `review-finding:pr-2524-f2` | Copilot | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F3 | `review-finding:pr-2524-f3` | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F4 | `review-finding:pr-2524-f4` | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F5 | `review-finding:pr-2524-f5` | Human | Suggestion | correctness | ORIGINAL | FIXED | RESOLVED |
| F6 | `review-finding:pr-2524-f6` | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - Keep the ban state and the gauge update ordered

- PR number: 2524
- Source review ID: 5515830780
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2524#discussion_r4269055532>
- Concern: `banning::reset_bans` and `handle_event` both released the `BanService` lock before
  awaiting the gauge write. On the multi-thread runtime a reset and a concurrent cookie error could
  publish out of order and leave the gauge inconsistent with the ban state. Copilot asked to
  serialize both sequences and add a concurrent regression test.
- Solution: both paths keep the ban service write guard until the gauge is written. They take the
  two locks in the same order (ban service, then gauge store); the launcher's `is_banned` read guard
  is a temporary dropped at the end of its `if` condition, so no path takes them in reverse. Two
  tests hold the gauge store's lock, run the publishing task on a current-thread runtime until it
  blocks, and assert that the ban service is still locked; they observe the lock ordering because
  the out-of-order interleaving cannot be produced deterministically without an injection seam.
- Current-tree verification: `packages/udp-server/src/banning/mod.rs` and
  `packages/udp-server/src/banning/event/handler.rs` drop the guard after
  `update_metric_for_banned_ips_total`; the two `keep_the_ban_service_locked_until` tests were red
  before the fix and green after, stable over five repeats (evidence in
  `manual-verification-evidence.md`, "PR Review Follow-Up - Gauge Publication Ordering").
- Resolution reference: `fix(udp-server): [#2505] publish the banned-IPs total while holding the ban lock`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2524#discussion_r4269636488>

### F2 - Name the gauge in the publication error

- PR number: 2524
- Source review ID: 5515830780
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2524#discussion_r4269055611>
- Concern: the shared helper, now also used by the reset path, logged "Failed to increase the
  counter" although it sets the banned-IPs gauge.
- Solution: the message is now "Failed to set the banned IPs total gauge". The same message in
  `udp-server`'s statistics handlers is left as is, because those handlers do increase counters.
- Current-tree verification: `packages/udp-server/src/banning/event/handler.rs` contains the new
  message once.
- Resolution reference: `fix(udp-server): [#2505] name the gauge in the banned-IPs publication error`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2524#discussion_r4269636783>

### F3 - State the measured baseline noise

- PR number: 2524
- Source review ID: 5516040147
- Reviewer finding ID: F1
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2524#discussion_r4269224628>
- Concern: `performance-evidence.md` stated the baseline Criterion noise as "up to about 3%", but
  by the file's own spread measure it is 5.1%, and AC5 relies on the stated noise.
- Solution: recomputed from the baseline table: as (highest - lowest) / mean the spread is up to
  5.1%, above 3% for 9 of 28 benchmarks, and 3.0% was the largest deviation from the mean. The
  baseline section and the conclusion now state the measure and the figure, and say that attributing
  the after-measurement excesses rests on the same-machine comparison. A new progress-log entry
  records the correction; the earlier entry keeps its wording.
- Current-tree verification: `performance-evidence.md` contains "up to **5.1%**" and the
  attribution paragraph in "Comparison"; `ISSUE.md`'s Progress Log has the correction entry.
- Resolution reference: `docs(issues): [#2505] state the measured P1 Criterion noise`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2524#discussion_r4269637086>

### F4 - Disambiguate the shared short identifiers

- PR number: 2524
- Source review ID: 5516040147
- Reviewer finding ID: F2
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2524#discussion_r4269224646>
- Concern: M1-M5, B1, and B2 each named two different things (manual scenarios and mutations;
  benchmark task and task review finding), and P1/P2 named both performance options and
  measurement phases (late PR #2506 finding F12).
- Solution: added an "Identifiers" glossary to the specification, renamed the measurement phases to
  "baseline" and "after" in the current text of the specification, `performance-evidence.md`, and
  the retrospective, and qualified scenario, mutation, benchmark-task, and task-review citations.
  Mutation labels inside recorded command output and Progress Log entries keep their identifiers; a
  note in `manual-verification-evidence.md` explains them.
- Current-tree verification: `ISSUE.md` has the "Identifiers" section; outside the options section,
  its Progress Log, and the glossary, it no longer uses P1/P2 for the phases;
  `performance-evidence.md` uses P1/P2 only for options.
- Resolution reference: `docs(issues): [#2505] disambiguate the shared short identifiers`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2524#discussion_r4269637330>

### F5 - Stop a wrapping error count from crossing the limit twice

- PR number: 2524
- Source review ID: 5516040147
- Reviewer finding ID: F3
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2524#discussion_r4269224659>
- Concern: in release builds `*count += 1` wraps after `u32::MAX` errors from one IP (reachable in
  `disabled` mode), and the wrapped count would cross the limit again and count an already banned
  IP twice. Cameron suggested `saturating_add`.
- Solution: `increase_counter` uses `checked_add` and returns without counting when the count is at
  `u32::MAX`. `saturating_add` was tried first and rejected: with a limit of `u32::MAX - 1` the
  saturated count equals `limit + 1` on every later error. Two boundary tests seed a count near
  `u32::MAX` through the private map.
- Current-tree verification: `packages/udp-core/src/services/banning.rs` has the `checked_add`
  guard and both boundary tests; mutate-then-restore shows the original `+=` fails both (overflow
  panic in the test profile) and `saturating_add` fails the second (3 instead of 1) (evidence:
  "PR Review Follow-Up - Error Count Overflow").
- Resolution reference: `fix(udp-core): [#2505] stop an IP's error count at the maximum instead of wrapping`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2524#discussion_r4269637555>

### F6 - AC4 evidence overlooked the open EPIC

- PR number: 2524
- Source review ID: 5516040147
- Reviewer finding ID: F4
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2524#discussion_r4269224668>
- Concern: the AC4 evidence said no reference remains outside archived docs, but the open EPIC
  `docs/issues/open/1669-overhaul-packages/EPIC.md` still lists `BanningStats` as an SI-30
  deliverable.
- Solution: the AC4 cell now says that no code reference remains and that the EPIC row records
  history and stays as is; it also names the refactor commit that now carries the removal.
- Current-tree verification: `ISSUE.md`'s Acceptance Verification row AC4 contains "no code
  reference remains" and the EPIC path.
- Resolution reference: `docs(issues): [#2505] correct the contract, origin, and AC4 statements`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2524#discussion_r4269637796>

## Commit IDs Cited by Reviewers

The branch was rewritten after these reviews: an interactive rebase split the leftover removals
out of the banned-count fix (late PR #2506 finding F11), which gave that commit and every later one
a new ID. The IDs cited in the review comments no longer exist on the branch; their commit subjects:

| ID cited in a review | Commit subject |
| -------------------- | -------------- |
| `a8f5fbbbf` | `fix(udp-core): [#2505] count only IPs over the limit as banned` (now preceded by `refactor(udp-core): [#2505] remove the unused BanningStats trait and the write-only reset timestamp`) |
| `4845e5804` | `perf(udp-core): [#2505] detect the ban crossing with one comparison` |
| `60d10af6d` | `test(udp-server): [#2505] make the handler test tell banned IPs from tracked IPs` |
| `c3d2f4a10` | `docs(issues): [#2505] map pre-rebase commit IDs in the review reports` (the head reviewed by Cameron) |

`b5f1801c6`, `d1bf013ea`, and `1ce2e3327` are on `develop` and remain valid.

## Processing Log

- 2026-10-14 09:30 UTC - Started audit: Copilot review 5515830780 arrived; read its two findings and,
  at the maintainer's choice, processed them while waiting for Cameron's review.
- 2026-10-14 09:47 UTC - Cameron's review 5516040147 arrived (CHANGES_REQUESTED, four findings),
  together with his post-merge review of PR #2506, handled under a maintainer approval recorded on
  that pull request.
- 2026-10-14 10:28 UTC - Pushed the fixes for F1-F6 and the late PR #2506 findings, after
  splitting the banned-count fix commit at the maintainer's choice; the pre-push checks passed.
- 2026-10-14 10:30 UTC - Replied on all six threads with the disposition, the current-tree
  verification, and the resolution reference.
- 2026-10-14 10:33 UTC - Recorded this audit; the threads are resolved after it is committed and
  pushed.

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
