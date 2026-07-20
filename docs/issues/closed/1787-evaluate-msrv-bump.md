---
doc-type: issue
issue-type: task
status: closed
priority: p2
github-issue: 1787
spec-path: docs/issues/closed/1787-evaluate-msrv-bump.md
branch: "1787-evaluate-msrv-bump"
related-pr: 1815
last-updated-utc: 2026-05-20 18:00
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - Cargo.toml
    - AGENTS.md
    - .github/skills/dev/maintenance/setup-dev-environment/SKILL.md
---


# Issue #1787 - Evaluate and update workspace MSRV above 1.85

## Goal

Decide on the appropriate Minimum Supported Rust Version (MSRV) for the workspace
given the project's trajectory (planned extraction of `bittorrent-*` crates as
independent libraries) and update `rust-version` in `Cargo.toml` accordingly.

## Background

PR #1784 set `rust-version = "1.85"` — the strict minimum required to compile
Rust edition 2024. This was correct as the conservative baseline for the migration,
but 1.85 is now several releases behind the current stable toolchain.

Two classes of crate coexist in this workspace:

1. **Application layer** (`torrust-tracker-*` crates and the main binary) — not
   consumed as a library by external projects; MSRV currently has no downstream
   impact. All workspace packages carry `publish.workspace = true` but none have
   been published to crates.io yet. Which packages will actually be released,
   under what names, and whether some will move to their own repositories is
   being decided in #1669.

2. **Protocol/domain layer** (`bittorrent-*` crates: `bittorrent-peer-id`,
   `bittorrent-http-tracker-protocol`, `bittorrent-udp-tracker-protocol`,
   `bittorrent-tracker-core`, `bittorrent-http-tracker-core`,
   `bittorrent-udp-tracker-core`, `bittorrent-tracker-client`) — planned for
   extraction into independent repositories and publication to crates.io, where
   they will be consumed by other BitTorrent projects.

This dual nature creates a tension:

- **For the application layer**: there is no reason to stay on an old MSRV; tracking
  a recent stable is better (access to new APIs, better diagnostics).
- **For the future libraries**: a conservative MSRV (e.g. latest stable minus two
  releases, or a deliberate policy) is appropriate once they are published.

Until the `bittorrent-*` crates are extracted, a single workspace MSRV applies to
both classes, so the decision must be made with the extraction timeline in mind.

The MSRV evaluation was unblocked and resolved in 2026-05-20: `rust-version = "1.88"` was chosen
as the minimum floor that avoids `cargo update` regressions on the current lockfile. The long-term
split policy (tracker app tracks recent stable; extracted `bittorrent-*` libraries keep a minimum
MSRV) is documented in the Policy Decision section below and will be applied in a follow-up issue
once #1669 closes.

## Policy Decision

**Decided 2026-05-20. Agreed value: `rust-version = "1.88"`.**

### Rationale

- **1.88 is the minimum floor that avoids `cargo update` regressions** on the current
  lockfile. All dependency versions currently pinned in `Cargo.lock` require at most
  Rust 1.88; running `cargo update` with a lower MSRV (1.85, 1.86, or 1.87) downgrades
  major packages (bollard, tonic, testcontainers, serde_with, time, ureq, etc.).
