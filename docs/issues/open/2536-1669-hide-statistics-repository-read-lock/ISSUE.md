---
schema-version: 1
doc-type: issue
issue-type: task
status: planned
priority: p3
epic: 1669
github-issue: 2536
spec-path: docs/issues/open/2536-1669-hide-statistics-repository-read-lock/ISSUE.md
branch: "2536-1669-hide-statistics-repository-read-lock"
related-pr: null
last-updated-utc: "2026-10-15 17:55"
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - .github/skills/dev/planning/create-issue/SKILL.md
    - "issue #1669"
    - docs/issues/closed/2517-1669-encapsulate-statistics-repository-state/ISSUE.md
    - docs/adrs/20260629000000_adopt_independent_package_versioning.md
    - docs/adrs/index.md
    - packages/udp-core/src/statistics/repository.rs
    - packages/udp-server/src/statistics/repository.rs
    - packages/http-core/src/statistics/repository.rs
    - packages/tracker-core/src/statistics/repository.rs
    - packages/swarm-coordination-registry/src/statistics/repository.rs
    - packages/rest-api-runtime-adapter/src/v1/adapters/stats.rs
---

<!-- skill-link: create-issue -->

# Issue #2536 - Hide the Statistics Repositories' Read Lock and Metric Collection Field

Parent EPIC: #1669 (Overhaul: Packages).

## Goal

Remove the lock type and the `MetricCollection` field from the public read API of the five
statistics repositories. Callers then read metrics through a closure that the repository runs
while it holds its read lock, and through a read-only `metric_collection()` accessor on
`Metrics`, so the repository can change its locking or storage without a breaking release.

## Background

Issue #2517 made each repository's `stats: Arc<RwLock<Metrics>>` field private. It left the read side
public on purpose, recording this follow-up in its Out of Scope section: each repository still
returns the lock's read guard.

| Package                       | Read accessor   | Return type                       |
| ----------------------------- | --------------- | --------------------------------- |
| `udp-core`                    | `get_stats()`   | `RwLockReadGuard<'_, Metrics>`    |
| `udp-server`                  | `get_stats()`   | `RwLockReadGuard<'_, Metrics>`    |
| `http-core`                   | `get_stats()`   | `RwLockReadGuard<'_, Metrics>`    |
| `tracker-core`                | `get_metrics()` | `RwLockReadGuard<'_, Metrics>`    |
| `swarm-coordination-registry` | `get_metrics()` | `RwLockReadGuard<'_, Metrics>`    |

This has three consequences:

- `tokio::sync::RwLockReadGuard` is part of each crate's public API. Replacing the tokio lock (for
  example with a `std` or `parking_lot` lock, or a lock-free store) would be a breaking change.
- A caller can hold the guard for as long as it wants, including across an `.await`, which blocks
  every writer (the statistics event listeners) for that time.
- Each `Metrics` type has a public field, `pub metric_collection: MetricCollection`, so
  the layout of `Metrics` is also public API.

