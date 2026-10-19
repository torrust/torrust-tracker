---
schema-version: 1
doc-type: issue
issue-type: task
status: in-progress
priority: p3
epic: null
github-issue: 2560
spec-path: docs/issues/open/2560-adopt-torrust-metrics-sum-counter/ISSUE.md
branch: "2560-adopt-torrust-metrics-sum-counter"
related-pr: null
last-updated-utc: "2026-10-19 11:23"
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - .github/skills/dev/maintenance/update-dependencies/SKILL.md
    - .github/skills/dev/planning/create-issue/SKILL.md
    - docs/issues/closed/2158-2003-inventory-existing-clippy-allows/clippy-exception-decision-framework.md
    - docs/issues/closed/2244-2243-review-metric-aggregate-conversions/ISSUE.md
---

<!-- skill-link: create-issue -->

# Issue #2560 - Adopt the exact counter sum from torrust-metrics 0.2.0

## Goal

Upgrade `torrust-metrics` to 0.2.0 in every package that uses it, and read integer counter totals through the new `sum_counter` API, so the tracker no longer converts a `u64` counter through `f64` and back.

## Background

Issue #2244 (PR #2552) reviewed the casts at the tracker's metric boundaries. The public aggregate trait of `torrust-metrics` 0.1.0 returns `Option<f64>` for both counters and gauges, so a `u64` counter sum was converted to `f64` inside the crate and back to `u64` in the tracker. #2244 funneled the tracker's side of that round trip into one private `whole_total` helper per package (HTTP core, UDP core, UDP server). Each helper carries one `#[expect]` whose reason documents the invariant: a counter sum is a finite, non-negative whole number, exact up to 2^53. The reason is deliberately permanent, so #2244 did not depend on an upstream release.

Upstream issue torrust/torrust-metrics#1 asked for an exact `u64` counter sum. It was implemented in torrust/torrust-metrics#2 and released as `torrust-metrics` 0.2.0 (release: <https://github.com/torrust/torrust-metrics/releases/tag/v0.2.0>). The new trait is `torrust_metrics::metric_collection::aggregate::sum::SumCounter`, with `fn sum_counter(&self, &MetricName, &LabelSet) -> Option<u64>`. It returns `None` when the metric does not exist or is a gauge, and its addition is unchecked (it panics on overflow in debug builds and wraps in release builds, like `Metric<Counter>::sum`). `Sum` and `Avg` are unchanged.

Cargo treats `0.1.0` and `0.2.0` as incompatible, and `MetricCollection` appears in public structs that cross package boundaries, so all eight manifests that depend on `torrust-metrics` must move together. A partial bump would put two copies of the crate, and two unrelated `MetricCollection` types, in one build.

One conversion cannot go away. The UDP server's `udp_banned_ips_total` reads a gauge, because the banned-IP count falls when `BanService::reset_bans` clears the bans. `sum_counter` returns `None` for a gauge name, so that accessor keeps an `f64` to `u64` cast. The maintainer asked for the cleanest and most maintainable outcome regardless of effort. The decision is to keep that one cast in one small helper with a permanent reason, for these reasons:

- It is a real gauge, so converting it to a counter would change how the metric behaves for every consumer of the metrics.
- Asking a float-gauge library for an integer gauge accessor would widen its API, and fix a rounding policy, for one consumer.
- After this issue the cast is the only one left in the counter and gauge read path, with one documented reason.

## Scope

### In Scope

- Bump `torrust-metrics` from `0.1.0` to `0.2.0` in the eight manifests: `axum-rest-api-server`, `http-core`, `rest-api-protocol`, `rest-api-runtime-adapter`, `swarm-coordination-registry`, `tracker-core`, `udp-core`, and `udp-server`. Update `Cargo.lock` and check that only expected entries change.
- Replace the `whole_total` helpers of HTTP core and UDP core with `sum_counter`, and remove their `#[expect]` attributes.
- In the UDP server, read counters with `sum_counter` and keep one gauge helper, renamed to say it reads a gauge, with a retained `#[expect]` and a permanent reason for `udp_banned_ips_total` only.
- Add a focused test per package that a counter above 2^53 is returned exactly. The old path rounded it.
- Keep the behavior of every accessor for realistic values, including `0` for a metric that does not exist yet.

