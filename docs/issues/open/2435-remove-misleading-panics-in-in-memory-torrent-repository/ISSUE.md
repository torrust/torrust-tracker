---
schema-version: 1
doc-type: issue
issue-type: task
status: in-progress
priority: p3
epic: null
github-issue: 2435
spec-path: docs/issues/open/2435-remove-misleading-panics-in-in-memory-torrent-repository/ISSUE.md
branch: "2435-remove-misleading-panics-in-in-memory-torrent-repository"
related-pr: null
last-updated-utc: "2026-10-05 18:42"
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - .github/skills/dev/planning/create-issue/SKILL.md
    - packages/tracker-core/src/torrent/repository/in_memory.rs
    - packages/swarm-coordination-registry/src/swarm/registry.rs
---

<!-- skill-link: create-issue -->

# Issue #2435 - Remove Misleading Panics From the In-Memory Torrent Repository

## Goal

`InMemoryTorrentRepository` in `tracker-core` no longer documents or implements panics for failures that cannot happen, and any genuinely fallible operation returns an error to its caller instead of panicking.

## Background

Ten methods of `InMemoryTorrentRepository` (`packages/tracker-core/src/torrent/repository/in_memory.rs`) call `.expect(...)` on results from the swarm coordination `Registry` and carry a `# Panics` section saying they panic "if the underlying swarms return an error". Panicking in a library path is discouraged, so this looks like a reliability problem.

However, the registry's error type is `pub type Error = Infallible;` (`packages/swarm-coordination-registry/src/swarm/registry.rs`, documented as "The registry currently exposes no recoverable error cases"). The `expect` calls therefore cannot panic today. The problem is that the code and documentation claim a failure mode that does not exist:

