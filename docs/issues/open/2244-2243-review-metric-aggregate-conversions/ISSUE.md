---
schema-version: 1
doc-type: issue
issue-type: task
status: in-progress
priority: p2
epic: 2243
github-issue: 2244
spec-path: docs/issues/open/2244-2243-review-metric-aggregate-conversions/ISSUE.md
branch: "2244-2243-review-metric-aggregate-conversions"
related-pr: null
last-updated-utc: "2026-10-15 21:10"
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - docs/issues/closed/2158-2003-inventory-existing-clippy-allows/clippy-allow-inventory.md
---

<!-- skill-link: create-issue -->

# Issue #2244 - Review Metric Aggregate Conversions

**Parent EPIC:** #2243 - Review Numeric Conversion Boundaries

## Goal

Review each cast at a metric boundary, decide whether it is correct for the metric's semantics, and
leave every retained cast with a native `reason` or replace it with a clearer expression.

## Background

Two cast directions are involved. Aggregate outputs return `f64` for gauges and averages, and the
tracker casts them to `u64` (`cast_sign_loss`, `cast_possible_truncation`). Gauge inputs cast
`usize` counts to `f64` (`cast_precision_loss`). Many of these are ordinary: a counter is an
integer, but its mean over time is a float, and a count far below `2^53` converts exactly. The
reasons were never recorded, so the review must state, per metric, whether the value is an integer
counter, a gauge, or an average, what its bounds are, and what the cast does at the edges.

## Scope

### In Scope

- #2158 entries A080-A087, A113-A114, A143-A154, A170, A178-A222, and A224-A227.
- Both directions: `f64` aggregate output to integer, and integer count to `f64` gauge input.
- Metric semantics and, where a bound or behavior changes, focused tests in HTTP core, UDP core,
  UDP server, and swarm coordination registry as required by the owned entries.
- For each owned entry: retain with a native `reason`, replace with a clearer conversion, or fix a
  demonstrated defect.
- Apply the shared [Clippy exception decision framework](../../closed/2158-2003-inventory-existing-clippy-allows/clippy-exception-decision-framework.md)
  when deciding whether a retained conversion exception is permanent or temporary.

### Out of Scope

- Protocol-width conversions owned by the wire-validation child issue.
- Domain conversions owned by the domain-contract child issue.
- A change to `torrust-metrics` unless tracker-owned boundaries cannot express the required policy.

## Architectural Decisions

- Related ADRs: None.
- ADRs to create: Create one only if metric aggregate semantics require a repository-wide API policy.

## Design and Ownership Review

Not applicable: this work changes local conversion boundaries and does not introduce child
processes, asynchronous I/O, network readiness, resource cleanup, or reusable test fixtures.

## Conversion Review

### Aggregate API Context

`torrust-metrics` 0.1.0 stores counters as `u64` and gauges as `f64`. Its public aggregate traits
(`Sum`, `Avg` on `MetricCollection`) return `Option<f64>` for both metric kinds, so an exact `u64`
counter sum is converted to `f64` inside the crate and back to `u64` by the tracker. The exact
per-metric integer sum (`Metric<Counter>::sum`) is not reachable from the tracker because the
collection fields are `pub(super)`; `get_counter_value` matches one exact label set and does not
aggregate.

Rust float-to-integer `as` casts saturate: negative values and `NaN` become `0`, values above
`u64::MAX` (including `+inf`) become `u64::MAX`, and fractional parts are truncated toward zero.
Integer-to-`f64` casts are exact up to `2^53`.

### Review Table

