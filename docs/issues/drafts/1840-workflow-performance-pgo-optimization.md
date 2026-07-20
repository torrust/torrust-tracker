---
doc-type: issue
issue-type: task
status: draft
priority: p3
github-issue: null
spec-path: docs/issues/drafts/1840-workflow-performance-pgo-optimization.md
branch: "{issue-number}-1840-pgo-optimization"
related-pr: null
last-updated-utc: 2026-06-03 00:00
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - Cargo.toml
    - Containerfile
    - .github/workflows/container.yaml
---


# Issue #[To be assigned] - Apply Profile-Guided Optimization (PGO) to the tracker release binary

## Goal

Apply Profile-Guided Optimization (PGO) to the tracker release binary to improve runtime performance of the deployed tracker, and define a sustainable workflow for collecting, storing, and refreshing PGO profiles in CI.

## Background

PGO is a compiler optimization technique that feeds real runtime statistics — branch frequencies, hot paths, inlining candidates — back into the compiler during a second build pass. LLVM (and therefore `rustc`) supports both instrumentation PGO and sampling PGO.

The optimization roadmap for native binaries is generally:

1. `opt-level = 3` (already in `[profile.release]`)
2. LTO — enables cross-crate inlining and dead code removal (already `lto = "fat"` in `[profile.release]`)
3. PGO — feeds runtime profiles to guide the above optimizations further

Published benchmarks show PGO improving real-world Rust applications by 10–30% or more on typical workloads. Because the tracker is a high-throughput network service where hot paths (announce/scrape handling, peer map operations) are well-defined and stable, it is a good candidate.

A talk at a Rust conference (June 2026) highlighted:

- Instrumentation PGO achieves the best optimization quality but requires compiling twice (once instrumented, once optimized), which adds CI time.
- Sampling PGO (e.g. via Linux `perf`) has near-zero runtime overhead (~2%) and avoids the double-compile cost but has limited tooling support and hardware requirements (BTS/BRS feature).
- `cargo-pgo` is the recommended Rust tooling for instrumentation PGO workflows.
- PGO profiles can become stale as code changes; they should be stored in version control and regenerated periodically.
- Combining LTO and PGO was previously broken in Rust but is fixed in current stable/nightly.

## Scope

### In Scope

- Evaluate instrumentation PGO for the tracker release binary using `cargo-pgo`.
- Define a representative training workload (announce/scrape traffic against a running tracker instance).
- Measure the impact on tracker binary throughput and latency using the existing benchmark suite.
- Define a CI workflow for collecting PGO profiles and using them in the release build.
- Document the PGO profile refresh policy (when and how often to regenerate).
- Store the PGO profile in version control alongside the build artifacts.

### Out of Scope

- Sampling PGO (defer until tooling support matures and hardware prerequisites are confirmed in CI runners).
- Advanced LLVM BOLT post-link optimization (defer as a follow-up).
- Applying PGO to debug or test builds.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task                                                                                        | Notes / Expected Output                                                                           |
| --- | ------ | ------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------- |
| T1  | TODO   | Install and configure `cargo-pgo` in the development environment                            | `cargo pgo` command available; verify `rustc` supports instrumentation PGO on current MSRV (1.88) |
| T2  | TODO   | Define a representative training workload script                                            | Script that sends realistic announce/scrape traffic to a running instrumented tracker             |
| T3  | TODO   | Run instrumented build, collect PGO profile, run optimized build                            | PGO-optimized release binary produced; profile stored under a well-known path                     |
| T4  | TODO   | Benchmark PGO-optimized binary against baseline (no PGO) using the existing benchmark suite | Measured throughput/latency delta; regression risk assessed                                       |
| T5  | TODO   | If T4 shows meaningful improvement: commit PGO profile and update `Containerfile` to use it | `Containerfile` release build uses stored PGO profile; double-compile cost documented             |
| T6  | TODO   | Document PGO profile refresh policy and add it to the release process                       | `docs/release_process.md` or a dedicated section documents when to regenerate the profile         |
| T7  | TODO   | Run pre-commit checks                                                                       | `./contrib/dev-tools/git/hooks/pre-commit.sh` exits with code 0                                   |

