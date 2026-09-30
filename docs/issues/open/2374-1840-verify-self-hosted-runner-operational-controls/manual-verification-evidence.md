---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2374-1840-verify-self-hosted-runner-operational-controls/ISSUE.md
last-updated-utc: 2026-09-30 07:03
semantic-links:
  related-artifacts:
    - ISSUE.md
    - ../../closed/2323-1840-hetzner-self-hosted-ci-runner/manual-verification-evidence.md
---

# Manual Verification Evidence: #2374 Operational Controls

## Purpose

Record the M5 and M7 observations performed for #2374. The detailed original evidence is retained
in #2323 because this follow-up completed that issue's deferred scenarios.

## Environment and Prerequisites

- Date and time (UTC): 2026-09-29
- Artifact under test: disposable PR #2378 for M5; Dependabot PR #2369 and external-contributor
  PR #2376 for M7.
- Operating system / environment: `torrust-runner-01` and GitHub-hosted Actions runners.
- Prerequisites and setup performed: runner online and idle before M5; fork approval policy
  `all_external_contributors`; Dependabot routes configured in `container.yaml` and `testing.yaml`.

## Verification Processes

### V1 - Runner-Offline Recovery (M5)

- Goal: show that `timeout-minutes` does not limit queued time, then verify restart recovery and
  the GitHub-hosted fallback.
- Initial state: the runner was online and idle; PR #2378 reduced the disposable test job timeout
  from 90 minutes to 2 minutes.
- Status: `DONE`

#### Steps Performed

1. Stopped the runner service at 18:03:34 UTC and verified it was offline before opening PR #2378.
2. Inspected Container runs `36610063594`, `36610612944`, and `36611356707` with the GitHub
   Actions runs and jobs APIs.
3. Restarted the runner, observed the queued self-hosted jobs, then pushed the PR-local fallback
   changing `runs-on` to `ubuntu-latest` and confirmed the final run's runner assignment.

#### Observed Result

```text
run 36610063594 | self-hosted | created 18:09:40Z | started 18:14:33Z | queue 293 s | cancelled 18:16:40Z
run 36610612944 | self-hosted | created 18:14:13Z | started 18:16:42Z | queue 149 s | cancelled 18:18:49Z
run 36611356707 | ubuntu-latest | created 18:20:31Z | started 18:20:33Z | queue 2 s | cancelled 18:23:12Z
```

#### Conclusion

M5 met: the first job remained queued for 293 seconds, beyond its 2-minute execution limit, and
ran after the service restarted. The PR-local fallback began on a GitHub-hosted runner. See #2323
[V5](../../closed/2323-1840-hetzner-self-hosted-ci-runner/manual-verification-evidence.md#v5---runner-offline-recovery-m5)
for the job links and complete observation record.

### V2 - Untrusted-Code Routing (M7)

- Goal: verify GitHub-hosted routing for Dependabot and the pre-approval workflow gate for an
  external contributor.
- Initial state: Dependabot routing was active after #2323; external contributions required
  approval.
- Status: `DONE`

#### Steps Performed

1. Inspected Container and Testing jobs for Dependabot PRs #2369 and #2338 using the Actions runs
   and jobs APIs.
2. Maintainer updated #2369 from `develop`, then pushed signed temporary trigger and restoration
   commits to observe the branch-push `Docker E2E` route.
3. Inspected PR #2376 before and after maintainer workflow approval.

#### Observed Result

```text
Dependabot #2369 and #2338: Test (Docker) and Docker E2E jobs used GitHub Actions runners with labels=ubuntu-latest.
Updated #2369 PR: Test (Docker) used GitHub Actions 1000091297 with labels=ubuntu-latest.
Later Dependabot branch push: Docker E2E job 109621539097 started on GitHub Actions 1000091340 with labels=ubuntu-latest.
External PR #2376: no workflow ran before approval; Docs Lint started after approval on GitHub Actions 1000091230.
```

#### Conclusion

M7 met: Dependabot routing remained GitHub-hosted for both pull-request and later branch-push
events, and the external-contributor workflow did not run before approval. The branch-push
observation establishes runner assignment, not `Docker E2E` success. See #2323
[V7](../../closed/2323-1840-hetzner-self-hosted-ci-runner/manual-verification-evidence.md#v7---untrusted-code-routing-m7)
for the complete run and job record.

## Failures and Follow-up

The maintainer's Dependabot merge update created no branch-push workflow, and its pull-request
`Docker E2E` job was skipped because the target was `develop`. The temporary signed trigger and
immediate signed revert supplied the required branch-push observation without a net source-content
change. The reusable lesson is recorded in [the implementation retrospective](implementation-retrospective.md).
