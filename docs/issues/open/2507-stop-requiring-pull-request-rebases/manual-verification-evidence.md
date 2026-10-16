---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2507-stop-requiring-pull-request-rebases/ISSUE.md
last-updated-utc: 2026-10-16 08:17
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
- Status: `DONE`; the maintainer chose option (e), run by the merge tool itself (2026-10-14)

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
- Status: `DONE`. On 2026-10-14 the maintainer chose option (e), run automatically by the merge
  tool, which T2 of the specification implements.

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

### T5 - Ruleset Changes Applied

- Goal: apply the maintainer-approved ruleset target state (specification "Ruleset Target State").
- Initial state: rulesets as listed below; `develop` and `main` had no `non_fast_forward` rule.
- Status: `DONE` (maintainer approval 2026-10-14; applied with `gh` authenticated as `josecelano`)

Steps performed (2026-10-14, 09:59:29 to 09:59:44 UTC):

1. Created ruleset 25068452 "(branches) [ * ; ! dependabot, ! staging ] Deny Force Push", active,
   with only `non_fast_forward` and the branch conditions of 94237, before deleting anything.
2. Saved the full definitions of the rulesets to delete, and of the new one, under
   [`evidence/`](evidence/), so each can be recreated exactly. The files keep the fields needed to
   recreate a ruleset and drop the response metadata (`source`, `source_type`, `node_id`, `_links`);
   Prettier formatted them afterwards.
3. Deleted 94237 (disabled; force-push ban and 16 strict required checks), 19245352 (disabled;
   Copilot code review), and 87098 (active; no rules).

```sh
gh api -X POST repos/torrust/torrust-tracker/rulesets --input <new-ruleset.json>
gh api repos/torrust/torrust-tracker/rulesets/<id> \
  --jq '{id,name,target,enforcement,conditions,rules,bypass_actors,created_at,updated_at}' \
  > evidence/deleted-ruleset-<id>.json
gh api repos/torrust/torrust-tracker/rulesets/25068452 \
  --jq '{id,name,target,enforcement,conditions,rules,bypass_actors,created_at}' \
  > evidence/created-ruleset-25068452.json
npx prettier@3 --write evidence/*.json
gh api -X DELETE repos/torrust/torrust-tracker/rulesets/<id>
gh api repos/torrust/torrust-tracker/rules/branches/develop --jq '[.[].type] | join(",")'
```

Observed result:

| Ruleset  | Before   | After            |
| -------- | -------- | ---------------- |
| 87085    | active   | active           |
| 94237    | disabled | deleted          |
| 94133    | active   | active           |
| 87098    | active   | deleted          |
| 86945    | active   | active           |
| 94096    | active   | active           |
| 90587    | active   | active           |
| 87069    | active   | active           |
| 94135    | active   | active           |
| 94130    | active   | active           |
| 19245352 | disabled | deleted          |
| 25068452 | —        | active (created) |

```text
before, develop: creation,pull_request,deletion,required_signatures
before, main:    creation,pull_request,deletion,required_signatures
after,  develop: creation,pull_request,deletion,non_fast_forward,required_signatures
after,  main:    creation,pull_request,deletion,non_fast_forward,required_signatures
```

Release branches (`releases/**`), checked for one existing release branch on 2026-10-14 at about
10:00 UTC and again at 10:36 UTC with the same output:

```sh
gh api repos/torrust/torrust-tracker/rules/branches/releases/v3.0.0-rc.1 --jq '[.[].type] | join(",")'
```

```text
non_fast_forward,required_signatures,creation,deletion,required_deployments,update
```

Ruleset 87098 kept only one retained version, from 2026-07-20 10:19 (+01:00) by `josecelano`, with
no rules; the `coverage` environment it once referenced still exists. No disabled ruleset remains.

Conclusion: the force-push ban is enforced again on `develop`, `main`, and release branches
(scenario M7), and no
required status check is configured until the follow-up `CI gate` issue.

### Merges Under the New Policy (Scenarios M1 to M6 and M8)

- Goal: verify scenarios M1 to M6 and M8 from the real maintainer merges made after PR #2526
  landed, as the specification's scenario notes intend ("executed during real maintainer merges").
- Date and time (UTC): data collected 2026-10-15 16:20 to 16:50; `develop` reached `fd04ac51e`
  (`Merge torrust/torrust-tracker#2543`) at 16:33 and was re-read at that commit.
