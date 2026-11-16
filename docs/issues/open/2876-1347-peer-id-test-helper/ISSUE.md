---
schema-version: 1
doc-type: issue
issue-type: task
status: in-progress
priority: p2
epic: 1347
github-issue: 2876
spec-path: docs/issues/open/2876-1347-peer-id-test-helper/ISSUE.md
branch: "2876-peer-id-test-helper"
related-pr: null
last-updated-utc: "2026-11-16 16:18"
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - .github/skills/dev/debugging/fix-bug/SKILL.md
    - .github/skills/dev/planning/create-issue/SKILL.md
    - docs/adrs/20261021190318_share_workspace_test_support_through_testing_features.md
    - docs/issues/closed/2482-1669-reorganize-shared-test-support/test-support-inventory.md
    - docs/issues/closed/2824-1347-add-tests-primitives/ISSUE.md
    - packages/primitives/src/peer.rs
    - packages/torrent-repository-benchmarking/tests/common/torrent_peer_builder.rs
    - packages/torrent-repository-benchmarking/tests/entry/mod.rs
---

<!-- skill-link: create-issue -->

# Issue #2876 - Move `peer::Id::new` to a realistic test helper and decide whether `peer::fixture` goes behind a `testing` feature

Parent EPIC: #1347 - Overhaul: Packages Testing

## Goal

Take `peer::Id::new`, a test-only helper, out of the production API of `torrust-tracker-primitives`,
replace it with a test helper that builds peer ids a real client could send, and decide whether
`peer::fixture` (and `Id::new` with it) moves behind a `testing` feature, as the ADR on shared test
support asks.

## Background

This is finding F5 of #2824 (tests for `torrust-tracker-primitives`, EPIC #1347). The maintainer
handed it off to its own issue on 2026-11-11, with this direction:

- prove that `Id::new` is not used in production;
- move it to a testing helper that generates realistic ids;
- decide whether `peer::fixture` and `Id::new` move behind a `testing` feature.

The Findings Register of #2824 (row F5) records the finding. What it says, re-checked on `develop` at
`abd7a10ff` (2026-11-15):

- `peer::Id::new` (`packages/primitives/src/peer.rs`, lines 367-382) is `pub` in a production crate.
  It takes any `T: Into<i128>` and builds a 20-byte id: 4 zero bytes, then the 16 little-endian bytes
  of the number. The conversion is correct and lossless, negative numbers included (two's
  complement). It has no doc comment and no tests.
- Its ids look like no real client: `get_client_name()` returns `None` for them.
- `peer::fixture` (lines 498-638) is compiled unconditionally. It holds `PeerBuilder` and the
  `Default` impls of `Peer` and `Id`.
- Both are already public in the published crate: `torrust-tracker-primitives` 3.0.0 on crates.io
  (tag `v3.0.0`) has `pub fn new<T>` and `pub mod fixture`. Its version in the workspace is 3.1.0.
- The ADR
  [`20261021190318_share_workspace_test_support_through_testing_features.md`](../../../adrs/20261021190318_share_workspace_test_support_through_testing_features.md)
  says that test support other packages use sits behind a `testing` feature that is off by default
  (criterion 1, rules A and D). EPIC #2482, which applied it, is closed and did not touch
  `peer::fixture`. Its
  [inventory](../../closed/2482-1669-reorganize-shared-test-support/test-support-inventory.md)
  searched for module names such as `testing`, `test_helpers`, `environment` and `fixtures`
  (plural), which is a likely reason the singular `peer::fixture` was missed.
- #2824 measured that the `Id::new` lines (12) and the `peer::fixture` lines (55) are the only
  uncovered lines left in `peer.rs` (plus one dead `Display` branch), and left their tests to this
  issue. When this spec was drafted (`develop` at `abd7a10ff`) the package also had 9 uncovered
  lines in `swarm_metadata.rs`; the S4 slice of #2824 (#2864) covered them (31 / 31), and #2824's
  final measurement puts all 68 uncovered lines of the package (1,078 / 1,146) in `peer.rs`.
- #2824 also found that the fixture's own ids (`-qB00000000000000000`, `...01`, `...02`) are not valid
  Azureus-style ids (the character after the four version characters must be `-`), so the client
  registry reports no client for them. Its new tests use `-qB4650-abcdefghijkl`, which the registry
  knows as qBittorrent.

The behavior of `Id::new` is not wrong. The problem is where it lives and what kind of ids it makes:
test data in a production API, with ids no tracker ever receives.

