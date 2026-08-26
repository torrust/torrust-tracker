---
doc-type: analysis
status: draft
related-issue: 387
last-updated-utc: 2026-08-26
semantic-links:
  related-artifacts:
    - docs/issues/open/387-implement-logging-using-rfc-5424-syslog-format/ISSUE.md
    - packages/configuration/src/v3_0_0/logging.rs
    - docs/adrs/20260519000000_define_global_cli_output_contract.md
    - docs/adrs/20260727000000_events_are_objective_facts.md
    - docs/adrs/20260822094338_adopt_secrecy_for_sensitive_values.md
    - https://www.rfc-editor.org/rfc/rfc5424.txt
---

# RFC 5424 Current-State Analysis for Issue #387

## Purpose

This analysis checks whether issue #387 remains meaningful against the tracker as of 2026-08-26. It compares the issue with RFC 5424, the current logging implementation, and relevant architecture decisions. It is not an implementation plan.

## Conclusion

Issue #387 remains valid, but its requested outcome combines three distinct concerns that should be decided separately before implementation:

1. RFC 5424 message serialization and transport.
2. Logging destination and lifecycle, including files, rotation, directory, and permissions.
3. Application log-level semantics and configuration.

The tracker has partially implemented the third concern. It has not implemented RFC 5424 messages, a syslog transport, application-managed log files, rotation, or file-permission policy.

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

`packages/configuration/src/v3_0_0/logging.rs` configures a `tracing_subscriber` formatter once per process. It provides these `trace_filter` values: `off`, `error`, `warn`, `info`, `debug`, and `trace`; the default is `info`. It provides `full`, `pretty`, `compact`, and `json` output styles; the default is `full`.

The `Json` style produces tracing-subscriber JSON, not RFC 5424 syslog messages. None of the configured styles emits RFC 5424's `PRI`, protocol version, `HOSTNAME`, `APP-NAME`, `PROCID`, `MSGID`, or RFC 5424 `STRUCTURED-DATA` grammar.

The current threshold vocabulary is tracing's six filters. It does not model the RFC 5424 facility, and does not expose all RFC severity concepts, notably `Emergency`, `Alert`, `Critical`, and `Notice`. `warn` is broadly comparable to RFC `Warning`, `info` to `Informational`, and `debug` to `Debug`, but that resemblance is insufficient for RFC 5424 compliance because `PRI` requires both facility and severity.

There is no current logging configuration for an output destination, a syslog endpoint, file path, rotation policy, retention policy, directory creation, ownership, or permissions.

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
| Log rotation                                                 | No application logging-to-file implementation             | Not implemented; RFC 5424 does not require it                                     |
| `/var/log/torrust/tracker` log directory                     | No configured log directory                               | Not implemented; should be deployment-policy driven                               |
| Permissions and ownership                                    | No application-managed log files                          | Not implemented; should account for containers and non-root execution             |
| Dynamic log level                                            | `logging.trace_filter` configuration                      | Partially implemented                                                             |
| Test and documentation                                       | Unit tests cover configuration values                     | RFC conformance, transport/sink, and deployment tests/docs are absent             |

## Decisions Required Before an Implementation Plan

1. Is the intended product an RFC 5424 formatter for stdout/stderr, a syslog client transport, or both?
2. Should production deployments delegate file persistence, rotation, ownership, and permissions to systemd/journald, Docker/Podman, or an external syslog daemon rather than the tracker process?
3. Which RFC 5424 facility should the tracker use by default, and should it be configurable?
4. How should tracing levels and the RFC severity values map, especially `trace`, `off`, `Critical`, `Alert`, and `Emergency`?
5. Which stable `APP-NAME`, `PROCID`, and `MSGID` values should the tracker emit?
6. Which tracing fields become RFC structured data, what enterprise ID or namespacing is used for `SD-ID`, and how are malformed/non-ASCII keys and values handled?
7. Does the daemon continue to send normal diagnostics to stderr as required by the CLI output ADR, or does a selected syslog sink replace that stream?
8. Which deployment modes must be supported: native package/service, rootless container, privileged container, and manual executable invocation?

## Recommended Issue Reshaping

Retain issue #387 as an umbrella or research issue. Split the implementation work only after the decisions above are recorded:

1. Define the logging-output architecture and RFC 5424 configuration contract.
2. Implement and test a standards-compliant RFC 5424 formatter and severity/facility mapping.
3. Add an optional syslog transport or integrate with the selected platform logger.
4. Document deployment-owned file rotation, directory, and permissions; only add application-managed files if a supported deployment requires them.

This avoids making the tracker responsible for OS-level policy where the container runtime, systemd/journald, or syslog daemon is the appropriate owner.

## Sources

- RFC 5424, sections 6, 6.2, and 6.3: <https://www.rfc-editor.org/rfc/rfc5424.txt>
- Current v3 logging setup: `packages/configuration/src/v3_0_0/logging.rs`
- CLI output contract: `docs/adrs/20260519000000_define_global_cli_output_contract.md`
- Events principle: `docs/adrs/20260727000000_events_are_objective_facts.md`
- Sensitive-data logging policy: `docs/adrs/20260822094338_adopt_secrecy_for_sensitive_values.md`
