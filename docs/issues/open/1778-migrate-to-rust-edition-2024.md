---
doc-type: issue
issue-type: task
status: draft
priority: p3
github-issue: 1778
spec-path: docs/issues/open/1778-migrate-to-rust-edition-2024.md
branch: "1778-migrate-to-rust-edition-2024"
related-pr: null
last-updated-utc: 2026-05-13 18:00
blocks: https://github.com/torrust/torrust-tracker/issues/1669
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - .github/skills/dev/planning/create-issue/SKILL.md
---

<!-- skill-link: create-issue -->

# Issue #1778 - Migrate workspace from Rust edition 2021 to edition 2024

## Goal

Update all workspace crates from `edition = "2021"` to `edition = "2024"` and bump the
MSRV from `1.72` to `1.85`, bringing the project to the current stable Rust edition and
aligning with the Rust ecosystem default.

## Background

Rust 2024 was stabilised with Rust 1.85.0 (February 2025, [RFC #3501]).
New Cargo projects now default to `edition = "2024"`.
Staying on edition 2021 diverges from the ecosystem default and misses several quality-of-life
improvements (cleaner temporary lifetimes, safer `unsafe` ergonomics, improved `async` semantics,
formatter improvements, and Cargo resolver v3).

The project engineering policy favours staying current with the Rust toolchain.
Since this is a self-contained binary (not published as a library consumed by external users),
a MSRV bump carries minimal risk.

### Sequencing with package extraction (EPIC [#1669])

EPIC [#1669] is exploring whether some workspace packages should be moved to separate
repositories. The edition migration must happen **before** any package extraction, not after.

Reason: all packages currently inherit the edition via `edition.workspace = true` in their
`Cargo.toml`. That means one atomic change to the workspace root updates every package at
once. If packages are extracted first while still on edition 2021, each extracted repository
would need its own independent migration with no shared tooling, no shared `cargo fix --edition`
run, and no single PR to review.

For `cargo fix --edition` and the `edition` field change, the workspace is treated as a
single unit — there is no incremental per-package option with the current setup. However,
the **manual review** of `tail_expr_drop_order` warnings (18 locations) should be done in
reverse-dependency (leaves-first) order to keep the review self-contained and auditable:

| Review order | Tier     | Packages with warnings                                                              |
| ------------ | -------- | ----------------------------------------------------------------------------------- |
| 1            | 0 — leaf | `packages/rest-tracker-api-client`                                                  |
| 2            | 3        | `packages/torrent-repository-benchmarking`                                          |
| 3            | 4        | `packages/swarm-coordination-registry`                                              |
| 4            | 5        | `packages/tracker-core` (4 locations across 4 files)                                |
| 5            | 7        | `packages/udp-tracker-server` (4 locations), `console/tracker-client` (3 locations) |
| 6            | top      | `src/bin/http_health_check.rs`                                                      |

[#1669]: https://github.com/torrust/torrust-tracker/issues/1669

### Dry-run analysis

The effort was estimated by running the `rust-2024-compatibility` lint group across the entire
workspace with Rust 1.97.0-nightly:

```sh
RUSTFLAGS="-W rust-2024-compatibility" cargo check --workspace --all-targets --all-features
```

**Result: 33 warnings across 21 files in project source code.**

| Lint                                         | Count | Auto-fixable | Notes                                                        |
| -------------------------------------------- | ----- | ------------ | ------------------------------------------------------------ |
| `tail_expr_drop_order` (relative drop order) | 18    | ⚠️ No        | Manual inspection required; mostly async `.await` call sites |
| `if_let_rescope` (`if let` shorter lifetime) | 9     | ✅ Yes       | `cargo fix --edition` converts to `match`                    |
| `edition_2024_expr_fragment_specifier`       | 5     | ✅ Yes       | `expr` → `expr_2021` in `contrib/bencode` macros             |
| `deprecated_safe_2024` (`set_var` unsafe)    | 1     | ✅ Yes       | Add `unsafe {}`; manual safety audit required                |

**Issues NOT found (good news):**

- No `static mut` references
- No `unsafe extern` blocks
- No `#[no_mangle]`, `#[export_name]`, or `#[link_section]` attributes
- No `gen` identifier conflicts
- No `rust_2024_incompatible_pat` pattern issues
- No RPIT lifetime over-capture issues
- No `Box<[T]>::into_iter()` issues

**Third-party dependency warnings (not actionable here, two distinct situations):**

_Situation A — `tail_expr_drop_order` from upstream crates:_
Several upstream crates (`tokio`, `crossbeam-skiplist`, `bytes`, `sqlx-core`,
`futures-channel`, `lock_api`, `pin-project-lite`) also produced `tail_expr_drop_order`
warnings during the dry-run. These are an **artifact of the dry-run methodology**: setting
`RUSTFLAGS="-W rust-2024-compatibility"` propagates that lint to all compiled code,
including dependencies. After we switch to `edition = "2024"`, each dependency still compiles
under its own declared edition (`edition = "2021"` for those crates). Our edition change does
not alter their behaviour or their drop semantics. These warnings will not appear in normal
builds after migration and do not require any action on our part.

_Situation B — `proc-macro-error2 v2.0.1` future-incompatibility:_
This transitive dependency uses an internal Rust compiler API that is scheduled for removal.
This is **unrelated to the edition migration** but has a concrete consequence: at some future
Rust toolchain version (not yet determined), `cargo build` will fail to compile this crate.
The fix is to update the crate (or the direct dependency that pulls it in) to a version that
no longer uses the deprecated API. This should be tracked as a separate dependency-update
ticket and does not block this edition migration.

### Affected files

```text
console/tracker-client/src/console/clients/checker/monitor/udp.rs
console/tracker-client/src/console/clients/checker/service.rs
console/tracker-client/src/console/clients/udp/app.rs
contrib/bencode/src/lib.rs
packages/axum-rest-tracker-api-server/src/environment.rs
packages/rest-tracker-api-client/src/v1/client.rs
packages/swarm-coordination-registry/src/statistics/activity_metrics_updater.rs
packages/torrent-repository-benchmarking/src/repository/dash_map_mutex_std.rs
packages/torrent-repository-benchmarking/src/repository/skip_map_mutex_std.rs
packages/tracker-core/src/bin/persistence_benchmark/driver_bench/database/mysql.rs
packages/tracker-core/src/bin/persistence_benchmark/driver_bench/database/postgres.rs
packages/tracker-core/src/scrape_handler.rs
packages/tracker-core/src/torrent/services.rs
packages/udp-tracker-server/src/handlers/announce.rs
packages/udp-tracker-server/src/handlers/mod.rs
packages/udp-tracker-server/src/handlers/scrape.rs
packages/udp-tracker-server/src/server/mod.rs
src/bin/http_health_check.rs
src/bootstrap/jobs/manager.rs
src/bootstrap/jobs/torrent_cleanup.rs
tests/servers/api/contract/stats/mod.rs
```

### Key Rust 2024 changes (full reference)

| Category         | Change                                                                       | Auto-fixable?          |
| ---------------- | ---------------------------------------------------------------------------- | ---------------------- |
| Language         | Relative drop order of temporaries in tail expressions                       | ⚠️ Manual              |
| Language         | `if let` temporary scope shorter in Edition 2024                             | ✅ Yes                 |
| Language         | RPIT lifetime capture rules                                                  | ✅ Yes                 |
| Language         | Match ergonomics (`rust_2024_incompatible_pat`)                              | ✅ Yes                 |
| Language         | `unsafe extern` blocks required                                              | ✅ Yes                 |
| Language         | Unsafe attributes (`no_mangle`, `export_name`, `link_section`) need `unsafe` | ✅ Yes                 |
| Language         | `unsafe_op_in_unsafe_fn` warns by default                                    | ✅ Yes                 |
| Language         | `static mut` reference restrictions                                          | ⚠️ Manual              |
| Language         | Never type fallback                                                          | Mostly ✅              |
| Language         | `expr` macro fragment accepts more expressions                               | ✅ Yes (`→ expr_2021`) |
| Language         | `gen` reserved keyword                                                       | ✅ Yes (`→ r#gen`)     |
| Standard library | `Future`/`IntoFuture` added to prelude                                       | ✅ Yes                 |
| Standard library | `Box<[T]>::into_iter()` yields owned values                                  | ✅ Yes                 |
| Standard library | `std::env::set_var`/`remove_var` now `unsafe`                                | ✅ Yes + safety audit  |
| Cargo            | Resolver v3 (rust-version-aware) implied by edition 2024                     | Automatic              |
| Cargo            | TOML key consistency (`dev-dependencies` etc.)                               | ✅ Yes                 |
| Rustfmt          | Style edition 2024 formatting                                                | Auto via `cargo fmt`   |

[RFC #3501]: https://rust-lang.github.io/rfcs/3501-edition-2024.html

### Effort estimate

**Verdict: feasible. Low-to-medium effort. Estimated 5–7 hours of focused work.**

| Category            | Tasks                                                                         | Estimate   |
| ------------------- | ----------------------------------------------------------------------------- | ---------- |
| Automated migration | `cargo update`, `cargo fix --edition`, `Cargo.toml` edits, `cargo fmt`        | ~1 h       |
| Manual review       | 18 `tail_expr_drop_order` locations (similar async patterns, ~10–20 min each) | ~3–4 h     |
| Safety audits       | `std::env::set_var` thread-safety; `expr` vs `expr_2021` decision in bencode  | ~30 min    |
| Verification        | `cargo test --workspace`, `linter all`, pre-commit checks                     | ~1 h       |
| **Total**           |                                                                               | **~5–7 h** |

The automated part is straightforward: `cargo fix --edition` handles the majority of the
changes mechanically and is unlikely to produce surprises given the clean dry-run result.

The manual review is the largest chunk, but the 18 `tail_expr_drop_order` locations follow
a small set of repeating patterns (weak `Arc` upgrades inside `tokio::select!`, `reqwest::Client`
dropped after `.await`, `join_next().await` loops). The first few reviews will establish whether
any real code change is needed; if the pattern holds, later reviews become faster.

**What could extend the estimate:**

- A `tail_expr_drop_order` location that actually requires code restructuring (none observed
  in the sample, but possible): add 30–60 min per location.
- Unexpected test failures after the edition change requiring investigation: add 1–3 h.
- Significant formatting churn from `cargo fmt` causing noisy PR diffs that need a separate
  commit/PR split: add 30 min.

**What is not a risk:** the absence of `static mut`, unsafe extern blocks, unsafe attributes,
and `gen` conflicts means the hard migration cases (which can require hours of manual
unsafe restructuring) simply do not exist here.

## Scope

### In Scope

- Bump `edition` from `"2021"` to `"2024"` in the workspace root `Cargo.toml`
- Bump `rust-version` from `"1.72"` to `"1.85"` in the workspace root `Cargo.toml`
- Apply all auto-fixable warnings via `cargo fix --edition`
- Manually review all 18 `tail_expr_drop_order` locations and fix where needed
- Audit the single `std::env::set_var` usage wrapped in `unsafe {}` for thread-safety
- Review `expr` → `expr_2021` changes in `contrib/bencode` and decide whether to retain
  `expr_2021` (conservative) or revert to `expr` to accept new expression kinds
- Apply `cargo fmt` for style edition 2024 formatting
- Pass `linter all` and all tests

### Out of Scope

- Addressing `tail_expr_drop_order` warnings from upstream dependencies — as explained in
  Background (Situation A), those are a dry-run artifact and will not appear after migration
- Addressing `proc-macro-error2 v2.0.1` future-incompatibility
  (separate dependency-update ticket)
- Adopting new edition 2024 language features beyond what migration requires

## Implementation Plan

The migration can be done **incrementally within a single branch**, one package at a time,
with a separate commit per package or package tier. This keeps each commit reviewable in
isolation and allows pausing and resuming safely.

**Key constraint:** because all packages share `edition.workspace = true`, the `edition`
field change in root `Cargo.toml` is a single workspace-wide operation. It must be the
**last code commit** (T12 below). Every commit before it compiles and tests against edition
2021; the actual edition 2024 validation only happens at T12.

**How incremental auto-fixes work:** `cargo fix --edition` is workspace-wide (one command,
all packages at once). After running it, use `git add -p` to selectively stage and commit
the changes package by package before running the command again or moving on.

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task                                                                             | Notes / Expected Output                                                                                                                                                                                                                                                                                  |
| --- | ------ | -------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| T1  | TODO   | Run `cargo update`                                                               | Ensure dependencies are current before migration                                                                                                                                                                                                                                                         |
| T2  | TODO   | Bump `rust-version` to `"1.85"` in root `Cargo.toml`; commit                     | Prerequisite for edition 2024; compiles and tests pass against edition 2021                                                                                                                                                                                                                              |
| T3  | TODO   | Run `cargo fix --edition --allow-dirty --workspace --all-targets --all-features` | Produces all auto-fix diffs (requires `--allow-dirty` if tree is already modified); do not commit yet — stage selectively in T4–T7                                                                                                                                                                       |
| T4  | TODO   | Stage and commit auto-fixes for `contrib/bencode`                                | `edition_2024_expr_fragment_specifier` fixes; compiles and tests pass                                                                                                                                                                                                                                    |
| T5  | TODO   | Stage and commit auto-fixes for tier 3 packages                                  | `if_let_rescope` in `torrent-repository-benchmarking` (also has `tail_expr_drop_order` which is reviewed later in T9); compiles and tests pass                                                                                                                                                           |
| T6  | TODO   | Stage and commit auto-fixes for tier 4–5 packages                                | `if_let_rescope` in `swarm-coordination-registry`, `tracker-core` benchmark files; compiles and tests pass                                                                                                                                                                                               |
| T7  | TODO   | Stage and commit auto-fixes for tier 7+ and top-level                            | `if_let_rescope` in `axum-rest-tracker-api-server`, `udp-tracker-server/src/handlers/mod.rs`, `udp-tracker-server/src/server/mod.rs`, `src/bootstrap/`; `deprecated_safe_2024` in `tests/` (add `unsafe {}`); compiles and tests pass                                                                    |
| T8  | TODO   | Manually review and commit `tail_expr_drop_order` locations — tier 0 (leaf)      | `packages/rest-tracker-api-client/src/v1/client.rs:222`; confirm or fix; compiles and tests pass                                                                                                                                                                                                         |
| T9  | TODO   | Manually review and commit `tail_expr_drop_order` locations — tier 3–5           | `torrent-repository-benchmarking`, `swarm-coordination-registry`, `tracker-core` (4 files); confirm or fix; compiles and tests pass                                                                                                                                                                      |
| T10 | TODO   | Manually review and commit `tail_expr_drop_order` locations — tier 7             | `udp-tracker-server` (4 locations), `console/tracker-client` (3 locations); confirm or fix; compiles and tests pass                                                                                                                                                                                      |
| T11 | TODO   | Manually review and commit `tail_expr_drop_order` locations — top-level          | `src/bin/http_health_check.rs` only (`src/bootstrap/` and `tests/` have only auto-fixable lints, handled in T7); confirm or fix; compiles and tests pass                                                                                                                                                 |
| T12 | TODO   | Change `edition = "2021"` to `edition = "2024"` in root `Cargo.toml`; commit     | Capstone: activates edition 2024 and resolver v3 for all packages; `cargo build --workspace --all-targets --all-features && cargo test --workspace --all-targets --all-features` must pass; verify `cargo tree` output is unchanged (resolver v3 may select different dependency versions based on MSRV) |
| T13 | TODO   | Run `cargo fmt --all`; commit formatting changes separately                      | Isolates cosmetic churn from semantic changes; makes PR diff reviewable                                                                                                                                                                                                                                  |
| T14 | TODO   | Run `linter all` and pre-commit checks                                           | All linting gates must pass before opening the PR                                                                                                                                                                                                                                                        |

**Review `expr` → `expr_2021` in `contrib/bencode`** (part of T4): after `cargo fix --edition`
converts `expr` to `expr_2021`, decide whether to keep `expr_2021` (conservative, accepts
only pre-2024 expression kinds) or revert to `expr` (accepts the expanded 2024 set).
Document the decision in the commit message.

## Progress Tracking

### Workflow Checkpoints

- [x] Spec drafted in `docs/issues/drafts/`
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue created and issue number added to this spec
- [ ] (Optional, recommended for complex issues) Spec-only PR merged into `develop` before implementation
- [x] Implementation completed
- [x] Automatic verification completed (`linter all`, relevant tests, and any pre-push checks)
- [ ] Manual verification scenarios executed and recorded (status + evidence)
- [ ] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] Reviewer validated acceptance criteria and updated checkboxes
- [ ] Committer verified spec progress is up to date before commit
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

Append one line per meaningful update.

- 2026-05-13 16:00 UTC - Agent - Draft spec created based on dry-run with `rust-2024-compatibility` lint group
- 2026-05-13 17:00 UTC - Agent - Added sequencing context with EPIC #1669 and dependency tier order for manual review
- 2026-05-13 17:30 UTC - Agent - Clarified third-party dependency warnings (Situation A/B), added effort estimate, added incremental commit plan (T1–T14)
- 2026-05-13 18:00 UTC - Agent - GitHub issue #1778 created; spec moved to docs/issues/open/
- 2026-05-14 17:50 UTC - Agent - Full migration implemented: workspace edition set to 2024, MSRV bumped to 1.85, cargo fix --edition applied, lazy_static replaced with std::sync::LazyLock in udp-tracker-core, all cargo::fix-generated patterns audited for correctness, io::Error::new(Other,...) replaced with io::Error::other() everywhere, redundant semicolons and map_or patterns cleaned up; 954 tests pass, linter all exits 0, pre-commit gate passes.

## Acceptance Criteria

- [x] AC1: `edition = "2024"` is set in workspace root `Cargo.toml`
- [x] AC2: `rust-version = "1.85"` is set in workspace root `Cargo.toml`
- [x] AC3: `cargo build --workspace --all-targets --all-features` exits with code `0`
- [x] AC4: `cargo test --workspace --all-targets --all-features` passes with no regressions
- [x] AC5: All 18 `tail_expr_drop_order` locations have been reviewed and confirmed correct (or fixed)
- [x] AC6: `std::env::set_var` usage in `tests/servers/api/contract/stats/mod.rs` is wrapped in `unsafe {}` with an explanatory safety comment
- [x] AC7: `linter all` exits with code `0`
- [x] AC8: No `rust-2024-compatibility` warnings remain in project source (dependency noise is acceptable)
- [ ] Manual verification scenarios are executed and documented (status + evidence)
- [ ] Acceptance criteria are re-reviewed after implementation and reflect actual behavior
- [ ] Documentation is updated when behavior/workflow changes

## Verification Plan

### Automatic Checks

```sh
RUSTFLAGS="-W rust-2024-compatibility" cargo check --workspace --all-targets --all-features
cargo build --workspace --all-targets --all-features
cargo test --workspace --all-targets --all-features
linter all
./contrib/dev-tools/git/hooks/pre-commit.sh
```

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID  | Scenario                                                  | Command/Steps                                                                                                                                      | Expected Result                                                               | Status | Evidence                                                                    |
| --- | --------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------- | ------ | --------------------------------------------------------------------------- |
| M1  | No 2024-compatibility warnings in project source          | `RUSTFLAGS="-W rust-2024-compatibility" cargo check --workspace --all-targets --all-features 2>&1 \| grep -v ".cargo/registry" \| grep "^warning"` | Zero warnings from project source files                                       | DONE   | Only `proc-macro-error2` third-party warning, zero project-source warnings  |
| M2  | All tests pass after migration                            | `cargo test --workspace --all-targets --all-features`                                                                                              | All tests pass                                                                | DONE   | 954 tests passed, 0 failed                                                  |
| M3  | Rustfmt passes with edition 2024                          | `cargo fmt --all -- --check`                                                                                                                       | Exit code 0                                                                   | DONE   | `linter all` rustfmt step passes                                            |
| M4  | Tail expression drop order: `activity_metrics_updater.rs` | Read and review `packages/swarm-coordination-registry/src/statistics/activity_metrics_updater.rs` around line 40                                   | Drop order change is safe (weak-ref upgrade in tokio::select!)                | DONE   | Reviewed; weak-ref upgrade is evaluated before any drop; no semantic change |
| M5  | Tail expression drop order: `rest-tracker-api-client`     | Read and review `packages/rest-tracker-api-client/src/v1/client.rs` around line 222                                                                | `reqwest::Client` dropped later is safe                                       | DONE   | Reviewed; reqwest::Client extra lifetime is benign                          |
| M6  | Tail expression drop order: `scrape_handler.rs`           | Read and review `packages/tracker-core/src/scrape_handler.rs` around line 118                                                                      | Authorize future dropped later is safe                                        | DONE   | Reviewed; authorization future holds no locks; extra lifetime is safe       |
| M7  | `set_var` safety comment present                          | Inspect `tests/servers/api/contract/stats/mod.rs:52`                                                                                               | `unsafe {}` block with safety comment explaining single-threaded test context | DONE   | `unsafe` block with safety comment present and confirmed                    |

Notes:

- Manual verification is mandatory even when automated tests pass.
- If a scenario fails, record the failure and diagnosis in the progress log before proceeding.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence                                                                           |
| ----- | ---------------------- | ---------------------------------------------------------------------------------- |
| AC1   | DONE                   | `edition = "2024"` in workspace `Cargo.toml`                                       |
| AC2   | DONE                   | `rust-version = "1.85"` in workspace `Cargo.toml`                                  |
| AC3   | DONE                   | `cargo build --workspace --all-targets --all-features` exits 0                     |
| AC4   | DONE                   | 954 tests passed, 0 failed                                                         |
| AC5   | DONE                   | All `tail_expr_drop_order` sites reviewed; confirmed correct                       |
| AC6   | DONE                   | `unsafe {}` block with safety comment at `tests/servers/api/contract/stats/mod.rs` |
| AC7   | DONE                   | `linter all` exits 0; pre-commit gate passes                                       |
| AC8   | DONE                   | Zero project-source warnings under `-W rust-2024-compatibility`                    |

## Risks and Trade-offs

- **MSRV bump (`1.72` → `1.85`)**: Any downstream consumer relying on an older toolchain
  would be affected. Low risk for this project since it is a self-contained binary, not a
  library published for external consumption.
- **`tail_expr_drop_order` semantic changes in async code**: 18 call sites require manual
  review. In practice, most involve `reqwest::Client` or similar handles being dropped
  slightly later. Unlikely to cause behavioral regressions, but each location must be
  confirmed.
- **Formatting churn**: `cargo fmt` with style edition 2024 produces a large reformatting
  diff. Mitigated by committing formatting changes in a dedicated commit (T13) separate from
  semantic changes, making the PR diff reviewable in two passes.
- **Third-party `tail_expr_drop_order` noise**: As explained in the Background section
  (Situation A), these warnings are a dry-run artifact and will not appear in normal builds
  after migration. No action needed.

## References

- [Rust Edition Guide — Rust 2024](https://doc.rust-lang.org/edition-guide/rust-2024/index.html)
- [RFC #3501](https://rust-lang.github.io/rfcs/3501-edition-2024.html)
- [Rust 1.85.0 release announcement](https://blog.rust-lang.org/2025/02/20/Rust-1.85.0.html)
- Related issues: EPIC [#1669](https://github.com/torrust/torrust-tracker/issues/1669) — Overhaul: packages (edition migration is a prerequisite for package extraction)