- Scope: the 18 pull-request merges from PR #2526 (GitHub `merged_at` 2026-10-14 12:18:38 UTC) to
  #2543, both included. #2500 was still open during collection and merged at 17:13:46 UTC, after
  `fd04ac51e`; it is not included. Its head was pushed at 16:49:48 UTC, after `develop` reached its
  merge base, so it would not have exercised the refresh anyway.
- Tools: `git` and GitHub CLI `gh` authenticated as `josecelano`.
- Status: `DONE` (M1 by indirect evidence; see V1)

For each first-parent merge commit `M`, `M^1` is the `develop` tip the merge was built on and
`M^2` is the merged pull-request head. A head is "behind" when `git merge-base M^1 M^2` is not
`M^1`. Commands, run in a Bash loop over the merges:

```sh
git log --first-parent --merges --since='2026-10-14T12:00:00Z' --format='%H %cI %s' torrust/develop
git merge-base "$M^1" "$M^2"
git merge-base --is-ancestor "$M^2" torrust/develop; echo $?
gh api repos/torrust/torrust-tracker/issues/<n>/timeline --paginate \
  --jq '[.[]|select(.event=="head_ref_force_pushed")|.created_at]|join(",")'
git diff --name-only "$M^1...$M^2"   # code files = paths not ending in .md
```

Observed result (merge time in UTC+01:00, as Git prints it; "ancestor" is the exit code of
`git merge-base --is-ancestor`):

| PR    | Merged           | `M^1` (`develop`) | `M^2` (head) | Behind at merge   | Ancestor | Head force-pushes (UTC)   | Code files | Files |
| ----- | ---------------- | ----------------- | ------------ | ----------------- | -------- | ------------------------- | ---------- | ----- |
| #2526 | 2026-10-14 13:17 | `1c6707a43`       | `8ab3174a5`  | no                | 0        | 10-14 10:52, 11:44        | 6          | 16    |
| #2524 | 2026-10-14 15:19 | `c1874facb`       | `f51bee826`  | no                | 0        | 10-14 10:28, 10:55, 13:14 | 19         | 26    |
| #2527 | 2026-10-14 16:03 | `8045bd252`       | `081f6e6b0`  | no                | 0        | none                      | 9          | 12    |
| #2528 | 2026-10-14 16:18 | `184446a2b`       | `620b2a606`  | yes (`8045bd252`) | 0        | none                      | 5          | 14    |
| #2531 | 2026-10-14 17:06 | `988c3bc76`       | `b65cd96cb`  | no                | 0        | none                      | 0          | 2     |
| #2529 | 2026-10-14 17:19 | `3b0b40047`       | `9af096dca`  | yes (`8045bd252`) | 0        | none                      | 19         | 23    |
| #2530 | 2026-10-14 17:28 | `90e8daf75`       | `ab6ea7da6`  | yes (`988c3bc76`) | 0        | 10-14 15:31               | 4          | 10    |
| #2533 | 2026-10-14 17:36 | `b24550171`       | `0d377b033`  | no                | 0        | none                      | 0          | 1     |
| #2534 | 2026-10-14 17:42 | `e2e21f20c`       | `0782a8210`  | yes (`3b0b40047`) | 0        | none                      | 0          | 6     |
| #2532 | 2026-10-14 22:08 | `48c4d3013`       | `2488cae4e`  | yes (`e2e21f20c`) | 0        | 10-14 16:36, 16:39        | 10         | 16    |
| #2537 | 2026-10-15 07:41 | `07750152f`       | `b084b2ea5`  | no                | 0        | 10-14 21:27               | 1          | 5     |
| #2535 | 2026-10-15 07:48 | `e6b46013b`       | `ed56f2efe`  | yes (`48c4d3013`) | 0        | none                      | 0          | 2     |
| #2539 | 2026-10-15 08:18 | `ee4eb8e9c`       | `51ca18aff`  | yes (`e6b46013b`) | 0        | none                      | 1          | 3     |
| #2540 | 2026-10-15 08:36 | `759893bd1`       | `e346138ce`  | no                | 0        | none                      | 0          | 2     |
| #2538 | 2026-10-15 16:06 | `9c8bcdf8a`       | `fffdb8c03`  | no                | 0        | 10-15 12:04               | 20         | 50    |
| #2430 | 2026-10-15 16:42 | `3195b8b60`       | `ff8015ad1`  | yes (`9c8bcdf8a`) | 0        | 10-15 11:31               | 0          | 3     |
| #2541 | 2026-10-15 16:47 | `09887e70d`       | `079b6e741`  | yes (`3195b8b60`) | 0        | none                      | 3          | 34    |
| #2543 | 2026-10-15 17:32 | `a9d8c5703`       | `34a8db13d`  | no                | 0        | none                      | 0          | 1     |

