---
schema-version: 1
doc-type: issue
issue-type: task
status: planned
priority: p2
epic: 2482
github-issue: 2681
spec-path: docs/issues/open/2681-2482-gate-tracker-core-test-modules-and-mocks/ISSUE.md
branch: "2681-2482-gate-tracker-core-test-modules-and-mocks"
related-pr: null
last-updated-utc: "2026-10-28 12:31"
semantic-links:
  skill-links:
    - create-issue
    - add-rust-dependency
    - write-unit-test
  related-artifacts:
    - docs/adrs/20261021190318_share_workspace_test_support_through_testing_features.md
    - docs/issues/open/2482-1669-reorganize-shared-test-support/EPIC.md
    - docs/issues/open/2482-1669-reorganize-shared-test-support/test-support-inventory.md
    - docs/issues/closed/2618-2482-decide-test-support-rules/manual-verification-evidence.md
    - docs/issues/closed/2670-2482-build-shipped-artifacts-without-test-features/ISSUE.md
    - packages/tracker-core/Cargo.toml
    - packages/tracker-core/src/lib.rs
    - packages/tracker-core/src/databases/mod.rs
    - packages/tracker-core/src/databases/traits/mod.rs
    - packages/tracker-core/src/databases/traits/torrent_metrics.rs
    - packages/http-core/Cargo.toml
    - packages/udp-core/Cargo.toml
    - deny.toml
    - .github/workflows/testing.yaml
---

<!-- skill-link: create-issue -->

# Issue #2681 - Gate `tracker-core`'s test modules and mocks