### Caller inventory (`develop` at `abd7a10ff`, 2026-11-15)

Searched with `grep -rn --include=*.rs` over `packages/`, `src/` and `tests/`, then checked by
compiling (below). Re-run at implementation start (T1) on `develop` at `c8b1477ad` (2026-11-16):
every table below is unchanged, and the compile proof again exited 0 with no warnings
(`manual-verification-evidence.md`, V1).

**`peer::Id::new`**: three calls, all in the integration tests of one package, which has
`publish = false`.

| File | Line | Use |
| --- | --- | --- |
| `packages/torrent-repository-benchmarking/tests/common/torrent_peer_builder.rs` | 8 | `a_completed_peer(id: i32)` |
| `packages/torrent-repository-benchmarking/tests/common/torrent_peer_builder.rs` | 20 | `a_started_peer(id: i32)` |
| `packages/torrent-repository-benchmarking/tests/entry/mod.rs` | 402 | 75 distinct peers (`1..=75`) to pass the scrape limit |

The two wrappers take an `i32` and pass it on, so their call sites are callers too. Every call site
in the package, with its arguments:

| File | Lines | Call and arguments |
| --- | --- | --- |
| `tests/entry/mod.rs` | 75, 80, 85, 93, 96, 99, 401 | `a_started_peer` / `a_completed_peer` with `1`, `2` or `3` |
| `tests/entry/mod.rs` | 192, 233 | `a_started_peer(-1)` |
| `tests/entry/mod.rs` | 427 | `a_completed_peer(-1)`; the test asserts `peers.len() + 1` after the upsert, so `-1` must give an id distinct from the made peers `1` to `3` |
| `tests/repository/mod.rs` | 84, 91, 98, 110, 115, 120 | `a_started_peer` / `a_completed_peer` with `1`, `2` or `3` |
| `tests/repository/mod.rs` | 140, 155 | `a_started_peer(i)`, with the `i32` loop variable over `0..408` (zero included); `i` also feeds `InfoHash::from(&i)` and `hash.write_i32(i)` |
| `tests/repository/mod.rs` | 548 | `a_completed_peer(-1)` |

(Paths are under `packages/torrent-repository-benchmarking/`.) So the callers pass zero and the
negative sentinel `-1`, which stands for "a peer not among the made ones". They depend on distinct ids
for distinct numbers, but not on the byte layout or on the `i128` range.

**`peer::fixture`** (`PeerBuilder`): imported by six packages besides `primitives`. Every import in
`src/` sits after the file's first `#[cfg(test)]`.

| Package | Where |
| --- | --- |
| `axum-http-server` | `src/` unit tests (1 file), `tests/` (5 files) |
| `axum-rest-api-server` | `tests/` (2 files) |
| `swarm-coordination-registry` | `src/` unit tests (2 files, 8 imports) |
| `torrent-repository-benchmarking` | `src/` unit tests (1 file), `tests/` (1 file) |
| `udp-core` | `src/` unit tests (1 file) |
| `udp-server` | `src/` unit tests (2 files), `benches/` (1 file) |

**`Peer::default()`** (its `impl Default` lives in `peer::fixture`): also used, outside the files
above, by `#[cfg(test)]` code in `http-core` (`src/event.rs`), `swarm-coordination-registry`
(`src/event.rs`) and the root crate (`src/app.rs`). Of these, only `http-core` and the root crate
are packages not listed above.

**Compile proof.** A throwaway change put `#[cfg(test)]` on the `impl Id` block that holds `new` and
on `pub mod fixture`, then ran `cargo check --workspace --lib --bins` (stable, cargo 1.99.1). It
finished with exit code 0 and no warnings, and the change was reverted. So no library or binary
target of the workspace uses `Id::new`, `PeerBuilder`, or the `Default` impls of `Peer` and `Id`.
Only test, bench and `#[cfg(test)]` code does. Code outside this workspace that depends on the
published 3.0.0 is not covered by this proof.

## Scope

### In Scope

- Re-run the caller inventory and the compile proof at the start of implementation.
- A test helper that builds realistic peer ids (ids the client registry recognizes), with unit
  tests, in the place decided in Q1.
- Move the three `torrent-repository-benchmarking` callers to it.
- Take `Id::new` out of the unconditional public API (remove or deprecate, Q3).
- Apply the `peer::fixture` decision (Q2, Q5): either gate it behind a `testing` feature of
  `primitives` and update every consumer's `[dev-dependencies]`, or record why it stays.
