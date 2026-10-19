---
schema-version: 1
doc-type: issue
issue-type: task
status: in-review
priority: p3
epic: null
github-issue: 2560
spec-path: docs/issues/open/2560-adopt-torrust-metrics-sum-counter/ISSUE.md
branch: "2560-adopt-torrust-metrics-sum-counter"
related-pr: 2561
last-updated-utc: "2026-10-19 12:10"
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

One conversion cannot go away. The UDP server's `udp_banned_ips_total` reads a gauge, because the banned-IP count falls when `BanService::reset_bans` clears the bans. `sum_counter` returns `None` for a gauge name, so that accessor keeps an `f64` to `u64` cast. The maintainer asked for the cleanest and most maintainable outcome regardless of effort. The decision is to keep that one cast in the `udp_banned_ips_total` accessor with a permanent reason, because it is the only gauge reader and a helper would only wrap a single call. The reasons are:

- It is a real gauge, so converting it to a counter would change how the metric behaves for every consumer of the metrics.
- Asking a float-gauge library for an integer gauge accessor would widen its API, and fix a rounding policy, for one consumer.
- After this issue it is the only cast left for the integer totals (counter and gauge sums), with one documented reason. The averaged accessors keep their own documented cast, which is out of scope here.

## Scope

### In Scope

- Bump `torrust-metrics` from `0.1.0` to `0.2.0` in the eight manifests: `axum-rest-api-server`, `http-core`, `rest-api-protocol`, `rest-api-runtime-adapter`, `swarm-coordination-registry`, `tracker-core`, `udp-core`, and `udp-server`. Update `Cargo.lock` and check that only expected entries change.
- Replace the `whole_total` helpers of HTTP core and UDP core with a cast-free `counter_total` helper over `sum_counter`, and remove their `#[expect]` attributes. (The draft planned to call `sum_counter` in every accessor. Review finding `review-finding:pr-2561-f1` asked for one place for that expression, and the UDP server alone has 17 call sites.)
- In the UDP server, read counters through the same cast-free helper and keep one retained `#[expect]` with a permanent reason in the `udp_banned_ips_total` accessor, which is the only gauge reader. (The draft proposed a renamed gauge helper. With a single call site the cast belongs in the accessor, so no helper remains.)
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
| T1 | DONE | Bump `torrust-metrics` to `0.2.0` in the eight manifests and `Cargo.lock` | `cargo build --workspace` passes. `git diff Cargo.lock` is reviewed and records any transitive change, which should be only `torrust-metrics` itself. Follow the `update-dependencies` skill. |
| T2 | DONE | Read counters through `sum_counter` in HTTP core and UDP core, with a test that a counter above 2^53 is exact | The `whole_total` helpers and their `#[expect]` attributes are gone. Each package has a test that fails on the old code and passes on the new. |
| T3 | DONE | Do the same in the UDP server and keep the cast only in `udp_banned_ips_total` | One retained `#[expect]` with a permanent reason that names the gauge and the `reset_bans` fall. `udp_processed_requests_total` and the other 16 counter accessors use `counter_total`, which calls `sum_counter`. |
| T4 | DONE | Run the workspace checks and the manual scenarios | Clippy with `-D warnings`, the affected package tests, `linter all`, and pre-push pass. Evidence is recorded. |

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
- [x] Implementation completed
- [x] Automatic verification completed (`linter all`, relevant tests, and any pre-push checks)
- [x] Manual verification scenarios executed and recorded in issue-local `manual-verification-evidence.md`
- [x] Acceptance criteria reviewed after implementation and updated with evidence
- [x] Evidence-based implementation completion review recorded
- [ ] Reviewer validated acceptance criteria and updated checkboxes
- [ ] Independent reviewer reports recorded in issue-local `agent-review-reports.md` when reviewers received this folder-style specification
- [x] Committer verified spec progress is up to date before commit
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-10-19 10:26 UTC - GitHub Copilot - Drafted after `torrust-metrics` 0.2.0 was released with the exact counter sum; the maintainer chose one task issue with one implementation PR that carries this spec, and asked for the cleanest long-term outcome for the banned-IP gauge - Awaiting maintainer review
- 2026-10-19 11:23 UTC - GitHub Copilot - Maintainer approved the spec; created issue #2560, moved the spec to `docs/issues/open/`, and renamed the branch to `2560-adopt-torrust-metrics-sum-counter` (no spec-only PR, as chosen) - Implementation starts
- 2026-10-19 11:38 UTC - GitHub Copilot - T1-T4 done: bumped `torrust-metrics` to 0.2.0 in the eight manifests (one lock entry changed); moved HTTP core, UDP core, and the UDP server's 17 counters to `sum_counter` with one red/green test per package; kept the banned-IP gauge cast in its accessor with a permanent reason (a single call site made a helper pointless, a deviation from the draft); ran M1 and M2 against a local release tracker and `pre-push.sh`, all passing; no retrospective needed - Awaiting review
- 2026-10-19 12:03 UTC - GitHub Copilot - PR #2561 review round 1 (Copilot, three findings, all valid): added a cast-free `counter_total` helper per package for the repeated `sum_counter` read, switched the UDP server test to a fixed timestamp, and set the status to `in-review` with `related-pr: 2561` - See `docs/pr-reviews/pr-2561-review/PR-REVIEW.md`
- 2026-10-19 12:10 UTC - GitHub Copilot - PR #2561 review round 2 (da2ce7, two findings, both valid): the Background now says the cast stays in the accessor and is the only one left for the integer totals, and the template's independent-reviewer-reports checkpoint is restored and left open - See `docs/pr-reviews/pr-2561-review/PR-REVIEW.md`

