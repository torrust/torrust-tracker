---
schema-version: 1
doc-type: issue
issue-type: bug
status: planned
priority: p3
epic: null
github-issue: 2798
spec-path: docs/issues/open/2798-peer-ord-inconsistent-with-eq/ISSUE.md
branch: "2798-peer-ord-inconsistent-with-eq-spec"
related-pr: null
last-updated-utc: "2026-11-10 13:09"
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

| Option | Change                                                                       | Public API impact                                                                                                                                                                                                                                                                                                          | Verdict                                                                                                                                                             |
| ------ | ---------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| A      | Remove `Ord`/`PartialOrd` from `Peer`                                        | Breaking: external sorters stop compiling; `PeerList`/`Torrent` must also drop their derived `Ord`                                                                                                                                                                                                                         | Correct but costs a major version for no in-repo gain                                                                                                               |
| B      | Make `Eq`/`Hash` compare `peer_id` only                                      | Breaking in behaviour (silent): `assert_eq!` on peers would ignore state changes; tests such as `should_be_comparable` change meaning                                                                                                                                                                                      | Rejected: equality over the full announce state is relied on by tests and is the safer semantics; a peer with different `left`/`event` is observably different data |
| C      | Make `Ord`/`PartialOrd` agree with `Eq`: compare all fields, `peer_id` first | Non-breaking for ordering of distinct `peer_id`s (same primary key and order); only ties are now broken by the remaining fields. The derive variant needs new `Ord` impls on `NumberOfBytes` and `AnnounceEvent` (additive, but it makes their order public); the hand-written variant needs none (see the recommendation) | **Recommended**                                                                                                                                                     |
| D      | Document only                                                                | None                                                                                                                                                                                                                                                                                                                       | Leaves the contract violation; acceptable only as a fallback                                                                                                        |

### Recommendation: C (tentative, pending T1)

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

**Maintainer decides** (after T1) between A/B/C/D and, for C, between the two variants.

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

## Scope

### In Scope

- Investigate why the two comparisons differ and classify call sites (T1).
- Make `Peer::cmp` equal exactly when `==` (per the option the maintainer picks).
- Use explicit `peer_id` comparisons where only the id is meant.
- A maintained regression test in `packages/primitives`.
- Update the rustdoc of `Peer` to state the ordering/equality contract.

### Out of Scope

