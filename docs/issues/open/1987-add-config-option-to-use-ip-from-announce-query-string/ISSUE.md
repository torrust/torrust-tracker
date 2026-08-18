---
doc-type: issue
issue-type: feature
status: open
priority: p2
github-issue: 1987
spec-path: docs/issues/open/1987-add-config-option-to-use-ip-from-announce-query-string/ISSUE.md
branch: "1987-add-config-option-to-use-ip-from-announce-query-string"
related-pr: null
depends-on:
  - docs/issues/open/1985-rename-peer-addr-to-ip-in-http-announce-request/ISSUE.md
  - docs/issues/open/1980-1978-configuration-overhaul-final-cleanup.md
blocks: null
last-updated-utc: 2026-08-18 00:00
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - packages/http-protocol/src/v1/requests/announce.rs
    - packages/http-core/src/services/announce.rs
    - packages/configuration/src/v2_0_0/
    - docs/issues/open/1978-configuration-overhaul-epic/EPIC.md
    - docs/issues/open/1640-1978-per-http-tracker-on-reverse-proxy-setting.md
    - evidence-opentracker-no-dns-support.md
    - evidence-chihaya-no-dns-support.md
---

# Issue #1987 - Add per-HTTP-tracker config option to use peer IP from `ip` GET parameter (sub-issue of #1978)

## Goal

Add an optional per-HTTP-tracker configuration setting that allows the tracker to use the IP address provided in the `ip` GET parameter of the announce request instead of always deriving the peer IP from the TCP connection. This feature is analogous to opentracker's `WANT_IP_FROM_QUERY_STRING` compile-time option.

## Background

### Current behaviour

