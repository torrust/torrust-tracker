---
schema-version: 1
doc-type: issue
issue-type: bug
status: in-progress
priority: p3
epic: null
github-issue: 2798
spec-path: docs/issues/open/2798-peer-ord-inconsistent-with-eq/ISSUE.md
branch: "2798-peer-ord-inconsistent-with-eq"
related-pr: 2811
last-updated-utc: "2026-11-10 17:46"
semantic-links:
  skill-links:
    - create-issue
    - fix-bug
  related-artifacts:
    - .github/skills/dev/debugging/fix-bug/SKILL.md
    - .github/skills/dev/planning/create-issue/SKILL.md
---

<!-- skill-link: create-issue -->

# Issue #2798 - `Peer`'s `Ord` is inconsistent with its `Eq`

## Goal

Make `torrust_tracker_primitives::peer::Peer` honour the standard-library contract that
`a.cmp(&b) == Ordering::Equal` if and only if `a == b`.

## Background

### What happens

`Peer` (`packages/primitives/src/peer.rs`) has seven public fields: `peer_id`, `peer_addr`,
`updated`, `uploaded`, `downloaded`, `left`, `event`. Two of its trait impls disagree about what
makes two peers "the same":

- `PartialEq`, `Eq` and `Hash` are **derived**, so they compare (and hash) **all seven fields**.
- `Ord` is **hand-written** and compares **`peer_id` only**; `PartialOrd` delegates to it.

Step by step: take a seeder `a` and a copy `b` with `left = 5` (so `b` is a leecher). `a == b` is
`false` because `left` differs, but `a.cmp(&b)` is `Ordering::Equal` because the `peer_id`s match.

### Contract violated

The `Ord` documentation requires `cmp` to be consistent with `PartialEq`: `a.cmp(&b) ==
Equal` must hold exactly when `a == b`. `Peer` is part of a published crate
(`torrust-tracker-primitives`), so this is a public API contract violation. Code that relies on
the contract (`BTreeSet<Peer>`, `binary_search`, `min`/`max`, `BinaryHeap`) can silently treat different peers as
duplicates, or lose one of them.

### Impact today

Latent, but wider than direct use of `Peer`:

- No production code puts `Peer` in a `BTreeSet`, sorts it, or takes `min`/`max` of it. The
  production swarm is `Coordinator` in
  `packages/swarm-coordination-registry/src/swarm/coordinator.rs`, which stores
  `BTreeMap<SocketAddr, Arc<PeerAnnouncement>>` (`PeerAnnouncement` is an alias of `Peer`), so it
  identifies a peer by its socket address and never orders `Peer` values.
- `PeerList` and `Torrent` (`packages/torrent-repository-benchmarking`, `publish = false`, a
  benchmark) **derive** `Ord`/`PartialOrd`, so they inherit the defect through the `Arc<Peer>`
  values of their `BTreeMap<PeerId, Arc<Peer>>`
  (`packages/torrent-repository-benchmarking/src/entry/peer_list.rs`): two `PeerList`s with the
  same peer ids but different peer data compare `Equal` while being `!=`. Nothing in the
  workspace orders those types either (audit below).
- The existing test `should_be_comparable` only checks `==`/`!=`; nothing tests `cmp`.

## Audit of usages (workspace, develop at `eaae0e487`)

Searched all workspace packages, `src/` and `console/` for `Peer`/`PeerList`/`Torrent` used with
`BTreeSet`, `HashSet`, `BinaryHeap`, `sort*`, `min`/`max`, `cmp`/`partial_cmp`, and for `derive(..Ord..)`
on types that contain a `Peer`.

- Direct `Ord` use of `Peer`: none outside its own `impl` in `peer.rs`.
- Production swarm (`swarm-coordination-registry`, `Coordinator`): keyed by `SocketAddr`; no
  ordering, sorting or set of `Peer`, and no derived `Ord` on `Coordinator`.
- Derived `Ord` that transitively uses `Peer::cmp`: `PeerList` and `Torrent`
  (`torrent-repository-benchmarking`), neither of which is ordered anywhere.
- `Eq`/`Hash` on `Peer` are consistent with each other (both derived over all fields).
- External consumers of the published crate cannot be audited; that is the breaking-change risk
  for options A and B.

## Options

