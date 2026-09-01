---
doc-type: issue
issue-type: bug
status: done
priority: p2
epic: 1978
github-issue: 2083
spec-path: docs/issues/closed/2083-1978-move-max-connection-id-errors-per-ip-to-udp-tracker-server.md
branch: "2083-move-max-connection-id-errors-per-ip-to-udp-tracker-server"
related-pr: null
depends-on: null
blocks: 1980
last-updated-utc: 2026-09-01 10:27
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - .github/skills/dev/planning/create-issue/SKILL.md
    - docs/issues/closed/1978-configuration-overhaul-epic/EPIC.md
    - packages/configuration/docs/migrate-v2-to-v3.md
    - docs/issues/closed/2067-1978-analyze-flat-service-configuration/max-connection-id-errors-per-ip-bug.md
    - docs/adrs/20260727180000_shared_services_across_tracker_instances.md
    - packages/configuration/src/v3_0_0/udp_tracker.rs
    - packages/configuration/src/v3_0_0/udp_tracker_server.rs
    - packages/udp-core/src/container.rs
    - src/container.rs
---

<!-- skill-link: create-issue -->

# Issue #2083 - Move UDP connection-ID error limit to shared server configuration

> **EPIC position**: Bug-fix subissue of EPIC #1978. This issue corrects
> the v3 shared UDP `BanService` configuration boundary identified during #2067.
> It must remain separate from #2067, which is analysis-only, and must precede
> #1980, which activates the corrected v3 configuration in production.

## Goal

Make `max_connection_id_errors_per_ip` an unambiguous global UDP-server policy.
Every UDP listener in one tracker process must use the one limit declared in the
shared `[udp_tracker_server]` configuration, and the result must not depend on
the order of `[[udp_trackers]]` entries.

## Background

`max_connection_id_errors_per_ip` is currently declared on each UDP listener in
both the v2 and v3 configuration schemas. That placement implies each listener
can choose its own threshold. The runtime instead constructs one shared
`BanService` for every UDP listener in the process. `AppContainer` silently
selects the first configured UDP listener's threshold and passes it to
`UdpTrackerCoreServices::initialize_from`; all listener containers then clone
the same `Arc<RwLock<BanService>>`.

For example, this configuration appears to assign different policies:

```toml
[[udp_trackers]]
bind_address = "127.0.0.1:6969"
max_connection_id_errors_per_ip = 1

[[udp_trackers]]
bind_address = "127.0.0.1:6970"
max_connection_id_errors_per_ip = 100
```

In reality, both listeners use `1`. Reversing the entries changes the process-wide
security threshold to `100`, without changing the intended shared-service design.
This is a configuration-model bug: neither listener-specific policy nor explicit
shared policy is represented honestly.

ADR-20260727180000 establishes that IP banning is shared deliberately so an
attacker cannot multiply the allowed invalid-request budget by targeting multiple
UDP listeners. Settings that govern that shared service must therefore be global.
The existing global `connection_id_validation` policy is the direct precedent.

The application currently consumes v2 aliases. This issue corrects the v3 schema
and its documentation without changing the supported v2 schema or introducing a
temporary dual-schema production path. #1980 then migrates production consumers
to v3, updates the construction paths, and wires the corrected global value into
the application container.

## Scope

### In Scope

- Move `max_connection_id_errors_per_ip` from v3 `UdpTracker` to v3
  `UdpTrackerServer`.
- Define one documented default for the global limit that preserves the existing
  default threshold of `10`.
- Remove the per-listener v3 field, its default helper, serialization behaviour,
  fixtures, constructors, examples, and documentation.
- Preserve the intentionally shared `BanService` architecture; do not create
  separate ban services for individual UDP listeners.
- Add schema coverage proving that listener declaration order cannot select the
  threshold because v3 declares one global limit.
- Update the v2-to-v3 migration guide with an explicit before/after example and
  explain that repeated per-listener values are no longer accepted in v3.
- Update v3 defaults, fixtures, examples, and user documentation affected by the
  field move.

### Out of Scope

- Implementing the fix as part of #2067 or changing that analysis-only issue's
  conclusions.
