---
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - docs/issues/open/2507-stop-requiring-pull-request-rebases/ISSUE.md
---

<!-- skill-link: create-issue -->

# Baseline Evidence: Routine Pull-Request Rebasing

Collected on 2026-10-13 between 15:00 and 16:15 UTC, before any policy change. All numbers come
from the GitHub REST and GraphQL APIs for `torrust/torrust-tracker` and from the local clone at
`develop` tip `48e11787f` (`Merge torrust/torrust-tracker#2504`).

## 1. Where the Rule Is Documented

No ADR, issue specification, or commit body records **why** pull requests must be rebased. The
rule exists only as workflow instructions and one tool guard:

| Source                                                 | Rule                                                                                      | Introduced                                                           |
| ------------------------------------------------------ | ----------------------------------------------------------------------------------------- | -------------------------------------------------------------------- |
| `.github/skills/dev/git-workflow/open-pull-request`    | Pre-flight check and "Keeping the branch up to date": always rebase before every push.    | `d2e7a1af2` (2026-05-20); the commit body states the rule, no reason |
| `.github/skills/dev/git-workflow/merge-pull-request`   | Rebase the PR immediately before the interactive merge.                                   | `6456efb97` (2026-09-15)                                             |
| `contrib/dev-tools/git/github-merge.py`                | Exit with `EXIT_STALE_MERGE_BASE` and "Rebase the pull request branch" when stale.        | `6456efb97`, refined in `2337469ee`, merged by PR #2229              |
| `AGENTS.md` (Commit history and PR scope)              | "only rebase onto `develop` to resolve conflicts" — contradicts the "always rebase" rule. | Pre-existing                                                         |
| Root ADR `20260926142648` (self-hosted Hetzner runner) | Records the cost: each merge makes the remaining PRs rebase and rerun the container job.  | 2026-09-26                                                           |

PR #2229 was a reaction to two `GH013` push rejections (PR #2181 on 2026-09-09 and PR #2113 on
2026-09-15), not a design decision about history shape.

Reproduce:

```sh
git log --format='%h %ad %s' --date=short -S'Always rebase your branch' -- .github
git log --format='%h %ad %s' --date=short -S'EXIT_STALE_MERGE_BASE' -- contrib/dev-tools/git/github-merge.py
gh pr view 2229 --repo torrust/torrust-tracker --json body --jq .body
```

## 2. GitHub Configuration

Repository merge settings: merge commits and rebase merges allowed, squash disabled,
`allow_update_branch: true`, auto-merge disabled. `develop` is the default branch and has no
classic branch protection; it is governed by rulesets.

| Ruleset  | Name (abridged)                                | Enforcement | Rules relevant to rebasing                                                                                           |
| -------- | ---------------------------------------------- | ----------- | -------------------------------------------------------------------------------------------------------------------- |
| 87085    | Deny Create; Deny Delete; Require Pull Request | active      | `pull_request`: 0 approvals, `dismiss_stale_reviews_on_push: true`, merge methods `merge`, `squash`, `rebase`        |
| 94133    | Required Signed                                | active      | `required_signatures`                                                                                                |
| 94237    | Deny Force Push; Require Status Checks         | disabled    | `non_fast_forward`, 16 required checks, `strict_required_status_checks_policy: true` ("require branches up to date") |
| 87098    | Require Deployment: coverage                   | active      | No rules configured                                                                                                  |
| 19245352 | Code Quality Copilot review for default branch | disabled    | `copilot_code_review` with `review_on_push: true` and `review_draft_pull_requests: true`                             |

Findings:

- GitHub **did** enforce "branches must be up to date before merging" through ruleset 94237 from
  2023-09-18. It has been effectively disabled since 2026-05-05 (section 2.1).
- No active rule requires an up-to-date branch today. The requirement is enforced only by our
  skills and the `github-merge.py` stale-base guard.
- `dismiss_stale_reviews_on_push: true` means every rebase force-push dismisses existing approvals.

Reproduce:

```sh
gh api repos/torrust/torrust-tracker --jq '{allow_merge_commit,allow_squash_merge,allow_rebase_merge,allow_update_branch}'
gh api repos/torrust/torrust-tracker/rules/branches/develop
gh api repos/torrust/torrust-tracker/rulesets/94237 --jq '{enforcement,rules}'
```