| Option | Change                                                                       | Public API impact                                                                                                                                                                                                                                                                                                                  | Verdict                                                                                                                                                             |
| ------ | ---------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| A      | Remove `Ord`/`PartialOrd` from `Peer`                                        | Breaking: external sorters stop compiling; `PeerList`/`Torrent` must also drop their derived `Ord`                                                                                                                                                                                                                                 | **Chosen** by the maintainer (see Decision)                                                                                                                         |
| B      | Make `Eq`/`Hash` compare `peer_id` only                                      | Breaking in behaviour (silent): `assert_eq!` on peers would ignore state changes; tests such as `should_be_comparable` change meaning                                                                                                                                                                                              | Rejected: equality over the full announce state is relied on by tests and is the safer semantics; a peer with different `left`/`event` is observably different data |
| C      | Make `Ord`/`PartialOrd` agree with `Eq`: compare all fields, `peer_id` first | Non-breaking for ordering of distinct `peer_id`s (same primary key and order); only ties are now broken by the remaining fields. The derive variant needs new `Ord` impls on `NumberOfBytes` and `AnnounceEvent` (additive, but it makes their order public); the hand-written variant needs none (see the considered alternative) | Rejected: keeps an ordering nobody uses                                                                                                                             |
| D      | Document only                                                                | None                                                                                                                                                                                                                                                                                                                               | Leaves the contract violation; acceptable only as a fallback                                                                                                        |

### Decision: A (maintainer, after T1)

After T1 the maintainer chose **A**: remove `Ord` and `PartialOrd` from `Peer`. Reasons: T1 found
no production code that orders a `Peer`; the breaking change is accepted because few external
consumers of `Peer: Ord` are expected (not verified), even though `torrust-tracker-primitives` is
publishable (`publish = true` through the workspace, version 3.1.0), so its next publish must be a
major version (see Risks and Trade-offs); and removing the ordering is the simplest way to make the
contract violation impossible, because no ordering is left to keep in step with `Eq`. The ordering of the non-id fields in option C would
be arbitrary, and the id-only order was an accident of the `AnnounceEvent` swap. The analysis of C
below is kept as the considered alternative.

### Considered alternative: C (not chosen)

It restores the contract with the smallest blast radius: no removal of public API, no change to
equality or hashing, and the existing order of peers with different ids is unchanged because
`peer_id` stays the first compared field. Two variants, to compare on their real cost:

- **Derive variant**: derive `PartialOrd, Ord` on `Peer`. A derive (or any tuple of the fields) is
  `Ord` only when every element is, and `NumberOfBytes` (`number_of_bytes.rs:1`) and
  `AnnounceEvent` (`announce.rs:112`) derive only `PartialEq, Eq, Hash, Clone, Copy, Debug`. So it
  requires adding `PartialOrd, Ord` to both types, which makes `AnnounceEvent`'s variant order and
  `NumberOfBytes`'s `i64` order part of their public API. The earlier all-field derive on `Peer`
  (see T1 history) was this variant.
- **Hand-written variant**: a hand-written `cmp` that compares `peer_id`, `peer_addr`, `updated`,
  `uploaded.0`, `downloaded.0`, `left.0` (the `i64` inside `NumberOfBytes`) and a **private rank**
  of `AnnounceEvent` (a private `match` mapping each variant to an integer), then `Eq` stays
  consistent because the projection is one-to-one. No new impls on other types and no new public
  ordering, at the cost of a private rank that must be kept in step with `AnnounceEvent`'s variants
  (a `match` without a wildcard makes the compiler flag new variants).

## Intent and call-site classification

Maintainer position (hypothesis to verify in T1, not yet a finding): whole-`Peer` equality is
probably used only by tests (`assert_eq!` on peers), and the `peer_id`-only ordering probably
exists for the few places that really meant "same peer id". Production should compare the whole
structure and say so explicitly (`a.peer_id == b.peer_id`) where only the id matters. `peer_id`
is chosen by each client in a decentralized way, so the tracker does not control it. Two peers
with the same `peer_id` but different other fields are either a bug, a (very unlikely) collision,
deliberate spoofing, or simply one client announcing from two addresses: the production swarm
(`Coordinator`) keys by `SocketAddr`, so such entries can coexist there. An implicit id-only
comparison would hide all of these. Comparing
only the id may be marginally cheaper, but that is not worth the risk of silent behavior.

T1 must produce, before the fix is chosen:

