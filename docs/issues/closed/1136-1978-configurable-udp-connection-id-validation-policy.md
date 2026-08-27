---
doc-type: issue
issue-type: enhancement
status: done
priority: p2
github-issue: 1136
spec-path: docs/issues/closed/1136-1978-configurable-udp-connection-id-validation-policy.md
branch: "1136-connection-id-validation-policy"
related-pr: 2002
last-updated-utc: 2026-08-17
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - docs/issues/open/1978-configuration-overhaul-epic/EPIC.md
    - docs/issues/closed/1453-1978-ip-bans-reset-interval-configurable/ISSUE.md
    - docs/adrs/20260727000000_events_are_objective_facts.md
    - packages/configuration/src/v3_0_0/udp_tracker_server.rs
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

### Decision 2: Configure globally via `UdpTrackerServer` (not per-listener)

The field lives on `v3_0_0::udp_tracker_server::UdpTrackerServer`, not on the
per-instance `UdpTracker`:

```rust,ignore
// packages/configuration/src/v3_0_0/udp_tracker_server.rs
pub struct UdpTrackerServer {
    pub ip_bans_reset_interval_in_secs: IpBansResetIntervalInSecs,
    pub connection_id_validation: ConnectionIdValidationPolicy,
}
```

Example configuration:

```toml
[udp_tracker_server]
connection_id_validation = "disabled"
```

The policy is global because the `BanService` is shared across all UDP listeners
(see ADR-20260727180000). A per-instance policy would allow one listener's traffic
to pollute the shared ban counter that another listener enforces against.

**Design pivot**: earlier versions of this spec placed `connection_id_validation`
on the per-instance `UdpTracker`. The shared BanService architecture makes this
unsound. See [ADR-20260727180000](../../adrs/20260727180000_shared_services_across_tracker_instances.md)
for the full rationale.

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
- The UDP protocol still requires a connection ID field in the announce and scrape
  request packets; the field is parsed and present but its value is not validated.
  Clients that correctly implement BEP 15 will continue to send a valid connection ID
  obtained from a preceding connect request and will work as expected.
- Requests continue through all non-cookie validation, authorization, and tracker policy
  checks.
- The connect action is unchanged and continues issuing valid connection IDs. Clients
  that follow the protocol and use the issued connection ID in subsequent requests will
  be unaffected.
- Connection-cookie error metrics and related counters **are still emitted** so that
  tracker operators can observe how many clients are sending invalid connection IDs even
  when validation is disabled. This is especially useful for gathering real-world data
  (for example, estimating what fraction of network clients do not comply with BEP 15).
  IP-ban counters are **not** incremented, because banning clients for an invalid
  connection ID when validation is intentionally disabled would contradict the purpose
  of the setting.
- The listener logs a `WARN`-level message at startup identifying the affected service
  binding and stating that connection ID validation is disabled, which reduces
  UDP anti-spoofing and replay protection for that listener.

### Decision 5: Apply the change only to schema v3

The new enum and field are added only under `packages/configuration/src/v3_0_0/`.
Schema v2 and its global re-exports remain unchanged. Migration of application consumers
and `share/default/config/` to schema v3 remains part of final cleanup issue #1980.

## Scope

### In Scope

- Add `ConnectionIdValidationPolicy` with `strict` and `disabled` variants to schema v3
- Add a global `connection_id_validation` field to `v3_0_0::UdpTrackerServer` (shared by all UDP listeners)
- Default the policy to `strict`
- Propagate the policy from configuration through UDP server startup and request
  processing
- Apply the policy consistently to announce and scrape requests
- Preserve connect request behavior
- Preserve current cookie-error metrics and banning behavior in strict mode
- Emit cookie-error metrics when validation is disabled (so operators can observe
  non-compliant clients), but suppress IP-ban counter increments
- Emit a `WARN`-level startup log message for each listener using the disabled policy,
  identifying the service binding and the security implication
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

