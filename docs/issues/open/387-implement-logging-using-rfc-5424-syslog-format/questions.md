---
doc-type: research-questions
status: open
related-issue: 387
last-updated-utc: 2026-08-26
semantic-links:
  related-artifacts:
    - docs/issues/open/387-implement-logging-using-rfc-5424-syslog-format/ISSUE.md
    - docs/issues/open/387-implement-logging-using-rfc-5424-syslog-format/rfc-5424-current-state-analysis.md
    - https://www.rfc-editor.org/rfc/rfc5424.txt
---

# Research Questions for Issue #387

This document records the questions that must be answered before deciding whether RFC 5424 support is worth implementing. It is deliberately separate from the issue description and current-state analysis so the research can remain open-ended.

## Q1. Why does RFC 5424 matter?

### Short answer

RFC 5424 matters when an operator needs the tracker to send interoperable, machine-readable records directly to a syslog daemon or collector. It standardizes the record header, severity, facility, timestamp, application identity, and optional structured data so that syslog-aware infrastructure can route, parse, retain, and alert on tracker messages consistently.

It is not inherently a better way for the tracker to create diagnostic events. The tracker already uses `tracing`, which provides structured events, levels, spans, and subscriber layers. RFC 5424 is an output and interoperability standard for one particular logging ecosystem.

### Benefits if implemented

- **Direct syslog integration**: The tracker could send logs to compatible syslog daemons and collectors over established syslog transports rather than relying on stdout/stderr capture.
- **Portable record envelope**: A receiving system can read standard `PRI`, timestamp, hostname, application name, process identifier, message identifier, and structured-data fields without tracker-specific parsing rules.
- **Facility-based routing**: Operators could use the syslog facility and severity to route tracker logs separately from other services, choose retention policies, or trigger alerting rules.
- **Structured-data interoperability**: If the tracker defined a stable RFC 5424 structured-data schema, syslog-aware tools could query tracker attributes without parsing free-form text.
- **Compatibility with existing operations tooling**: Some organizations standardize on syslog relays, SIEM products, and central log collectors that accept RFC 5424 directly.

### Capabilities the tracker does not have today

The current tracker logging setup does not itself provide:

- A standards-compliant RFC 5424 message envelope with `PRI`, facility, syslog protocol version, and syslog header fields.
- A built-in syslog client transport to a daemon through UDP, TCP, or a Unix-domain socket.
- A tracker-defined RFC 5424 structured-data schema for fields such as torrent hash, client label, protocol, or request context.
- A standard syslog facility by which an operator can route the tracker independently in syslog infrastructure.

### Capabilities the tracker already has

The absence of RFC 5424 does not mean the tracker lacks logging or observability:

- `tracing` provides severity filtering and structured event fields to the configured subscriber.
- The current configuration supports dynamic filtering; the newer v3 schema also supports multiple human-readable and JSON output styles.
- Operators can capture stdout/stderr using their chosen runtime, system logger, container platform, or log collector.
- Torrust Tracker Deployer and the Tracker Demo already keep rotation, file retention, directory, ownership, and permissions in deployment infrastructure, where those policies belong.
- Events, metrics, and health checks remain separate observability mechanisms; RFC 5424 would not replace them.

### Decision implication

The relevant question is not whether RFC 5424 is objectively better than `tracing`. The relevant question is whether a current or planned deployment needs **direct, standards-based syslog delivery** strongly enough to justify a new sink, configuration, dependency review, and ongoing support.

Absent that requirement, the current `tracing` output plus operator-managed collection keeps the tracker simpler while preserving its existing logging capabilities.

## Q2. Does the current tracker deployment model need direct syslog delivery?

### Answer

Not as a general capability. Direct RFC 5424 delivery is most useful for a horizontally distributed service fleet, where many instances send records to common syslog infrastructure for correlation, routing, retention, and alerting.

The current tracker architecture does not use horizontally interchangeable tracker replicas. Each tracker process owns in-memory swarm state that is not separated into an independently shared coordination layer. A larger deployment therefore scales one tracker process vertically rather than running multiple equivalent tracker instances behind a load balancer.

For the expected single-instance tracker deployment, the host, container runtime, or operator-managed log agent can collect the existing stderr output and forward it to a central syslog service, SIEM, or another log collector. That supplies centralized retention and analysis without requiring the tracker to become a syslog client.

### Decision implication

The current deployment model does not justify implementing RFC 5424 support, either as a complete formatter or as a partial `tracing-rfc-5424` integration. The issue should remain research only. Reconsider it only if a real deployment or customer requirement needs the tracker itself to deliver RFC 5424 records directly to a syslog daemon.

### Evidence

- RFC 5424 defines the syslog message format and header fields: <https://www.rfc-editor.org/rfc/rfc5424.txt>
- Current tracker logging and RFC gap assessment: `rfc-5424-current-state-analysis.md`
