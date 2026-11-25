---
schema-version: 1
doc-type: issue
issue-type: bug
status: planned
priority: p1
epic: null
github-issue: 3051
spec-path: docs/issues/open/3051-random-expiring-key-duration-overflow-panic/ISSUE.md
branch: "3051-random-expiring-key-duration-overflow-panic-spec"
related-pr: null
last-updated-utc: "2026-11-25 14:18"
semantic-links:
  skill-links:
    - create-issue
    - fix-bug
    - write-unit-test
  related-artifacts:
    - .claude/skills/fix-bug/SKILL.md
    - .claude/skills/create-issue/SKILL.md
    - packages/tracker-core/src/authentication/key/mod.rs
    - packages/tracker-core/src/authentication/handler.rs
    - packages/tracker-core/src/error.rs
    - packages/rest-api-runtime-adapter/src/v1/adapters/auth_key.rs
    - packages/axum-rest-api-server/tests/server/v1/contract/context/auth_key.rs
---

<!-- skill-link: create-issue -->

# Issue #3051 - Return an Error, Not a Panic, When a Random Expiring Key Duration Overflows the Clock

Related: #1347 (EPIC, package testing) and #2902 (tracker-core unit tests), where the bug was found. Not a
subissue of either: the fix is independent of the test rollout.

## Goal

Creating a random expiring authentication key with a `seconds_valid` that overflows the clock returns
`DurationOverflow` through every public path (tracker-core API and both REST endpoints) instead of
panicking.

## Background

### What happens

1. An admin calls `POST /api/v1/keys` with `{"key": null, "seconds_valid": 18446744073709551615}`, or the
   generate-key endpoint `POST /api/v1/key/18446744073709551615`.
2. The REST adapter reaches `KeysHandler::generate_expiring_peer_key(Some(Duration::from_secs(u64::MAX)))`
   (directly, or through the random branch of `KeysHandler::add_peer_key`).
3. That calls `key::generate_key`, which computes the expiry with `CurrentClock::now_add(&lifetime).unwrap()`.
4. `now_add` returns `None` when `now + lifetime` does not fit, so the `unwrap()` panics in the request task.
   The client receives no response (the connection is dropped). The tracker process survives, because the
   runtime isolates the task panic.

### Which contract it violates

The pre-generated-key branch of `add_peer_key` already checks `CurrentClock::now_add` first and returns
`PeerKeyError::DurationOverflow`, which the REST layer maps to `AuthKeyError::DurationOverflow` and a
`400 Bad Request` ("invalid auth key duration"). The same input must get the same answer on the random path.
`add_peer_key` documents that it returns an error when "the provided key duration exceeds the maximum
allowed value", and the doc of `generate_key` admits the panic as "an extremely unlikely event", which a
client-controlled `u64` makes trivial to trigger.

### Impact

Availability of an authenticated endpoint: only a holder of an admin token can reach it, so this is not an
open attack surface. Observable today: an empty reply and a panic line in the tracker log per request. The
generate-key endpoint would also map any error to `AuthKeyError::Database`, so after the panic is removed it
would still answer with the wrong error kind unless fixed too.

### Reproduction

**Reproduced** against a real tracker and at the unit seam on 2026-11-25. See
[`manual-verification-evidence.md`](manual-verification-evidence.md).

## Decision

Fix at the seam (maintainer decision, 2026-11-25). Alternatives considered: pre-checking in each caller (leaves
the panicking `pub` functions reachable by other callers and duplicates the check), and changing only
`generate_expiring_peer_key` (leaves `generate_key` panicking).

Public API changes in `torrust-tracker-core` (allowed: 4.0.0 is pending; they are breaking and go in the changelog
note of the PR):

| Item                                      | Before                                                | After                                                    |
| ----------------------------------------- | ----------------------------------------------------- | -------------------------------------------------------- |
| `key::generate_key`                       | `-> PeerKey` (panics on overflow)                     | `-> Result<PeerKey, key::DurationOverflow>`              |
| `KeysHandler::generate_expiring_peer_key` | `-> Result<PeerKey, databases::error::Error>`         | `-> Result<PeerKey, PeerKeyError>`                       |
| `key::DurationOverflow` (new)             | n/a                                                   | `pub struct DurationOverflow { pub seconds_valid: u64 }` |

`key::DurationOverflow` is a dedicated error type (`thiserror`, same message as
`PeerKeyError::DurationOverflow`), so `generate_key`'s signature states its single failure mode and the
low-level `key` module does not depend on the handler-level `PeerKeyError`. `impl From<DurationOverflow> for
PeerKeyError` lets the handler use `?`. `seconds_valid` is `lifetime.as_secs()`: exact for the REST inputs,
which are whole seconds; the doc says so.