| Group | Entries | Source | Semantics and bounds | Edge behavior | Outcome |
| ----- | ------- | ------ | -------------------- | ------------- | ------- |
| G1 Counter totals | A080-A087 | `http-core/src/statistics/metrics.rs` (4 accessors) | Integer `u64` counter sum, round-tripped through `f64`. Non-negative, whole, finite. Exact below `2^53` (~285 years at 1M req/s). | Fractional, negative, and non-finite values cannot occur. Above `2^53` the value is the nearest representable `f64`. | Replace per-accessor casts with one private package helper carrying a permanent `#[expect]` reason (external `f64` interface + whole-counter invariant). |
| G1 Counter totals | A143-A154 | `udp-core/src/statistics/metrics.rs` (6 accessors) | Same as above. | Same as above. | Same package-helper outcome. |
| G1 Counter totals | A181-A190, A199-A222 | `udp-server/src/statistics/metrics.rs` (17 accessors: accepted, processed, aborted, discarded, banned requests and IPv4/IPv6 received, accepted, responses, errors) | Same as above. | Same as above. | Same package-helper outcome. |
| G2 Integer gauge | A191-A192 | `udp-server` `udp_banned_ips_total` | Gauge set from a `usize` banned-IP count; whole, non-negative, bounded by the ban-service map size. | Same as G1. | Use the same package helper; it reads a whole, non-negative `f64`. |
| G3 Moving-average gauge | A178, A179-A180 | `udp-server` `recalculate_udp_avg_processing_time_ns`, `udp_avg_processing_time_ns` | Fractional gauge in nanoseconds; running mean of request processing times. | **Defect:** the previous average is truncated to `u64` before each update, so the running sum loses `(n-1) * fraction` per request. A simulation of the exact arithmetic with 20-60 µs requests reports ~20,175 ns against a true mean of ~40,009 ns after 200k requests ([script](evidence/moving_average_truncation_simulation.py), [output](evidence/moving_average_truncation_simulation-output.txt)); the average ratchets toward the fastest requests and gets stuck. | Fix: read the previous average as `f64` without truncation, and add a red/green regression test. The processed-request count is still read as a `u64` through `whole_total` and converted to `f64`. Retain two statement-level `#[expect(clippy::cast_precision_loss)]` reasons for the two exact conversions: that count, and `Duration::as_nanos()` (`u128`), whose value is far below `2^53` ns (~104 days). |
| G4 Averaged gauges | A193-A198 | `udp-server` `udp_avg_{connect,announce,scrape}_processing_time_ns_averaged` | Mean of per-server moving-average gauges in nanoseconds; non-negative, finite; the REST API exposes whole nanoseconds. | Truncates the sub-nanosecond fraction; `0` when no samples. | Retain rounding toward zero (no behavior change) in one private `averaged_processing_time_ns` helper with an `#[expect]` reason; the REST API documents these fields as an "average rounded time" in whole nanoseconds. |
| G5 Gauge inputs | A113-A114 | `swarm-coordination-registry` `update_inactive_{peers,torrents}_total` | `usize` counts of inactive peers and torrents. | Exact: memory cannot hold `2^53` peers or torrents. | Retain with a permanent `#[expect]` reason. |
| G5 Gauge inputs | A170 | `udp-server/src/banning/event/handler.rs` | `usize` count of banned IPs. | Same as above. | Retain with a permanent `#[expect]` reason, narrowed to the cast expression. |
| G6 Test reads | A224-A227 | `udp-server/src/statistics/repository.rs` test | Gauge read only to assert on it. | Not applicable. | Remove: assert on the `f64` gauge value directly. |

Total: 72 entries (G1 54, G2 2, G3 3, G4 6, G5 3, G6 4).

### Maintainer Decisions

Recorded 2026-10-15 from the implementation-start review with josecelano:

1. Fix the G3 moving-average defect in this issue with a red/green regression test, following the
   `fix-bug` skill for that part.
2. Handle G1 counter totals with one private helper per package (HTTP core, UDP core, UDP server)
   carrying a single `#[expect]` with the invariant reason, not a cast per accessor.
3. File a follow-up in `torrust/torrust-metrics` for an exact `u64` counter-sum API. The helper
   reasons stay **permanent** (external `f64` interface + whole-counter invariant), not temporary,
   so this issue does not depend on an external release; adopting the upstream API later is an
   optional cleanup.
4. Keep the current rounding toward zero to whole nanoseconds for the G4 averaged accessors.
   Implemented through one private helper rather than an attribute per accessor, as for G1.
5. Retain G5 `usize` to `f64` gauge inputs with permanent reasons.
6. Remove G6 test casts by asserting on the `f64` gauge directly.
7. Use `#[expect(..., reason = "...")]` for every retained cast: these pedantic cast lints fire on
   every supported toolchain.