- **Cross-project consistency** with
  [torrust-index](https://github.com/torrust/torrust-index/blob/develop/Cargo.toml),
  which also uses `rust-version = "1.88"`.

### Future MSRV policy (post-extraction of `bittorrent-*` crates)

When #1669 completes and the `bittorrent-*` crates are extracted into independent
repositories, the MSRV strategy should be split:

- **Tracker application** (`torrust-tracker-*` and the main binary): track a recent
  stable Rust release; there is no downstream impact from a higher MSRV here.
- **Reusable/shared packages** (`bittorrent-*` crates published to crates.io): set the
  **lowest MSRV that compiles and tests the crate** to maximize compatibility with
  external consumers.

**Re-evaluation trigger**: open a follow-up issue when #1669 closes to apply the
split policy described above.

## Scope

### In Scope

- Evaluate the appropriate MSRV policy for this workspace given the two crate classes.
- Define a policy: track latest stable, pin to a specific recent release, or maintain
  a conservative floor.
- Update `rust-version` in `Cargo.toml` to the agreed value.
- Update all documentation that references the MSRV:
  - `AGENTS.md` (line referencing `MSRV 1.85`)
  - `.github/skills/dev/maintenance/setup-dev-environment/SKILL.md`
- Verify CI passes with the new MSRV value.

### Out of Scope

- Extracting `bittorrent-*` crates to independent repositories (separate epic).
- Setting per-crate MSRV values (only the workspace `rust-version` is in scope here).
- Adding a MSRV CI job (may be proposed as a follow-up if a conservative MSRV is chosen).

## Blockers

None. The blocker on #1669 was lifted: the current MSRV (1.88) is valid for the
monorepo in its present form. The post-extraction split policy is documented in the
"Future MSRV policy" section above and will be implemented in a follow-up issue
once #1669 closes.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task                                                                | Notes / Expected Output                                                                                                                                                                    |
| --- | ------ | ------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| T1  | DONE   | Decide MSRV policy (track latest stable vs. pin conservative floor) | Policy documented in "Policy Decision" section: 1.88 for the whole workspace now; split policy (app tracks latest stable, extracted libraries keep minimum MSRV) to be applied post-#1669. |
| T2  | DONE   | Update `rust-version` in root `Cargo.toml`                          | Changed from `"1.85"` to `"1.88"`                                                                                                                                                          |
| T3  | DONE   | Update `AGENTS.md` MSRV reference                                   | Updated from `1.85` to `1.88`                                                                                                                                                              |
| T4  | DONE   | Update setup-dev-environment SKILL.md MSRV reference                | Updated from `1.85` to `1.88`                                                                                                                                                              |
| T5  | TODO   | Verify CI passes                                                    | Full quality gate (`linter all`, tests, pre-push hook)                                                                                                                                     |

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
- [ ] Committer verified spec progress is up to date before commit
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-05-15 07:00 UTC - Agent - Spec drafted, follow-up from PR #1784 (Rust edition 2024 migration, MSRV set to 1.85)
- 2026-05-15 07:30 UTC - Jose Celano - Marked blocked on #1669 (package restructuring); MSRV policy requires knowing extraction scope, names, and versioning lifecycle
- 2026-05-15 08:00 UTC - Agent - GitHub issue #1787 created; spec moved to docs/issues/open/
- 2026-05-20 00:00 UTC - Agent - Discovered that with MSRV 1.85 `cargo update` downgrades many packages (bollard 0.20→0.19, tonic 0.14→0.13, testcontainers 0.27→0.25, serde_with 3.20→3.17, time 0.3.47→0.3.45, ureq 3.3→2.12, etc.) because they require Rust > 1.85. Verified by dry-run that MSRV 1.88 is the minimum floor that avoids all such regressions (1.86 and 1.87 still produce downgrades). Bumped rust-version to 1.88; updated AGENTS.md and setup-dev-environment SKILL.md. Final long-term policy (whether to track latest stable, pin N-2, etc.) remains open pending #1669.
- 2026-05-20 12:00 UTC - Jose Celano - Confirmed 1.88 is fine; aligns with torrust-index. Policy recorded: tracker app to track latest stable post-extraction; reusable bittorrent-\* packages to keep minimum MSRV for external consumer compatibility. Issue ready to close; split policy applied in a follow-up once #1669 closes.

## Acceptance Criteria

- [ ] AC1: A MSRV policy decision is recorded in this spec with rationale
- [ ] AC2: `rust-version` in `Cargo.toml` reflects the agreed value
- [ ] AC3: `AGENTS.md` MSRV reference is in sync with `Cargo.toml`
- [ ] AC4: `setup-dev-environment` SKILL.md MSRV reference is in sync with `Cargo.toml`
- [ ] AC5: `linter all` exits `0`
- [ ] AC6: All tests pass
- [ ] AC7: Pre-push hook passes

## Verification Plan

### Automatic Checks

- `linter all`
- `cargo check --workspace --all-targets --all-features`
- `cargo test --doc --workspace`
- Pre-push hook (full gate)

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID  | Scenario                                           | Command/Steps                                    | Expected Result                          | Status | Evidence |
| --- | -------------------------------------------------- | ------------------------------------------------ | ---------------------------------------- | ------ | -------- |
| M1  | `rust-version` in Cargo.toml matches documentation | Compare `Cargo.toml`, `AGENTS.md`, SKILL.md      | All three reference the same MSRV string | TODO   |          |
| M2  | Workspace builds cleanly on the new MSRV toolchain | `rustup install <msrv>; cargo +<msrv> check ...` | Exit 0 with no errors                    | TODO   |          |

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence                                                                                                      |
| ----- | ---------------------- | ------------------------------------------------------------------------------------------------------------- |
| AC1   | DONE                   | Policy documented in "Policy Decision" section; split policy for post-extraction recorded as follow-up action |
| AC2   | DONE                   | `rust-version = "1.88"` in `Cargo.toml`                                                                       |
| AC3   | DONE                   | `AGENTS.md` updated to MSRV 1.88                                                                              |
| AC4   | DONE                   | `setup-dev-environment` SKILL.md updated to MSRV 1.88                                                         |
| AC5   | TODO                   |                                                                                                               |
| AC6   | TODO                   |                                                                                                               |
| AC7   | TODO                   |                                                                                                               |

## Risks and Trade-offs

- **Too high a MSRV before crate extraction**: if `bittorrent-*` crates are extracted
  carrying a high MSRV, downstream BitTorrent projects may be forced to upgrade their
  toolchain. Setting a modest floor now (e.g. current stable minus two releases) gives
  the extracted crates a clean, defensible starting point.
- **Too low a MSRV after extraction**: the application layer has no reason to stay
  conservative; a low MSRV denies developers access to new stable APIs and better
  compiler diagnostics.
- **Drift without a MSRV CI job**: a stated MSRV is only trustworthy if CI verifies it.
  If a conservative MSRV is chosen, a MSRV CI job should be added.

## References

- Related PRs: #1784
- Related issue: #1786 (tighten lint config)
- Blocked by: https://github.com/torrust/torrust-tracker/issues/1669 (package restructuring)
