---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
---

<!-- skill-link: process-pr-review -->

# PR #2552 Review Audit

Source: pull-request reviews and inline review threads for <https://github.com/torrust/torrust-tracker/pull/2552>.

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

Round 1: Copilot's review 5545198548 (COMMENTED, 2026-10-16 08:44:03 UTC, on the PR's first head)
carries two inline findings, F1 and F2. Its body is an overview and adds no further row. Copilot
marked F1 with a medium and F2 with a low severity icon, so both severities are inferred.

Round 2: da2ce7's review 5546913953 (APPROVED, 2026-10-16 11:05:05 UTC, on the PR's second head,
the audit's first commit) carries five inline findings. da2ce7 numbered them F1 to F5, which collide
with the rows above, so they are F3 to F7 here and the original number is kept in each detail entry
as the reviewer finding ID. The review body is a verification report. Its only other judgment is a
severity rating of round 1: F1 a Nit and F2 a Minor, the reverse of the icons. The F1 and F2
severities above follow that rating, still marked inferred because it is free prose. Its approval
and its ACK comment (<https://github.com/torrust/torrust-tracker/pull/2552#issuecomment-6214120780>)
name the second head, so they do not cover the commits that fix F3 to F7.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | `review-finding:pr-2552-f1` | Copilot | Nit (inferred) | testing | ORIGINAL | FIXED | RESOLVED |
| F2 | `review-finding:pr-2552-f2` | Copilot | Minor (inferred) | documentation | ORIGINAL | FIXED | RESOLVED |
| F3 | `review-finding:pr-2552-f3` | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F4 | `review-finding:pr-2552-f4` | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F5 | `review-finding:pr-2552-f5` | Human | Nit | metadata | ORIGINAL | FIXED | RESOLVED |
| F6 | `review-finding:pr-2552-f6` | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F7 | `review-finding:pr-2552-f7` | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - `f64::EPSILON` is not a general absolute tolerance for the gauge assertions

- PR number: 2552
- Source review ID: 5545198548
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2552#discussion_r4292605809>
- Concern: the two assertions this PR changed in `packages/udp-server/src/statistics/repository.rs`
  compare the gauge with `1000.0` and `2000.0` using `f64::EPSILON`, the machine epsilon around
  1.0. That is not a generally correct absolute tolerance, so the test could fail on a value that
  is merely very close.
- Solution: compare with an absolute tolerance of `1e-9` instead. `assert_eq!` was not used because
  `clippy::float_cmp` is denied. The unchanged `f64::EPSILON` assertions elsewhere in the file
  predate this PR and were left alone.
- Current-tree verification: `grep -n 1e-9 packages/udp-server/src/statistics/repository.rs` shows
  both assertions, and `cargo test -p torrust-tracker-udp-server --lib statistics::repository`
  passes (16 tests).
- Resolution reference: test(udp-server): [#2244] use an absolute tolerance for the gauge assertions
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2552#discussion_r4293675643>

### F2 - The cast reasons say the cast saturates above 2^53

- PR number: 2552
- Source review ID: 5545198548
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2552#discussion_r4292605882>
- Concern: the `#[expect]` reason of each `whole_total` helper said the `f64` to `u64` cast is
  exact below 2^53 and saturates above it. A float to integer cast saturates only for NaN,
  infinities, negative values, and values beyond `u64::MAX`. Between 2^53 and `u64::MAX` the `f64`
  may already have been rounded by the earlier conversion. The reviewer asked for the same fix in
  the HTTP core and UDP core helpers, which use the same text.
- Solution: the reasons in the HTTP core, UDP core, and UDP server helpers now say the cast is
  exact up to 2^53, that beyond that the `f64` may already be rounded, and that the cast itself
  saturates only above `u64::MAX`.
- Current-tree verification: `grep -rn "saturates above it" packages --include=*.rs` finds nothing,
  the new wording is in all three helpers, and Clippy with `-D warnings` is clean for the three
  packages, so each `#[expect]` still fires.
- Resolution reference: docs(statistics): [#2244] correct the float-to-integer cast reasons
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2552#discussion_r4293675987>

### F3 - Ten inventory rows, and the spec's G1/G2/G4 ID ranges, attach the wrong outcome to their IDs

- PR number: 2552
- Source review ID: 5546913953
- Reviewer finding ID: F1
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2552#discussion_r4293970855>
- Concern: the #2158 inventory IDs are keyed by the 2026-09-15 scan's line anchors. I had assigned
  the outcomes by accessor order instead, so A185-A186, A189-A192, and A195-A198 carried the outcome
  of a different accessor, and the spec's G1, G2, and G4 ID ranges had the same swap. Until fixed,
  acceptance criterion AC1 was not true for these ten entries. The group sizes, the 68 removed and 4
  retained totals, and the code were right.
- Solution: the ten rows now carry the outcome of the accessor their anchor names (A185-A190 the
  aborted, discarded, and banned-requests counters, A191-A192 the banned-IP gauge, A195-A198 the
  announce and scrape averages), and the spec reads G1 = A181-A190 and A199-A222, G2 = A191-A192,
  G4 = A193-A198.
- Current-tree verification: a script over the 44 udp-server IDs from A179 to A222 checks each row's
  evidence text against the reviewer's mapping, which `git show` of the 2026-09-15 scan confirms,
  and finds 0 mismatches. `grep` over the spec shows the three corrected ranges.
- Resolution reference: docs(issues): [#2244] correct the per-ID outcomes of ten inventory entries
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2552#discussion_r4294058629>

### F4 - The reason calls truncation the stats API contract

- PR number: 2552
- Source review ID: 5546913953
- Reviewer finding ID: F2
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2552#discussion_r4293970870>
- Concern: the `averaged_processing_time_ns` reason said the stats API reports whole nanoseconds so
  that truncation is intended. The REST API type documents these fields as an average rounded time
  and nowhere says truncated. `as u64` rounds toward zero, so the claim held only under that
  reading, and an earlier commit body and the evidence repeated it.
- Solution: no behavior change. The reason, the helper's doc comment, the six inventory rows, the
  spec, and the evidence now say that the REST API reports a whole-nanosecond average and that
  `as u64` rounds toward zero. An earlier commit body cannot be reworded without rewriting history.
- Current-tree verification: `grep -rn "documented contract\|stats API reports"` over
  `packages/udp-server`, the spec folder, and the inventory finds nothing, and Clippy with
  `-D warnings` is clean for `torrust-tracker-udp-server`.
- Resolution reference: docs(udp-server): [#2244] say the averaged accessors round toward zero
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2552#discussion_r4294059092>

### F5 - The processing-log stamp is later than the commit that carries it

- PR number: 2552
- Source review ID: 5546913953
- Reviewer finding ID: F3
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2552#discussion_r4293970881>
- Concern: this audit's log stamp (10:40 UTC) was later than its commit (authored 10:37:10 UTC),
  which the audit validator does not compare. The spec's 21:00 and 21:10 entries and the evidence's
  20:55 stamp had the same shape against their commits.
- Solution: the stamps had been estimated instead of read from the clock. This audit is append-only,
  so a correction entry names the rewritten content. The spec and EPIC progress logs got correction
  entries with the real author times, and the `last-updated-utc` of the spec, evidence, and EPIC
  equals the time of the correcting commit.
- Current-tree verification: `grep -n Correction` finds one entry in each of the spec and the EPIC
  and one in this audit's Processing Log, and the three `last-updated-utc` values read
  2026-10-16 11:12.
- Resolution reference: docs(issues): [#2244] correct the progress-log times of the implementation round
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2552#discussion_r4294059408>

### F6 - Evidence details the committed bytes do not reproduce

- PR number: 2552
- Source review ID: 5546913953
- Reviewer finding ID: F4
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2552#discussion_r4293970888>
- Concern: three details of `manual-verification-evidence.md` did not match the committed bytes.
  The recorded red run cites panic lines 1486 and 1496, but the committed tests panic on other
  lines. The preserving section said each log excerpt carries lag warnings, but the after excerpt
  has none. V5 step 2 named no command, and its output was not the preserved script's.
- Solution: V2 keeps the first run, explains the line drift, and adds a mutate-then-restore run of
  the committed tests at the PR head (panics at lines 1300 and 1310, values 100.25 and 100.0001,
  tree restored). The preserving section states that only the before excerpt has lag warnings. The
  V5 sender is saved verbatim as `evidence/m1_stats_check.py` and named in step 2.
- Current-tree verification: `grep -n "1300:13\|1310:13\|m1_stats_check.py"` over the evidence file
  finds the new text, and the evidence folder holds the script.
- Resolution reference: docs(issues): [#2244] make the manual verification evidence reproducible
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2552#discussion_r4294059725>

### F7 - The G3 outcome no longer matches the code or the inventory

- PR number: 2552
- Source review ID: 5546913953
- Reviewer finding ID: F5
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2552#discussion_r4293970896>
- Concern: the spec's G3 row said the processed count is read as `f64` and that only the
  `Duration::as_nanos()` cast is retained. The code reads the count through `whole_total` and keeps
  two statement-level `#[expect]` attributes, as the A178 inventory row says. The simulation the row
  cites had no artifact in the issue folder.
- Solution: the G3 outcome now describes the code, and the simulation is saved as
  `evidence/moving_average_truncation_simulation.py` with its output file and linked from the row.
- Current-tree verification: the G3 row matches `Metrics::recalculate_udp_avg_processing_time_ns`,
  and re-running the saved script prints the figures the row cites (20,175.1 against 40,009.1 ns).
- Resolution reference: docs(issues): [#2244] align the G3 outcome with the code and keep its simulation
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2552#discussion_r4294059998>

## Processing Log

- 2026-10-16 10:40 UTC - Started audit after Copilot's review 5545198548. Fixed F1 and F2 in
  separate commits, pushed them, and replied on both threads.
- 2026-10-16 11:14 UTC - Correction to the entry above: it is stamped 10:40 UTC, but its commit was
  authored at 10:37:10 UTC. The stamp was an estimate, not read from the clock. The fixes (10:33),
  the replies (10:35), and the push all came before the audit commit.
- 2026-10-16 11:14 UTC - Round 2: da2ce7's review 5546913953 (APPROVED and ACK on the second head)
  carried five findings, F3 to F7 here. Fixed each in its own commit, pushed, and replied on the five
  threads. Adopted da2ce7's severity rating of round 1 for F1 and F2.

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
