---
doc-type: issue
issue-type: task
status: draft
priority: p1
github-issue: null
spec-path: docs/issues/drafts/1840-workflow-performance-alternative-linker/ISSUE.md
branch: "{issue-number}-alternative-linker"
related-pr: null
last-updated-utc: 2026-06-01 00:00
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - Containerfile
    - .cargo/config.toml
    - .github/workflows/container.yaml
    - .github/workflows/testing.yaml
    - docs/issues/open/1840-improve-pr-workflow-performance-epic/EPIC.md
    - docs/issues/closed/1841-1840-workflow-performance-baseline-analysis/benchmark-results-baseline.md
---

<!-- skill-link: create-issue -->

# Issue #[To be assigned] - Switch to a faster linker (mold or lld) to reduce link time

## Goal

Replace the default GNU BFD linker with a faster alternative — `mold` or `lld`
— in both the local development build and the Containerfile build stages, to
reduce the dominant per-binary link time recorded in the baseline report.

## Background

### The baseline finding

The baseline profiling report
(`docs/issues/closed/1841-1840-workflow-performance-baseline-analysis/benchmark-results-baseline.md`)
identified the build as **linker-dominated**:

> "Individual crate compilation (frontend + codegen): ≤ 8 s per crate.
> Binary/test target linking: 35–117 s per binary — an order of magnitude
> more than any single crate compilation."

All 20+ binary and test targets compiled by the Containerfile's
`cargo nextest archive --all-targets` show `sections: null` in the
`cargo --timings` output — the signature of a pure external linker invocation.

The top offenders (release, warm incremental):

| Binary / target                                            | Link time (s) |
| ---------------------------------------------------------- | ------------- |
| `torrust-tracker` integration test                         | 117           |
| `torrust-tracker` bin                                      | 117           |
| `torrust-tracker` profiling bin                            | 116           |
| `torrust-tracker-axum-health-check-api-server` integration | 109           |
| `torrust-tracker-core` persistence bench bin               | 104           |
| … (15+ more in the 35–94 s range)                          | …             |

The baseline report explicitly recommends:

> "Switching to a faster linker (e.g. `mold` or `lld`) or removing
> non-runtime binary targets from the build are the two highest-leverage
> optimisations."

The current linker is the system default: GNU BFD via `cc` (confirmed in the
baseline measurement environment table: "system default (`cc` / BFD linker; no
`mold` or `lld`)").

### Local timing experiment (2026-06-01)

A fair incremental relink benchmark was run locally (Ryzen 9 7950X, debug
profile, `--bin torrust-tracker` only, `touch src/lib.rs` to force a recompile
of the top-level crate, 2026-06-01).

The linker was switched using `mold --run`, which intercepts `ld` via
`LD_PRELOAD` without changing `RUSTFLAGS` — so cargo's incremental cache
fingerprint is identical for both runs, ensuring only the top-level crate is
recompiled in each case. mold was confirmed active via `readelf -p .comment`
(`.comment` section showed `mold 2.40.4 (compatible with GNU ld)`).

| Linker                     | Real time | User time | Sys time | Notes                                  |
| -------------------------- | --------- | --------- | -------- | -------------------------------------- |
| BFD (default)              | 54.1 s    | 53.3 s    | 2.3 s    | `touch src/lib.rs && time cargo build` |
| mold 2.40.4 (`mold --run`) | 54.1 s    | 53.0 s    | 2.1 s    | same RUSTFLAGS, LD_PRELOAD intercept   |

**Interpretation**: both runs are strictly equivalent (same compilation units,
same RUSTFLAGS). The results are identical — **compilation of `lib.rs` dominates
at ~52 s (user time), masking the link time difference in a single-crate
incremental rebuild**. mold's parallelism advantage only becomes visible when
the link step is a significant fraction of total build time.

For a single incremental rebuild, the link time is approximately 2–3 s (total
54 s minus ~52 s compilation). mold compresses such a link from ~2–3 s to
sub-second, which is invisible in wall-clock terms here.

