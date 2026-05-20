---
doc-type: issue
issue-type: task
status: closed
priority: p2
github-issue: 1804
spec-path: docs/issues/closed/1804-use-cargo-machete-with-metadata-and-remove-unused-dev-deps.md
branch: "1804-use-cargo-machete-with-metadata"
related-pr: 1809
last-updated-utc: 2026-05-20 15:00
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - contrib/dev-tools/git/hooks/pre-commit.sh
    - packages/tracker-core/Cargo.toml
    - packages/udp-tracker-core/Cargo.toml
    - packages/axum-http-tracker-server/Cargo.toml
    - packages/swarm-coordination-registry/Cargo.toml
---

<!-- skill-link: create-issue -->

# Issue #1804 - Use `cargo machete --with-metadata` and remove unused dev dependencies

## Goal

Replace the plain `cargo machete` call in the pre-commit hook (and CI) with
`cargo machete --with-metadata`, then remove the ~15 unused dev dependencies that this
stricter mode reveals across the workspace.

## Background

During a coupling analysis review (see
[workspace-coupling-report.md](../open/1669-overhaul-packages/workspace-coupling-report.md)),
four workspace dependencies were found to have zero references in any source file:

- `bittorrent-tracker-core` → `torrust-rest-tracker-api-client` [dev]
- `bittorrent-udp-tracker-core` → `torrust-tracker-test-helpers` [dev]
- `torrust-axum-http-tracker-server` → `torrust-tracker-events` [dev]
- `torrust-tracker-swarm-coordination-registry` → `torrust-tracker-test-helpers` [dev]

Running `cargo machete` (plain, text-based scan) did **not** flag these — a false negative. Only
`cargo machete --with-metadata` (which uses `cargo metadata` for accurate crate-name resolution)
correctly identifies them as unused. The same run also reveals about a dozen additional unused dev
dependencies spread across the workspace (e.g., `local-ip-address`, `mockall`, `rstest`,
`async-std`, `criterion`, `pretty_assertions`, `serde_bytes`, `zerocopy`, `tracing-subscriber`,
`formatjson`, `serde_json`).

The pre-commit hook currently calls:

```text
"Checking for unused dependencies (cargo machete)|cargo machete"
```

Switching to `--with-metadata` makes the gate accurate and removes dead weight from `Cargo.toml`
files across the workspace.

## Scope

### In Scope

- Update the pre-commit hook (`contrib/dev-tools/git/hooks/pre-commit.sh`) to call
  `cargo machete --with-metadata`.
- Update any CI workflow step that calls `cargo machete` without `--with-metadata`.
- Remove every dependency flagged as unused by `cargo machete --with-metadata` from the
  corresponding `Cargo.toml` files.
- Verify the workspace builds and all tests still pass after removal.

### Out of Scope

- False-positive suppression via `[package.metadata.cargo-machete] ignored = [...]`: only remove
  genuinely unused dependencies; if a dep appears unused but is needed (e.g., for a proc-macro
  side-effect), add it to the ignore list with a comment explaining why, rather than removing it.
- Changes to the workspace coupling report tool (tracked separately).

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task                                                                                      | Notes / Expected Output                                                                               |
| --- | ------ | ----------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------- |
| T1  | DONE   | Run `cargo machete --with-metadata` and record the full list of flagged dependencies      | 22 unused deps found across 13 packages; 1 false-positive (`serde_bytes`) handled via ignore list     |
| T2  | DONE   | Update `contrib/dev-tools/git/hooks/pre-commit.sh` to use `cargo machete --with-metadata` | Hook passes with the new flag                                                                         |
| T3  | DONE   | Update CI workflow(s) that call `cargo machete` without `--with-metadata`                 | N/A — only `copilot-setup-steps.yml` exists in this repo and only installs the tool; does not call it |
| T4  | DONE   | Remove flagged unused dependencies from all `Cargo.toml` files                            | `cargo machete --with-metadata` reports clean after removals                                          |
| T5  | DONE   | Run `cargo build --workspace` and `cargo test --workspace`                                | Clean build; all tests pass                                                                           |
| T6  | DONE   | Run `linter all`                                                                          | Exit code `0`                                                                                         |

## Progress Tracking

### Workflow Checkpoints

