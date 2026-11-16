---
doc-type: coverage-evidence
issue: 2876
package: torrust-tracker-primitives
measured-commit: c8b1477ad
measured-utc: 2026-11-16
---

# Coverage Evidence - `torrust-tracker-primitives` (#2876)

## Baseline (T1)

Measured on 2026-11-16 at `develop` commit `c8b1477ad` (the merge of #2878), with
`cargo-llvm-cov 0.6.16` and stable Rust 1.99.1, from a clean workspace, with a separate
`CARGO_LLVM_COV_TARGET_DIR`:

```console
cargo llvm-cov clean --workspace
cargo llvm-cov -p torrust-tracker-primitives --all-features --lib --json --output-path <report>.json
cargo llvm-cov -p torrust-tracker-primitives --all-features --json --output-path <report>.json
```

Totals are filtered to `packages/primitives/src/`. The package has no `tests/` folder and no
examples, so the aggregate run (without `--lib`) ran the same 108 unit tests and gave the same
numbers.

| Scope | Tests | Lines | Regions | Functions |
| --- | ---: | ---: | ---: | ---: |
| Unit-only (`--lib`) | 108 unit | 1,075 / 1,143 (94.05%) | 1,694 / 1,778 (95.28%) | 210 / 223 (94.17%) |
| Aggregate | 108 unit | 1,075 / 1,143 (94.05%) | 1,694 / 1,778 (95.28%) | 210 / 223 (94.17%) |

The final measurement of #2824 was 1,078 / 1,146 lines; `develop` moved since then. As in #2824, every
uncovered line of the package is in `peer.rs`: 504 / 572 lines covered, 68 uncovered.

### Uncovered `peer.rs` lines and their owners

Line numbers are those of `develop` at `c8b1477ad`.

| Lines | Code | Owner |
| --- | --- | --- |
| 369-382 | `Id::new` | #2876 (T4 removes it) |
| 413 | `Display for Id`, the `None` branch | #2824 (recorded there as a dead branch); not F5 |
| 547-605 | `PeerBuilder` methods `with_peer_id` to `with_event_completed` | #2876 (T7) |
| 619-629 | `impl Default for Peer` | #2876 (T7) |
| 633-636 | `impl Default for Id` | #2876 (T7) |

`PeerBuilder::seeder`, `leecher`, `build` and `into` are already covered by #2824's tests.

## After T7 (fixture tests), 2026-11-16

Measured on branch `2876-peer-id-test-helper` with the T7 tests in the working tree (after the
commit with subject `test(primitives): [#2876] build the fixture's peer ids with qbittorrent_peer_id`),
same command and toolchain as the baseline (separate `CARGO_LLVM_COV_TARGET_DIR`), unit-only:

| Scope | Tests | Lines | Regions | Functions |
| --- | ---: | ---: | ---: | ---: |
| Unit-only (`--lib`) | 125 unit | 1,270 / 1,273 (99.76%) | 1,920 / 1,925 (99.74%) | 242 / 242 (100.00%) |

`peer.rs` is 699 / 702 lines (baseline 504 / 572). `Id::new` is gone (T4); the new lines are
`qbittorrent_peer_id`, its tests and the fixture tests. The 3 uncovered lines:

| Line | Code | Owner / decision |
| --- | --- | --- |
| 417 | `Display for Id`, the `None` branch | #2824 (dead branch); not F5 |
| 1087 | The failure message of the `qbittorrent_peer_id` format test | No test: test code that runs only when the assertion fails |
| 1100 | `TestResult::discard()` in the distinctness property | No test: test code that runs only when quickcheck draws two equal numbers |

No line owned by F5 is left uncovered in production or fixture code.

## Final Measurement (T9)

Pending.