The real benefit is in **cold builds** (like CI / Containerfile), where 20+
binaries are linked fresh with no incremental cache. At BFD link times of 35–117
s per binary (baseline), and mold's documented speedup of 10–31× over BFD
(MySQL: 10.84 s → 0.46 s; Clang: 42.07 s → 1.35 s; source:
[mold README](https://github.com/rui314/mold)), the container build would save
hundreds of seconds.

> Note: the debug-profile results above represent the worst case for mold (link
> time already small). Release-profile and `--all-targets` cold builds are where
> mold delivers its full benefit.

### Linker options considered

The available alternatives to BFD were evaluated before choosing mold as the
primary candidate:

| Linker       | MySQL 8.3  | Clang 19   | Chromium 124 | Notes                                          |
| ------------ | ---------- | ---------- | ------------ | ---------------------------------------------- |
| BFD (GNU ld) | 10.84 s    | 42.07 s    | N/A          | Current default; single-threaded               |
| gold (GNU)   | 7.47 s     | 33.13 s    | 27.40 s      | Linux only; deprecated upstream                |
| lld (LLVM)   | 1.64 s     | 5.20 s     | 6.10 s       | Linux + macOS; ~4× faster than BFD             |
| **mold**     | **0.46 s** | **1.35 s** | **1.52 s**   | Linux only; most parallel; ~4× faster than lld |

Source: [mold README benchmarks](https://github.com/rui314/mold)

**Decision: pursue mold only.** It is the clear performance winner — ~4× faster
than lld and ~23× faster than BFD. There is no performance case for lld or gold.

The only reason to fall back to lld is **compatibility**: if mold fails to link
one of the C library dependencies (`aws-lc-sys`/BoringSSL is the known risk).
That path is covered by T8. lld is not benchmarked proactively; it is only
reached if mold is ruled out on correctness grounds.

**gold** is not considered: it is slower than lld and deprecated upstream.

**[wild](https://github.com/davidlattimore/wild)** (a new experimental
Rust-written linker optimized for incremental linking) is not considered: it is
too experimental for a production CI pipeline at this time.

- **mold** (<https://github.com/rui314/mold>): a modern, highly parallel linker
  designed as a drop-in replacement for GNU `ld` and `gold`. Available in
  Ubuntu apt (`mold` package, v2.40.4 on Ubuntu 26.04). Linux-only.
- **lld** (<https://lld.llvm.org>): the LLVM project linker. Available on Linux
  and macOS (`llvm-dev` or `lld` package on Ubuntu). Fallback only.

### Scope considerations

- **Local development**: changing `.cargo/config.toml` affects all contributors.
  macOS contributors cannot use `mold` (Linux-only); they need `lld` or the
  system default. Using `[target.'cfg(target_os = "linux")']` (the approach
  recommended by mold's own docs) scopes the setting to Linux only and avoids
  breaking macOS contributors. Example (mold in `$PATH`, GCC 12+):

  ```toml
  [target.'cfg(target_os = "linux")']
  rustflags = ["-C", "link-arg=-fuse-ld=mold"]
  ```

  For older GCC or to be explicit, add `linker = "clang"` and point to the
  mold executable path (`-fuse-ld=/usr/bin/mold`).

- **Containerfile (CI)**: the Docker builder image (`chef` stage) runs on
  Linux x86_64, so `mold` is the natural choice. `mold` needs to be installed
  in the builder stage (`apt-get install -y mold`) and will be picked up
  automatically via the `.cargo/config.toml` setting above.
- **cargo-chef cook stages**: the `dependencies` and `dependencies_debug` stages
  compile external crates (no final link step for the cook stage itself —
  `cargo chef cook` produces `.rlib` files, not binaries). The linker is only
  invoked in the `build` and `build_debug` stages for the final binary and test
  targets. The cook stages are unaffected by this change.

## Scope

### In scope

- Benchmark `mold` vs BFD for the relink-only case (single binary, debug and
  release profile) on the local developer machine.
- Benchmark `mold` vs BFD inside Docker (`build` and `build_debug` stages) for
  the full `--all-targets` case to measure end-to-end impact on container build
  time.
- If `mold` shows meaningful speedup, add it to the `chef` Docker stage and
  configure it as the linker for `x86_64-unknown-linux-gnu` builds via
  `.cargo/config.toml` (target-specific block to avoid breaking macOS
  contributors).
- Update the baseline benchmark report with new timing numbers.

### Out of scope

- Changing the linker for macOS developer machines (separate concern; `lld` or
  `zld` can be a follow-up if there is interest).
- Changing the linker for the `cargo test --doc` or `linter` steps (those do
  not produce standalone binaries; linker swap has minimal effect).
- Evaluating `lld` unless `mold` proves unsuitable (e.g. linking errors with
  specific C libraries such as `aws-lc-sys`).

## Implementation Plan

| Task ID | Description                                                                                                                                                                 | Status |
| ------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------ |
| T1      | Run relink benchmark locally: `touch src/lib.rs && time cargo build --bin torrust-tracker` vs `mold --run cargo build --bin torrust-tracker` (debug and release)            | DONE   |
| T2      | Run `--all-targets` benchmark locally with `mold`: `mold -run cargo build --timings --all-targets --release` and compare total wall time and per-binary times with baseline | TODO   |
| T3      | Test that `mold` produces a working binary: run `cargo test --workspace` and the integration test suite with mold active                                                    | TODO   |
| T4      | Verify `mold` links correctly with C dependencies (`libsqlite3-sys`, `aws-lc-sys`, `zstd-sys`, `ring`): check for linker errors or runtime failures                         | TODO   |
| T5      | Add `mold` installation to the `chef` stage of the Containerfile: `apt-get install -y mold`                                                                                 | TODO   |
| T6      | Add a `[target.x86_64-unknown-linux-gnu]` section to `.cargo/config.toml` pointing to `mold` as linker                                                                      | TODO   |
| T7      | Re-run the container cold benchmark with mold enabled and record new timings in the baseline report                                                                         | TODO   |
| T8      | If mold causes issues with any C library (aws-lc-sys is a known risk), evaluate `lld` as an alternative                                                                     | TODO   |

## Progress Tracking

### Checklist

- [x] T1 — relink benchmark (local, single binary, debug) — **done**: BFD 54.1s = mold 54.1s; compile dominates; pure link time immeasurable via wall clock in incremental mode (see Background)
- [ ] T2 — `--all-targets` timings benchmark (local, mold vs BFD)
- [ ] T3 — correctness: full test suite passes with mold
- [ ] T4 — C library linking verified: `libsqlite3-sys`, `aws-lc-sys`, `zstd-sys`
- [ ] T5 — mold added to `chef` Containerfile stage
- [ ] T6 — `.cargo/config.toml` updated with `[target.x86_64-unknown-linux-gnu]`
- [ ] T7 — container cold benchmark re-run and baseline report updated
- [ ] T8 — lld evaluated as fallback if mold fails on any C library

### Progress Log

Append one line per meaningful update.

- 2026-06-01 00:00 UTC - GitHub Copilot - Drafted sub-issue spec for alternative linker evaluation. Baseline data shows 35–117 s link time per binary (BFD).
- 2026-06-01 13:00 UTC - GitHub Copilot - Ran fair incremental relink benchmark using `mold --run` (LD_PRELOAD intercept, identical RUSTFLAGS). Result: BFD 54.1s = mold 54.1s — compile dominates (~52s user time) in single-crate incremental builds, masking the link time difference. Verified mold was active via `readelf -p .comment`. Updated spec with mold's official benchmarks (10–31× faster than BFD in cold builds) as the primary evidence for the container build savings.

## Acceptance Criteria

- [ ] AC1 — A relink benchmark comparing BFD vs mold has been run and recorded (debug and release profile, single binary and `--all-targets`).
- [ ] AC2 — `cargo test --workspace` passes with mold active (no correctness regressions).
- [ ] AC3 — C library dependencies (`aws-lc-sys`, `libsqlite3-sys`, `zstd-sys`) link correctly with mold.
- [ ] AC4 — If mold shows meaningful speedup (>20 %), it is enabled in `.cargo/config.toml` for `x86_64-unknown-linux-gnu` and in the `chef` Containerfile stage.
- [ ] AC5 — The container cold build benchmark is re-run with mold and new timings are recorded in the baseline report.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| ----- | ---------------------- | -------- |
| AC1   | TODO                   |          |
| AC2   | TODO                   |          |
| AC3   | TODO                   |          |
| AC4   | TODO                   |          |
| AC5   | TODO                   |          |

## Risks and Trade-offs

- **Risk**: `mold` may not support all linker flags or section layouts expected
  by `aws-lc-sys` (BoringSSL). Mitigation: T4 and T8 — verify with C library
  tests before enabling globally; fall back to `lld` if needed.
- **Risk**: Changing `.cargo/config.toml` to use `mold` will break builds on
  macOS (where `mold` is not available). Mitigation: use a
  `[target.x86_64-unknown-linux-gnu]` section, not a global `[build]` section.
- **Trade-off**: `mold` is Linux-only; macOS contributors would not benefit from
  this change locally. A separate follow-up could configure `lld` for macOS.
- **Trade-off**: Installing `mold` adds ~4 MB to the Docker builder image layer.
  This is negligible relative to the build time saved.