- **History**: why the `peer_id`-only `Ord` replaced an all-field derive. Known so far (verified
  with `git show` and `git log -S'impl Ord for Peer'`, which finds no earlier commit):
  - `9a43815d0` (2024-02-13, move out of `src/core`): `Peer` derived only `PartialEq, Eq, Debug,
Clone, Serialize, Copy`; no `Ord`, no `Hash`.
  - `e18cae46e` ("dev: torrent repository cleanups", 2024-03-25): added a derived `PartialOrd, Ord,
Hash` over all fields, consistent with `Eq`.
  - `00af70f25` ("dev: remove announce event wrapper", 2024-07-14): replaced that derive with the
    hand-written `peer_id`-only `Ord`/`PartialOrd`; the same diff swapped the crate-local
    `AnnounceEvent` wrapper for `aquatic_udp_protocol::AnnounceEvent`.

  T1 finds out why `00af70f25` changed the ordering (most likely the replacement `AnnounceEvent`
  had no `Ord`, so the derive stopped compiling; to be confirmed from the commit, its PR and
  issue) and records what the authors intended. The earlier all-field derive is also the derived
  variant of option C.

- **Call-site table**: every place in the workspace that compares peers (`==`, `!=`, `assert_eq!`,
  `assert_ne!`, `cmp`, `sort`, set/map keys, `Hash`), split into production vs test code, and each
  classified as "needs whole-peer equality", "only wanted the peer id", or "unused".
- **Conclusion** on whether production code needs any ordering of `Peer` at all. If not, option A
  (remove `Ord`) becomes attractive for the in-repo code, with its breaking-change cost for the
  published crate, and any id-only comparison becomes an explicit `peer_id` comparison.

### T1 findings (investigation done)

**History.** The old crate-local `AnnounceEvent` wrapper derived `PartialOrd, Ord`
(`git show 00af70f25^:packages/primitives/src/announce_event.rs`). The replacement,
`aquatic_udp_protocol::AnnounceEvent`, has no `Ord`, so the all-field derive on `Peer` stopped
compiling in `00af70f25`. The commit message is only "dev: remove announce event wrapper" and
names no reason, so this is an inference from the diff, not a documented intent. The `peer_id`-only
`Ord` is the smallest code that restores `Ord` and fixes the build; nothing in the commit says
that id-only ordering was wanted. The ordering was added and replaced without any call site that
needs it (see below), which fits an accident of the dependency swap rather than a design.

**Call sites** (searched `packages/`, `src/`, `tests/` for `==`, `!=`, `assert_eq!`/`assert_ne!`,
`cmp`, `sort`, set and map keys, `Hash`; classification by reading the matches):

| Where                                                                                                                                | Kind       | Comparison                                                       | Class                               |
| ------------------------------------------------------------------------------------------------------------------------------------ | ---------- | ---------------------------------------------------------------- | ----------------------------------- |
| `Coordinator` (`swarm-coordination-registry`), `peers: BTreeMap<SocketAddr, Arc<PeerAnnouncement>>`                                  | production | filters by address (`get_address(..) != *peer_addr`)             | key is the address, no `Peer` order |
| `PeerList`, `Torrent` (`torrent-repository-benchmarking`, `publish = false`) derive `Ord` over `BTreeMap<PeerId, Arc<Peer>>`         | production | derived `Ord` inherited from `Peer`'s `Ord`; nothing orders them | unused                              |
| `Event` enums (`swarm-coordination-registry`, `http-core`, `udp-core`) derive `PartialEq, Eq` over `PeerAnnouncement`                | production | `==` only through `mockall::predicate::eq` in tests              | whole-peer, test-only use           |
| `assert_eq!`/`assert_ne!`/`contains` on `Peer`, `Arc<Peer>`, `Vec<Peer>` (about 18 files, e.g. `announce_handler.rs`, `registry.rs`) | test       | `Eq` on the whole peer                                           | needs whole-peer equality           |
| `.cmp()`, `sort`, `BTreeSet<Peer>`, `HashSet<Peer>`, `Hash` on `Peer`                                                                | none       | none found outside `peer.rs` itself                              | unused                              |

Every `assert_eq!` on a peer sits after the first `#[cfg(test)]` line of its file, so all whole-peer
equality in `assert_eq!` is test code. No production code orders or hashes a `Peer`.

**Conclusion.** The maintainer's hypothesis holds: whole-peer equality is used by tests and by the
test-only `Event` equality, and the id-only `Ord` is used by nothing. Production does not need any
ordering of `Peer`. Option A (remove `Ord` and `PartialOrd` from `Peer`) therefore needs no change
to in-repo call sites except the derived `Ord` on `PeerList` and `Torrent`, which only inherited it.
Its cost is the breaking change for external users (none known, not verified). Where only the id is
meant, code already compares `peer_id` explicitly. The maintainer chose A (see Decision).