The generation failure of `generate_expiring_peer_key` maps to `PeerKeyError::DurationOverflow`; a database
failure maps to `PeerKeyError::DatabaseError`. The REST `generate_key` adapter reuses `map_peer_key_error`, so
both REST paths yield `AuthKeyError::DurationOverflow` and the existing `400` response.

## Scope

### In Scope

- `key::generate_key`, `key::DurationOverflow` and the `From` impl; update the `# Panics` doc, the doctests and
  the `#[cfg(test)]` helpers in `key/mod.rs`.
- `KeysHandler::generate_expiring_peer_key` and `add_peer_key` (random branch); the other callers: tests in
  `tracker-core`, `axum-http-server`, `axum-rest-api-server`, `http-core` and `persistence-benchmark`.
- `rest-api-runtime-adapter` `generate_key` error mapping.
- Regression tests listed below.

### Out of Scope

- Splitting or renaming `PeerKeyError` (maintainer remark: it is generic and does not say whether a failure is a
  parse, construction or usage error). Candidate follow-up issue; not decided here.
- Changing the REST response bodies or status codes.
- Any other `unwrap()` in the workspace.

## Architectural Decisions

- Related ADRs: none.
- ADRs to create: None known. The seam decision above is recorded here.

## Design and Ownership Review

Not applicable.

## Bug-Fix Process

Follows [fix-bug](../../../../.claude/skills/fix-bug/SKILL.md):

1. Analysis: the `unwrap()` in `key::generate_key` (hypothesis above, confirmed by the panic location).
2. Reproduction: real tracker plus a temporary unit test, recorded in
   [`manual-verification-evidence.md`](manual-verification-evidence.md) (**Reproduced**).
3. Regression-test boundary: see below.
4. Red run recorded before the fix (T2).
5. Fix (T3), then green run and like-for-like recheck against a real tracker after the last production change
   (T4).

## Regression Test Strategy

Smallest deterministic maintained tests, all unit or contract level, none need a network beyond the existing test
servers:

| Boundary                               | Test (name sketch)                                                                          | Why this boundary                                              |
| -------------------------------------- | ------------------------------------------------------------------------------------------- | -------------------------------------------------------------- |
| `key::generate_key` (unit)             | `it_should_fail_generating_an_expiring_key_when_the_duration_overflows_the_clock`           | The causal seam: the `unwrap()`.                               |
| `KeysHandler::add_peer_key` (unit)     | `it_should_fail_adding_a_randomly_generated_key_when_the_key_duration_exceeds_the_maximum_duration` | Mirrors the existing pre-generated test next to it.            |
| `KeysHandler::generate_expiring_peer_key` (unit) | overflow returns `PeerKeyError::DurationOverflow`; a database failure still returns `DatabaseError` | The public API whose error type changes.                       |
| REST contract: `POST /api/v1/keys` random key | `400` with "invalid auth key duration" for `u64::MAX`                               | Reachable behavior; the adapter mapping has no unit tests today. |
| REST contract: generate-key endpoint   | the same `400` for `u64::MAX`                                                            | The second path, which today maps to `Database`.               |

The handler and contract tests are written first and run red against the unfixed code (the handler test panics;
the contract tests get an empty reply or the wrong error). The `generate_key` unit test does not compile until the
signature changes, so it is added with the fix and proven by mutate-then-restore (reintroduce the `unwrap()`).

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task                                                                                                         | Notes / Expected Output                                                                 |
| --- | ------ | ------------------------------------------------------------------------------------------------------------ | --------------------------------------------------------------------------------------- |
| T0  | DONE   | Analyze, reproduce against a real tracker and at the unit seam                                               | Evidence file sections V1 and V2.                                                       |
| T1  | TODO   | Write the handler and the two contract regression tests                                                      | Tests compile against the current API.                                                  |
| T2  | TODO   | Run them red and record output                                                                               | Handler test panics at `key/mod.rs`; contract tests fail. Output in the evidence file.   |
| T3  | TODO   | Fix: `DurationOverflow`, new signatures, callers, REST mapping, docs; add the `generate_key` unit test        | Mutate-then-restore proves the unit test.                                               |
| T4  | TODO   | Green run, `linter all`, like-for-like recheck against a real tracker                                        | Requests that dropped the connection now return `400`.                                  |
| T5  | TODO   | Review loop, completion review                                                                               | Checks green, reviews addressed.                                                        |

