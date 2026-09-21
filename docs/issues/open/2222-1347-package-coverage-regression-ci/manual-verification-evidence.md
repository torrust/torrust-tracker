---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2222-1347-package-coverage-regression-ci/ISSUE.md
last-updated-utc: 2026-09-21 18:11
---

# Manual Verification Evidence

## Purpose

Record real, human-oriented verification of Issue #2222's completed report-only coverage
comparison. No commands, output, logs, or results are recorded until the corresponding scenario
is actually executed.

## Environment and Prerequisites

- Date and time (UTC): Pending implementation.
- Artifact under test: Pending implementation.
- Operating system / environment: Pending implementation.
- Prerequisites and setup performed: Pending implementation.

## Verification Processes

### M1 Base/Head Double-Build Proof of Concept

- Goal: Verify a controlled changed-package comparison at the pull request base and head
  revisions without a persisted baseline artifact.
- Initial state: Pending implementation.
- Status: `TODO`

#### Steps Performed

Pending implementation.

#### Observed Result

Pending implementation.

#### Conclusion

Pending implementation.

### M2 Dynamic Changed-Package Matrix

- Goal: Verify that the workflow matrix includes only directly changed comparable packages while
  the final report job remains present for every run.
- Initial state: Pending implementation.
- Status: `TODO`

#### Steps Performed

Pending implementation.

#### Observed Result

Pending implementation.

#### Conclusion

Pending implementation.

### M3 Equal, Improved, and Reduced Coverage

- Goal: Verify ratio calculation reports equal, increased, and decreased coverage; warn only for a
  decrease greater than five percentage points without blocking a merge.
- Initial state: Pending implementation.
- Status: `TODO`

#### Steps Performed

Pending implementation.

#### Observed Result

Pending implementation.

#### Conclusion

Pending implementation.

### M4 Exceptional Package Outcomes

- Goal: Verify documentation/workspace-config-only changes and new, deleted, or unmatched renamed
  package outcomes.
- Initial state: Pending implementation.
- Status: `TODO`

#### Steps Performed

Pending implementation.

#### Observed Result

Pending implementation.

#### Conclusion

Pending implementation.

### M5 Fork Pull Request

- Goal: Verify the report-only check stays in the unprivileged `pull_request` workflow and the
  trusted uploader remains artifact-only.
- Initial state: Pending implementation.
- Status: `TODO`

#### Steps Performed

Pending implementation.

#### Observed Result

Pending implementation.

#### Conclusion

Pending implementation.

## Failures and Follow-up

None recorded.