## Progress Tracking

### Workflow Checkpoints

- [ ] Spec drafted in `docs/issues/drafts/`
- [ ] Spec reviewed and approved by user/maintainer
- [ ] GitHub issue created and issue number added to this spec
- [ ] Implementation completed
- [ ] Automatic verification completed (`linter all`, relevant tests, and any pre-push checks)
- [ ] Manual verification scenarios executed and recorded (status + evidence)
- [ ] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] Reviewer validated acceptance criteria and updated checkboxes
- [ ] Committer verified spec progress is up to date before commit
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-06-03 00:00 UTC - GitHub Copilot - Spec drafted based on PGO talk at Rust conference (June 2026) and discussion about LTO settings in `Cargo.toml`

## Acceptance Criteria

- [ ] AC1: A PGO-optimized release binary is produced by the `Containerfile` release stage using a stored profile
- [ ] AC2: Benchmarks show a measurable throughput or latency improvement over the non-PGO baseline, or a documented conclusion that PGO does not benefit this workload at this time
- [ ] AC3: The PGO profile is stored in version control with a documented refresh policy
- [ ] AC4: The additional CI cost (double-compile) is measured and documented
- [ ] AC5: `linter all` exits with code 0
- [ ] AC6: Manual verification scenarios are executed and documented (status + evidence)
- [ ] AC7: Acceptance criteria are re-reviewed after implementation and reflect actual behavior

## Verification Plan

### Automatic Checks

- `linter all`
- `cargo test --tests --workspace --all-features`
- `./contrib/dev-tools/git/hooks/pre-commit.sh`

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID  | Scenario                                                    | Command/Steps                                                                          | Expected Result                                            | Status | Evidence |
| --- | ----------------------------------------------------------- | -------------------------------------------------------------------------------------- | ---------------------------------------------------------- | ------ | -------- |
| M1  | PGO-optimized binary benchmarked against baseline           | Run benchmark suite against PGO binary and baseline; compare throughput and latency    | PGO binary meets or exceeds baseline performance           | TODO   |          |
| M2  | Container release build uses PGO profile without errors     | `docker build --target release --tag torrust-tracker:release --file Containerfile .`   | Build completes; no PGO-related errors                     | TODO   |          |
| M3  | Stored PGO profile is used reproducibly across fresh builds | Clean build using committed PGO profile; compare binary performance to first PGO build | Performance is stable across builds using the same profile | TODO   |          |

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| ----- | ---------------------- | -------- |
| AC1   | TODO                   |          |
| AC2   | TODO                   |          |
| AC3   | TODO                   |          |
| AC4   | TODO                   |          |
| AC5   | TODO                   |          |
| AC6   | TODO                   |          |
| AC7   | TODO                   |          |

## Risks and Trade-offs

- Instrumentation PGO requires compiling twice, adding significant CI time (measured in T4/T5). This is a direct trade-off against EPIC #1840's goal of reducing CI wall-clock time; the benefit must be weighed against the cost before enabling PGO in the main container build.
- PGO profiles become stale as the codebase evolves. Stale profiles can slightly pessimize newly added code paths. Mitigation: define and follow a refresh policy (T6).
- The training workload must represent production traffic patterns. A poor training workload can cause PGO to optimize the wrong paths. Mitigation: design the training script against realistic announce/scrape ratios.
- If benchmarks (T4) show no meaningful improvement, PGO should not be enabled — the CI cost would not be justified. The spec treats this as a valid outcome.

## References

- [cargo-pgo](https://github.com/Kobzol/cargo-pgo) — Rust tooling for PGO workflows by Jakub Beránek
- [rustc PGO documentation](https://doc.rust-lang.org/rustc/profile-guided-optimization.html)
- [LLVM PGO documentation](https://llvm.org/docs/HowToBuildWithPGO.html)
- [awesome-pgo](https://github.com/zamazan4ik/awesome-pgo) — community PGO benchmarks and resources
- Talk: "Profile-Guided Optimization for Rust applications" (Rust conference, June 2026)
- Related: EPIC #1840 — adding PGO to the container build has a CI time cost that must be weighed against this EPIC's performance goals
