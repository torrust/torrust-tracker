---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2931-nightly-uncommon-codepoints-zerocopy/ISSUE.md
last-updated-utc: 2026-11-18 07:54
---

# Manual Verification Evidence

## Environment and Prerequisites

- Date and time (UTC): 2026-11-18 07:51
- Artifact under test: `develop` at merge commit 8d932f312 (torrust/torrust-tracker#2921).
- Operating system / environment: Linux x86_64, rustup toolchains installed with
  `rustup toolchain install --profile minimal nightly-2026-11-17 nightly-2026-11-18`. Rustup's channel date is one day
  after the rustc commit date:
  - `nightly-2026-11-17` = `rustc 1.102.0-nightly (5df8c02af 2026-11-16)`
  - `nightly-2026-11-18` = `rustc 1.102.0-nightly (59cbdee2e 2026-11-17)` (the CI failure's toolchain)
- Prerequisites and setup performed: none beyond the toolchains. The repository's
  `.cargo/config.toml` rustflags (including `-D warnings`) apply. No `RUSTC_WRAPPER`/sccache.

## Verification Processes

### V1 - Red reproduction

- Goal: observe the CI error locally and confirm the toolchain boundary.
- Initial state: clean checkout of `develop` above.
- Status: `DONE`

#### Steps Performed

1. `cargo +nightly-2026-11-17 check -p torrust-tracker-udp-protocol`, finished 07:51:18Z.
2. `cargo +nightly-2026-11-18 check -p torrust-tracker-udp-protocol`, finished 07:51:18Z (the crate
   had been built before in that target dir, so only the failing crate was rechecked).

Full output: [`evidence/red-nightly-2026-11-17.txt`](evidence/red-nightly-2026-11-17.txt),
[`evidence/red-nightly-2026-11-18.txt`](evidence/red-nightly-2026-11-18.txt).

#### Observed Result

```text
# nightly-2026-11-17
    Finished `dev` profile [optimized + debuginfo] target(s) in 3.05s
exit: 0

# nightly-2026-11-18
error: identifier contains an uncommon character: 'ẕ'
  --> packages/udp-protocol/src/announce.rs:21:9
   |
21 |     pub connection_id: ConnectionId,
   |         ^^^^^^^^^^^^^
   |
   = note: this character is included in the Unicode general security profile
   = note: `-D uncommon-codepoints` implied by `-D warnings`
...
error: could not compile `torrust-tracker-udp-protocol` (lib) due to 17 previous errors
exit: 101
```

A one-line crate `pub enum ẕfield {}` built with plain `rustc` warns `uncommon character: 'ẕ'`
on `nightly-2026-11-18` and does not on `nightly-2026-11-17`, which isolates the cause to the
toolchain's Unicode data (rust-lang/rust#165289), not to this repository.

During analysis, an item-level `#[allow(uncommon_codepoints)]` on the `announce.rs` structs was
tried (then reverted) and the errors remained, consistent with rust-lang/rust#86824.

#### Conclusion

**Reproduced**: the same 17 errors as CI on `nightly-2026-11-18`; clean on `nightly-2026-11-17`.

### V2 - Green recheck

- Goal: confirm the crate-level allow removes the errors on the failing nightly, without breaking the
  workspace, stable or the MSRV.
- Initial state: branch `2931-nightly-uncommon-codepoints-zerocopy` with the allow added to
  `packages/udp-protocol/src/lib.rs` (commit `fix(udp-protocol): allow uncommon_codepoints for zerocopy derives`).
- Status: `DONE`

#### Steps Performed

Run one after another from the same checkout, same command as V1 for the first step. Each output file holds
the command, UTC start and end times, and the exit code.

1. `cargo +nightly-2026-11-18 check -p torrust-tracker-udp-protocol` ([output](evidence/green-nightly-2026-11-18-crate.txt))
2. `cargo +nightly-2026-11-18 check --workspace --all-targets` ([output](evidence/green-nightly-2026-11-18-workspace.txt))
3. `cargo +stable check -p torrust-tracker-udp-protocol` ([output](evidence/green-stable-crate.txt))
4. `cargo +1.88.0 check -p torrust-tracker-udp-protocol` ([output](evidence/green-msrv-1.88.0-crate.txt))

#### Observed Result

```text
nightly-2026-11-18 (-p torrust-tracker-udp-protocol): Finished ... exit: 0
nightly-2026-11-18 (--workspace --all-targets):       Finished ... exit: 0
stable (-p torrust-tracker-udp-protocol):             Finished ... exit: 0
1.88.0 (-p torrust-tracker-udp-protocol):             Finished ... exit: 0
```

#### Conclusion

The same command that gave 17 errors in V1 now succeeds on `nightly-2026-11-18`, and nothing else in the
workspace fails on that nightly. Stable and the MSRV still build. The CI nightly jobs of the PR are the
final check (AC5).

## Failures and Follow-up

None.
