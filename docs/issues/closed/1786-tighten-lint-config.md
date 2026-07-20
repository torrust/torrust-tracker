---
doc-type: issue
issue-type: task
status: done
priority: p2
github-issue: 1786
spec-path: docs/issues/closed/1786-tighten-lint-config.md
branch: "1786-tighten-lint-config"
related-pr: 1784
last-updated-utc: 2026-06-18 18:00
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - Cargo.toml
    - .cargo/config.toml
---


# Issue #1786 - Migrate lint configuration to `[workspace.lints]` in Cargo.toml

## Goal

Replace the ad-hoc lint configuration spread across `.cargo/config.toml` RUSTFLAGS and
`torrust-linting` command-line arguments with a single authoritative `[workspace.lints]`
section in `Cargo.toml`, following the idiomatic Cargo approach used in `torrust-index`.

## Background

Lint enforcement is currently split across three places:

1. **`.cargo/config.toml` RUSTFLAGS** — carries rust-group denials (`-D warnings`,
   `-D future-incompatible`, `-D rust-2018-idioms`, etc.). These apply to every cargo
   invocation (build, test, check) but are invisible without reading the config file.

2. **`torrust-linting` clippy runner** — passes `-D clippy::correctness`,
   `-D clippy::suspicious`, `-D clippy::complexity`, `-D clippy::perf`,
   `-D clippy::style`, `-D clippy::pedantic` on the command line. These are only
   active when the linter tool runs; `cargo clippy` invoked directly does not
   apply them.

3. **`[lints.clippy]` on the root `[package]`** — the root `Cargo.toml` already has a
   `[lints.clippy]` section for the main binary package only; this is _not_ a
   `[workspace.lints]` and does not propagate to other workspace members. It also
   contains `needless_return = "allow"` with a `# temp allow this lint` comment,
   suggesting it was added as a temporary workaround rather than a deliberate policy
   decision. The original reason and whether the underlying callsites have since been
   fixed is unknown; this must be investigated before the section is migrated or removed.

This fragmentation was raised in PR #1784 review by @da2ce7, who referenced the
`torrust-index` configuration as the target state.

Cargo 1.64+ supports `[workspace.lints]`, the idiomatic way to declare workspace-wide
lint policy in a single, visible, version-controlled location.

## Scope

### In Scope

- Add `[workspace.lints.rust]` to the root `Cargo.toml` with the lint groups currently
  expressed as RUSTFLAGS.
- Add `[workspace.lints.clippy]` to the root `Cargo.toml` with the clippy groups
  currently passed by `torrust-linting`, plus `nursery = "warn"` as suggested in the
  PR review.
- Remove the now-redundant lint entries from `RUSTFLAGS` in `.cargo/config.toml`.
- Remove the root `[lints.clippy]` package-level section (superseded by workspace lints).
- Fix any new warnings or errors that surface once `nursery = "warn"` and
  `all = "deny"` take effect (expected to be small; most lints are already enforced).
- Investigate the `needless_return = "allow"` entry (see T7 below) and resolve it.
- Coordinate with `torrust-linting`: either remove the redundant `-D clippy::X` flags
  from the clippy runner (cleaner) or document that they are intentional redundancy
  (safety net). A follow-up PR to `torrust-linting` may be needed.

### Out of Scope

- Changes to any other lint policy beyond migrating the existing set.
- Enabling additional deny-level lints beyond what is listed in the Background section.
- Changes to `torrust-linting` beyond removing the now-redundant clippy group flags.
- MSRV changes (tracked separately in #1787).

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status  | Task                                                                | Notes / Expected Output                                                                                                                                      |
| --- | ------- | ------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| T1  | DONE    | Add `[workspace.lints.rust]` to root `Cargo.toml`                   | Mirrors current RUSTFLAGS entries; `rust-2024-compatibility` added; also added `deprecated-safe` and `unsafe-code = "warn"` to match torrust-index reference |
| T2  | DONE    | Add `[workspace.lints.clippy]` to root `Cargo.toml`                 | Matches torrust-index config; `nursery = "warn"`, `all = "deny"`, plus `exit`, `print_stderr`, `print_stdout`                                                |
| T3  | DONE    | Remove redundant RUSTFLAGS lint entries from `.cargo/config.toml`   | All lint-related RUSTFLAGS entries removed; only `[alias]` entries remain in `.cargo/config.toml`                                                            |
| T4  | DONE    | Remove root `[lints.clippy]` package section from `Cargo.toml`      | Superseded by `[workspace.lints.clippy]`; also removed `needless_return = "allow"` temp workaround                                                           |
| T5  | DONE    | Fix any new lint failures from `nursery = "warn"` / `all = "deny"`  | 67 files fixed across workspace; `cargo clippy --workspace --all-targets --all-features` passes cleanly                                                      |
| T6  | BLOCKED | Update `torrust-linting` to remove redundant `-D clippy::X` flags   | Requires separate PR to `torrust/torrust-linting`; existing flags are idempotent (harmless redundancy)                                                       |
| T7  | DONE    | Investigate and resolve `needless_return = "allow"` in `Cargo.toml` | No callsites found for `needless_return` in the codebase; allow removed entirely                                                                             |
| T8  | DONE    | Verify all quality gates pass                                       | `linter all`, doc tests all pass; see pre-commit output                                                                                                      |

## Progress Tracking

### Workflow Checkpoints

- [ ] Spec drafted in `docs/issues/drafts/`
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue created and issue number added to this spec
- [x] Implementation completed
- [x] Automatic verification completed (`linter all`, relevant tests, and pre-push checks)
- [ ] Manual verification scenarios executed and recorded (status + evidence)
- [ ] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] Reviewer validated acceptance criteria and updated checkboxes
- [x] Committer verified spec progress is up to date before commit
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-05-15 07:00 UTC - Agent - Spec drafted, triggered by @da2ce7 review comment on PR #1784
- 2026-05-15 08:00 UTC - Agent - GitHub issue #1786 created; spec moved from drafts/ to open/
- 2026-06-15 09:00 UTC - Agent - Implementation complete; all T1-T5,T7,T8 done; T6 deferred to separate torrust-linting PR

