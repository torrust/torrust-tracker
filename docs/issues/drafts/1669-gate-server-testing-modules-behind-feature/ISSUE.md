---
schema-version: 1
doc-type: issue
issue-type: task
status: draft
priority: p3
epic: 1669
github-issue: null
spec-path: docs/issues/drafts/1669-gate-server-testing-modules-behind-feature/ISSUE.md
branch: "{issue-number}-1669-gate-server-testing-modules-behind-feature"
related-pr: null
last-updated-utc: "2026-10-07 08:44"
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - packages/axum-rest-api-server/src/testing/environment.rs
    - packages/axum-http-server/src/testing/environment.rs
    - packages/udp-server/src/testing/environment.rs
    - docs/issues/open/1669-overhaul-packages/DECISIONS.md
    - docs/issues/open/1669-overhaul-packages/workspace-coupling-report-2026-10-06.md
    - docs/issues/open/1669-overhaul-packages/EPIC.md
---

<!-- skill-link: create-issue -->

# Issue #[To be assigned] - Gate server `testing` modules behind a Cargo feature

Subissue of EPIC [#1669](../../open/1669-overhaul-packages/EPIC.md) (Overhaul: Packages).

## Goal

Stop the server packages' test environments from forcing runtime dependencies, by compiling
each public `src/testing/` module only when a `testing` Cargo feature is enabled. Dependencies
used only by a `testing` module become optional, enabled only by that feature; dependencies used
only by tests become dev dependencies.

Note: `cargo metadata` still reports an optional dependency with `kind: null` (normal), marked
`optional: true`. The coupling tool and the dependency diagram currently classify edges by kind
only, so this issue also makes them show optional edges explicitly.

## Background

DEC-13 relocated the server test environments to public `src/testing/` modules in
`axum-rest-api-server`, `axum-http-server` and `udp-server`, and accepted that these modules
compile into release builds. The 2026-10-06 workspace coupling report
([finding 2](../../open/1669-overhaul-packages/workspace-coupling-report-2026-10-06.md#2-server-testing-modules-turn-test-only-edges-into-runtime-dependencies),
issue #2446) shows what that costs:

- `axum-rest-api-server` → `udp-server`, `udp-core`, `http-core` and
  `swarm-coordination-registry` are imported only by `src/testing/environment.rs` and
  `#[cfg(test)]` code. Production code reaches UDP only through `rest-api-runtime-adapter`.
- `axum-http-server` → `swarm-coordination-registry` and `udp-server` →
  `swarm-coordination-registry` are imported only by their `src/testing/` modules and tests.

The `testing` modules are used by each package's own tests and examples
(`examples/http_only_public_tracker.rs`, `examples/udp_only_public_tracker.rs`) and by
`axum-health-check-api-server`'s tests.

## Scope

### In Scope

- Add a `testing` feature to the three server packages and gate `pub mod testing` with it.
- Make the dependencies used only by the `testing` modules optional (enabled by the feature),
  and move them to `[dev-dependencies]` where only tests use them.
- Enable the feature where it is needed: each package's own tests and examples
  (`required-features`) and `axum-health-check-api-server`'s dev-dependencies.
- Record the change as a follow-up to DEC-13 in `DECISIONS.md`.
- Make the coupling tool label optional dependencies (for example `[normal, optional]`) and
  draw optional edges distinctly (for example dashed) in the dependency diagram.
- Update the coupling report observations and the dependency diagram for the removed edges.

### Out of Scope

- Changing what the test environments do.
- The `test_helpers` module of `tracker-core` (not reported as a runtime-edge problem).
- Revisiting `deny.toml` layer rules beyond what the removed edges allow.

## Architectural Decisions

- Related decisions: DEC-13 in [`DECISIONS.md`](../../open/1669-overhaul-packages/DECISIONS.md)
- Decision to record: a DEC-13 follow-up stating that `testing` modules are feature-gated

## Design and Ownership Review

Not applicable: the test environments' ownership and behaviour do not change; only their
compilation condition does.

## Bug-Fix Process

Not applicable.

## Regression Test Strategy

Not applicable. All tests and examples that use the `testing` modules must still build and pass
with the feature enabled, and the packages must build without it.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task                                                                                 | Notes / Expected Output                                                       |
| --- | ------ | ------------------------------------------------------------------------------------ | ----------------------------------------------------------------------------- |
| T1  | TODO   | Record the DEC-13 follow-up in `DECISIONS.md`                                        | Decision entry approved by the maintainer                                     |
| T2  | TODO   | Gate `axum-rest-api-server`'s `testing` module and adjust its dependencies           | `udp-server`, `udp-core`, `http-core`, swarm registry optional (feature `testing`) or dev deps |
| T3  | TODO   | Gate `axum-http-server`'s `testing` module and adjust its dependencies               | Swarm registry optional or a dev dep, unless production code needs it        |
| T4  | TODO   | Gate `udp-server`'s `testing` module and adjust its dependencies                     | Swarm registry optional or a dev dep, unless production code needs it        |
| T5  | TODO   | Label optional dependencies in the coupling tool, draw them distinctly in the diagram, and update the report observations | Finding 2 marked resolved; optional edges marked in report and diagram; diagram matches `cargo metadata` including `optional` |

## Commit Points

| Task | Coherent change set                                         | Commit policy                                                       |
| ---- | ----------------------------------------------------------- | ------------------------------------------------------------------- |
| T1   | `DECISIONS.md` entry                                        | One commit after maintainer approval.                               |
| T2   | Feature, gate and manifest changes for `axum-rest-api-server` | One commit after its tests and the health-check tests pass.       |
| T3   | Same for `axum-http-server`                                 | One commit after its tests and examples build and pass.             |
| T4   | Same for `udp-server`                                       | One commit after its tests and examples build and pass.             |
| T5   | Tool labelling, diagram edges and report note               | One commit for the tool change with its tests, one for the docs after MV2. |

Use a Conventional Commit message with the issue reference and sign every commit with GPG.

## Progress Tracking

### Workflow Checkpoints

- [x] Spec drafted in `docs/issues/drafts/`
- [ ] Spec reviewed and approved by user/maintainer
- [ ] GitHub issue created and issue number added to this spec
- [ ] Spec moved to `docs/issues/open/` with issue number prefix
- [ ] Implementation completed
- [ ] Automatic verification completed (`linter all`, relevant tests and pre-push checks)
- [ ] Manual verification scenarios executed and recorded in issue-local
      `manual-verification-evidence.md`
- [ ] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] Evidence-based implementation completion review recorded
- [ ] Issue closed and spec moved to `docs/issues/closed/`

### Progress Log

- 2026-10-06 16:01 UTC - GitHub Copilot - Drafted from finding 2 of the 2026-10-06 workspace
  coupling report (#2446).
- 2026-10-07 08:44 UTC - GitHub Copilot - Optional dependencies stay `kind: null` in
  `cargo metadata`, so the goal, tasks, AC3 and MV2 now describe optional edges explicitly and
  AC8 requires the tool and diagram to show them, per PR #2462 review finding F4.

## Acceptance Criteria

- [ ] AC1: The DEC-13 follow-up is recorded in `DECISIONS.md`.
- [ ] AC2: Each of the three server packages compiles its `testing` module only with the
      `testing` feature.
- [ ] AC3: Every dependency of a server package that only its `testing` module uses is
      `optional: true` and enabled only by the `testing` feature; every dependency that only
      tests use is a dev dependency.
- [ ] AC4: Each server package builds without the `testing` feature, and all tests and examples
      that use the module build and pass with it.
- [ ] AC5: `cargo deny check bans` and `linter all` exit with code `0`.
- [ ] AC6: Manual verification scenarios are executed and documented in issue-local
      `manual-verification-evidence.md`.
- [ ] AC7: The acceptance criteria are re-reviewed after implementation.
- [ ] AC8: The coupling report labels optional dependencies and the dependency diagram draws
      optional edges distinctly, so feature-gated edges are not shown as plain runtime edges.

## Verification Plan

### Automatic Checks

- `cargo test --workspace --all-targets --all-features`
- `cargo build` for each server package without features
- `cargo deny check bans`, `cargo machete` and `linter all`

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID  | Scenario                           | Human-oriented command/steps                                                                        | Expected Result                                      | Status | Evidence                                      |
| --- | ---------------------------------- | --------------------------------------------------------------------------------------------------- | ---------------------------------------------------- | ------ | --------------------------------------------- |
| MV1 | Release build without test code    | `cargo build --release -p <server>` for each server, then confirm the `testing` module is excluded | Builds; no `testing` symbols in the library          | TODO   | `manual-verification-evidence.md` section MV1 |
| MV2 | Test-only edges are optional       | List the three servers' dependencies with `kind` and `optional` from `cargo metadata --no-deps`, then regenerate the coupling report | The test-only edges show `optional: true` (or dev kind), and the report labels them optional | TODO   | `manual-verification-evidence.md` section MV2 |

Create `manual-verification-evidence.md` from `docs/templates/MANUAL-VERIFICATION-EVIDENCE.md`
when executing these scenarios, and record the toolchain for each `cargo` command.

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
| AC8   | TODO                   |          |

## Risks and Trade-offs

- Consumers outside each package must enable the feature explicitly; forgetting it gives a
  compile error, not a silent change.
- More feature combinations to build in CI; `--all-features` covers the gated code.

## Implementation Completion Review

After implementation, compare the result with this specification. Record
invalidated assumptions, material design changes, unexpected validation
findings, and reusable lessons.

- Retrospective: `Not yet assessed`
- If needed, create `implementation-retrospective.md` from the repository
  template at `docs/templates/IMPLEMENTATION-RETROSPECTIVE.md` in this issue
  specification's directory.
- If no retrospective is needed, add a concise progress-log entry explaining
  why the work had no material discovery.
- When an independent reviewer receives this folder-style specification, it
  records its result in `agent-review-reports.md` using
  `docs/templates/AGENT-REVIEW-REPORTS.md`.

## References

- EPIC: [`docs/issues/open/1669-overhaul-packages/EPIC.md`](../../open/1669-overhaul-packages/EPIC.md)
- Source finding: [2026-10-06 coupling report](../../open/1669-overhaul-packages/workspace-coupling-report-2026-10-06.md)
- DEC-13: [`DECISIONS.md`](../../open/1669-overhaul-packages/DECISIONS.md)
- Baseline issue: #2446
