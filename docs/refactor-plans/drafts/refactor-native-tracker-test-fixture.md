---
doc-type: refactor-plan
status: draft
related-issue: 2238
spec-path: docs/refactor-plans/drafts/refactor-native-tracker-test-fixture.md
last-updated-utc: 2026-09-16 15:36
semantic-links:
  skill-links:
    - create-refactor-plan
  related-artifacts:
    - .github/skills/dev/planning/create-refactor-plan/SKILL.md
    - docs/issues/open/2238-refactor-native-tracker-test-fixture/ISSUE.md
    - tests/common/native_tracker.rs
    - tests/lifecycle/signals.rs
    - tests/configuration/cli_configuration.rs
---

<!-- skill-link: create-refactor-plan -->

# Refactor Plan - Native Tracker Test Fixture

## Goal

Make the native tracker executable fixture easier and safer to understand, review, test, and
change. Separate its private responsibilities so a maintainer, human or AI agent, can reason
about one concern with a bounded context: the owning module, its colocated tests, and at most one
or two named collaborators. Improve module paths, names, and the test-facing API when doing so
makes the fixture more expressive; retain only the executable behavior and process-cleanup
contracts protected by the tests.

Reducing line count is a consequence, not the success criterion. The split succeeds when:

- the normal and failed-start lifecycle paths are each traceable within one module;
- every child process, output reader, temporary workspace, permission guard, and deadline has one
  visible owner;
- each module opens with a short doc comment stating what it owns and what it must not do, so the
  ownership model lives in Git-tracked code rather than in anyone's working memory;
- a common maintenance task maps to one primary module (see the task map below); and
- both consumers clearly express their scenarios through the resulting fixture API and continue
  to protect the same observable behavior.

Related artifact:
[`docs/issues/open/2238-refactor-native-tracker-test-fixture/ISSUE.md`](../../issues/open/2238-refactor-native-tracker-test-fixture/ISSUE.md)

## Design Constraints Discovered in the Current Code

These constraints were verified against `tests/common/native_tracker.rs` and its two consumers
and must be honored by the implementation.

### Module resolution under `#[path]`

Both binaries include the fixture directly, not through `tests/common/mod.rs`:

```rust
#[path = "../common/native_tracker.rs"]
mod native_tracker;
```

A file loaded through `#[path]` owns its directory for nested module resolution. A bare
`mod failed_start;` inside `tests/common/native_tracker.rs` therefore resolves to
`tests/common/failed_start.rs`, not to `tests/common/native_tracker/failed_start.rs`. Choose one:

1. **Recommended**: move the root to `tests/common/native_tracker/mod.rs` and update the two
   `#[path]` attributes to `"../common/native_tracker/mod.rs"`. Children then use standard bare
   `mod` declarations and live beside their root.
2. Keep `tests/common/native_tracker.rs` and give every child an explicit
   `#[path = "native_tracker/<name>.rs"]` attribute.

Record the choice in the issue progress log. Do not rely on an unverified assumption that children
resolve into a same-named subdirectory.

### Two-binary compilation and dead-code allowances

The file carries 21 `#[allow(dead_code)]` attributes because `lifecycle-signals` compiles the whole
fixture but uses only the running-tracker API, while `cli-configuration` already allows dead code on
its entire `mod native_tracker;` declaration. With `-D unused` in `.cargo/config.toml`, an unused
`pub(super)` item is a compile error, not a warning.

Policy after the split:

- Where an entire child module is unused by one binary (`failed_start`), replace item-level
  allowances with one module-level `#[allow(dead_code)]` on its `mod` declaration, with a one-line
  comment naming the binary that does not use it.
- Keep item-level allowances where only a few items in a module are unused by one binary (the three
  environment/override builders on `NativeTrackerConfigurationSources`).
- Do not add allowances that would hide genuinely dead code introduced by the refactor.

### Naming collisions with sibling helpers

`tests/common/workspace.rs` and `tests/common/configuration.rs` already exist for the in-process
`TrackerApplicationFixture`. Do not name native fixture children `workspace.rs` or
`configuration.rs`; use `command.rs` for the module that materializes a child workspace and builds
its `Command`.

### Colocated unit tests are required, not optional

Existing unit tests construct private structs directly, for example
`NativeTrackerPermissionRestore { path, mode }` and a `NativeTrackerFailedStart` literal with all
fields. These tests must move into the module that defines those structs; they cannot remain in a
root `tests` module without widening visibility.

### Internal API changes are allowed