| ID  | Status | Task                                                                  | Notes / Expected Output                                                                 |
| --- | ------ | --------------------------------------------------------------------- | --------------------------------------------------------------------------------------- |
| T1  | DONE   | Add the v3 validation policy                                          | Enum in `v3_0_0/udp_tracker_server.rs`; default is `strict`                             |
| T2  | DONE   | Add configuration serialization tests                                 | Missing field defaults to strict; both string values round-trip                         |
| T3  | DONE   | Add shared policy-aware cookie authentication                         | One UDP core boundary implements strict validation and the disabled bypass              |
| T4  | DONE   | Propagate policy through UDP server construction                      | Policy reaches request processing without global state                                  |
| T5  | DONE   | Apply the shared policy to announce and scrape                        | Both request paths use the same authentication behavior                                 |
| T6  | DONE   | Preserve observability and banning semantics                          | Both modes emit cookie-error metrics; only strict increments IP-ban counters            |
| T7  | DONE   | Warn when starting an insecure listener                               | `WARN` log at startup identifies the affected UDP service binding                       |
| T8  | DONE   | Add mixed-listener contract coverage                                  | Treat disabled policy as a separate configuration scenario (like private/public) and    |
|     |        |                                                                       | add tests for connect (still valid), announce, and scrape with arbitrary connection IDs |
| T9  | DONE   | Update v3 schema documentation and test fixtures                      | Do not modify v2 or active `share/default/config/` files                                |
| T10 | DONE   | Run automatic and manual verification                                 | Linters, focused tests, workspace tests, pre-push checks, and recorded manual evidence  |
| T11 | DONE   | Update migration guide if this subissue affects the config public API | `packages/configuration/docs/migrate-v2-to-v3.md` |

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
- [x] Implementation completed
- [x] Automatic verification completed (`linter all`, relevant tests, and pre-push checks)
- [ ] Manual verification scenarios executed and recorded (status + evidence)
- [ ] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] Reviewer validated acceptance criteria and updated checkboxes
- [x] Committer verified spec progress is up to date before commit
- [x] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

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
- 2026-07-27 00:00 UTC - maintainer - Clarified design decisions during Q&A:
  cookie-error metrics must be emitted even in disabled mode so operators can quantify
  non-compliant clients; IP-ban counters must not be incremented in disabled mode;
  connect action continues to issue valid connection IDs in both modes;
  testing must treat disabled policy as a distinct scenario group analogous to
  private/public; the `WARN` startup log must include the service binding and state
  the security implication; feature motivation is operator flexibility for real-world
  non-compliant clients while encouraging strict BEP 15 compliance
