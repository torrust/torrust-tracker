---
doc-type: issue
issue-type: task
status: planned
priority: p2
github-issue: 1875
spec-path: docs/issues/open/1875-review-lto-fat-in-dev-profile.md
branch: "1875-review-lto-fat-in-dev-profile"
related-pr: null
last-updated-utc: 2026-06-03 00:00
semantic-links:
  skill-links:
    - create-issue
    - create-adr
  related-artifacts:
    - Cargo.toml
    - docs/adrs/
    - docs/skills/semantic-skill-link-convention.md
---


# Issue #1875 - Review and fix `lto = "fat"` in `[profile.dev]`

## Goal

Determine whether `lto = "fat"` in `[profile.dev]` is still necessary, and remove or replace it with an appropriate setting that does not unnecessarily slow down development builds.

## Background

Commit `3c715fbb` (fix: [#898] docker build error: failed to load bitcode of module criterion) changed `lto = "thin"` to `lto = "fat"` in `[profile.dev]` as a workaround for an LLVM bitcode compatibility error that occurred when building benchmarks (criterion) inside a Docker container with Rust 1.79/1.81-nightly (mid-2024):

```text
error: failed to load bitcode of module "criterion-...": failed to load bitcode
```

The root cause was an LLVM cross-module bitcode version mismatch triggered by `lto = "thin"` when mixing crates compiled with different LLVM/rustc versions inside a container build. Switching to `"fat"` forced all bitcode into a single monolithic unit, eliminating the cross-module issue.

This was a legitimate workaround at the time but carries a significant cost: `lto = "fat"` in `[profile.dev]` applies full-program LTO to every incremental development build, substantially increasing compile times with no benefit for day-to-day development iteration.

The project now targets MSRV 1.88 (as of 2026-06). The LLVM version bundled with Rust 1.88 is well past the version where this bug was observed, and the `Containerfile` now builds with the stable toolchain. The original triggering conditions may no longer exist.

## Scope

### In Scope

- Investigate whether removing `lto = "fat"` from `[profile.dev]` still causes the Docker build to fail with current Rust/LLVM versions
- If the bug is gone: remove `lto = "fat"` from `[profile.dev]` (restore `lto = "thin"` or remove the key to use the Cargo default of `false`)
- If the bug persists: document exactly why, pin the minimum fix to the narrowest possible scope (e.g. only the benchmark crate, only the release profile, or via a per-crate override), and open a follow-up tracking upstream resolution
- Keep `lto = "fat"` in `[profile.release]` — it is appropriate there for production binary optimization

### Out of Scope

- Changing `[profile.release]` LTO settings
- Restructuring the Containerfile beyond what is required to verify the fix

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task                                                                                | Notes / Expected Output                                                                                                                                                                                                                                                                                                                     |
| --- | ------ | ----------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| T1  | TODO   | Reproduce the original failure (optional, low priority)                             | Confirm what Rust/LLVM version combination triggers the bitcode error, if reproducible at all                                                                                                                                                                                                                                               |
| T2  | TODO   | Remove `lto = "fat"` from `[profile.dev]` (or restore `lto = "thin"`)               | `Cargo.toml` `[profile.dev]` no longer carries fat LTO                                                                                                                                                                                                                                                                                      |
| T2a | TODO   | If the final LTO choice is non-obvious, create an ADR and link it from `Cargo.toml` | ADR created in `docs/adrs/` (see `.github/skills/dev/planning/create-adr/SKILL.md`). A `# adr-link: <adr-name>` comment added to `Cargo.toml` near `[profile.dev]` following the semantic-link convention in `docs/skills/semantic-skill-link-convention.md`. Skip if the change is straightforward (e.g. removing an obsolete workaround). |
| T3  | TODO   | Run the full local test suite with the updated dev profile                          | `cargo test --tests --benches --examples --workspace --all-targets --all-features` exits with code 0                                                                                                                                                                                                                                        |
| T4  | TODO   | Run the Docker build with the updated dev profile to verify no bitcode error        | `docker build --target release ...` completes without `failed to load bitcode` error                                                                                                                                                                                                                                                        |
| T5  | TODO   | If T4 fails: scope the workaround narrowly and document the upstream tracking issue | Narrowest fix applied; comment in `Cargo.toml` explains why with a link                                                                                                                                                                                                                                                                     |
| T6  | TODO   | Run pre-commit checks                                                               | `./contrib/dev-tools/git/hooks/pre-commit.sh` exits with code 0                                                                                                                                                                                                                                                                             |

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

- 2026-06-03 00:00 UTC - GitHub Copilot - Spec drafted after investigating git history for `lto = "fat"` in `[profile.dev]`; root cause traced to commit `3c715fbb`

## Acceptance Criteria

- [ ] AC1: `[profile.dev]` in `Cargo.toml` does not use `lto = "fat"` (unless the Docker build failure is confirmed to still require it, in which case a comment linking to a tracking issue is present)
- [ ] AC1a: If the final LTO choice constitutes a non-obvious design decision, an ADR exists in `docs/adrs/` documenting the choice and rationale, and `Cargo.toml` carries a `# adr-link: <adr-name>` comment near `[profile.dev]` following `docs/skills/semantic-skill-link-convention.md`
- [ ] AC2: `cargo test --tests --benches --examples --workspace --all-targets --all-features` exits with code 0
- [ ] AC3: Docker build (`docker build --target release`) completes without a `failed to load bitcode` error
- [ ] AC4: `linter all` exits with code 0
- [ ] AC5: Manual verification scenarios are executed and documented (status + evidence)
- [ ] AC6: Acceptance criteria are re-reviewed after implementation and reflect actual behavior

## Verification Plan

### Automatic Checks

- `linter all`
- `cargo test --tests --benches --examples --workspace --all-targets --all-features`
- `./contrib/dev-tools/git/hooks/pre-commit.sh`

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID  | Scenario                                             | Command/Steps                                                                        | Expected Result                                    | Status | Evidence |
| --- | ---------------------------------------------------- | ------------------------------------------------------------------------------------ | -------------------------------------------------- | ------ | -------- |
| M1  | Local test suite passes without fat LTO in dev       | `cargo test --tests --benches --examples --workspace --all-targets --all-features`   | All tests pass, no bitcode errors                  | TODO   |          |
| M2  | Docker release build succeeds without fat LTO in dev | `docker build --target release --tag torrust-tracker:release --file Containerfile .` | Build completes; no `failed to load bitcode` error | TODO   |          |

Notes:

- M2 is the key regression guard for the original bug fix.
- If M2 fails, T5 applies: scope the workaround narrowly and document why.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| ----- | ---------------------- | -------- |
| AC1   | TODO                   |          |
| AC2   | TODO                   |          |
| AC3   | TODO                   |          |
| AC4   | TODO                   |          |
| AC5   | TODO                   |          |
| AC6   | TODO                   |          |

## Risks and Trade-offs

- If the bitcode LLVM bug is still present in some container environments, removing `lto = "fat"` from `[profile.dev]` could break Docker CI builds. Mitigation: verify M2 before merging; scope any required workaround to the narrowest target (e.g. a per-crate `[profile.dev.package.criterion]` override or a Containerfile-level `CARGO_PROFILE_DEV_LTO` env var).
- `lto = "fat"` in `[profile.dev]` has been present since mid-2024; removing it will improve local incremental build times noticeably for all contributors.

## References

- Commit `3c715fbb` — original workaround: "fix: [#898] docker build error: failed to load bitcode of module criterion"
- [Cargo reference — profiles](https://doc.rust-lang.org/cargo/reference/profiles.html#lto)
- [Rust issue tracker — LTO bitcode compatibility](https://github.com/rust-lang/rust/issues) (search "failed to load bitcode")
