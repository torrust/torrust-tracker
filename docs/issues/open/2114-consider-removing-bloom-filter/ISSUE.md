---
doc-type: issue
issue-type: task
status: open
priority: p2
epic: null
github-issue: 2114
spec-path: docs/issues/open/2114-consider-removing-bloom-filter/ISSUE.md
branch: "2114-consider-removing-bloom-filter"
related-pr: null
last-updated-utc: 2026-08-29 10:07
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - .github/skills/dev/planning/create-issue/SKILL.md
    - packages/udp-core/src/services/banning.rs
    - packages/udp-server/src/banning/event/handler.rs
    - src/bootstrap/jobs/udp_tracker_server.rs
---

<!-- skill-link: create-issue -->

# Issue #2114 - Evaluate Removing the UDP Bloom Filter

## Goal

Remove the UDP banning service's `bloom` 0.3.2 counting Bloom filter if
reproducible correctness, memory, and performance evidence shows that it adds
no material value. Record the removal decision and its evidence in an ADR so
future maintainers understand why the service does not use a Bloom filter.

## Background

The UDP banning service was introduced in commit `10f9bdaa` to limit repeated
invalid connection-ID requests. That commit described a two-level design:

1. A counting Bloom filter performs a fast, low-memory, probabilistic check.
2. A `HashMap<IpAddr, u32>` verifies the exact count before an address is
   banned, avoiding false bans from Bloom-filter collisions.

The initial commit states that the approach was suitable only when the number
of IPs was low and that IPv6 range ownership needed a different solution. No
benchmark was committed with the feature or later banning-service changes.

The current implementation inserts every invalid-cookie source address into
both data structures in `BanService::increase_counter`. The Bloom filter is
therefore not an admission control for the exact map: a high-cardinality flood
can still grow the `HashMap` until the configured cleanup job clears it. Its
current observable role is to avoid an exact-map lookup when its estimate is at
or below the ban threshold.

