---
doc-type: issue
issue-type: enhancement
status: open
github-issue: 387
spec-path: docs/issues/open/387-implement-logging-using-rfc-5424-syslog-format/ISSUE.md
branch: "387-rfc-5424-syslog-logging"
related-pr: null
last-updated-utc: 2026-08-26
semantic-links:
  related-artifacts:
    - https://github.com/torrust/torrust-tracker/issues/387
    - docs/issues/open/387-implement-logging-using-rfc-5424-syslog-format/rfc-5424-current-state-analysis.md
---

# Issue #387 - Implement Logging Using RFC 5424 Syslog Format

> **Source**: GitHub issue [#387](https://github.com/torrust/torrust-tracker/issues/387), opened by [Cameron (da2ce7)](https://github.com/da2ce7) on 2023-08-27. The issue content below is preserved verbatim, apart from this source note and Markdown link normalization.

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
