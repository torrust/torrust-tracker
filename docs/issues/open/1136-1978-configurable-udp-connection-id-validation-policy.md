---
doc-type: issue
issue-type: enhancement
status: planned
priority: p2
github-issue: 1136
spec-path: docs/issues/open/1136-1978-configurable-udp-connection-id-validation-policy.md
branch: "1136-connection-id-validation-policy"
related-pr: 2002
last-updated-utc: 2026-07-20 12:32
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - docs/issues/open/1978-configuration-overhaul-epic.md
    - docs/issues/open/1453-1978-ip-bans-reset-interval-configurable.md
    - packages/configuration/src/v3_0_0/udp_tracker.rs
    - packages/udp-core/src/connection_cookie.rs
    - packages/udp-core/src/services/announce.rs
    - packages/udp-core/src/services/scrape.rs
    - packages/udp-server/src/server/processor.rs
    - packages/udp-server/tests/server/contract.rs
---

# Issue #1136 - Add configurable UDP connection ID validation policy

> **EPIC position**: Subissue 7 of 11 in EPIC #1978, immediately after
> #1453. It is not functionally dependent on #1453, but implementing #1453 first
> establishes the global ban-cleanup configuration boundary before this issue
> adds a per-listener validation policy.

## Goal

Allow operators to disable UDP connection ID validation for a specific UDP tracker
listener when compatibility with non-compliant clients is more important than the
anti-spoofing and replay protection provided by BEP 15 connection IDs.

Strict validation remains the secure default.

## Background

BEP 15 clients first obtain a connection ID from the tracker and then include it in
announce and scrape requests. Torrust generates a stateless encrypted cookie from the
client socket address fingerprint and issue time. Validation accepts only decoded issue
times inside a narrow range determined by `cookie_lifetime`.

Some clients reuse expired connection IDs. Issue #1136 originally proposed ignoring
connection ID expiration, while a later discussion suggested a Boolean option that
would disable validation entirely.

The existing per-listener `cookie_lifetime` setting can already increase the accepted
time window. It does not provide an explicit way to support clients that reuse IDs
indefinitely.

### Security constraint

An expiration-only bypass is not a safe middle ground with the current cookie design.
The cookie uses non-authenticated encryption, and the fingerprint is mixed into the
cookie through wrapping arithmetic rather than a MAC. The narrow timestamp window is
therefore part of what makes arbitrary or wrong-fingerprint connection IDs unlikely to
validate.

A random or wrong-fingerprint connection ID can decode to a normal timestamp classified
as expired. Accepting every `ValueExpired` result would consequently accept more than
known, previously valid but expired IDs. It would weaken validation without making that
trade-off obvious to operators.

For that reason, this specification exposes only two honest policies:

- `strict`: preserve all existing validation.
- `disabled`: skip connection ID validation for announce and scrape requests.

## Design Decisions

### Decision 1: Use an enum, not a Boolean

Add a public `ConnectionIdValidationPolicy` enum to the v3 UDP tracker configuration:

```rust,ignore
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone, Default)]
#[serde(rename_all = "kebab-case")]
pub enum ConnectionIdValidationPolicy {
    #[default]
    Strict,
    Disabled,
}
```

An enum communicates that this is a security policy and leaves room for a future mode
only if a safe, precisely defined alternative becomes available.

### Decision 2: Configure each UDP listener independently

Add the following field to `v3_0_0::udp_tracker::UdpTracker`:

```rust,ignore
pub connection_id_validation: ConnectionIdValidationPolicy,
```

Example configuration:

```toml
[[udp_trackers]]
bind_address = "0.0.0.0:6969"
connection_id_validation = "strict"

[[udp_trackers]]
bind_address = "127.0.0.1:6970"
connection_id_validation = "disabled"
```

Per-listener placement allows an operator to expose a strict public listener while
isolating a compatibility listener through network controls.

### Decision 3: Preserve strict validation by default

When the field is omitted, behavior is identical to the current implementation:

- Reject non-normal decoded values.
- Reject expired values.
- Reject future-dated values.
- Reject values that fail when checked against the client socket fingerprint and valid
  time range.
- Emit the existing connection-cookie error and banning events.

### Decision 4: Define `disabled` precisely

When `connection_id_validation = "disabled"`:

- Announce and scrape handlers do not call the connection cookie validator.
- The connection ID value is ignored, including malformed, expired, future-dated, and
  wrong-fingerprint values that can be represented by the protocol type.
- Requests continue through all non-cookie validation, authorization, and tracker policy
  checks.
- The connect action is unchanged and continues issuing connection IDs.
- No connection-cookie error, connection-ID error metric, or IP-ban counter increment is
  produced for the bypassed check.
- The listener logs a warning at startup stating that connection ID validation is
  disabled and UDP anti-spoofing/replay protection is reduced.

### Decision 5: Apply the change only to schema v3