### Bug-Fix Process

The G3 defect follows the [`fix-bug`](../../../../.github/skills/dev/debugging/fix-bug/SKILL.md)
skill:

1. **Analysis.** `recalculate_udp_avg_processing_time_ns` computes
   `new = prev + (x - prev) / n` with `prev` read back through a `u64` cast. Each update drops the
   fraction of `prev`, so the running sum `n * avg` loses `(n - 1) * fraction` per request. Once `n`
   exceeds the spread of processing times, faster-than-average requests lower the average by about
   1 ns each, and slower ones cannot raise it. The REST API `udp_avg_*_processing_time_ns` fields
   report this biased value. The contract is that they report the mean processing time.
2. **Reproduction (Reproduced).** A local tracker answered 50,000 connect requests. The API reported
   10,227 ns while the true mean of the 39,113 averaged processing times was 17,812.4 ns. See
   `manual-verification-evidence.md` V1.
3. **Regression tests (red).** See the Regression Test Strategy and evidence V2.
4. **Fix.** Read the moving-average gauge as `f64`, its native type, with no cast.
5. **Green and recheck.** Evidence V3 and V4: the API reported 13,851 ns for a true mean of
   13,851.4 ns.

### Regression Test Strategy

The boundary is unit tests at the causal seam, `Metrics::recalculate_udp_avg_processing_time_ns` in
`packages/udp-server/src/statistics/metrics.rs` (`processing_time_moving_average` module). The seam
is pure arithmetic over the metric collection, so it is deterministic and needs no clock, network,
or event bus:

- `it_should_keep_the_fractional_part_of_the_previous_average`: processing times 100, 101, 101,
  and 101 ns average to exactly 100.75 ns. The defect gives 100.25 ns.
- `it_should_not_drift_toward_the_fastest_requests_over_many_requests`: 10,000 alternating 100 and
  101 ns requests average to 100.5 ns. The defect gives 100.0001 ns.

The real-tracker reproduction is the like-for-like recheck. It is not a maintained test, because it
depends on wall-clock processing times.

## Implementation Plan

| ID | Status | Task | Notes / Expected Output |
| -- | ------ | ---- | ----------------------- |
| T1 | DONE | Classify aggregate semantics | See the Review Table. |
| T2 | DONE | Judge each cast | See the Review Table and Maintainer Decisions. |
| T3a | DONE | Add G3 regression tests | Recorded red run (evidence V2): 100.25 and 100.0001 instead of 100.75 and 100.5. |
| T3b | DONE | Fix G3 | `fix(udp-server): [#2244] keep the fractional UDP average processing time`. |
| T3c | DONE | Green and like-for-like recheck | Evidence V3 and V4. |
| T3d | DONE | Apply G1, G2, and G4-G6 outcomes per package | One `refactor(...)` commit each for HTTP core, UDP core, UDP server, and swarm coordination registry. |
| T4 | DONE | Reconcile inventory | #2158 inventory rows updated by ID: 68 `Removed`, 4 `Retain` (A113, A114, A170, A178); follow-up and evidence tables updated. |

## Commit Points

