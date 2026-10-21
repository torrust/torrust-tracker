---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
    - docs/benchmarking.md
    - docs/issues/closed/2518-http-first-announce-benchmark/ISSUE.md
---

<!-- skill-link: process-pr-review -->

# PR #2519 Review Audit

Source: pull-request reviews and inline review threads for <https://github.com/torrust/torrust-tracker/pull/2519>.

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

Copilot review 5514492175 (submitted 2026-10-14 07:31:55 UTC, before merge) left two inline
findings without a `[<Severity>][<FindingId>]` header, so they are assigned the audit-local IDs
`F1` and `F2` in source order. Their severity is inferred as `Minor` from the "Low severity" badges
in the review overview. The overview repeats the two findings and adds no other request, so it adds
no row.

Cameron's review 5515219215 (`da2ce7`, submitted 2026-10-14 08:38:27 UTC, before merge, state
`CHANGES_REQUESTED`) left five inline findings headed `[<Severity>][<FindingId>]`, `F3` to `F7`.
They do not collide with `F1` and `F2`, so they are used unchanged. The review body summarizes the
same five findings and records checks that found nothing; it adds no other request, so it adds no
row.

Cameron's review 5516004305 (`da2ce7`, submitted 2026-10-14 09:43:40 UTC, before merge, state
`APPROVED`) left three non-blocking inline findings, `F8` to `F10`, which do not collide with earlier
IDs and are used unchanged. The review body verifies the round 2 fixes, summarizes the three
findings, and records checks that found nothing; it adds no row.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | `review-finding:pr-2519-f1` | Copilot | Minor (inferred) | correctness | ORIGINAL | FIXED | RESOLVED |
| F2 | `review-finding:pr-2519-f2` | Copilot | Minor (inferred) | testing | ORIGINAL | FIXED | RESOLVED |
| F3 | `review-finding:pr-2519-f3` | Human | Major | correctness | ORIGINAL | FIXED | RESOLVED |
| F4 | `review-finding:pr-2519-f4` | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F5 | `review-finding:pr-2519-f5` | Human | Minor | testing | ORIGINAL | FIXED | RESOLVED |
| F6 | `review-finding:pr-2519-f6` | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F7 | `review-finding:pr-2519-f7` | Human | Nit | metadata | ORIGINAL | FIXED | RESOLVED |
| F8 | `review-finding:pr-2519-f8` | Human | Minor | testing | ORIGINAL | FIXED | RESOLVED |
| F9 | `review-finding:pr-2519-f9` | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F10 | `review-finding:pr-2519-f10` | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - Compare first-announce results only with each other

- PR number: 2519
- Source review ID: 5514492175
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2519#discussion_r4267966296>
- Concern: AC4 required every first-announce result to be above the repeat-announce
  (`announce_once`) result, but D1 and the Risks section say the two benchmarks use different
  timing strategies and must never be compared. The completion criteria contradicted the design.
- Solution: AC4 now requires only that the 74- and 1000-peer results are above the 0-peer result,
  or that the evidence explains why not, and states that the results are compared only with each
  other, never with `announce_once` (D1).
- Current-tree verification: AC4 in `ISSUE.md` no longer mentions the repeat-announce result, and
  no remaining mention of `announce_once` in `ISSUE.md` asks for a comparison with it: AC4's own
  mention is the exclusion, and the others describe that benchmark or rule the comparison out.
  (Corrected for `review-finding:pr-2519-f6`: the first version listed only three of the six
  mentions.)
- Resolution reference: `docs(issues): [#2518] compare first-announce results only with each other`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2519#discussion_r4268395667>

### F2 - Record each run's Criterion medians in M2

- PR number: 2519
- Source review ID: 5514492175
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2519#discussion_r4267966392>
- Concern: M2 ran the benchmark three times without saying when to read the medians. Criterion
  overwrites `new/estimates.json` on every run, and `docs/benchmarking.md` requires reading the
  median immediately after each run, so M2 could keep only the last run and could not show
  run-to-run variation.
- Solution: M2 now records `uptime` before each run and, immediately after each run, reads the
  median of every swarm size from `target/criterion/http_tracker_first_announce/<N>/new/estimates.json`.
  Its expected result is a table of three medians per size with each run's load average and the
  toolchain (`rustc --version`), stating the variation per size.
- Current-tree verification: the M2 row of the Manual Verification Scenarios table in `ISSUE.md`
  contains these steps and the expected result; `linter markdown` and `linter cspell` pass.