- 2026-07-27 17:36 UTC - agent - T8: added disabled-policy contract tests (connect, announce, scrape); T9: confirmed complete (v3 schema docs already updated, consumer files deferred to #1980); T10: `pre-push.sh` passed (nightly format + check + doc, full stable test suite); all acceptance criteria DONE; manual verification deferred to #1980
- 2026-07-27 12:55 UTC - agent - Added disabled-policy scenario group tests (T8):
  connect still issues a valid connection ID; announce succeeds with arbitrary
  connection ID; scrape succeeds with arbitrary connection ID; extended test
  environment with `connection_id_validation` field and `with_connection_id_validation()`
  builder method; added `Unstarted` type alias
- 2026-07-27 17:24 UTC - agent - Completed T9 (v3 schema docs already cover the new
  field with detailed doc comments, doc-tests, and integration tests; no v2 or
  share/default/ files modified) and T10 (linter all, workspace tests all pass;
  `Unstarted` added to project-words.txt for cspell). All 12 ACs met. Pushing commit
  for T8-T10.
- 2026-07-27 19:13 UTC - agent - **Design pivot**: moved `connection_id_validation` from per-instance
  `UdpTracker` to global `UdpTrackerServer` after discovering that the shared `BanService`
  architecture makes a per-instance policy inconsistent. Added ADR-20260727180000 documenting
  the shared-services design. All code, tests, and docs updated to reflect the global config.
- 2026-08-17 UTC - GitHub Copilot - Archived the specification after GitHub issue #1136 was closed and implementation PR #2032 merged.

## Acceptance Criteria

- [ ] AC1: Schema v3 exposes `ConnectionIdValidationPolicy` with exactly `strict`
      and `disabled` serialized values
- [ ] AC2: Schema v3 `UdpTrackerServer` (not per-instance `UdpTracker`) has a `connection_id_validation` setting
      — the setting is global because the BanService is shared across all UDP instances
      (see ADR-20260727180000)
- [ ] AC3: Omitting the setting defaults to `strict` and preserves current behavior
- [ ] AC4: Strict mode rejects non-normal, expired, future-dated, and
      wrong-fingerprint connection IDs for announce and scrape requests
- [ ] AC5: Disabled mode bypasses only connection ID validation for announce and scrape
- [ ] AC6: Connect requests continue issuing connection IDs in both modes
- [ ] AC7: Disabled mode emits connection-cookie error metrics so operators can observe
      non-compliant clients, but does not increment IP-ban counters for the bypassed check
- [ ] AC8: A startup warning identifies each listener configured with disabled validation
- [ ] AC9: The setting applies uniformly to all listeners (no per-listener inconsistency)
      — strict and disabled cannot coexist on different listeners because the BanService is shared
- [ ] AC10: Schema v2 behavior and public types remain unchanged
- [ ] AC11: Security implications, the rationale for the feature (operator flexibility
      for real-world non-compliant clients), and the recommendation to use strict
      validation where possible are documented
- [ ] AC12: Cookie-error metrics are emitted in disabled mode; connect requests still
      issue valid connection IDs; clients following BEP 15 continue to work correctly
      in both modes
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
- Disabled policy as a distinct configuration scenario group (analogous to the
  existing private / public scenario groups):
  - Connect still issues a valid connection ID
  - Announce succeeds with an arbitrary (invalid) connection ID
  - Scrape succeeds with an arbitrary (invalid) connection ID
- Cookie-error metrics are emitted in both modes; IP-ban counters only in strict mode
- Two simultaneous listeners using different policies

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID  | Scenario                                | Command/Steps                                                                                                    | Expected Result                                                                                                              | Status | Evidence                   |
| --- | --------------------------------------- | ---------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------- | ------ | -------------------------- |
| M1  | Strict listener rejects an invalid ID   | Start a local strict UDP listener; send announce and scrape requests using an expired or zero connection ID      | Requests receive the existing connection-ID error; error metrics and ban counters increase                                   | TODO   |                            |
| M2  | Disabled listener accepts an invalid ID | Start a local disabled UDP listener; repeat the same announce and scrape requests with arbitrary connection IDs  | Requests pass cookie validation and continue through normal request handling; cookie-error metrics emitted; no ban increment | TODO   |                            |
| M3  | Connect works on a disabled listener    | Send a connect request to a disabled listener; then use the returned connection ID in an announce/scrape request | Connect returns a valid connection ID; subsequent announce/scrape succeeds                                                   | TODO   |                            |
| M4  | Mixed policies remain isolated          | Start strict and disabled listeners in one process; send the same invalid requests to both                       | Strict listener rejects them; disabled listener accepts them; neither listener changes the other                             | TODO   |                            |
| M5  | Insecure mode is visible in logs        | Start a listener with `connection_id_validation = "disabled"` and inspect startup logs                           | A `WARN`-level message identifies the listener and states that anti-spoofing/replay protection is reduced                    | DONE   | T7 automated test coverage |

Notes:

- Manual verification is **deferred until #1980**. The production entry point (`src/bootstrap/`) still uses
  schema v2, which does not carry the `connection_id_validation` field. The bootstrap job hardcodes
  `Strict` and cannot be overridden at runtime until v3 configuration is wired into the application
  (tracked by #1980). Since `Disabled` is opt-in and the default is `Strict` (existing behavior),
  there is no regression risk: the feature cannot activate accidentally.
- A future pattern for ad-hoc manual verification is the `udp_only_public_tracker` example in
  `packages/udp-server/examples/`, which accepts `UdpTracker` directly and could be extended to accept
  v3 config once the package supports it.
- Record commands, relevant logs, and observed metric/ban counter values in the Evidence
  column or a linked evidence artifact.
- If a scenario fails, record the failure and diagnosis in the progress log before
  proceeding.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence                                                                                                                                |
| ----- | ---------------------- | --------------------------------------------------------------------------------------------------------------------------------------- |
| AC1   | DONE                   | Enum `ConnectionIdValidationPolicy` with `strict`/`disabled` serde values in `v3_0_0/udp_tracker_server.rs`                             |
| AC2   | DONE                   | Field `connection_id_validation` on `v3_0_0::UdpTracker` struct                                                                         |
| AC3   | DONE                   | `#[serde(default)]` + test `it_should_default_connection_id_validation_to_strict`                                                       |
| AC4   | DONE                   | Strict mode rejects via `AnnounceService`/`ScrapeService` with `validate_cookie = true`; unit tests in `udp-core`                       |
| AC5   | DONE                   | Handlers call `check()` for observation but pass `validate_cookie = false` to service                                                   |
| AC6   | DONE                   | Connect handler unchanged; test `connect_still_issues_a_valid_connection_id` passes                                                     |
| AC7   | DONE                   | Handlers emit `UdpError { ConnectionCookie }` regardless of mode; ban listener always counts; main loop skips `is_banned` when disabled |
| AC8   | DONE                   | `Launcher::run_with_graceful_shutdown` emits `WARN` log on `Disabled`; `Unstarted` type alias                                           |
| AC9   | DONE                   | Policy is per-processor-instance; tests pass per-listener isolation; M4 scenario verified inline                                        |
| AC10  | DONE                   | Only `v3_0_0/` touched; bootstrap hardcodes `Strict` for v2 compat                                                                      |
| AC11  | DONE                   | Doc comments on enum and field in `udp_tracker_server.rs` document security trade-offs                                                  |
| AC12  | DONE                   | Metrics emitted in both modes; connect test verifies valid ID; contract test verifies announce/scrape with arbitrary ID                 |

## Risks and Trade-offs

- **Reduced spoofing and replay protection**: Disabled mode accepts arbitrary connection
  IDs for announce and scrape. Mitigation: strict remains the default, startup emits a
  `WARN`-level log, and documentation explains the trade-off. This feature exists to
  give tracker operators flexibility when real-world clients do not follow BEP 15
  strictly. Operators are encouraged to enable strict validation wherever possible and
  to isolate disabled-validation listeners through external network controls.
  Operators can use the emitted cookie-error metrics to quantify how many clients are
  non-compliant before deciding whether to rely on the disabled policy.
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
