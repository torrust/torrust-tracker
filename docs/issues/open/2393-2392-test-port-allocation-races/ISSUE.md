---
schema-version: 1
doc-type: issue
issue-type: bug
status: planned
priority: p2
epic: 2392
github-issue: 2393
spec-path: docs/issues/open/2393-2392-test-port-allocation-races/ISSUE.md
branch: "2392-test-isolation-spec"
related-pr: 2397
last-updated-utc: "2026-10-02 13:30"
semantic-links:
  skill-links:
    - create-issue
    - fix-bug
    - write-unit-test
  related-artifacts:
    - docs/issues/open/2392-test-isolation/EPIC.md
    - docs/issues/open/2395-2392-e2e-runner-dynamic-host-ports/ISSUE.md
    - .github/skills/dev/debugging/fix-bug/SKILL.md
    - .github/skills/dev/testing/write-unit-test/SKILL.md
    - packages/axum-http-server/src/server.rs
    - tests/configuration/cli_configuration/base_source_precedence.rs
    - docs/issues/closed/2298-rust-dev-tool-container-integration/manual-verification-evidence.md
    - docs/issues/open/1419-allow-multiple-integration-tests-at-main-app-level/ISSUE.md
    - docs/issues/closed/2386-1840-self-hosted-runner-minimum-capacity/ISSUE.md
    - docs/issues/closed/2132-add-sigterm-to-main/native-shutdown-test-plan.md
    - tests/metrics/fixed_ports.rs
    - src/console/ci/e2e/runner.rs
---

<!-- skill-link: create-issue -->

<!-- markdownlint-disable MD003 -->

# Issue #2393 - Eliminate Parallel Test Port Allocation Races

Parent EPIC: #2392 - Test Isolation (`docs/issues/open/2392-test-isolation/EPIC.md`)

## Goal

Make the tracker test suite reliable under parallel execution by replacing unsafe test port
allocation patterns. The fix must preserve the tests' intended listener and configuration
contracts while preventing unrelated tests from claiming their addresses.

## Background

Several tests choose a TCP or UDP address by binding loopback port `0`, reading the address, and
then dropping the socket before the system under test binds it. Between those operations, another
parallel test or process can claim the released port. Other tests use fixed ports in Linux's
configurable ephemeral-port range, so a concurrent port-`0` bind can receive the same port.

The wrong outcome is an intermittent `Address already in use` failure unrelated to the behavior a
test intends to verify. The failure makes the suite nondeterministic and can hide regressions behind
infrastructure noise.

Real-artifact reproduction is recorded in `manual-verification-evidence.md`. A full nextest run
reproduced both observed failure modes: the `axum-http-server` duplicate-registration test failed
when an unrelated process claimed its released address, and a CLI configuration test failed because
its fixed health-check port was already in use.

### Related Issues

| Issue | Relationship |
| --- | --- |
| #1419 (open) | Same problem class for main-application integration tests. It moved most suites to port `0` with registry-based address discovery; `tests/metrics/fixed_ports.rs` still uses fixed ports `17091`-`17094`, below Linux's default ephemeral range (`32768`-`60999`). This bug reuses #1419's pattern and audits that suite in T5; it does not change #1419's remaining #1488 shutdown work. |
| #2386 (open) | Documents fixed E2E host ports (`6969/udp`, `7070`, `1212`, `1313`) in `src/console/ci/e2e/runner.rs` that prevent concurrent E2E jobs on one Docker host. Different failure domain (Docker host, not nextest); sibling subissue #2395 owns it. |
| #2132 (closed) | Its native-process fixture design explicitly rejected "find a free port, release it, then spawn" and chose port `0` plus startup-log discovery. That fixture is the precedent for T4. |

Of these, only #1419 is itself a subissue of the parent EPIC; #2386 and #2132 are references.

## Scope

### In Scope

- Replace the unsafe address-selection fixture in `axum-http-server` duplicate-registration tests.
- Replace fixed ephemeral-range health-check ports in CLI configuration tests with exclusive,
  test-owned addresses while preserving precedence assertions.