## Scope

### In Scope

- Investigate why the two comparisons differ and classify call sites (T1).
- Remove `Ord` and `PartialOrd` from `Peer` (option A, chosen by the maintainer), so no ordering can disagree with `==`.
- Remove the derived `Ord`/`PartialOrd` from `PeerList` and `Torrent` in `torrent-repository-benchmarking`, which only inherited it.
- Use explicit `peer_id` comparisons where only the id is meant.
- A maintained regression test in `packages/primitives`: a `compile_fail` doctest asserting that `Peer` is not `Ord`.
- Update the rustdoc of `Peer` to state that equality is over every field and that `Peer` has no ordering.

### Out of Scope

- The `packages/primitives` test-coverage work (EPIC #1347, separate session).
- Changing `PeerList`'s keying by `PeerId` or the "duplicate peers with different ids" review note (the `code-review:` comment above `PeerList` in `packages/torrent-repository-benchmarking/src/entry/peer_list.rs`).

## Architectural Decisions

- Related ADRs: none known.
- ADRs to create: None. The maintainer chose A (a breaking public API change); the versioning ADR above already covers its version consequence, so no new ADR is needed.

## Design and Ownership Review

Not applicable.

## Bug-Fix Process

Follows `.github/skills/dev/debugging/fix-bug/SKILL.md`:

1. **Analysis**: hypothesis — `impl Ord for Peer` compares `peer_id` only while derived `Eq`
   compares all fields. Done above.
2. **Real-artifact reproduction**: the library has no CLI or service surface for ordering, so the
   artifact is the test binary. A temporary unit test captured `a == b` and `a.cmp(&b)`.
   Outcome: **Reproduced** (`a == b: false`, `a.cmp(&b): Equal`). Code and output recorded in
   `manual-verification-evidence.md`.
3. **Regression-test boundary**: unit test in `packages/primitives/src/peer.rs` (see below).
4. **Red evidence**: add the test before the fix, run it against the current code, record the
   failure.
5. **Fix**: smallest change for the chosen option.
6. **Green and recheck**: rerun the regression test and `cargo test -p torrust-tracker-primitives`,
   then rerun the original reproduction test like-for-like on the final commit and record it.

## Regression Test Strategy

A `compile_fail` doctest on `Peer` (in `peer.rs`), deterministic and with no I/O: it asserts at
compile time that `Peer` implements neither ordering trait, using a generic function with a
`PartialOrd` bound (`fn assert_partial_ord<T: PartialOrd>() {}` called with `Peer`). `Ord` requires
`PartialOrd`, so this one bound guards both removals, including a `PartialOrd`-only implementation.
With the buggy code `Peer: PartialOrd` holds, so the doctest compiles and `compile_fail` reports a
failure: that is the red run, recorded before the fix. After the fix the doctest fails to compile
as required and passes. A runtime test is not
possible for A because there is no ordering left to call. The existing tests that compare whole
peers with `assert_eq!` keep guarding equality over every field. The workspace build
(`cargo test --workspace --no-run`) is the guard for the dropped derives in
`torrent-repository-benchmarking`.

The tests are written with the `write-unit-test` skill, in the smallest behavior-focused increments.
After each increment passes focused validation, a prose-first Arrange-Act-Assert design review is
recorded (the one causal initial-state difference is exposed, the fixture owns only incidental
mechanics, the production Act and the independently specified expected result stay visible), and
the work stops for maintainer review after the final increment, before final verification, commit
or PR. The independent Task Reviewer does the final pre-PR review of the completed issue.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task                                                                                                                                    | Notes / Expected Output                                                                                                                                                                                                                                                                                                |
| --- | ------ | --------------------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| T1  | DONE   | Investigate history and intent: why `Eq` compares the whole `Peer` and `Ord` only `peer_id`, and which call sites need which comparison | Findings recorded in this spec (section "Intent and call-site classification"); maintainer reviews them before choosing the fix                                                                                                                                                                                        |
| T2  | DONE   | Analyse defect and audit usages                                                                                                         | Section "Audit of usages"                                                                                                                                                                                                                                                                                              |
| T3  | DONE   | Reproduce with a temporary test                                                                                                         | `manual-verification-evidence.md` V1 (reproduced)                                                                                                                                                                                                                                                                      |
| T4  | DONE   | Add maintained regression tests in `peer.rs`                                                                                            | Red run recorded before the fix. Use the `write-unit-test` skill; one smallest behavior-focused increment at a time; after each passing increment, record a prose-first Arrange-Act-Assert design review in `manual-verification-evidence.md`; then stop for maintainer review before final verification, commit or PR |
| T5  | DONE   | Apply the maintainer-chosen fix and update `Peer` rustdoc                                                                               | Regression tests pass                                                                                                                                                                                                                                                                                                  |
| T6  | DONE   | Green run and like-for-like recheck on the final commit                                                                                 | `manual-verification-evidence.md` V2                                                                                                                                                                                                                                                                                   |

## Commit Points

| Task | Coherent change set                      | Commit policy                                                                                                                   |
| ---- | ---------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------- |
| T1   | Spec update with the T1 findings         | Commit after focused validation and required review.                                                                            |
| T4   | Regression tests, failing before the fix | Commit each reviewed test-design increment (design review recorded) after focused validation, and only after maintainer review. |
| T5   | `Ord` fix and rustdoc                    | Commit after focused validation and required review.                                                                            |
| T6   | Evidence update                          | Commit after focused validation and required review.                                                                            |

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/open/2798-peer-ord-inconsistent-with-eq/ISSUE.md`
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue created and issue number added to this spec
- [x] (Optional, recommended for complex issues) Spec-only PR merged into `develop` before implementation (PR 2800)
- [x] Implementation completed
- [x] Automatic verification completed (`linter all`, relevant tests, and any pre-push checks)
- [x] Manual verification scenarios executed and recorded in issue-local `manual-verification-evidence.md`
- [x] Acceptance criteria reviewed after implementation and updated with evidence
- [x] Evidence-based implementation completion review recorded: issue-local retrospective created for material discoveries, or progress log states why none was needed
- [ ] Reviewer validated acceptance criteria and updated checkboxes
- [ ] Independent reviewer reports recorded in issue-local `agent-review-reports.md` when reviewers received this folder-style specification
- [ ] Committer verified spec progress is up to date before commit
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-11-10 17:46 UTC - Claude Code (Claude Sonnet 5.5) - Review round 1 of PR 2811 (the `da2ce7` findings): restored the major-version note and corrected the publishability reason, brought the checkpoints, completion review and frontmatter up to date, fixed stale cross-references and an overstated claim, reworded two commit messages, and fixed the evidence file. No Task Reviewer pass ran before this PR was opened, so `agent-review-reports.md` does not exist; the PR review by `da2ce7` and Copilot is the independent review - `ISSUE.md`, `manual-verification-evidence.md`
- 2026-11-10 17:40 UTC - Claude Code (Claude Sonnet 5.5) - Review round 1 of PR 2811: changed the doctest bound from `Ord` to `PartialOrd` so it also guards a `PartialOrd`-only implementation - `ISSUE.md`, `peer.rs`, evidence
- 2026-11-10 16:49 UTC - Claude Code (Claude Sonnet 5.5) - Implemented option A: removed `Ord`/`PartialOrd` from `Peer` and the inherited derives from `PeerList` and `Torrent`; doctest green, workspace builds, V2 recheck recorded - `peer.rs`, `peer_list.rs`, `entry/mod.rs`, evidence
- 2026-11-10 16:38 UTC - Claude Code (Claude Sonnet 5.5) - Maintainer chose option A; updated the spec (decision, scope, regression strategy as a `compile_fail` doctest, acceptance criteria, risks) - `ISSUE.md`
- 2026-11-10 15:50 UTC - Claude Code (Claude Sonnet 5.5) - Completed T1: recorded the history of the `Ord` change, the call-site table and the conclusion that production needs no `Peer` ordering; waiting for the maintainer to choose the option - `ISSUE.md`
- 2026-11-10 13:34 UTC - Claude Code (Claude Sonnet 5.5) - Re-ran the V1 reproduction with a start and an end stamp for each of its three steps (review round 2 of PR 2800) - `manual-verification-evidence.md` V1
- 2026-11-10 13:09 UTC - Claude Code (Claude Sonnet 5.5) - Addressed review round 1 of PR 2800 (findings F1 to F10): corrected T1 history and the production swarm key, stated the hand-written variant's projection, restored template rows and the test-development loop, rebuilt the evidence file from its template - `ISSUE.md`, `manual-verification-evidence.md`
- 2026-11-10 12:34 UTC - Claude Code (Claude Sonnet 5.5) - Maintainer approved the spec; created GitHub issue 2798; moved spec to `docs/issues/open/` for a spec-only PR - `ISSUE.md`
- 2026-11-10 12:20 UTC - Claude Code (Claude Sonnet 5.5) - Added T1 (history and intent investigation) and the maintainer position, per maintainer request - `ISSUE.md`
- 2026-11-10 11:55 UTC - Claude Code (Claude Sonnet 5.5) - Drafted spec, audited usages, reproduced the bug - `manual-verification-evidence.md` V1

## Acceptance Criteria

- [x] AC1: `Peer` implements neither `Ord` nor `PartialOrd`, so no ordering can disagree with `==`.
- [x] AC2: A maintained unit test fails on current code and passes after the fix (red/green recorded).
- [x] AC3: The workspace builds (`cargo test --workspace --no-run`) with the `Ord` derives dropped from `PeerList` and `Torrent`.
- [x] AC4: `Peer` rustdoc states that equality is over every field and that there is no ordering.
- [x] `linter all` exits with code `0`
- [x] `cargo test -p torrust-tracker-primitives` passes
- [x] Manual verification scenarios are executed and documented in issue-local `manual-verification-evidence.md`
- [x] Acceptance criteria are re-reviewed after implementation and reflect actual behavior

## Verification Plan

### Automatic Checks

- `linter all`
- `cargo test -p torrust-tracker-primitives`
- `cargo test --workspace --no-run` (confirms no other package breaks)

### Manual Verification Scenarios

| ID  | Scenario                                     | Human-oriented command/steps                                                                    | Expected Result                              | Status | Evidence                                     |
| --- | -------------------------------------------- | ----------------------------------------------------------------------------------------------- | -------------------------------------------- | ------ | -------------------------------------------- |
| M1  | Reproduce: `eq` false but `cmp` Equal        | Temporary test: `cargo test -p torrust-tracker-primitives --lib throwaway_repro -- --nocapture` | `a == b: false`, `a.cmp(&b): Equal`          | DONE   | `manual-verification-evidence.md` section V1 |
| M2  | Final recheck, same test on the final commit | Same temporary test re-run on the fixed code                                                    | Test no longer compiles: `Peer` has no `cmp` | DONE   | `manual-verification-evidence.md` section V2 |

### Acceptance Verification

| AC ID | Status | Evidence |
| ----- | ------ | -------- |
| AC1   | DONE   | tests    |
| AC2   | DONE   | evidence |
| AC3   | DONE   | tests    |
| AC4   | DONE   | diff     |

## Risks and Trade-offs

- Option A is a breaking change to the publishable `torrust-tracker-primitives` (3.1.0): its next
  publish is a major version (ADR `20260629000000_adopt_independent_package_versioning.md`).
  Accepted by the maintainer because few external consumers of `Peer: Ord` are expected (not
  verified).
- Option C (not chosen) changes `cmp` results for peers that share a `peer_id` but differ elsewhere (previously
  `Equal`). Nothing in the workspace depends on it; external users who did are relying on the bug.
- Option B would silently change equality semantics for every `assert_eq!`/`==` on peers.

## Implementation Completion Review

- Retrospective: none needed. The one discovery (the id-only `Ord` came from the `AnnounceEvent` swap in `00af70f25`) is recorded in the T1 findings.
- If needed (material discoveries, invalidated assumptions, or design changes), create
  `implementation-retrospective.md` from `docs/templates/IMPLEMENTATION-RETROSPECTIVE.md` in this
  issue specification's directory.
- If no retrospective is needed, add a concise progress-log entry explaining why the work had no
  material discovery.
- When an independent reviewer receives this folder-style specification, it records its result in
  `agent-review-reports.md` using `docs/templates/AGENT-REVIEW-REPORTS.md`.

## References

- Origin: found while adding tests to `packages/primitives` (EPIC #1347), where it is finding F1 of that work. That test spec and findings register are not yet published in the repository; this issue is the record. What the finding said: `Ord` by `peer_id` disagrees with the derived `Eq`, confirmed with a throwaway test (see `manual-verification-evidence.md`).
- `packages/primitives/src/peer.rs` (`impl Ord for Peer`)
- `packages/torrent-repository-benchmarking/src/entry/peer_list.rs`