- If the fixture's ids change (Q6), update the tests that assert on them.
- Tests, or a recorded no-test decision, for every line of the new helper and of the fixture that
  remains, so `primitives` has no uncovered line owned by F5.
- The ADR record the decisions need (Q7).

### Out of Scope

- Other behavior of `peer::Id` (`From`, `TryFrom`, `Display`, `Serialize`, `get_client_name`): #2824
  tested it.
- Peer id literals written by hand in other packages' tests and benches (for example
  `-qB00000000000000001` in `tracker-core` and root `tests/`, and `-qB00000000000000000` in the
  `http-core` bench helper). They are not in
  `primitives`. If Q6 shows they should change, a separate issue does it.
- Publishing a new version of `primitives` (#2583 owns version bumps).
- The `testing` feature of other packages.

## Architectural Decisions

- Related ADRs:
  [`20261021190318_share_workspace_test_support_through_testing_features.md`](../../../adrs/20261021190318_share_workspace_test_support_through_testing_features.md)
  (rules A, D and E; the ownership map has no row for `peer::fixture`) and
  [`20260629000000_adopt_independent_package_versioning.md`](../../../adrs/20260629000000_adopt_independent_package_versioning.md)
  (removing a public item is a breaking change for the published crate).
- ADRs to create: none expected. An amendment of the test-support ADR is expected (Q7), adding a
  row for `peer::fixture` to the ownership map. That would be the first amendment to add a row: the
  amendments of #2681 and #2699 only added a dated note at the top of the ADR (the first qualifies
  an existing row in prose, the second settles the examples question).

During implementation, stop and create an ADR when a decision affects project
architecture or design patterns, selects an approach among meaningful
alternatives, or has consequences future contributors need to understand. Do not
create ADRs for routine implementation details or style choices already governed
by project conventions.

## Open Design Decisions (for the maintainer)

Each question has a recommendation. Nothing is implemented until the maintainer answers them.

**Answers (the maintainer, in chat, 2026-11-16).** Every recommendation was accepted:

| Q | Decision |
| --- | --- |
| Q1 | (a) The helper lives in `primitives`, in `peer::fixture` next to `PeerBuilder`. |
| Q2 | Yes. `peer::fixture` moves behind a `testing` feature of `primitives`. |
| Q3 | (a) Remove `Id::new`, guarded by a `compile_fail` doctest. |
| Q4 | (b) The helper takes a `u32`; the `-1` sentinel becomes a named constant outside the made range, and the `0..408` loops iterate over `u32` and convert only where an `i32` is needed. |
| Q5 | Gate `impl Default for Peer` and `impl Default for Id` with the module. |
| Q6 | Yes, the three fixture ids become realistic (built with the helper); literals in other packages stay out of scope. |
| Q7 | Amend the test-support ADR. |

**Q1. Where does the realistic-id helper live?**

- (a) In `primitives`, next to `PeerBuilder` in `peer::fixture` (behind `testing` if Q2 says so).
  The fixture and its ids are in one place, and the six packages that use `PeerBuilder` can use
  realistic ids too.
- (b) In `torrent-repository-benchmarking/tests/common/` (ADR rule E: one consuming package). No
  public API, but the fixture keeps its own unrealistic ids, and other packages cannot reuse it.
- (c) In `torrust-tracker-test-helpers`. Ruled out by ADR rule B.1: that crate may not depend on
  `primitives`.

Recommendation: (a).

**Q2. Does `peer::fixture` move behind a `testing` feature of `primitives`?**

If yes: `#[cfg(feature = "testing")]` on the module, `testing` never in `default`, the path-only self
dev-dependency (`torrust-tracker-primitives = { path = ".", features = ["testing"] }`, ADR rule A.3),
and a `features = ["testing"]` `[dev-dependencies]` entry in each consumer (six packages, plus
`http-core` and the root crate if Q5 gates the `Default` impls). Most consumers list `primitives`
under `[dependencies]` today, so each needs a second entry under `[dev-dependencies]`.

Recommendation: yes. ADR criterion 1: a normal build contains no test-only code unless the consumer
opts in. The compile proof shows production does not need it.

**Q3. What happens to `Id::new`?**

- (a) Remove it. Its only callers are in a `publish = false` package. Removing a public item of a
  published crate is a breaking change, recorded for #2583.
- (b) Mark it `#[deprecated]` now, pointing at the new helper, and remove it in a later breaking
  release.