- Changing the v2 schema or adding a v2 compatibility fallback for the moved
  field.
- Changing the default threshold value, BanService counting algorithm, ban-reset
  interval, connection-cookie validation policy, or ban enforcement semantics.
- Creating per-listener `BanService` instances or allowing mixed error limits in
  one process.
- Redesigning the broader UDP configuration model or a flat service collection.
- Changing production `AppContainer` wiring, default v2 configuration, or the
  active v2 runtime path; #1980 performs that v3 activation.

## Architectural Decisions

### Decision 1: Represent the limit once on `UdpTrackerServer`

Add `max_connection_id_errors_per_ip` to v3 `UdpTrackerServer`, beside
`ip_bans_reset_interval_in_secs` and `connection_id_validation`. Those fields all
govern the shared UDP ban service. `UdpTracker` retains only listener-specific
values such as its bind address, cookie lifetime, public URL, and network
topology.

### Decision 2: Preserve one shared BanService

This issue changes the configuration boundary, not the service lifetime. One
shared BanService keeps the error budget process-wide and prevents an attacker
from multiplying it by the number of configured listeners.

### Decision 3: Do not validate repeated per-listener values

The rejected interim option is to retain the field on every listener and require
all entries to repeat the same number. That would prevent contradictory input but
would still duplicate a global policy, retain an ambiguous public schema, and add
unnecessary consistency validation. The policy must be declared once.

### Decision 4: Correct v3 before activating it in production

This v3-only field move must land before #1980 migrates production consumers to
v3. That sequence makes the global configuration model complete before runtime
activation and avoids a temporary production implementation based on a
per-listener v3 field. The v2-to-v3 migration guide is the compatibility
contract for operators moving from the currently active v2 field placement.

- Related ADRs: `docs/adrs/20260727180000_shared_services_across_tracker_instances.md`
- ADRs to create: None known. Create one during implementation only if the
  shared-service architecture or configuration-version lifecycle changes beyond
  these established decisions.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task                                             | Notes / Expected Output                                                                                                                                                                                                               |
| --- | ------ | ------------------------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| T1  | DONE   | Confirm the pre-#1980 v3 configuration boundary  | Inventoried v3 schema, generated defaults, schema tests, migration guide, and #1980 runtime handoff; active v2 files remain untouched.                                                                                                |
| T2  | DONE   | Move the v3 configuration field                  | `UdpTrackerServer` owns the global default and serde field; `UdpTracker` no longer exposes it.                                                                                                                                        |
| T3  | DONE   | Update v3 fixtures, examples, and documentation  | Updated generated v3 defaults and the v2-to-v3 migration guide; active v2 defaults remain unchanged.                                                                                                                                  |
| T4  | DONE   | Define #1980 production-wiring handoff           | #1980 already owns T12–T13: migrate constructors, read the one v3 `udp_tracker_server` limit in `AppContainer`, pass it once to `UdpTrackerCoreServices`, and prove runtime enforcement; no production v2 path changes in this issue. |
| T5  | DONE   | Add schema regression tests                      | Added default, explicit round-trip, two-listener global configuration, and obsolete listener-field rejection coverage. Direct-construction and runtime cross-listener enforcement coverage remains in #1980.                          |
| T6  | DONE   | Update migration and configuration documentation | Documented the v2 per-listener to v3 global move and updated generated v3 defaults.                                                                                                                                                   |
| T7  | DONE   | Run automatic and manual verification            | Focused, full workspace, pre-commit, and pre-push checks passed; manual schema scenarios are recorded below.                                                                                                                          |
| T8  | DONE   | Re-review acceptance criteria                    | Re-reviewed after independent audit; AC1–AC5 are satisfied and AC6 remains correctly deferred to #1980.                                                                                                                               |

## Progress Tracking

### Workflow Checkpoints