- The `packages/primitives` test-coverage work (EPIC #1347, separate session).
- Changing `PeerList`'s keying by `PeerId` or the "duplicate peers with different ids" review note (the `code-review:` comment above `PeerList` in `packages/torrent-repository-benchmarking/src/entry/peer_list.rs`).
- Dropping the derived `Ord` from `PeerList`/`Torrent` (unrelated unless option A is chosen).

## Architectural Decisions

- Related ADRs: none known.
- ADRs to create: None known. Revisit if the maintainer picks A or B (public semantics change).

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

Unit test at the causal seam (`peer.rs`, existing `test` module), deterministic and with no I/O.
Inputs: a seeder `a` and a copy `b` with `left = 5`. Asserts, for option C: `a != b` implies
`a.cmp(&b) != Equal`; `a.cmp(&b)` and `b.cmp(&a)` are reverse of each other; and a peer compares
`Equal` to an identical copy. A second case checks that peers with different `peer_id`s keep the
order of their ids regardless of other fields (guards the "ordering of distinct ids unchanged"
claim). Red run is recorded before the fix; no infeasibility constraints are expected. For
option A the test becomes a compile-fail/removal check, to be redefined if chosen.

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
| T1  | TODO   | Investigate history and intent: why `Eq` compares the whole `Peer` and `Ord` only `peer_id`, and which call sites need which comparison | Findings recorded in this spec (section "Intent and call-site classification"); maintainer reviews them before choosing the fix                                                                                                                                                                                        |
| T2  | DONE   | Analyse defect and audit usages                                                                                                         | Section "Audit of usages"                                                                                                                                                                                                                                                                                              |
| T3  | DONE   | Reproduce with a temporary test                                                                                                         | `manual-verification-evidence.md` V1 (reproduced)                                                                                                                                                                                                                                                                      |
| T4  | TODO   | Add maintained regression tests in `peer.rs`                                                                                            | Red run recorded before the fix. Use the `write-unit-test` skill; one smallest behavior-focused increment at a time; after each passing increment, record a prose-first Arrange-Act-Assert design review in `manual-verification-evidence.md`; then stop for maintainer review before final verification, commit or PR |
| T5  | TODO   | Apply the maintainer-chosen fix and update `Peer` rustdoc                                                                               | Regression tests pass                                                                                                                                                                                                                                                                                                  |
| T6  | TODO   | Green run and like-for-like recheck on the final commit                                                                                 | `manual-verification-evidence.md` V2                                                                                                                                                                                                                                                                                   |

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
- [ ] (Optional, recommended for complex issues) Spec-only PR merged into `develop` before implementation (PR 2800)
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

- 2026-11-10 13:09 UTC - Claude Code (Claude Sonnet 5.5) - Addressed review round 1 of PR 2800 (findings F1 to F10): corrected T1 history and the production swarm key, stated the hand-written variant's projection, restored template rows and the test-development loop, rebuilt the evidence file from its template - `ISSUE.md`, `manual-verification-evidence.md`
- 2026-11-10 12:34 UTC - Claude Code (Claude Sonnet 5.5) - Maintainer approved the spec; created GitHub issue 2798; moved spec to `docs/issues/open/` for a spec-only PR - `ISSUE.md`
- 2026-11-10 12:20 UTC - Claude Code (Claude Sonnet 5.5) - Added T1 (history and intent investigation) and the maintainer position, per maintainer request - `ISSUE.md`
- 2026-11-10 11:55 UTC - Claude Code (Claude Sonnet 5.5) - Drafted spec, audited usages, reproduced the bug - `manual-verification-evidence.md` V1

## Acceptance Criteria

- [ ] AC1: For any two `Peer`s `a`, `b`: `a.cmp(&b) == Equal` if and only if `a == b`.
- [ ] AC2: A maintained unit test fails on current code and passes after the fix (red/green recorded).
- [ ] AC3: Peers with different `peer_id`s keep the ordering of their ids (option C).
- [ ] AC4: `Peer` rustdoc states the equality/ordering contract.
- [ ] `linter all` exits with code `0`
- [ ] `cargo test -p torrust-tracker-primitives` passes
- [ ] Manual verification scenarios are executed and documented in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria are re-reviewed after implementation and reflect actual behavior

## Verification Plan

### Automatic Checks

- `linter all`
- `cargo test -p torrust-tracker-primitives`
- `cargo test --workspace --no-run` (confirms no other package breaks, important for A and C)

### Manual Verification Scenarios

| ID  | Scenario                                     | Human-oriented command/steps                                                                    | Expected Result                       | Status | Evidence                                     |
| --- | -------------------------------------------- | ----------------------------------------------------------------------------------------------- | ------------------------------------- | ------ | -------------------------------------------- |
| M1  | Reproduce: `eq` false but `cmp` Equal        | Temporary test: `cargo test -p torrust-tracker-primitives --lib throwaway_repro -- --nocapture` | `a == b: false`, `a.cmp(&b): Equal`   | DONE   | `manual-verification-evidence.md` section V1 |
| M2  | Final recheck, same test on the final commit | Same temporary test re-run on the fixed code                                                    | `cmp` no longer `Equal` when `a != b` | TODO   | `manual-verification-evidence.md` section V2 |

### Acceptance Verification

| AC ID | Status | Evidence |
| ----- | ------ | -------- |
| AC1   | TODO   | tests    |
| AC2   | TODO   | evidence |
| AC3   | TODO   | tests    |
| AC4   | TODO   | diff     |

## Risks and Trade-offs

- Option C changes `cmp` results for peers that share a `peer_id` but differ elsewhere (previously
  `Equal`). Nothing in the workspace depends on it; external users who did are relying on the bug.
- Option B would silently change equality semantics for every `assert_eq!`/`==` on peers.
- Option A is a breaking change to a published crate (major version, per ADR
  `20260629000000_adopt_independent_package_versioning.md`).

## Implementation Completion Review

- Retrospective: `Not yet assessed`
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