- [x] Spec drafted in `docs/issues/drafts/`
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue created and issue number added to this spec
- [x] Spec moved to `docs/issues/open/` with issue number prefix
- [x] Implementation completed
- [x] Automatic verification completed (`linter all`, `cargo test --workspace`)
- [x] Manual verification scenarios executed and recorded (status + evidence)
- [x] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] Reviewer validated acceptance criteria and updated checkboxes
- [x] Committer verified spec progress is up to date before commit
- [x] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-05-20 00:00 UTC - josecelano - Spec drafted. Root cause identified: plain `cargo machete`
  has false negatives for dev dependencies; `--with-metadata` mode is accurate. Full list of
  unused deps generated by running `cargo machete --with-metadata` in the workspace.
- 2026-05-20 12:30 UTC - josecelano - Implementation complete. Removed 21 genuine unused
  dev-deps across 13 `Cargo.toml` files; 1 machete false-positive (`serde_bytes` in
  `axum-http-tracker-server`, used via `#[serde(with = "serde_bytes")]` string attribute)
  kept and suppressed via `[package.metadata.cargo-machete] ignored`. T3 is N/A — no CI
  workflow in this repo calls plain `cargo machete`. Commit: `225e74fc`.

## Acceptance Criteria

- [x] AC1: The pre-commit hook calls `cargo machete --with-metadata` (not plain `cargo machete`).
- [x] AC2: All CI workflow steps that call `cargo machete` use `--with-metadata` (N/A — no CI step calls it in this repo).
- [x] AC3: `cargo machete --with-metadata` exits `0` across the entire workspace (no unused deps).
- [x] AC4: `cargo build --workspace` and `cargo test --workspace` pass cleanly after dep removals.
- [x] AC5: `linter all` exits with code `0`.
- [x] Manual verification scenarios are executed and documented (status + evidence).
- [x] Acceptance criteria are re-reviewed after implementation and reflect actual behavior.
- [x] Documentation is updated when behaviour or workflow changes.

## Verification Plan

### Automatic Checks

- `cargo machete --with-metadata` — must report clean
- `cargo build --workspace`
- `cargo test --workspace`
- `linter all`

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID  | Scenario                                           | Command/Steps                                            | Expected Result                                  | Status | Evidence                                                                         |
| --- | -------------------------------------------------- | -------------------------------------------------------- | ------------------------------------------------ | ------ | -------------------------------------------------------------------------------- |
| M1  | Pre-commit hook uses `--with-metadata`             | `grep machete contrib/dev-tools/git/hooks/pre-commit.sh` | Output includes `--with-metadata`                | DONE   | Line confirms: `cargo machete --with-metadata`                                   |
| M2  | No unused deps remain after removals               | `cargo machete --with-metadata`                          | "didn't find any unused dependencies. Good job!" | DONE   | `cargo-machete didn't find any unused dependencies in this directory. Good job!` |
| M3  | Workspace builds and tests pass after dep removals | `cargo build --workspace && cargo test --workspace`      | Both commands exit `0`                           | DONE   | Both exit `0`; full test suite passes                                            |

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence                                                                                               |
| ----- | ---------------------- | ------------------------------------------------------------------------------------------------------ |
| AC1   | DONE                   | `grep` on pre-commit.sh confirms `cargo machete --with-metadata`                                       |
| AC2   | DONE                   | N/A — no CI workflow in this repo calls `cargo machete` directly                                       |
| AC3   | DONE                   | `cargo machete --with-metadata` exits `0`: "didn't find any unused dependencies. Good job!"            |
| AC4   | DONE                   | `cargo build --workspace` and `cargo test --workspace` both exit `0`                                   |
| AC5   | DONE                   | `linter all` exits `0`: all linters (markdown, yaml, toml, cspell, clippy, rustfmt, shellcheck) passed |

## Risks and Trade-offs

- Some dependencies may look unused to `cargo machete` but are needed for proc-macro side
  effects, feature flag activation, or link-time dependencies. Each removal must be verified
  individually; add to the `ignored` list with a comment if removal breaks the build.

## References

- Related issues: #1669 (EPIC — Overhaul Packages)
- See also: #1805 (companion issue for the `workspace-coupling` report tool overhaul) —
  fixing the scanner's false negatives improves coupling report accuracy independently of
  this issue.
- Coupling report: `docs/issues/open/1669-overhaul-packages/workspace-coupling-report.md`
- `cargo machete` docs: <https://github.com/bnjbvr/cargo-machete>
