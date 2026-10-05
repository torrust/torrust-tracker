---
schema-version: 1
doc-type: issue
issue-type: task
status: planned
priority: p3
epic: null
github-issue: 2435
spec-path: docs/issues/open/2435-remove-misleading-panics-in-in-memory-torrent-repository/ISSUE.md
branch: "2435-remove-misleading-panics-in-in-memory-torrent-repository-spec"
related-pr: null
last-updated-utc: "2026-10-05 07:05"
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - .github/skills/dev/planning/create-issue/SKILL.md
    - packages/tracker-core/src/torrent/repository/in_memory.rs
    - packages/swarm-coordination-registry/src/swarm/registry.rs
---

<!-- skill-link: create-issue -->

# Issue #2435 - Remove Misleading Panics From the In-Memory Torrent Repository

## Goal

`InMemoryTorrentRepository` in `tracker-core` no longer documents or implements panics for failures that cannot happen, and any genuinely fallible operation returns an error to its caller instead of panicking.

## Background

Ten methods of `InMemoryTorrentRepository` (`packages/tracker-core/src/torrent/repository/in_memory.rs`) call `.expect(...)` on results from the swarm coordination `Registry` and carry a `# Panics` section saying they panic "if the underlying swarms return an error". Panicking in a library path is discouraged, so this looks like a reliability problem.

However, the registry's error type is `pub type Error = Infallible;` (`packages/swarm-coordination-registry/src/swarm/registry.rs`, documented as "The registry currently exposes no recoverable error cases"). The `expect` calls therefore cannot panic today. The problem is that the code and documentation claim a failure mode that does not exist:

- callers and reviewers reasonably conclude these methods can crash the tracker;
- the `# Panics` sections and `expect` messages add noise to every new method (PR #2423 added `get_swarm_metadata` following the same pattern);
- if the registry ever gains a real error variant, these call sites would silently become real panics instead of failing to compile.

Found during PR #2423 (issue #2406) review.

## Scope

### In Scope

- Inventory every `Registry` method returning `Result<_, Error>` and every caller in `tracker-core` that unwraps it.
- Decide, and record the decision, between:
  - **A.** Keep `Result<_, Infallible>` in the registry and destructure it irrefutably in the repository (for example `let Ok(value) = result;`, available since Rust 1.82; MSRV is 1.88), removing the `expect` calls and `# Panics` docs.
  - **B.** Remove `Result` from the infallible registry methods so they return values directly.
  - **C.** Introduce a real registry error and propagate it through `InMemoryTorrentRepository` to its callers.
- Implement the chosen option and update the affected doc comments.

### Out of Scope

- Other `expect`/`unwrap` uses outside `InMemoryTorrentRepository` and the registry methods it calls.
- Changing swarm-coordination behavior.

## Architectural Decisions

- Related ADRs: none known.
- ADRs to create: only if option C introduces a new error-propagation contract between `swarm-coordination-registry` and `tracker-core`.

## Design and Ownership Review

Not applicable.

## Bug-Fix Process

Not applicable. No incorrect runtime behavior is known; the defect is a misleading contract and a latent risk. Although the `create-issue` skill treats misleading behavior as a bug, the maintainer kept this a task: there is no runtime defect to reproduce, so the `fix-bug` reproduction and regression-test sections have nothing to act on.

## Regression Test Strategy

Not applicable. Options A and B are compile-time guarantees; option C needs tests for the propagated error path.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID | Status | Task | Notes / Expected Output |
| --- | --- | --- | --- |
| T1 | TODO | Inventory fallible registry methods and repository callers | List of methods, callers, and whether any can fail |
| T2 | TODO | Choose option A, B, or C | Record the decision, rationale, and maintainer approval here before implementation |
| T3 | TODO | Implement the chosen option | No `expect` on registry results in `in_memory.rs`; doc comments match behavior |

## Commit Points

| Task | Coherent change set | Commit policy |
| --- | --- | --- |
| T2 | Decision recorded in the spec | Commit after maintainer approval |
| T3 | Production change and doc updates | Commit after focused validation and review |

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/drafts/remove-misleading-panics-in-in-memory-torrent-repository/ISSUE.md`
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue created and issue number added to this spec
- [ ] Implementation completed
- [ ] Automatic verification completed (`linter all`, relevant tests, and any pre-push checks)
- [ ] Manual verification scenarios executed and recorded in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] Evidence-based implementation completion review recorded: issue-local retrospective created for material discoveries, or progress log states why none was needed
- [ ] Reviewer validated acceptance criteria and updated checkboxes
- [ ] Committer verified spec progress is up to date before commit
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-10-03 07:45 UTC - Copilot - Drafted from a maintainer observation during PR #2423 review; confirmed the registry error type is `Infallible`.
- 2026-10-03 07:58 UTC - Copilot - Maintainer approved the draft; the A/B/C decision is deferred until the draft moves to `docs/issues/open/`. Committed as a draft in PR #2423; no GitHub issue yet.
- 2026-10-05 07:05 UTC - Copilot - Maintainer confirmed the task classification (no runtime defect). Created GitHub issue #2435 and moved the spec to `docs/issues/open/` on a spec-only branch; the A/B/C decision remains open for T2.

## Acceptance Criteria

- [ ] AC1: No method of `InMemoryTorrentRepository` documents a panic that cannot occur.
- [ ] AC2: No method of `InMemoryTorrentRepository` calls `expect` or `unwrap` on a registry result.
- [ ] AC3: If the registry gains a real error variant, the repository fails to compile or propagates the error, rather than panicking.
- [ ] `linter all` exits with code `0`
- [ ] Relevant tests pass
- [ ] Manual verification scenarios are executed and documented in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria are re-reviewed after implementation and reflect actual behavior
- [ ] Documentation is updated when behavior/workflow changes

## Verification Plan

### Automatic Checks

- `linter all`
- `cargo test -p torrust-tracker-core -p torrust-tracker-swarm-coordination-registry`
- Pre-push checks

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID | Scenario | Human-oriented command/steps | Expected Result | Status | Evidence |
| --- | --- | --- | --- | --- | --- |
| M1 | Tracker smoke test | Start a local tracker, announce and scrape one torrent over UDP and HTTP with `tracker_client` | Normal announce and scrape responses; no panics in the logs | TODO | `manual-verification-evidence.md` section V1 |

### Disposable Verification Scripts

None planned.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| --- | --- | --- |
| AC1 | TODO | Doc comments in `in_memory.rs` |
| AC2 | TODO | `grep` of `in_memory.rs` |
| AC3 | TODO | Chosen option and its compile-time or test evidence |

## Risks and Trade-offs

- Option B or C changes public signatures of `swarm-coordination-registry`; check downstream callers and benchmarks.
- Option A keeps the `Result` wrapper; it is the smallest change but leaves an unusual API shape.

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

## References

- Related issues: #2406
- Related PRs: #2423
- Related ADRs: none
