---
doc-type: issue
issue-type: enhancement
status: done
priority: p2
github-issue: 387
spec-path: docs/issues/closed/387-implement-logging-using-rfc-5424-syslog-format/ISSUE.md
branch: "387-rfc-5424-syslog-logging"
related-pr: null
last-updated-utc: 2026-09-01 10:27
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - .github/skills/dev/planning/create-issue/SKILL.md
    - https://github.com/torrust/torrust-tracker/issues/387
    - docs/issues/closed/387-implement-logging-using-rfc-5424-syslog-format/rfc-5424-current-state-analysis.md
    - docs/issues/closed/387-implement-logging-using-rfc-5424-syslog-format/questions.md
---

# Issue #387 - Implement Logging Using RFC 5424 Syslog Format

> **Source**: GitHub issue [#387](https://github.com/torrust/torrust-tracker/issues/387), opened by [Cameron (da2ce7)](https://github.com/da2ce7) on 2023-08-27. The issue content below is preserved verbatim, apart from this source note and Markdown link normalization.

## Research Outcome

Research completed on 2026-08-26 concludes that RFC 5424 support should not be implemented, either completely or partially, at this time. The tracker should retain its existing `tracing`-based logging and operator-managed log collection.

The current tracker architecture scales a process vertically around process-local, in-memory swarm state. It is not currently deployed as a horizontally interchangeable fleet of tracker replicas, which is the main scenario where direct syslog delivery and central correlation are compelling. For the expected single-instance deployment, the container runtime, host logger, or operator log agent can collect stderr and forward it to central infrastructure when required.

No implementation subissues or `tracing-rfc-5424` integration should be created now. Reconsider the issue only when a concrete deployment or customer requires the tracker process itself to send RFC 5424 records directly to a syslog daemon. Cameron should decide whether to close #387 as out of current priorities or retain it as a deferred enhancement.

- [Current-state analysis](rfc-5424-current-state-analysis.md)
- [Research questions](questions.md)

## Implement Logging

Enhance the program's logging functionality by adopting the [RFC 5424 syslog format](https://tools.ietf.org/html/rfc5424). This format ensures structured, consistent log entries that align with industry best practices. Follow these steps to implement the update:

### Integrate RFC 5424 Format:

Revise the logging mechanism to adhere to the `RFC 5424`, ensuring each log entry includes priority level, timestamp, hostname, program name, and structured data when applicable.

### Manage Severity Levels:

Implement the recommended severity levels (e.g., emergency, alert, warning, notice, info, debug) to accurately reflect the importance of log messages.

### Configure Log Rotation:

Develop a log rotation strategy to control log file size and retention, preventing excessive disk space consumption.

### Define Log Directory:

Designate a dedicated directory (e.g., `/var/log/torrust/tracker`) for log files, maintaining alignment with Linux directory structure conventions.

### Enforce Permissions:

Apply appropriate permissions and ownership to log files and directories to ensure authorized access and modification.

### Dynamic Log Levels:

Enable log level configuration (e.g., INFO, DEBUG, ERROR) to control verbosity based on configuration settings.

### Test and Document:

Thoroughly test the updated logging mechanism, verifying adherence to `RFC 5424` and proper handling of structured data. Document the changes for clarity.

## Expected Outcomes:

- Consistent and structured log entries following `RFC 5424`.
- Efficient log file management with rotation and controlled disk space usage.
- Improved program monitoring and troubleshooting through enhanced log data.

## Related Discussion

- [torrust/torrust-demo#4](https://github.com/torrust/torrust-demo/issues/4)