The "Behind at merge" column names the merge base when it is not `M^1`.

#### V1 - Stale Recorded Base Is Refreshed (M1)

The terminal output of these merges was not captured, so the printed line
`Refreshed the recorded base of torrust/torrust-tracker#<n>` is not available. The evidence is
indirect. For each behind pull request, its recorded base after merge was compared with `M^1`, and
the time its head was first pushed (the creation of the earliest `pull_request` workflow run for
`M^2`) with the time `develop` reached `M^1`:

```sh
gh api repos/torrust/torrust-tracker/pulls/<n> --jq .base.sha
gh api "repos/torrust/torrust-tracker/actions/runs?head_sha=$(git rev-parse "$M^2")&event=pull_request&per_page=100" \
  --jq '[.workflow_runs[].created_at]|min'
git log -1 --format=%cI "$M^1"
gh api repos/torrust/torrust-tracker/issues/<n>/timeline --paginate \
  --jq '[.[]|select(.event=="reopened" or .event=="closed" or .event=="base_ref_changed")|"\(.event)@\(.created_at)"]|join(",")'
```

| PR    | `base.sha` = `M^1` | Head first pushed (UTC) | `develop` reached `M^1` (UTC) | Head push before `M^1`? |
| ----- | ------------------ | ----------------------- | ----------------------------- | ----------------------- |
| #2528 | yes                | 2026-10-14 14:58:41     | 2026-10-14 15:03:58           | yes                     |
| #2529 | yes                | 2026-10-14 16:00:25     | 2026-10-14 16:06:29           | yes                     |
| #2530 | yes                | 2026-10-14 16:06:44     | 2026-10-14 16:19:52           | yes                     |
| #2534 | yes                | 2026-10-14 16:34:46     | 2026-10-14 16:36:43           | yes                     |
| #2532 | yes                | 2026-10-14 19:43:48     | 2026-10-14 16:42:40           | no                      |
| #2535 | yes                | 2026-10-14 16:55:35     | 2026-10-15 06:41:23           | yes                     |
| #2539 | yes                | 2026-10-15 07:01:02     | 2026-10-15 06:48:41           | no                      |
| #2430 | yes                | 2026-10-15 15:22:41     | 2026-10-15 15:06:29           | no                      |
| #2541 | yes                | 2026-10-15 15:22:40     | 2026-10-15 15:42:11           | yes                     |

The timeline query returned only the `closed` event of each merge: no `reopened` or
`base_ref_changed` event. For #2528, #2529, #2530, #2534, #2535, and #2541 the head was pushed before
`develop` reached `M^1`, yet `base.sha` equals `M^1` and GitHub accepted the merge push.

What the T1 sandbox established:

- GitHub accepts the direct merge push only when `base.sha` is the target tip (steps 2 and 7
  rejected).
- `base.sha` was refreshed by close and reopen, Update branch, a base toggle, and a no-op base
  edit.
- `base.sha` was not refreshed by a title-only edit (step 10), or by doing nothing for about 14
  minutes while the base moved (step 7).
- Under option (a), GitHub once recomputed a merge ref with no visible trigger, about 9 minutes
  after a base push. `base.sha` was not recorded there.

Conclusion: the six merges were not rewritten (no head push after the first) and have no close,
reopen, or Update-branch trace. The merge tool's no-op base edit is the explanation consistent with
all of this evidence. A spontaneous `base.sha` refresh cannot be excluded completely, but it would
have to happen in all six cases between `develop` reaching `M^1` and the merge, which T1 never
observed. M1 is therefore `DONE` by inference. The expected tool output was not observed (see
"Failures and Follow-up").

#### V2 - Behind Pull Request Merged Without Rewriting Commits (M2)

Pull requests #2528, #2529, #2534, #2535, #2539, and #2541 were behind `develop` at merge and had
no force-push at all. #2530, #2532, and #2430 were force-pushed earlier but were behind again at merge. GitHub accepted
every merge push, and each pull request shows as merged.

#### V3 - Reviewed Commit Ids Survive the Merge (M3)

`git merge-base --is-ancestor "$M^2" torrust/develop` exited `0` for all 18 merged heads (column
"Ancestor" above), including the nine behind ones.

#### V4 - Conflicting Pull Request Follows the Rebase Path (M4)

