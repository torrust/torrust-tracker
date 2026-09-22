# Manual Verification Evidence

## M1 - Review #2158 Reconciliation

**Executed:** 2026-09-22 06:23 UTC

### Commands and Observed Output

```sh
owned_lints='default_trait_access|doc_markdown|empty_enums|explicit_iter_loop|legacy_numeric_constants|match_same_arms|missing_errors_doc|missing_panics_doc|must_use_candidate|needless_pass_by_value|semicolon_if_nothing_returned|wildcard_imports'
rg -n "clippy::($owned_lints)" packages/udp-protocol/src || printf 'No #2261-owned Clippy controls found.\n'
rg -n 'clippy::cast_possible_truncation|reason = ' packages/udp-protocol/src/lib.rs
```

```text
No #2261-owned Clippy controls found.
9:    clippy::cast_possible_truncation,
10:    reason = "temporary: #2245 reviews numeric protocol wire conversion bounds"
```

Compared this output with A157-A168 in `clippy-allow-inventory.md`. Each entry
is marked `Removed` and identifies its #2261 outcome.

No tracker logs apply: M1 is a source and inventory reconciliation, not a
running-service scenario.

### Result

PASS. The inventory marks A157-A168 as removed and links each outcome to #2261.
No owned suppression remains in the final UDP protocol source. A156 remains
unambiguously owned by #2245; no numeric conversion scope was absorbed by #2261.

## M1 Revalidation - Current Nightly Toolchain

**Executed:** 2026-09-22 07:49 UTC

After `rustup update nightly` installed Rust 1.100.0 (2026-09-21), CI's failing
diagnostic was reproduced locally:

```sh
cargo +nightly clippy -p torrust-tracker-udp-protocol --all-targets --all-features -- -D warnings
```

The command reported 37 `clippy::empty_enums` diagnostics from `FromBytes` derive
expansion across the inhabited protocol wire structs in `common.rs`, `connect.rs`,
and `scrape.rs`. The crate-level A159 allowance was restored with a native reason.
The #2158 inventory now records A159 as retained; A157-A158 and A160-A168 remain
removed, and A156 remains owned by #2245.
