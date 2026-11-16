---
schema-version: 1
doc-type: issue
issue-type: bug
status: draft
priority: p2
epic: 2392
github-issue: null
spec-path: docs/issues/drafts/2392-fixed-port-metrics-test-concurrent-runs/ISSUE.md
branch: "{issue-number}-2392-fixed-port-metrics-test-concurrent-runs"
related-pr: null
last-updated-utc: "2026-11-16 17:30"
semantic-links:
  skill-links:
    - create-issue
    - fix-bug
    - write-unit-test
  related-artifacts:
    - .github/skills/dev/debugging/fix-bug/SKILL.md
    - .github/skills/dev/planning/create-issue/SKILL.md
    - docs/issues/open/2392-test-isolation/EPIC.md
    - docs/issues/open/2394-2392-audit-shared-test-resources/inventory.md
    - docs/issues/closed/2393-2392-test-port-allocation-races/ISSUE.md
    - tests/metrics/fixed_ports.rs
    - packages/test-helpers/src/random.rs
---

<!-- skill-link: create-issue -->

# Issue #[To be assigned] - Let Two Runs of the Fixed-Port Metrics Test Share a Machine

Parent EPIC: #2392 - Test Isolation (`docs/issues/open/2392-test-isolation/EPIC.md`)

## Goal

Two concurrent runs of the `metrics-fixed-ports` integration test on one machine both pass. The
test keeps covering tracker listeners on fixed ports.

## Background