### 2.1 Why Ruleset 94237 Is Disabled

GitHub keeps a version history per ruleset. Every enforcement change since May 2026 was made by
`josecelano`, and every one brackets a maintainer merge (times are UTC+01:00; the merge time is the
local commit time of the signed merge, which precedes its push):

| Version change      | Enforcement | Merge in the window                                         |
| ------------------- | ----------- | ----------------------------------------------------------- |
| 2026-05-01 15:23    | disabled    | #1724 committed 15:32                                       |
| 2026-05-01 15:33    | active      |                                                             |
| 2026-05-01 17:13:29 | disabled    | #1725 committed 17:13:45                                    |
| 2026-05-01 17:14    | active      |                                                             |
| 2026-05-04 11:42    | active      | Added 9 new check names; kept the 7 obsolete ones           |
| 2026-05-04 13:22:28 | disabled    | #1728 committed 13:21:53, pushed after the disable          |
| 2026-05-04 15:43    | active      | #1727 committed 15:43:08, just before the re-enable         |
| 2026-05-05 07:14:26 | disabled    | #1730 committed 07:13:50, pushed after the disable          |
| 2026-05-05 11:44    | active      | #1729 committed 11:44:26, just before the re-enable         |
| 2026-05-05 21:02    | disabled    | Never re-enabled for longer than the next row               |
| 2026-07-20 10:19    | active      |                                                             |
| 2026-07-20 10:35:05 | disabled    | #2000 (documentation-only README change) committed 10:35:53 |

No merge was ever pushed while the ruleset was active. The rule could not be satisfied:

