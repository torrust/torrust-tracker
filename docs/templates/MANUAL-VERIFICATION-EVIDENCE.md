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

## Preserving Verification Artifacts

`.tmp/` and other git-ignored paths are not part of the repository, so a path
there is not evidence once the run ends. Record everything a reviewer needs in
this issue folder:

- Embed small artifacts, such as a tracker configuration file, verbatim in
  this document.
- Store larger artifacts, such as full logs or captured responses, in an
  `evidence/` subfolder of the issue folder and link them from here.
- Record runtime outputs (database rows, responses, relevant log lines) inline
  in each process.

A local `.tmp/` path may still be mentioned to show where a run wrote its
files, but never as the only copy of something the evidence relies on.

## Environment and Prerequisites

- Date and time (UTC):
- Artifact under test: {Name PR-branch patches by their Conventional Commit subject, never by
  branch commit id: rebases onto `develop` rewrite ids. Commit ids are durable only for commits
  already on `develop`, tags, or external repositories.}
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