- (c) Keep it, but behind `testing` with the fixture (Q2).

Recommendation: (a). A `compile_fail` doctest can guard the removal, as F1's fix did for `Ord`.

**Q4. What does the helper's API and id format look like?**

The callers need distinct ids for distinct numbers, including zero and the negative sentinel `-1`
(see the caller inventory). The format: a known client prefix (for example qBittorrent `-qB4650-`,
used by #2824's tests) followed by the number written in the 12 remaining bytes, as zero-padded
decimal (readable in test failures) or as hex. A `u64` does not fit in either form (`u64::MAX` needs
20 decimal or 16 hex digits), so the choice is between two 32-bit inputs:

- (a) Keep `i32`, the type the wrappers take today. The sign goes in the first of the 12 bytes (for
  example `-` or `0`) and the absolute value in the other 11 as decimal digits (`i32::MIN` needs 10).
  No call site changes its argument.
- (b) Take a `u32` (10 decimal digits, fits). The wrappers take a `u32` too, and the call sites
  change: `-1` becomes a named constant outside the made range (for example
  `const OTHER_PEER: u32 = 1000;`). The two `0..408` loops need a `u32` for the wrapper while `i` also
  feeds `InfoHash::from(&i)` and `hash.write_i32(i)`: either keep `i` an `i32` and pass
  `u32::try_from(i).expect(...)` (a `Result`, so it must be unwrapped), or iterate over `0..408_u32`
  and convert only for those two uses.

Recommendation: (b). The sentinel means "a peer not among the made ones", and a name says that
better than `-1`; the ids also stay a plain number after the prefix. Unit tests check that
`get_client_name()` is `Some("qBittorrent")`, that different numbers give different ids, and the
exact bytes of one example.

**Q5. What happens to `impl Default for Peer` and `impl Default for Id`?**

They sit inside `peer::fixture`, but trait impls apply everywhere, so they are part of the public API.
A "default peer" is test data. If Q2 gates the module they are gated too, which removes
`Peer: Default` and `Id: Default` from production builds. The alternative is to move them out of the
module and keep them unconditional.

Recommendation: gate them with the module. The compile proof shows no library or binary needs them.

**Q6. Do the fixture's own ids change to realistic ones?**

`-qB00000000000000000`, `...01` and `...02` are not valid Azureus-style ids. Changing them may
break tests that compare against these bytes (for example JSON or hex in `axum-http-server` and
`axum-rest-api-server` tests). The implementation would first list those assertions.

Recommendation: yes, the three fixture ids only (built with the Q4 helper). Literals in other
packages stay out of scope.

**Q7. How is the decision recorded?**

Recommendation: an amendment of the test-support ADR (the decision stands; `peer::fixture` gets a
row in the ownership map and rule D's reasoning covers shared fixtures, not only mocks). If the
maintainer prefers, a short note in this spec is enough.

## Design and Ownership Review

This is reusable test-fixture work, so ownership applies; the process, I/O and deadline items do not.

- `primitives` owns `Peer`, `Id` and the shared fixture that builds them. With Q1 (a) it also owns
  the realistic-id helper.
- Consumers own their own test data and reach the fixture only through `[dev-dependencies]`
  (ADR rule A.4). No `[dependencies]` entry enables `testing`.
- Lifetime and deadline invariants: not applicable (pure, synchronous value builders).
- Design-review checkpoint: after the helper and its first unit test pass (T2), stop for maintainer
  review of the helper's name, signature and format before migrating callers.

## Bug-Fix Process

F5 is a design problem, not a behavior defect: `Id::new` returns the bytes it promises, for every
input. So the `fix-bug` sequence
([fix-bug](../../../../.github/skills/dev/debugging/fix-bug/SKILL.md)) is adapted:

- **Analysis.** Done in this spec: test-only data in a production API, with ids that no client
  sends, and fixture code compiled into every build against ADR criterion 1.
- **Reproduction.** Instead of reproducing wrong behavior, reproduce the finding: re-run the caller
  inventory and the compile proof (gate both items with `#[cfg(test)]`, run
  `cargo check --workspace --lib --bins`, revert). Record commands and output in
  `manual-verification-evidence.md`. Show that today's fixture ids give `get_client_name() == None`.
- **Regression boundary.** See below.
- **Red evidence.** The helper's unit tests are written first and fail (the helper does not exist,
  or returns ids with no client name).
- **Change.** Helper, caller migration, `Id::new` removal and the fixture gate, each in its own
  commit (see Commit Points).
- **Green evidence and recheck.** All tests pass, and the compile proof is repeated on the final
  code without a throwaway change.

## Regression Test Strategy

Two things must not regress: the behavior the callers depend on, and the boundary between test
support and production code.

- **Caller behavior.** The `torrent-repository-benchmarking` tests that use the helper must pass,
  in particular the scrape-limit test that adds 75 peers and needs 75 distinct ids, and the test at
  `tests/entry/mod.rs:427` that needs the sentinel peer to differ from the made peers. With Q4 (a)
  only the helper call changes. With Q4 (b) the call sites that pass `-1` or the loop variable also
  change their arguments (see the caller inventory), and each change must keep the ids distinct where
  the test relies on it.
- **Helper behavior.** Unit tests in `primitives`, at the helper itself: recognized client name,
  distinct ids for distinct inputs, exact bytes of one example. Follow the `write-unit-test` skill.
- **Test-support boundary.** If Q2 is yes: `cargo check -p torrust-tracker-primitives` (no
  features) must compile without the fixture, and the repository's own check of shipped features,
  `cargo run -p shipped-features-check -- --package torrust-tracker` (the `shipped-features` job in
  `.github/workflows/testing.yaml`), must exit 0. It reads only normal and build edges. A plain
  `cargo tree -e features` would not do: it keeps dev-dependency edges, so it would list the
  `testing` feature that the self dev-dependency and the consumers' `[dev-dependencies]` enable. If Q3 is (a): a `compile_fail` doctest
  shows that `Id::new` is gone.

Red and green output and the final recheck go in issue-local `manual-verification-evidence.md`.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task | Notes / Expected Output |
| --- | ------ | ---- | ----------------------- |
| T0  | DONE   | Get the maintainer's answers to Q1-Q7 | Answers recorded in this spec; later tasks adjusted. |
| T1  | DONE   | Re-run the caller inventory and the compile proof | Tables above updated if `develop` moved; evidence in `manual-verification-evidence.md`. |
| T2  | DONE   | Add the realistic-id helper with unit tests | In the Q1 location, with the Q4 format; design-review checkpoint after the first passing test. |
| T3  | DONE   | Move the `torrent-repository-benchmarking` callers to the helper | Its tests pass; ids are distinct. |
| T4  | DONE   | Remove or deprecate `Id::new` (Q3) | `compile_fail` doctest if removed. |
| T5  | DONE   | Gate `peer::fixture` (and the `Default` impls, Q5) behind `testing` (Q2) | Feature in `primitives/Cargo.toml` with the semver comment of ADR rule A.6, self dev-dependency, consumer `[dev-dependencies]`; `cargo machete --with-metadata` clean. |
| T6  | TODO   | Make the fixture's ids realistic (Q6) | Assertions on the old bytes listed first, then updated. |
| T7  | TODO   | Tests or a no-test decision for every remaining fixture line | Issue-local `coverage-evidence.md` (see Coverage Evidence) shows no line owned by F5 left uncovered. |
| T8  | TODO   | Record the decision (Q7) | ADR amendment, or a note in this spec. |
| T9  | TODO   | Final verification and acceptance review | `linter all`, tests, manual scenarios, AC table filled in. |

## Commit Points

| Task | Coherent change set | Commit policy |
| ---- | ------------------- | ------------- |
| T0   | The maintainer's answers to Q1-Q7, recorded in this spec | Commit before T1 (`docs(issues):`). |
| T1   | Updated inventory tables, `manual-verification-evidence.md` (V1 baseline) and the coverage baseline | Commit after the inventory and the compile proof run. |
| T2   | The helper and its unit tests | Commit after focused validation and the design review. |
| T3   | Caller migration in `torrent-repository-benchmarking` | Commit after its tests pass. |
| T4   | `Id::new` removal or deprecation (+ `compile_fail` doctest) | Commit after `cargo test -p torrust-tracker-primitives`. |
| T5   | `testing` feature and consumer manifests | Commit after `cargo test --workspace` and `cargo machete`. |
| T6   | Fixture ids and the assertions on them | Commit after the affected packages' tests pass. |
| T7   | Fixture tests and coverage evidence | Commit after focused validation and the design review. |
| T8   | ADR amendment | Commit separately (`docs(adrs):`). |
| T9   | Final evidence, AC table and progress log | Commit after the final verification. |

Record a justified no-change decision in the task's evidence without creating an empty commit. For
test-producing work, use the `write-unit-test` skill and complete an explicit design review after
each passing test increment, before maintainer review and commit. Confirm that the test exposes the
one causal initial-state difference; its fixture owns only incidental mechanics; and the production
Act plus independently specified expected result remain visible. The review must use the mandatory
prose-first Arrange-Act-Assert comparison: write temporary prose for each section, refactor until
the code expresses it, remove redundant prose, and retain only irreducible context. Record this
review in task evidence or a file-local test plan. Assess helper boundaries by meaningful named
actions and abstraction-level alignment, not caller count: a single-use helper is valid when it
keeps the test readable and hides only incidental mechanics. Commit each reviewed test-design
increment before starting the next planned file or behavior area. Use a Conventional Commit message
with the narrow affected scope, and sign every commit with GPG.

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/drafts/peer-id-test-helper/ISSUE.md`
- [x] Draft spec reviewed and merged through #2865 (filing approved by the maintainer in chat, 2026-11-16)
- [x] Open design questions Q1-Q7 answered by the maintainer (before T0)
- [x] GitHub issue created (#2876) and issue number added to this spec
- [x] (Optional, recommended for complex issues) Spec-only PR merged into `develop` before implementation
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

- 2026-11-15 12:42 UTC - Claude Code (Opus 5.5) - Drafted from finding F5 of #2824 (maintainer
  hand-off of 2026-11-11). Caller inventory re-run on `develop` at `abd7a10ff`; compile proof
  (`cargo check --workspace --lib --bins` with `Id::new` and `peer::fixture` under `#[cfg(test)]`)
  passed and was reverted. Found that both items are public in the published 3.0.0, and that the
  `Default` impls of `Peer` and `Id` live in the fixture module (Q5). (Stamp corrected in review
  round 1: it first said 12:45, after the commit.)
- 2026-11-15 20:32 UTC - Claude Code (Opus 5.5) - Review round 1 of PR #2865 (da2ce7 F1-F11,
  Copilot two comments). The inventory now lists the wrapper call sites, which pass `0` and `-1`;
  Q4 weighs `i32` against `u32` with a named sentinel; M2 and the regression strategy use
  `shipped-features-check`, which exited 0 for `torrust-tracker` on `abd7a10ff`; a Coverage
  Evidence section and commit points for T0, T1 and T9 were added; smaller wording fixes.
