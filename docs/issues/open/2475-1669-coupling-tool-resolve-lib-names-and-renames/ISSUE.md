---
schema-version: 1
doc-type: issue
issue-type: bug
status: in-review
priority: p3
epic: 1669
github-issue: 2475
spec-path: docs/issues/open/2475-1669-coupling-tool-resolve-lib-names-and-renames/ISSUE.md
branch: "2475-1669-coupling-tool-resolve-lib-names-and-renames"
related-pr: 2559
last-updated-utc: "2026-10-19 12:46"
semantic-links:
  skill-links:
    - create-issue
    - fix-bug
  related-artifacts:
    - docs/issues/open/2475-1669-coupling-tool-resolve-lib-names-and-renames/manual-verification-evidence.md
    - contrib/dev-tools/analysis/workspace-coupling/src/lib.rs
    - contrib/dev-tools/analysis/workspace-coupling/src/main.rs
    - contrib/dev-tools/analysis/workspace-coupling/tests/parse_imports.rs
    - contrib/dev-tools/analysis/workspace-coupling/tests/resolve_module_name.rs
    - docs/issues/open/1669-overhaul-packages/workspace-coupling-report-2026-10-06.md
    - docs/issues/open/1669-overhaul-packages/workspace-coupling-report-2026-10-17.md
    - docs/issues/open/1669-overhaul-packages/EPIC.md
---

<!-- skill-link: create-issue -->

# Issue #2475 - Coupling tool misses renamed dependencies and custom library names