The Torrust Tracker HTTP announce handler always derives the peer IP from the TCP connection (or from the `X-Forwarded-For` header when running behind a reverse proxy). The `ip` GET parameter — defined as optional in [BEP 3](https://www.bittorrent.org/beps/bep_0003.html) — is parsed but then **silently ignored**.

BEP 3 states:

> An optional parameter giving the IP (or dns name) which this peer is at. Generally used for the origin if it's on the same machine as the tracker.

The BEP's "generally used for the origin" note explains the primary use case: a peer that is on the same host as the tracker announces itself and wants the tracker to register a specific routable IP (rather than `127.0.0.1` from the loopback connection).

### Feature request

A user request was filed (see [torrust/torrust-tracker #163 comment](https://github.com/torrust/torrust-tracker/issues/163#issuecomment-1836642956)) asking for the ability to use the IP from the query string. This mirrors opentracker's `WANT_IP_FROM_QUERY_STRING` feature, which is enabled via a compile-time flag.

### Why it belongs to the configuration overhaul epic (#1978)

This feature requires adding a new per-HTTP-tracker configuration field. The configuration overhaul (schema v3.0.0) is the right time to introduce new per-tracker settings cleanly, rather than adding them to the existing `v2.0.0` schema that is already being overhauled. The related per-tracker `on_reverse_proxy` setting (#1640) is being introduced in the same epic.

### Prerequisites

This issue depends on the `ip` GET parameter rename (from `peer_addr` to `ip`) being completed first. The rename issue must be resolved before this feature is implemented.

The runtime activation of configuration schema v3.0.0 depends on #1980. Until that migration is complete, this issue uses a temporary internal disabled policy for production wiring. The v3 configuration field and all address-selection behavior are implemented and unit tested now, but the running tracker cannot enable the setting from configuration until #1980 is complete.

### `ip` parameter validation and selection

The tracker distinguishes **absent** and **empty** `ip` parameters:

- **Absent**: the query string does not contain an `ip` parameter.
- **Empty**: the query string contains `ip=` with an empty value.

Both absent and empty parameters are accepted and use the normal connection-derived address (or the address derived through reverse-proxy handling). This deliberately supports clients that automatically emit all known query parameter names while omitting values that are not relevant.

For a non-empty `ip` parameter, the tracker accepts only IPv4 or IPv6 literals. DNS names are not supported. The following contract applies:

| `ip` parameter          | `use_ip_from_query_string = false`            | `use_ip_from_query_string = true`           |
| ----------------------- | --------------------------------------------- | ------------------------------------------- |
| Absent                  | Accept; use the connection/reverse-proxy IP   | Accept; use the connection/reverse-proxy IP |
| Empty (`ip=`)           | Accept; treat as absent                       | Accept; treat as absent                     |
| Valid IPv4/IPv6 literal | Reject; client-supplied peer IPs are disabled | Accept; use the supplied IP                 |
| DNS name                | Reject; DNS names are unsupported             | Reject; DNS names are unsupported           |
| Invalid non-empty value | Reject; an IPv4 or IPv6 literal is required   | Reject; an IPv4 or IPv6 literal is required |

This makes the setting control whether a non-empty client-supplied peer IP override is accepted. A client must receive a protocol failure rather than a successful announce that silently registers a different peer address.

Malformed query-string encoding remains a normal request-parsing failure. The tracker should provide the most specific failure reason it can reliably determine.

### Interaction with `on_reverse_proxy`

When both `use_ip_from_query_string` and `on_reverse_proxy` are enabled, the query string `ip` takes precedence over the `X-Forwarded-For` header. This is because the operator explicitly opted into trusting the query string value. When `use_ip_from_query_string` is disabled (default), the existing `on_reverse_proxy` logic applies unchanged. The two settings are not mutually exclusive; the query string IP wins when both are active and a valid IP is provided.

### Security consideration

Enabling this feature allows a remote client to claim any IP address in its announce request. The tracker would accept that address and include it in the peer list. This is a potential source of IP spoofing in the peer list. The feature must therefore be **opt-in**, disabled by default, and clearly documented as a trust-based setting — suitable only for private/controlled deployments, or as a workaround for peers behind symmetric NAT that cannot be reached via their connection IP.

## Scope

### In Scope

- Add a new optional boolean configuration field to the per-HTTP-tracker configuration (name TBD during schema design, e.g. `use_ip_from_query_string`), disabled by default.
- Accept an absent or empty `ip` GET parameter in both configuration modes, using the normal connection-derived address.
- Reject a non-empty `ip` parameter that is invalid, is a DNS name, or is supplied while the option is disabled, with a precise protocol failure reason.
- When the option is enabled and the `ip` GET parameter contains a valid IP address, use that IP as the peer's address instead of the connection IP.
- Record rejected `ip` parameters in an operator-visible counter and debug log without treating them as application errors.
- Document the security implications of enabling this option in the configuration schema and in the module documentation.
- Preserve the `ip` parameter's raw request state at the HTTP protocol boundary so absent, empty, valid literal, DNS-name, and invalid non-empty values remain distinguishable.
- Add exhaustive tests for every raw-parameter validation and address-selection case. Prefer focused unit tests; add contract/integration tests only where HTTP boundary behavior cannot be validated by unit tests.
- Until schema v3.0.0 is active at runtime, wire the production announce service to an explicit internal disabled policy. Do not add an environment-variable override or a temporary v2 configuration setting.

### Out of Scope

- DNS name resolution in the `ip` parameter (decided against in a separate ADR — see the rename issue).
- Changing the default behaviour (the tracker still uses the connection IP by default).
- Any changes to the UDP tracker protocol.
- Any changes to the scrape endpoint.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status  | Task                                                                  | Notes / Expected Output                                                                                                                                                                                                                                                                |
| --- | ------- | --------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| T1  | TODO    | Design the configuration field name and schema placement              | Align with #1978 schema v3.0.0 design; propose name (e.g. `use_ip_from_query_string`)                                                                                                                                                                                                  |
| T2  | TODO    | Add the field to the per-HTTP-tracker configuration struct            | Target the v3.0.0 schema under `packages/configuration/` as part of the #1978 overhaul                                                                                                                                                                                                 |
| T3  | TODO    | Preserve the raw `ip` parameter state in the HTTP protocol            | Replace the lossy `Option<IpAddr>` parsing boundary, or add a companion representation, so absent, empty, valid literal, DNS-name, and invalid non-empty values can receive their required distinct treatment.                                                                         |
| T4  | TODO    | Inject the address-selection policy into the announce service         | `packages/http-core/src/services/announce.rs` `peer_from_request`. Until #1980 activates v3 at runtime, production constructs the service with an explicit disabled policy; unit tests inject both policy values.                                                                      |
| T5  | TODO    | Validate and select the peer IP                                       | Accept absent and empty `ip`; reject non-empty invalid/DNS `ip`, and a valid non-empty `ip` when the policy is disabled; use a valid `ip` only when enabled. When both this option and `on_reverse_proxy` are enabled, the query-string IP takes precedence.                           |
| T6  | TODO    | Add rejected-parameter observability                                  | Increment a bounded-reason counter and emit debug logging for rejection reasons; do not log raw client-supplied IP values or application errors.                                                                                                                                       |
| T7  | TODO    | Add exhaustive tests for validation and selection                     | Prefer focused unit tests for raw parameter state, absent, empty, valid, invalid, DNS, enabled, disabled, and reverse-proxy precedence behavior. Add only the minimum contract/integration coverage needed to verify HTTP failure responses and configuration wiring.                  |
| T8  | TODO    | Update configuration documentation                                    | `packages/configuration/` docs and `share/default/` config file                                                                                                                                                                                                                        |
| T9  | TODO    | Run `cargo test --workspace` — no regressions                         | All tests pass                                                                                                                                                                                                                                                                         |
| T10 | TODO    | Run `linter all`                                                      | Must exit `0`                                                                                                                                                                                                                                                                          |
| T11 | TODO    | Update migration guide if this subissue affects the config public API | `docs/issues/open/1978-configuration-overhaul-epic/configuration-v2-to-v3-migration.md`                                                                                                                                                                                                |
| T12 | TODO    | Capture baseline behavior locally                                     | Before implementation, run a local HTTP tracker and local tracker client against the baseline request matrix. Record actual behavior in `manual-verification.md`; in particular, establish that non-empty `ip` values are currently silently ignored.                                  |
| T13 | TODO    | Manually verify disabled behavior locally                             | After implementation but before v3 runtime activation, rerun the baseline request matrix. Verify absent/empty behavior is unchanged and document the intentional change: non-empty `ip` values are rejected with precise failure reasons. Append evidence to `manual-verification.md`. |
| T14 | BLOCKED | Manually verify enabled behavior locally with active v3 configuration | After #1980 activates schema v3.0.0 at runtime, enable `use_ip_from_query_string` in a local per-HTTP-tracker config and run the enabled-mode scenarios with the local tracker and tracker client. Append reproducible evidence to `manual-verification.md`.                           |

## Progress Tracking

### Workflow Checkpoints

- [ ] Spec drafted in `docs/issues/drafts/`
- [ ] Spec reviewed and approved by user/maintainer
- [ ] GitHub issue created and issue number added to this spec
- [ ] Prerequisites completed (rename `peer_addr` → `ip` issue resolved)
- [ ] (Optional, recommended for complex issues) Spec-only PR merged into `develop` before implementation
- [ ] Implementation completed
- [ ] Automatic verification completed (`linter all`, relevant tests, and any pre-push checks)
- [ ] Manual verification scenarios executed and recorded (status + evidence)
- [ ] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] Reviewer validated acceptance criteria and updated checkboxes
- [ ] Committer verified spec progress is up to date before commit
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-07-15 00:00 UTC - Copilot/User - Spec drafted as a sub-issue of #1978; feature deferred to the configuration overhaul epic.
- 2026-08-18 00:00 UTC - Copilot/User - Clarified the strict `ip` parameter contract: absent and empty values are accepted as no override; non-empty invalid/DNS values and valid overrides supplied while disabled are rejected. Added observability requirements for rejected parameters.
- 2026-08-18 00:00 UTC - Copilot/User - Required post-implementation manual verification against a local tracker using the local tracker client, with reproducible evidence retained in this issue directory.
- 2026-08-18 00:00 UTC - Copilot/User - Chose staged delivery while v2 remains the active runtime schema: production wiring remains explicitly disabled; unit tests cover both policies; enabled-mode local manual verification is deferred until #1980 activates v3.0.0 configuration.
- 2026-08-18 00:00 UTC - Copilot/User - Required a three-phase local manual verification record: baseline behavior before implementation, disabled-policy behavior after implementation, and enabled-v3 behavior after #1980. The baseline documents the intentional change from silently ignoring non-empty `ip` values to rejecting them when overrides are disabled.

## Acceptance Criteria

- [ ] AC1: When `use_ip_from_query_string` is `false` (default), an absent or empty `ip` GET parameter uses the connection IP; a non-empty `ip` value is rejected with a precise failure reason.
- [ ] AC2: When `use_ip_from_query_string` is `true` and a valid IP is provided in the `ip` GET parameter, the tracker uses that IP as the peer's address.
- [ ] AC3: When `use_ip_from_query_string` is `true`, an absent or empty `ip` GET parameter uses the connection IP; a non-empty invalid IP or DNS name is rejected with a precise failure reason.
- [ ] AC4: The default configuration file (`share/default/`) has `use_ip_from_query_string` set to `false` (or omitted, defaulting to `false`).
- [ ] AC5: The configuration schema documentation clearly states the security implications of enabling this option.
- [ ] AC6: Focused unit tests cover every `ip` parameter validation and address-selection case; minimum contract/integration tests verify HTTP failure responses and configuration wiring where unit tests cannot.
- [ ] AC6a: A counter records rejected non-empty `ip` parameters using bounded reason labels, and debug logs record the rejection without raw client IP values or application error logs.
- [ ] AC7: `linter all` exits with code `0`.
- [ ] AC8: Relevant tests pass with no regressions.
- [ ] AC9: Baseline manual verification runs a local tracker and local tracker client before implementation; reproducible commands, output, expected/actual results, and environment details are recorded in `manual-verification.md` in this issue directory.
- [ ] AC10: Before v3.0.0 runtime activation, manual verification reruns the baseline matrix and documents the intentional disabled-policy change: absent/empty values remain accepted while non-empty values are rejected with precise failure reasons.
- [ ] AC11: After #1980 activates v3.0.0 configuration at runtime, manual verification runs a local tracker and local tracker client with `use_ip_from_query_string` enabled; the resulting evidence is appended to `manual-verification.md`.
- [ ] Acceptance criteria are re-reviewed after implementation and reflect actual behaviour.
- [ ] Documentation is updated when behaviour/workflow changes.

## Verification Plan

### Automatic Checks

- `linter all`
- `cargo test --workspace`
- Pre-push checks (when applicable)

### Required Automated Test Matrix

The implementation must add automated coverage for every row in the parameter contract. Prefer unit tests at the validation and peer-address selection boundaries. Use contract/integration tests only for behavior that requires the HTTP transport boundary.

| ID  | `ip` value                                 | Setting  | Expected outcome                                                 | Preferred test level                   |
| --- | ------------------------------------------ | -------- | ---------------------------------------------------------------- | -------------------------------------- |
| A1  | Raw state: absent                          | Disabled | Accept; use connection/reverse-proxy address                     | Protocol unit + service unit           |
| A2  | Raw state: empty (`ip=`)                   | Disabled | Accept; treat as absent                                          | Protocol unit + service unit           |
| A3  | Valid IPv4 literal                         | Disabled | Reject with a disabled-override failure reason                   | Unit + HTTP contract response          |
| A4  | Valid IPv6 literal                         | Disabled | Reject with a disabled-override failure reason                   | Unit + HTTP contract response          |
| A5  | Raw state: DNS name                        | Disabled | Reject with a DNS-not-supported failure reason                   | Protocol unit + HTTP contract response |
| A6  | Raw state: invalid non-empty value         | Disabled | Reject with an invalid-IP failure reason                         | Protocol unit + HTTP contract response |
| A7  | Raw state: absent                          | Enabled  | Accept; use connection/reverse-proxy address                     | Protocol unit + service unit           |
| A8  | Raw state: empty (`ip=`)                   | Enabled  | Accept; treat as absent                                          | Protocol unit + service unit           |
| A9  | Valid IPv4 literal                         | Enabled  | Accept; use supplied address                                     | Unit                                   |
| A10 | Valid IPv6 literal                         | Enabled  | Accept; use supplied address                                     | Unit                                   |
| A11 | Raw state: DNS name                        | Enabled  | Reject with a DNS-not-supported failure reason                   | Protocol unit + HTTP contract response |
| A12 | Raw state: invalid non-empty value         | Enabled  | Reject with an invalid-IP failure reason                         | Protocol unit + HTTP contract response |
| A13 | Valid IPv4/IPv6 literal with reverse proxy | Enabled  | Accept; supplied address takes precedence over `X-Forwarded-For` | Unit + minimum integration coverage    |
| A14 | Rejected value                             | Either   | Increment the bounded-reason counter and emit a safe debug entry | Unit                                   |

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

Run the same applicable request matrix against a local tracker in three phases: before implementation, after implementation with the disabled policy, and after #1980 activates v3 configuration with the setting enabled. Use the local `tracker_client` for typed valid-IP announces. Use a raw local HTTP client (for example, `curl`) for `ip=`, DNS-name, invalid-IP, and `X-Forwarded-For` requests, which the typed tracker client cannot construct. Follow `.github/skills/dev/environment-setup/run-tracker-locally/SKILL.md` and `.github/skills/usage/use-tracker-client/SKILL.md`. Do not rely on a public tracker for this verification. Record every execution in `manual-verification.md` in this directory, including:

- date/time, commit SHA, OS, Rust toolchain, and effective local tracker configuration;
- exact tracker and client commands, with sensitive values redacted;
- relevant client output and metric/debug-log evidence;
- expected and actual results for every executed scenario.

| ID  | Scenario                                            | Command/Steps                                                                                                            | Expected Result                                                           | Status | Evidence |
| --- | --------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------ | ------------------------------------------------------------------------- | ------ | -------- |
| M1  | Default config: valid non-empty `ip` is rejected    | Start tracker with default config; announce with `ip=1.2.3.4`                                                            | Announce fails, explaining that client-supplied peer IPs are disabled     | TODO   |          |
| M2  | Opt-in config: `ip` GET param is used               | Enable `use_ip_from_query_string`; announce with `ip=1.2.3.4`; check the peer list                                       | Peer is registered with `1.2.3.4`                                         | TODO   |          |
| M3  | Opt-in config: absent or empty `ip` — fallback      | Enable `use_ip_from_query_string`; announce without `ip` and with `ip=`                                                  | Peer is registered with the connection IP in both cases                   | TODO   |          |
| M4  | Opt-in + reverse proxy: `ip` param takes precedence | Enable both `use_ip_from_query_string` and `on_reverse_proxy`; announce with `ip=1.2.3.4` and `X-Forwarded-For: 5.6.7.8` | Peer is registered with `1.2.3.4` (query string wins)                     | TODO   |          |
| M5  | Non-empty invalid or DNS `ip` is rejected           | Announce with enabled and disabled configurations using `ip=invalid` and `ip=example.com`                                | Announce fails with the specific validation reason                        | TODO   |          |
| M6  | Rejected parameters are observable                  | Submit rejected non-empty `ip` parameters and inspect metrics and debug logs                                             | Counter increments by bounded reason; debug log has reason without raw IP | TODO   |          |

**Baseline expectation:** Before implementation, use M1–M5 as an address-selection request matrix. Valid, DNS-name, and invalid non-empty `ip` values are expected to be silently ignored and the announce is expected to succeed using the connection-derived address. Empty and absent values are expected to succeed. M6 is post-implementation only because its metric and debug log do not yet exist.

**Post-implementation disabled-policy expectation:** M1, the disabled-mode portion of M5, and M6 apply. M2–M4 and the enabled-mode portion of M5 remain blocked until #1980 activates schema v3.0.0 configuration at runtime. Execute and document them under T14 once the setting can be enabled in the local tracker configuration.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence                                                    |
| ----- | ---------------------- | ----------------------------------------------------------- |
| AC1   | TODO                   |                                                             |
| AC2   | TODO                   |                                                             |
| AC3   | TODO                   |                                                             |
| AC4   | TODO                   |                                                             |
| AC5   | TODO                   |                                                             |
| AC6   | TODO                   |                                                             |
| AC6a  | TODO                   |                                                             |
| AC7   | TODO                   |                                                             |
| AC8   | TODO                   |                                                             |
| AC9   | TODO                   |                                                             |
| AC10  | TODO                   |                                                             |
| AC11  | BLOCKED                | Requires #1980 to activate v3.0.0 configuration at runtime. |

## Risks and Trade-offs

- **IP spoofing**: When enabled, a client can register any IP address in the peer list. This is inherent to the feature and must be clearly documented. The opt-in default mitigates the risk for deployments that do not need this.
- **Compatibility versus ambiguity**: This feature intentionally rejects non-empty `ip` overrides while disabled, rather than silently ignoring them. This makes configuration support transparent to clients, but is a documented HTTP announce compatibility change for 4.0.0.
- **Interaction with reverse proxy mode**: Resolved — when both `use_ip_from_query_string` and `on_reverse_proxy` are enabled, the query string `ip` takes precedence. See "Interaction with `on_reverse_proxy`" above for rationale.
- **IPv4/IPv6**: The `ip` parameter accepts both IPv4 and IPv6 addresses (via `IpAddr::from_str`). If the tracker is bound to an IPv6-only socket and a client sends an IPv4 `ip`, the address is accepted as-is — the tracker does not validate address family compatibility with the listener binding.

## References

- BEP 3 — The BitTorrent Protocol Specification: <https://www.bittorrent.org/beps/bep_0003.html>
- Feature request: <https://github.com/torrust/torrust-tracker/issues/163#issuecomment-1836642956>
- Parent epic: [#1978 — Configuration Overhaul](../1978-configuration-overhaul-epic/EPIC.md)
- Prerequisite issue: rename `peer_addr` → `ip` (to be linked once created)
- Related issue: [#1640 — Per-HTTP-tracker `on_reverse_proxy` setting](../1640-1978-per-http-tracker-on-reverse-proxy-setting.md)
- opentracker `WANT_IP_FROM_QUERY_STRING`: <https://erdgeist.org/arts/software/opentracker/>
- Research evidence — opentracker DNS name support: [evidence-opentracker-no-dns-support.md](evidence-opentracker-no-dns-support.md)
- Research evidence — chihaya DNS name support: [evidence-chihaya-no-dns-support.md](evidence-chihaya-no-dns-support.md)