The direct runtime dependency declares `GPL-2.0` in its package metadata, while
source-file notices state GPL version 2 or any later version. The dependency
license review in [PR #2113](https://github.com/torrust/torrust-tracker/pull/2113)
records this as requiring qualified legal review. This unresolved licensing risk
is a reason to evaluate removal, but it is not a legal conclusion that removal
is required. This issue must not make a legal compatibility conclusion; it
investigates a technical remediation option.

@da2ce7 (Cameron) is developing a tool in the Torrust Index repository that may
help build bounded filters for spam resistance. Include its design and maturity
in this investigation, but do not assume it satisfies the tracker requirements
until its behavior, performance, memory bounds, and provenance are evaluated.

## Historical Evidence

- `87401e89` added the `bloom` dependency before banning was implemented.
- `10f9bdaa` introduced `BanService`, both counters, and the stated fast-check,
  false-positive, and IPv6 considerations.
- `1299f172` made the ban service shared across UDP trackers; it did not change
  the counter algorithm.
- `1ce2e332` exposed the current exact-map length as the banned-IP total metric.
- `760341fe` added the configurable cleanup job; it clears both counters.
- `547f8484` activated the v3 runtime configuration; it did not change the
  counter algorithm.
- `637c17b1` moved the configurable connection-ID error threshold into UDP
  tracker configuration; it did not change the counter algorithm.

The repository history inspected for Bloom-filter, banning, connection-ID,
cookie-error, false-positive, IPv6, and memory-related commits records no
benchmark and no additional reason for keeping the filter.

The current-source and benchmark-target search also found no existing
Bloom-filter versus exact-map comparison. `udp-core` already has a Criterion
benchmark harness, so this issue can add a focused counter benchmark without
introducing benchmark infrastructure.

This issue addresses one UDP resource-growth factor only. It does not claim to
resolve denial-of-service resilience across the tracker: [Issue #324](https://github.com/torrust/torrust-tracker/issues/324)
tracks separate, open research into HTTP and API idle-connection handling.

## Scope

### In Scope

- Establish a behavioral baseline for invalid-cookie counting, threshold
  enforcement, resets, metrics, and strict versus disabled validation policy.
- Measure the current two-level implementation against a direct exact-map
  lookup with a focused Criterion benchmark. Keep the exact-map-only reference
  implementation benchmark-local; do not introduce a production abstraction
  solely to support measurement.
- Measure allocations and retained memory for many distinct IPv4 and IPv6
  source addresses until the configured cleanup interval.
- Remove `bloom` and simplify the ban service if the measurements show it has
  no material correctness, memory, or performance benefit.
- Create an ADR for a removal decision, including the evidence and the exact
  ban-decision guarantees retained by the direct exact-map design.
- Identify bounded-memory alternatives as follow-up designs only. An
  alternative that permits false negatives must state a measurable rate and be
  approved in its own ADR before implementation.
- Update the dependency-license review after the final disposition is merged.

### Out of Scope

- Declaring `bloom` license-compatible or changing its third-party metadata.
- Copying code from `bloom` into this repository.
- Implementing a new counting Bloom filter in this issue without an approved
  design and provenance review.
- Introducing a false-negative rate as an incidental consequence of removing
  `bloom`; direct exact-map lookup must retain current ban decisions.
- Changing the connection-ID validation policy, ban threshold semantics, or
  cleanup interval solely to make a benchmark favorable.
- Treating an unbounded exact-map implementation as an IPv6 memory-abuse fix.

## Questions to Answer

1. Does the current Bloom-filter pre-check improve `increase_counter` or
   `is_banned` throughput compared with a direct `HashMap<IpAddr, u32>` lookup
   at realistic small, medium, and high exact-map cardinalities?
2. What allocation and retained-memory behavior occurs when many distinct IPv4
   or IPv6 sources each submit invalid connection IDs before cleanup?
3. Is the Bloom filter configuration of four hash functions, one percent false
   positive rate, and 100 expected entries appropriate for observed workloads?
4. Does direct exact-map lookup preserve the current no-false-ban and
   no-false-negative guarantees after the threshold is crossed?
5. If memory bounding remains required, can a future design bound per-source
   exact state while retaining the required ban-decision semantics or an
   explicitly approved false-negative rate?

## Architectural Decisions

The preferred direction is to remove `bloom` when the planned evidence shows
that its pre-check has no material value. A removal must be recorded in an ADR,
including the evidence and the preserved direct exact-map decision semantics.
Any bounded-memory alternative, including one that accepts a false-negative
rate, needs its own approved ADR and follow-up specification before
implementation.

- Related ADRs: None known.
- ADRs to create: Document removal of the Bloom filter in T5 if removal is
  approved. Any alternative design requires a separate ADR.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`, `NOT_APPLICABLE`.

| ID  | Status | Task                                                    | Expected Output                                                                                                                                                                                       |
| --- | ------ | ------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| T1  | TODO   | Record current semantics                                | Tests and a short baseline note covering counters, thresholds, resets, metrics, and policy modes                                                                                                      |
| T2  | TODO   | Add a focused Criterion counter benchmark               | Current two-level service and a benchmark-local exact-map reference measured for `increase_counter` and `is_banned`, using repeated and distinct IPv4/IPv6 sources below, at, and above the threshold |
| T3  | TODO   | Add adversarial-memory measurement                      | IPv4 and IPv6 distinct-source allocation and retained-memory evidence through one cleanup interval                                                                                                    |
| T4  | TODO   | Evaluate Torrust Index filter tooling                   | @da2ce7's candidate tool assessed for semantics, maturity, performance, memory bounds, and provenance                                                                                                 |
| T5  | TODO   | Review removal evidence and record decision             | Maintainer-approved removal decision or documented evidence that blocks removal; removal ADR created when approved                                                                                    |
| T6  | TODO   | Remove the approved dependency and simplify the service | Minimal implementation preserves exact-map ban decisions; alternatives remain follow-up work                                                                                                          |
| T7  | TODO   | Update license review                                   | Link final technical disposition from Issue 269 review material; legal review remains independent if `bloom` remains                                                                                  |

## Progress Tracking

### Workflow Checkpoints

- [x] Spec drafted in `docs/issues/drafts/`
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue [#2114](https://github.com/torrust/torrust-tracker/issues/2114) created and issue number added to this spec
- [ ] (Optional, recommended for complex issues) Spec-only PR merged into `develop` before implementation
- [ ] Implementation completed, when approved
- [ ] Automatic verification completed
- [ ] Manual verification scenarios completed with evidence
- [ ] Acceptance criteria reviewed against observed behavior
- [ ] Reviewer validated acceptance criteria and updated checkboxes
- [ ] Committer verified spec progress is up to date before commit
- [ ] Issue closed and specification moved to `docs/issues/closed/`

### Progress Log

- 2026-08-29 00:00 UTC - Copilot - Created a folder-style draft after history review of the original banning feature and later service, metrics, cleanup, and configuration work.
- 2026-08-29 07:44 UTC - Copilot - Confirmed no existing Bloom-filter versus exact-map benchmark; scoped T2 to a focused `udp-core` Criterion benchmark and benchmark-local reference implementation.
- 2026-08-29 10:07 UTC - User - Approved the issue specification.
- 2026-08-29 10:07 UTC - Copilot - Created GitHub issue [#2114](https://github.com/torrust/torrust-tracker/issues/2114) and promoted this specification to the open-issue lifecycle.

## Acceptance Criteria

- [ ] AC1: The final decision cites reproducible Criterion benchmarks for the
      current two-level service and a benchmark-local exact-map reference. They
      cover `increase_counter` and `is_banned`, repeated and distinct IPv4/IPv6
      sources, threshold boundaries, and different exact-map cardinalities.
- [ ] AC2: The final decision cites a reproducible distinct-source IPv4 and
      IPv6 memory measurement covering behavior before cleanup.
- [ ] AC3: Tests explicitly verify the retained ban-decision guarantees,
      threshold behavior, reset behavior, and strict versus disabled validation
      policy.
- [ ] AC4: No `bloom` code is copied into Torrust Tracker.
- [ ] AC5: If `bloom` is removed, `Cargo.lock` contains no runtime dependency
      path to it, the dependency-license review records the removal, and an ADR
      records the evidence and preserved direct exact-map semantics.
- [ ] AC6: If the evidence does not support removal, the review records why the
      Bloom filter remains and its unresolved license-review status.
- [ ] AC7: A bounded-memory alternative, especially one with false negatives,
      is not implemented without an approved follow-up design, ADR, and stated
      behavior guarantee or maximum false-negative rate.
- [ ] AC8: `linter all` exits with code 0.
- [ ] AC9: Relevant tests pass.
- [ ] AC10: Manual verification scenarios are completed and documented.
- [ ] AC11: Acceptance criteria are re-reviewed after implementation and
      reflect observed behavior.

## Verification Plan

### Automatic Checks

- `cargo test -p torrust-tracker-udp-core`
- Relevant UDP server and integration tests for banning behavior
- `cargo bench -p torrust-tracker-udp-core --bench ban_service_benchmark`
- `linter all`
- Pre-push checks

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID  | Scenario                    | Command/Steps                                                                                                                                           | Expected Result                                                                                           | Status | Evidence |
| --- | --------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------- | ------ | -------- |
| M1  | Baseline ban semantics      | Send invalid-cookie UDP traffic from one source until and beyond the threshold, then reset                                                              | Enforcement begins only at the documented threshold and reset restores access                             | TODO   |          |
| M2  | IPv4 distinct-source memory | Generate documented-volume invalid-cookie requests from distinct IPv4 addresses before cleanup                                                          | Memory and exact-map cardinality are recorded without a crash or uncontrolled test environment growth     | TODO   |          |
| M3  | IPv6 distinct-source memory | Repeat M2 with distinct IPv6 addresses                                                                                                                  | Memory and exact-map cardinality are recorded; results are compared with M2                               | TODO   |          |
| M4  | Counter throughput          | Run `cargo bench -p torrust-tracker-udp-core --bench ban_service_benchmark` with the documented hardware, Rust version, workloads, and Criterion output | Results compare current and exact-map-only counter paths fairly; no unsupported performance claim remains | TODO   |          |
| M5  | Policy compatibility        | Run strict and disabled connection-ID validation scenarios                                                                                              | Existing enforcement and observability behavior is retained unless an approved change states otherwise    | TODO   |          |

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| ----- | ---------------------- | -------- |
| AC1   | TODO                   |          |
| AC2   | TODO                   |          |
| AC3   | TODO                   |          |
| AC4   | TODO                   |          |
| AC5   | TODO                   |          |
| AC6   | TODO                   |          |
| AC7   | TODO                   |          |
| AC8   | TODO                   |          |
| AC9   | TODO                   |          |
| AC10  | TODO                   |          |
| AC11  | TODO                   |          |

## Risks and Trade-offs

- **Incorrect simplification**: removing the filter without measurements could
  regress a hot request path. Mitigate with equivalent benchmarks and retain the
  current implementation until a disposition is approved.
- **Memory-abuse regression**: a direct exact-map design does not improve the
  existing high-cardinality risk. Mitigate with explicit distinct-source IPv4
  and IPv6 measurements and a separately approved bounded-memory design where
  needed.
- **False-ban regression**: relying solely on approximate counts can ban an
  innocent address after a collision. Preserve exact confirmation unless an
  approved design explicitly changes that guarantee.
- **Unstated false-negative trade-off**: bounded-memory alternatives can stop
  tracking some invalid requests. Keep direct exact-map semantics in this issue;
  require a quantified and approved trade-off before any future alternative is
  implemented.
- **Overstated security outcome**: removing `bloom` does not resolve every
  resource-exhaustion path. Keep this issue focused on UDP invalid-cookie
  counting and track distinct concerns, such as Issue 324, independently.
- **Unsupported license conclusion**: this technical investigation does not decide whether
  the existing dependency can legally remain. Keep the Issue 269 finding
  blocked while `bloom` remains in the runtime graph.

## References

- Original implementation: `10f9bdaa` - ban IP after connection-ID errors
- Dependency introduction: `87401e89` - add `bloom`
- Shared-service change: `1299f172` - generic ban service for trackers
- Banned-IP metric: `1ce2e332` - UDP banned IP total
- Cleanup job: `760341fe` - IP-ban cleanup configuration and job
- License-review report: `docs/issues/open/269-review-dependency-licenses/`
- Active license-review PR: [#2113](https://github.com/torrust/torrust-tracker/pull/2113)
- Related DoS research: [#324](https://github.com/torrust/torrust-tracker/issues/324) - HTTP and API idle-connection handling
- Upstream licensing clarification: <https://github.com/nicklan/bloom-rs/issues/11>
