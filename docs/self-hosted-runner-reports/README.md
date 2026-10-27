---
semantic-links:
  skill-links:
    - check-runner-health
  related-artifacts:
    - docs/self-hosted-runner.md
    - docs/templates/RUNNER-HEALTH-REPORT.md
    - .github/skills/dev/maintenance/check-runner-health/SKILL.md
---

<!-- skill-link: check-runner-health -->

# Self-Hosted Runner Health Reports

Periodic, read-only snapshots of the self-hosted CI runner described in
[self-hosted-runner.md](../self-hosted-runner.md): CPU, memory, disk space, and whether the automatic
cleanup works. Each report is one immutable file named `<YYYYMMDD-HHMMSS>-runner-report.md` (UTC).
Do not edit an old report; add a new one and compare it with the previous one.

To produce a report, follow the `check-runner-health` skill
([SKILL.md](../../.github/skills/dev/maintenance/check-runner-health/SKILL.md)). The report structure is
[RUNNER-HEALTH-REPORT.md](../templates/RUNNER-HEALTH-REPORT.md).

## Reports

Newest first. Update this table in the same pull request that adds a report.

| Report                                                   | Overall  | Disk used of `/` | Build cache | Headline                                                           |
| -------------------------------------------------------- | -------- | ---------------- | ----------- | ------------------------------------------------------------------ |
| [2026-10-27 11:43 UTC](20261027-114300-runner-report.md) | WARN     | 51%              | 127.2 GB    | Recovered and fix installed; waiting for the first scheduled runs. |
| [2026-10-27 09:12 UTC](20261027-091200-runner-report.md) | CRITICAL | 100%             | 253.6 GB    | Disk full; the daily prune freed 0 B for three days in a row.      |