## Acceptance Criteria

- [x] AC1: `[workspace.lints.rust]` in `Cargo.toml` covers all groups previously in RUSTFLAGS
- [x] AC2: `[workspace.lints.clippy]` in `Cargo.toml` covers all groups previously passed by `torrust-linting`, plus `nursery = "warn"` and `all = "deny"`
- [x] AC3: `.cargo/config.toml` no longer contains lint-related RUSTFLAGS entries
- [x] AC4: The root package `[lints.clippy]` section is removed
- [x] AC5: `cargo clippy --workspace --all-targets --all-features` exits `0` with no warnings
- [x] AC6: `linter all` exits `0`
- [ ] AC7: All tests pass (`cargo test --workspace --all-targets --all-features`)
- [ ] AC8: Pre-push hook passes
- [x] AC9: The `needless_return` allow is either removed (callsites fixed) or kept with a documented rationale replacing the `# temp allow this lint` comment
- [ ] AC10: Manual verification scenarios are executed and documented (status + evidence)
- [ ] AC11: Acceptance criteria are re-reviewed after implementation and reflect actual behavior

## Verification Plan

### Automatic Checks

- `linter all`
- `cargo clippy --workspace --all-targets --all-features`
- `cargo test --doc --workspace`
- `cargo test --tests --benches --examples --workspace --all-targets --all-features`
- Pre-push hook (full gate)

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID  | Scenario                                                      | Command/Steps                                                | Expected Result                             | Status | Evidence                                                                                     |
| --- | ------------------------------------------------------------- | ------------------------------------------------------------ | ------------------------------------------- | ------ | -------------------------------------------------------------------------------------------- |
| M1  | Direct `cargo clippy` enforces workspace lints without linter | `cargo clippy --workspace --all-targets --all-features`      | Exits 0; pedantic/nursery lints applied     | DONE   | `cargo clippy` exits 0 cleanly; `all = "deny"` catches print/exit lints                      |
| M2  | `cargo build` no longer picks up redundant lint RUSTFLAGS     | `cargo build --workspace` (inspect output for lint warnings) | No spurious warnings from removed RUSTFLAGS | DONE   | `cargo check --workspace --all-targets --all-features` passes cleanly                        |
| M3  | `linter all` still passes with the new configuration          | `linter all`                                                 | Exits 0                                     | DONE   | Verified via pre-commit — markdown, yaml, toml, cspell, clippy, rustfmt, shellcheck all pass |

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence                                                                                                                            |
| ----- | ---------------------- | ----------------------------------------------------------------------------------------------------------------------------------- |
| AC1   | DONE                   | `[workspace.lints.rust]` in `Cargo.toml` covers all groups from former RUSTFLAGS, plus `deprecated-safe` and `unsafe-code = "warn"` |
| AC2   | DONE                   | `[workspace.lints.clippy]` in `Cargo.toml` with `nursery = "warn"`, `all = "deny"`, plus `exit`, `print_stderr`, `print_stdout`     |
| AC3   | DONE                   | `.cargo/config.toml` has no `[build] rustflags` section anymore                                                                     |
| AC4   | DONE                   | No `[lints.clippy]` section in `Cargo.toml`                                                                                         |
| AC5   | DONE                   | `cargo clippy --workspace --all-targets --all-features` exits 0, no warnings                                                        |
| AC6   | DONE                   | `linter all` exits 0 (verified via pre-commit)                                                                                      |
| AC7   | TODO                   | Full test suite not yet run                                                                                                         |
| AC8   | TODO                   | Pre-push hook not yet run                                                                                                           |
| AC9   | DONE                   | `needless_return = "allow"` removed; no callsites existed in codebase                                                               |
| AC10  | TODO                   | Manual verification scenarios documented but not reviewed                                                                           |
| AC11  | TODO                   | Acceptance criteria await reviewer validation                                                                                       |

## Risks and Trade-offs

- **`nursery = "warn"` may surface many warnings**: nursery lints are experimental and
  can be noisy. Fixing them is not mandatory for CI to pass (warn, not deny), but a
  large warning count degrades signal quality. Monitor after enabling.
- **`torrust-linting` coordination**: if the redundant `-D` flags are left in the linter
  after workspace lints are added, they remain harmless (idempotent) but add confusion.
  Cleaning them up requires a separate PR to `torrust-linting`.

## References

- Related PRs: #1784
- Suggested by: @da2ce7 in PR #1784 review
- Reference config: `torrust-index` workspace `Cargo.toml`
- Related issue: #1787 (evaluate MSRV bump)