None of the five crates is published yet (see the #2517 spec's Semver impact section), so this can
change without a semver-major bump if it lands before their first publish. That matters for the
goal of EPIC #1669: independently versioned, publishable packages.

### Current callers

Checked on `develop` at `48c4d3013` (2026-10-14):

- Production callers outside the five crates are all in
  `packages/rest-api-runtime-adapter/src/v1/adapters/stats.rs`:
  - `get_stats()` reads derived values (for example `tcp4_announces_handled()` and
    `udp_banned_ips_total()`) from the HTTP and UDP-server guards to build the REST `Stats`.
  - `get_labeled_stats()` merges each repository's `metric_collection` into one collection.
- Production callers inside the crates: the `udp-core`, `udp-server`, and `http-core` statistics
  event handlers log `tracing::debug!("stats: {:?}", stats_repository.get_stats().await)` on every
  event, and `tracker-core`'s repository reads its own counters through `get_metrics()`.
- Test callers outside the crates: `axum-http-server` contract tests, `udp-server` contract
  tests, `tracker-core`'s integration test environment, and a `src/bootstrap/jobs` unit test.
- Unit tests inside the five crates: 57 call sites in the crates' test modules. They are
  `udp-server` 30, `udp-core` 12, `http-core` 9, `swarm-coordination-registry` 4, and
  `tracker-core` 2, so the `udp-server` task is by far the largest. (A plain count of calls after
  each file's first `#[cfg(test)]` gives 58: it also picks up the `udp-server` companion doctest
  call at `statistics/repository.rs`:35, which compiles as its own crate and is listed with the
  companion examples below.) Some need their own closure shape:
  - `udp-server`'s `metrics_snapshot` test helper in `statistics/repository.rs`, which copies the
    metric collection out of the guard;
  - the swarm job test's `inactive_peers_total` helper in `statistics/mod.rs`;
  - `udp-server`'s `it_should_return_a_read_guard_to_metrics` test, which tests the guard accessor
    itself and is replaced by a test of `read_metrics()`.

  Removing the accessors makes the compiler report every call, so none can be missed silently.
- The `metric_collection` field is written outside each crate's `metrics.rs` only by its own
  `describe_metrics()` function (in `statistics/mod.rs`). Code outside the crates only reads it.
- The #2517 privacy doctests' companion examples call `get_stats()` / `get_metrics()`, and the
  rustdoc lead-in above them links to the accessor: "External callers read statistics through
  [`Repository::get_stats`]" in `udp-core`, `udp-server`, and `http-core`, and the same line with
  [`Repository::get_metrics`] in `tracker-core` and `swarm-coordination-registry`. Once the
  accessor is gone, that link is broken. The root `Cargo.toml` denies `warnings`, so the library
  docs step in `.github/workflows/testing.yaml` and the pre-push nightly doc build would fail.

## Proposed Design

Each repository replaces `get_stats()` / `get_metrics()` with one read method that has the same
name in all five crates:

```rust,ignore
impl Repository {
    /// Runs `read` on the current metrics while holding the read lock.
    ///
    /// Writers wait while `read` runs, so keep it short: it must not block the thread
    /// (blocking I/O, sleeping, or long computation) or call `read_metrics` on this repository.
    pub async fn read_metrics<R>(&self, read: impl FnOnce(&Metrics) -> R) -> R {
        let metrics = self.stats.read().await;
        read(&metrics)
    }
}
```

Each `Metrics` type makes its field `pub(crate)` and adds a read-only accessor:

```rust,ignore
impl Metrics {
    #[must_use]
    pub fn metric_collection(&self) -> &MetricCollection {
        &self.metric_collection
    }
}
```

Why this design (chosen by the maintainer during drafting):

- **No lock type in the API.** The signature names only `Metrics` and the closure, so the lock
  implementation can change freely.
- **Consistent reads.** A caller that reads several values (the REST `Stats` reads about twenty
  from one repository) gets them from one lock acquisition, as it does today.
- **No copies.** Unlike an owned snapshot, nothing is cloned on each read.
- **No lock held across an `.await`.** The closure is synchronous, so a caller cannot hold the
  lock across an `.await`. It can still block the thread; avoiding that is the caller's
  obligation (see Design and Ownership Review).

The rejected alternatives were per-value accessor methods (one lock acquisition per value, so
several values read together may not be consistent with each other) and an owned snapshot (a
full clone on every read; the #2517 spec already ruled that out for event paths).

Expected caller shapes:

```rust,ignore
// One value
let banned = repository.read_metrics(Metrics::udp_banned_ips_total).await;

// Several consistent values
let stats = repository.read_metrics(|metrics| (metrics.tcp4_announces_handled(), metrics.tcp4_scrapes_handled())).await;

// Merge into an outer collection
repository.read_metrics(|metrics| collection.merge(metrics.metric_collection())).await?;
```

## Scope

### In Scope

- Replace `get_stats()` / `get_metrics()` with `read_metrics()` in the five repositories and
  remove the old accessors. The crates are unpublished, so no deprecation period is needed.
- Make `Metrics.metric_collection` `pub(crate)` in the five crates and add the
  `metric_collection()` read accessor.
- Update every caller: `rest-api-runtime-adapter`, the event handlers, `tracker-core`'s own
  counter reads, and every test caller, both outside and inside the five crates (see Current
  callers).
- Keep the event handlers' debug logging lazy: the lock must still be taken only when `debug` is
  enabled for that event (see Risks).
- Pin the new API with doctests in each crate:
  - a `compile_fail,E0616` doctest that reads `metrics.metric_collection` from outside the crate;
  - a compiling example of `read_metrics()` and `metric_collection()`;
  - an update to the #2517 companion doctests, which currently call the removed accessors;
  - an update to the rustdoc lead-in above those doctests, whose intra-doc link
    (`Repository::get_stats` or `Repository::get_metrics`) must point to `Repository::read_metrics`.

### Out of Scope

- The write methods (counter and gauge setters) and their signatures.
- The lock itself: the repositories keep `tokio::sync::RwLock`; this issue only makes that choice
  private.
- The `Metrics` value methods (for example `udp_banned_ips_total()`) and their names.
- Unifying the five repositories or the five `Metrics` types into shared generic types.
- The `stats` field name inside each repository.

## Architectural Decisions

- Related ADRs: `docs/adrs/20260629000000_adopt_independent_package_versioning.md` (the read API
  is public API; changing it after a publish is breaking).
- ADRs to create: one root ADR in `docs/adrs/` (task T9), because the maintainer decided that
  closure-based reads become the rule for every statistics repository. It is a multi-package
  decision, so it is a root ADR. It records the rule (a repository exposes no lock type and no
  metric-collection field in its public read API; callers read through `read_metrics()` and
  `metric_collection()`), the alternatives rejected in Proposed Design, and the caller obligation
  from the Design and Ownership Review. Follow the `create-adr` skill, including registering it
  in `docs/adrs/index.md`.

## Design and Ownership Review

Not applicable: no child processes, readiness, or resource cleanup. Lock ownership stays inside
each repository. While a `read_metrics()` closure runs, the read lock is held and every writer
(the statistics event listeners) waits, so the closure must be short and must not block. The API
enforces only part of this:

- **Enforced:** the closure cannot `.await`, because `FnOnce(&Metrics) -> R` is synchronous.
- **Caller obligation:** the closure must not block the thread (no blocking I/O, `thread::sleep`,
  or long CPU work) and must not call `read_metrics()` on the same repository, which could
  deadlock. The type system does not prevent either. `read_metrics()`'s rustdoc states this
  obligation, and every current caller meets it: each only reads values or merges a collection.

## Bug-Fix Process

Not applicable: not a bug.

## Regression Test Strategy

Not a bug, so there is no defect to reproduce. The API is pinned by doctests (see In Scope) and by
the existing caller tests, which keep their assertions and change only how they read. Use the
`write-unit-test` skill for any new or changed test, and record the red/green check for each
`compile_fail` doctest: it must compile while the field is `pub` and fail with `E0616` after.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task                                                    | Notes / Expected Output                                                                                                                  |
| --- | ------ | ------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------- |
| T1  | TODO   | Re-verify the callers on current `develop`              | Repeat the caller search; record the commit and any new caller in the Progress Log.                                                     |
| T2  | TODO   | Capture the M1 baseline on `develop`                    | Run M1 before any change and store the five `develop` outputs listed under Manual Verification Scenarios under `evidence/`. |
| T3  | TODO   | Hide the read lock and field in `udp-core`              | `read_metrics()`, `metric_collection()`, `pub(crate)` field, doctests and their rustdoc lead-in link, and updated callers; the workspace builds and its tests pass. |
| T4  | TODO   | Hide the read lock and field in `udp-server`            | Same as T3; the largest task (30 in-crate unit-test calls, the `metrics_snapshot` helper, and the guard-accessor test to replace). |
| T5  | TODO   | Hide the read lock and field in `http-core`             | Same as T3.                                                                                                                              |
| T6  | TODO   | Hide the read lock and field in `tracker-core`          | Same as T3, including the repository's own counter reads.                                                                               |
| T7  | TODO   | Hide the read lock and field in `swarm-coordination-registry` | Same as T3.                                                                                                                        |
| T8  | TODO   | Verification                                            | Starts only after the maintainer review that follows T7. See Verification Plan; run M1 on the branch, store its five outputs, and compare them with T2's. |
| T9  | TODO   | Write the root ADR for closure-based statistics reads   | `docs/adrs/<timestamp>_<title>.md` registered in `docs/adrs/index.md`; passes `linter markdown` and `linter cspell`. Needs no maintainer test review, so it can be written before T8. |

## Commit Points

| Task  | Coherent change set                                                                                                                           | Commit policy                                               |
| ----- | --------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------- |
| T3-T7 | `refactor(<package>): [#2536] hide the statistics read lock and metric collection field`, one per package, each with its callers and doctests, so every commit builds | Commit after focused validation and the test-design review; T7's commit also waits for the maintainer review (see Test-Development Workflow). |
| T2, T8 | Manual verification evidence and spec progress updates                                                                                       | Commit after the evidence is recorded.                      |
| T9    | `docs(adrs): [#2536] record closure-based reads for the statistics repositories`                                                              | Commit after the ADR passes `linter markdown` and `linter cspell`. |

Every commit is GPG signed. Because `rest-api-runtime-adapter` and the tests read from all five
crates, each package commit updates the callers of that package's accessor in the same commit.

### Test-Development Workflow

T3-T7 each produce tests (the new doctests and the migrated caller tests), so each follows the
`write-unit-test` skill as one increment:

1. Write or change the tests for that package in the smallest behavior-focused steps, and see each
   `compile_fail,E0616` doctest compile while the field is `pub` (red) and fail with `E0616` after
   (green).
2. After focused validation passes, do the test-design review before committing, and record it in
   the Progress Log or task evidence:
   - the test exposes the one causal initial-state difference;
   - fixtures own only incidental mechanics;
   - the production Act and the independently specified expected result stay visible;
   - each helper's boundary is a meaningful named action at the test's level of abstraction,
     judged by that and not by caller count (a single-use helper is valid when it hides only
     incidental mechanics). This applies to the helpers that wrap today's guard and must be
     reshaped, such as `udp-server`'s `metrics_snapshot` and the swarm job test's
     `inactive_peers_total`.
3. Do the mandatory prose-first Arrange-Act-Assert comparison: write temporary prose for each
   section, refactor until the code expresses it, remove the redundant prose, and record the
   result. For a `compile_fail` doctest the Act is compiling and the Assert is the expected error;
   record the comparison in those terms rather than skipping it.
4. For T3-T6, finish and commit this review before starting the next package task. T7 is the
   final increment, so its commit waits for the maintainer review below.

After the final increment (T7) passes its own test-design review, stop for maintainer review of all
the test increments before T7's commit, T8 (final verification), the last spec commit, or the pull
request.

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/drafts/1669-hide-statistics-repository-read-lock/ISSUE.md`
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue created and issue number added to this spec
- [ ] (Optional, recommended for complex issues) Spec-only PR merged into `develop` before implementation
- [ ] Test-design review (with the prose-first AAA comparison) recorded for each of T3-T7 before the next task started
- [ ] Maintainer reviewed the test increments after T7, before T7's commit and final verification
- [ ] Implementation completed
- [ ] Automatic verification completed (`linter all`, relevant tests, and any pre-push checks)
- [ ] Manual verification scenarios executed and recorded in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] Evidence-based implementation completion review recorded: issue-local retrospective created for material discoveries, or progress log states why none was needed
- [ ] Reviewer validated acceptance criteria and updated checkboxes
- [ ] Independent reviewer reports recorded in issue-local `agent-review-reports.md` when reviewers received this folder-style specification
- [ ] Committer verified spec progress is up to date before commit
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-10-14 16:59 UTC - GitHub Copilot - Drafted from the #2517 Out of Scope follow-up. The
  maintainer chose the closure-based `read_metrics()` design and asked to include the
  `Metrics.metric_collection` field. Callers checked on `develop` at `48c4d3013`.
- 2026-10-14 17:03 UTC - GitHub Copilot - Maintainer approved the spec. Created #2536, linked it as
  a sub-issue of #1669, moved the spec to `docs/issues/open/`, and added it to the EPIC on branch
  `2536-1669-hide-statistics-repository-read-lock-spec` for a spec-only PR.
- 2026-10-14 17:30 UTC - GitHub Copilot - PR #2537 review (F1): a synchronous closure cannot
  await but can still block the thread; recorded non-blocking closures as a caller obligation in
  the Design and Ownership Review, the proposed rustdoc, and the design rationale.
- 2026-10-14 19:36 UTC - GitHub Copilot - PR #2537 review (F2): added the Test-Development Workflow
  (per-increment test-design review with the prose-first AAA comparison, finished before the next
  package task) and the maintainer stop after T7, with matching workflow checkpoints.
- 2026-10-14 19:37 UTC - GitHub Copilot - PR #2537 review (F3): the lazy debug-log example now formats
  the returned `String` with `{}`, preserving today's `stats: Metrics { .. }` text (tokio
  1.53.3's `RwLockReadGuard` `Debug` delegates to `Metrics`).
- 2026-10-14 19:47 UTC - GitHub Copilot - PR #2537 review (F4, da2ce7): Current callers now lists
  the in-crate unit-test calls per crate (58 by our count), names the helpers and the
  guard-accessor test that need their own shape, and In Scope covers every test caller.
- 2026-10-14 19:48 UTC - GitHub Copilot - PR #2537 review (F5, da2ce7): M1 now names the ten
  stored outputs (five per run), states the full normalization (sorted samples, recorded_at
  ignored, processing times masked), and makes byte-identical normalized outputs the expected
  result; T2 and T8 store their five outputs each.
- 2026-10-14 19:49 UTC - GitHub Copilot - PR #2537 review (F6, da2ce7): the overlap risk now
  names the three open sibling specs outside #1669 that touch the removed accessors (#2414, #1348,
  #2518).
- 2026-10-14 19:59 UTC - GitHub Copilot - PR #2537 review (F7, da2ce7): the AC1 call grep is now
  limited to Rust sources, excludes torrent-repository-benchmarking, and must print exactly the two
  REST-port lines (stats_service.get_stats() and query_port.get_stats()), so the check has a pass
  or fail result.
- 2026-10-14 20:01 UTC - GitHub Copilot - PR #2537 review (F8, da2ce7): the rustdoc lead-in above
  each repository's #2517 doctests links to the removed accessor; Current callers, the In Scope
  doctests bullet, T3, and AC3 now cover that link, and an AC3 check greps for it and runs the
  nightly library docs build.
- 2026-10-14 20:10 UTC - GitHub Copilot - PR #2537 review (F9, da2ce7): the test-design review
  gains the create-issue helper-boundary check (meaningful named actions at the test's level of
  abstraction, not caller count), applied to the guard-wrapping helpers metrics_snapshot and
  inactive_peers_total.
- 2026-10-14 20:11 UTC - GitHub Copilot - PR #2537 review (F10, da2ce7): T7's commit now waits
  for the maintainer review, in line with create-issue's stop before commit; T3-T6 still commit
  after their own test-design review. Step 4, the stop, the T3-T7 commit policy, and the checkpoint
  say so.
- 2026-10-14 21:10 UTC - GitHub Copilot - PR #2537 review (F13, da2ce7): the in-crate unit-test
  count is 57 (udp-server 30), counted inside the test modules. The earlier 58 also counted the
  udp-server companion doctest call at statistics/repository.rs:35, which compiles as its own
  crate; Current callers explains the difference and T4 says 30.
- 2026-10-14 21:11 UTC - GitHub Copilot - PR #2537 review (F14, da2ce7): M1's normalization now
  masks the two activity-job gauges (swarm_coordination_registry_peers_inactive_total and
  _torrents_inactive_total), whose presence and values depend on time since startup, not on the
  announces.
- 2026-10-15 17:55 UTC - GitHub Copilot - Implementation started on branch
  `2536-1669-hide-statistics-repository-read-lock` from `torrust/develop` at `fe5d8264a`. The caller
  grep now prints 80 lines (84 at `48c4d3013`); T1 records the difference. Maintainer feedback:
  (1) write a root ADR making closure-based reads the rule (new task T9 and AC6; Architectural
  Decisions updated); (2) the agent runs M1 itself (T2 baseline and T8); (3) nobody is working on
  #2414, so the overlap risk needs no rebase ordering now; (4) no other spec changes.

## Acceptance Criteria

- [ ] AC1: No public method of the five statistics repositories returns a lock guard or any other
  lock type; `get_stats()` and `get_metrics()` no longer exist.
- [ ] AC2: The five `Metrics` types have no `pub` `metric_collection` field; code outside each crate
  reads it only through `metric_collection()`.
- [ ] AC3: Each crate has a `compile_fail,E0616` doctest for the field and a compiling
  `read_metrics()` example, and neither the #2517 companion doctests nor the rustdoc lead-in
  above them refers to a removed accessor.
- [ ] AC4: The `udp-core`, `udp-server`, and `http-core` event handlers still take the read lock for
  their debug log only when `debug` is enabled.
- [ ] AC5: REST `/api/v1/stats` and `/api/v1/metrics` report the same values as `develop` for the
  same announces.
- [ ] AC6: A root ADR records closure-based reads as the rule for the statistics repositories and is
  listed in `docs/adrs/index.md`.
- [ ] `linter all` exits with code `0`
- [ ] Relevant tests pass
- [ ] Manual verification scenarios are executed and documented in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria are re-reviewed after implementation and reflect actual behavior
- [ ] Documentation is updated when behavior/workflow changes

## Verification Plan

### Automatic Checks

- `linter all`
- `cargo test --tests --benches --examples --workspace --all-targets --all-features` (stable Rust
  toolchain)
- `cargo test --doc --workspace` (stable Rust toolchain) and
  `cargo +nightly test --doc` for the five packages (nightly Rust toolchain, which checks the
  pinned `E0616` code)
- Pre-push checks (`./contrib/dev-tools/git/hooks/pre-push.sh`)
- AC1 check: `grep -nE 'RwLock(Read|Write)Guard' packages/*/src/statistics/repository.rs` matches
  only non-`pub` items (for example the `#[cfg(test)] pub(crate)` `write_lock_for_test()`), and
  this call grep prints exactly the two REST-port lines below, which call the port and service
  methods of the same name rather than a statistics repository:

  ```sh
  grep -rnE '\.(get_stats|get_metrics)\(\)' --include='*.rs' \
    --exclude-dir=torrent-repository-benchmarking packages src tests console contrib
  ```

  ```text
  packages/axum-rest-api-server/src/v1/context/stats/handlers.rs:29:    let stats = stats_service.get_stats().await;
  packages/rest-api-application/src/v1/use_cases/stats.rs:24:        self.query_port.get_stats().await
  ```

  The grep is limited to Rust sources so that `docs/` and `target/` cannot match, and it
  excludes `torrent-repository-benchmarking`, whose own `get_metrics()` is out of scope. Line
  numbers may shift if those two files change; the file paths and calls must not. At
  `48c4d3013` it prints 84 lines.
- AC2 check: `grep -nE 'pub +metric_collection *:' packages/*/src/statistics/metrics.rs` prints
  nothing.
- AC3 check: this grep prints nothing (at `48c4d3013` it prints the five rustdoc lead-ins), and
  `cargo +nightly doc --no-deps --lib --workspace --all-features` succeeds:

  ```sh
  grep -rnE 'Repository::(get_stats|get_metrics)' --include='*.rs' \
    --exclude-dir=torrent-repository-benchmarking packages src tests console contrib
  ```

- AC4 check: a review of each event handler's logging line, plus a unit test or a documented
  inspection showing the read happens inside the `tracing::debug!` arguments.

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID  | Scenario                 | Human-oriented command/steps                                                                                                                                                                                         | Expected Result                                                                                                  | Status | Evidence                                     |
| --- | ------------------------ | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------- | ------ | -------------------------------------------- |
| M1  | REST metrics unchanged   | Run the tracker (`run-tracker-locally` skill) on `develop` and on the branch with fresh databases; send the same HTTP and UDP `started` and `completed` announces with `tracker_client`; read `/api/v1/stats` and `/api/v1/metrics` before and after | The five normalized outputs (see Notes) are byte-identical between the two runs | TODO   | `manual-verification-evidence.md` section V1 |

Notes:

- Store all ten compared outputs under `evidence/`, five per run, so a reviewer can re-run the
  comparison from the committed files. #2517's evidence stored one copy per output and omitted the
  before-announce stats, so its comparison rested on a transcript; a post-merge review flagged both.
  For each of `develop` and the branch, store:
  - the client responses;
  - the stats before the announces;
  - the stats after the announces;
  - the metrics before the announces;
  - the metrics after the announces.
- Normalize before comparing, and record the normalization in the evidence:
  - metrics: one sorted line per sample (`name {sorted labels} value`, or `name <no samples>`);
  - ignore each sample's `recorded_at`, which differs between runs;
  - in both metrics and stats, replace the values of time-dependent `*processing_time*` entries
    with `<timing>`;
  - in metrics, replace each line of `swarm_coordination_registry_peers_inactive_total` and
    `swarm_coordination_registry_torrents_inactive_total`, sample or `<no samples>`, with
    `name <activity-job>`. Only the activity job writes these two gauges. It skips the interval's
    first tick and then runs every 15 s, so a reading shows them or not depending on how long after
    startup it was taken, not on the announces; past the development config's
    `max_peer_timeout = 60` their values change as well. The stats output does not include them.

  The Expected Result is byte-identical normalized outputs, checked with `diff -u`.
- Avoid the aggregate `/health_check` endpoint during M1: it sends UDP requests that change the UDP
  metrics. Use `/api/health_check` for readiness.
- Create `manual-verification-evidence.md` from `docs/templates/MANUAL-VERIFICATION-EVIDENCE.md`
  and record the actual commands, output, and relevant tracker logs.

### Disposable Verification Scripts

None planned. If M1's steps are wrapped in a local script for exact repetition, record its full
steps in the evidence instead of committing it, as #2517 did.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence           |
| ----- | ---------------------- | ------------------ |
| AC1   | TODO                   | {test/log/PR link} |
| AC2   | TODO                   | {test/log/PR link} |
| AC3   | TODO                   | {test/log/PR link} |
| AC4   | TODO                   | {test/log/PR link} |
| AC5   | TODO                   | {test/log/PR link} |
| AC6   | TODO                   | {test/log/PR link} |

## Risks and Trade-offs

- **Eager debug logging**: wrapping the handlers' `tracing::debug!` in a `read_metrics()` closure
  would take the read lock on every event even when `debug` is disabled, adding lock traffic on the
  request path. Keep the read inside the macro arguments, for example
  `tracing::debug!("stats: {}", stats_repository.read_metrics(|metrics| format!("{metrics:?}")).await)`,
  which the macro evaluates only when `debug` is enabled. Format the returned `String` with `{}`,
  not `{:?}`: `{:?}` would add quotes and escapes, changing today's `stats: Metrics { .. }` log
  text.
- **Closure ergonomics**: closures cannot `.await` or return a borrow of `Metrics`. Every current
  caller only reads values or merges a collection synchronously, so none needs either. A future
  caller that does should copy what it needs inside the closure.
- **Merge errors inside the closure**: `MetricCollection::merge` returns a `Result`; the REST
  adapter keeps its current `expect` messages by returning the `Result` from the closure.
- **Overlap with other planned work**: three open specs outside #1669 touch what this issue
  changes. Whichever lands second adapts, and each author should know that `get_stats()` and
  `get_metrics()` are going away:
  - #2414 (drain listener queues on shutdown,
    `docs/issues/open/2414-2410-si-22-2-drain-listener-queues-on-shutdown/ISSUE.md`) edits all
    five `statistics/event/listener.rs` files, whose unit tests call the removed accessors, and
    changes `tracker-core/tests/common/test_env.rs`, one of this issue's callers.
  - #1348 (udp-core tests, `docs/issues/closed/1348-1347-add-tests-udp-core/ISSUE.md`) plans tests
    for `udp-core`'s `statistics/repository.rs`.
  - #2518 (HTTP first-announce benchmark, `docs/issues/open/2518-http-first-announce-benchmark/ISSUE.md`)
    plans an assertion that reads through the swarm `Repository::get_metrics`.

  Any other open branch that touches these repositories or the REST adapter also needs a rebase.

## Implementation Completion Review

After implementation, compare the result with this specification. Record
invalidated assumptions, material design changes, unexpected validation
findings, and reusable lessons.

- Retrospective: `Not yet assessed`
- If needed, create `implementation-retrospective.md` from the repository
  template at `docs/templates/IMPLEMENTATION-RETROSPECTIVE.md` in this issue
  specification's directory.
- If no retrospective is needed, add a concise progress-log entry explaining
  why the work had no material discovery.
- When an independent reviewer receives this folder-style specification, it
  records its result in `agent-review-reports.md` using
  `docs/templates/AGENT-REVIEW-REPORTS.md`.

## References

- Parent EPIC: #1669
- Related issues: #2517 (made the repositories' `stats` field private; this is its recorded
  follow-up), #2521 (removed the unused SI-30 statistics traits from the same files)
- Related ADRs: `docs/adrs/20260629000000_adopt_independent_package_versioning.md`