1. **Obsolete check names.** `33e58200d` (2026-05-01 12:03, "collapse matrix jobs into
   test-nightly and test-stable") removed the jobs behind `Formatting`, `Units (nightly)`,
   `Units (stable)`, `E2E (nightly)`, and `E2E (stable)`, three hours before the first disable.
   `Test (Docker) (debug)` is no longer reported either. Six of the 16 required contexts never
   report today (checked against the head of code PR #2459).
2. **Documentation-only fast path.** Since `9a7d83b1a` (2026-05-08, issue #1743) and `2b177a2fd`
   (2026-05-12), the Rust workflows use `paths-ignore` for Markdown-only changes. A skipped workflow
   reports nothing, so a documentation-only PR (for example #2504) reports only `Docs Lint` and all
   16 required contexts stay pending forever.
3. **Unverified: required checks on a locally pushed merge.** The maintainer merge pushes a new
   merge commit that never ran checks itself. Whether GitHub accepts it based on the PR head's check
   results was never observable, because causes 1 and 2 always blocked first. Combined with
   `strict_required_status_checks_policy: true`, the PR head must also contain the `develop` tip.

Disabling the ruleset also disabled its `non_fast_forward` (deny force-push) rule for `develop`,
`main`, and `releases/*`.

Reproduce:

```sh
gh api 'repos/torrust/torrust-tracker/rulesets/94237/history?per_page=100'
gh api repos/torrust/torrust-tracker/rulesets/94237/history/<version_id> \
  --jq '{updated_at, enforcement: .state.enforcement}'
gh api 'repos/torrust/torrust-tracker/commits/<pr-head-sha>/check-runs?per_page=100' \
  --jq '[.check_runs[].name] | unique'
```

### 2.2 Why Ruleset 19245352 Is Disabled

Ruleset 19245352 requests a Copilot code review on the default branch. Its history has two
versions, both by GitHub's `ghost` actor (user id 10137), which suggests a GitHub-managed change,
probably from the Code Quality feature named in the ruleset, rather than a maintainer edit:

| Version          | Enforcement |
| ---------------- | ----------- |
| 2026-07-20 19:00 | active      |
| 2026-08-05 12:06 | disabled    |

Disabling it had no visible effect on maintainer PRs: 115 of 135 PRs created between 2026-08-05 and
2026-09-15 received a Copilot review. On current PRs Copilot is requested one second after
creation **by the PR author** (`da2ce7` on #2504, `josecelano` on #2459), which matches the
personal "automatically request Copilot review" setting. The only external PR opened after the
change (#2054, by an external contributor) received no Copilot review. Copilot reviewed each sampled PR once,
regardless of how many times it was force-pushed.

Organization-level rulesets could not be read with the available token (`admin:org` scope
missing); a maintainer must confirm none exist.

Reproduce:

```sh
gh api 'repos/torrust/torrust-tracker/rulesets/19245352/history?per_page=100'
gh api user/10137 --jq .login
```

## 3. Why the Merge Tool Needs a Fresh Merge Ref

`github-merge.py` builds and signs its own merge commit locally and pushes it to `develop`. The
`pull_request` rule accepted that push only when GitHub's `refs/pull/<n>/merge` was computed
against the current `develop` tip; both `GH013` incidents happened when it was not.

GitHub does not reliably recompute that ref after `develop` advances through these locally pushed
merges. On 2026-10-13:

| PR    | `refs/pull/<n>/merge` parents | `develop` tip at check | Head behind `develop` | Recomputed after mergeability query and 48 min wait |
| ----- | ----------------------------- | ---------------------- | --------------------- | --------------------------------------------------- |
| #2500 | `7711ade56` + `bbc488856`     | `48e11787f`            | 131 commits           | No                                                  |
| #2430 | `7711ade56` + `6535ce21e`     | `48e11787f`            | 678 commits           | No                                                  |

So today a push to the PR head (in practice a rebase) is what makes a behind PR mergeable with the
tool. Any policy change must provide another way to refresh the merge ref, or accept one push at
merge time.

Reproduce:

```sh
git fetch torrust develop "+refs/pull/2500/merge:refs/remotes/tmp/pr-2500-merge"
git log -1 --format='%p' refs/remotes/tmp/pr-2500-merge
gh api repos/torrust/torrust-tracker/pulls/2500 --jq .merge_commit_sha
git update-ref -d refs/remotes/tmp/pr-2500-merge
```

## 4. Pull-Request Volume

Pull requests created and merged per month (all authors):

| Month   | Opened | Merged | Median open PR queue (daily) | Max queue | Median hours to merge |
| ------- | -----: | -----: | ---------------------------: | --------: | --------------------: |
| 2026-03 |     15 |      2 |                            — |         — |                     — |
| 2026-04 |     37 |     20 |                            0 |         1 |                   2.5 |
| 2026-05 |     85 |     76 |                            0 |         2 |                   1.2 |
| 2026-06 |     75 |     67 |                            0 |         3 |                   1.9 |
| 2026-07 |     64 |     41 |                            1 |         3 |                   1.4 |
| 2026-08 |     60 |     42 |                            2 |         4 |                   1.6 |
| 2026-09 |    200 |    170 |                            3 |         6 |                   2.1 |
| 2026-10 |     73 |     67 |                            8 |        12 |                   4.0 |

October covers 1 to 13 October. Merged per year: 2022: 61, 2023: 145, 2024: 199, 2025: 167,
2026 (to 13 October): 487. Of the PRs merged since April 2026, 468 were opened from the
`josecelano` account, most of them prepared by AI agents. The queue counts PRs that were later
merged or are still open, sampled daily at 12:00 UTC.

Reproduce:

```sh
gh pr list --repo torrust/torrust-tracker --state all --limit 3000 \
  --json number,createdAt,mergedAt,state,author
```

## 5. Force-Pushes per Merged PR

`HEAD_REF_FORCE_PUSHED_EVENT` timeline items per merged PR. Most are rebases because the repository
forbids rewriting history for any other reason.

| Merged in | PRs | Force-pushes | Mean per PR | Max | PRs with at least one |
| --------- | --: | -----------: | ----------: | --: | --------------------: |
| 2026-06   |  67 |           26 |        0.39 |   2 |              23 (34%) |
| 2026-07   |  41 |           33 |        0.80 |   8 |              20 (49%) |
| 2026-08   |  42 |           20 |        0.48 |   3 |              17 (40%) |
| 2026-09   | 170 |          183 |        1.08 |  10 |             115 (68%) |
| 2026-10   |  67 |          106 |        1.58 |   6 |              54 (81%) |

Caveat: GraphQL `timelineItems(itemTypes: ...).totalCount` ignores the type filter and returns the
count of all timeline items. Count the returned nodes instead.

Reproduce:

```sh
gh api graphql -f query='{repository(owner:"torrust",name:"torrust-tracker"){pullRequest(number:2501){
  timelineItems(first:100,itemTypes:[HEAD_REF_FORCE_PUSHED_EVENT]){nodes{__typename}}}}}' \
  --jq '.data.repository.pullRequest.timelineItems.nodes | length'
```

## 6. CI Cost of Pull-Request Runs (1 to 13 October 2026)

899 `pull_request` workflow runs. Runner time per run is the mean summed job duration of six
randomly sampled runs per workflow, so the totals are estimates.

| Workflow                       | Runs | Mean job-minutes per run | Estimated runner-hours |
| ------------------------------ | ---: | -----------------------: | ---------------------: |
| OS Compatibility               |  113 |                     28.2 |                   53.2 |
| Generate Coverage Reports (PR) |  113 |                     27.6 |                   52.0 |
| Testing                        |  113 |                     22.5 |                   42.4 |
| Container                      |  113 |                     13.4 |                   25.2 |
| Database Compatibility         |   36 |                     17.5 |                   10.5 |
| Docs Lint                      |  371 |                      1.4 |                    8.9 |
| Database Benchmarking          |   36 |                      5.9 |                    3.6 |
| Security Scan                  |    4 |                     30.5 |                    2.0 |
| **Total**                      |  899 |                          |              **197.8** |

Rust workflows ignore Markdown-only changes (`paths-ignore: "**/*.md"`), so a rebase of a
documentation-only PR costs about 1.4 job-minutes, while a rebase of a code PR reruns about 92
job-minutes across the four main workflows. Docs Lint runs once per pushed head (371 runs), and
106 of those pushes were force-pushes; if force-pushes are spread like other pushes, rebases account
for roughly 29% of PR CI time, about 57 runner-hours in 13 days. This attribution is an estimate,
not a per-PR measurement.

## 7. Review Noise Caused by Rebases

Commits on `develop` since September 2026 whose only purpose was to repair rebase fallout,
selected by reading the subjects:

- `505fa5745` correct the rebased commit times in the PR #2501 audit log
- `ef27e6585` keep completion evidence rebase-safe
- `1736f0d91` ... log the second rebase on #2468
- `03bc9d794` rerun manual verification on the rebased head
- `4596fb323` regenerate the coupling report after rebasing onto develop
- `6f7053686` name reviewed commits by subject in the SI-17 review report
- `648d184b6` cite the PR head instead of a rebased-away id in the #2451 record
- `9bc523862` cite the base commit by subject in the evidence
- `0be2875aa` restore the EPIC #1669 log lines lost in the #2445 rebase
- `3b6c63027` cite #2435 branch commits by subject instead of id
- `df4b11a29` cite commit subject instead of branch commit id
- `91b0f5b22` name review heads by subject ...
- `943b02765` cite the spec commit by subject instead of an orphaned id
- `bed746cc7` cite branch commits by subject
- `4fc38fda0` record the rebase and the upstream F3 fix
- `b5f217a04` cite #2261 evidence commits by subject
- `26255101f` keep review history rebase-safe
- `428b5f428` keep review evidence rebase-safe
- `2f6a19232` cite SI-13 commits by subject instead of branch ids
- `5ef275390` cite scrape evidence patches by subject, not branch id
- `30b9cf821` repair rebase workflow regressions
- `929e0c99f` correct rebased lifecycle inventory
- `0779425f7` refresh plan after develop rebase

The PR #2320 review retrospective (`docs/pr-reviews/pr-2320-review/review-retrospective.md`)
records an approval dismissed 1 h 24 min after it was given, by a byte-identical rebase, and three
findings caused by evidence that cited ids the rebases orphaned.

Reproduce:

```sh
git log --no-merges --since=2026-06-01 --format='%h %ad %s' --date=short -i -E \
  --grep='rebase|rebased|rebasing|instead of (the )?(branch )?(commit )?id|by subject|rebase-safe|orphan'
```

The command also returns a few unrelated commits (for example an "orphan" regression test); the
list above is the manually filtered result.