The new enum and field are added only under `packages/configuration/src/v3_0_0/`.
Schema v2 and its global re-exports remain unchanged. Migration of application consumers
and `share/default/config/` to schema v3 remains part of final cleanup issue #1980.

## Scope

### In Scope

- Add `ConnectionIdValidationPolicy` with `strict` and `disabled` variants to schema v3
- Add a per-listener `connection_id_validation` field to `v3_0_0::UdpTracker`
- Default the policy to `strict`
- Propagate the policy from configuration through UDP server startup and request
  processing
- Apply the policy consistently to announce and scrape requests
- Preserve connect request behavior
- Preserve current cookie-error metrics and banning behavior in strict mode
- Suppress cookie-error metrics and ban increments when validation is disabled
- Emit a startup warning for each listener using the disabled policy
- Add configuration, unit, integration, and mixed-listener tests
- Document the security implications of the disabled policy

### Out of Scope

- Adding an expiration-only compatibility mode
- Changing the cookie generation or cryptographic algorithm
- Changing `cookie_lifetime` semantics or defaults
- Changing ban thresholds or cleanup scheduling (covered by #1453)
- Disabling authorization, whitelist, private tracker, or request-shape validation
- Adding the field to schema v2
- Switching application consumers or default configuration files to schema v3 (covered
  by #1980)

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task                                             | Notes / Expected Output                                                                |
| --- | ------ | ------------------------------------------------ | -------------------------------------------------------------------------------------- |
| T1  | TODO   | Add the v3 validation policy                     | Enum and per-listener field in `v3_0_0/udp_tracker.rs`; default is `strict`            |
| T2  | TODO   | Add configuration serialization tests            | Missing field defaults to strict; both string values round-trip                        |
| T3  | TODO   | Add shared policy-aware cookie authentication    | One UDP core boundary implements strict validation and the disabled bypass             |
| T4  | TODO   | Propagate policy through UDP server construction | Policy reaches request processing without global state                                 |
| T5  | TODO   | Apply the shared policy to announce and scrape   | Both request paths use the same authentication behavior                                |
| T6  | TODO   | Preserve observability and banning semantics     | Strict emits current events; disabled emits no cookie-error or ban-counter event       |
| T7  | TODO   | Warn when starting an insecure listener          | Warning identifies the affected UDP service binding                                    |
| T8  | TODO   | Add mixed-listener contract coverage             | Strict and disabled listeners behave independently in the same process                 |
| T9  | TODO   | Update v3 schema documentation and test fixtures | Do not modify v2 or active `share/default/config/` files                               |
| T10 | TODO   | Run automatic and manual verification            | Linters, focused tests, workspace tests, pre-push checks, and recorded manual evidence |

## Progress Tracking

### Workflow Checkpoints

- [x] Spec drafted in `docs/issues/drafts/`
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue already exists and issue number matches spec
- [x] GitHub issue title/body updated to match the approved specification
- [x] Issue linked as a subissue of EPIC #1978
- [x] EPIC #1978 local specification updated with the new ordering and dependency edge
- [x] Spec moved to `docs/issues/open/` after approval
- [ ] (Recommended) Spec-only PR merged into `develop` before implementation
- [ ] Implementation completed
- [ ] Automatic verification completed (`linter all`, relevant tests, and pre-push checks)
- [ ] Manual verification scenarios executed and recorded (status + evidence)
- [ ] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] Reviewer validated acceptance criteria and updated checkboxes
- [x] Committer verified spec progress is up to date before commit
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-07-20 11:52 UTC - agent - Drafted local specification for maintainer
  review; proposed secure-default per-listener `strict | disabled` policy
- 2026-07-20 11:52 UTC - maintainer - Approved the proposed design decisions
- 2026-07-20 12:12 UTC - agent - Promoted the approved specification and added
  #1136 to the local EPIC as subissue 7 of 11
- 2026-07-20 12:23 UTC - agent - Updated GitHub issue #1136, linked it to
  EPIC #1978, and verified its position immediately after #1453
- 2026-07-20 12:26 UTC - committer - Verified the specification progress and
  two-file commit scope before the spec-only commit
- 2026-07-20 12:32 UTC - agent - Opened spec-only PR #2002 against `develop`

## Acceptance Criteria

- [ ] AC1: Schema v3 exposes `ConnectionIdValidationPolicy` with exactly `strict`
      and `disabled` serialized values
- [ ] AC2: Every v3 UDP tracker listener has a `connection_id_validation` setting
- [ ] AC3: Omitting the setting defaults to `strict` and preserves current behavior
- [ ] AC4: Strict mode rejects non-normal, expired, future-dated, and
      wrong-fingerprint connection IDs for announce and scrape requests
- [ ] AC5: Disabled mode bypasses only connection ID validation for announce and scrape
- [ ] AC6: Connect requests continue issuing connection IDs in both modes
- [ ] AC7: Disabled mode does not emit connection-cookie error events, increment
      connection-ID error metrics, or increment IP-ban counters for the bypassed check
- [ ] AC8: A startup warning identifies each listener configured with disabled validation
- [ ] AC9: Strict and disabled listeners can run simultaneously without sharing policy
- [ ] AC10: Schema v2 behavior and public types remain unchanged
- [ ] AC11: Security implications and recommended network isolation are documented
- [ ] `linter all` exits with code `0`
- [ ] Relevant focused and workspace tests pass
- [ ] Pre-push checks pass
- [ ] Manual verification scenarios are executed and documented (status + evidence)
- [ ] Acceptance criteria are re-reviewed after implementation and reflect actual behavior

## Verification Plan

Define verification before implementation starts and execute it before closing the issue.

### Automatic Checks

- `linter all`
- `cargo test -p torrust-tracker-configuration`
- `cargo test -p torrust-tracker-udp-core`
- `cargo test -p torrust-tracker-udp-server`
- `cargo test --workspace --tests --benches --examples --all-targets --all-features`
- `./contrib/dev-tools/git/hooks/pre-push.sh`

Required focused coverage:

- Configuration default and TOML round-trip for both policy values
- Announce with valid, expired, future-dated, non-normal, and wrong-fingerprint IDs in
  strict mode
- Scrape with the same connection ID classes in strict mode
- Announce and scrape with arbitrary IDs in disabled mode
- Cookie-error metrics and ban counters in both modes
- Two simultaneous listeners using different policies

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID  | Scenario                                | Command/Steps                                                                                               | Expected Result                                                                                  | Status | Evidence |
| --- | --------------------------------------- | ----------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------ | ------ | -------- |
| M1  | Strict listener rejects an invalid ID   | Start a local strict UDP listener; send announce and scrape requests using an expired or zero connection ID | Requests receive the existing connection-ID error; error metrics and ban counters increase       | TODO   |          |
| M2  | Disabled listener accepts an invalid ID | Start a local disabled UDP listener; repeat the same announce and scrape requests                           | Requests pass cookie validation and continue through normal request handling; no ban increment   | TODO   |          |
| M3  | Mixed policies remain isolated          | Start strict and disabled listeners in one process; send the same invalid requests to both                  | Strict listener rejects them; disabled listener accepts them; neither listener changes the other | TODO   |          |
| M4  | Insecure mode is visible                | Start a listener with `connection_id_validation = "disabled"` and inspect startup logs                      | A warning identifies the listener and reduced anti-spoofing/replay protection                    | TODO   |          |

Notes:

- Manual verification is mandatory even when automated tests pass.
- Record commands, relevant logs, and observed metric/ban counter values in the Evidence
  column or a linked evidence artifact.
- If a scenario fails, record the failure and diagnosis in the progress log before
  proceeding.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| ----- | ---------------------- | -------- |
| AC1   | TODO                   |          |
| AC2   | TODO                   |          |
| AC3   | TODO                   |          |
| AC4   | TODO                   |          |
| AC5   | TODO                   |          |
| AC6   | TODO                   |          |
| AC7   | TODO                   |          |
| AC8   | TODO                   |          |
| AC9   | TODO                   |          |
| AC10  | TODO                   |          |
| AC11  | TODO                   |          |

## Risks and Trade-offs

- **Reduced spoofing and replay protection**: Disabled mode accepts arbitrary connection
  IDs for announce and scrape. Mitigation: strict remains the default, startup emits a
  warning, documentation recommends binding compatibility listeners to trusted networks
  or protecting them with external network controls.
- **Misleading partial validation**: An expiration-only bypass could appear safer while
  accepting arbitrary values decoded as old timestamps. Mitigation: do not expose that
  mode with the current cookie design.
- **Policy propagation complexity**: The setting crosses configuration, UDP server, and
  UDP core boundaries. Mitigation: pass an immutable enum value explicitly and avoid
  global state.
- **Behavior drift between announce and scrape**: Separate authentication paths can
  diverge. Mitigation: share policy evaluation or add mirrored tests for both services.
- **Operational confusion with `cookie_lifetime`**: Operators may not understand which
  option to use. Mitigation: document that `cookie_lifetime` widens strict validation,
  while `disabled` removes it entirely.
- **Mixed-listener assumptions**: Ban services and metrics must remain scoped correctly.
  Mitigation: add a contract test with strict and disabled listeners in one process.

## References

- GitHub issue: #1136
- Configuration overhaul EPIC: #1978
- Related ban-cleanup subissue: #1453
- UDP tracker protocol: BEP 15
- Existing cookie validation: `packages/udp-core/src/connection_cookie.rs`
- Existing announce validation: `packages/udp-core/src/services/announce.rs`
- Existing scrape validation: `packages/udp-core/src/services/scrape.rs`