The module path, public fixture types, method names, arguments, and return types are internal test
support and may change. Rename, narrow, merge, or remove them when the result improves clarity,
modularity, or resource ownership. Update both consumers in the same item as the API change.

## Current Consumer Surface

This inventory prevents accidental omissions; it is not a compatibility contract. Each item must
either remain because it is the clearest resulting API, change together with its consumers, or be
removed as unused.

| Type | Public methods |
| --- | --- |
| `NativeTracker` | `start`, `start_with_configuration_sources`, `wait_until_ready`, `pid`, `health_check_address`, `configuration_path`, `storage_path`, `shutdown`, `gracefully_shutdown`, `take_drop_cleanup_observer` |
| `NativeTrackerConfigurationSources` | `with_cli_health_check_port`, `with_environment_path_health_check_port`, `with_environment_toml_health_check_port`, `with_health_check_api_bind_address_override` |
| `NativeTrackerInvalidCliSource` | six variants |
| `NativeTrackerStartAttempt` | `with_invalid_cli_source`, `with_unreadable_regular_file`, `start` |
| `NativeTrackerUnreadableCliSource` | `enforced_or_report_skip` |
| `NativeTrackerFailedStart` | `source_path`, `wait_for_exit` |
| `NativeTrackerFailedStartResult` | `exit_code`, `output`, `source_path`, `source_mode`, `assert_usage_error`, `assert_startup_failure`, `assert_diagnostic_names_source_path`, `assert_explicit_configuration_file_load_failure` |

## Responsibility Map

| Module | Owns | Must not |
| --- | --- | --- |
| root (`mod.rs`) | `NativeTracker`; startup, readiness loop, `shutdown`, `gracefully_shutdown`, drop-path kill/reap; `STARTUP_DEADLINE`, `SHUTDOWN_DEADLINE`, `RETRY_INTERVAL`, `SIGNAL_HANDLERS_READY_MESSAGE`; public re-exports | contain failed-start, rendering, or probe implementation |
| `failed_start.rs` | `NativeTrackerInvalidCliSource`, `NativeTrackerUnreadableCliSource`, `NativeTrackerStartAttempt`, `NativeTrackerFailedStart`, `NativeTrackerFailedStartResult`, `NativeTrackerPermissionRestore`, `invalid_source_command`, `FAILURE_DEADLINE`, usage/startup exit-code constants | touch the running-tracker readiness loop |
| `command.rs` | `CONFIGURATION`, `NativeTrackerConfigurationSources`, `NativeTrackerWorkspace`, `write_configuration`, `write_configuration_in_directory`, `tracker_command`, `configure_tracker_command`, `tracker_binary` | know about invalid-source cases or readiness |
| `output.rs` | `TrackerOutputCapture`, `drain_output`; invariant: both reader tasks are joined before contents are treated as final diagnostics | parse or interpret output |
| `health.rs` | `HealthCheckClient`, `HealthCheckProbe`, `HealthCheckProbeError`, `parse_health_check_address`, `HEALTH_CHECK_*` constants | own any process, task, or file resource |

`output.rs` and `health.rs` are separated because one owns asynchronous reader tasks (a resource
with a lifetime invariant) and the other is a stateless client plus a parser. If `health.rs` proves
trivially small during implementation, merging it into `output.rs` is acceptable; record the
decision.

Approximate resulting sizes, including colocated tests: root ~320 lines, `failed_start.rs` ~520,
`command.rs` ~280, `output.rs` ~50, `health.rs` ~95.

## Maintenance Task Map

This table is the concrete check for the bounded-context goal. Each row must hold after the
refactor and is verified by manual scenario M3 in the issue specification.

| Maintenance task | Primary module to open | Collaborators | Focused check |
| --- | --- | --- | --- |
| Add an invalid CLI source case | `failed_start.rs` | `tests/configuration/cli_configuration/invalid_sources.rs`; `command.rs` only if a new rendering helper is needed | `cargo test --test cli-configuration` |
| Change the rendered fixture TOML or child environment isolation | `command.rs` | none | both binaries |
| Change what "ready" means (health status, signal-handler log line) | root | `health.rs` for probe or parsing changes | both binaries |
| Change how child output is captured or joined | `output.rs` | none | both binaries |
| Change a running-tracker deadline or drop-path policy | root | none | `cargo test --test lifecycle-signals --test cli-configuration` |
| Change failed-start reaping, permission restoration, or its drop fallback | `failed_start.rs` | `output.rs` for reader joining | `cargo test --test cli-configuration` |

## Items

