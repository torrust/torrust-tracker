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
last-updated-utc: "2026-11-10 12:45"
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
the contract (`BTreeSet<Peer>`, `sort` then removing equal neighbors, `binary_search`, `min`/`max`, `BinaryHeap`) can
silently treat different peers as duplicates, or lose one of them.

### Impact today

Latent, but wider than direct use of `Peer`:

- No production code puts `Peer` in a `BTreeSet`, sorts it, or takes `min`/`max` of it. The swarm
  keys by `PeerId` in a `BTreeMap<PeerId, Arc<Peer>>`
  (`packages/torrent-repository-benchmarking/src/entry/peer_list.rs`).
- `PeerList` and `Torrent` (`packages/torrent-repository-benchmarking`, `publish = false`) **derive**
  `Ord`/`PartialOrd`, so they inherit the defect through `Arc<Peer>` values: two `PeerList`s with
  the same peer ids but different peer data compare `Equal` while being `!=`. Nothing in the
  workspace orders those types either (audit below).
- The existing test `should_be_comparable` only checks `==`/`!=`; nothing tests `cmp`.

## Audit of usages (workspace, develop at `eaae0e487`)

Searched all workspace packages, `src/` and `console/` for `Peer`/`PeerList`/`Torrent` used with
`BTreeSet`, `HashSet`, `BinaryHeap`, `sort*`, `min`/`max`, `cmp`/`partial_cmp`, and for `derive(..Ord..)`
on types that contain a `Peer`.

- Direct `Ord` use of `Peer`: none outside its own `impl` in `peer.rs`.
- Derived `Ord` that transitively uses `Peer::cmp`: `PeerList` and `Torrent`
  (`torrent-repository-benchmarking`), neither of which is ordered anywhere.
- `Eq`/`Hash` on `Peer` are consistent with each other (both derived over all fields).
- External consumers of the published crate cannot be audited; that is the breaking-change risk
  for options A and B.

## Options

| Option | Change                                                                       | Public API impact                                                                                                                                                                              | Verdict                                                                                                                                                             |
| ------ | ---------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| A      | Remove `Ord`/`PartialOrd` from `Peer`                                        | Breaking: external sorters stop compiling; `PeerList`/`Torrent` must also drop their derived `Ord`                                                                                             | Correct but costs a major version for no in-repo gain                                                                                                               |
| B      | Make `Eq`/`Hash` compare `peer_id` only                                      | Breaking in behaviour (silent): `assert_eq!` on peers would ignore state changes; tests such as `should_be_comparable` change meaning                                                          | Rejected: equality over the full announce state is relied on by tests and is the safer semantics; a peer with different `left`/`event` is observably different data |
| C      | Make `Ord`/`PartialOrd` agree with `Eq`: compare all fields, `peer_id` first | Non-breaking for ordering of distinct `peer_id`s (same primary key and order); only ties are now broken by the remaining fields; needs `Ord` on `NumberOfBytes` and `AnnounceEvent` (additive) | **Recommended**                                                                                                                                                     |
| D      | Document only                                                                | None                                                                                                                                                                                           | Leaves the contract violation; acceptable only as a fallback                                                                                                        |

### Recommendation: C (tentative, pending T1)

It restores the contract with the smallest blast radius: no removal of public API, no change to
equality or hashing, and the existing order of peers with different ids is unchanged because
`peer_id` stays the first compared field. Implementation detail to settle in review: derive
`PartialOrd, Ord` (requires adding `PartialOrd, Ord` derives to `NumberOfBytes` and
`AnnounceEvent`, which makes the order depend on field and variant order) versus a hand-written
`cmp` over a tuple of the seven fields (no new derives on other types, explicit order). The
spec leans to the hand-written tuple to avoid committing `AnnounceEvent`'s variant order as a
public ordering. **Maintainer decides** (after T1) between A/B/C/D and between the two C variants.

## Intent and call-site classification

Maintainer position (hypothesis to verify in T1, not yet a finding): whole-`Peer` equality is
probably used only by tests (`assert_eq!` on peers), and the `peer_id`-only ordering probably
exists for the few places that really meant "same peer id". Production should compare the whole
structure and say so explicitly (`a.peer_id == b.peer_id`) where only the id matters. `peer_id`
is chosen by each client in a decentralized way, so the tracker does not control it. Two peers
with the same `peer_id` but different other fields are either a bug or a (very unlikely)
collision, or deliberate spoofing, and an implicit id-only comparison would hide that. Comparing
only the id may be marginally cheaper, but that is not worth the risk of silent behavior.

T1 must produce, before the fix is chosen:

- **History**: when and why `Ord` by `peer_id` was added (it predates `00af70f25`, 2024-07-14, and
  was already in `packages/primitives/src/peer.rs` after the move in `9a43815d0`, 2024-02-13;
  trace it further back through `src/core` and the original PR/issue), and the same for the derived
  `Eq`. Record what the authors intended.
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
- Changing `PeerList`'s keying by `PeerId` or the "duplicate peers with different ids" review note.
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

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task                                                                                                                                    | Notes / Expected Output                                                                                                         |
| --- | ------ | --------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------- |
| T1  | TODO   | Investigate history and intent: why `Eq` compares the whole `Peer` and `Ord` only `peer_id`, and which call sites need which comparison | Findings recorded in this spec (section "Intent and call-site classification"); maintainer reviews them before choosing the fix |
| T2  | DONE   | Analyse defect and audit usages                                                                                                         | Section "Audit of usages"                                                                                                       |
| T3  | DONE   | Reproduce with a temporary test                                                                                                         | `manual-verification-evidence.md` V1 (reproduced)                                                                               |
| T4  | TODO   | Add maintained regression tests in `peer.rs`                                                                                            | Red run recorded before the fix                                                                                                 |
| T5  | TODO   | Apply the maintainer-chosen fix and update `Peer` rustdoc                                                                               | Regression tests pass                                                                                                           |
| T6  | TODO   | Green run and like-for-like recheck on the final commit                                                                                 | `manual-verification-evidence.md` V2                                                                                            |

## Commit Points

| Task | Coherent change set                      | Commit policy                                        |
| ---- | ---------------------------------------- | ---------------------------------------------------- |
| T4   | Regression tests, failing before the fix | Commit after focused validation and required review. |
| T5   | `Ord` fix and rustdoc                    | Commit after focused validation and required review. |
| T6   | Evidence update                          | Commit after focused validation and required review. |

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/open/2798-peer-ord-inconsistent-with-eq/ISSUE.md`
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue created and issue number added to this spec
- [ ] Implementation completed
- [ ] Automatic verification completed (`linter all`, relevant tests, and any pre-push checks)
- [ ] Manual verification scenarios executed and recorded in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] Implementation completion review recorded
- [ ] Reviewer validated acceptance criteria and updated checkboxes
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-11-10 12:45 UTC - Claude Code (Claude Sonnet 5.5) - Maintainer approved the spec; created GitHub issue 2798; moved spec to `docs/issues/open/` for a spec-only PR - `ISSUE.md`
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

## References

- Finding F1 of the `packages/primitives` test work (EPIC #1347).
- `packages/primitives/src/peer.rs` (`impl Ord for Peer`)
- `packages/torrent-repository-benchmarking/src/entry/peer_list.rs`
