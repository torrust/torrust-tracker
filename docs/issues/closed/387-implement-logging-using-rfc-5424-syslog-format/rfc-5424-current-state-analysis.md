---
doc-type: analysis
status: complete
related-issue: 387
last-updated-utc: 2026-08-26
semantic-links:
  related-artifacts:
    - docs/issues/closed/387-implement-logging-using-rfc-5424-syslog-format/ISSUE.md
    - packages/configuration/src/v3_0_0/logging.rs
    - docs/adrs/20260519000000_define_global_cli_output_contract.md
    - docs/adrs/20260727000000_events_are_objective_facts.md
    - docs/adrs/20260822094338_adopt_secrecy_for_sensitive_values.md
    - https://www.rfc-editor.org/rfc/rfc5424.txt
    - https://crates.io/crates/tracing-rfc-5424
    - https://github.com/sp1ff/syslog-tracing
---

# RFC 5424 Current-State Analysis for Issue #387

## Purpose

This analysis checks whether issue #387 remains meaningful against the tracker as of 2026-08-26. It compares the issue with RFC 5424, the current logging implementation, and relevant architecture decisions. It gives a hypothetical remaining-effort estimate; it is not an implementation plan.

## Conclusion

Issue #387 remains valid, but its requested outcome combines three distinct concerns:

1. RFC 5424 message serialization and transport.
2. Logging destination and lifecycle, including files, rotation, directory, and permissions.
3. Application log-level semantics and configuration.

The tracker has partially implemented the third concern. It has not implemented RFC 5424 messages or a syslog transport. The tracker deliberately does not own log files, rotation, directory creation, ownership, or permissions: those are infrastructure concerns managed by the tracker operator. Torrust Tracker Deployer configures them for production deployments; the Tracker Demo uses Docker Compose log rotation.

### Recommendation

Do not implement RFC 5424 support now, either completely or partially. The existing `tracing`-based logging is adequate for the tracker and avoids creating and maintaining a custom logging subsystem solely to satisfy a complex standard without a demonstrated operational requirement.

The tracker should continue to use `tracing` and `tracing_subscriber` as its logging abstraction. Strict RFC 5424 output would require either a custom `tracing_subscriber` formatter or layer, or an additional maintained crate that provides the required behavior. This work can remain at the logging-output boundary and does not require changes to domain events, servers, or the tracker architecture. However, it would introduce a new formatting contract, configuration, conformance tests, and ongoing compatibility work with the tracing ecosystem.

The primary scenario in which direct syslog delivery is valuable is a distributed fleet of services or tracker instances whose logs need central collection and correlation. That is not a likely current tracker deployment: each tracker process owns process-local, in-memory swarm state, so the current architecture scales vertically rather than as horizontally interchangeable replicas. For the expected single-instance deployment, the runtime, host logger, or operator log agent can collect the existing stderr output and forward it centrally without making the tracker a syslog client.

Retain issue #387 as research only and let Cameron decide whether the remaining benefit warrants that cost. Until a concrete deployment, integration, or customer requirement needs direct RFC 5424 records, no implementation subissues should be created.

## RFC 5424 Requirements Relevant to This Issue

RFC 5424 section 6 defines a syslog message as:

```text
SYSLOG-MSG = HEADER SP STRUCTURED-DATA [SP MSG]
HEADER = PRI VERSION SP TIMESTAMP SP HOSTNAME SP APP-NAME SP PROCID SP MSGID
```

The header uses seven-bit ASCII. `PRI` is `<facility * 8 + severity>`; facility values are in $0..=23$ and severity values in $0..=7$. RFC 5424 defines severities `Emergency` (0), `Alert` (1), `Critical` (2), `Error` (3), `Warning` (4), `Notice` (5), `Informational` (6), and `Debug` (7). The RFC's version is `1`.

`TIMESTAMP`, `HOSTNAME`, `APP-NAME`, `PROCID`, and `MSGID` may use the nil value (`-`) when unavailable. `STRUCTURED-DATA` is either `-` or one or more bracketed elements. Structured-data parameter values must escape `"`, `\\`, and `]`.

RFC 5424 specifies a message format. It does not mandate that an application writes local log files, rotates them, creates `/var/log/torrust/tracker`, or changes Unix ownership and permissions. Those are deployment and operational-policy decisions.