## Commit Points

| Task  | Coherent change set                                          | Commit policy                                        |
| ----- | ------------------------------------------------------------ | ---------------------------------------------------- |
| T1-T2 | Failing regression tests plus red evidence                   | Commit after focused validation and required review. |
| T3    | Production change, callers, the `generate_key` unit test      | Commit after focused validation and required review. |
| T4    | Green and recheck evidence                                   | Commit after focused validation and required review. |

Tests follow the `write-unit-test` skill; the test-design review is recorded in the PR description.

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/open/3051-random-expiring-key-duration-overflow-panic/ISSUE.md`
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue created and issue number added to this spec
- [ ] Spec-only PR merged into `develop` before implementation
- [ ] Implementation completed
- [ ] Automatic verification completed (`linter all`, relevant tests, and any pre-push checks)
- [ ] Manual verification scenarios executed and recorded in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] Implementation completion review recorded
- [ ] Reviewer validated acceptance criteria and updated checkboxes
- [ ] Committer verified spec progress is up to date before commit
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-11-25 14:15 UTC - Claude Code (Claude Sonnet 5.5) - Drafted the spec, reproduced the bug, maintainer chose the fix at the seam with a dedicated `DurationOverflow` type - [evidence](manual-verification-evidence.md)
- 2026-11-25 14:20 UTC - Claude Code (Claude Sonnet 5.5) - Maintainer approved the spec; GitHub issue #3051 created; spec-only PR next

## Acceptance Criteria

- [ ] AC1: `key::generate_key(Some(d))` returns `Err(DurationOverflow)` when `now + d` overflows, and never panics.
- [ ] AC2: `KeysHandler::add_peer_key` with `opt_key: None` and `opt_seconds_valid: Some(u64::MAX)` returns
      `PeerKeyError::DurationOverflow`.
- [ ] AC3: `KeysHandler::generate_expiring_peer_key` returns `DurationOverflow` on overflow and `DatabaseError` on a
      database failure.
- [ ] AC4: `POST /api/v1/keys` (random key) and the generate-key endpoint answer `400` with "invalid auth key
      duration" for `u64::MAX`.
- [ ] AC5: The API change is stated in the PR description.
- [ ] `linter all` exits with code `0`
- [ ] Relevant tests pass
- [ ] Manual verification scenarios are executed and documented in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria are re-reviewed after implementation and reflect actual behavior
- [ ] Documentation is updated when behavior/workflow changes

## Verification Plan

### Automatic Checks

- `linter all`; `cargo test -p torrust-tracker-core -p torrust-tracker-rest-api-runtime-adapter -p torrust-tracker-axum-rest-api-server`
  and the other packages whose callers change; the pre-commit and pre-push checks.

### Manual Verification Scenarios

| ID  | Scenario                                       | Human-oriented command/steps                                                                  | Expected Result                                          | Status | Evidence                                      |
| --- | ---------------------------------------------- | --------------------------------------------------------------------------------------------- | -------------------------------------------------------- | ------ | --------------------------------------------- |
| M1  | Before the fix: three requests on a real tracker | `curl` the two random-key requests and the pre-generated control (see evidence file)          | Empty replies for the random paths, `400` for the control | DONE   | `manual-verification-evidence.md` section V1   |
| M2  | Before the fix: handler test                   | Temporary test, `cargo test -p torrust-tracker-core`                                          | Panic at `key/mod.rs:111`                                | DONE   | `manual-verification-evidence.md` section V2   |
| M3  | After the fix: the same requests as M1         | The same commands against the fixed build                                                     | `400` "invalid auth key duration" for all three          | TODO   | `manual-verification-evidence.md` section V3   |

The M3 recheck runs after the last production change, including changes made for review findings.

### Acceptance Verification

| AC ID | Status | Evidence |
| ----- | ------ | -------- |
| AC1   | TODO   |          |
| AC2   | TODO   |          |
| AC3   | TODO   |          |
| AC4   | TODO   |          |
| AC5   | TODO   |          |

## Risks and Trade-offs

- Breaking change in two `pub` functions of `torrust-tracker-core`: acceptable before 4.0.0, called out in the PR.
  Mitigation: all in-workspace callers are updated in the same PR; the compiler finds the rest.
- `DurationOverflow` duplicates `PeerKeyError::DurationOverflow`. Mitigation: the `From` impl; a follow-up on
  `PeerKeyError` can merge them.

## Implementation Completion Review

- Retrospective: `Not yet assessed`

## References

- Related issues: #1347, #2902
- Related ADRs: none