- [x] Bug documented separately from #2067 implementation work
- [x] Draft specification created in `docs/issues/drafts/`
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue #2083 created and issue number added to this spec
- [x] Linked as a subissue of EPIC #1978 in GitHub and in the EPIC specification
- [x] Spec moved to `docs/issues/open/` after approval
- [x] V3 schema correction completed before #1980
- [x] #1980 production wiring handoff recorded and accepted
- [ ] (Optional, recommended for this cross-cutting bug) Spec-only PR merged into `develop` before implementation
- [x] Implementation completed
- [x] Automatic verification completed (`linter all`, relevant tests, and any pre-push checks)
- [x] Manual verification scenarios executed and recorded (status + evidence)
- [x] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] Reviewer validated acceptance criteria and updated checkboxes
- [ ] Committer verified spec progress is up to date before commit
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

Append one line per meaningful update.

- 2026-08-24 00:00 UTC - GitHub Copilot - Drafted a dedicated bug specification from the confirmed finding in #2067; it corrects v3 before #1980 activates v3 in the production runtime.
- 2026-08-24 11:04 UTC - GitHub Copilot/User - User approved the draft; created GitHub issue #2083 and linked it as a native subissue of EPIC #1978.
- 2026-08-24 11:04 UTC - GitHub Copilot/User - User confirmed that #1980 must own the production-wiring handoff. Verified that its T12–T13 and AC8–AC9 already explicitly cover the required `AppContainer` migration and order-independent two-listener runtime test.
- 2026-08-24 15:00 UTC - GitHub Copilot - Moved the v3 field from `UdpTracker` to `UdpTrackerServer`, updated generated defaults and migration documentation, and added focused schema regression tests. `cargo test -p torrust-tracker-configuration` passed (109 tests).
- 2026-08-24 15:00 UTC - GitHub Copilot - Independent review confirmed the v3 schema change is correctly scoped. Corrected the premature AC6 completion claim because #1980 production wiring remains pending. Pre-commit, full workspace, and pre-push verification subsequently passed.

## Acceptance Criteria

- [ ] AC1: V3 `UdpTrackerServer` exposes one documented
      `max_connection_id_errors_per_ip` setting with default `10`.
- [ ] AC2: V3 `UdpTracker` no longer exposes, serializes, or accepts
      `max_connection_id_errors_per_ip` as a per-listener field.
- [ ] AC3: Reordering `[[udp_trackers]]` entries cannot change a v3 configured
      global error threshold because the field is declared only once.
- [ ] AC4: V3 defaults, test fixtures, and examples contain no
      obsolete per-listener setting.
- [ ] AC5: The v2-to-v3 migration guide tells operators to move the field from
      each `[[udp_trackers]]` entry to `[udp_tracker_server]` and explains the
      shared-service rationale.
- [ ] AC6: #1980 explicitly records and implements the remaining production
      wiring: read the v3 server-wide limit in `AppContainer` and initialize the
      shared `BanService` with it.
- [ ] `linter all` exits with code `0`.
- [ ] Relevant focused, integration, workspace, and pre-push tests pass.
- [ ] Manual verification scenarios are executed and documented (status + evidence).
- [ ] Acceptance criteria are re-reviewed after implementation and reflect actual behaviour.

## Verification Plan

Define verification before implementation starts and execute it before closing the issue.

### Automatic Checks

- `linter all`
- `cargo test -p torrust-tracker-configuration`
- `cargo test --workspace --tests --benches --examples --all-targets --all-features`
- `./contrib/dev-tools/git/hooks/pre-push.sh` when applicable

Required focused coverage:

- A missing global field deserializes to `10`.
- An explicit global field deserializes and serializes correctly.
- V3 rejects `max_connection_id_errors_per_ip` inside `[[udp_trackers]]`.

