---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2606-2278-validate-audit-record-merged-fix-subjects/ISSUE.md
last-updated-utc: 2026-10-21 11:51
---

# Manual Verification Evidence

## Purpose

Record real verification of the `validate-audit-record.py` defect. This file holds the initial
reproduction (V1, V2), recorded before the specification review as the `fix-bug` skill requires.
The final like-for-like recheck is added at implementation time and is still pending.

## Environment and Prerequisites

- Date and time (UTC): 2026-10-21 11:50, runs listed per step below.
- Artifact under test: the unmodified `.github/skills/dev/pr-reviews/process-pr-review/scripts/validate-audit-record.py`,
  run from the working tree of the PR branch whose only change to `develop` is the commit
  `docs(issues): add issue specification for #2606`.
- Operating system / environment: Linux. Python 3.14.4 (`python3 --version`), `gh version 2.76.2
  (2025-07-30)` (`gh --version`).
- Ref states, read with `git rev-parse` after `git fetch torrust develop`:

  | Ref                                      | Commit                                     | Note                                         |
  | ---------------------------------------- | ------------------------------------------ | -------------------------------------------- |
  | `HEAD` (the PR branch)                   | `a18cd04a7b83006129135eb0200ed87acacb06fd` | Contains the audited spec commit only        |
  | `torrust/develop`                        | `9f20a89b89a55734bc845b2d6bfc2389080c934c` | Contains every audited fix commit            |
  | local `develop`                          | `954c5611e33230a699bd2da9e24d489d406669fa` | Stale: behind `torrust/develop`              |

- Prerequisites: authenticated `gh` (the script fetches review comments through `gh api`). The
  audits of PR #2584 and PR #2599 are on `develop`, so the working tree has both audit files.

## Verification Processes

All steps ran one at a time, in the order below, and each finished before the next started.

### V1 - Merged audits fail with an up-to-date base

- Goal: observe whether a valid audit is rejected when `--base` is current.
- Initial state: the three audited fix subjects exist on `torrust/develop`. Checked first with
  `git log torrust/develop --format=%s | grep -cFx '<subject>'`, which printed `1` for each of:
  `docs(issues): clarify manual runs and folder prefix in #2581 spec`,
  `docs(issues): [#2581] count unresolved threads over the whole PR in the poller exit condition`,
  and `docs(pr-reviews): move the WAIT-001 closure entry into the Processing Log`.
- Status: `DONE` (reproduction). The final green recheck is pending.

#### Steps Performed

1. `python3 .github/skills/dev/pr-reviews/process-pr-review/scripts/validate-audit-record.py --pr-number 2584 --base torrust/develop`
   (started 11:50:32.579, ended 11:50:36.406 UTC).
2. `python3 .github/skills/dev/pr-reviews/process-pr-review/scripts/validate-audit-record.py --pr-number 2599 --base torrust/develop`
   (started 11:50:36.409, ended 11:50:39.559 UTC).

#### Observed Result

```text
$ ... --pr-number 2584 --base torrust/develop
F1: resolution reference not a commit subject on branch: docs(issues): clarify manual runs and folder prefix in #2581 spec
F2: resolution reference not a commit subject on branch: docs(issues): clarify manual runs and folder prefix in #2581 spec
WAIT-001: resolution reference not a commit subject on branch: docs(issues): [#2581] count unresolved threads over the whole PR in the poller exit condition
{"status": "failed", "rows": 4, "log_entries": 6, "failures": 3}
exit=1

$ ... --pr-number 2599 --base torrust/develop
AUDIT-001: resolution reference not a commit subject on branch: docs(pr-reviews): move the WAIT-001 closure entry into the Processing Log
{"status": "failed", "rows": 1, "log_entries": 1, "failures": 1}
exit=1
```

#### Conclusion

Outcome: **Reproduced**. The wrong outcome itself was observed: both audits are valid and every cited
fix subject exists on `torrust/develop` (Initial state), yet the validator exits 1 and reports each
subject as "not a commit subject on branch". The cause is that `<base>..HEAD` no longer contains a
merged fix once the base is current. WAIT-001's subject is also not in the audited PR #2584, because
the fix merged in PR #2596.

### V2 - Result depends on how stale the base is

- Goal: observe that the same audit passes or fails depending on the base ref, with unchanged `HEAD`.
- Initial state: local `develop` is `954c5611e`, behind `torrust/develop` (`9f20a89b8`), as in the table
  above. `HEAD` did not change during V1 and V2.
- Status: `DONE` (reproduction of the dependency on the base).

#### Steps Performed

1. `python3 .github/skills/dev/pr-reviews/process-pr-review/scripts/validate-audit-record.py --pr-number 2584 --base develop`
   (started 11:50:39.562, ended 11:50:43.296 UTC). The default `--base` is `develop`, which resolves to the
   stale local branch.
2. `python3 .github/skills/dev/pr-reviews/process-pr-review/scripts/validate-audit-record.py --pr-number 2584 --base fcd78975c^1`
   (started 11:50:43.298, ended 11:50:46.923 UTC). `fcd78975c^1` is the parent of the PR #2584 merge commit, so
   the range still contains the fix commits.

#### Observed Result

```text
$ ... --pr-number 2584 --base develop
{"status": "ok", "rows": 4, "log_entries": 6, "failures": 0}
exit=0

$ ... --pr-number 2584 --base fcd78975c^1
{"status": "ok", "rows": 4, "log_entries": 6, "failures": 0}
exit=0
```

#### Conclusion

Outcome: **Reproduced**. The same audit, with the same `HEAD`, exits 0 with `--base develop` and
`--base fcd78975c^1`, and exits 1 with `--base torrust/develop` (V1). The result depends on the base
ref, not on the audit. The default `--base develop` passes here only because the local branch is stale.

## Failures and Follow-up

No failed or blocked process. Pending: the final like-for-like recheck of V1 and V2 after the fix, and
the V3 scenario (rejection of a subject that exists in no commit).