- 2026-11-15 21:27 UTC - Claude Code (Opus 5.5) - Review round 2 of PR #2865 (da2ce7, approved with
  two Nit findings on this spec): Q4 (b) now names both ways to pass the loop variable as a `u32`
  (`u32::try_from(i)` returns a `Result`), and the `swarm_metadata.rs` coverage note is dated and
  updated with #2824's final measurement.
- 2026-11-16 05:57 UTC - Claude Code (Opus 5.5) - Review round 3 of PR #2865 (da2ce7, approved with
  one Nit): the coverage note now cites #2824's final rows, 1,078 / 1,146, instead of the "After
  `swarm_metadata.rs`" row. (Entry added in review round 4: round 3 changed the spec without one.)
- 2026-11-16 06:08 UTC - Claude Code (Opus 5.5) - Review round 4 of PR #2865 (da2ce7, approved with
  one Nit on this spec): `last-updated-utc` bumped and the round-3 entry above added.
- 2026-11-16 07:54 UTC - Claude Code (Opus 5.5) - Rebased onto `develop` after #2871 moved the #2824
  spec to `docs/issues/closed/`; the `related-artifacts` entry and the References path now point there.
- 2026-11-16 08:45 UTC - Claude Code (Sonnet 5.5) - Filed as #2876 under EPIC #1347 after #2865 merged
  (`a25a27e23`). Moved from `docs/issues/drafts/peer-id-test-helper/` to
  `docs/issues/open/2876-1347-peer-id-test-helper/`; `epic`, `github-issue`, `spec-path` and `branch`
  set; status `planned`. Q1-Q7 are still unanswered; nothing is implemented.