## Acceptance Criteria

- [x] AC1: All eight manifests depend on `torrust-metrics` `0.2.0`, and `Cargo.lock` resolves a single `torrust-metrics` version.
- [x] AC2: HTTP core and UDP core read counter totals through `sum_counter` and have no float-to-integer cast and no `#[expect]` in their metrics modules.
- [x] AC3: The UDP server reads counter totals with `sum_counter`. Its only remaining float-to-integer cast for these totals is in the `udp_banned_ips_total` accessor, with one `#[expect]` whose reason names the gauge and the `reset_bans` fall.
- [x] AC4: A counter above 2^53 is returned exactly by an accessor in each of the three packages, and each test is proven red against the old code.
- [x] AC5: Every accessor returns the same value as before for realistic values, including `0` for a metric that does not exist yet. The existing tests of the three packages pass unchanged.
- [x] `linter all` exits with code `0`
- [x] Relevant tests pass
- [x] Manual verification scenarios are executed and documented in issue-local `manual-verification-evidence.md`
- [x] Acceptance criteria are re-reviewed after implementation and reflect actual behavior
- [x] Documentation is updated when behavior or workflow changes

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
| M1 | UDP counters over the REST API | Run a local release tracker on loopback ports. Send 1,000 sequential `tracker_client udp announce` invocations (each sends one connect and one announce request), then read `GET /api/v1/stats`. | `udp4_connections_handled` and `udp4_announces_handled` each read exactly 1,000, and `udp4_requests` and `udp4_responses` read 2,000. | DONE | `manual-verification-evidence.md` section V2 |
| M2 | HTTP counters and the banned-IP gauge | On the same tracker, send 50 HTTP announces with `tracker_client`, then send invalid-connection-ID UDP announces until the IP is banned, and read the stats. | `tcp4_announces_handled` reads 50, and `udp_banned_ips_total` reads the number of banned IPs through the retained gauge cast. | DONE | `manual-verification-evidence.md` section V3 |

Notes:

- Manual verification is mandatory even when automated tests pass.
- Every recorded validation command result identifies the toolchain that produced it when one can affect behavior, for example the nightly Rust toolchain for `cargo +nightly fmt`.
- Create `manual-verification-evidence.md` from `docs/templates/MANUAL-VERIFICATION-EVIDENCE.md`.
- If a scenario fails, record the failure and diagnosis in the progress log before proceeding.

### Disposable Verification Scripts

None kept. M1 and M2 use the maintained `tracker_client` binary. M2 needs UDP announces with an invalid connection ID, which `tracker_client` cannot send, so one shell one-liner over `/dev/udp` was used and is recorded verbatim in the evidence.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| ----- | ---------------------- | -------- |
| AC1 | DONE | `grep -rn 'torrust-metrics' --include=Cargo.toml` shows `0.2.0` in all eight manifests, and `Cargo.lock` has one `torrust-metrics` entry at 0.2.0. The lock diff is that entry's version and checksum only. |
| AC2 | DONE | `grep -rn 'whole_total\|#\[expect' packages/http-core/src/statistics/metrics.rs packages/udp-core/src/statistics/metrics.rs` finds nothing. Both packages read every counter through a cast-free `counter_total` helper, whose body is the only `sum_counter` call. |
| AC3 | DONE | 17 counter accessors in `packages/udp-server/src/statistics/metrics.rs` use `counter_total`, whose body is the only `sum_counter` call. The four remaining `#[expect]` attributes there are the averaged helper, the two moving-average conversions, and `udp_banned_ips_total`. |
| AC4 | DONE | One test per package, red against the old path with `left: 9007199254740992` and `right: 9007199254740993`, and green after (`manual-verification-evidence.md` V1). |
| AC5 | DONE | The existing tests pass unchanged (HTTP core 33, UDP core 97, UDP server 225 after one new test each), `pre-push.sh` passes, and M1 and M2 read the expected counters. |

## Risks and Trade-offs

- **Overflow behavior differs above `u64::MAX`.** The old path saturated through `f64`, and `sum_counter` adds with unchecked `u64`, which panics in debug and wraps in release. A counter that reaches `u64::MAX` is unreachable at any realistic request rate, since it would need about 580,000 years at one million requests per second. The difference is recorded here and in the `SumCounter` documentation, and is not repeated on each accessor, because it cannot occur.
- **A partial bump breaks the build, or worse, builds with two copies of the crate.** Mitigation: bump all eight manifests in one commit and assert a single resolved version in AC1.
- **Transitive dependency changes in `Cargo.lock`.** Mitigation: review the lock diff and record any change that is not `torrust-metrics` itself.
- **The gauge cast stays.** It is the cleanest outcome for a true gauge and is documented as permanent, so it is not a deferred cleanup.

## Implementation Completion Review

- Retrospective: Not needed. The plan held. The one deviation, keeping the single gauge cast in its accessor instead of a renamed helper, is recorded in the scope and the progress log.
- Create `implementation-retrospective.md` from `docs/templates/IMPLEMENTATION-RETROSPECTIVE.md` in this folder if the work invalidates an assumption, changes design materially, or yields a reusable lesson; otherwise add a progress-log entry stating why none is needed.

## References

- Related issues: #2244, #2243, torrust/torrust-metrics#1
- Related PRs: #2552 (the tracker side of the round trip), torrust/torrust-metrics#2 (the new API)
- Release: <https://github.com/torrust/torrust-metrics/releases/tag/v0.2.0>
- Related ADRs: None
