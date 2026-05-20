---
doc-type: issue
issue-type: task
status: open
priority: p2
github-issue: 1804
spec-path: docs/issues/open/1804-use-cargo-machete-with-metadata-and-remove-unused-dev-deps.md
branch: "1804-use-cargo-machete-with-metadata"
related-pr: null
last-updated-utc: 2026-05-20 00:00
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

| ID  | Status | Task                                                                                      | Notes / Expected Output                                        |
| --- | ------ | ----------------------------------------------------------------------------------------- | -------------------------------------------------------------- |
| T1  | TODO   | Run `cargo machete --with-metadata` and record the full list of flagged dependencies      | Baseline list; confirm each is genuinely unused before removal |
| T2  | TODO   | Update `contrib/dev-tools/git/hooks/pre-commit.sh` to use `cargo machete --with-metadata` | Hook passes with the new flag                                  |
| T3  | TODO   | Update CI workflow(s) that call `cargo machete` without `--with-metadata`                 | CI step passes with the new flag                               |
| T4  | TODO   | Remove flagged unused dependencies from all `Cargo.toml` files                            | `cargo machete --with-metadata` reports clean after removals   |
| T5  | TODO   | Run `cargo build --workspace` and `cargo test --workspace`                                | Clean build; all tests pass                                    |
| T6  | TODO   | Run `linter all`                                                                          | Exit code `0`                                                  |

## Progress Tracking

### Workflow Checkpoints

- [x] Spec drafted in `docs/issues/drafts/`
- [ ] Spec reviewed and approved by user/maintainer
- [ ] GitHub issue created and issue number added to this spec
- [ ] Spec moved to `docs/issues/open/` with issue number prefix
- [ ] Implementation completed
- [ ] Automatic verification completed (`linter all`, `cargo test --workspace`)
- [ ] Manual verification scenarios executed and recorded (status + evidence)
- [ ] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] Reviewer validated acceptance criteria and updated checkboxes
- [ ] Committer verified spec progress is up to date before commit
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-05-20 00:00 UTC - josecelano - Spec drafted. Root cause identified: plain `cargo machete`
  has false negatives for dev dependencies; `--with-metadata` mode is accurate. Full list of
  unused deps generated by running `cargo machete --with-metadata` in the workspace.

## Acceptance Criteria

- [ ] AC1: The pre-commit hook calls `cargo machete --with-metadata` (not plain `cargo machete`).
- [ ] AC2: All CI workflow steps that call `cargo machete` use `--with-metadata`.
- [ ] AC3: `cargo machete --with-metadata` exits `0` across the entire workspace (no unused deps).
- [ ] AC4: `cargo build --workspace` and `cargo test --workspace` pass cleanly after dep removals.
- [ ] AC5: `linter all` exits with code `0`.
- [ ] Manual verification scenarios are executed and documented (status + evidence).
- [ ] Acceptance criteria are re-reviewed after implementation and reflect actual behavior.
- [ ] Documentation is updated when behaviour or workflow changes.

## Verification Plan

### Automatic Checks

- `cargo machete --with-metadata` — must report clean
- `cargo build --workspace`
- `cargo test --workspace`
- `linter all`

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID  | Scenario                                           | Command/Steps                                            | Expected Result                                  | Status | Evidence |
| --- | -------------------------------------------------- | -------------------------------------------------------- | ------------------------------------------------ | ------ | -------- |
| M1  | Pre-commit hook uses `--with-metadata`             | `grep machete contrib/dev-tools/git/hooks/pre-commit.sh` | Output includes `--with-metadata`                | TODO   |          |
| M2  | No unused deps remain after removals               | `cargo machete --with-metadata`                          | "didn't find any unused dependencies. Good job!" | TODO   |          |
| M3  | Workspace builds and tests pass after dep removals | `cargo build --workspace && cargo test --workspace`      | Both commands exit `0`                           | TODO   |          |

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| ----- | ---------------------- | -------- |
| AC1   | TODO                   |          |
| AC2   | TODO                   |          |
| AC3   | TODO                   |          |
| AC4   | TODO                   |          |
| AC5   | TODO                   |          |

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