- 2026-11-16 09:41 UTC - Claude Code (Opus 5.5) - Review round 1 of PR #2878 (da2ce7 F1-F5, Copilot
  two re-raises): `branch` is now the `-spec` branch, the parent EPIC line is under the title, the
  approval checkpoint is split from the Q1-Q7 answers, and the F5 owner cells of EPIC row 11 and the
  closed #2824 spec point at #2876.
- 2026-11-16 15:57 UTC - Claude Code (Opus 5.5) - The spec-only PR #2878 merged and the maintainer
  answered Q1-Q7 in chat, accepting every recommendation (Q4 (b): `u32` with a named sentinel).
  Answers recorded under Open Design Decisions; status `in-progress` on branch
  `2876-peer-id-test-helper`. T0 done.
- 2026-11-16 15:59 UTC - Claude Code (Opus 5.5) - T1 on `develop` at `c8b1477ad`: the caller
  inventory is unchanged and the compile proof exited 0 with no warnings (V1); a scratch test
  reproduced the finding, every fixture and `Id::new` id gives `get_client_name() == None` (V0); the
  coverage baseline is 1,075 / 1,143 lines, with the 68 uncovered lines all in `peer.rs`
  (`coverage-evidence.md`).
- 2026-11-16 16:05 UTC - Claude Code (Opus 5.5) - T2: `peer::fixture::qbittorrent_peer_id(u32) -> Id`
  (`const fn`, `-qB4650-` + 12 zero-padded decimal digits) with three unit tests, red first, each
  guarded by a mutation; the maintainer approved its name, signature and format at the design-review
  checkpoint (`test-plans/qbittorrent-peer-id.md`).