Parent EPIC: #2482 (Reorganize test support code shared between workspace packages), row 3 of its
Subissues table. Draft written by subissue 1 (#2618); refreshed on 2026-10-28 against `develop` at
`883cc834c` after rows 2, 8 and 9 merged, and its design questions settled with the maintainer the
same day. It is the first move that adds a `testing` feature to a real crate, so it is also the
first time the `Shipped Features` job of #2670 has something to check.

## Goal

Apply rules C and D of the
[ADR](../../../adrs/20261021190318_share_workspace_test_support_through_testing_features.md) to
`tracker-core`: make its crate-private test helper modules private, and compile the one mock that
other packages' tests use (`MockTorrentMetricsStore`) only behind a non-default `testing` feature,
so `mockall` leaves `tracker-core`'s normal dependencies and no `Mock*` type is in its default
public API.

## Background

Patterns C and D of the
[inventory](../../open/2482-1669-reorganize-shared-test-support/test-support-inventory.md). Facts below
were re-verified on `develop` at `883cc834c` on 2026-10-28.

### Pattern C: three `pub mod` declarations of test-only code

| Declaration | File:line | Content |
| --- | --- | --- |
| `pub mod peer_tests;` | `packages/tracker-core/src/lib.rs:131` | `#![cfg(test)]` unit tests of `Peer` |
| `pub mod test_helpers;` | `packages/tracker-core/src/lib.rs:132` | `#[cfg(test)] pub(crate) mod tests { ... }` fixtures |
| `pub mod test_helpers;` | `packages/tracker-core/src/whitelist/mod.rs:23` | `#[cfg(test)] pub(crate) mod tests { ... }` whitelist service setup |

Nothing ships (the content is `#[cfg(test)]`), but each declaration adds an empty public module to
the API. Nothing outside `tracker-core` imports them (`grep` for `tracker_core::test_helpers`,
`tracker_core::peer_tests` and `whitelist::test_helpers` outside `packages/tracker-core/src` finds
nothing). The module documentation of `whitelist/mod.rs` (line 17) lists `test_helpers` as one of
the module's parts.

### Pattern D: four unconditional `#[automock]` traits

`mockall` is a normal dependency (`packages/tracker-core/Cargo.toml` line 28) and also a
dev-dependency (line 46). The four traits in `packages/tracker-core/src/databases/traits/` carry an
unconditional `#[automock]` and a `use mockall::automock;` line, and the generated mocks are
re-exported at `databases/mod.rs` (line 66) and `databases/traits/mod.rs` (lines 11, 13, 14, 15).
Users of each mock, found by `grep` over the workspace (excluding `target/`, `docs/pr-reviews/` and
closed specs):

| Mock | Used in `tracker-core` | Used in other packages |
| --- | --- | --- |
| `MockTorrentMetricsStore` | `scrape_handler.rs`, `statistics/event/handler.rs` (both `#[cfg(test)]`) | `http-core` `services/announce.rs` and `services/scrape.rs`; `udp-core` `services/announce.rs` and `services/scrape.rs` (all `#[cfg(test)]`) |
| `MockAuthKeyStore` | `authentication/handler.rs` (`#[cfg(test)]`) | None |
| `MockWhitelistStore` | None (declared and re-exported only) | None |
| `MockSchemaMigrator` | None (declared and re-exported only) | None |

So only `MockTorrentMetricsStore` is shared test support. The other three are used, if at all, by
`tracker-core`'s own unit tests, and ADR rule D.2 sends such a mock to `#[cfg_attr(test, automock)]`
(the pattern of `events`). `#[cfg_attr(test, automock)]` alone cannot serve
`MockTorrentMetricsStore`, because `cfg(test)` is only set while a crate compiles its own tests;
the four consumer modules would stop compiling.

### Facts from earlier rows that this one relies on

- `workspace-coupling` labels an optional normal dependency `optional (<features>)` and skips self
  edges (row 8, #2645).
- The `Containerfile` builds the shipped binaries in their own `cargo build` stages, and the CI job
  `Shipped Features` (`contrib/dev-tools/checks/shipped-features-check`, `testing.yaml`) fails if a
  workspace package has a `testing` feature enabled in
  `cargo tree -p torrust-tracker -e normal,build` (row 9, #2670). Every `features = ["testing"]`
  edge must therefore be in `[dev-dependencies]`.
- The test archives and `deployment-packages.yaml` still use `--all-features`; replacing them with
  named features is a follow-up after rows 3 to 7, not part of this issue.

### Baseline (measured on 2026-10-28, `cargo test -p <package>`, no `--all-features`)

| Package | Unit tests | Integration tests | Doc tests |
| --- | --- | --- | --- |
| `torrust-tracker-core` | 155 | 12 (`tests/integration.rs`) | 16 passed |
| `torrust-tracker-http-core` | 191 | none | 1 passed |
| `torrust-tracker-udp-core` | 99 | none | 3 passed, 3 ignored |

The `--all-features` baseline is recorded by T1.

### Findings of the exploratory prototype (2026-10-28, discarded)

The prototype put all four mocks behind `testing` and was reverted; the real change redoes it with
the narrower scope of D1. It showed two forced side effects that the earlier draft did not
mention:

1. `cargo deny check bans` fails with the self dev-dependency: `torrust-tracker-core` is banned
   except for listed `wrappers`, and the package becomes its own wrapper. The fix is to add
   `torrust-tracker-core` to its own `wrappers` list in `deny.toml` (as
   `torrust-tracker-axum-health-check-api-server` already does for itself). `cargo machete
   --with-metadata` stays clean.
2. Clippy's `redundant_pub_crate` rejects `pub(crate) mod tests` inside the now-private helper
   modules. See D5.

It also showed that, with the self dev-dependency, plain `cargo test -p` of the three packages
passes with the baseline counts, and that the `Shipped Features` check passes on the real tree and
fails (exit 1) when `features = ["testing"]` is put on the root crate's `torrust-tracker-core`
dependency. These results are prototype evidence only; the manual scenarios below record the real
ones.

## Scope

### In Scope

- Rule C: declare `peer_tests` and both `test_helpers` modules as `#[cfg(test)] mod`, not
  `pub mod`; remove `test_helpers` from the module documentation of `whitelist/mod.rs`; make the
  inner `pub(crate) mod tests` modules `pub mod tests` (D5).
- Rule D for `MockTorrentMetricsStore`: add the `testing` feature (not in `default`) with
  `testing = ["dep:mockall"]`, make `mockall` an optional normal dependency
  (`mockall = { version = "0", optional = true }`), generate the mock with
  `#[cfg_attr(feature = "testing", automock)]`, and gate its `use mockall::automock;` line and its
  two re-exports (`databases/mod.rs`, `databases/traits/mod.rs`) with `#[cfg(feature = "testing")]`.
- Rule D.2 for the other three mocks: `#[cfg_attr(test, automock)]`, with `#[cfg(test)]` on their
  `use mockall::automock;` lines and re-exports. `mockall` stays a dev-dependency.
- The lint attributes of `auth_keys.rs` that cite `automock` stay valid under the `cfg_attr`.
- Rule A points 3, 4 and 6 for the feature: the path-only self dev-dependency in
  `[dev-dependencies]` (`torrust-tracker-core = { path = ".", features = ["testing"] }`, no
  `version`); the `[dev-dependencies]` entries in `http-core` and `udp-core` that enable
  `features = ["testing"]` of `torrust-tracker-core`; a comment on the feature saying it is test
  support outside semver; and no `testing` in `[package.metadata.docs.rs]` (none exists today).
- The `deny.toml` change of the prototype finding 1, in the same commit as the self
  dev-dependency, because the pre-commit gate runs `cargo deny check bans` on every commit.
- Audit every other normal dependency of `tracker-core`, workspace or third-party, against rule
  A.2 of the ADR (see T1).
- Update the inventory rows for patterns C and D, and an amendment note in the ADR for rule D.1
  (D1).

### Out of Scope

- Mocks used only inside their own crate in other packages (`events`,
  `swarm-coordination-registry`, `http-core` and `udp-core` benches, `udp-server`).
- Removing `MockWhitelistStore` and `MockSchemaMigrator`, which nothing uses (dead test support).
  They keep existing under `cfg(test)` to keep this change behaviour-preserving; deleting them is a
  separate cleanup.
- The server environments (rows 4 to 7) and the protocol helpers (row 2).
- Replacing `--all-features` in the test archives and in `deployment-packages.yaml`.
- Any version bump or publishing (#2583).

## Architectural Decisions

- Related ADRs: [`20261021190318`](../../../adrs/20261021190318_share_workspace_test_support_through_testing_features.md)
  (rules C, D and A; ownership map: `tracker-core` owns its database mocks).
- ADRs to create: None. Rule D.1 of the ADR gets a short dated amendment note (D1, T7).

### Decisions settled with the maintainer (2026-10-28)

#### D1 - Which mocks go behind `testing`

| Option | Strengths | Weaknesses | Result |
| --- | --- | --- | --- |
| All four behind `testing` (literal ADR rule D.1) | One rule for the four sibling files; nothing to decide for the next mock | Three of the four mocks are not used by any other package, so the public `testing` surface carries test support nobody outside needs; contradicts the principle that code used only inside a crate stays private (ADR rule D.2, option O5) | Rejected |
| Only `MockTorrentMetricsStore` behind `testing`; the other three `#[cfg_attr(test, automock)]` | Public test surface is exactly what other packages use; follows D.2 for the three crate-private mocks | The four sibling traits use two different attributes; deviates from the sentence "applies this to all four database traits" in rule D.1 | **Chosen** (maintainer, 2026-10-28: "only what other crates need is shared") |
| Delete the three unshared mocks | Smallest surface | Changes tests and removes test support: not behaviour-preserving, and `MockAuthKeyStore` is used by `tracker-core` tests | Rejected (out of scope) |

The ADR's rule D.1 says `tracker-core` applies the feature to all four traits "so the rule is the
same for each of them". That sentence was written before the consumers of each mock were counted
per mock, and it conflicts with rule D.2 and with the ownership map, which names only
`MockTorrentMetricsStore` as used by other packages. T7 adds an amendment note to the ADR instead
of rewriting it.

#### D2 - `mockall` as an optional dependency

| Option | Strengths | Weaknesses | Result |
| --- | --- | --- | --- |
| `mockall = { version = "0", optional = true }` enabled by `testing = ["dep:mockall"]` | Leaves `tracker-core`'s normal dependency tree in a default build; no implicit `mockall` feature; matches ADR rule D.1 | `cargo metadata` reports it as a normal edge (row 8 labels it `optional (testing)`); needs a `version` because it is a normal dependency for packaging | **Chosen** |
| Keep `mockall` a normal dependency and gate only the attributes | Smallest manifest change | `mockall` stays in every build, so the stated goal is not met | Rejected |

`mockall` also stays in `[dev-dependencies]` for the `cfg(test)` mocks and the tests'
`mockall::predicate` imports.

#### D3 - Mechanism for the targets that need the feature

| Option | Strengths | Weaknesses | Result |
| --- | --- | --- | --- |
| Path-only self dev-dependency in `tracker-core` (ADR O1c) | Plain `cargo test -p torrust-tracker-core` builds unit tests, `tests/` and doc tests with `testing` on; `cargo package` strips it | `cargo metadata` lists a self edge (row 8 skips it); needs a `deny.toml` wrapper entry (prototype finding 1) | **Chosen** (ADR) |
| `--features testing` / `--all-features` only (O1a) | No manifest change | Plain `cargo test -p` fails to compile the unit tests that use `MockTorrentMetricsStore` | Rejected by the ADR |
| `required-features` on targets (O1b) | Builds without the feature | Silently skips targets; banned | Rejected by the ADR |
| `#[cfg_attr(any(test, feature = "testing"), automock)]` on `MockTorrentMetricsStore` | Unit tests of `tracker-core` need no self dev-dependency | Does not help `tests/` or other packages; two gates to keep in step; the ADR chose O1c uniformly so rows 4 to 7 use one pattern | Rejected |

`http-core` and `udp-core` enable the feature from their own `[dev-dependencies]` (ADR rule A.4);
no `[dependencies]` entry enables it.

#### D4 - Form of the consumers' dev-dependency entry

`torrust-tracker-core = { version = "0.1.0", path = "../tracker-core", features = ["testing"] }` in
`http-core` and `udp-core`, the same form as their normal dependency. A path-only form would also
package (the dev-dependency is stripped), but nothing requires it: `tracker-core` does not depend
back on either package, so the packaging cycle of ADR evidence V3 does not apply, and a versioned
entry keeps the two entries of one package consistent. The self dev-dependency in `tracker-core`
itself is path-only, as rule A.3 requires.

#### D5 - Inner `pub(crate) mod tests` becomes `pub mod tests`

Once `test_helpers` is private, `clippy::redundant_pub_crate` (denied by the workspace flags)
rejects `pub(crate) mod tests` inside it. The options are `pub mod tests` (clippy's own
suggestion; the module is still unreachable outside the crate because its parent is private and
`#[cfg(test)]`) and keeping `pub(crate)` with a documented `#[allow(clippy::redundant_pub_crate,
reason = "...")]` (more noise, a lint suppression for no benefit). Chosen: `pub mod tests`.

#### D6 - How the `automock` import is gated

`#[cfg(test)] use mockall::automock;` (three files) and `#[cfg(feature = "testing")] use
mockall::automock;` (`torrent_metrics.rs`), next to `#[cfg_attr(..., automock)]`, instead of
writing `#[cfg_attr(..., mockall::automock)]` with no import. The import form keeps the existing
short-name style and a one-line diff per attribute.

## Design and Ownership Review

`tracker-core` owns the mock of its `TorrentMetricsStore` trait; `http-core` and `udp-core` reach
it only through `[dev-dependencies]` with `features = ["testing"]`. The other three mocks and the
helper modules belong to `tracker-core`'s own unit tests and become invisible outside them. No
lifetimes, I/O or process ownership change, and no test body changes. After the first slice that
adds the feature (T4), check that `cargo build -p torrust-tracker-core` and
`cargo build -p torrust-tracker-core --features testing` both compile and that the consumers'
tests still find `MockTorrentMetricsStore`. T2 and T3 add no feature, so their checkpoint is the
package's plain `cargo test` and clippy.

## Bug-Fix Process

Not applicable.

## Regression Test Strategy

Not applicable as a bug regression. The existing unit, integration and doc tests of the three
packages are the regression suite and their counts must not drop. The new guard against a
`testing` feature reaching a shipped binary already exists (`Shipped Features` job, #2670); this
issue gives it a real feature to check and records red/green evidence on the real tree (M4).

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID | Status | Task | Notes / Expected Output |
| --- | --- | --- | --- |
| T1 | TODO | Record the baseline and audit the dependencies | `cargo tree -p torrust-tracker-core -e normal --depth 1` (shows `mockall`); test counts of the three packages from four commands each, `cargo test -p <package>`, `cargo test -p <package> --all-features`, `cargo test -p <package> --all-targets --all-features` and `cargo test -p <package> --doc --all-features` (`--all-targets` does not run doc tests, so the doc-test count comes from the plain and `--doc` runs); re-`grep` the five `Mock*` names across the workspace; audit every other normal dependency of `tracker-core` against ADR rule A.2 (only `#[cfg(test)]` users become dev-dependencies, the audit result is recorded even if empty) |
| T2 | TODO | Rule C: make the three helper modules private | `#[cfg(test)] mod`; `pub mod tests` inside; doc bullet removed. `cargo test -p torrust-tracker-core` and clippy pass |
| T3 | TODO | Rule D.2: the three crate-private mocks use `cfg_attr(test, automock)` | Imports and re-exports gated by `cfg(test)`; tracker-core tests pass |
| T4 | TODO | Rule D: gate `MockTorrentMetricsStore` behind `testing`, add the feature, optional `mockall`, self dev-dependency, `deny.toml` wrapper, and the consumers' dev-dependency entries | One commit: the gate runs `cargo machete` and `cargo deny` on every commit. Plain `cargo test -p` passes for the three packages |
| T5 | TODO | Verify | Without-feature build and tree; tests with and without `--all-features`; `cargo machete --with-metadata`; `cargo deny check bans`; Shipped Features red/green; the coupling report's `normal` and `dev` entries and the `cargo metadata` view of the optional `mockall` (M5); `cargo +stable package --workspace --no-verify` |
| T6 | TODO | Update the inventory (patterns C and D resolved) and check the docs and diagram | `docs/packages.md` and the READMEs mention no mock; the 2026-10-17 coupling report is not edited |
| T7 | TODO | Add the dated amendment note to ADR rule D.1 (D1) | Short note; the ADR index needs no change |
| T8 | TODO | Manual verification evidence, acceptance review, completion review, independent pre-PR review | `manual-verification-evidence.md`; retrospective if there are lessons |

## Commit Points

| Task | Coherent change set | Commit policy |
| --- | --- | --- |
| T1 | Baseline notes go to the evidence file | Commit with the evidence file once it has content. |
| T2 | The three module declarations, the inner modules, the doc bullet | Commit after `cargo test -p torrust-tracker-core` and clippy pass. |
| T3 | The three `cfg_attr(test, ...)` traits with their imports and re-exports | Commit after `cargo test -p torrust-tracker-core` passes. |
| T4 | Feature, optional dependency, `cfg_attr`, gated import and re-exports, self dev-dependency, `deny.toml`, consumers' entries, `Cargo.lock` | One commit after the three packages' tests and `cargo deny check bans` pass without `--all-features`. |
| T6 | Inventory | One commit. |
| T7 | ADR amendment note | One commit. |
| T8 | Evidence, review and retrospective | Commit after the evidence is real. |

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/drafts/2482-gate-tracker-core-test-modules-and-mocks/ISSUE.md` (now `docs/issues/open/2681-2482-gate-tracker-core-test-modules-and-mocks/ISSUE.md`)
- [x] Spec reviewed and approved by user/maintainer (the maintainer, 2026-10-28)
- [x] GitHub issue created, linked as a sub-issue of #2482 (`parent_issue_url` verified), and its number added to this spec
- [ ] Spec-only PR merged into `develop` before implementation
- [ ] Implementation completed
- [ ] Automatic verification completed (`linter all`, relevant tests, and pre-push checks)
- [ ] Manual verification scenarios executed and recorded in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] Evidence-based implementation completion review recorded
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-10-21 19:16 UTC - AI assistant (Copilot SDK in VS Code, #2482 subissue 1) - Drafted from row 3 of
  the #2482 Subissues table and rules C and D of the ADR. The scope includes the consumers' entries
  because `MockTorrentMetricsStore` is used by `http-core` and `udp-core` (inventory correction).
- 2026-10-22 18:10 UTC - AI assistant (Copilot SDK in VS Code, #2482 subissue 1) - Changed after PR #2624's review: AC1 and M1 check direct dependencies (`--depth 1`); the scope names the `use mockall::automock;` lines and the `Mock*` re-exports that must also be gated, and the dependency audit.
- 2026-10-22 18:39 UTC - AI assistant (Copilot SDK in VS Code, #2482 subissue 1) - Changed after PR #2624's review round 2: the `Mock*` re-exports in `databases/traits/mod.rs` are at lines 11, 13, 14 and 15 (all four mocks), not only line 14.
- 2026-10-28 12:05 UTC - AI assistant (Copilot SDK in VS Code, #2482 subissue 3) - Refreshed against `develop` at `883cc834c` after rows 2, 8 and 9. Re-verified every module declaration, line number and consumer of each `Mock*` type; measured the plain-test baseline; ran a discarded prototype that found the `deny.toml` wrapper and the `redundant_pub_crate` side effects. The maintainer decided on 2026-10-28 that only `MockTorrentMetricsStore` (the one mock other packages use) goes behind `testing` and the other three use `cfg_attr(test, automock)` (D1), and accepted the defaults of D4 to D6. The scope, tasks, acceptance criteria and scenarios changed accordingly. Awaiting the maintainer's approval of the spec.
- 2026-10-28 12:07 UTC - AI assistant (Copilot SDK in VS Code, #2482 subissue 3) - The maintainer approved the spec and the T7 ADR amendment note on 2026-10-28. Created issue #2681, linked it as a sub-issue of #2482 (`parent_issue_url` verified), moved the spec to `docs/issues/open/2681-2482-gate-tracker-core-test-modules-and-mocks/`, set `status: planned` and updated row 3 of the EPIC Subissues table (IN_PROGRESS) and the inventory links.

## Acceptance Criteria

- [ ] AC1: `cargo tree -p torrust-tracker-core -e normal --depth 1` shows no direct dependency on
      `mockall` in a default build, and a default build's public API contains none of the five
      `Mock*` types and none of the three helper modules. (`mockall` can remain a transitive
      dependency through other crates; say which when recording.)
- [ ] AC2: With `--features testing`, `torrust_tracker_core::databases::MockTorrentMetricsStore` exists
      and `mockall` is a direct normal dependency; `MockAuthKeyStore`, `MockWhitelistStore` and
      `MockSchemaMigrator` do not exist outside `cfg(test)`.
- [ ] AC3: `cargo test -p torrust-tracker-core`, `-p torrust-tracker-http-core` and
      `-p torrust-tracker-udp-core` pass without `--all-features`, and each target's test count
      equals the baseline (unit, integration and doc tests, counted separately); the same
      comparison holds for the other three T1 commands per package (`--all-features`,
      `--all-targets --all-features` and `--doc --all-features`), each against the count its own
      baseline command recorded.
- [ ] AC4: Every `features = ["testing"]` edge on `torrust-tracker-core` is in a
      `[dev-dependencies]` table; the `Shipped Features` check exits 0 on the real tree, and exits 1
      when the edge is temporarily moved to `[dependencies]` (evidence, not committed).
- [ ] AC5: `cargo machete --with-metadata` and `cargo deny check bans` pass, with the self
      dev-dependency having no `version`; `cargo +stable package --workspace --no-verify` passes.
- [ ] AC6: Inventory patterns C and D are marked resolved, and the ADR carries the D.1 amendment note.
- [ ] `linter all` exits with code `0`; pre-push checks pass.

## Verification Plan

The [#2482 default completion policy](../../open/2482-1669-reorganize-shared-test-support/EPIC.md#how-we-work-in-this-epic)
applies. This is an O1 move: build the package without the feature, run `cargo test -p <package>`
without `--all-features`, compare test counts, and make sure no test is silently skipped
(`required-features` is not used).

### Automatic Checks

- `linter all` (via the pre-commit gate on every commit) and the pre-push script before the PR.
- For each of `torrust-tracker-core`, `torrust-tracker-http-core` and `torrust-tracker-udp-core`, the
  four commands of T1 (`cargo test -p <package>`, `... --all-features`,
  `... --all-targets --all-features` and `... --doc --all-features`), each count compared with the
  baseline recorded by the same command.
- `cargo clippy` on the three packages with `--all-targets`, with and without `--all-features`.
- `cargo machete --with-metadata`, `cargo deny check bans`,
  `cargo +stable package --workspace --no-verify`.
- `cargo run --quiet --package shipped-features-check -- --package torrust-tracker`.
- `cargo test --workspace --all-targets --all-features` (the shape of the test archives) and, if
  Docker is available, the native-tracker tests in both targets (`tests/common/native_tracker`,
  with and without `TORRUST_TRACKER_BINARY`).

### Manual Verification Scenarios

| ID | Scenario | Human-oriented command/steps | Expected Result | Status | Evidence |
| --- | --- | --- | --- | --- | --- |
| M1 | The default build has no test dependency | `cargo tree -p torrust-tracker-core -e normal --depth 1`, before and after; `cargo build -p torrust-tracker-core` | `mockall` is a direct dependency before and absent after | TODO | `manual-verification-evidence.md` V1 |
| M2 | Each package's tests run with a plain command | `cargo test -p torrust-tracker-core`, `-p torrust-tracker-http-core`, `-p torrust-tracker-udp-core` (no `--all-features`) | Counts equal the baseline for every target | TODO | V2 |
| M3 | A consumer still gets the mock in its tests and not in its production graph | `cargo test -p torrust-tracker-http-core -- scrape` and `-p torrust-tracker-udp-core -- scrape`; then, before and after the change, `cargo tree -p torrust-tracker-http-core -e normal -i mockall` and the same for `torrust-tracker-udp-core` | The tests that use `MockTorrentMetricsStore` run and pass; before the change each `cargo tree -i` prints `mockall` reached through `torrust-tracker-core`, after it `mockall` is not in the normal graph of either consumer (`cargo tree` resolves the graph itself, so a cached build cannot hide it) | TODO | V3 |
| M4 | The shipped binary cannot get the feature | Run `shipped-features-check` on the real tree; temporarily put `features = ["testing"]` on a `[dependencies]` entry of `torrust-tracker-core`; run it again; restore | Exit 0, then exit 1 naming `torrust-tracker-core`, then exit 0 again | TODO | V4 |
| M5 | The dependency edges are reported as designed | Run `cargo run -p workspace-coupling -- /tmp/<report>.md` (an output path outside the tracked reports) and `cargo metadata --format-version 1 --no-deps`, and read the `torrust-tracker-core` dependency entries | In the report, `http-core` and `udp-core` list `torrust-tracker-core` under both `[normal]` and `[dev]` headings, and `tracker-core` has no self edge. The report lists only workspace packages, so `mockall` is absent from it by design; the optional `mockall` is checked in the `cargo metadata` output instead (a normal dependency of `torrust-tracker-core` with `optional: true`, enabled by the `testing` feature `["dep:mockall"]`), where it appears before and after the change | TODO | V5 |
| M6 | The feature is visible only when asked | `cargo doc -p torrust-tracker-core --no-deps` with and without `--features testing` | `MockTorrentMetricsStore` appears only with the feature; the three helper modules never appear | TODO | V6 |

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| ----- | ---------------------- | -------- |
| AC1   | TODO                   |          |
| AC2   | TODO                   |          |
| AC3   | TODO                   |          |
| AC4   | TODO                   |          |
| AC5   | TODO                   |          |
| AC6   | TODO                   |          |

## Risks and Trade-offs

- `cfg_attr` ordering with `#[async_trait]` and the lint attributes may change what `mockall`
  generates. Mitigation: run the three packages' tests and clippy after each trait.
- Other consumers of the mocks may exist outside `http-core` and `udp-core`; T1 re-greps the
  five `Mock*` names across the workspace before editing, including benches and examples.
- A `testing` feature unifies across one Cargo invocation: `cargo test --workspace` builds
  `tracker-core` with `testing` on because `http-core` enables it. That is harmless for tests;
  ADR rule A.5 and the `Shipped Features` job keep it out of the shipped binary.
- The self dev-dependency adds a `tracker-core -> tracker-core` dev edge that `cargo deny` treats
  as a wrapper. The `deny.toml` entry is part of the same commit.
- Making `peer_tests`, `test_helpers` and `whitelist::test_helpers` private, and removing the
  three unshared mocks and `mockall` from the default API, is a breaking change for a crate that
  is already at 0.1.0 in the workspace but not yet published (#2583 decides versions).
- Deviating from the literal text of ADR rule D.1 (D1) leaves two attribute forms in sibling
  files. The amendment note and a comment in `traits/mod.rs` are the mitigation.

## Implementation Completion Review

- Retrospective: `Not yet assessed`
- If no retrospective is needed, add a progress-log entry saying why.

## References

- Parent EPIC: [`#2482`](../../open/2482-1669-reorganize-shared-test-support/EPIC.md); ADR above
- Inventory patterns C and D
- [#2618 manual verification evidence](../../closed/2618-2482-decide-test-support-rules/manual-verification-evidence.md) (V1 to V4c)
- [#2670 spec](../../closed/2670-2482-build-shipped-artifacts-without-test-features/ISSUE.md) (the `Shipped Features` job)
