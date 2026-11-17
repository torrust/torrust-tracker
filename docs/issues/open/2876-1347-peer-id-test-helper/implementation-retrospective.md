---
semantic-links:
  skill-links:
    - write-markdown-docs
  related-artifacts:
    - docs/issues/open/2876-1347-peer-id-test-helper/ISSUE.md
    - docs/adrs/20261021190318_share_workspace_test_support_through_testing_features.md
---

# Implementation Retrospective — #2876 Peer Id Test Helper

## Purpose

Record evidence-based process improvements discovered while implementing #2876. This is a blameless
review of the implementation approach; it does not replace acceptance-criteria verification.

## Outcome

`peer::Id::new` is removed (a `compile_fail` doctest guards it). `peer::fixture`, with its new
`qbittorrent_peer_id(u32) -> Id` helper and the `Default` impls of `Peer` and `Id`, compiles only
with the new `testing` feature of `primitives`. Eight consumers enable it from `[dev-dependencies]`.
The fixture's ids are realistic, every fixture line is tested, and the test-support ADR is amended.
Evidence: `test-plans/qbittorrent-peer-id.md`, `manual-verification-evidence.md`,
`coverage-evidence.md`.

## What Went Well

1. The red stub that returned today's `Id::new` ids showed which helper tests guard new behavior (the
   format and the client name) and which do not (distinct ids, which the old function had too), so
   that one got its own mutation.
2. Changing the fixture ids before touching any assertion gave the list of dependent assertions by
   running the tests: it was empty, which no text search could have shown.

## What Changed During Implementation

- **The workspace build hides missing consumer entries.** `cargo test --workspace --no-run` passed
  without any consumer `[dev-dependencies]` entry, because the self dev-dependency of `primitives`
  turns `testing` on for every member of a workspace build. Building each of the 34 members on its
  own found the eight that need it. The ADR already notes this for per-package runs (O1a, V4); it
  mattered here because the consumer list had to be complete.
- **`cargo machete --with-metadata` flagged the self dev-dependency.** The ADR (O1c) says machete
  does not flag it. That held for the server packages, whose `tests/` and examples name the crate by
  its package name; `primitives` has no `tests/` folder and its `src/` uses `crate::`, so machete saw
  no use. The self dev-dependency is now in the package's machete `ignored` list, with the reason.
- **No assertion depended on the fixture's id bytes.** The spec (Q6, T6) expected some in the
  `axum-http-server` and `axum-rest-api-server` tests; the tests that read the ids compare them with
  values from the same fixture.

## Root Cause

The O1c statement about machete was verified on packages that all had integration tests or
examples. The spec's T6 expectation came from a text search for the old literals, which cannot tell
hand-written literals from bytes the fixture produces.

## Improvements for Future Work

1. When a `testing` feature gets a self dev-dependency, run `cargo machete --with-metadata` and,
   for a package without `tests/` or examples, expect to add the self dev-dependency to its machete
   `ignored` list. The ADR amendment of 2026-11-16 now says so (added after review finding F2 of
   PR #2888).
2. To find the consumers of a newly gated module, build each member on its own
   (`cargo test -p <member> --no-run`), not the workspace.

## Avoiding Overcorrection

No new check is justified: `cargo machete` already fails the commit, and the per-package build is a
one-off step of a gating change, not something CI needs to repeat.

## Evidence

- [Issue specification](ISSUE.md), T5 and T6
- [Test plan](test-plans/qbittorrent-peer-id.md), sections `testing` Feature and Realistic Fixture Ids