- 2026-11-16 16:07 UTC - Claude Code (Opus 5.5) - T3: the `torrent-repository-benchmarking`
  tests use `qbittorrent_peer_id`; `-1` is the named `OTHER_PEER` (1000) and the `0..408` loops
  iterate over `u32`. Same tests pass (17 and 1,468); with `OTHER_PEER` set to a made number, the
  cutoff test fails (`test-plans/qbittorrent-peer-id.md`, Caller Migration).
- 2026-11-16 16:09 UTC - Claude Code (Opus 5.5) - T4: `Id::new` removed. A `compile_fail,E0599`
  doctest on `peer::Id`, with a companion that builds an id from 20 bytes, fails when `Id::new` is
  put back (stable and nightly). The break adds to the 4.0.0 that `primitives` already owes (#2583).
- 2026-11-16 16:18 UTC - Claude Code (Opus 5.5) - T5: `peer::fixture` and the `Default` impls are
  behind the new `testing` feature of `primitives` (self dev-dependency, rule A.6 comment). Built
  member by member, eight packages needed a `[dev-dependencies]` entry: the six of the inventory plus
  `http-core` and the root crate (Q5). Machete, packaging, M2 and `cargo test --workspace` (4,087
  passed) are green (`test-plans/qbittorrent-peer-id.md`, `testing` Feature).

## Acceptance Criteria

- [ ] AC1: The caller inventory and the compile proof are re-run at implementation start and at the
  end, and show that no library or binary target uses `Id::new` or the fixture.
- [ ] AC2: `Id::new` is no longer part of the unconditional public API of `primitives`, as decided
  in Q3.
- [ ] AC3: A test helper builds peer ids that the client registry recognizes
  (`get_client_name()` is `Some`), gives distinct ids for distinct inputs, and has unit tests.
- [ ] AC4: The `torrent-repository-benchmarking` tests use the helper and pass.
- [ ] AC5: The Q2 and Q5 decisions are applied: either `cargo check -p torrust-tracker-primitives`
  without features compiles no fixture code and every consumer enables `testing` only from
  `[dev-dependencies]`, or the spec records why the fixture stays unconditional.
- [ ] AC6: Every line of the helper and of the fixture that remains is covered by a unit test or has
  a recorded no-test decision; issue-local `coverage-evidence.md` shows no uncovered line owned by
  F5.
- [ ] AC7: The decision is recorded as Q7 says.
- [ ] `linter all` exits with code `0`
- [ ] Relevant tests pass
- [ ] Manual verification scenarios are executed and documented in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria are re-reviewed after implementation and reflect actual behavior
- [ ] Documentation is updated when behavior/workflow changes

## Verification Plan

### Automatic Checks

- `linter all`
- `cargo test -p torrust-tracker-primitives` and `cargo test -p torrust-tracker-torrent-repository-benchmarking`
  (stable Rust toolchain)
- `cargo test --workspace` (stable), because the fixture has consumers in six packages
- `cargo machete --with-metadata` (pre-commit gate)
- `cargo package --workspace --no-verify` (stable; ADR criterion 5: the self dev-dependency has no
  `version`)
- Pre-push checks

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID  | Scenario | Human-oriented command/steps | Expected Result | Status | Evidence |
| --- | -------- | ---------------------------- | --------------- | ------ | -------- |
| M1  | Production builds need no test support | Before the change: the throwaway gate and `cargo check --workspace --lib --bins`. After: the same command on the final code, no throwaway change. | Exit code 0 both times | TODO | `manual-verification-evidence.md` section V1 |
| M2  | The release tracker carries no `testing` feature of `primitives` (if Q2 is yes) | `cargo run -p shipped-features-check -- --package torrust-tracker`, and as the readable view `cargo tree -p torrust-tracker -e normal,build,features -i torrust-tracker-primitives` | The check exits 0; the tree shows no `testing` feature of `primitives` | TODO | `manual-verification-evidence.md` section V2 |
| M3  | Realistic ids | Print a few helper ids and their `get_client_name()` in a scratch test; compare with the old fixture ids | New ids name a client; old ones give `None` | TODO | `manual-verification-evidence.md` section V3 |

Notes:

- Manual verification is mandatory even when automated tests pass.
- Record the toolchain for every command result (for example `stable Rust toolchain`, cargo 1.99.1).
- Create `manual-verification-evidence.md` from `docs/templates/MANUAL-VERIFICATION-EVIDENCE.md`.
- If a scenario fails, record the failure and diagnosis in the progress log before proceeding.

### Coverage Evidence

Coverage is measured for AC6, so the issue keeps an issue-local `coverage-evidence.md`:

- The command, the same as #2824 uses (stable Rust toolchain, `cargo-llvm-cov`):
  `cargo llvm-cov clean --workspace`, then
  `cargo llvm-cov -p torrust-tracker-primitives --all-features --lib --json --output-path <report>.json`.
  It covers the `src/` code of `primitives`, run by the package's own unit tests only (`--lib`).
  `--all-features` turns on `testing` if Q2 adds it, so the fixture is measured.
- Separate unit-only and aggregate tables (aggregate adds `cargo llvm-cov -p
  torrust-tracker-primitives --all-features` without `--lib`), each with a baseline at T1 and a
  final measurement at T9.
- Per-file rows for `peer.rs` (and the helper's file, if it is new), with the uncovered lines or
  functions listed and an owner for each.
- Concise Markdown only; no raw JSON, LCOV or HTML reports are committed.

### Disposable Verification Scripts

None planned. The compile proof is a temporary source edit, reverted at once, and recorded as
commands and output.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| ----- | ---------------------- | -------- |
| AC1   | TODO | |
| AC2   | TODO | |
| AC3   | TODO | |
| AC4   | TODO | |
| AC5   | TODO | |
| AC6   | TODO | |
| AC7   | TODO | |

## Risks and Trade-offs

- Removing `Id::new`, the fixture or the `Default` impls breaks code outside the workspace that
  uses the published 3.0.0. Mitigation: Q3 (b) deprecates first; #2583 owns the version bump.
- A `testing` feature turned on by `--all-features` or `--all-targets` reaches a shipped binary
  (ADR rule A.5). Mitigation: M2, and the release build rules of #2482 already in place.
- Changing the fixture ids may change many test expectations. Mitigation: Q6 limits the change to
  the three fixture ids, and T6 lists the assertions first.
- `develop` moves under the work. Mitigation: T1 re-runs the inventory at the start.

## Implementation Completion Review

After implementation, compare the result with this specification. Record
invalidated assumptions, material design changes, unexpected validation
findings, and reusable lessons.

- Retrospective: `Not yet assessed`
- If needed, create `implementation-retrospective.md` from
  `docs/templates/IMPLEMENTATION-RETROSPECTIVE.md` in this directory.
- If no retrospective is needed, add a concise progress-log entry explaining
  why the work had no material discovery.
- When an independent reviewer receives this folder-style specification, it
  records its result in `agent-review-reports.md` using
  `docs/templates/AGENT-REVIEW-REPORTS.md`.

## References

- Origin: finding F5 of #2824 (Findings Register in
  `docs/issues/closed/2824-1347-add-tests-primitives/ISSUE.md`), EPIC #1347
- Test-support reorganization: EPIC #2482 (closed) and its
  [inventory](../../closed/2482-1669-reorganize-shared-test-support/test-support-inventory.md)
- Version bumps: #2583
- Related ADRs:
  [test support through `testing` features](../../../adrs/20261021190318_share_workspace_test_support_through_testing_features.md),
  [independent package versioning](../../../adrs/20260629000000_adopt_independent_package_versioning.md)
