---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2179-fix-docker-e2e-package-flag/ISSUE.md
last-updated-utc: 2026-09-19 13:17
---

# Manual Verification Evidence - Docker E2E Package Selection

## Purpose

Record GitHub Actions verification that the corrected package-qualified runner commands execute in
the event contexts governed by the `docker-e2e` job condition.

## Environment and Prerequisites

- Date and time (UTC): 2026-09-19 12:43-13:17
- Artifact under test: commit `a58ff983c0d981d2c2050e1f5a92af7017a34f36`
- Environment: GitHub-hosted `ubuntu-latest` runner
- Prerequisites: branch `2179-fix-docker-e2e-package-flag` pushed to the `josecelano` fork

## Verification Processes

### V1 - Feature-Branch Docker E2E

- Goal: Confirm a feature-branch push admits and completes the `Docker E2E` job.
- Initial state: The branch contains explicit `torrust-tracker-e2e-tools` package selection for all
  four runner commands.
- Status: `DONE`

#### Steps Performed

1. Pushed `2179-fix-docker-e2e-package-flag` to `josecelano/torrust-tracker`.
2. Opened the resulting `Testing` workflow run and inspected the `Docker E2E` job and its steps.

#### Observed Result

Workflow run: <https://github.com/josecelano/torrust-tracker/actions/runs/35443663968>

```text
Testing: success
Docker E2E: success
Run E2E Tests: success
Run qBittorrent E2E Test (SQLite): success
Run qBittorrent E2E Test (MySQL): success
Run qBittorrent E2E Test (PostgreSQL): success
```

#### Conclusion

The feature-branch push executed all four E2E runners and completed successfully, satisfying M1.

### V2 - Pull Request to Develop

- Goal: Confirm a pull request targeting `develop` continues to skip the duplicate `Docker E2E`
  job under the unchanged guard.
- Initial state: No pull request has been opened yet.
- Status: `TODO`

#### Steps Performed

Pending pull request creation.

#### Observed Result

```text
Pending
```

#### Conclusion

Pending.

## Failures and Follow-up

No failures observed. Complete V2 after opening the pull request.
