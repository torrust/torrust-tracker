---
semantic-links:
  skill-links:
    - write-markdown-docs
    - write-unit-test
  related-artifacts:
    - docs/issues/open/2132-add-sigterm-to-main/ISSUE.md
    - docs/issues/open/2132-add-sigterm-to-main/native-shutdown-test-plan.md
    - docs/issues/open/2132-add-sigterm-to-main/native-tracker-refactor-plan.md
    - tests/lifecycle/native_tracker.rs
    - tests/lifecycle/signals.rs
---

# Implementation Retrospective

## Purpose

Record evidence-based process improvements from implementing issue #2132. This
is a blameless review of the implementation approach, not a change to the
issue's completed behavior or a reason to rewrite its history.

## Outcome

The issue delivered Unix `SIGTERM` handling at the tracker executable boundary,
manual direct-binary evidence, and native executable-boundary SIGTERM/SIGINT
coverage. The resulting fixture correctly starts the Cargo-built tracker,
proves readiness before delivering a typed signal to the exact child PID, and
cleans up child processes after normal, failing, and panic paths.

The final `NativeTracker` fixture has explicit ownership boundaries:

- `NativeTracker` provides the narrow lifecycle-facing test interface.
- `NativeTrackerWorkspace` owns isolated temporary configuration and storage.
- `TrackerOutputCapture` concurrently drains and retains child output.
- `HealthCheckClient` performs deadline-bounded health-check communication.

The readiness loop remains on `NativeTracker` because it combines the fixture's
child-lifecycle checks, deadline, retry policy, and tracker-specific signal
handler readiness rule. The completed assessment found no evidence that a
separate `ReadinessProbe` would improve that design.

## What Went Well

1. The issue correctly identified the executable boundary as the required test
   layer. In-process tests could not prove `main()` receives an operating-system
   signal, while container E2E testing would have added unrelated cost.
2. The readiness contract avoided timing-based test flakiness. It requires a
   successful health response with `Status::Ok` and the explicit
   signal-handler-installed log marker before a signal scenario begins.
3. The fixture used child-specific configuration, temporary workspaces, and
   loopback port `0`, avoiding global environment mutation and port-allocation
   races in parallel tests.
4. Small commits and repeated focused validation exposed lifecycle issues before
   they became part of a large, difficult-to-review change.
5. The refactor explicitly rejected generic process, signal, output-query, and
   readiness abstractions without a demonstrated requirement or second
   consumer.

## What Changed During Implementation

The initial `tests/lifecycle/native_tracker.rs` correctly delivered a first
end-to-end vertical slice, but one `NativeTracker` type owned workspace
creation, output reader tasks, health HTTP requests, tracker readiness
interpretation, child lifecycle, and cleanup. Its readiness method interleaved
log discovery, HTTP response handling, report decoding, deadline handling,
child-exit detection, retries, and diagnostics.

Implementation and review provided concrete evidence for a narrower design:

1. Workspace ownership had to survive asynchronous drop-path cleanup until the
   child was reaped.
2. Output reader tasks had to drain concurrently for the child lifetime and be
   joined before completed output could support shutdown diagnostics.
3. Output capture needed to remain passive; parsing tracker startup logs and
   checking the signal marker are tracker-specific readiness rules.
4. A single absolute startup deadline needed to bound HTTP requests and response
   decoding, rather than only the retry loop.
5. Health checks needed explicit distinction between a transient unavailable
   endpoint, a successful decoded report, a timeout, an unexpected HTTP status,
   and an invalid report.

These findings led to `NativeTrackerWorkspace`, `TrackerOutputCapture`, and
`HealthCheckClient`, plus a smaller readiness orchestrator.

## Root Cause

The native shutdown test plan specified behavioral and lifecycle invariants
well, but it did not require an initial fixture responsibility map or explicit
internal ownership boundaries. It described a reusable fixture with isolated
workspace, captured output, readiness, and cleanup, so a first implementation
could legitimately consolidate those responsibilities in `NativeTracker`.

A substantially better first version was possible. The plan should have
required a narrow lifecycle facade with separate, coherent owners for temporary
workspace/configuration, passive output capture, and health endpoint
communication. The precise internal details still needed implementation
feedback, but the primary responsibility boundaries were foreseeable.

## Improvements for Future Issue Specifications

For fixtures that combine a child process, asynchronous I/O, network readiness,
and panic-safe cleanup, add these requirements before implementation:

1. Define a responsibility and ownership map before writing the main fixture.
   Specify the public fixture interface and identify which private collaborators
   own temporary resources, output draining, external API communication, and
   lifecycle coordination.
2. State cross-path lifetime invariants. Resources required by the child must
   remain alive through explicit shutdown and asynchronous drop cleanup until
   the child is reaped; completed output should be retained for teardown
   diagnostics where possible.
3. Define readiness deadlines as absolute bounds over all awaited operations,
   including connection attempts, response decoding, child-exit checks, and
   retry delays.
4. Separate passive infrastructure from domain interpretation. For example,
   output capture owns bytes and reader tasks; the fixture's readiness policy
   owns tracker-specific log parsing and meaning.
5. Require a design review immediately after the first passing vertical slice
   and before treating the fixture as complete. The review must either confirm
   the responsibility boundaries or schedule a bounded refactor.
6. Require an explicit assessment for proposed extra abstractions. Add a type
   only when it owns a coherent responsibility and improves the current design;
   do not create generic frameworks or abstractions solely in anticipation of
   future reuse.

## Avoiding Overcorrection

The lesson is not to specify every private type, helper method, or error enum in
an issue specification. That would make the specification an untested
implementation blueprint and encourage abstractions without evidence.

The appropriate requirement is a clear ownership model and a short
post-vertical-slice design checkpoint. Implementation evidence should still
shape private contracts such as cleanup transfer details and health-probe
outcome classification.

## Evidence

- [Issue specification](ISSUE.md)
- [Native executable shutdown test plan](native-shutdown-test-plan.md)
- [Native tracker fixture incremental refactor plan](native-tracker-refactor-plan.md)
- [`tests/lifecycle/native_tracker.rs`](../../../../tests/lifecycle/native_tracker.rs)
- [`tests/lifecycle/signals.rs`](../../../../tests/lifecycle/signals.rs)

The original native fixture entered history in `92fc32ac`. The completed
fixture refactor and small evidence-based follow-ups were validated with
`cargo test --test lifecycle-signals`, `cargo fmt --check`, and `linter all`.
