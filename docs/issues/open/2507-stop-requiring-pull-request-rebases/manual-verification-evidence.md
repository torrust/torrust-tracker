---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2507-stop-requiring-pull-request-rebases/ISSUE.md
last-updated-utc: 2026-10-14 07:30
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
- Status: `DONE` (method pending maintainer approval)

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

#### Sandbox for Options (b) to (e)

The maintainer approved a throwaway sandbox in the fork `josecelano/torrust-tracker` instead of
experimenting on upstream pull requests (2026-10-14).

- Branches `exp-2507-base` and `exp-2507-alt`, both created at `d383d4b45`, and one head branch per
  sandbox pull request, each adding one signed commit with a Markdown file under `exp-2507/`, so the
  fork's heavy workflows skip it.
- Sandbox ruleset scoped to `refs/heads/exp-2507-base`, copying upstream rulesets 87085 and 94133:
  `deletion`; `pull_request` with 0 approvals, `dismiss_stale_reviews_on_push: true`, and merge
  methods `merge`, `squash`, `rebase`; `required_signatures`. Step T5 later added
  `required_status_checks`.
- Every merge was built the way `github-merge.py` builds it and pushed directly:

  ```sh
  git switch --detach josecelano/exp-2507-base
  git merge --no-ff -S -m "Merge josecelano/torrust-tracker#<n>: <title>" josecelano/exp-2507-<x>
  git push josecelano HEAD:refs/heads/exp-2507-base
  ```

- Merge refs were polled with `git fetch josecelano "+refs/pull/<n>/merge:refs/remotes/tmp/f<n>"`
  and `git log -1 --format=%p`; the PR's recorded base with
  `gh api repos/josecelano/torrust-tracker/pulls/<n> --jq .base.sha`.
- Cleanup at 07:23 UTC: sandbox ruleset deleted, all `exp-2507-*` branches deleted from the fork,
  last open sandbox pull request (fork 19) closed.

Observed results (UTC, 2026-10-14):

| Step | Fork PR | Action                                                                                                                 | Merge ref first parent              | `base.sha`               | Push result                                                     |
| ---- | ------- | ---------------------------------------------------------------------------------------------------------------------- | ----------------------------------- | ------------------------ | --------------------------------------------------------------- |
| 1    | 9 (b)   | Baseline: push local merge of a current PR (06:52:55)                                                                  | current                             | current                  | Accepted `d383d4b45..a9de9217d`                                 |
| 2    | 8 (a)   | Negative control: push while behind (06:53:14)                                                                         | stale `d383d4b45`                   | stale                    | Rejected: `GH013` "Changes must be made through a pull request" |
| 3    | 8 (a)   | Option (b): close and reopen (06:53:30); ref refreshed by 06:53:42; push 06:53:42                                      | refreshed                           | refreshed                | Accepted `a9de9217d..6f51ae958`                                 |
| 4    | 10 (c)  | Option (c): `PUT pulls/10/update-branch` (06:54:03); refreshed by 06:54:15                                             | refreshed                           | refreshed                | Accepted `6f51ae958..551f4e67d`                                 |
| 5    | 11 (d)  | Option (d): base switched to `exp-2507-alt` and back (06:54:44); push 06:58:18                                         | not refreshed in 180 s              | not recorded             | Accepted `551f4e67d..96e2497eb`                                 |
| 6    | 12 (e)  | Option (d) repeated (06:59:48), polled until 07:13:36; push 07:13:51                                                   | `d383d4b45`, the `exp-2507-alt` tip | refreshed                | Accepted `c89e6bdae..8fe89bf5a`                                 |
| 7    | 13 (f)  | Control for step 6: no action, polled until 07:13:36; push 07:14:03                                                    | stale `96e2497eb`                   | stale                    | Rejected: `GH013` "Changes must be made through a pull request" |
| 8    | 13 (f)  | Same local merge after a base toggle (`base.sha` `96e2497eb` to `8fe89bf5a`); push 07:14:40                            | `d383d4b45`, the `exp-2507-alt` tip | refreshed                | Accepted `8fe89bf5a..86da8a097`                                 |
| 9    | 15 (h)  | Option (e): no-op base edit `PATCH pulls/15 base=exp-2507-base` (`base.sha` `86da8a097` to `c85c0cb84`); push 07:16:18 | stale `86da8a097`                   | refreshed                | Accepted `c85c0cb84..89fac4b9f`                                 |
| 10   | 17 (j)  | Control for step 9: title-only edit                                                                                    | not checked                         | stayed stale `86da8a097` | Not pushed                                                      |