## Current Tracker State

### Logging implementation

The running tracker daemon initializes logging once during bootstrap through `packages/configuration/src/logging.rs`. The public configuration currently aliases the v2 schema, whose `[logging]` section has one `threshold` setting with values `off`, `error`, `warn`, `info`, `debug`, and `trace`; the default is `info`. The active setup uses the default `tracing_subscriber` formatter, not a configurable style.

`packages/configuration/src/v3_0_0/logging.rs` is a newer, not-yet-active configuration schema. It adds `trace_filter` and the `full`, `pretty`, `compact`, and `json` styles. Its `Json` style produces tracing-subscriber JSON, not RFC 5424 syslog messages. None of the configured styles emits RFC 5424's `PRI`, protocol version, `HOSTNAME`, `APP-NAME`, `PROCID`, `MSGID`, or RFC 5424 `STRUCTURED-DATA` grammar.

The current threshold vocabulary is tracing's six filters. It does not model the RFC 5424 facility, and does not expose all RFC severity concepts, notably `Emergency`, `Alert`, `Critical`, and `Notice`. `warn` is broadly comparable to RFC `Warning`, `info` to `Informational`, and `debug` to `Debug`, but that resemblance is insufficient for RFC 5424 compliance because `PRI` requires both facility and severity.

There is no current logging configuration for an output destination or syslog endpoint. There is intentionally no application configuration for a file path, rotation policy, retention policy, directory creation, ownership, or permissions; these belong to the deployment configuration selected by the operator.

### Existing observability and safety guidance

The event ADR requires event variants to describe objective facts and keeps enforcement policy at the consumer or enforcement point. An RFC 5424 formatter should therefore serialize existing tracing events and fields without reshaping domain events to suit a log sink.

The secrecy ADR requires sensitive values to remain redacted in tracing, `Debug`, `Display`, errors, and diagnostics. Any RFC 5424 structured-data encoder must preserve that invariant and must not stringify secret wrappers through an unsafe display path.

The global CLI output contract says the long-running `torrust-tracker` daemon sends tracing diagnostics to stderr. The current v3 logging module's documentation says stdout, and its subscriber setup does not explicitly choose a production writer. The desired output stream must be clarified before introducing a syslog destination or file sink.

## Gap Assessment

| Issue #387 request                                           | Current state                                             | Gap                                                                               |
| ------------------------------------------------------------ | --------------------------------------------------------- | --------------------------------------------------------------------------------- |
| RFC 5424 format                                              | Full, pretty, compact, and JSON tracing formats           | Not implemented                                                                   |
| Priority, timestamp, hostname, program name, structured data | Tracing metadata, timestamp, and fields vary by formatter | RFC header, PRI calculation, and RFC structured-data encoding are not implemented |
| RFC severity levels                                          | `off`, `error`, `warn`, `info`, `debug`, `trace` filters  | No facility; no complete RFC severity mapping                                     |
| Log rotation                                                 | Managed by the deployment infrastructure                  | Not a tracker application responsibility; RFC 5424 does not require it            |
| `/var/log/torrust/tracker` log directory                     | Managed by the deployment infrastructure                  | Not a tracker application responsibility                                          |
| Permissions and ownership                                    | Managed by the deployment infrastructure                  | Not a tracker application responsibility; account for containers and non-root use |
| Dynamic log level                                            | `logging.trace_filter` configuration                      | Partially implemented                                                             |
| Test and documentation                                       | Unit tests cover configuration values                     | RFC conformance, transport/sink, and deployment tests/docs are absent             |

## RFC 5424 Facility

The facility is the source category encoded in the RFC 5424 `PRI` value. It is not the same as a log level. For example, a facility of `local0` has numeric value 16; an `Informational` severity has numeric value 6; together they produce `<134>` because $16 \times 8 + 6 = 134$.

Facilities let a syslog receiver route records from different applications or subsystems. The standard reserves `local0` through `local7` for local policy. A tracker implementation should select one `local*` facility, normally as a fixed product decision, unless operators have a demonstrated need to configure it. This decision matters only if the tracker emits RFC 5424 records or sends them to a syslog receiver.

## `tracing-rfc-5424` Crate Assessment