Items are ordered dependency-first so every step is a compiling, behavior-preserving move: shared
leaf collaborators (`output`, `health`, `command`) move before the module that depends on them
(`failed_start`), and the root is trimmed last. Child modules can access the root's private items,
so any order compiles; leaf-first keeps each diff a pure move with `pub(super)` visibility.

### 1. [ ] Establish ownership, behavior, and consumer baselines [High impact / Low effort]

**Problem**: A mechanical module split can compile while obscuring or changing which type owns a
child, output reader, workspace, permission guard, or cleanup deadline. API changes can also make
the consumers less expressive when they are not reviewed together with the fixture.

**Files**:

- `tests/common/native_tracker.rs`
- `tests/lifecycle/signals.rs`
- `tests/configuration/cli_configuration.rs`

**Change**: Confirm the current-consumer and responsibility tables above against the current code.
Run both binaries once to record a green behavioral baseline. Use the inventory to ensure every API
change is intentional and updates its consumers, not to preserve current signatures.

---

### 2. [ ] Establish the module root and resolve the `#[path]` layout decision [Medium impact / Low effort]

**Problem**: The two binaries include the fixture by path, and `#[path]`-loaded files own their
directory for child resolution. Without an explicit decision, children land in the wrong
directory or fail to compile.

**Files**:

- `tests/common/native_tracker.rs` (moves to `tests/common/native_tracker/mod.rs` under option 1)
- `tests/lifecycle/signals.rs`
- `tests/configuration/cli_configuration.rs`
- `docs/issues/open/2238-refactor-native-tracker-test-fixture/ISSUE.md`
- this refactor plan

**Change**: Select the layout that produces the clearest module ownership and consumer imports.
The recommended option remains a `tests/common/native_tracker/mod.rs` root, but narrower direct
imports are allowed if they better separate running and failed-start fixtures. Update both
consumers and every live documentation path or related-artifact entry that names the old root;
preserve historical progress-log paths that accurately describe the earlier state. Run both
binaries.

---

### 3. [ ] Extract output capture and health probing [Medium impact / Low effort]

**Problem**: `TrackerOutputCapture`, `drain_output`, `HealthCheckClient`, the probe enums, and
`parse_health_check_address` are leaf collaborators with no dependency on the rest of the fixture,
yet they sit between lifecycle and failed-start code.

**Files**:

- `tests/common/native_tracker/output.rs`
- `tests/common/native_tracker/health.rs`
- root module

**Change**: Move the items per the responsibility map with `pub(super)` visibility. Move the
`parse_health_check_address` tests into `health.rs`. Add a `//!` doc to each new module stating
its responsibility and, for `output.rs`, the reader-joining invariant.

---

### 4. [ ] Extract workspace and shared command construction [Medium impact / Low effort]

**Problem**: Configuration rendering, temporary storage creation, environment isolation, and
binary lookup are one construction concern shared by normal and failed starts, currently
interleaved with process-ownership logic.

**Files**:

- `tests/common/native_tracker/command.rs`
- root module

**Change**: Move `CONFIGURATION`, `NativeTrackerConfigurationSources`, `NativeTrackerWorkspace`,
`write_configuration`, `write_configuration_in_directory`, `tracker_command`,
`configure_tracker_command`, and `tracker_binary`. Re-export `NativeTrackerConfigurationSources`
from the root. Move the `tracker_command` and `write_configuration` tests. Keep the three item-level
dead-code allowances on the environment/override builders. Add a `//!` doc stating that the module
owns workspace/configuration materialization and shared command construction but does not own child
processes or lifecycle decisions.

---

### 5. [ ] Extract failed-start and invalid-source behavior [High impact / Medium effort]

**Problem**: Invalid CLI source construction, failed-start results and assertions, Unix mode
restoration, deadline-bounded reaping, and the no-runtime drop fallback form one distinct fixture
used only by `cli-configuration`, yet they interrupt the running-tracker flow and account for most
of the item-level dead-code allowances.

**Files**:

- `tests/common/native_tracker/failed_start.rs`
- root module
- `tests/configuration/cli_configuration/invalid_sources.rs`

**Change**: Move the items per the responsibility map, including `invalid_source_command` and
`FAILURE_DEADLINE`. Expose only the surface needed by the updated invalid-source consumer; rename
or simplify that surface where it improves expression. If `lifecycle-signals` no longer imports
this module, remove its dead-code allowances entirely; otherwise use one justified module-level
allowance. Move the permission-restore, failed-start drop, unreadable-source, and
parent-only-relative-source tests. Add a `//!` doc stating the normal (`wait_for_exit`) versus
best-effort (`Drop`) cleanup contract.

---

### 6. [ ] Trim the root to lifecycle orchestration and document ownership [Medium impact / Low effort]

