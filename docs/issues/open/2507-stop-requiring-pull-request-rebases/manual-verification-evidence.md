---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2507-stop-requiring-pull-request-rebases/ISSUE.md
last-updated-utc: 2026-10-14 06:50
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

- Date and time (UTC): 2026-10-14 06:45 onward
- Artifact under test: GitHub's pull-request merge refs for `torrust/torrust-tracker` and, from
  T2 on, the maintainer merge tool on this issue's implementation branch (patches named by their
  Conventional Commit subject).
- Operating system / environment: Linux workstation; `git` and GitHub CLI `gh` authenticated as
  `josecelano`.
- Prerequisites and setup performed: `git fetch torrust` with `develop` at `d383d4b45`
  (`Merge torrust/torrust-tracker#2508`).

## Verification Processes

### T1 - Merge-Ref Refresh Experiment

- Goal: find the least disruptive way to make GitHub recompute `refs/pull/<n>/merge` against the
  current `develop` tip before a maintainer merge (spec task T1).
- Initial state: eight open pull requests; `develop` at `d383d4b45`.
- Status: `IN_PROGRESS`

#### Option (a): Wait and Re-Query

Steps performed (2026-10-14 06:45 UTC), for every open pull request:

```sh
git fetch torrust "+refs/pull/<n>/merge:refs/remotes/tmp/pr-<n>-merge" \
  "+refs/pull/<n>/head:refs/remotes/tmp/pr-<n>-head"
git log -1 --format='%p %cI' refs/remotes/tmp/pr-<n>-merge
git log -1 --format='%cI' refs/remotes/tmp/pr-<n>-head
git update-ref -d refs/remotes/tmp/pr-<n>-merge
git update-ref -d refs/remotes/tmp/pr-<n>-head
```

Observed result (merge-ref first parent, and when the merge ref and the head were last
committed):

| PR    | Merge-ref base | Merge ref computed (UTC) | Last head commit (UTC) |
| ----- | -------------- | ------------------------ | ---------------------- |
| #2515 | `8af007187`    | 2026-10-14 06:31:19      | 2026-10-14 06:29:32    |
| #2514 | `b5f1801c6`    | 2026-10-14 02:03:54      | 2026-10-14 02:03:47    |
| #2513 | `b5f1801c6`    | 2026-10-14 02:03:38      | 2026-10-14 02:03:31    |
| #2512 | `b5f1801c6`    | 2026-10-14 02:03:28      | 2026-10-14 02:03:22    |
| #2511 | `b5f1801c6`    | 2026-10-14 02:03:18      | 2026-10-14 02:03:12    |
| #2510 | `b5f1801c6`    | 2026-10-14 02:03:15      | 2026-10-14 02:03:07    |
| #2500 | `7711ade56`    | 2026-10-13 11:38:17      | 2026-10-08 19:48:09    |
| #2430 | `7711ade56`    | 2026-10-13 11:36:59      | 2026-10-03 15:09:07    |

`develop` advanced after every listed computation: `8af007187` (#2509) and `d383d4b45` (#2508)
after the 02:03 group, and `c7aeaf46c` (#2431) through `d383d4b45` after the 11:3x pair.

Findings:

- Every merge ref computed in the last day follows its head push by about 6 to 10 seconds, and none
  was recomputed when `develop` later advanced through a maintainer-pushed merge.
- #2500 and #2430 were recomputed once, about 9 minutes after `7711ade56`
  (`Merge torrust/torrust-tracker#2486`, committed 11:28:27 UTC) was pushed, with no head push and
  no timeline event at that time. Six later `develop` merges over about 19 hours triggered no further
  recomputation.

Conclusion: waiting is not a reliable refresh. GitHub sometimes recomputes after a base push, with
no visible trigger, but usually does not. Option (a) is rejected.

#### Option (b): Close and Reopen, Option (c): Update Branch

Not yet run; see the progress log of the specification for the agreed procedure.

## Failures and Follow-up

None yet.