- Inventory other Rust tests that reserve port `0` and then drop the listener or use fixed
  ephemeral ports; repair each that can race under parallel execution. The EPIC's shared-resource
  audit (#2394) covers the other resource classes and reuses this inventory for ports.
- Add deterministic regression tests at each selected causal seam.
- Record red, green, and like-for-like parallel-run evidence in this issue directory.

### Out of Scope

- Changing production listener startup APIs unless retaining a bound test listener is impossible
  through existing test seams.
- Serializing the entire test suite or lowering nextest concurrency.
- Changing tracker protocol behavior, default service ports, or deployment configuration.
- Fixed E2E container host ports in `src/console/ci/e2e/runner.rs` (sibling subissue #2395).
- #1419's remaining cooperative-shutdown acceptance criterion (AC8a, blocked on #1488).
- Refactoring tests that use port `0` but retain the socket until the tested service owns it.

## Architectural Decisions

- Related ADRs: none known. Create an ADR only if the fix introduces a shared repository-wide test
  networking abstraction with a policy beyond local fixture mechanics.
- Test addresses are owned resources. A fixture must retain an address-reserving socket until it
  transfers ownership to the tested component, or use an address allocation mechanism that cannot
  race with concurrent tests.
- Fixed ports used by concurrent tests must be outside the platform's ephemeral allocation range or
  be allocated exclusively by the fixture.

## Design and Ownership Review

| Concern | Owner | Required invariant |
| --- | --- | --- |
| Test fixture | Test module | Retains or exclusively allocates the address until the tested server owns it. |
| Tested server | Production component | Binds only the address or listener supplied through its established API. |
| Test teardown | Test module | Releases listener and child-process resources on success, failure, and drop paths. |
| Readiness wait | Test helper | Uses an absolute timeout and reports the configured address when startup fails. |

After the first vertical slice, review whether an existing helper makes the resource lifetime clear.
Do not add a shared abstraction merely to avoid a local fixture.

## Bug-Fix Process

Follow `.github/skills/dev/debugging/fix-bug/SKILL.md`:

1. Confirm each candidate's causal port-allocation sequence from source and reproduce it against a
   real test artifact.
2. Record commands, toolchain, failures, and logs in `manual-verification-evidence.md`.
3. Select the smallest deterministic regression boundary. Use a focused test when it can force the
   former allocation race; otherwise use the smallest parallel nextest run that exposes it and
   record why a unit boundary cannot model OS socket ownership.
4. Add the regression test before changing behavior, prove it red, then make the smallest fixture
   or configuration change.
5. Prove green and rerun the original parallel reproduction command.

## Regression Test Strategy

The defect is observable only through OS socket allocation and concurrent test processes. The first
candidate boundary is the affected fixture plus the server startup test, because a pure unit test
cannot prove the kernel preserves an address after a fixture drops its listener. Each subtask must
first assess whether a focused test can make the allocation deterministic (for example, by retaining
an occupied listener). If not, the maintained regression test is the narrowest parallel nextest
command covering the affected test binaries. Prove it red with the old fixture or fixed port before
accepting the fix.

When adding or changing tests, use the `write-unit-test` skill. For every increment, write
prose-first Arrange-Act-Assert, make the test express that prose, run focused validation, and record
the design review before moving to the next increment.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID | Status | Task | Expected Output |
| --- | --- | --- | --- |
| T1 | DONE | Reproduce and inventory the observed failures | Evidence records the 200 isolated, 100 package, and 51 full-suite runs and source inventory. |
| T2 | TODO | Plan `axum-http-server` repair | Identify a test-only seam that retains or transfers the reserved listener; record the selected regression boundary. |
| T3 | TODO | Prove and repair the duplicate-registration test | Red regression evidence, smallest fixture change, focused green run, and design review. |
| T4 | TODO | Prove and repair CLI configuration ports | The precedence contract needs three distinct, predictable health-check addresses, so plain port `0` cannot identify the winning source. Choose between ports outside the ephemeral range or exclusively owned addresses; prove red and green under the selected parallel boundary. |
| T5 | TODO | Audit related fixtures | Classify each port-`0` or fixed-port fixture as safe, repaired, or follow-up, including #1419's `tests/metrics/fixed_ports.rs`; do not change unrelated fixtures without evidence. |
| T6 | TODO | Recheck and complete review | Like-for-like parallel recheck, acceptance review, completion review, and evidence update. |

## Commit Points

| Task | Coherent change set | Commit policy |
| --- | --- | --- |
| T2 | Per-file test plan and regression boundary | Documentation-only commit if it changes tracked evidence. |
| T3 | `axum-http-server` fixture/test repair | One commit after red and focused green evidence. |
| T4 | CLI configuration test repair | One commit after red and focused green evidence. |
| T5 | Each independently repaired fixture | One commit per fixture or small cohesive fixture family. |
| T6 | Final evidence and completion review | Documentation-only commit. |

## Progress Tracking

### Workflow Checkpoints

- [x] Initial real-artifact reproduction recorded.
- [x] Draft reviewed and approved by the maintainer.
- [x] GitHub issue created and issue number added to this specification.
- [ ] Regression tests proven red before each fix.
- [ ] Fixes implemented and focused checks pass.
- [ ] Like-for-like parallel recheck recorded.
- [ ] Acceptance criteria reviewed after implementation.
- [ ] Implementation completion review recorded.

### Progress Log

- 2026-09-30 13:29 UTC - GitHub Copilot - Drafted after an `axum-http-server`
  listener-release failure was reproduced in the full local nextest suite. The same run exposed a
  fixed health-check port collision in a CLI configuration test. No implementation has started.
- 2026-09-30 13:34 UTC - GitHub Copilot - Linked related fixed-port issues #1419, #2386, and
  #2132; excluded E2E host ports (#2386) from scope; recorded that the CLI precedence test needs
  distinct predictable ports.
- 2026-09-30 13:38 UTC - GitHub Copilot - Placed under the new test isolation EPIC
  draft, with the E2E host-port work as a sibling subissue.
- 2026-09-30 14:49 UTC - GitHub Copilot - Consistency review: corrected which related issues are
  EPIC subissues and stated the port-inventory boundary with the shared-resource audit subissue.
- 2026-09-30 14:58 UTC - GitHub Copilot - Maintainer approved the draft. Created #2393 as a
  sub-issue of EPIC #2392 and moved the specification to `docs/issues/open/`.

## Acceptance Criteria

- [ ] AC1: The duplicate-registration tests do not release a selected address before the tested
  server owns it, and their registration-error assertions remain intact.
- [ ] AC2: CLI configuration tests do not use fixed addresses that overlap the platform ephemeral
  allocation range while running concurrently.
- [ ] AC3: Every audited related fixture has a documented safe, repaired, or follow-up disposition.
- [ ] AC4: Each changed test has a regression boundary proven red against the previous behavior and
  green after the fix.
- [ ] AC5: The original full parallel reproduction command passes repeatedly after the fixes.
- [ ] `linter all` exits with code `0`.
- [ ] Relevant focused tests and pre-push checks pass.
- [ ] Manual verification evidence records reproduction, red/green checks, and final recheck.

## Verification Plan

### Automatic Checks

- Focused `cargo nextest run` commands for every changed package and test.
- The selected parallel regression commands with `--no-fail-fast`.
- `linter all`.
- Pre-push checks before publishing an implementation branch.

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID | Scenario | Human-oriented command/steps | Expected Result | Status | Evidence |
| --- | --- | --- | --- | --- | --- |
| M1 | Original failure reproduction | Run the full default-member nextest suite repeatedly with `--no-fail-fast`. | The prior race is observed or the evidence records the exact attempted run count. | DONE | `manual-verification-evidence.md#V1` |
| M2 | Duplicate-registration repair | Run the affected tests repeatedly in the selected parallel boundary. | No `AddrInUse` failure; registration-error contract remains visible. | TODO | `manual-verification-evidence.md#V2` |
| M3 | CLI configuration repair | Run CLI configuration tests concurrently with the selected suite. | Tracker starts on the intended address without a collision. | TODO | `manual-verification-evidence.md#V3` |
| M4 | Full recheck | Repeat the original full-suite command after all repairs. | All runs pass; no listed port-allocation failure returns. | TODO | `manual-verification-evidence.md#V4` |

### Acceptance Verification

Re-review every acceptance criterion against the recorded evidence after implementation. Do not
mark AC5 done from a focused test alone.

## Risks and Trade-offs

- A listener-retaining fixture may require exposing a test-only server seam; keep it private to the
  package unless several modules genuinely need it.
- Repeated parallel runs are probabilistic evidence, not a substitute for ownership-correct fixture
  design.
- The local machine has 32 logical processors, so its nextest scheduling may differ from CI; record
  both environments if CI exposes additional failures.

## Implementation Completion Review

After implementation, compare the result with this specification. Record whether the selected
fixture ownership model generalized safely, any tests intentionally left unchanged, and reusable
lessons. Create `implementation-retrospective.md` only if the work changes repository-wide test
networking policy; otherwise add a concise progress-log entry explaining why no separate
retrospective is needed.

## References

- Issue #2298 manual verification follow-up:
  `docs/issues/closed/2298-rust-dev-tool-container-integration/manual-verification-evidence.md`
- `axum-http-server` duplicate-registration tests: `packages/axum-http-server/src/server.rs`
- CLI configuration failure: `tests/configuration/cli_configuration/base_source_precedence.rs`
- Test fixture refactoring patterns: `docs/testing/refactoring-patterns/scenario-fixtures-for-causal-initial-state.md`
- Parallel main-application integration tests: #1419
- E2E fixed host ports as a self-hosted runner concurrency limit: #2386
- Native-process port `0` discovery precedent: #2132 `native-shutdown-test-plan.md`