- Resolution reference: `docs(issues): [#2518] record each run's Criterion medians in M2`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2519#discussion_r4268395950>

### F3 - Wire the swarm-registry events the benchmark claims to time

- PR number: 2519
- Source review ID: 5515219215
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2519#discussion_r4268555292>
- Concern: D1 said each timed announce includes the "peer added" event, but the bench helper
  builds `InMemoryTorrentRepository::default()`, whose registry's `event_sender` is `None`, so no
  swarm-registry event is sent. Production builds `Registry::new(stats_event_sender)`. A benchmark
  built on the shared helper would time an insert without the event while its documentation
  claimed otherwise.
- Solution: The maintainer chose to wire the events (decision D4, with the discarded option and its
  pros and cons). Scope, D1, and T1 now require the new context to wire the registry's event sender
  and statistics listener as production does; Out of Scope notes that `announce_once` sends no
  such events either and that its live documentation does not claim them.
- Current-tree verification: `ISSUE.md` has the D4 section, the Scope bullet naming
  `InMemoryTorrentRepository::default()`, D1's first pro citing D4, and the T1 notes; the code
  facts were re-checked in `packages/http-core/benches/helpers/util.rs`,
  `swarm/registry.rs`, `swarm/coordinator.rs`, and `container.rs`.
- Resolution reference: `docs(issues): [#2518] wire the swarm-registry events in the benchmark context`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2519#discussion_r4268909114>

### F4 - State the departure from the rotating pool #2479 sketched

- PR number: 2519
- Source review ID: 5515219215
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2519#discussion_r4268555305>
- Concern: The Background shortened #2479's design to "a bounded set of peers", hiding that this
  specification rejects the rotating pool #2479 and `docs/benchmarking.md` suggested.
- Solution: The Background quotes both sources and states the departure; D1 option E says it is
  the design #2479 sketched; T4 replaces the `docs/benchmarking.md` sentence that still suggests it.
- Current-tree verification: `ISSUE.md` Background, D1 option E, and the T4 row contain these
  statements; `linter markdown` and `linter cspell` pass.
- Resolution reference: `docs(issues): [#2518] state the departure from the rotating pool #2479 sketched`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2519#discussion_r4268909448>

### F5 - Replace the probe's false rationale with a maintained assertion

- PR number: 2519
- Source review ID: 5515219215
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2519#discussion_r4268555329>
- Concern: The disposable probe was justified by "the production code has no public seam reporting
  insert versus update", which is false: `Coordinator::upsert_peer` sends `PeerAdded` or
  `PeerUpdated` through a sender injected by public constructors.
- Solution: The maintainer chose a maintained assertion (decision D5, with the discarded option and
  its pros and cons). After the run the benchmark waits for the swarm statistics listener and
  asserts the added, removed, and updated counts; before measurement it checks one first announce's
  response size. Scope, T1, M1, and the new AC6 are updated; no disposable probe is planned.
- Current-tree verification: `ISSUE.md` has the D5 section, AC6 with its Acceptance Verification
  row, the rewritten M1 row, and the Disposable Verification Scripts section without the probe;
  the metric names were checked in `packages/swarm-coordination-registry/src/statistics/mod.rs`.
- Resolution reference: `docs(issues): [#2518] count inserts with a maintained assertion on the swarm metrics`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2519#discussion_r4268909755>

### F6 - State the F1 verification without an incomplete list

- PR number: 2519
- Source review ID: 5515219215
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2519#discussion_r4268555338>
- Concern: F1's Current-tree verification listed three mentions of `announce_once` in the
  specification, which had six; the F1 thread reply repeats the same sentence.
- Solution: F1's verification now states the conclusion without the list and cites this finding.
  The posted F1 reply is left unchanged; the F6 reply records the correction.
- Current-tree verification: every line `grep -n announce_once` returns on the specification at
  the current head was read; none asks for a comparison with `announce_once`. (Corrected for
  `review-finding:pr-2519-f10`: the first version stated a line count, which was already stale
  at the head it described.)
- Resolution reference: `docs(pr-reviews): [#2518] state the F1 verification without an incomplete list`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2519#discussion_r4268910041>

### F7 - Log the review fixes and update the stamp

- PR number: 2519
- Source review ID: 5515219215
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2519#discussion_r4268555346>
- Concern: `last-updated-utc` and the Progress Log were not updated for the AC4 and M2 rewrites.
- Solution: Added Progress Log entries at 08:18 UTC (F1, F2) and 09:12 UTC (F3 to F7, D4, D5), and
  set `last-updated-utc` to `"2026-10-14 09:12"`.
