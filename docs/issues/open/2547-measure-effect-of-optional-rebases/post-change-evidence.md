---
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - docs/issues/open/2547-measure-effect-of-optional-rebases/ISSUE.md
    - docs/issues/closed/2507-stop-requiring-pull-request-rebases/baseline-evidence.md
---

<!-- skill-link: create-issue -->

# Post-Change Evidence: Optional Pull-Request Rebases

Collected on 2026-10-30 between 07:50 and 09:30 UTC from the GitHub REST and GraphQL APIs for
`torrust/torrust-tracker` (authenticated as `josecelano`) and from a local clone at
`torrust/develop` tip `49641da38` (`Merge torrust/torrust-tracker#2688`). Window: 2026-10-15 00:00
UTC to 2026-10-28 23:59 UTC (14 days). Baseline: 1 to 13 October 2026 (13 days), from
[`baseline-evidence.md`](../../closed/2507-stop-requiring-pull-request-rebases/baseline-evidence.md).
Raw output was kept in a scratch directory and is not committed; the tables below are the raw
per-item results. Retrieval commands are recorded verbatim in each section; the commands that turn
their output into the tables, and the per-run CI data, are in Appendix A.

## 1. Pull-Request Volume (baseline section 4)

```sh
gh pr list --repo torrust/torrust-tracker --state all --limit 3000 \
  --json number,createdAt,mergedAt,state,author
```

The queue is sampled daily at 12:00 UTC and counts PRs that were later merged or are still open, as in
the baseline. Medians of an even count are the mean of the two middle values.

Two baseline populations appear in this evidence and are kept apart:

- **Published snapshot**: the figures `baseline-evidence.md` recorded on 2026-10-13 between 15:00 and
  16:15 UTC, for 67 merged PRs.
- **Full-window recount**: the same 13 calendar days (2026-10-01 to 2026-10-13 UTC) recounted on
  2026-10-30, for 68 merged PRs. The extra PR is #2506, created 2026-10-13 16:13:59 UTC and merged
  17:50:54 UTC, after the snapshot was taken. #2506 has no force-push.

| Measure                      | Published snapshot (67 PRs) | Recount (68 PRs) | 15 to 28 Oct |
| ---------------------------- | --------------------------: | ---------------: | -----------: |
| PRs opened                   |                          73 |               76 |          130 |
| PRs merged                   |                          67 |               68 |          121 |
| Merged PRs per day           |                         5.2 |              5.2 |          8.6 |
| Median open-PR queue (daily) |                           8 |                8 |            5 |
| Max open-PR queue            |                          12 |               12 |            8 |
| Median hours to merge        |                         4.0 |              3.8 |          1.9 |

Running the same code on the 67 merges without #2506 reproduces the published queue (8), maximum
(12) and median hours to merge (4.0086). With #2506 the median hours to merge is 3.7828.

## 2. Force-Pushes per Merged PR (baseline section 5)

Population: the 121 PRs with `merged_at` in the window, each with its whole force-push timeline.
Nodes were counted, not `totalCount`. No timeline had a second page.

```sh
for n in $(cat win.txt); do gh api graphql -f query='query($n:Int!){repository(owner:"torrust",name:"torrust-tracker"){pullRequest(number:$n){number mergedAt mergeCommit{oid} timelineItems(first:100,itemTypes:[HEAD_REF_FORCE_PUSHED_EVENT]){pageInfo{hasNextPage} nodes{... on HeadRefForcePushedEvent{createdAt beforeCommit{oid} afterCommit{oid}}}}}}}' -F n=$n; done
```