### Out of Scope

- `averaged_processing_time_ns` and the moving-average code in the UDP server. They read gauges and averages, which are fractional.
- Changing the banned-IP metric from a gauge to a counter, or adding an integer gauge accessor upstream.
- Rewriting the archived #2158 inventory or the archived #2244 spec. They record history. Their text mentions `whole_total` and torrust/torrust-metrics#1, and that stays accurate as a record.
- Any other dependency update, and any change to `torrust-metrics` itself.

## Architectural Decisions

- Related ADRs: None.
- ADRs to create: None known. This is a dependency bump and a local refactor behind private helpers.

## Design and Ownership Review

Not applicable: this work changes dependency versions and local conversion boundaries, and does not introduce child processes, asynchronous I/O, network readiness, resource cleanup, or reusable test fixtures.

## Bug-Fix Process

Not applicable: this is a dependency adoption and refactor. No defect is being fixed.

## Regression Test Strategy

Not applicable: this is not a bug. The new tests protect the one behavior that differs after the change, the exact value above 2^53, and are described in the Implementation Plan.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID | Status | Task | Notes / Expected Output |
| -- | ------ | ---- | ----------------------- |
| T1 | TODO | Bump `torrust-metrics` to `0.2.0` in the eight manifests and `Cargo.lock` | `cargo build --workspace` passes. `git diff Cargo.lock` is reviewed and records any transitive change, which should be only `torrust-metrics` itself. Follow the `update-dependencies` skill. |
| T2 | TODO | Read counters through `sum_counter` in HTTP core and UDP core, with a test that a counter above 2^53 is exact | The `whole_total` helpers and their `#[expect]` attributes are gone. Each package has a test that fails on the old code and passes on the new. |
| T3 | TODO | Do the same in the UDP server and keep a gauge-only helper for `udp_banned_ips_total` | One retained `#[expect]` with a permanent reason that names the gauge and the `reset_bans` fall. `udp_processed_requests_total` and every counter accessor use `sum_counter`. |
| T4 | TODO | Run the workspace checks and the manual scenarios | Clippy with `-D warnings`, the affected package tests, `linter all`, and pre-push pass. Evidence is recorded. |

The tests for T2 and T3 prove they fail on the old code with the mutate-then-restore method: temporarily route the accessor back through `Sum::sum` and `as u64`, observe the failure, restore.

## Commit Points

| Task | Coherent change set | Commit policy |
| ---- | ------------------- | ------------- |
| T1 | The eight manifests and `Cargo.lock` | Commit after `cargo build --workspace` and a review of the lock diff. |
| T2 | HTTP core and UDP core helpers and tests, one commit per package | Commit after focused validation and required review. |
| T3 | UDP server helpers and tests | Commit after focused validation and required review. |
| T4 | Verification evidence | Commit after the checks pass. |

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/drafts/adopt-torrust-metrics-sum-counter/ISSUE.md`
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue created and issue number added to this spec
- [ ] Implementation completed
- [ ] Automatic verification completed (`linter all`, relevant tests, and any pre-push checks)
- [ ] Manual verification scenarios executed and recorded in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] Evidence-based implementation completion review recorded
- [ ] Reviewer validated acceptance criteria and updated checkboxes
- [ ] Committer verified spec progress is up to date before commit
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-10-19 10:26 UTC - GitHub Copilot - Drafted after `torrust-metrics` 0.2.0 was released with the exact counter sum; the maintainer chose one task issue with one implementation PR that carries this spec, and asked for the cleanest long-term outcome for the banned-IP gauge - Awaiting maintainer review
- 2026-10-19 11:23 UTC - GitHub Copilot - Maintainer approved the spec; created issue #2560, moved the spec to `docs/issues/open/`, and renamed the branch to `2560-adopt-torrust-metrics-sum-counter` (no spec-only PR, as chosen) - Implementation starts

## Acceptance Criteria

- [ ] AC1: All eight manifests depend on `torrust-metrics` `0.2.0`, and `Cargo.lock` resolves a single `torrust-metrics` version.
- [ ] AC2: HTTP core and UDP core read counter totals with `sum_counter` and have no float-to-integer cast and no `#[expect]` in their metrics modules.
- [ ] AC3: The UDP server reads counter totals with `sum_counter`. Its only remaining float-to-integer cast for these totals is the gauge helper, with one `#[expect]` whose reason names the gauge and the `reset_bans` fall.
- [ ] AC4: A counter above 2^53 is returned exactly by an accessor in each of the three packages, and each test is proven red against the old code.
- [ ] AC5: Every accessor returns the same value as before for realistic values, including `0` for a metric that does not exist yet. The existing tests of the three packages pass unchanged.
- [ ] `linter all` exits with code `0`
- [ ] Relevant tests pass
- [ ] Manual verification scenarios are executed and documented in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria are re-reviewed after implementation and reflect actual behavior
- [ ] Documentation is updated when behavior or workflow changes