Subissue of EPIC [#1669](../../open/1669-overhaul-packages/EPIC.md) (Overhaul: Packages).

## Goal

Make `contrib/dev-tools/analysis/workspace-coupling/` find the imports of every workspace
dependency, including dependencies renamed in `Cargo.toml` and packages whose library target has
a custom name.

## Background

### What goes wrong

1. For each dependency edge `A → B`, the tool builds the Rust module name to search for from
   B's package name: `name.replace('-', "_")` in `src/main.rs`.
2. It then scans A's `src/`, `tests/` and `benches/` for `B_module::` paths.
3. Rust code does not use that name when the dependency is renamed in A's `Cargo.toml`
   (`foo = { package = "b", ... }` makes the module `foo`) or when B's library target has a
   custom name (`[lib] name = ...`).
4. The scan finds nothing and the report says "No `…::` references found in source", although
   A imports B.

This contradicts the report's own contract ("For every dependency the items actually imported
from it are listed"). Impact today: five edges show no imports, so a reader could take a used
dependency for an unused one, and thin-dependency reviews skip those edges.

### Reproduction

**Reproduced** on 2026-10-07 with the real tool; commands and output are in
[`manual-verification-evidence.md`](manual-verification-evidence.md) (R1). The five affected
edges:

| Edge                                     | Kind   | Why the module name differs                                                           |
| ---------------------------------------- | ------ | ------------------------------------------------------------------------------------- |
| `client` → `client-lib`                  | normal | Dependency renamed to `torrust-tracker-client` in `console/tracker-client/Cargo.toml` |
| `udp-server` → `client-lib`              | normal | Dependency renamed to `torrust-tracker-client` in `packages/udp-server/Cargo.toml`    |
| `test-helpers` → `client-lib`            | normal | Library target named `torrust_tracker_client` in `packages/tracker-client/Cargo.toml` |
| `axum-http-server` → `client-lib`        | dev    | Library target named `torrust_tracker_client`                                         |
| `e2e-tools` → `torrust-tracker`          | normal | Root library target named `torrust_tracker_lib`                                       |

The 2026-10-06 report ([finding 3](../../open/1669-overhaul-packages/workspace-coupling-report-2026-10-06.md#3-the-tool-misses-renamed-crates-and-custom-library-names),
issue #2446) now lists all five.

`cargo metadata` already provides both facts: each dependency's `rename`, and each package's
library target name (`targets[]` with kind `lib`).

## Scope

### In Scope

- Resolve the module name of each dependency edge from the dependency's `rename` when present,
  otherwise from the dependency package's library target name, falling back to the package name.
- Add regression tests covering a renamed dependency and a custom library name.
- Regenerate the coupling report as a new dated file and confirm the five edges list imports.

### Out of Scope

- Other scan limitations (macro expansion, inactive `cfg` code) documented in the report header.
- Changing the report format.

## Architectural Decisions

- Related ADRs: None
- ADRs to create: None known

## Design and Ownership Review

Not applicable: a single pure name-resolution step changes; no processes, I/O or fixtures.

### Maintainer Decisions (pre-implementation review, 2026-10-17)

- **Placement**: the `cargo metadata` structs (`Metadata`, `Package`, `Dep`) move from
  `src/main.rs` to `src/lib.rs`, together with a new pure `resolve_module_name` function, so the
  integration tests in `tests/` can exercise it with `cargo metadata`-shaped JSON. `main.rs`
  keeps only report generation and I/O.
- **Library target kinds**: the library target is the first `targets[]` entry whose `kind`
  contains `lib`, `rlib`, `dylib` or `proc-macro` (the kinds other crates can import).
  `cdylib` and `staticlib` targets are ignored. If none matches, fall back to the package name.
- **Report file**: the regenerated report is
  `docs/issues/open/1669-overhaul-packages/workspace-coupling-report-2026-10-17.md`. Earlier
  reports stay untouched.

## Bug-Fix Process

Follows the [`fix-bug`](../../../../.github/skills/dev/debugging/fix-bug/SKILL.md) skill:

1. Analysis and reproduction: done before review; the real tool was run and the outcome
   classified as **Reproduced** in `manual-verification-evidence.md` (R1).
2. Regression tests (T1): written first and recorded red against the unfixed tool.
3. Fix (T2): resolve the module name from `rename`, then the library target name.
4. Green and recheck (T3): tests pass; the R1 command is rerun unchanged (R2) and lists no
   false "no references" edge.

## Regression Test Strategy

- Boundary: unit tests of the module-name resolution, the single function the bug lives in,
  fed with `cargo metadata`-shaped package and dependency data. No workspace is needed.
- Cases: a renamed dependency, a custom library target name, and the plain package-name
  default.
- Each test is recorded failing against the unfixed code before the fix lands, per the
  `write-unit-test` skill.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task                                                                                     | Notes / Expected Output                                                       |
| --- | ------ | ---------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------- |
| T1  | DONE   | Add regression tests for a renamed dependency, a custom library name and the default    | Recorded red run: the rename and library-name cases fail on the current tool |
| T2  | DONE   | Resolve module names from `rename` and the library target name                          | The T1 tests pass                                                             |
| T3  | DONE   | Green run plus like-for-like recheck: rerun the R1 command unchanged                     | R2 in the evidence lists no false "no references" edge                        |
| T4  | DONE   | Regenerate the coupling report as a new dated file                                       | The five edges list their imports                                             |

## Commit Points

| Task | Coherent change set                       | Commit policy                                                              |
| ---- | ----------------------------------------- | -------------------------------------------------------------------------- |
| T1   | Regression tests                           | Commit together with T2 so every commit passes; record the red run first. |
| T2   | Name-resolution fix                        | One commit with T1.                                                        |
| T3   | R2 recheck evidence                        | One commit after the recheck.                                              |
| T4   | New dated coupling report                  | One commit after R2.                                                       |

Use a Conventional Commit message with the issue reference and sign every commit with GPG.

## Progress Tracking

### Workflow Checkpoints

- [x] Spec drafted in `docs/issues/drafts/`
- [x] Bug reproduced and classified in `manual-verification-evidence.md` before review (R1)
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue created and issue number added to this spec
- [x] Spec moved to `docs/issues/open/` with issue number prefix
- [x] Implementation completed
- [x] Automatic verification completed (`linter all`, relevant tests and pre-push checks)
- [x] Manual verification scenarios executed and recorded in issue-local
      `manual-verification-evidence.md`
- [ ] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] Evidence-based implementation completion review recorded
- [ ] Issue closed and spec moved to `docs/issues/closed/`

### Progress Log

- 2026-10-06 16:01 UTC - GitHub Copilot - Drafted from finding 3 of the 2026-10-06 workspace
  coupling report (#2446).
- 2026-10-07 08:52 UTC - GitHub Copilot - Reworked to follow the `fix-bug` skill, per PR #2462
  review finding F5: plain-language explanation, reproduction run and recorded as
  **Reproduced** (R1; it also found a fifth affected edge, the `axum-http-server` dev
  dependency), red regression-test task, and a like-for-like recheck (R2).
- 2026-10-07 09:25 UTC - GitHub Copilot - Copied the template's completion-review conditions,
  per PR #2462 review finding F15. Backfilled at 12:39 UTC, per review finding F20.
- 2026-10-07 12:29 UTC - GitHub Copilot - The link to finding 3 now says it lists all five
  edges, per PR #2462 review finding F21.
- 2026-10-07 16:39 UTC - GitHub Copilot - Maintainer approved the draft (reviewed in PR #2462).
  Created issue #2475, linked it as a subissue of EPIC #1669, and moved the spec to
  `docs/issues/open/`.
- 2026-10-17 12:26 UTC - GitHub Copilot - Recorded the maintainer's pre-implementation
  decisions (resolver placement in `lib.rs`, library target kinds, new report file name) in
  "Maintainer Decisions". The spec-only branch had already merged; implementation proceeds on
  branch `2475-1669-coupling-tool-resolve-lib-names-and-renames`.
- 2026-10-17 12:33 UTC - GitHub Copilot - Implemented T1-T4: regression tests recorded red,
  per-edge module-name resolution, R2 recheck (zero false "no references" edges) and the
  regenerated `workspace-coupling-report-2026-10-17.md`. The generated Observations text was
  hand-adjusted in the report to pass markdownlint (blank line before a list) and to describe
  this regeneration. Awaiting maintainer review (AC6).
- 2026-10-19 11:50 UTC - GitHub Copilot - Set `status: in-review` and `related-pr: 2559`, per
  `review-finding:pr-2559-f1`.
- 2026-10-19 11:51 UTC - GitHub Copilot - Ticked the checkboxes of AC1 to AC5 and of "Automatic
  verification completed" so they agree with the Acceptance Verification table, per
  `review-finding:pr-2559-f3`.
- 2026-10-19 11:51 UTC - GitHub Copilot - Restamped the implementation entry from 12:40 to 12:33
  UTC, the time of the commit that wrote it. The `last-updated-utc` of this spec and of the
  evidence file were restamped by the commits that edit them, per `review-finding:pr-2559-f2`.
  (The stamp of this entry was first written as 11:52, later than its commit at 11:51:50; the
  next commit corrected it to 11:51.)
- 2026-10-19 11:53 UTC - GitHub Copilot - Added `lib.rs`, `tests/resolve_module_name.rs` and the
  2026-10-17 report to `related-artifacts`, and linked the 2026-10-17 report from the EPIC, per
  `review-finding:pr-2559-f7`.
- 2026-10-19 11:53 UTC - GitHub Copilot - Correction to the 2026-10-17 12:33 UTC entry: it logs
  T4 (the regenerated report) as done, but the report was committed by the next commit,
  `docs(issues): [#2475] regenerate the workspace coupling report`, after the commit that wrote the
  entry (`docs(issues): [#2475] record the red run, green run and R2 recheck`). The history is kept
  as it happened, per the repository's commit-history policy, and the Commit Points table lists T4
  as the last commit. This is `review-finding:pr-2559-f5`.
- 2026-10-19 12:18 UTC - GitHub Copilot - T3 now pastes the `cargo test` result lines verbatim
  (all five, with the `Running` and `Doc-tests` lines as printed) and says they are filtered with
  `grep`, per `review-finding:pr-2559-f11`. The branch was also rebased onto `develop` for
  `review-finding:pr-2559-f8`; the rebase kept every commit and changed only the EPIC file's
  conflicting lines.
- 2026-10-19 12:44 UTC - GitHub Copilot - The log entries of this round now cite findings by their
  immutable reference, `review-finding:pr-2559-f<n>`, instead of "review finding F<n>", which was
  ambiguous because the review comments carry their own numbers (`review-finding:pr-2559-f12`).
  The EPIC's progress-log entry changed the same way.

## Acceptance Criteria

- [x] AC1: The tool resolves a dependency's module name from its `rename`, then its library
      target name, then its package name.
- [x] AC2: Regression tests cover a renamed dependency, a custom library name and the default;
      the rename and library-name tests were recorded failing against the unfixed tool.
- [x] AC3: Rerunning the R1 command unchanged (R2) lists no false "no references" edge, and the
      five edges in the Background list their imports.
- [x] AC4: `cargo test -p workspace-coupling` and `linter all` exit with code `0`.
- [x] AC5: The reproduction (R1) and the recheck (R2) are recorded in issue-local
      `manual-verification-evidence.md`.
- [ ] AC6: The acceptance criteria are re-reviewed after implementation.

## Verification Plan

### Automatic Checks

- `cargo test -p workspace-coupling`
- `linter all`
- Pre-push checks

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID  | Scenario                     | Human-oriented command/steps                                                                 | Expected Result                                             | Status | Evidence                                     |
| --- | ---------------------------- | -------------------------------------------------------------------------------------------- | ----------------------------------------------------------- | ------ | -------------------------------------------- |
| R1  | Initial reproduction          | `cargo run -q -p workspace-coupling -- /tmp/repro-2446.md`, then list the "no references" edges | Five used edges reported with no references (the bug)       | DONE   | `manual-verification-evidence.md` section R1 |
| R2  | Like-for-like recheck         | Same command and listing as R1, after the fix                                                 | No false "no references" edge; the five edges list imports  | DONE   | `manual-verification-evidence.md` section R2 |

Record the toolchain for each `cargo` command in the evidence file.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| ----- | ---------------------- | -------- |
| AC1   | DONE                   | `resolve_module_name` in `src/lib.rs`; commit `fix(workspace-coupling): [#2475] resolve module names from renames and library target names`. |
| AC2   | DONE                   | `tests/resolve_module_name.rs`; red run (6 of 9 failing) recorded as T1 in `manual-verification-evidence.md`. |
| AC3   | DONE                   | R2 in `manual-verification-evidence.md`: zero "no references" edges; the five edges list imports. |
| AC4   | DONE                   | `cargo test -p workspace-coupling` and `linter all` exit 0 (T3 in the evidence). Pre-push checks run before pushing. |
| AC5   | DONE                   | R1, T1, T3 and R2 in `manual-verification-evidence.md`. |
| AC6   | TODO                   |          |

## Risks and Trade-offs

- A dependency could be renamed differently by different dependents; resolution must be per
  edge, not per package.

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
- Baseline issue: #2446