| Task | Coherent change set | Commit policy |
| ---- | ------------------- | ------------- |
| T1-T2 | Review table in this specification | Commit after maintainer review of the outcomes. |
| T3 | One package's reasons, alternatives, and tests | Commit after focused validation and required review. |
| T4 | Inventory evidence for that package | Commit after focused validation and required review. |

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/drafts/numeric-conversion-metric-aggregate-review/ISSUE.md`
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue created and issue number added to this spec
- [x] Spec-only PR merged into `develop` before implementation
- [x] Implementation completed
- [x] Automatic verification completed (`linter all`, relevant tests, and any pre-push checks)
- [x] Manual verification scenarios executed and recorded in issue-local `manual-verification-evidence.md`
- [x] Acceptance criteria reviewed after implementation and updated with evidence
- [x] Evidence-based implementation completion review recorded
- [ ] Committer verified spec progress is up to date before commit
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-09-15 14:46 UTC - GitHub Copilot - Drafted from the #2158 metric aggregate design input - Awaiting maintainer review
- 2026-09-16 12:20 UTC - josecelano - Reframed as a review with retain-with-reason as a valid outcome - Chat decision
- 2026-10-15 20:32 UTC - GitHub Copilot - T1-T2 review table recorded; found the moving-average truncation defect (G3); maintainer decisions recorded; synchronized stale checkboxes with the GitHub issue body - Implementation started on branch `2244-2243-review-metric-aggregate-conversions`
- 2026-10-15 20:40 UTC - GitHub Copilot - Filed torrust/torrust-metrics#1 for an exact `u64` counter-sum API (maintainer decision 3) - Helper reasons remain permanent
- 2026-10-15 21:00 UTC - GitHub Copilot - T3-T4 done: G3 defect reproduced, fixed red/green, and rechecked; all 72 entries resolved; inventory reconciled; no retrospective needed because the plan held and the defect fix was within the spec's "fix a demonstrated defect" scope - Awaiting maintainer implementation review and pre-push checks
- 2026-10-15 21:10 UTC - GitHub Copilot - `pre-push.sh` passed - Awaiting maintainer implementation review

## Acceptance Criteria

- [x] Every owned entry has a recorded semantics classification, bounds, and outcome.
- [x] Every retained cast carries a native `reason` stating its invariant.
- [x] Every changed conversion has a focused test for the stated bound or new behavior.
- [x] `linter all` exits with code `0` and relevant package tests pass.

## Verification Plan

### Automatic Checks

- Focused HTTP core, UDP core, UDP server, and swarm-registry tests for changed packages.
- Focused Clippy checks for changed packages.
- `linter all` and required pre-push checks.

### Manual Verification Scenarios

| ID | Scenario | Human-oriented command/steps | Expected Result | Status | Evidence |
| -- | -------- | ---------------------------- | --------------- | ------ | -------- |
| M1 | Inspect metric output | Run a local tracker with metric collection enabled and inspect the affected statistics endpoint or output. | Integer and average metrics match their documented semantics. | DONE | `manual-verification-evidence.md` V5 (M1); V1-V4 cover the G3 defect |

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| ----- | ---------------------- | -------- |
| AC1 | DONE | Review Table above; #2158 inventory rows updated by ID with each outcome. |
| AC2 | DONE | Every retained cast is an `#[expect]` with an invariant reason: the `whole_total` helpers in HTTP core, UDP core, and UDP server, `averaged_processing_time_ns`, two statements in `recalculate_udp_avg_processing_time_ns`, and the three gauge inputs. No `#2244` temporary reason remains (`grep -rn 2244 packages --include=*.rs` is empty). `#[expect]` also fails Clippy if a lint stops firing. |
| AC3 | DONE | G3 (the only behavior change): two regression tests, red then green (evidence V2-V3). G1, G2, and G4 preserve behavior, and existing accessor tests in each package pass. G6 asserts on the `f64` gauge directly. |
| AC4 | DONE | Focused Clippy (`-D warnings`, all targets and features) and the tests of the five affected packages pass. Pre-commit `linter all` passed on every commit. `pre-push.sh` passed (nightly fmt, check, and docs; all tests) on `docs(issues): [#2244] record metric conversion outcomes and verification evidence`. |

## Risks and Trade-offs

- Treating every cast as a defect leads to needless helpers and behavior changes. Retaining a
  documented cast is a complete outcome.
- Saturating, rejecting, rounding, and retaining floating values express different metric contracts.
  Choose behavior per metric semantics rather than applying one conversion helper.

## Implementation Completion Review

- Retrospective: Not needed. The plan held, and the G3 defect fix was within the spec's scope (see progress log).
- Create `implementation-retrospective.md` from `docs/templates/IMPLEMENTATION-RETROSPECTIVE.md` in
  this folder if the work invalidates an assumption, changes design materially, or yields a reusable
  lesson; otherwise add a progress-log entry stating why none is needed.

## References

- Related issues: #2158, torrust/torrust-metrics#1
- Related PRs: None
- Related ADRs: None