**Problem**: After extraction the root still needs to read as one coherent lifecycle: spawn,
readiness, shutdown, drop. Constants and re-exports need a deliberate order, and the module doc
should state the ownership model that reviewers and agents rely on.

**Files**:

- root module

**Change**: Order the root as: module doc, child `mod` declarations, `pub use` re-exports,
lifecycle constants, `NativeTracker` struct, `impl NativeTracker`, `impl Drop`. Extend the `//!` doc
with a short ownership summary matching the responsibility map and state that the root must not
contain failed-start, rendering, or probe implementation. No behavior changes.

---

### 7. [ ] Simplify the test-facing API and update consumers [High impact / Medium effort]

**Problem**: A mechanically preserved facade can carry names, re-exports, and methods that made
sense only when all responsibilities lived in one file. Internal compatibility has no value when
it leaves the tests harder to read or keeps unrelated fixtures coupled.

**Files**:

- all modules under `tests/common/native_tracker/`
- `tests/lifecycle/signals.rs`
- `tests/configuration/cli_configuration.rs`
- `tests/configuration/cli_configuration/*.rs`

**Change**: Review every item in the current consumer surface. Rename, narrow, merge, or remove
items where that improves scenario expression, ownership, or module independence. Prefer each
consumer importing only its relevant fixture surface. Keep Act and typed assertions visible in
the tests. Run both binaries after each coherent API change.

---

### 8. [ ] Optional: consolidate duplicated configuration rendering [Low impact / Trivial effort]

**Problem**: `write_configuration` and `write_configuration_in_directory` render `CONFIGURATION`
with identical placeholder replacement. The duplication is only obvious once both sit together in
`command.rs`.

**Files**:

- `tests/common/native_tracker/command.rs`

**Change**: Have one helper delegate to the other. Do this after the move and API cleanup commits,
so the move-only diff stays reviewable. Skip if the result is not clearly simpler.

---

### 9. [ ] Reconcile ownership, tests, and documentation [Medium impact / Low effort]

**Problem**: Pure moves can still leave hidden coupling, a widened `pub` item, or a redundant
allowance that hides real dead code. Any optional cleanup must also receive final validation.

**Files**:

- all modules under `tests/common/native_tracker/`
- `docs/issues/open/2238-refactor-native-tracker-test-fixture/ISSUE.md`

**Change**: Review extraction commits with `git diff --color-moved=dimmed-zebra` so non-move edits
stand out, and review API cleanup commits separately. Confirm every exposed item has a current
consumer and the narrowest practical visibility. Walk the maintenance task map. Run both binaries
and `linter all`. Record evidence in the issue specification.

## Order of Execution

| Order | Status | Item | Impact | Effort |
| --- | --- | --- | --- | --- |
| 1 | [ ] | Establish ownership, behavior, and consumer baselines | High | Low |
| 2 | [ ] | Establish the module root and resolve the `#[path]` layout decision | Medium | Low |
| 3 | [ ] | Extract output capture and health probing | Medium | Low |
| 4 | [ ] | Extract workspace and shared command construction | Medium | Low |
| 5 | [ ] | Extract failed-start and invalid-source behavior | High | Medium |
| 6 | [ ] | Trim the root to lifecycle orchestration and document ownership | Medium | Low |
| 7 | [ ] | Simplify the test-facing API and update consumers | High | Medium |
| 8 | [ ] | Optional: consolidate duplicated configuration rendering | Low | Trivial |
| 9 | [ ] | Reconcile ownership, tests, and documentation | Medium | Low |

## Commit Points

These are the only implementation commit points. The issue specification tracks documentation
lifecycle commits and does not repeat this table.

| Items | Coherent change set | Commit policy |
| --- | --- | --- |
| 1-2 | Baseline plus chosen module layout | Commit after both binaries pass with the chosen imports. |
| 3 | Output and health extraction with colocated tests | Keep as a pure move; commit after both binaries pass. |
| 4 | Command construction extraction with colocated tests | Keep as a pure move; commit after both binaries pass. |
| 5 | Failed-start extraction with colocated tests | Keep as a pure move; commit after both binaries pass. |
| 6 | Root lifecycle ordering and ownership docs | Commit after both binaries pass. |
| 7 | Test-facing API cleanup and consumer updates | Commit each coherent API improvement with both affected sides and focused tests. |
| 8 | Optional rendering deduplication | Separate commit only when the result is clearly simpler. |
| 9 | Final reconciliation and verification evidence | Commit after the maintenance task map, both binaries, and `linter all` pass. |