Option (c) details: GitHub's update commit had parents `13fa21fa3 6f51ae958` (the reviewed head and
the base tip), committer `GitHub`, and `verification.verified: true` (`valid`); it passed the
`required_signatures` rule, and the reviewed commit stayed in the branch history.

Side effects of option (e) on fork 15: its timeline shows no base-change event (only the
`RenamedTitleEvent` of the step-10 control was added to fork 17), and no workflow ran for it. No
upstream workflow declares `pull_request` `types`, so the default `opened`, `synchronize`, and
`reopened` apply: options (b) and (c) rerun pull-request CI, option (e) does not.

#### Conclusion

- GitHub's `pull_request` rule accepts the maintainer's direct merge push when the pull request's
  recorded base SHA (`base.sha`) equals the branch tip. Freshness of `refs/pull/<n>/merge` is not the
  condition: steps 5, 6, 8, and 9 were accepted with a stale or foreign merge ref, and steps 2 and 7
  were rejected with a stale `base.sha`. The `github-merge.py` stale-base guard checks the merge ref
  and therefore refuses merges GitHub would accept.
- Every option except (a) refreshes `base.sha` without rewriting the reviewed commits. Option (e),
  a no-op base edit (`gh api -X PATCH repos/<owner>/<repo>/pulls/<n> -f base=<base-branch>`), adds no
  commit, needs no temporary branch, leaves no timeline event, and triggers no CI. Option (b)
  reruns CI and adds close and reopen events; option (c) adds a GitHub-signed merge commit, reruns
  CI, and dismisses approvals through `dismiss_stale_reviews_on_push`.
- Status: `DONE`, pending maintainer approval of the chosen method.

### T5 - Required Status Checks Under the Maintainer Merge

- Goal: determine whether GitHub enforces required status checks on the maintainer's direct merge
  push, and whether a maintainer can override a known-unrelated failure without disabling a ruleset.
- Initial state: the T1 sandbox with `required_status_checks` added to its ruleset
  (`strict_required_status_checks_policy: false`). Each pull request was refreshed with option (e)
  before its push.
- Status: `DONE`

| Step | Fork PR | Required context | Head state                                                                                                             | Push result                                               |
| ---- | ------- | ---------------- | ---------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------- |
| 11   | 18 (k)  | `exp-gate`       | No status (07:17:19)                                                                                                   | Rejected: `Required status check "exp-gate" is expected.` |
| 12   | 17 (j)  | `exp-gate`       | Commit status `exp-gate=success` posted through the API (07:17:33)                                                     | Accepted `89fac4b9f..2fc009604`                           |
| 13   | 18 (k)  | `exp-gate`       | Commit status `exp-gate=failure` (07:18:06)                                                                            | Rejected: `Required status check "exp-gate" is failing.`  |
| 14   | 18 (k)  | `exp-gate`       | Later commit status `exp-gate=success`, description "maintainer override: known unrelated failure" (07:18:10)          | Accepted `2fc009604..fa21f3ded`                           |
| 15   | 19 (l)  | `Docs Lint`      | Two failing `Docs Lint` Actions check runs (deliberate misspelling) and a commit status `Docs Lint=success` (07:22:14) | Rejected: `Required status check "Docs Lint" is failing.` |

GitHub kept both `exp-gate` statuses of step 13 and 14 with creator and description, so a
status-based override is auditable. While one of the `Docs Lint` check runs was still in progress,
the push was rejected with `Required status check "Docs Lint" is in progress.`

Conclusion:

- Required status checks are enforced on the maintainer's direct merge push and are read from the
  pull-request head commit, so GitHub-required checks are compatible with `github-merge.py`.
- A skipped workflow reports nothing, so a required check it would produce stays "expected" and
  blocks the merge; any required check must report on every pull request.
- A later commit status overrides an earlier commit status with the same context, but **not** a
  failing GitHub Actions check run with the same name. An override for an Actions-produced gate needs
  another mechanism (re-running the job, or a gate that honors a maintainer signal).

## Failures and Follow-up

None yet.