The `open-pull-request` skill requires a rebase or other update when "the branch conflicts with
`develop`" and shows `git rebase` followed by a push "use `--force-with-lease` when rewriting
history". Two real pull requests took that path. Each force-push's before and after commits came from
the GraphQL `HeadRefForcePushedEvent` (`beforeCommit` and `afterCommit`), and the `develop` tip at
that time from `git rev-list -1 --first-parent --before=<time> torrust/develop`. A `beforeCommit` is
a head that the force-push removed from the pull-request branch, so it is on no `develop` commit and
no tag, and a clone does not hold it until it is fetched. Fetch each full `oid` the query returns
before comparing it:

```sh
gh api graphql -f query='{repository(owner:"torrust",name:"torrust-tracker"){pullRequest(number:<n>){
  timelineItems(itemTypes:[HEAD_REF_FORCE_PUSHED_EVENT],first:10){nodes{... on HeadRefForcePushedEvent{
  createdAt beforeCommit{oid} afterCommit{oid}}}}}}}'
git fetch torrust <oid from beforeCommit or afterCommit>
git merge-tree --write-tree <develop tip> <commit> >/dev/null && echo clean || echo CONFLICT
git merge-base <develop tip> <after commit>
```

The full `oid` values the query returned (the `develop` tips are `b245501717fe251e99b1364f80cb209fffed0c2e`
and `07750152f1f37a2115570289f651481b8535519a`):

```text
#2532 beforeCommit 55a414e98a1df60999ae5aa9689b6662ee168c5a
#2532 afterCommit  2328de9cccf737fec570bb0bb7ab199742288a6c
#2537 beforeCommit 94fcf5a7fc466d42f0dcc060d85341fb0f79661b
#2537 afterCommit  b084b2ea57571ce1facdcf2f3317f82d08cddc18
```

The values are fixed event data, not commits of any branch of this repository's history, so the
`open-pull-request` commit-id check reports them in a clone that fetched them. The skill defines no
exception for this case.

| PR    | Force-push (UTC)    | `beforeCommit` | `develop` tip | Before vs `develop` | `afterCommit` | After based on tip | After vs `develop` |
| ----- | ------------------- | -------------- | ------------- | ------------------- | ------------- | ------------------ | ------------------ |
| #2532 | 2026-10-14 16:36:21 | `55a414e98`    | `b24550171`   | conflict            | `2328de9cc`   | yes                | clean              |
| #2537 | 2026-10-14 21:27:23 | `94fcf5a7f`    | `07750152f`   | conflict            | `b084b2ea5`   | yes                | clean              |