The [`tracing-rfc-5424`](https://crates.io/crates/tracing-rfc-5424) crate, from [`sp1ff/syslog-tracing`](https://github.com/sp1ff/syslog-tracing), is an existing `tracing_subscriber::Layer`. It formats tracing events as RFC 5424 or RFC 3164 syslog messages and sends them to a syslog daemon through UDP, TCP, or Unix-domain socket transports. It can be composed with the tracker's existing `tracing_subscriber` formatter rather than replacing the tracker logging architecture.

This means that a future tracker integration could use a maintained implementation for the RFC message grammar and transport instead of implementing those low-level details itself. The crate's default is RFC 5424 over UDP to a local syslog daemon on port 514; therefore, using it would add an optional network or Unix-socket logging sink and a deployment dependency on a syslog daemon. It would not configure file rotation, retention, directory creation, ownership, or permissions, which remain operator concerns.

The crate is not a drop-in replacement for the tracker's current human-readable stderr output:

- Its supplied `TrivialTracingFormatter` extracts only the tracing event's `message` field. It does not preserve arbitrary tracker fields such as `client`, `torrent`, and `error` in the emitted message.
- It can emit RFC 5424 structured data for selected tracing metadata, such as source file and line number. It does not supply the tracker-specific field mapping described above, so preserving arbitrary tracing fields would still require a custom formatter or an upstream contribution.
- Its published roadmap describes the `0.2.x` series as preliminary and lists broader tracing-field mapping, span support, asynchronous transports, and additional documentation as future work. A logging call may therefore perform synchronous transport work, and transport failure and backpressure behavior would need explicit evaluation.
- The crate is licensed `GPL-3.0-or-later`. The tracker is `AGPL-3.0-only`; a future dependency proposal must include the repository's normal license-compatibility review before adoption.

The absence of a specific `MSGID` mapping does not itself prevent valid RFC 5424 output because the RFC permits `-` as the nil value. Similarly, RFC 5424 permits `-` instead of structured data. Consequently, the crate may be sufficient for a narrow future requirement such as sending basic compliant event messages to a local syslog daemon. It is not sufficient for a requirement to preserve the full structured tracker context without further work.

**Recommendation:** do not add the crate now. If a concrete deployment requires RFC 5424 delivery to a syslog daemon, perform a small, time-boxed compatibility spike first. It should verify tracker MSRV/dependency compatibility, license approval, non-blocking behavior under an unavailable or slow daemon, the selected facility and transport, and whether losing arbitrary tracing fields is acceptable.

## Requirements if the Issue Is Reopened

1. Is the intended product an RFC 5424 formatter for stdout/stderr, a syslog client transport, or both?
2. Which RFC 5424 facility should the tracker use by default, and should it be configurable?
3. How should tracing levels and the RFC severity values map, especially `trace`, `off`, `Critical`, `Alert`, and `Emergency`?
4. Which stable `APP-NAME`, `PROCID`, and `MSGID` values should the tracker emit?
5. Which tracing fields become RFC structured data, what enterprise ID or namespacing is used for `SD-ID`, and how are malformed/non-ASCII keys and values handled?
6. Does the daemon continue to send normal diagnostics to stderr as required by the CLI output ADR, or does a selected syslog sink replace that stream?

### Structured-Data Field Mapping

Requirement 5 concerns the difference between a tracing event and an RFC 5424 record. The tracker can emit arbitrary named tracing fields, for example:

```rust
tracing::info!(client = client_label, torrent = %hash, "torrent is absent");
```

An RFC 5424 formatter must decide whether those fields are omitted, added only to the free-form message, or translated to `STRUCTURED-DATA`. A possible translation is:

```text
<134>1 2026-08-26T10:00:00Z tracker.example torrust-tracker 1234 - [torrust@PEN client="qbittorrent" torrent="abc..."] torrent is absent
```

This example is illustrative only. `torrust@PEN` would need a valid structured-data identifier: `torrust` is the element name and `PEN` would need to be replaced by Torrust's IANA Private Enterprise Number. The formatter must also define stable parameter names such as `client` and `torrent`. Once deployed, log collectors, dashboards, alerts, and parsers may depend on those names, so changing them becomes a compatibility concern.

The formatter would additionally need rules for tracing fields that RFC 5424 cannot represent directly. Structured-data names are restricted to printable ASCII and exclude spaces, `=`, `]`, and `"`; parameter values must escape `"`, `\\`, and `]`. Tracing field names or values that are non-ASCII, contain invalid characters, are nested, or are not meaningful operational attributes require a deliberate policy: reject them, omit them, encode them, or retain them only in the free-form message.

Finally, the mapping must preserve the secrecy ADR. Fields containing credentials, tokens, client-identifying data, or other sensitive values must remain redacted before a formatter serializes them. This is why strict RFC 5424 output is more than changing the timestamp or severity label: it introduces a public schema and serializer for every selected tracing field.

## Hypothetical Remaining-Effort Estimate

The following estimate applies only to a custom RFC 5424 formatter that emits records to the existing stderr stream. It leaves persistence, rotation, and permissions to the deployment infrastructure.

| Work item                                               | Estimated effort | Notes                                                                                                                           |
| ------------------------------------------------------- | ---------------- | ------------------------------------------------------------------------------------------------------------------------------- |
| Decide the RFC field contract and tracing-level mapping | 1-2 days         | Covers facility, application/process/message identifiers, `trace`/`off`, structured-data namespace, and secret-redaction review |
| Implement an RFC 5424 tracing formatter                 | 3-5 days         | Includes header generation, `PRI`, RFC escaping, timestamp handling, and preserving existing tracing fields                     |
| Add configuration, migration, and documentation         | 1-2 days         | Must target the active v2 schema or be coordinated with the v3 configuration migration                                          |
| Unit, integration, and conformance tests                | 2-3 days         | Covers deterministic formatting, escaping, severity/facility mapping, configuration, and stderr output                          |
| Review and contingency                                  | 1-2 days         | Covers tracing-subscriber extension constraints and compatibility fixes                                                         |

**Total for a custom formatter: 8-14 engineering days.** A separate syslog network transport, TLS support, reconnection/backpressure policy, or multiple destination support is a separate feature and would materially increase the estimate. Application-owned file rotation is explicitly out of scope.

Using `tracing-rfc-5424` changes the future research path, not the current recommendation. A 1-2 day compatibility spike could establish whether the crate's basic RFC 5424 messages, daemon transport, synchronous behavior, GPL license, and loss of arbitrary tracing fields are acceptable for a specific deployment. If they are, the subsequent integration is likely smaller than a custom formatter. If full tracker field preservation is required, the crate does not eliminate the custom formatter or upstream-contribution work.

## Recommended Issue Outcome

Retain issue #387 as a research issue. Do not create implementation subissues and do not perform a partial crate integration now. Cameron can decide whether to close it as out of current priorities or leave it open as a deferred enhancement after reviewing this analysis.

If a future requirement makes RFC 5424 support worthwhile, the likely work is:

1. Run the `tracing-rfc-5424` compatibility spike against the concrete deployment requirement.
2. Define the logging-output architecture and RFC 5424 configuration contract only if the spike shows that the crate is insufficient or unsuitable.
3. Adopt and test the crate for the narrow syslog-delivery use case, or implement a standards-compliant formatter and field mapping if full tracker context is required.
4. Keep rotation, directory, ownership, and permissions documented and implemented in Torrust Tracker Deployer or equivalent operator infrastructure.

This avoids making the tracker responsible for OS-level policy where the container runtime, systemd/journald, or syslog daemon is the appropriate owner.

## Sources

- RFC 5424, sections 6, 6.2, and 6.3: <https://www.rfc-editor.org/rfc/rfc5424.txt>
- `tracing-rfc-5424` v0.2.1 crate metadata and documentation: <https://crates.io/crates/tracing-rfc-5424>
- `sp1ff/syslog-tracing` source and roadmap: <https://github.com/sp1ff/syslog-tracing>
- Current v3 logging setup: `packages/configuration/src/v3_0_0/logging.rs`
- CLI output contract: `docs/adrs/20260519000000_define_global_cli_output_contract.md`
- Events principle: `docs/adrs/20260727000000_events_are_objective_facts.md`
- Sensitive-data logging policy: `docs/adrs/20260822094338_adopt_secrecy_for_sensitive_values.md`
