---
doc-type: manual-verification-evidence
issue-spec: docs/issues/closed/2606-2278-validate-audit-record-merged-fix-subjects/ISSUE.md
last-updated-utc: 2026-10-26 15:47
---

# Manual Verification Evidence

## Purpose

Record real verification of the `validate-audit-record.py` defect. This file holds the initial
reproduction (V1, V2), recorded before the specification review as the `fix-bug` skill requires.
The final like-for-like recheck, V3, and the rerun of the existing audits are in the last section.

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
- Status: `DONE` (reproduction). The final green recheck is in the last section of this file.

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

No failed or blocked process in V1 and V2. The final recheck of V1 and V2, the V3 scenario, and the
rerun of the existing audits (T10) follow.

## Final Recheck (After the Fix)

Artifact under test: the fixed script at the commit
`docs(pr-reviews): [#2606] describe the three commit-subject sources of validate-audit-record`
(`HEAD` of branch `2606-validate-audit-record-merged-fix-subjects`). Python 3.14.4, `gh version 2.76.2 (2025-07-30)`.
All runs on 2026-10-24 UTC, one after another. `torrust/develop` was `abce11ec8` (after
`git fetch torrust develop`).

### V1 (recheck) - Merged audits pass with an up-to-date base

Same commands as V1, same `--base torrust/develop`. Started 19:54 UTC.

```text
$ ... --pr-number 2584 --base torrust/develop
{"status": "ok", "rows": 4, "log_entries": 6, "failures": 0}   exit=0
$ ... --pr-number 2599 --base torrust/develop
{"status": "ok", "rows": 1, "log_entries": 1, "failures": 0}   exit=0
```

Outcome: **Green**. The 4 failures of V1 are gone (M1, AC1). WAIT-001 passes through PR #2596, named in
its `Follow-up PR URL` field.

### V2 (recheck) - Result is independent of the local `develop`

`HEAD` (the commit named above) was unchanged before, between, and after the runs (19:54:57 to
19:55:16 UTC). Local `develop` was `954c5611e33230a699bd2da9e24d489d406669fa` (stale) while
`torrust/develop` was `abce11ec8a510010e2b69a5f77f0bc005b8541dc`.

1. `--base develop` with the stale local `develop`: PR #2584 and PR #2599 both `"status": "ok"`.
2. `git fetch torrust develop:develop` fast-forwarded local `develop` to `abce11ec8`, equal to
   `torrust/develop`.
3. The same two `--base develop` commands: both `"status": "ok"`, identical summaries.

Outcome: **Green**. Same result with a stale and a current local `develop`, `HEAD` unchanged (M2, AC2).
Side effect: the maintainer's local `develop` branch is now at `abce11ec8`.

### V3 - A bogus subject is still rejected

Goal: a `Resolution reference` that exists in no commit still fails. Steps: copied
`docs/pr-reviews/pr-2599-review/PR-REVIEW.md` to a temporary file, changed the `Resolution reference`
of AUDIT-001 to `fix(none): this subject exists in no commit`, ran (19:55 UTC):

```text
$ ... --pr-number 2599 --base torrust/develop --audit-file <copy>
AUDIT-001: resolution reference not a commit subject on branch: fix(none): this subject exists in no commit
{"status": "failed", "rows": 1, "log_entries": 1, "failures": 1}
exit=1
```

Outcome: **Green**. Exactly one `not a commit subject` failure (M3, AC3). The temporary copy was deleted.

### T10 - Existing audits with a `FIXED` row

Ran the fixed script with `--base torrust/develop` on the 157 `PR-REVIEW.md` files under
`docs/pr-reviews/` that contain the word `FIXED` (started 19:55 UTC). The count is of files with the
word, not of audits with a `FIXED` table row: some files hold it only in their copied Status Values,
and the reviewer of PR #2641 counted 148 files with a `FIXED` row, which I did not recount. The rerun
covers all 157: 146 report `"status": "ok"`, 11 report `"failed"`:
PR #2232, #2235, #2236, #2237, #2239, #2249, #2252, #2259, #2269, #2272, and #2461.

To tell whether the change caused them, the unmodified script from `torrust/develop` ran on the same
11 audits with the same base. All 11 failed there too, with more `not a commit subject` failures
(for example #2269: 29 before, 2 after; #2461: 19 before, 0 after). No audit passes with the old
script and fails with the new one.

The remaining `not a commit subject` failures are in four audits and are audit content, not the
defect: their `Resolution reference` fields hold prose (`Superseded by F2 reply`,
`This audit update.`, `Follow-up documented in ...`) or a subject with a trailing backtick and comma
(#2272 `DOC-2` and `F7`). The other failures in the 11 audits are unrelated rules (review ID, reply
URL count) and were not touched. Repairing those historical audits is out of scope.

Outcome: existing audits do not regress.

### AC4 - `gh` failure exits 2 (observed during T10)

While re-running PR #2236 for the comparison, GitHub returned a transient `HTTP 502` for the commits
endpoint. The fixed script printed
`Error: gh api repos/torrust/torrust-tracker/pulls/2236/commits failed: gh: Server Error (HTTP 502)`
and did not fall back to `<base>..HEAD`. The next two runs succeeded. The unit test
`test_it_should_exit_with_a_usage_error_naming_the_endpoint_when_listing_commits_fails` covers it
deterministically.
