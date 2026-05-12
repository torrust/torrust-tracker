---
doc-type: issue
issue-type: task
status: blocked
priority: p2
github-issue: null
spec-path: docs/issues/drafts/1737-native-http3-readiness.md
branch: 1737-native-http3-readiness
related-pr: null
last-updated-utc: 2026-05-12 00:00
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - docs/templates/ISSUE.md
---

<!-- skill-link: create-issue -->

# Issue #[To be assigned] - docs(http): test and evaluate native HTTP/3 support in tracker

## Goal

Once upstream Rust HTTP dependencies (Hyper, Axum) provide stable HTTP/3 support, evaluate and test native HTTP/3 support in the tracker HTTP server. Document the results, performance impact, and any required code changes or configuration additions.

## Background

As documented in issue #1736, the tracker can expose HTTP/3 to clients via a reverse proxy today. However, direct/native HTTP/3 support in the tracker's Axum-based HTTP server would simplify deployments and potentially improve performance. This task creates a placeholder to track that work once upstream dependencies mature.

**Current blocker**: The Rust HTTP ecosystem (Hyper, Axum) is still stabilizing HTTP/3 support (see [hyperium/hyper#3925](https://github.com/hyperium/hyper/pull/3925)).

## Scope

### In Scope

- Monitor upstream Hyper/Axum HTTP/3 readiness (tracking issue watchers).
- Test functional correctness of native HTTP/3 on tracker announce/scrape endpoints and REST API.
- Benchmark performance and resource usage (CPU, memory) of direct HTTP/3 vs. proxy-terminated HTTP/3.
- Document migration path and backward compatibility requirements.
- Create or update tracker HTTP server code if upstream support reaches production-ready status.
- Update deployment docs with native HTTP/3 configuration (if implemented).

### Out of Scope

- Implementing workarounds for incomplete upstream support.
- Adding HTTP/3 support to other parts of the tracker (only HTTP server in scope).
- Performance optimization unrelated to HTTP/3 adoption.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task                                               | Notes / Expected Output                                                                       |
| --- | ------ | -------------------------------------------------- | --------------------------------------------------------------------------------------------- |
| T1  | TODO   | Check upstream HTTP/3 readiness                    | Review Hyper and Axum release notes; confirm stable HTTP/3 support is available.              |
| T2  | TODO   | Set up local test environment for native HTTP/3    | Configure tracker HTTP server with HTTP/3; set up client tools (curl, qBittorrent, etc.).     |
| T3  | TODO   | Test functional correctness                        | Verify announce, scrape, and REST API routes work over HTTP/3.                                |
| T4  | TODO   | Run performance and resource benchmarks            | Compare direct HTTP/3 vs. proxy-terminated HTTP/3; measure CPU, memory, latency.              |
| T5  | TODO   | Document results and migration path                | Write findings; identify any code changes or config additions needed.                         |
| T6  | TODO   | Update deployment docs if native HTTP/3 is enabled | Add native HTTP/3 config examples to [docs/containers.md](../../containers.md) if applicable. |
| T7  | TODO   | Run linter and validation checks                   | Ensure all documentation and code changes pass quality gates.                                 |

## Progress Tracking

### Workflow Checkpoints

- [ ] Spec drafted in `docs/issues/drafts/`
- [ ] Spec reviewed and approved by user/maintainer
- [ ] GitHub issue created and issue number added to this spec
- [ ] Implementation completed (testing and docs)
- [ ] Reviewer validated acceptance criteria
- [ ] Committer verified spec progress is up to date before commit
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-05-12 00:00 UTC - Agent - Spec drafted in `docs/issues/drafts/1737-native-http3-readiness.md`

## Acceptance Criteria

- [ ] AC1: Upstream HTTP/3 support status is confirmed stable or nearly stable (documented in issue comments).
- [ ] AC2: Functional tests confirm HTTP/3 works correctly for all tracker endpoints (announce, scrape, API).
- [ ] AC3: Performance benchmarks (CPU, memory, latency) are documented for native HTTP/3 vs. proxy-terminated HTTP/3.
- [ ] AC4: A clear migration path is documented (e.g., backward compatibility, config options).
- [ ] AC5: If native HTTP/3 is viable, tracker HTTP server code is updated and deployment docs are updated.
- [ ] AC6: If native HTTP/3 is not viable, rationale and blocker details are documented in a comment.
- [ ] AC7: `linter all` exits with code `0`.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence                           |
| ----- | ---------------------- | ---------------------------------- |
| AC1   | TODO                   | Issue comment with upstream status |
| AC2   | TODO                   | Test logs / validation report      |
| AC3   | TODO                   | Benchmark results in issue/PR      |
| AC4   | TODO                   | docs/containers.md or PR comments  |
| AC5   | TODO                   | Code changes and docs updates      |
| AC6   | TODO                   | Issue comment if not viable        |
| AC7   | TODO                   | linter output                      |

## Risks and Trade-offs

- **Risk**: Upstream HTTP/3 support may not reach stable status for an extended period.
  - _Mitigation_: This task is explicitly blocked; no work begins until upstream readiness is confirmed.
- **Risk**: Native HTTP/3 performance may not outperform proxy-terminated HTTP/3 significantly.
  - _Mitigation_: Benchmarks will inform decision to adopt; proxy-based approach remains viable.
- **Risk**: Tracker HTTP server changes for HTTP/3 support may introduce regressions.
  - _Mitigation_: Comprehensive functional testing of announce/scrape/API routes before merge.

## References

- Parent issue: #1736 (docs: document HTTP/3 support via reverse proxy)
- Upstream tracking: https://github.com/hyperium/hyper/pull/3925
- Axum HTTP/3 support: [Axum changelog / roadmap](https://github.com/tokio-rs/axum)
- Demo HTTP/3 issue: https://github.com/torrust/torrust-tracker-demo/issues/31
- Related docs: [docs/containers.md](../../containers.md)