| Period                                | PRs | Force-pushes | Mean per PR | PRs with at least one |
| ------------------------------------- | --: | -----------: | ----------: | --------------------: |
| Baseline, published snapshot          |  67 |          106 |        1.58 |             54 (81%)  |
| Baseline, recount (adds #2506, none)  |  68 |          106 |        1.56 |             54 (79.4%) |
| 15 to 28 Oct                          | 121 |           11 |        0.09 |             10 (8.3%)  |

The baseline rows are the published figures of `baseline-evidence.md` section 5 for the 2026-10
merges; #2506's timeline was read with the GraphQL query above and has no force-push event.

The 11 force-pushes are lifetime events of the 121 merged PRs, as the population rule requires. They
belong to 10 PRs (#2678 has two); the other 111 merged PRs have none. Ten of the 11 happened inside the
window; the push on #2537 is dated 2026-10-14 21:27 UTC.

### 2.1 Classification (method of #2507 V4)

The `develop` tip at push time is the last first-parent `develop` commit whose PR has a `merged_at`
before the push. The `beforeCommit` objects were fetched with `git fetch torrust <sha>`.

```sh
git log --first-parent --format='%H%x09%s' torrust/develop --since=2026-10-01   # tip lookup
git merge-base <tip> <beforeCommit>; git merge-base <tip> <afterCommit>
git merge-tree --write-tree <tip> <beforeCommit>   # exit != 0: conflict
```

| PR    | Push (UTC)           | `develop` tip | Base before | Base after  | Class                             |
| ----- | -------------------- | ------------- | ----------- | ----------- | --------------------------------- |
| #2430 | 2026-10-15T11:31:01Z | `9c8bcdf8a` | `eb96d2957` | `9c8bcdf8a` | rebase, recommended (clean) |
| #2537 | 2026-10-14T21:27:23Z | `07750152f` | `48c4d3013` | `07750152f` | rebase, required (conflict) |
| #2538 | 2026-10-15T12:04:26Z | `9c8bcdf8a` | `48c4d3013` | `9c8bcdf8a` | rebase, recommended (clean) |
| #2559 | 2026-10-19T12:21:03Z | `2dc157b9c` | `e9c01fbb6` | `2dc157b9c` | rebase, required (conflict) |
| #2578 | 2026-10-20T17:28:48Z | `e760e714c` | `7771f09a4` | `e760e714c` | rebase, required (conflict) |
| #2629 | 2026-10-23T16:48:24Z | `67e37efd3` | `64e6783b0` | `67e37efd3` | rebase, recommended (clean) |
| #2630 | 2026-10-23T17:46:08Z | `5e9e575d8` | `5e9e575d8` | `5e9e575d8` | not a rebase (base unchanged) |
| #2638 | 2026-10-24T20:24:05Z | `d09a45f0a` | `abce11ec8` | `d09a45f0a` | rebase, recommended (clean) |
| #2672 | 2026-10-27T12:39:25Z | `79f550718` | `72fbeacc1` | `79f550718` | rebase, required (conflict) |
| #2678 | 2026-10-28T10:00:55Z | `09263a46c` | `802612dce` | `09263a46c` | rebase, required (conflict) |
| #2678 | 2026-10-28T13:31:06Z | `2025431d3` | `09263a46c` | `2025431d3` | rebase, required (conflict) |

| Class                                 | Force-pushes | Per merged PR (121) |
| ------------------------------------- | -----------: | ------------------: |
| Rebase, required (conflicted)         |            6 |                0.05 |
| Rebase, recommended (merged cleanly)  |            4 |                0.03 |
| Not a rebase (base unchanged)         |            1 |                0.01 |
| **All force-pushes**                  |       **11** |            **0.09** |
| **Rebase-only**                       |       **10** |            **0.08** |

The baseline counted every force-push as a rebase. The one non-rebase push is #2630 at
2026-10-23 17:46 UTC.

## 3. Merges Behind `develop` and the Refresh Path (method of #2507 V1)

For each of the 121 first-parent merges `M`: `M^1` is the `develop` tip the merge was built on and
`M^2` the PR head. A head is behind when `git merge-base M^1 M^2` is not `M^1`. "Head pushed" is the
creation time of the earliest `pull_request` workflow run for `M^2`; "refresh path" means that time is
before the commit date of `M^1`.

```sh
git merge-base "$M^1" "$M^2"
gh api "repos/torrust/torrust-tracker/actions/runs?head_sha=$M2&event=pull_request&per_page=100" --jq '[.workflow_runs[].created_at]|min'
git log -1 --format=%cI "$M^1"
gh api repos/torrust/torrust-tracker/pulls/<n> --jq .base.sha
```

| Measure                                                      | Count | Share of 121 |
| ------------------------------------------------------------ | ----: | -----------: |
| Merged while behind `develop`                                |    93 |          77% |
| Behind, head pushed before `develop` reached `M^1` (refresh path) |    68 |          56% |
| ... of which recorded `base.sha` equals `M^1` after merge    |    66 |          55% |
| Behind, head pushed after `develop` reached `M^1`            |    25 |          21% |

Two refresh-path merges (listed below with `base=M^1` "no") do not show `base.sha` equal to `M^1`; the
count is inferred, like #2507's, because the merge tool's output is not captured. Per-PR rows
for the 93 behind merges:

| PR | `M^1` | `M^2` | Head pushed (UTC) | `develop` reached `M^1` (UTC) | Refresh path | `base.sha` = `M^1` |
| -- | ----- | ----- | ----------------- | ----------------------------- | ------------ | ------------------ |
| #2430 | `3195b8b60` | `ff8015ad1` | 2026-10-15T15:22:41Z | 2026-10-15T15:06:29Z | no | yes |
| #2500 | `fd04ac51e` | `cac0c04f6` | 2026-10-15T16:49:48Z | 2026-10-15T16:32:03Z | no | yes |
| #2535 | `e6b46013b` | `ed56f2efe` | 2026-10-14T16:55:35Z | 2026-10-15T06:41:23Z | yes | yes |
| #2539 | `ee4eb8e9c` | `51ca18aff` | 2026-10-15T07:01:02Z | 2026-10-15T06:48:41Z | no | yes |
| #2541 | `09887e70d` | `079b6e741` | 2026-10-15T15:22:40Z | 2026-10-15T15:42:11Z | yes | yes |
| #2544 | `fe5d8264a` | `af3e157d0` | 2026-10-15T20:40:01Z | 2026-10-15T17:12:11Z | no | yes |
| #2545 | `ae4288087` | `a8165585f` | 2026-10-16T11:18:28Z | 2026-10-16T11:35:22Z | yes | yes |
| #2546 | `702cff6cc` | `250d4b541` | 2026-10-16T10:58:30Z | 2026-10-16T07:41:31Z | no | yes |
| #2549 | `2dc157b9c` | `5f972f6ab` | 2026-10-16T06:29:36Z | 2026-10-19T11:08:50Z | yes | yes |
| #2550 | `04013e7f1` | `b2d33e199` | 2026-10-16T13:18:14Z | 2026-10-16T15:19:59Z | yes | yes |
| #2552 | `e2b431666` | `8037065bb` | 2026-10-16T12:23:16Z | 2026-10-16T15:35:03Z | yes | yes |
| #2553 | `7a02904ee` | `863d0905d` | 2026-10-19T12:16:28Z | 2026-10-19T15:09:36Z | yes | yes |
| #2554 | `e9c01fbb6` | `117f84067` | 2026-10-19T10:21:54Z | 2026-10-17T12:14:55Z | no | yes |
| #2555 | `55e08830f` | `2e5e8eeb4` | 2026-10-16T15:43:31Z | 2026-10-19T15:27:17Z | yes | yes |
| #2556 | `7ba4c696d` | `d020502aa` | 2026-10-19T16:19:34Z | 2026-10-20T11:49:39Z | yes | yes |
| #2558 | `94490c497` | `f2ad9298e` | 2026-10-19T16:33:53Z | 2026-10-20T12:11:05Z | yes | yes |
| #2559 | `f2edc52c3` | `fba43030b` | 2026-10-19T12:50:48Z | 2026-10-19T15:16:35Z | yes | yes |
| #2561 | `5956368b3` | `dc461b1de` | 2026-10-19T15:40:26Z | 2026-10-20T12:02:50Z | yes | yes |
| #2562 | `1eae3c33e` | `9c50e4649` | 2026-10-19T15:16:57Z | 2026-10-19T16:26:35Z | yes | yes |
| #2563 | `6d6dd92ee` | `80e3381e0` | 2026-10-19T15:35:19Z | 2026-10-19T15:41:08Z | yes | yes |
| #2564 | `f37c0eca2` | `900fbaa9a` | 2026-10-19T15:58:36Z | 2026-10-19T16:22:45Z | yes | yes |
| #2568 | `e3beee28a` | `24276fb59` | 2026-10-20T07:11:01Z | 2026-10-20T11:32:45Z | yes | yes |
| #2569 | `550bddbb8` | `f24a690ec` | 2026-10-19T20:26:07Z | 2026-10-20T11:19:20Z | yes | yes |
| #2570 | `e1bf6fef2` | `ad6e2aa7a` | 2026-10-20T11:27:52Z | 2026-10-20T11:43:56Z | yes | yes |
| #2571 | `87690b15e` | `2e6b5d8c1` | 2026-10-20T12:01:52Z | 2026-10-20T12:16:55Z | yes | yes |
| #2572 | `b413306d2` | `3edca64d0` | 2026-10-20T12:23:42Z | 2026-10-20T12:31:31Z | yes | yes |
| #2573 | `954c5611e` | `1a536293d` | 2026-10-20T12:25:05Z | 2026-10-20T12:58:10Z | yes | yes |
| #2574 | `5ee038f1c` | `a032526f4` | 2026-10-20T15:43:41Z | 2026-10-20T15:57:10Z | yes | yes |
| #2575 | `7771f09a4` | `61c34601e` | 2026-10-20T16:19:10Z | 2026-10-20T16:07:21Z | no | yes |
| #2577 | `fe70cf908` | `1e0d7bb0a` | 2026-10-20T16:51:00Z | 2026-10-20T16:44:19Z | no | yes |
| #2579 | `5b54bb0f4` | `5f278ca4b` | 2026-10-20T16:53:44Z | 2026-10-20T16:55:01Z | yes | yes |
| #2580 | `f8e3bb2d7` | `d1f4ece84` | 2026-10-20T18:00:39Z | 2026-10-20T19:24:52Z | yes | yes |
| #2584 | `d60b680bd` | `0f6ae3235` | 2026-10-20T19:49:37Z | 2026-10-20T19:53:21Z | yes | yes |
| #2585 | `0a7145528` | `ae6d53b10` | 2026-10-20T18:03:46Z | 2026-10-20T19:32:48Z | yes | yes |
| #2586 | `e42eb1b37` | `1b0176415` | 2026-10-20T17:55:46Z | 2026-10-20T19:41:39Z | yes | yes |
| #2587 | `fefefdd9e` | `5782782bf` | 2026-10-20T19:34:30Z | 2026-10-20T19:46:30Z | yes | yes |
| #2589 | `ff9f7745a` | `2bd2f667e` | 2026-10-21T07:10:06Z | 2026-10-21T07:04:50Z | no | yes |
| #2590 | `12da1ea7d` | `ab78903d0` | 2026-10-21T07:53:34Z | 2026-10-21T07:42:40Z | no | yes |
| #2591 | `afbd5dbf1` | `492d19a86` | 2026-10-21T08:23:43Z | 2026-10-21T09:08:07Z | yes | yes |
| #2593 | `321e83bc4` | `580b5922b` | 2026-10-21T08:00:46Z | 2026-10-21T09:16:25Z | yes | yes |
| #2595 | `6fd4bc597` | `8893a0a67` | 2026-10-21T08:15:47Z | 2026-10-21T08:58:45Z | yes | yes |
| #2596 | `b5ab35ae9` | `f8f53e8a5` | 2026-10-21T07:51:19Z | 2026-10-21T09:23:24Z | yes | yes |
| #2597 | `d21282aa4` | `0ebb43117` | 2026-10-21T09:12:51Z | 2026-10-21T09:30:24Z | yes | yes |
| #2598 | `b5894fb67` | `d7f32d451` | 2026-10-21T09:25:51Z | 2026-10-21T09:38:56Z | yes | yes |
| #2599 | `4ad183d18` | `ae26e2c77` | 2026-10-21T10:08:27Z | 2026-10-21T10:25:47Z | yes | yes |
| #2600 | `97c689fe1` | `8efbf8cbb` | 2026-10-21T09:55:15Z | 2026-10-21T09:46:46Z | no | yes |
| #2601 | `01bf6624d` | `3b91c65f2` | 2026-10-21T10:52:31Z | 2026-10-21T11:02:43Z | yes | yes |
| #2602 | `6da62f0ee` | `a252e0566` | 2026-10-21T10:35:36Z | 2026-10-21T10:47:59Z | yes | yes |
| #2604 | `c160fe1e2` | `a61c46a9e` | 2026-10-21T11:08:47Z | 2026-10-21T11:08:59Z | yes | yes |
| #2605 | `29e1f7ce3` | `118fbf509` | 2026-10-22T19:45:01Z | 2026-10-21T17:35:20Z | no | yes |
| #2607 | `889a6dc36` | `dca0cd6dd` | 2026-10-21T12:04:07Z | 2026-10-21T12:02:31Z | no | yes |
| #2608 | `9f20a89b8` | `af69b317c` | 2026-10-21T11:38:36Z | 2026-10-21T11:39:56Z | yes | yes |
| #2609 | `4fbc38da3` | `28370ac89` | 2026-10-21T11:32:52Z | 2026-10-21T11:28:17Z | no | yes |
| #2612 | `9bd58da1f` | `d83b32bee` | 2026-10-21T14:01:34Z | 2026-10-21T14:45:38Z | yes | no |
| #2613 | `11d5d2dd0` | `c104a6f02` | 2026-10-21T15:05:44Z | 2026-10-21T15:09:22Z | yes | yes |
| #2614 | `5a63b9ada` | `eca8d0916` | 2026-10-21T15:30:04Z | 2026-10-21T15:15:32Z | no | yes |
| #2615 | `0126fa3f7` | `c2fdf1ff5` | 2026-10-21T15:08:27Z | 2026-10-21T16:15:07Z | yes | yes |
| #2616 | `0043e21e3` | `2c2f0dcf2` | 2026-10-21T16:22:57Z | 2026-10-21T16:44:08Z | yes | yes |
| #2619 | `80c45b606` | `82bcbc65a` | 2026-10-21T17:10:22Z | 2026-10-21T16:52:39Z | no | yes |
| #2620 | `7963007ee` | `1bef679ea` | 2026-10-21T17:00:34Z | 2026-10-21T17:12:11Z | yes | yes |
| #2621 | `97c214c76` | `98aa6b65c` | 2026-10-23T11:10:05Z | 2026-10-23T15:31:04Z | yes | yes |
| #2623 | `5e9e575d8` | `01a65acee` | 2026-10-24T09:00:39Z | 2026-10-23T16:43:47Z | no | yes |
| #2624 | `756b08b2e` | `ff44a2bac` | 2026-10-22T18:50:56Z | 2026-10-23T07:12:02Z | yes | yes |
| #2627 | `67e37efd3` | `26bf2f59f` | 2026-10-23T15:51:02Z | 2026-10-23T15:55:25Z | yes | no |
| #2629 | `8f8b4d714` | `3a62c7e3c` | 2026-10-24T17:23:16Z | 2026-10-24T12:51:05Z | no | yes |
| #2630 | `a900e4545` | `ca34aa9ce` | 2026-10-24T17:26:00Z | 2026-10-24T19:17:58Z | yes | yes |
| #2633 | `0599fbd05` | `c19fca7d1` | 2026-10-24T17:19:30Z | 2026-10-24T19:12:34Z | yes | yes |
| #2634 | `e80d5c70e` | `7c2b7c7a3` | 2026-10-24T19:29:01Z | 2026-10-24T19:59:02Z | yes | yes |
| #2636 | `1d14569cd` | `9e5c38005` | 2026-10-24T20:15:30Z | 2026-10-24T20:42:24Z | yes | yes |
| #2641 | `849f5a809` | `453c6fa9a` | 2026-10-24T20:39:27Z | 2026-10-25T06:18:48Z | yes | yes |
| #2642 | `722f2b5b2` | `aede6ed69` | 2026-10-24T21:11:30Z | 2026-10-24T20:52:20Z | no | yes |
| #2643 | `2451f97bb` | `dc9d81b0a` | 2026-10-24T20:57:36Z | 2026-10-25T06:39:24Z | yes | yes |
| #2644 | `1b6129768` | `0af1e95b2` | 2026-10-25T06:27:57Z | 2026-10-25T06:23:29Z | no | yes |
| #2648 | `945d482d3` | `1cab588e6` | 2026-10-26T12:44:24Z | 2026-10-26T16:46:03Z | yes | yes |
| #2649 | `5124c66e2` | `7d8ee56e4` | 2026-10-25T14:36:43Z | 2026-10-25T14:47:34Z | yes | yes |
| #2652 | `d0c3b7e15` | `6c0d9cd80` | 2026-10-26T16:46:33Z | 2026-10-26T18:37:02Z | yes | yes |
| #2654 | `47cba5a97` | `9378ec1dd` | 2026-10-26T12:52:32Z | 2026-10-26T12:31:13Z | no | yes |
| #2657 | `0b4b2d0d3` | `5de6253f2` | 2026-10-26T17:21:22Z | 2026-10-26T17:24:31Z | yes | yes |
| #2658 | `f4b221095` | `68833b888` | 2026-10-26T17:11:58Z | 2026-10-26T17:15:15Z | yes | yes |
| #2659 | `b6cf72147` | `6b5177304` | 2026-10-26T17:56:56Z | 2026-10-26T18:21:44Z | yes | yes |
| #2664 | `5cbc4ec66` | `998637855` | 2026-10-26T20:09:07Z | 2026-10-26T21:54:55Z | yes | yes |
| #2666 | `71e130242` | `3b4a648ca` | 2026-10-27T07:05:05Z | 2026-10-26T21:59:54Z | no | yes |
| #2667 | `802612dce` | `ca784109d` | 2026-10-27T08:09:48Z | 2026-10-27T10:44:17Z | yes | yes |
| #2669 | `05c088306` | `65faa45b1` | 2026-10-27T09:23:06Z | 2026-10-27T10:00:44Z | yes | yes |
| #2671 | `24d708208` | `6ec28a96a` | 2026-10-27T09:13:24Z | 2026-10-27T11:44:47Z | yes | yes |
| #2675 | `249216aed` | `e7c0887ab` | 2026-10-27T12:17:39Z | 2026-10-27T11:51:27Z | no | yes |
| #2676 | `8e5459ada` | `248f95836` | 2026-10-28T07:10:08Z | 2026-10-28T09:29:36Z | yes | yes |
| #2678 | `0edeb136c` | `a6457bf87` | 2026-10-28T13:31:11Z | 2026-10-28T16:21:42Z | yes | yes |
| #2680 | `883cc834c` | `3d7d18368` | 2026-10-28T11:45:30Z | 2026-10-28T11:44:08Z | no | yes |
| #2682 | `f6c26d855` | `98a6e13b3` | 2026-10-28T13:48:48Z | 2026-10-28T16:12:00Z | yes | yes |
| #2683 | `aace1874f` | `a70d3a662` | 2026-10-28T17:54:46Z | 2026-10-28T18:10:01Z | yes | yes |
| #2686 | `db1789b66` | `a08a842d0` | 2026-10-28T16:43:30Z | 2026-10-28T17:11:37Z | yes | yes |
| #2687 | `424dd91fd` | `b2bb60d3d` | 2026-10-28T17:25:01Z | 2026-10-28T17:18:15Z | no | yes |

## 4. CI Cost of Pull-Request Runs (baseline section 6)

The runs are the `pull_request` workflow runs created in the window (all PRs, merged or not). One
request for the whole window returns at most 1000 runs, so the runs were listed per creation day:

```sh
for d in $(seq 15 28); do
  gh api --paginate "repos/torrust/torrust-tracker/actions/runs?event=pull_request&created=2026-10-$d&per_page=100" \
    --jq '.workflow_runs[]|[.id,.name,.created_at,.run_started_at,.updated_at,.conclusion]|@tsv'
done
```

That gives 1894 runs. Runner time per run is the mean summed job duration (`completed_at` minus
`started_at`, all jobs) of six runs sampled per workflow (all five for `Copilot Setup Steps`). The
sample is reproducible with seed `2547`: the run ids of a workflow, sorted ascending, are shuffled with
`shuf -n 6 --random-source=<(openssl enc -aes-256-ctr -pass pass:2547 -nosalt </dev/zero)`.

```sh
gh api --paginate repos/torrust/torrust-tracker/actions/runs/<id>/jobs \
  --jq '.jobs[]|select(.started_at!=null and .completed_at!=null)|[.started_at,.completed_at]|@tsv'
```

| Workflow                       | Runs | Mean job-minutes per run | Estimated runner-hours |
| ------------------------------ | ---: | -----------------------: | ---------------------: |
| Generate Coverage Reports (PR) | 324 | 34.4 | 185.5 |
| Testing | 324 | 26.0 | 140.2 |
| OS Compatibility | 324 | 24.8 | 133.8 |
| Container | 324 | 14.1 | 76.4 |
| Docs Lint | 520 | 1.6 | 13.8 |
| Database Compatibility | 32 | 16.1 | 8.6 |
| Security Scan | 9 | 32.8 | 4.9 |
| Database Benchmarking | 32 | 5.8 | 3.1 |
| Copilot Setup Steps | 5 | 10.1 | 0.8 |
| **Total**                      | 1894 |                          |              **567.1** |

Sampled run ids by workflow:

- Container: 39594972858, 39980508319, 40672441023, 39561659745, 39315165857, 40078761362
- Copilot Setup Steps: 40229361313, 40225974357, 40013596424, 40228883080, 40402491003
- Database Benchmarking: 39253735180, 38892902300, 38892583325, 38866301345, 39246697370, 39254704308
- Database Compatibility: 39253735217, 38892902287, 38892583343, 38866301295, 39246697243, 39254704250
- Docs Lint: 39929995064, 39837141278, 40489021802, 39253735195, 39255154889, 40077375038
- Generate Coverage Reports (PR): 39594972871, 39980508208, 40672440955, 39561659690, 39315165500, 40078761355
- OS Compatibility: 39594972937, 39980508168, 40672441248, 39561659823, 39315165637, 40078761306
- Security Scan: 40648337098, 39313465505, 40402491840, 40391663023, 39421908057, 40219343861
- Testing: 39594972851, 39980508199, 40672440978, 39561659655, 39315165758, 40078761307

Each mean rests on six runs, so a total can move by tens of runner-hours with another seed. About 41%
of `Testing` runs (134 of 324) ended `cancelled` (superseded pushes), so job time of those runs is
partial; the baseline method also averaged over all runs, so the comparison is like for like.
`Copilot Setup Steps` (5 runs) is not in the baseline table.

## 5. Rebase-Fallout Commits (baseline section 7)

Candidates by the baseline command, filtered by author date to the window; then classified by reading
the subjects. The baseline's section 7 list is a broad proxy for review noise caused by rebases: it
holds commits that record or repair a rebase, and commits that re-cite an id by subject so a later
rebase cannot orphan it, which is preventive and not a repair. This evidence applies one definition to
both periods and reports the two kinds apart. The maintainer reviews this classification.

```sh
git log --no-merges --since=2026-09-25 --until=2026-11-01 --format='%h|%aI|%s' -i -E \
  --grep='rebase|rebased|rebasing|instead of (the )?(branch )?(commit )?id|by subject|rebase-safe|orphan' torrust/develop
```

Kind A, the subject names a rebase (it records one or repairs its effects). Window, author dates
2026-10-15 to 2026-10-28 (5):

- `a6457bf87` record the second rebase onto develop for #2668
- `6d8eefc4c` link pull request 2678 and record the rebase for #2668
- `2d58f3b46` [#2582] record the checks rerun after rebasing onto develop
- `147a22cf5` [#2414] record the second rebase onto develop
- `4a1ef60a0` [#2414] record the rebase onto the latest develop

Kind B, the subject cites a commit by its subject instead of its id, a preventive housekeeping edit
under the branch-id rule. Window (6):

- `a7bfc15f7` [#2466] cite the verified commit by subject in the evidence file
- `453c6fa9a` [#2606] cite the checked commit by subject in the evidence
- `ae08130cb` [#2628] cite the round 3 head by subject in the F12 correction
- `faf21a128` [#2416] cite a branch commit by its subject, not its id, in the audit
- `261efbe43` cite the reviewed head by subject in the PR 2624 audit
- `f3b1b1bc0` [#2413] cite the approved head by commit subject in the retrospective

Excluded from both kinds, a repair after a force-push that was not a rebase (1): `ed3a4f756` cite
commit subjects instead of ids in the PR 2630 audit. PR #2630's one force-push (2026-10-23 17:46 UTC,
section 2.1) left the base unchanged, and the commit was authored on 2026-10-24. The baseline period
has no such case, because history rewrites other than rebases were not allowed then.

Candidates matched but not counted (not repairs of rebase fallout): `12b9c922f`, `b6d87209d`,
`80b42b7b2`, `f25c370e6`, `62a87a4dd`, `aaf2f9b4b`, `437f79b54`, the #2547 and #2507 measurement
commits (`cbf2b8b3b`, `1dea06489`, `0dbe53d32`, `436a91dde`, `7a3d1ccc1`, `2733f2ecf`). Borderline:
`b174608fa` (corrects an audit's head citation and findings; not counted).

Baseline recount: 13 of the 23 commits listed in baseline section 7 have author dates from 1 to 13
October (checked with `git log -1 --format='%h %aI' <id>`). Kind A (6): `505fa5745`, `1736f0d91`,
`03bc9d794`, `4596fb323`, `648d184b6`, `0be2875aa`. Kind B (7): `ef27e6585`, `6f7053686`, `9bc523862`,
`3b6c63027`, `df4b11a29`, `91b0f5b22`, `943b02765`. The other ten are dated 2 to 30 September.

| Kind                       | 1 to 13 Oct | Per merged PR (68) | 15 to 28 Oct | Per merged PR (121) |
| -------------------------- | ----------: | -----------------: | -----------: | ------------------: |
| A, the subject names a rebase |        6 |               0.09 |            5 |                0.04 |
| B, cite by subject         |           7 |               0.10 |            6 |                0.05 |
| **A + B (broad proxy)**    |      **13** |           **0.19** |       **11** |            **0.09** |

## 6. Workflow-Changing Merges in the Window

```sh
git log --first-parent --format='%h %cI %s' --since=2026-10-14T20:00:00Z --until=2026-10-29T00:00:00Z \
  torrust/develop -- .github/workflows
git diff --name-only "$M^1" "$M" -- .github/workflows    # files of one merge M
```

Four merges changed workflow files in the window. They changed seven distinct files between them; the
files per merge are listed below, so the number of merges and the number of files are not confused.
Times are GitHub's `merged_at` (UTC). The Git committer times of the same merges are 12:58:10,
11:08:59, 21:54:55 and 09:32:27 UTC; `merged_at` is the later of the two by 2 to 10 minutes.

| PR    | Merged (`merged_at`, UTC) | Change (subject)                                           | Workflow files changed                                                                                                                                       | #2525 |
| ----- | ------------------------- | ---------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------ | ----- |
| #2572 | 2026-10-20 13:00:47       | ci(workflows): update GitHub Actions                       | `coverage.yaml`, `generate_coverage_pr.yaml`, `security-scan.yaml`, `testing.yaml` (4)                                                                       | no    |
| #2601 | 2026-10-21 11:18:42       | fix(axum-health-check-api-server): remove the self dev-dependency and check workspace packaging | `testing.yaml` (1)                                                                                                       | no    |
| #2665 | 2026-10-26 21:57:03       | ci(workflows): update GitHub Actions                       | `container.yaml`, `copilot-setup-steps.yml`, `coverage.yaml`, `docs-lint.yaml`, `generate_coverage_pr.yaml`, `security-scan.yaml`, `testing.yaml` (7)        | no    |
| #2676 | 2026-10-28 09:34:42       | ci(container): build the shipped binaries without test features | `testing.yaml` (1)                                                                                                                                  | no    |

PR #2574 (the #2525 change-detection tool, `merged_at` 2026-10-20 16:10:04 UTC), #2580 and #2675 are
in the window but change no file under `.github/workflows` (`git diff --name-only M^1 M | grep -c
'^.github/workflows'` is 0), so the path search does not list them. No merge in the window wires the
`CI gate` of #2525 into a workflow. PR #2523 (`merged_at` before the window) is not listed.

## 7. Merge Bunching and Waiting

The maintainer expects a gain mainly from not waiting: with required rebases, each merge made the
next PR stale, so a queue of passing PRs had to merge one at a time, each after a rebase and its CI
run. This section measures that with two figures from data already collected. It was added after the
first read of the results.

- Gaps between consecutive merges, by `merged_at`, within each period. A gap of 15 minutes or less is
  a merge that followed another almost at once, which a rebase and CI wait between merges prevents
  for code PRs.
- Time from the end of the last finished `pull_request` run of the merged head to `merged_at`. The
  runs of each head came from the request below, taking the maximum `updated_at` of runs concluded
  `success` or `failure`. `M2` is the PR head of the first-parent merge, as in section 3.

```sh
gh api "repos/torrust/torrust-tracker/actions/runs?head_sha=$M2&event=pull_request&per_page=100" \
  --jq '[.workflow_runs[]|select(.conclusion=="success" or .conclusion=="failure")|.updated_at]|max // "none"'
```

The statistics were computed with this ad-hoc Python (`SCRATCH` was a scratch directory holding the
`prs.json` of section 1 and a `ci2merge.tsv` file with one line per merge: period, PR, `merged_at`, last
run time):

```python
import json,sys,statistics as st
from datetime import datetime as D
def t(s): return D.strptime(s,"%Y-%m-%dT%H:%M:%SZ").timestamp()
prs=json.load(open(SCRATCH+"/prs.json"))
for name,a,b in [("base","2026-10-01","2026-10-14"),("win","2026-10-15","2026-10-29")]:
    m=sorted(t(p["mergedAt"]) for p in prs if p["mergedAt"] and a<=p["mergedAt"][:10]<b)
    g=[(y-x)/60 for x,y in zip(m,m[1:])]
    n=len(g)
    print(name,"merges",len(m),"median gap min %.1f"%st.median(g),"<=15min %d (%.0f%%)"%(sum(x<=15 for x in g),100*sum(x<=15 for x in g)/n),"<=5min %d (%.0f%%)"%(sum(x<=5 for x in g),100*sum(x<=5 for x in g)/n))
    # runs: merges that have another merge within the previous 15 min
    r=[]
    for line in open(SCRATCH+"/ci2merge.tsv"):
        per,n_,mt,last=line.rstrip("\n").split("\t")
        if per==name: r.append((t(mt)-t(last))/60)
    r.sort()
    q=lambda p:r[int(p*(len(r)-1))]
    print(name,"CI-done->merge n=%d median %.1f min p75 %.1f p90 %.1f; <=15min %d (%.0f%%); >60min %d (%.0f%%)"%(len(r),q(.5),q(.75),q(.9),sum(x<=15 for x in r),100*sum(x<=15 for x in r)/len(r),sum(x>60 for x in r),100*sum(x>60 for x in r)/len(r)), "negative:",sum(x<0 for x in r))
```

| Measure                                               | 1 to 13 Oct | 15 to 28 Oct |
| ----------------------------------------------------- | ----------: | ------------: |
| Merges                                                |          68 |           121 |
| Median gap between consecutive merges (minutes)       |        68.3 |          23.3 |
| Gaps of 15 minutes or less                            |     3 (4%)  |     49 (41%)  |
| Gaps of 5 minutes or less                             |     1 (1%)  |      7 (6%)   |
| Merges with a finished CI run (CI to merge)           |          68 |           121 |
| Median minutes from last finished CI run to merge     |        14.9 |          46.3 |
| 90th percentile of the same                           |       130.1 |         605.3 |

Reading:

- Merges are far more bunched in the window: the share of consecutive merges within 15 minutes rose
  from 4% to 41%, and the median gap fell from 68 to 23 minutes. Part of this is volume (5.2 to 8.6
  merges per day), but volume alone is unlikely to multiply the 15-minute share tenfold. This is
  consistent with queued PRs no longer waiting for a rebase and its CI between merges. It does not
  prove the cause.
- The time from the last CI run to merge is longer in the window. It is not a measure of rebase
  waiting: in the baseline the last run was usually the rebase's, so the merge followed it closely
  by construction, while in the window a green PR waits for review and for the maintainer. The figure
  cannot separate idle time after CI from waiting for a rebase, so it neither supports nor
  contradicts the expectation.

## 8. Workload Mix (documentation and code)

The specification names the mix of documentation and code PRs as a confounder of CI time, because the
Rust workflows skip Markdown-only changes (`paths-ignore: "**/*.md"`). Rule: a merge is
documentation-only when every path changed between its first parent `M^1` and the merge commit `M` is a
Markdown file or `project-words.txt`; any other path makes it a code (other) merge. The command is in
Appendix A.6.

| Period       | Merges | Documentation-only | Other | Other share |
| ------------ | -----: | -----------------: | ----: | ----------: |
| 1 to 13 Oct  |     68 |                 49 |    19 |       27.9% |
| 15 to 28 Oct |    121 |                 62 |    59 |       48.8% |

The share of merges that run the full Rust workflows rose by a factor of 1.75 (48.8 / 27.9). The
per-merged-PR CI cost rose by a factor of 1.59 (4.7 / 2.95, with the published 67-PR denominator for
the baseline). A documentation-only PR costs about 1.4 job-minutes and a code PR about 92 (baseline
section 6), so a change of mix of this size can account for most or all of the per-PR increase. The
data here cannot split the CI cost by class, because the sample of six runs per workflow was not
stratified, and the runs of PRs that never merged are counted in the window total but have no merge
to classify. The per-merged-PR CI rows are therefore not evidence of a policy effect in either
direction.

## 9. Comparison

Per-day figures divide by 13 (baseline) or 14 (window); per-PR figures by the merged PRs of the same
column: 67 for the published snapshot, 68 for the recount, 121 for the window. A figure the snapshot
did not record, or that this evidence did not recount, is `n/a`. The CI rows use the published
snapshot because the baseline's sampled runs cannot be recounted, so they are not recounted.

| Measure                                    | Baseline published | Baseline recount | Window (15 to 28 Oct) |
| ------------------------------------------ | -----------------: | ---------------: | --------------------: |
| Merged PRs                                 |                 67 |               68 |                   121 |
| Merged PRs per day                         |                5.2 |              5.2 |                   8.6 |
| Force-pushes per merged PR                 |               1.58 |             1.56 |                  0.09 |
| Force-pushes per day                       |                8.2 |              8.2 |                   0.8 |
| Merged PRs force-pushed at least once      |                81% |            79.4% |                  8.3% |
| Rebase-only force-pushes per merged PR     |                n/a |              n/a |                  0.08 |
| Merges while behind `develop`              |                n/a |              n/a |             93 (77%)  |
| Merges on the refresh path (inferred)      |                n/a |              n/a |             68 (56%)  |
| `pull_request` runs                        |       899 (69/day) |              n/a |        1894 (135/day) |
| `pull_request` runs per merged PR          |               13.4 |              n/a |                  15.7 |
| Estimated PR CI runner-hours               |              197.8 |              n/a |                 567.0 |
| Runner-hours per day                       |               15.2 |              n/a |                  40.5 |
| Runner-hours per merged PR                 |                3.0 |              n/a |                   4.7 |
| Fallout commits, kind A (names a rebase)   |                n/a |                6 |                     5 |
| Fallout commits, A + B (broad proxy)       |                n/a |               13 |                    11 |
| A + B per day                              |                n/a |             1.00 |                  0.79 |
| A + B per merged PR                        |                n/a |             0.19 |                  0.09 |
| Median daily open-PR queue                 |                  8 |                8 |                     5 |
| Consecutive merges within 15 minutes       |                n/a |               4% |                   41% |
| Median gap between merges (minutes)        |                n/a |             68.3 |                  23.3 |
| Median hours to merge                      |                4.0 |              3.8 |                   1.9 |

## 10. Conclusion

- Force-pushes fell by about 94% per merged PR (1.56 to 0.09 against the recount; 1.58 against the
  published snapshot) and the share of merged PRs that were force-pushed fell from 79.4% (81%
  published) to 8.3%. Of the 11 pushes, 10 moved the base: six because the head
  conflicted with `develop`, four recommended and clean.
- 77% of merges were made behind `develop`; 56% of all merges are consistent with the merge tool's
  recorded-base refresh path (inferred, not observed).
- Median time to merge halved (3.8 to 1.9 hours against the recount; 4.0 published) and the median
  queue fell from 8 to 5, with 1.65 times as many merges
  per day (121/14 against 68/13). The broad fallout proxy per merged PR halved (0.19 to 0.09) and kind A, the commits that name a rebase, fell from 0.09 to 0.04 per merged PR; per day the broad proxy fell less, from 1.00 to 0.79 (section 5).
- Merges are bunched far more tightly: consecutive merges within 15 minutes went from 4% to 41%
  (section 7), consistent with queued PRs no longer waiting for a rebase and its CI between merges.
  The time from CI to merge cannot show this and is inconclusive.
- PR CI cost did not fall: it rose from about 15 to about 41 runner-hours per day and from 3.0 to 4.7
  per merged PR. Runs per merged PR rose only from 13.4 to 15.7, so most of the per-day increase
  follows volume (merged PRs per day up 65%). The per-PR rise coincides with a change of workload
  mix: the share of merges that touch code rose from 27.9% to 48.8% (section 8), and the Rust workflows
  skip documentation-only changes. This evidence cannot separate the mix from any effect of the
  policy, so it neither shows a CI saving nor a CI increase caused by the policy.
- The data does not argue for reverting the policy. It does not show a CI saving either.

## 11. Caveats

- A rebase onto `develop` is not the only reason for a new head. The CI figure counts every pushed
  head, not only rebases; the baseline's "29% of CI time attributable to rebases" is an estimate, and
  none is made here.
- Six sampled runs per workflow give wide error bars; the totals are estimates, and the seed above
  reproduces the sample, not the exact minutes.
- The window had 1.65 times the merges per day of the baseline (121/14 against 68/13), so absolute
  figures are not comparable. Per-merged-PR figures remove that difference but not the workload mix:
  the share of code merges rose from 27.9% to 48.8% (section 8), which affects the CI rows in
  particular.
- The `CI gate` of #2525 landed no workflow wiring in the window (section 6), and four merges
  changed seven workflow files for other reasons. #2665 alone touched all seven, so a change of
  workflow runtime between the periods cannot be attributed to one of them.
- The refresh-path count is inferred from run creation times, because the merge tool's output is not
  captured. Two of its 68 merges do not show `base.sha` equal to `M^1`.
- Fallout classification is a reading of commit subjects and is the maintainer's to confirm.

## Appendix A. Aggregation Commands and Per-Run Data

The retrieval commands in sections 1 to 7 stop at the API. This appendix records the commands that
turn their output into the tables, and the per-run totals the CI table is built from. `prs.json` is the
file written by the `gh pr list` command of section 1. `SCRATCH` stands for the scratch directory that
held the raw files; the per-run data that matters is kept in A.5.

### A.1 The merged-PR lists

```sh
jq -r '.[]|select(.mergedAt!=null and .mergedAt>="2026-10-15T00:00:00Z" and .mergedAt<"2026-10-29T00:00:00Z")|.number' prs.json | sort -n > win.txt
jq -r '.[]|select(.mergedAt!=null and .mergedAt>="2026-10-01T00:00:00Z" and .mergedAt<"2026-10-14T00:00:00Z")|.number' prs.json | sort -n > base.txt
wc -l < win.txt; wc -l < base.txt     # 121 and 68
```

### A.2 Volume, daily queue and median hours to merge (section 1)

The queue at day `d` is the number of PRs created at or before 12:00 UTC that day and either still
open or merged after that time. The baseline "67 PRs" row excludes #2506.

```python
import json,sys,statistics as st
from datetime import datetime as D
t=lambda s:D.strptime(s,"%Y-%m-%dT%H:%M:%SZ").timestamp()
prs=json.load(open("prs.json"))
for name,a,b,days,excl in [("base68","2026-10-01","2026-10-14",13,[]),("base67","2026-10-01","2026-10-14",13,[2506]),("win","2026-10-15","2026-10-29",14,[])]:
    mg=[p for p in prs if p["mergedAt"] and a<=p["mergedAt"][:10]<b and p["number"] not in excl]
    h=[(t(p["mergedAt"])-t(p["createdAt"]))/3600 for p in mg]
    day0=t(a+"T00:00:00Z")
    q=[]
    for d in range(days):
        pt=day0+d*86400+43200
        q.append(sum(1 for p in prs if t(p["createdAt"])<=pt and (p["state"]=="OPEN" or (p["mergedAt"] and t(p["mergedAt"])>pt)) and not (p["number"] in excl)))
    print(name,len(mg),"median_h %.4f"%st.median(h),"queue median",st.median(q),"max",max(q))
```

Output:

```text
base68 68 median_h 3.7828 queue median 8 max 12
base67 67 median_h 4.0086 queue median 8 max 12
win 121 median_h 1.9186 queue median 5.0 max 8
```

Opened counts: `jq '[.[]|select(.createdAt>="2026-10-15T00:00:00Z" and .createdAt<"2026-10-29T00:00:00Z")]|length' prs.json`
gives 130 and the same filter for 1 to 14 October gives 76.

### A.3 Force-push classification (section 2.1)

`fp_merges_t.txt` lists, newest first, each first-parent `develop` merge commit with its PR number and
`merged_at`. `fp11.tsv` has one line per force-push: PR, time, `beforeCommit`, `afterCommit`.

```sh
git log --first-parent --format='%H%x09%s' torrust/develop --since=2026-10-01 \
  | sed -n 's/^\([0-9a-f]*\)\tMerge torrust\/torrust-tracker#\([0-9]*\):.*/\1 \2/p' > fp_merges.txt
jq -r '.[]|select(.mergedAt!=null)|"\(.number) \(.mergedAt)"' prs.json | sort > merged_at.txt
while read c n; do echo "$c $n $(grep "^$n " merged_at.txt | cut -d' ' -f2)"; done < fp_merges.txt > fp_merges_t.txt
while IFS=$'\t' read n t b a; do
  tip=$(awk -v t=$t '$3<t{print $1; exit}' fp_merges_t.txt)
  mb=$(git merge-base $tip $b); ma=$(git merge-base $tip $a)
  if [ "$mb" = "$ma" ]; then cls="not-rebase"; else
    if git merge-tree --write-tree $tip $b >/dev/null 2>&1; then cls="rebase-recommended"; else cls="rebase-required"; fi; fi
  echo -e "$n\t$t\t${tip:0:9}\t${mb:0:9}\t${ma:0:9}\t$cls"
done < fp11.tsv
```

`git log --since` stops at an older commit, so it missed two merges of 1 October (#2397 and #2400);
they were found with `git log --first-parent --grep="#<n>:" torrust/develop`. Neither is a force-push
candidate, but both are in the merge-bunching and workload-mix figures.

### A.4 Merges behind `develop` and the refresh path (section 3)

```sh
for n in $(cat win.txt); do
  M=$(awk -v n=$n '$2==n{print $1}' fp_merges.txt)
  p1=$(git rev-parse $M^1); p2=$(git rev-parse $M^2)
  mb=$(git merge-base $p1 $p2); behind=no; [ "$mb" != "$p1" ] && behind=yes
  t1=$(git log -1 --format=%cI $p1 | xargs -I{} date -u -d {} +%Y-%m-%dT%H:%M:%SZ)
  first=$(gh api "repos/torrust/torrust-tracker/actions/runs?head_sha=$p2&event=pull_request&per_page=100" --jq '[.workflow_runs[].created_at]|min // "none"')
  base=$(gh api repos/torrust/torrust-tracker/pulls/$n --jq .base.sha); [ "$base" = "$p1" ] && bm=yes || bm=no
  echo -e "$n\t${M:0:9}\t${p1:0:9}\t${p2:0:9}\t$behind\t$t1\t$first\tbase=M1:$bm"
done > v1.tsv
awk -F'\t' '$5=="yes"' v1.tsv | wc -l                       # behind: 93
awk -F'\t' '$5=="yes" && $7!="none" && $7<$6' v1.tsv | wc -l # refresh path: 68
```

### A.5 CI cost (section 4)

`runs_u.tsv` is the deduplicated per-day run list of section 4. The per-workflow run count is
`cut -f2 runs_u.tsv | sort | uniq -c`. A run's job-minutes (`LC_ALL=C` keeps the decimal point):

```sh
gh api --paginate repos/torrust/torrust-tracker/actions/runs/<id>/jobs \
  --jq '.jobs[]|select(.started_at!=null and .completed_at!=null)|[(.started_at|fromdateiso8601),(.completed_at|fromdateiso8601)]|@tsv' \
  | awk -F'\t' '{t+=$2-$1} END{printf "%.2f\n", t/60}'
```

The mean per workflow is the arithmetic mean of its sampled runs' job-minutes below, and the
estimated runner-hours are `runs x mean / 60`. Workflow means to three decimals, and the resulting
hours before rounding:

| Workflow                       | Runs | Sampled | Mean job-minutes | Runner-hours |
| ------------------------------ | ---: | ------: | ---------------: | -----------: |
| Container | 324 | 6 | 14.142 | 76.37 |
| Copilot Setup Steps | 5 | 5 | 10.136 | 0.84 |
| Database Benchmarking | 32 | 6 | 5.852 | 3.12 |
| Database Compatibility | 32 | 6 | 16.090 | 8.58 |
| Docs Lint | 520 | 6 | 1.595 | 13.82 |
| Generate Coverage Reports (PR) | 324 | 6 | 34.353 | 185.51 |
| OS Compatibility | 324 | 6 | 24.765 | 133.73 |
| Security Scan | 9 | 6 | 32.752 | 4.91 |
| Testing | 324 | 6 | 25.957 | 140.17 |
| **Total** | 1894 | 53 | | **567.05** |

The total is 567.05 runner-hours before rounding, which is 567.1 to one decimal; section 4 and the
comparison round the per-workflow hours first. Per-run job-minutes of the 53 sampled runs, in the order
the seeded shuffle produced them (the first run of each workflow is the one the manual check V2 used):

| Workflow                       | Run id       | Job-minutes |
| ------------------------------ | ------------ | ----------: |
| OS Compatibility | 39594972937 | 25.88 |
| OS Compatibility | 39980508168 | 23.55 |
| OS Compatibility | 40672441248 | 21.13 |
| OS Compatibility | 39561659823 | 23.25 |
| OS Compatibility | 39315165637 | 29.85 |
| OS Compatibility | 40078761306 | 24.93 |
| Generate Coverage Reports (PR) | 39594972871 | 17.32 |
| Generate Coverage Reports (PR) | 39980508208 | 40.67 |
| Generate Coverage Reports (PR) | 40672440955 | 11.50 |
| Generate Coverage Reports (PR) | 39561659690 | 16.43 |
| Generate Coverage Reports (PR) | 39315165500 | 85.38 |
| Generate Coverage Reports (PR) | 40078761355 | 34.82 |
| Testing | 39594972851 | 21.47 |
| Testing | 39980508199 | 31.07 |
| Testing | 40672440978 | 23.40 |
| Testing | 39561659655 | 27.92 |
| Testing | 39315165758 | 26.00 |
| Testing | 40078761307 | 25.88 |
| Container | 39594972858 | 12.48 |
| Container | 39980508319 | 13.68 |
| Container | 40672441023 | 15.20 |
| Container | 39561659745 | 16.32 |
| Container | 39315165857 | 13.90 |
| Container | 40078761362 | 13.27 |
| Database Compatibility | 39253735217 | 14.33 |
| Database Compatibility | 38892902287 | 15.28 |
| Database Compatibility | 38892583343 | 17.70 |
| Database Compatibility | 38866301295 | 14.55 |
| Database Compatibility | 39246697243 | 19.83 |
| Database Compatibility | 39254704250 | 14.85 |
| Docs Lint | 39929995064 | 1.07 |
| Docs Lint | 39837141278 | 1.07 |
| Docs Lint | 40489021802 | 1.00 |
| Docs Lint | 39253735195 | 0.97 |
| Docs Lint | 39255154889 | 1.03 |
| Docs Lint | 40077375038 | 4.43 |
| Database Benchmarking | 39253735180 | 5.37 |
| Database Benchmarking | 38892902300 | 5.43 |
| Database Benchmarking | 38892583325 | 6.43 |
| Database Benchmarking | 38866301345 | 5.25 |
| Database Benchmarking | 39246697370 | 8.43 |
| Database Benchmarking | 39254704308 | 4.20 |
| Security Scan | 40648337098 | 40.40 |
| Security Scan | 39313465505 | 31.70 |
| Security Scan | 40402491840 | 33.50 |
| Security Scan | 40391663023 | 32.78 |
| Security Scan | 39421908057 | 23.88 |
| Security Scan | 40219343861 | 34.25 |
| Copilot Setup Steps | 40229361313 | 2.73 |
| Copilot Setup Steps | 40225974357 | 15.85 |
| Copilot Setup Steps | 40013596424 | 14.60 |
| Copilot Setup Steps | 40228883080 | 2.83 |
| Copilot Setup Steps | 40402491003 | 14.67 |

### A.6 Workload mix (section 8)

```sh
for n in $(cat base.txt win.txt); do
  m=$(grep " $n\$" fp_merges.txt | cut -d' ' -f1)
  [ -z "$m" ] && m=$(git log --first-parent --format=%H --grep="#$n:" torrust/develop | head -1)
  other=$(git diff --name-only $m^1 $m | grep -v -E '\.md$|^project-words\.txt$' | wc -l)
  echo -e "$n\t$([ $other -eq 0 ] && echo docs || echo other)"
done
```

Counts: baseline 49 docs and 19 other; window 62 docs and 59 other.