Both rebased heads were mergeable and both were merged (table above). The other force-pushes
(#2530, #2532 at 16:39:53, #2538, and #2430) rebased heads that merged cleanly. Those were
recommended rebases, not required ones. Whether `--force-with-lease` was used cannot be read from
GitHub.

#### V5 - Code Pull Request Merges Under the New Rulesets (M5)

Code pull request #2529 (19 Rust and other non-Markdown files) was merged with the maintainer tool
after T5. #2541 also ran the full heavy CI and merged the same way. Its 3 non-Markdown files are
archived files under `docs/issues/closed/` (`Cargo.toml`, `main.rs`, and `.gitignore` of an archived
crate), so it counts as a heavy-CI pull request rather than a code change. No ruleset was disabled or changed at any point after T5: the only ruleset version created since
2026-10-14 09:00 UTC is the creation of 25068452, and every ruleset is active.

```sh
for id in $(gh api repos/torrust/torrust-tracker/rulesets --jq '.[].id'); do
  gh api repos/torrust/torrust-tracker/rulesets/$id/history \
    --jq ".[]|select(.updated_at > \"2026-10-14T09:00:00Z\")|\"$id \(.version_id) \(.updated_at)\""
  gh api repos/torrust/torrust-tracker/rulesets/$id --jq '"\(.id) \(.enforcement) updated:\(.updated_at)"'
done
```

```text
87085 active updated:2024-11-29T16:01:32.717+00:00
25068452 53138124 2026-10-14T10:59:31.292+01:00
25068452 active updated:2026-10-14T10:59:31.166+01:00
94133 active updated:2024-10-03T11:55:05.557+01:00
86945 active updated:2024-10-03T11:55:49.590+01:00
94096 active updated:2026-07-20T10:12:19.610+01:00
90587 active updated:2024-10-03T11:55:58.199+01:00
87069 active updated:2024-10-03T11:55:29.524+01:00
94135 active updated:2024-10-03T11:56:06.114+01:00
94130 active updated:2024-10-03T11:55:45.553+01:00
```

Workflows that ran on the merged code heads:

```sh
gh api "repos/torrust/torrust-tracker/actions/runs?head_sha=<M^2>&per_page=100" \
  --jq '[.workflow_runs[]|"\(.name)=\(.conclusion)"]|unique|join(", ")'
```

```text
#2529: Container=success, Docs Lint=success, Generate Coverage Reports (PR)=success, OS Compatibility=success, Testing=success
#2541: Container=success, Docs Lint=success, Generate Coverage Reports (PR)=success, OS Compatibility=success, Testing=success
```

#### V6 - Documentation-Only Pull Request Merges Under the New Rulesets (M6)

Pull requests #2533, #2540, and #2543 changed only Markdown files and were merged with the maintainer tool after
T5, under the same unchanged rulesets (V5). The same workflow-run query shows that only the docs
workflow ran:

```text
#2533: Docs Lint=success
#2540: Docs Lint=success
#2543: Docs Lint=success
```

#### V8 - First-Parent History Unchanged (M8)

```sh
git log --first-parent --format='%h %ad %s' --date=format:'%m-%d %H:%M' -19 torrust/develop
```

```text
fd04ac51e 10-15 17:32 Merge torrust/torrust-tracker#2543: docs(skills): document periodic audit in cleanup-completed-issues
a9d8c5703 10-15 16:47 Merge torrust/torrust-tracker#2541: chore(issues): archive closed issue #1348 spec
09887e70d 10-15 16:42 Merge torrust/torrust-tracker#2430: docs(discussions): [#2429] add my round on templated documentation
3195b8b60 10-15 16:06 Merge torrust/torrust-tracker#2538: test(udp-core): [#1348] add tests to the udp-core package
9c8bcdf8a 10-15 08:36 Merge torrust/torrust-tracker#2540: docs(pr-reviews): close the PR #2537 follow-up findings after PR #2539 merged
759893bd1 10-15 08:18 Merge torrust/torrust-tracker#2539: chore(pr-reviews): land the PR #2537 review fixes that missed its merge
ee4eb8e9c 10-15 07:48 Merge torrust/torrust-tracker#2535: chore(issues): archive closed issue #2493 spec
e6b46013b 10-15 07:41 Merge torrust/torrust-tracker#2537: docs(issues): [#2536] add specification for hiding the statistics read lock
07750152f 10-14 22:08 Merge torrust/torrust-tracker#2532: chore(issues): archive closed issue #2521 spec
48c4d3013 10-14 17:42 Merge torrust/torrust-tracker#2534: docs(skills): apply the #2505 retrospective process improvements
e2e21f20c 10-14 17:36 Merge torrust/torrust-tracker#2533: docs(pr-reviews): close the PR #2527 post-merge findings after PR #2530 merged
b24550171 10-14 17:28 Merge torrust/torrust-tracker#2530: chore(issues): archive closed issue #2517 spec
90e8daf75 10-14 17:19 Merge torrust/torrust-tracker#2529: refactor: [#2521] remove the unused UDP statistics traits and services
3b0b40047 10-14 17:06 Merge torrust/torrust-tracker#2531: docs(agents): allow rewriting an unmerged pull request's history with safeguards
988c3bc76 10-14 16:18 Merge torrust/torrust-tracker#2528: chore(issues): archive the closed #2505 spec and close its late spec-review findings
184446a2b 10-14 16:03 Merge torrust/torrust-tracker#2527: refactor: [#2517] make the statistics repositories' metrics lock private
8045bd252 10-14 15:19 Merge torrust/torrust-tracker#2524: fix(udp-core): [#2505] count only IPs over the limit as banned and refresh the total on reset
c1874facb 10-14 13:17 Merge torrust/torrust-tracker#2526: fix(git): [#2507] refresh the recorded pull-request base and reconcile rulesets
1c6707a43 10-14 12:32 Merge torrust/torrust-tracker#2522: docs(issues): [#2521] add issue specification for #2521
```

Times are UTC+01:00. Each first-parent commit is one pull-request merge, newest first, in the same
order as the per-PR table above (#2526 to #2543); the last line, #2522, is the merge before #2526.

### Automatic Checks Re-Run

- Date and time (UTC): 2026-10-15 16:45, `develop` at `fd04ac51e`, Python 3.14.4.

```text
$ python3 contrib/dev-tools/git/tests/test-github-merge-symlinks.py
Ran 51 tests in 7.007s
OK
$ bash contrib/dev-tools/git/tests/test-merge-pull-request.sh
All merge workflow wrapper tests passed.
```

## Failures and Follow-up

- M1's literal tool output was not captured. V1 infers the refresh from GitHub state instead. The
  next maintainer merge of a behind pull request can paste the line here, but closing the issue does
  not depend on it.
