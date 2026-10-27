---
semantic-links:
  skill-links:
    - check-runner-health
  related-artifacts:
    - docs/self-hosted-runner.md
    - docs/self-hosted-runner-reports/README.md
    - .github/skills/dev/maintenance/check-runner-health/SKILL.md
---

<!-- skill-link: check-runner-health -->

# Runner Health Report Template

Copy the report skeleton below to
`docs/self-hosted-runner-reports/<YYYYMMDD-HHMMSS>-runner-report.md` (UTC time of the first
measurement) and replace every placeholder with a value you measured. Do not invent or round
results, and do not reuse numbers from an earlier report. Keep the report read-only evidence: a
fix belongs in an issue and a pull request, not in the report.

Never record the runner's public IP address, tokens, or other credentials.

Use the `check-runner-health` skill for the exact commands that produce each value.

## Report Skeleton

````markdown
---
semantic-links:
  related-artifacts:
    - docs/self-hosted-runner.md
    - docs/self-hosted-runner-reports/README.md
---

# Self-Hosted Runner Health Report: <YYYY-MM-DD HH:MM UTC>

## Summary

| Area                | Status                  | Headline                             |
| ------------------- | ----------------------- | ------------------------------------ |
| Runner availability | <OK / WARN / CRITICAL>  | <one sentence>                       |
| CPU                 | <OK / WARN / CRITICAL>  | <one sentence>                       |
| Memory and swap     | <OK / WARN / CRITICAL>  | <one sentence>                       |
| Disk space          | <OK / WARN / CRITICAL>  | <one sentence>                       |
| Automatic cleanup   | <OK / WARN / CRITICAL>  | <one sentence>                       |

Overall: <OK / WARN / CRITICAL>. <One to three sentences with the decision the reader needs.>

## Method

- Collected at: <UTC start time> to <UTC end time>.
- Host: `<hostname>`, reached over SSH from <where the commands ran>.
- Every command was read-only: <yes / list any exception>.

## Runner Availability

<GitHub runner status line, service state, restart count, agent version, uptime.>

## CPU

<Instantaneous load and idle, then per-day average idle, minimum idle, and peak five-minute
load average from sysstat for the available days. State the vCPU count so load is readable.>

## Memory and Swap

<`free -h` summary, swap used, out-of-memory kills in the last 30 days.>

## Disk Space

<`df -h /` and `df -i /`, `docker system df`, the largest directories, and the host-side Cargo
target cache size. Compare with the previous report.>

## Automatic Cleanup

<State of each cleanup mechanism: timer next/last run, last result, bytes reclaimed per run over
the available journal history, whether containers or images are being held, and whether any
cleanup is missing for a growing directory.>

## CI Impact

<Recent runs on the runner that failed or queued because of the runner. Link the run ids.>

## Findings

| Id  | Severity                | Finding | Evidence           |
| --- | ----------------------- | ------- | ------------------ |
| F1  | <INFO / WARN / CRITICAL> | <text>  | <section or run id> |

## Recommendations

1. <Action, owner, and the issue that tracks it. A recommendation without an issue is a draft.>

## Comparison with the Previous Report

<Link the previous report and give a table of measure, previous value, current value, and change.
List corrections to conclusions of the previous report, which is not edited. Write "first report"
when none exists.>
````