The shared-resource audit (#2394, finding P2 in its `inventory.md`) found that
`tests/metrics/fixed_ports.rs` starts the tracker with four listeners on fixed ports:
`0.0.0.0:17091` and `17092` (HTTP), `17093` and `17094` (UDP). Within one suite run nothing else
uses them, and they are below the ephemeral range (EPIC rule 6). But a second run of the same test
binary on the same machine binds the same addresses:

1. Run A (an agent's `cargo test`, or its pre-push hook, which runs `cargo test --workspace`)
   starts the tracker on `0.0.0.0:17091`-`17094`.
2. Run B, from another checkout on the same machine, starts the same test while A still holds the
   ports.
3. Run B's tracker fails to start: `ServiceStartup { service: "HTTP tracker", ... AddrInUse }`.

Reproduced on 2026-11-16 at `b009b6425` (Linux 7.0.0): with `0.0.0.0:17091` held by another
process the test fails at once, and running the built test binary twice at the same time, 20
times, failed one of the two runs in all 20 pairs (20 failures in 40 runs).

CI runs the Rust tests only on GitHub-hosted runners (one job per virtual machine), so today this
hits development machines: several AI agents on one machine. It would also hit a self-hosted
runner that runs the Rust suite with more than one runner instance.

## Scope

### In Scope

- Make concurrent runs of `metrics-fixed-ports` not collide, keeping listeners on configured
  non-zero ports. Candidate approach, to be confirmed in spec review: bind the four fixed ports on
  a random loopback address from `torrust_tracker_test_helpers::random::loopback_ipv4()` (#2393)
  instead of `0.0.0.0`, so two runs use different addresses with the same ports. Linux only, like
  the nine #2393 tests that use the helper, so the test is ignored off Linux.
- Document in the test why the ports are fixed and how concurrent runs avoid each other.

### Out of Scope

- Other tests (the audit found no other fixed-port listener in Rust tests).
- A new shared port helper (EPIC decision of 2026-11-14).
- Fixed E2E host ports (#2395).

## Architectural Decisions

- Related ADRs: none.
- ADRs to create: none expected.
- Using `loopback_ipv4()` here extends the helper beyond the nine #2393 tests. The EPIC records
  that only those nine use it; the maintainer must approve the extension in spec review, or pick
  another option (for example deleting the test if `metrics-port-zero` already covers the same
  contract, after the disable-first decision of 2026-11-14).

## Design and Ownership Review

The test binary owns one tracker application instance (`tests/AGENTS.md`). The loopback address
is chosen once per run, substituted into the configuration before the tracker starts, and the
listeners are released by the existing fixture shutdown. No new fixture type is expected.

## Bug-Fix Process

The defect is in the test, not in tracker behavior. Follow
`.github/skills/dev/debugging/fix-bug/SKILL.md`:

1. Real-artifact reproduction: hold `0.0.0.0:17091` (or run the test binary twice at once) and
   record the `AddrInUse` failure in `manual-verification-evidence.md`.
2. Regression boundary: the existing test binary run twice concurrently; a unit test cannot show
   a cross-process collision.
3. Red: the paired-run reproduction before the change. Green: the same paired runs after it.

## Regression Test Strategy

No new test. The guard is the paired-run recheck: before the change one run fails in every pair,
so 20 green pairs after the change is strong evidence. Record both in
`manual-verification-evidence.md`.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID | Status | Task | Notes / Expected Output |
| --- | --- | --- | --- |
| T1 | TODO | Reproduce on the current `develop` | Held-port and paired-run failures recorded (V1). |
| T2 | TODO | Isolate the fixed-port listeners per run | Approach approved in spec review; test documents why the ports are fixed. |
| T3 | TODO | Recheck | 20 paired runs, all green (V2); `linter all` passes. |

## Commit Points

| Task | Coherent change set | Commit policy |
| --- | --- | --- |
| T1 | Reproduction evidence | Documentation-only commit. |
| T2-T3 | Test change and recheck evidence | Commit after the recheck and maintainer review. |

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/drafts/2392-fixed-port-metrics-test-concurrent-runs/ISSUE.md`
- [ ] Spec reviewed and approved by user/maintainer
- [ ] GitHub issue created and issue number added to this spec
- [ ] Implementation completed
- [ ] Automatic verification completed (`linter all`, relevant tests, and any pre-push checks)
- [ ] Manual verification scenarios executed and recorded in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] Evidence-based implementation completion review recorded
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-11-16 17:30 UTC - Claude Code (Claude Opus 5.5), #2394 session - Drafted from audit finding
  P2 after the maintainer classified it as a new problem; reproduced the collision at `b009b6425`.

## Acceptance Criteria

- [ ] AC1: Two runs of `metrics-fixed-ports` started at the same time on one Linux machine both
  pass (20 paired runs).
- [ ] AC2: The test still starts listeners on configured non-zero ports and checks the same
  aggregate metrics.
- [ ] AC3: The test documents why its ports are fixed and how concurrent runs avoid each other.
- [ ] `linter all` exits with code `0`

## Verification Plan

### Automatic Checks

- `linter all`
- `cargo test --test metrics-fixed-ports`

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID | Scenario | Human-oriented command/steps | Expected Result | Status | Evidence |
| --- | --- | --- | --- | --- | --- |
| V1 | Reproduce | Hold `0.0.0.0:17091`, run the test; run the built binary twice at once, 20 times | Fails with `AddrInUse` | TODO | `manual-verification-evidence.md` |
| V2 | Recheck | Run the built binary twice at once, 20 times | 40 of 40 pass | TODO | `manual-verification-evidence.md` |

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| --- | --- | --- |
| AC1 | TODO | V2 |
| AC2 | TODO | Diff review |
| AC3 | TODO | Diff review |

## Risks and Trade-offs

- **Linux only.** The loopback range trick does not work on macOS or the BSDs; the test would be
  ignored there. CI runs no tests off Linux.
- **Coverage of `0.0.0.0`.** The test stops binding the wildcard address. If binding `0.0.0.0` is
  part of the contract, record that and choose another option.

## Implementation Completion Review

- Retrospective: `Not yet assessed`

## References

- Parent EPIC: #2392
- Audit: #2394, `docs/issues/open/2394-2392-audit-shared-test-resources/inventory.md` (P2)
- Loopback helper: #2393, `packages/test-helpers/src/random.rs`
