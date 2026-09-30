---
doc-type: file-test-plan
issue: 2283
package: torrust-tracker-udp-server
target-file: packages/udp-server/src/testing/environment.rs
status: done
semantic-links:
  related-artifacts:
    - packages/udp-server/src/testing/environment.rs
    - docs/issues/open/1488-overhaul-tracker-shutdown/ISSUE.md
    - docs/issues/open/2283-1347-increase-udp-server-package-coverage/coverage-evidence.md
---

# UDP Server Testing Environment File Test Plan

Follow the shared [guidance](README.md). This plan covers only
`packages/udp-server/src/testing/environment.rs`.

## Current State

- **Current unit-only coverage:** 183 / 198 lines (92.42%), 16 / 22 functions, and 191 / 215
  regions from `cargo llvm-cov -p torrust-tracker-udp-server --all-features --lib --json` (stable
  Rust toolchain).
- **Module-owned decision:** map the configured connection-ID validation policy to the UDP-core
  policy.
- **Lifecycle orchestration:** construct, start, and stop the test environment and its listener
  jobs. This is owned by #1488 SI-14/SI-17.
- **Existing test:** starts and stops a live UDP server, but hides its lifecycle condition behind
  fixed one-second sleeps and has no observable assertion beyond successful completion.
- **T1 hypothesis:** add the two direct unit contracts for the pure policy mapping; do not define
  new lifecycle behavior before #1488 establishes its stable seam.

## Current Tests Review

The live-server test is a lifecycle collaboration test, not a unit contract for this file's pure
decision. Its real server construction and stop action are relevant, but the fixed sleeps make the
test slow and its successful completion is its only assertion. The plan does not alter it: #1488
owns lifecycle readiness, shutdown, listener joining, and cleanup policy.

## Coverage Analysis

| Behavior group | Classification | Decision |
| --- | --- | --- |
| Strict configuration policy mapping | Unit | Add one direct pure-function test. |
| Disabled configuration policy mapping | Unit | Add one direct pure-function test. |
| Environment construction and server start | Lifecycle (#1488 SI-14) | Retain the existing live-server coverage; do not add or redefine readiness behavior. |
| Listener shutdown, server stop, joining, and cleanup | Lifecycle (#1488 SI-17) | No test change; the shutdown design is owned by #1488. |
| Container composition and persistence setup | Collaboration | Do not assert collaborator semantics from this fixture. |

## Steps

| ID | Status | Work | Completion evidence |
| --- | --- | --- |
| R1 | DONE | Added direct tests for strict and disabled policy mapping. | Focused policy test run: 2 passed. |
| R2 | DONE | Re-ran unit-only coverage and recorded the file result. | Stable report: 183 / 198 lines, 16 / 22 functions, 191 / 215 regions. |
| R3 | DONE | Validated formatting, documentation records, and the package test slice. | Nightly formatting, Markdown lint, cspell, diff check, and 3 environment tests passed. |

## Completed-File Review

Approved by Jose Celano on 2026-09-30.

## Progress Log

- 2026-09-30 06:03 UTC - GitHub Copilot - Created the plan from source inspection, the Issue
  #2283 coverage baseline, the existing live-server test, and #1488 SI-14/SI-17 ownership.
  Maintainer approval is requested before R1.
- 2026-09-30 06:06 UTC - GitHub Copilot - Added the two approved pure mapping tests. The focused
  policy run passed 2 tests; the complete environment module run passed 3 tests. Stable unit-only
  coverage is 183 / 198 lines (92.42%), 16 / 22 functions, and 191 / 215 regions.
- 2026-09-30 06:06 UTC - GitHub Copilot - Completed R3: nightly Rust formatting, Markdown lint,
  cspell, diff check, and the 3-test environment module slice passed. Completed-file review is
  requested.
- 2026-09-30 06:07 UTC - Jose Celano - Approved the completed-file result.
