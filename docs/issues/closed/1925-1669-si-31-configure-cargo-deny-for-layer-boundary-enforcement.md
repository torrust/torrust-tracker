---
doc-type: issue
issue-type: task
status: completed
priority: p2
github-issue: 1925
spec-path: docs/issues/closed/1925-1669-si-31-configure-cargo-deny-for-layer-boundary-enforcement.md
branch: 1925-configure-cargo-deny-for-layer-boundary-enforcement
related-pr: 1932
last-updated-utc: 2026-06-23
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - deny.toml
    - .github/workflows/testing.yaml
    - .github/workflows/copilot-setup-steps.yml
    - contrib/dev-tools/git/hooks/pre-commit.sh
    - Cargo.toml
    - packages/AGENTS.md
    - docs/packages.md
    - docs/issues/open/1669-overhaul-packages/EPIC.md
    - docs/issues/open/1669-overhaul-packages/DECISIONS.md
    - docs/issues/open/1669-overhaul-packages/workspace-coupling-report-2026-06-10.md
---

<!-- skill-link: create-issue -->

# Issue #1925 - Configure `cargo deny` for workspace layer boundary enforcement

## Goal

Install and configure [`cargo deny`](https://embarkstudios.github.io/cargo-deny/) to
programmatically enforce the workspace's layered architecture rules, preventing
accidental dependency edges between layers from being introduced.

## Background

The workspace has a documented layered architecture (see `packages/AGENTS.md` and the
EPIC #1669 layer guardrails). Dependencies may only flow downward — outer layers
(servers) may depend on inner layers (core, protocol, domain), but inner layers must
**not** depend on outer layers.

The EPIC defines these forbidden edges:

- `core -> server`
- `tracker-core -> core`
- `tracker-core -> protocol`
- `tracker-core -> server`
- `protocol -> core`
- `protocol -> tracker-core`
- `protocol -> server`

Currently, these rules are documented but not enforced by CI. A developer could
accidentally add a `core -> server` dependency edge (e.g., a core package depending
on `udp-server`) and it would compile and pass CI without any check.

`cargo deny` is the standard Rust tool for linting dependency graphs. Its **bans
check** supports per-crate `wrappers` — a mechanism to allow a crate as a
dependency only for a specific set of direct dependents while denying it for
everyone else. This is exactly the primitive needed for layer enforcement.

### Why `cargo deny` instead of other approaches

| Approach                       | Limitation                                  |
| ------------------------------ | ------------------------------------------- |
| Manual code review             | Human error; not automated                  |
| Custom CI script               | Reinventing `cargo deny`                    |
| Cargo features / cfg gates     | Wrong abstraction for this concern          |
| Rust compiler (`#![deny(..)]`) | Cannot enforce cross-crate dependency rules |

`cargo deny` is purpose-built for this, widely adopted in the Rust ecosystem,
and can be run as a GitHub Action or pre-commit hook.

## Scope

### In Scope

- Install `cargo deny` in CI (GitHub Action) or as part of the existing lint pipeline.
- Create `deny.toml` configuration file at the workspace root.
- Configure the `[bans]` section with explicit `deny` entries for each server crate,
  using `wrappers` to list the packages that are legitimately allowed to depend on them.
- Add `cargo deny check bans` to the CI testing workflow (GitHub Actions).
- Add `cargo deny check bans` to the pre-commit hook if it runs fast enough,
  otherwise to the pre-push hook. This decision aligns with the ongoing work
  in [#1843](https://github.com/torrust/torrust-tracker/issues/1843) (migrate
  git hooks from bash to Rust), which may introduce a generic orchestrator
  (like `torrust-linting` or a `just` recipe) that can run tasks on demand
  from git hooks, CI, or AI agent sessions.
- Configure the `[licenses]` and `[advisories]` checks if desired (secondary benefit).

### Out of Scope

- Fixing existing layer violations (those are separate subissues in EPIC #1669).
- Configuring `cargo deny` for non-workspace external dependencies (license checking,
  advisory scanning) — those are separate concerns.
- Configuring `cargo deny` sources checks.

### Known existing layer violation

One violation exists today: `rest-api-core` (a core-layer package) depends on
`udp-server` (a server-layer package). This is tracked in the dedicated subissue
[docs/issues/drafts/1669-decouple-rest-api-core-from-udp-internals.md](./1669-decouple-rest-api-core-from-udp-internals.md).
Until that violation is fixed, the `wrappers` list for `udp-server` will include
`rest-api-core` as a legitimate direct dependent (see the Proposed configuration
below for the exact entry). Once the decoupling is done, `rest-api-core` should be
removed from the wrappers list.

> **Execution order**: This issue should be implemented **after** the `rest-api-core`
> decoupling ([`1669-decouple-rest-api-core-from-udp-internals.md`](./1669-decouple-rest-api-core-from-udp-internals.md)),
> or the final PR must include a second commit that removes `rest-api-core` from the
> `udp-server` wrappers. If implemented first, deny will pass but the stale exception
> would remain until explicitly cleaned up.

## Layer map and forbidden edges

### Layer classification

| Layer                             | Packages                                                                                                                                                                                |
| --------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **Server** (`axum-*`, `*-server`) | `torrust-tracker-axum-http-server`, `torrust-tracker-axum-rest-api-server`, `torrust-tracker-axum-health-check-api-server`, `torrust-tracker-axum-server`, `torrust-tracker-udp-server` |
| **Core** (`*-core`)               | `torrust-tracker-core`, `torrust-tracker-http-core`, `torrust-tracker-udp-core`, `torrust-tracker-rest-api-core`                                                                        |
| **Protocol** (`*-protocol`)       | `torrust-tracker-http-protocol`, `torrust-tracker-udp-protocol`                                                                                                                         |
| **Domain / Shared**               | `torrust-tracker-configuration`, `torrust-tracker-primitives`, `torrust-tracker-events`, `torrust-tracker-swarm-coordination-registry`, `torrust-tracker-client-lib`                    |
| **Tools / Benchmarks**            | `torrust-tracker-test-helpers`, `torrust-tracker-torrent-repository-benchmarking`, `e2e-tools`, `persistence-benchmark`                                                                 |
| **CLI tools**                     | `torrust-tracker-client` (console binary)                                                                                                                                               |

### Forbidden dependency edges

| Edge                       | Description                                                    | Current violations            |
| -------------------------- | -------------------------------------------------------------- | ----------------------------- |
| `core -> server`           | Core must not depend on delivery-layer packages                | `rest-api-core -> udp-server` |
| `tracker-core -> core`     | Tracker core must not depend on its protocol-specific wrappers | None                          |
| `tracker-core -> protocol` | Tracker core must not depend on protocol parsing crates        | None                          |
| `tracker-core -> server`   | Tracker core must not depend on server crates                  | None                          |
| `protocol -> core`         | Protocol crates must not depend on core logic                  | None (SI-12 fixed this)       |
| `protocol -> tracker-core` | Protocol crates must not depend on tracker core                | None (SI-12 fixed this)       |
| `protocol -> server`       | Protocol crates must not depend on server crates               | None                          |
| `domain -> server`         | Domain/shared packages must not depend on server crates        | None                          |

### Legitimate direct dependents (wrappers)

These are the packages that are currently allowed to depend on server-layer crates:

| Server crate                                   | Legitimate direct dependents                                                                                          |
| ---------------------------------------------- | --------------------------------------------------------------------------------------------------------------------- |
| `torrust-tracker-axum-http-server`             | Root binary (`torrust-tracker`)                                                                                       |
| `torrust-tracker-axum-rest-api-server`         | Root binary (`torrust-tracker`)                                                                                       |
| `torrust-tracker-axum-health-check-api-server` | Root binary (`torrust-tracker`)                                                                                       |
| `torrust-tracker-axum-server`                  | `axum-http-server`, `axum-rest-api-server`, `axum-health-check-api-server`, root (`torrust-tracker`)                  |
| `torrust-tracker-udp-server`                   | `axum-rest-api-server`, `axum-health-check-api-server` (dev), `rest-api-core` (pending fix), root (`torrust-tracker`) |

## Proposed `deny.toml` configuration

```toml
# deny.toml
# Configuration for `cargo deny check bans`

[bans]
multiple-versions = "deny"
wildcards = "deny"

# Ban server-layer crates from being depended on by non-server packages.
# The `wrappers` list specifies which packages are allowed to use each
# server crate as a direct dependency. All other transitive uses are denied.
deny = [
    # axum server crates — only the root binary and other axum servers may depend on them
    { crate = "torrust-tracker-axum-http-server", wrappers = ["torrust-tracker"] },
    { crate = "torrust-tracker-axum-rest-api-server", wrappers = ["torrust-tracker"] },
    { crate = "torrust-tracker-axum-health-check-api-server", wrappers = ["torrust-tracker"] },
    { crate = "torrust-tracker-axum-server", wrappers = [
        "torrust-tracker-axum-http-server",
        "torrust-tracker-axum-rest-api-server",
        "torrust-tracker-axum-health-check-api-server",
        "torrust-tracker",
    ] },

    # udp server — only server-layer + root + rest-api-core (pending fix) may depend on it
    { crate = "torrust-tracker-udp-server", wrappers = [
        "torrust-tracker-axum-rest-api-server",
        "torrust-tracker-axum-health-check-api-server",
        "torrust-tracker-rest-api-core",
        "torrust-tracker",
    ] },

    # Protocol crates must not be used directly by torrust-tracker-core.
    # Only servers and the respective protocol-specific *-core may depend on them.
    { crate = "torrust-tracker-http-protocol", wrappers = [
        "torrust-tracker-axum-http-server",
        "torrust-tracker-http-core",
    ] },
    { crate = "torrust-tracker-udp-protocol", wrappers = [
        "torrust-tracker-client-lib",
        "torrust-tracker-udp-core",
        "torrust-tracker-udp-server",
    ] },

    # Core protocol-specific wrappers must not be depended on by tracker-core
    { crate = "torrust-tracker-http-core", wrappers = [
        "torrust-tracker-axum-http-server",
        "torrust-tracker-axum-rest-api-server",
        "torrust-tracker-rest-api-core",
        "torrust-tracker",
    ] },
    { crate = "torrust-tracker-udp-core", wrappers = [
        "torrust-tracker-udp-server",
        "torrust-tracker-axum-rest-api-server",
        "torrust-tracker-rest-api-core",
        "torrust-tracker",
    ] },
]
```

> **Crate name note**: The crate names above use the current naming convention (after SI-29).
> SI-29 is already done, so no name updates are needed.
>
> **Cross-reference to client extraction**: The wrappers list for `torrust-tracker-udp-protocol`
> currently includes `torrust-tracker-client-lib`. The console binary crate `torrust-tracker-client`
> was initially included but removed from wrappers during implementation because it is not
> consumed as a dependency by any other crate (standalone binary). When the
> client extraction removes `torrust-tracker-client-lib` from the workspace, its wrapper entry
> must also be removed.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task                                                                                            | Notes / Expected Output                                      |
| --- | ------ | ----------------------------------------------------------------------------------------------- | ------------------------------------------------------------ |
| T1  | DONE   | Install `cargo deny` (or confirm it's available)                                                | `cargo install --locked cargo-deny` (v0.19.9)                |
| T2  | DONE   | Create `deny.toml` at the workspace root with bans configuration                                | Created with minor adjustments (see final config below)      |
| T3  | DONE   | Run `cargo deny check bans` and verify it passes                                                | `bans ok`, exit code 0, zero errors                          |
| T4  | DONE   | Add `cargo deny check bans` to CI testing workflow (GitHub Actions)                             | Added to `testing.yaml` and `copilot-setup-steps.yml`        |
| T5  | DONE   | Add `cargo deny check bans` to pre-commit (fast) or pre-push (slow), per ongoing #1843 decision | Added to pre-commit (0.57s runtime — fast enough)            |
| T6  | DONE   | Verify that adding a test `core -> server` dep triggers a deny error                            | Verified: `http-core -> udp-server` triggers `error[banned]` |
| T7  | DONE   | Document the `deny.toml` configuration in `packages/AGENTS.md`                                  | Step 7 in "Adding or Modifying a Package" section            |

## Progress Tracking

### Workflow Checkpoints

- [ ] Spec drafted in `docs/issues/drafts/`
- [ ] Spec reviewed and approved by user/maintainer
- [ ] GitHub issue created and issue number added to this spec
- [ ] Spec moved to `docs/issues/open/` with issue number prefix
- [x] Implementation completed
- [x] Automatic verification completed (`cargo deny check bans` ✓, `linter all` ✓, `cargo test --workspace` ✓)
- [ ] Manual verification scenarios executed and recorded
- [x] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] EPIC #1669 Active Subissues table updated to `DONE`
- [ ] Issue closed and spec moved to `docs/issues/closed/`

### Progress Log

- 2026-06-11 00:00 UTC - josecelano - Spec drafted as subissue of EPIC #1669
- 2026-06-22 16:34 UTC - josecelano - Implementation completed on branch `1925-configure-cargo-deny-for-layer-boundary-enforcement`

## Acceptance Criteria

- [x] `deny.toml` exists at the workspace root with bans configuration.
- [x] `cargo deny check bans` passes (exit code 0) on the current workspace state.
- [x] Adding a forbidden dependency edge (e.g., `core -> server`) causes `cargo deny check bans` to fail.
- [x] CI (GitHub Actions testing workflow) runs `cargo deny check bans` and rejects changes with new banned edges.
- [x] The pre-commit hook runs `cargo deny check bans` (0.57s runtime).
- [x] `packages/AGENTS.md` references the `deny.toml` enforcement in its Adding/Modifying a Package section.

## Verification Plan

### Automatic Checks

- `cargo deny check bans`
- `cargo build --workspace` (ensure no build breakage)
- `linter all` (ensure linters still pass)

### Manual Verification Scenarios

| ID  | Scenario                                | Command / Steps                                                                                               | Expected Result                   | Status |
| --- | --------------------------------------- | ------------------------------------------------------------------------------------------------------------- | --------------------------------- | ------ |
| M1  | Baseline pass on current workspace      | `cargo deny check bans`                                                                                       | Exit code 0                       | PASS   |
| M2  | Forbidden edge detected                 | Temporarily add `torrust-tracker-udp-server` to `packages/http-core/Cargo.toml`, then `cargo deny check bans` | `error[banned]` and `bans FAILED` | PASS   |
| M3  | Legitimate edge allowed                 | No action needed — current legitimate edges (e.g., `axum-rest-api-server -> udp-server`) pass                 | No errors on those edges          | PASS   |
| M4  | Pre-commit hooks pass after adding deny | `./contrib/dev-tools/git/hooks/pre-commit.sh`                                                                 | Exit code 0                       | PASS   |
