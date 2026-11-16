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

## Final Measurement (T9)

Pending.