## Verification Plan

### Automatic Checks

- `cargo build --workspace`
- `cargo clippy -p torrust-tracker-http-core -p torrust-tracker-udp-core -p torrust-tracker-udp-server --all-targets --all-features -- -D warnings`. `#[expect]` makes Clippy fail when a lint stops firing, so a stale exception is detected.
- `cargo test` for the packages that depend on `torrust-metrics` and for the packages that consume their statistics.
- `cargo machete --with-metadata`, `linter all`, and the pre-push script.

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID | Scenario | Human-oriented command/steps | Expected Result | Status | Evidence |
| -- | -------- | ---------------------------- | --------------- | ------ | -------- |
| M1 | UDP counters over the REST API | Run a local release tracker on loopback ports. Send 1,000 sequential BEP 15 connect requests, then read `GET /api/v1/stats`. | `udp4_requests`, `udp4_connections_handled`, and `udp4_responses` each read exactly 1,000. | TODO | `manual-verification-evidence.md` section V1 |
| M2 | HTTP counters and the banned-IP gauge | On the same tracker, send a known number of HTTP announce requests with `curl` and read the stats. Read `udp_banned_ips_total` too. | `tcp4_announces_handled` equals the number sent, and `udp_banned_ips_total` is a whole number matching the ban service. | TODO | `manual-verification-evidence.md` section V2 |

Notes:

- Manual verification is mandatory even when automated tests pass.
- Every recorded validation command result identifies the toolchain that produced it when one can affect behavior, for example the nightly Rust toolchain for `cargo +nightly fmt`.
- Create `manual-verification-evidence.md` from `docs/templates/MANUAL-VERIFICATION-EVIDENCE.md`.
- If a scenario fails, record the failure and diagnosis in the progress log before proceeding.

### Disposable Verification Scripts

None planned. M1 and M2 use `curl` and the maintained `tracker_client` binary.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| ----- | ---------------------- | -------- |
| AC1 | TODO | Pending implementation. |
| AC2 | TODO | Pending implementation. |
| AC3 | TODO | Pending implementation. |
| AC4 | TODO | Pending tests. |
| AC5 | TODO | Pending validation. |

## Risks and Trade-offs

- **Overflow behavior differs above `u64::MAX`.** The old path saturated through `f64`, and `sum_counter` adds with unchecked `u64`, which panics in debug and wraps in release. A counter that reaches `u64::MAX` is unreachable at any realistic request rate, since it would need about 580,000 years at one million requests per second. The doc comment on the new calls should say so, and the UDP server code should not claim saturation.
- **A partial bump breaks the build, or worse, builds with two copies of the crate.** Mitigation: bump all eight manifests in one commit and assert a single resolved version in AC1.
- **Transitive dependency changes in `Cargo.lock`.** Mitigation: review the lock diff and record any change that is not `torrust-metrics` itself.
- **The gauge cast stays.** It is the cleanest outcome for a true gauge and is documented as permanent, so it is not a deferred cleanup.

## Implementation Completion Review

- Retrospective: `Not yet assessed`
- Create `implementation-retrospective.md` from `docs/templates/IMPLEMENTATION-RETROSPECTIVE.md` in this folder if the work invalidates an assumption, changes design materially, or yields a reusable lesson; otherwise add a progress-log entry stating why none is needed.

## References

- Related issues: #2244, #2243, torrust/torrust-metrics#1
- Related PRs: #2552 (the tracker side of the round trip), torrust/torrust-metrics#2 (the new API)
- Release: <https://github.com/torrust/torrust-metrics/releases/tag/v0.2.0>
- Related ADRs: None
