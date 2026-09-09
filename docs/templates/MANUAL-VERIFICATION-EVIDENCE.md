---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/{number}-{short-description}/ISSUE.md
last-updated-utc: YYYY-MM-DD HH:MM
---

# Manual Verification Evidence

## Purpose

Record real, human-oriented verification of the completed behavior. This is
evidence from commands or interactions actually performed against the artifact;
do not invent commands, output, logs, or results.

## Environment and Prerequisites

- Date and time (UTC):
- Artifact under test:
- Operating system / environment:
- Prerequisites and setup performed:

## Verification Processes

Add one section per manual verification process. A process may cover one or
more issue-spec scenarios when its steps and evidence clearly identify each
result.

### V1 - {Scenario Name}

- Goal:
- Initial state:
- Status: `TODO` / `IN_PROGRESS` / `DONE` / `FAILED` / `BLOCKED`

#### Steps Performed

1. {Human-oriented action or exact command actually executed.}
2. {Next action or command.}

#### Observed Result

```text
{Actual relevant program output, response, or tracker log.}
```

#### Conclusion

{State whether the observed result met the issue scenario's expected result.}

## Failures and Follow-up

Record any failed or blocked process, diagnosis, remediation, and whether the
scenario was rerun.