Run Cargo checks with the repository's supported Rust 1.88-or-newer toolchain.
If the environment has no configured default Rust toolchain, complete the
documented development-environment setup before recording verification results.

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID  | Scenario                                     | Command/Steps                                                                                                                                                                                                            | Expected Result                                                                                                                                                    | Status | Evidence                                                                                                                                       |
| --- | -------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------ | ------ | ---------------------------------------------------------------------------------------------------------------------------------------------- |
| M1  | Inspect the corrected v3 configuration shape | Deserialize a v3 fixture with two `[[udp_trackers]]` entries and `[udp_tracker_server] max_connection_id_errors_per_ip = 2` in `torrust-tracker-configuration` tests. Run `cargo test -p torrust-tracker-configuration`. | The threshold is accepted only in `[udp_tracker_server]`; both listener entries remain free of the global setting.                                                 | DONE   | `v3_0_0::tests::configuration_should_apply_one_global_connection_id_error_limit_to_multiple_udp_trackers`; 2026-08-24 focused test run passed. |
| M2  | Reject obsolete listener configuration       | Add `max_connection_id_errors_per_ip = 2` inside a v3 `[[udp_trackers]]` block and deserialize it with `torrust-tracker-configuration` configuration tests.                                                              | Loading is rejected as an unknown `UdpTracker` field; the migration guide supplies the correct `[udp_tracker_server]` placement.                                   | DONE   | `v3_0_0::tests::configuration_should_reject_a_listener_scoped_connection_id_error_limit`; 2026-08-24 focused test run passed.                  |
| M3  | Verify #1980 runtime-test handoff            | Review the #1980 implementation plan and acceptance criteria after updating them for the production container handoff.                                                                                                   | #1980 explicitly owns the constructor migration, cross-listener runtime test, and production `AppContainer` wiring required to activate this corrected v3 setting. | DONE   | #1980 T12–T13, AC8–AC9, and M4 already record the required ownership.                                                                          |

Notes:

- The cross-listener protocol-level test requires the production v3 startup path
  and therefore belongs to #1980. It must use one bound UDP socket to send
  deliberately invalid connection IDs to two listener addresses.
- Record the exact configuration, commands, and test output
  in the Evidence column or an issue-local artifact.
- If a scenario fails, record the failure and diagnosis in the progress log
  before proceeding.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence                                                                                                                                                       |
| ----- | ---------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| AC1   | DONE                   | `UdpTrackerServer` declares the documented, serde-defaulted global field with default `10`; focused configuration tests passed.                                |
| AC2   | DONE                   | Removed the field from v3 `UdpTracker`; aggregate schema test rejects the obsolete listener-scoped key.                                                        |
| AC3   | DONE                   | A two-listener configuration accepts one server-wide value; no listener field remains to select by order. Runtime enforcement is explicitly deferred to #1980. |
| AC4   | DONE                   | Updated generated v3 default TOML and schema tests; no v3 listener-scoped value remains.                                                                       |
| AC5   | DONE                   | Migration guide includes v2/v3 before-and-after TOML, the shared-service rationale, default, and rejection behavior.                                           |
| AC6   | TODO                   | #1980 T12–T13, AC8–AC9, and M4 explicitly own the remaining production migration and runtime validation; its implementation is pending.                        |

## Risks and Trade-offs

- **Breaking v3 configuration change**: Existing early v3 adopters may repeat
  the field in listener blocks. Mitigation: correct v3 before #1980 activation,
  reject obsolete fields through `deny_unknown_fields`, and provide a precise
  migration example.
- **Incomplete propagation paths**: Direct v3 container constructors, examples,
  and test helpers may still expect the old field. Mitigation: inventory all
  v3 uses before changing the schema and compile/test every affected package.
- **Deferred production activation**: Schema tests cannot prove the active v2
  runtime is fixed. Mitigation: #1980 owns the production container wiring and
  cross-listener runtime test as an explicit prerequisite to closing its work.
- **Security regression during activation**: An accidental fallback to the first
  listener could retain order-dependent behaviour. Mitigation: remove the old
  field entirely from v3 and require #1980 runtime coverage using both listener
  orders.
- **Operator surprise**: A global threshold removes the appearance of
  per-listener tuning. Mitigation: document that independent thresholds conflict
  with the deliberate, shared security boundary and would require a separately
  approved architecture change.

## References

- Parent EPIC: #1978
- Source analysis issue: #2067
- Blocks: #1980
- Confirmed bug record: `docs/issues/closed/2067-1978-analyze-flat-service-configuration/max-connection-id-errors-per-ip-bug.md`
- Shared-services rationale: `docs/adrs/20260727180000_shared_services_across_tracker_instances.md`
- Existing global-policy precedent: #1136
- V2-to-v3 migration guide: `packages/configuration/docs/migrate-v2-to-v3.md`
