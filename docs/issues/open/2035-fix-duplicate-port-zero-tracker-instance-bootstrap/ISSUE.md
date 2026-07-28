---
doc-type: issue
issue-type: bug
status: open
priority: p1
github-issue: 2035
spec-path: docs/issues/open/2035-fix-duplicate-port-zero-tracker-instance-bootstrap/ISSUE.md
branch: 2035-fix-duplicate-port-zero-tracker-instance-bootstrap
related-pr: null
last-updated-utc: 2026-07-28 13:06
semantic-links:
  skill-links:
    - write-unit-test
  related-artifacts:
    - src/container.rs
    - src/app.rs
    - docs/issues/open/1419-allow-multiple-integration-tests-at-main-app-level/ISSUE.md
    - evidence.md
  related-issues:
    - 1419
---

# Issue #2035 - Fix Duplicate Port-Zero Tracker Instance Bootstrap

## Goal

Start every configured HTTP and UDP tracker instance with its own configuration, including when
multiple same-protocol blocks use the same configured port-zero bind address.

## Background

`AppContainer` stores HTTP and UDP instance containers in `HashMap<SocketAddr, _>`, keyed by each
configuration block's `bind_address`. `HashMap::insert` replaces the previous value for an equal
key. Consequently, two HTTP tracker blocks both configured as `0.0.0.0:0` leave only the later
container in the map.

Application startup then iterates both configuration blocks and looks up a container using the
same configured address. Both services start using the surviving later configuration, even though
the operating system gives each listener a distinct final port. The same defect exists for UDP
trackers. This can silently apply the wrong per-instance behavior, for example
`tracker_usage_statistics`, TLS, or network settings.

The local reproduction is recorded in [evidence.md](evidence.md).

## Scope

### In Scope

- Preserve each configured HTTP and UDP tracker instance even when configured bind addresses are equal.
- Replace address-keyed instance-container storage with an order-preserving representation aligned
  with configuration entries, or an equivalent stable configuration-instance identifier.
- Start each configured HTTP and UDP instance with its matching container.
- Include the configuration instance index in HTTP and UDP bootstrap lifecycle logs, including
  events that report configured and final bound addresses.
- Add regressions with repeated `0.0.0.0:0` blocks whose configuration differs.

### Out of Scope

- Runtime registry metadata or health-check API changes.
- Public endpoint, proxy, or DNS configuration.
- User-supplied persistent service IDs in configuration.

## Implementation Plan

| ID  | Status | Task                                   | Notes / Expected Output                                                                                |
| --- | ------ | -------------------------------------- | ------------------------------------------------------------------------------------------------------ |
| T1  | DONE   | Add failing HTTP bootstrap regression  | Ignored stats-contract regression records the current `2 != 1` failure.                                |
| T2  | TODO   | Add failing UDP bootstrap regression   | Same identity preservation for UDP instances.                                                          |
| T3  | TODO   | Replace address-keyed container lookup | Startup aligns each configuration entry with its own initialized container.                            |
| T4  | TODO   | Remove obsolete address lookup API     | No bootstrap path relies on configured `SocketAddr` uniqueness.                                        |
| T5  | TODO   | Correlate bootstrap lifecycle logs     | Every HTTP and UDP lifecycle event that emits a configured or final binding includes `instance_index`. |
| T6  | TODO   | Run focused and workspace validation   | Record before/after evidence in this issue folder.                                                     |

## Progress Tracking

### Workflow Checkpoints

- [x] Specification drafted and approved by user/maintainer
- [x] GitHub issue created: #2035
- [ ] Implementation completed
- [ ] Automatic verification completed (`linter all`, relevant tests)
- [ ] Manual verification scenarios executed and recorded
- [ ] Acceptance criteria reviewed after implementation
- [ ] Issue closed and specification moved to `docs/issues/closed/`

### Progress Log

- 2026-07-28 14:51 UTC - agent - User-approved specification promoted to GitHub issue #2035;
  the ignored HTTP stats-contract regression and its current `2 != 1` failure are recorded in
  [evidence.md](evidence.md).

## Acceptance Criteria

- [ ] AC1: Two HTTP tracker blocks with the same `0.0.0.0:0` binding each start with their own configuration.
- [ ] AC2: Two UDP tracker blocks with the same `0.0.0.0:0` binding each start with their own configuration.
- [ ] AC3: Bootstrap does not use configured `SocketAddr` as a unique instance identity.
- [ ] AC4: HTTP and UDP startup logs include the configuration `instance_index`, allowing logs
      with duplicate configured addresses to be correlated with their source configuration block.
- [ ] AC5: Focused HTTP, UDP, and application bootstrap tests pass.
- [ ] AC6: `linter all` exits with code `0`.

## Verification Plan

### Automatic Checks

- Focused regression tests for `AppContainer` and startup jobs.
- `cargo test --test stats --test scaffold` after the runtime-registry prerequisite lands.
- `linter all`.

### Manual Verification Scenarios

| ID  | Scenario                                                                            | Expected Result                                                          | Status | Evidence                   |
| --- | ----------------------------------------------------------------------------------- | ------------------------------------------------------------------------ | ------ | -------------------------- |
| M1  | Start two HTTP trackers with identical `0.0.0.0:0` bindings and different settings. | Each listener retains the settings from its own configuration block.     | TODO   | [evidence.md](evidence.md) |
| M2  | Repeat M1 for UDP trackers.                                                         | Each UDP listener retains the settings from its own configuration block. | TODO   |                            |

## References

- Issue #1419: [main-application integration tests](../../open/1419-allow-multiple-integration-tests-at-main-app-level/ISSUE.md)
- [Runtime registry investigation](../../open/1419-allow-multiple-integration-tests-at-main-app-level/investigation-registar-and-health-check.md)
- Feature #2036: [add runtime service registry metadata](../2036-add-runtime-service-registry-metadata/ISSUE.md)