- callers and reviewers reasonably conclude these methods can crash the tracker;
- the `# Panics` sections and `expect` messages add noise to every new method (PR #2423 added `get_swarm_metadata` following the same pattern);
- if the registry ever gains a real error variant, these call sites would silently become real panics instead of failing to compile.

Found during PR #2423 (issue #2406) review.

## Scope

### In Scope

- Inventory every `Registry` method returning `Result<_, Error>` and every caller in `tracker-core` that unwraps it.
- Decide, and record the decision, between:
  - **A.** Keep `Result<_, Infallible>` in the registry and destructure it irrefutably in the repository (for example `let Ok(value) = result;`, available since Rust 1.82; MSRV is 1.88), removing the `expect` calls and `# Panics` docs.
  - **B.** Remove `Result` from the infallible registry methods so they return values directly.
  - **C.** Introduce a real registry error and propagate it through `InMemoryTorrentRepository` to its callers.
- Implement the chosen option and update the affected doc comments.
- Fix the registry's own misleading `# Errors` sections (they claim a panic when a lock "cannot be acquired"; `tokio::sync::Mutex::lock` cannot fail).
- Write an ADR for the error-signature policy of public packages.
- Undo the option C work (T4 to T6) after the switch to option B (see [Decision Revision (T7)](#decision-revision-t7)).

### Out of Scope

- Other `expect`/`unwrap` uses outside `InMemoryTorrentRepository` and the registry methods it calls (for example the `MetricCollection::merge` `expect` calls in the REST labeled-stats adapter).
- `.unwrap()`/`.expect()` on registry or repository results in test code, test-support modules (`src/testing/`), examples, and benchmarks.
- Adding `#[non_exhaustive]` to existing public error enums before the first crates.io publish. This belongs to the package-publishing work in EPIC #1669; this issue only drafts the checklist item for maintainer approval.
- Changing swarm-coordination behavior.

## Architectural Decisions

- Related ADRs: [independent package versioning](../../../adrs/20260629000000_adopt_independent_package_versioning.md).
- ADRs to create: a root ADR in `docs/adrs/` recording when public package APIs return `Result`. It is root-scoped because every workspace package will be published and consumed independently (EPIC #1669).

## Decision Revision (T7)

Maintainer decision, 2026-10-05 (supersedes [Decision (T2)](#decision-t2)): **option B**. The registry methods return plain values, and the `Error` type is deleted.

Why the decision changed: after T4 to T6 were implemented, the maintainer asked whether counting the swarms (`Registry::len`) could ever fail, and neither of us could imagine a case. The argument for the counting methods returning `Result` was just as weak: a different backend would affect `len()` equally, and the only remaining candidate, a future lock-acquisition timeout, is speculative. The maintainer prefers a breaking change over filling the code with `Result` "just in case".

The cost of option C was already visible on the branch: `SwarmRegistry` variants in `AnnounceError` and `ScrapeError`, a `StatsError` and a REST `500` path, error handling in the cleanup job, and 37 new `.unwrap()` calls in tests, all for an error that cannot occur and cannot be tested. The plumbing also recreated the original problem: readers infer a failure mode that does not exist.

Publishing context: every workspace package will be published on crates.io within weeks (EPIC #1669), with new crates starting at `0.x`. A breaking change then costs consumers a semver-signalled migration (for example `0.1` to `0.2`), paid once and only if a real failure ever appears. A speculative `Result` costs every consumer from day one.

Policy (recorded in the ADR): a public API returns `Result` when at least one of these holds:

1. a failure can happen today (I/O, parsing, validation, limits);
2. the operation crosses an I/O boundary (database, network, filesystem);
3. it is a trait or port designed for swappable backends, and a realistic backend can fail.

Otherwise it returns a plain value, and a real failure is introduced later as a semver-signalled breaking change. Existing public error enums that have real variants should get `#[non_exhaustive]` before their first publish, so that adding variants is not a breaking change. That is a #1669 checklist item.

Shape:

- `Registry`: the 10 methods return `T` directly; `pub enum Error` and its `compile_fail` doctest are deleted; the `# Errors` sections go away.
- `InMemoryTorrentRepository`: plain delegation with no `expect` and no `# Panics`.
- Revert T6 (propagation) and T5 (REST `StatsError` and `500` path) with `git revert` commits, keeping the history visible.

## Decision (T2)

Superseded by [Decision Revision (T7)](#decision-revision-t7). Kept as history.

Maintainer decision, 2026-10-05: **option C, with an uninhabited `#[non_exhaustive]` error type and full propagation**.

Rationale (maintainer): every package in this workspace is public and usable independently of the tracker. Implementations often change and suddenly need to return an error. Unless an operation can never fail, it should return `Result`, so consumers are ready to handle the error case when it appears. Consumers who investigate can see that no errors happen today. The decision must be documented and honest: if the API returns `Result`, callers propagate it instead of hiding it behind `expect`.

Why not `Infallible`: `Result<_, Infallible>` does not give that forward compatibility. Consumers can write `let Ok(v) = ...;`, `match e {}`, or `impl From<Infallible> for MyError`, and all of these break when the alias becomes a real type. Consumers that `.unwrap()` keep compiling and silently start panicking.

Chosen shape:

- `swarm-coordination-registry` replaces `pub type Error = Infallible` with `#[non_exhaustive] pub enum Error {}` (implementing `Debug`, `Clone`, `Display`, and `std::error::Error`). Inside the registry it is uninhabited, so internal code stays trivial. Other crates cannot treat it as uninhabited (they cannot use exhaustive patterns to prove that `Err` is impossible), so their code stays valid when variants are added, and adding variants later is not a breaking change. Verified on 2026-10-05 with a two-crate scratch build: `let Ok(v) = lib::count();` compiles in the defining crate and fails with `E0005: pattern Err(_) not covered` in the consumer crate.
- `InMemoryTorrentRepository` returns `Result<T, registry::Error>` from the 10 registry-backed methods; no `expect`, no `# Panics`.
- Propagation boundaries (all the way to delivery layers):
  - Announce: `AnnounceError` gains a `SwarmRegistry` variant. HTTP maps it through the existing `TrackerCoreError` → `failure_reason` path. UDP maps it to `ErrorKind::InternalServer` in `udp-server/src/event.rs`.
  - Scrape: `ScrapeError` gains a `SwarmRegistry` variant, mapped the same way as announce.
  - Torrent cleanup: `TorrentsManager::cleanup_torrents` returns `Result`. The cleanup job runner logs the error and keeps running on the next tick (`Completion` has no error variant, and one failed pass must not stop future cleanups).
  - REST stats: `StatsQueryPort::get_stats` and `StatsApiService::get_stats` return `Result` with a `rest-api-application`-owned port error. The `get_stats_handler` responds `500` through the existing `unhandled_rejection_response` pattern.
  - `udp-core` and `udp-server` `statistics::services::get_metrics` return `Result` (public functions with no production caller; their callers are their own tests).

Rejected: option A (keeps the misleading `Infallible` and hides the result in one consumer) and option B (removes `Result`, contrary to the forward-compatibility policy above).

Test consequence: the registry error has no values today, so registry-originated error paths cannot be exercised at runtime. AC3 is guarded by a `compile_fail` doctest on the registry error. The REST `500` mapping is tested with a stub `StatsQueryPort` returning the constructible port error.

## Design and Ownership Review

Not applicable.

## Bug-Fix Process

Not applicable. The bug rule in the `create-issue` and `fix-bug` skills covers observed behavior, and none is observable here: the registry's error type is `Infallible`, so the documented panics cannot occur. Regression protection is AC3.

## Regression Test Strategy

Option B was chosen (T7). It is a compile-time guarantee: if a registry operation ever becomes fallible, its signature changes to `Result` and every caller fails to compile until it handles the error. No runtime test can exercise a failure that does not exist.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID | Status | Task | Notes / Expected Output |
| --- | --- | --- | --- |
| T1 | DONE | Inventory fallible registry methods and repository callers | See [T1 Inventory](#t1-inventory) |
| T2 | DONE | Choose option A, B, or C | Option C; see [Decision (T2)](#decision-t2) |
| T3 | DONE | Write the ADR | `docs/adrs/20261005145329_use_crate_owned_non_exhaustive_errors_for_potentially_fallible_public_apis.md` plus index row and `handle-errors-in-code` skill link |
| T4 | DONE | Registry error type | `#[non_exhaustive] pub enum Error {}`, honest `# Errors` docs, `compile_fail` doctest |
| T5 | DONE | REST stats port returns `Result` | `StatsError` in `rest-api-protocol` (same pattern as `WhitelistError`); `StatsQueryPort`/`StatsApiService::get_stats` return `Result`; handler responds `500` via `failed_to_get_stats_response`; stub-port handler test |
| T6 | DONE | Propagate through `tracker-core` and delivery layers | Repository returns `Result`; `AnnounceError`/`ScrapeError::SwarmRegistry`; `TorrentsManager::cleanup_torrents` returns `Result` and the job logs `tracing::error!`; UDP `ErrorKind::InternalServer`; `udp-core`/`udp-server` `get_metrics` return `Result`; REST adapter maps to `StatsError` |
| T7 | DONE | Revise the decision | Option B; see [Decision Revision (T7)](#decision-revision-t7) |
| T8 | DONE | Rewrite the ADR for the revised policy | `docs/adrs/20261005145329_return_result_only_for_concretely_fallible_public_apis.md` (same timestamp, new slug; not merged yet); index row and `handle-errors-in-code` skill updated |
| T9 | TODO | Revert T6 and T5 | `git revert` commits; spec edits from those commits are kept |
| T10 | TODO | Registry returns plain values | Delete `Error` and the doctest; drop `.unwrap()` on registry calls in tests and examples; remove `expect` and `# Panics` from `in_memory.rs` |
| T11 | TODO | Draft the #1669 pre-publish checklist item | `#[non_exhaustive]` audit of existing public error enums; wording proposed to the maintainer, not committed to the EPIC |

### T1 Inventory

Registry methods returning `Result<_, Error>` (all infallible today): `handle_announcement`, `get_swarm_metadata`, `get_swarm_metadata_or_default`, `get_peers_peers_excluding`, `get_swarm_peers`, `remove_inactive_peers`, `remove_peerless_torrents`, `get_aggregate_swarm_metadata`, `count_peerless_torrents`, `count_peers`.

- `tracker-core`: `in_memory.rs` is the only caller, with one `expect` per method (10 in total) and a matching `# Panics` section.
- Other direct callers: about 63 `.unwrap()` calls in the registry's own tests, `examples/bench_peers.rs`, and `statistics/mod.rs` tests.
- Production callers of the repository methods: `AnnounceHandler`, `ScrapeHandler`, `TorrentsManager` (cleanup and metadata logging), `TrackerStatsAdapter::get_stats` (REST), and `udp-core`/`udp-server` `statistics::services::get_metrics`.
- None of these calls can fail today: every registry method returns `Ok`, and `tokio::sync::Mutex::lock` is infallible.

## Commit Points

| Task | Coherent change set | Commit policy |
| --- | --- | --- |
| T2 | Decision recorded in the spec | Commit after maintainer approval |
| T3 | ADR and index row | One `docs(adrs)` commit |
| T4 | Registry error type and docs | One commit; the workspace still compiles because callers only need `Debug` for `expect` |
| T5 | REST stats port `Result` | One commit; the adapter returns `Ok` until T6 |
| T6 | Propagation through `tracker-core` and delivery layers | One commit (signature changes must land together to compile) |
| T7 | Revised decision in the spec | One `docs(issues)` commit |
| T8 | Rewritten ADR, index row, and skill | One `docs(adrs)` commit |
| T9 | One revert commit per reverted task (T6, then T5) | Each revert compiles on its own |
| T10 | Registry plain values and all callers | One commit |

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/drafts/remove-misleading-panics-in-in-memory-torrent-repository/ISSUE.md`
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue created and issue number added to this spec
- [ ] Implementation completed
- [ ] Automatic verification completed (`linter all`, relevant tests, and any pre-push checks)
- [x] Manual verification scenarios: not applicable (maintainer decision, 2026-10-05; compile-time and automated tests are sufficient)
- [ ] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] Evidence-based implementation completion review recorded: issue-local retrospective created for material discoveries, or progress log states why none was needed
- [ ] Reviewer validated acceptance criteria and updated checkboxes
- [ ] Committer verified spec progress is up to date before commit
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-10-03 07:45 UTC - Copilot - Drafted from a maintainer observation during PR #2423 review; confirmed the registry error type is `Infallible`.
- 2026-10-03 07:58 UTC - Copilot - Maintainer approved the draft; the A/B/C decision is deferred until the draft moves to `docs/issues/open/`. Committed as a draft in PR #2423; no GitHub issue yet.
- 2026-10-05 07:05 UTC - Copilot - Maintainer confirmed the task classification (no runtime defect). Created GitHub issue #2435 and moved the spec to `docs/issues/open/` on a spec-only branch; the A/B/C decision remains open for T2.
- 2026-10-05 08:43 UTC - Copilot - Reworded the Bug-Fix Process paragraph after PR #2436 review (F1): the classification is recorded as outside the bug rule's scope (no observable behavior), not as an exception to it, and AC3 is named as the regression protection.
- 2026-10-05 14:26 UTC - Copilot - Created implementation branch `2435-remove-misleading-panics-in-in-memory-torrent-repository`. T1 inventory recorded. Maintainer chose option C (keep `Result` for forward compatibility of public packages) with a `#[non_exhaustive]` uninhabited error instead of `Infallible`, full propagation to delivery layers, a root ADR, and registry `# Errors` doc fixes. Manual scenario M1 dropped. Work stops at local commits (no push or PR).
- 2026-10-05 15:01 UTC - Copilot - T3 committed (ADR). T4: replaced the `Infallible` alias with `#[non_exhaustive] pub enum Error {}`. `Display` uses the same `match *self {}` as `std`'s `Display for Infallible`, with a documented `expect` for `clippy::uninhabited_references`. The registry crate does not depend on `thiserror`, and adding it for one empty enum was not justified.
- 2026-10-05 15:09 UTC - Copilot - T5: the port error lives in `rest-api-protocol` (`StatsError::TorrentRepository(String)`), following the existing `WhitelistError` boundary pattern, so the application layer does not depend on tracker internals. Added `async-trait = "0.1"` as an `axum-rest-api-server` dev-dependency (already in the lockfile and used by sibling crates) for the stub port. Mutation proof: returning `ok_response()` from the error branch made the test fail (`left: 200, right: 500`); restored by hand.
- 2026-10-05 15:49 UTC - Copilot - T6: propagated the registry error through `tracker-core` and every production caller (see the T6 row). Test seeding helpers in `axum-http-server`/`axum-rest-api-server` `src/testing/environment.rs` use a documented `expect` (test support, out of scope). Test code uses `.unwrap()`. Verified that all 37 added `unwrap`/`expect` lines sit in `#[cfg(test)]` modules or test targets, with none in production. `cargo clippy --workspace --all-targets --all-features` is clean; `cargo test --tests --benches --examples --workspace --all-targets --all-features` passed 2976 tests with 0 failures. Completion review: no retrospective needed. The one material discovery (that `Infallible` defeats forward compatibility) changed the design before implementation and is recorded in the Decision (T2) section and the ADR; the `clippy::uninhabited_references` workaround is recorded in the T4 entry.
- 2026-10-05 18:03 UTC - Copilot - Revised the ADR after an external AI review; the decision is unchanged. Renamed it to "Use Crate-Owned Non-Exhaustive Errors for Potentially Fallible Public APIs". Revisions: separated the abstraction's semantics from the current implementation's capabilities; explained that `Infallible` states the wrong contract; replaced "forced to handle `Err`" with "cannot treat the error as uninhabited" (consumers can still `unwrap`); limited the scope to independently consumed API boundaries; presented the empty-enum pattern as a repository convention built on established practice rather than a standard idiom; added a generic-error alternative and expanded consequences; cited RFC 2008, C-GOOD-ERR, the Reference, and `std::convert::Infallible` (all checked against the live pages). Following the new derive guidance, dropped `PartialEq, Eq` from the registry `Error` because no caller used them and a future variant may not support them. Added an intent comment to the `compile_fail` doctest.
- 2026-10-05 18:42 UTC - Copilot - Maintainer reversed the decision to option B (T7); see [Decision Revision (T7)](#decision-revision-t7). Trigger: no plausible failure for `Registry::len`, and an equally weak case for the counting methods. Packages will be published within weeks (EPIC #1669), so the trade-off was re-assessed in that context: a semver-signalled breaking change, paid once if a failure ever appears, is cheaper than a speculative `Result` every consumer pays for. Reopened AC3, AC5, and AC6 and reworded them for option B. Plan: rewrite the ADR (T8), revert T6 and T5 with `git revert` (T9), make the registry methods return plain values (T10), and draft the #1669 checklist item (T11).

## Acceptance Criteria

- [x] AC1: No method of `InMemoryTorrentRepository` documents a panic that cannot occur.
- [x] AC2: No method of `InMemoryTorrentRepository` calls `expect` or `unwrap` on a registry result.
- [ ] AC3: If the registry gains a real error variant, the repository fails to compile or propagates the error, rather than panicking.
- [ ] `linter all` exits with code `0`
- [x] AC4: The registry `# Errors` docs no longer claim a lock-acquisition failure.
- [ ] AC5: The infallible registry methods return plain values; no `Result`, error variant, or error-response path exists for an error that cannot occur.
- [x] AC6: The ADR records when public package APIs return `Result`.
- [ ] Relevant tests pass
- [ ] Acceptance criteria are re-reviewed after implementation and reflect actual behavior
- [ ] Documentation is updated when behavior/workflow changes

## Verification Plan

### Automatic Checks

- `linter all`
- `cargo test --tests --benches --examples --workspace --all-targets --all-features` (signatures change across several packages)
- `cargo test --doc --workspace`
- Pre-push checks

### Manual Verification Scenarios

None. Maintainer decision on 2026-10-05: the change alters types and error plumbing only; compile-time checks and automated tests are sufficient.

### Disposable Verification Scripts

None planned.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| --- | --- | --- |
| AC1 | TODO | Re-check `in_memory.rs` after T10 |
| AC2 | TODO | Re-check `in_memory.rs` after T10 |
| AC3 | TODO | Option B: the registry signatures are plain values, so introducing an error changes them and every caller fails to compile |
| AC4 | DONE | The misleading sections were corrected in T4; under option B they are removed entirely with T10 |
| AC5 | TODO | Signatures after T9 and T10 |
| AC6 | DONE | [ADR 20261005145329](../../../adrs/20261005145329_return_result_only_for_concretely_fallible_public_apis.md) and its index row |

## Risks and Trade-offs

- Option B changes public `swarm-coordination-registry` signatures (`Result<T, Error>` to `T`) and deletes `Error`. The crate is unpublished, and every caller is in this workspace.
- If a registry operation becomes fallible later, its signature changes again. That is accepted as a semver-signalled breaking change (see the ADR).

## Implementation Completion Review

After implementation, compare the result with this specification. Record
invalidated assumptions, material design changes, unexpected validation
findings, and reusable lessons.

- Retrospective: `Not yet assessed` (the T7 reversal is a material design change; assess after T10)
- If needed, create `implementation-retrospective.md` from the repository
  template at `docs/templates/IMPLEMENTATION-RETROSPECTIVE.md` in this issue
  specification's directory.
- If no retrospective is needed, add a concise progress-log entry explaining
  why the work had no material discovery.

## References

- Related issues: #2406
- Related PRs: #2423
- Related ADRs: none