- Current-tree verification: the two entries and the stamp are in `ISSUE.md`.
- Resolution reference: `docs(issues): [#2518] log the PR #2519 review fixes and update the stamp`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2519#discussion_r4268910346>

### F8 - Apply exact swarm-event counts only when no event can be dropped

- PR number: 2519
- Source review ID: 5516004305
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2519#discussion_r4269194260>
- Concern: D5's exact counts assumed the listener never lags, but the swarm event bus is a
  broadcast channel with a capacity of 65,536 events and the listener skips events after
  `RecvError::Lagged`, so exact counts could fail under long `cargo bench` runs. A receiver also
  sees only events sent after it subscribes.
- Solution: The maintainer chose a lag rule, recorded in D5 with the two discarded rules and their
  pros and cons: exact counts only when the run sent no more swarm events than the channel
  capacity; otherwise `updated == 0` and added and removed not above their expected values. D5 now
  subscribes the listener before the pre-fill; AC6 cites the rule; Risks lists the dropped-event
  risk.
- Current-tree verification: `ISSUE.md` has the "Lag rule" paragraph and discarded rules in D5, the
  subscription order in D5's chosen paragraph, AC6's reference to the rule, and the first Risks
  entry; `CHANNEL_CAPACITY` and the `Lagged` handling were re-checked in
  `packages/events/src/broadcaster.rs` and the swarm listener.
- Resolution reference: `docs(issues): [#2518] apply exact swarm-event counts only when no event can be dropped`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2519#discussion_r4269403951>

### F9 - Name the D5 assertions in the Regression Test Strategy

- PR number: 2519
- Source review ID: 5516004305
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2519#discussion_r4269194291>
- Concern: The Regression Test Strategy named only the `RoutineGuard` and the swarm-size assertion,
  omitting the D5 assertions that also run under `cargo test --benches`.
- Solution: It now names the `RoutineGuard`, the swarm-size assertion, and the D5 swarm-metrics and
  response-size assertions.
- Current-tree verification: the Regression Test Strategy section of `ISSUE.md` names all four
  checks, matching In Scope, T1, and AC6.
- Resolution reference: `docs(issues): [#2518] name the D5 assertions in the regression test strategy`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2519#discussion_r4269404190>

### F10 - State the F6 verification without a line count

- PR number: 2519
- Source review ID: 5516004305
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2519#discussion_r4269194304>
- Concern: The F6 entry said `grep -n announce_once` returned six lines, but the head it described
  returned seven, the same class of error F6 was about.
- Solution: The F6 verification states what was checked without a count and cites this finding.
- Current-tree verification: every `announce_once` line in `ISSUE.md` at the current head was
  re-read after this round's edits; none asks for a comparison with `announce_once`.
- Resolution reference: `docs(pr-reviews): [#2518] state the F6 verification without a line count`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2519#discussion_r4269404385>

## Processing Log

- 2026-10-14 08:15 UTC - Started audit: fetched review 5514492175 and its two unresolved threads
  with the `github-review-threads` tool.
- 2026-10-14 08:20 UTC - Pushed one signed fix commit per finding (F1, F2); the pre-push checks
  passed. Replied on both threads with the disposition, verification, and resolution reference.
- 2026-10-14 08:22 UTC - Recorded this audit; the threads are resolved after it is committed and
  pushed.
- 2026-10-14 09:00 UTC - Started round 2: fetched Cameron's review 5515219215 and its five
  unresolved threads (F3 to F7); asked the maintainer to decide F3 and F5.
- 2026-10-14 09:15 UTC - Pushed one signed fix commit per finding (F3 to F7); the pre-push checks
  passed. Replied on the five threads with the disposition, verification, and resolution reference.
- 2026-10-14 09:18 UTC - Recorded round 2 in this audit; the threads are resolved after it is
  committed and pushed.
- 2026-10-14 09:56 UTC - Started round 3: fetched Cameron's approving review 5516004305 and its
  three unresolved threads (F8 to F10); asked the maintainer to decide F8.
- 2026-10-14 10:05 UTC - Pushed one signed fix commit per finding (F8 to F10) and a Progress Log
  update; the pre-push checks passed. Replied on the three threads.
- 2026-10-14 10:07 UTC - Recorded round 3 in this audit; the threads are resolved after it is
  committed and pushed.

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
