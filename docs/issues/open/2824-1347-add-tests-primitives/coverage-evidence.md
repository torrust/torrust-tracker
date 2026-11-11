---
doc-type: coverage-evidence
issue: 2824
package: torrust-tracker-primitives
measured-commit: eaae0e487
measured-utc: 2026-11-10
---

# Coverage Evidence - `torrust-tracker-primitives`

Baseline for the sub-issue spec of #2824. Measured on 2026-11-10 at `develop` commit `eaae0e487`
(`Merge torrust/torrust-tracker#2793: feat: [#2751] deprecate the legacy shutdown API`), with
`cargo-llvm-cov 0.6.16` and stable Rust 1.99.1, from a clean workspace:

```console
cargo llvm-cov clean --workspace
cargo llvm-cov -p torrust-tracker-primitives --all-features --lib --json --output-path <report>.json
```

The package has no `tests/` directory and no examples, so the aggregate scope equals the unit-only
scope by construction; only one report exists. Totals are filtered to
`packages/primitives/src/` and include the in-file `#[cfg(test)]` modules and the
`peer::fixture` test support compiled into the library.

| Scope | Tests | Lines | Regions | Functions |
| --- | ---: | ---: | ---: | ---: |
| Unit-only (`--lib`) = aggregate | 47 unit | 453 / 748 (60.56%) | 712 / 1,048 (67.94%) | 91 / 159 (57.23%) |

## Module Inventory

Every `.rs` file under `packages/primitives/src/` (16 files), once. "Initial hypothesis" is a
starting point for the per-file plan, not a decision. Property candidate: a universal invariant or
arithmetic/encoding logic with round-trip or bounds properties.

| # | File | Unit-only lines | Tests | Uncovered lines (groups) | Property candidate | Initial hypothesis |
| ---: | --- | ---: | ---: | --- | --- | --- |
| 1 | `peer.rs` | 115 / 325 (35.4%) | 8 | `PeerRole` impls (48-77); `Peer` serializers and `ReadInfo` impls (164-318); `Id` conversions/`Display`/`Serialize` (348-495); `fixture` builders (547-636) | Yes: `Id` byte round-trip and hex length; seeder/leecher classification table | Test candidate (largest gap, domain core). Fixture = test support, to be classified in the S2 `peer.rs` record; any move behind a `testing` feature is owned by the F5 follow-up (EPIC #2482 is closed and did not reorganize it). |
| 2 | `pagination.rs` | 0 / 21 | 0 | `Pagination` constructor, defaults, `new_with_options` (22-52) | Weak (Option precedence, 2 cases) | Test candidate (small) |
| 3 | `policy.rs` | 0 / 16 | 0 | `TrackerPolicy` defaults, serde defaults (49-69) | No | Test candidate (documented defaults) |
| 4 | `announce.rs` | 0 / 16 | 0 | `AnnouncePolicy` defaults and serde defaults, `AnnounceData` default (64-84) | No | Test candidate |
| 5 | `mode.rs` | 0 / 8 | 0 | `PrivateMode` default, serde default (27-37) | No | Test candidate (small) |
| 6 | `swarm_metadata.rs` | 3 / 18 (16.7%) | 0 | the three accessors (41-53) and `AggregateActiveSwarmMetadata::add_assign` (75-80); `zeroed` (36-38) is the covered part, reached through the `scrape.rs` tests | Weak (`add_assign` is a field-wise sum; #2823 deletes it) | Test candidate (small); `add_assign` is not tested |
| 7 | `announce_interval.rs` | 79 / 88 (89.8%) | 10 | `TryFrom<u32>`, `From<_> for u32`, `Display` (70-72, 83-85, 101-103) | Yes: accepts iff `<= MAX`; serde round-trip | Test candidate (3 trivial gaps); property decision in plan |
| 8 | `configuration_instance_id.rs` | 42 / 42 | 5 | none | No | No change (clean) |
| 9 | `driver.rs` | 59 / 59 | 13 | none | No | No change; one-test-per-variant style reviewed in plan |
| 10 | `readiness.rs` | 52 / 52 | 5 | none | No | No change (clean) |
| 11 | `runtime_service_metadata.rs` | 32 / 32 | 2 | none | No | No change (clean) |
| 12 | `scrape.rs` | 28 / 28 | 1 | none | No | No change (only `zeroed` and `add_file_with_zeroed_metadata` tested; `empty` and `add_file` covered indirectly): review in plan |
| 13 | `service_role.rs` | 40 / 40 | 3 | none | No | No change (clean) |
| 14 | `number_of_bytes.rs` | 3 / 3 | 0 | none | No | No change (trivial newtype) |
| 15 | `peer_id.rs` | no executable entries | 0 | deprecated re-export | No | Wiring only; removal tracked by #1669 |
| 16 | `lib.rs` | no executable entries | 0 | module declarations, deprecated re-exports, type aliases | No | Wiring only |

Uncovered line numbers are from the unit-only run; the per-line classification (test, owner or
unselected) is recorded in each file's plan.

## Test-Technique Assessment (initial)

| Technique | Assessment |
| --- | --- |
| Unit tests | Selected; primary objective, in-file `#[cfg(test)]` modules. |
| Package integration tests | Not applicable: the package is pure types with no I/O and no `tests/` directory. |
| Runnable examples / doctests | `peer.rs` has two `no_run` examples (module docs, line 5; `Peer` docs, line 94), which are not evidence for behavior, and, since the F1 fix, a `compile_fail` doctest (line 117) that guards `Peer` against implementing `Ord`; that doctest is behavior evidence and is not changed. |
| Property-based testing | Assessed per seam (inventory column). Proposed: `AnnounceIntervalInSecs` bounds and `peer::Id` byte round-trip, with `quickcheck` as in #1348. Needs the maintainer's decision. |
| Mutation testing | Bounded whole-package run in the final slice (about 160 functions). |
| Fuzzing, benchmarks | Not selected / out of scope. |
