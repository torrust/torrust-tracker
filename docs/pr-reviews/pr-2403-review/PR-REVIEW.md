---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
    - "issue #2386"
    - "issue #2402"
---

<!-- skill-link: process-pr-review -->

# PR #2403 Review Audit

Source: pull-request reviews and inline review threads for
<https://github.com/torrust/torrust-tracker/pull/2403>.

## Ownership

The PR author owns this tracked audit record. Reviewers, including repository review agents,
deliver findings through GitHub and have no repository-artifact obligation.

- Post-merge workflow approval: N/A

## Status Values

- Relationship: `ORIGINAL`, `RE_RAISE_OF:<FindingId>`
- Disposition: `FIXED`, `NO_ACTION`, `SUPERSEDED`, `FOLLOW_UP`
- Thread state: `OPEN`, `RESOLVED`, `NON_RESOLVABLE`, `SUPERSEDED`. `OPEN` applies prospectively
  to audits created or updated for approved follow-up work; historical audits remain valid without
  bulk migration.
- Severity: `Blocker`, `Major`, `Minor`, `Nit`, `Suggestion`; append `(inferred)` when derived
  from free prose.
- Author class: `Copilot`, `Human`, `Unknown`
- Category: `link-integrity`, `formatting`, `metadata`, `testing`, `correctness`,
  `documentation`, `maintainability`, `security`, `other`
- An outdated thread whose concern was fixed is `FIXED`/`RESOLVED`, even when GitHub marks the
  original thread outdated after the push. For in-PR feedback, use `NO_ACTION`/`SUPERSEDED` only
  for a duplicate, superseded, or no-change concern. A post-merge `NO_ACTION` requires maintainer
  approval to decline the follow-up work.

## Findings

Copilot review 5381662466 (round 1) left seven inline comments with reviewer finding IDs F1 to F6
and F8 (there is no F7) and `[Major]`/`[Minor]` severity brackets, recorded as given. Its overview
badges rate them one High, two Medium, and four Low. The audit keeps the reviewer IDs, which do not
collide. The review body holds only the overview and no further request.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | `review-finding:pr-2403-f1` | Copilot | Major | correctness | ORIGINAL | FIXED | RESOLVED |
| F2 | `review-finding:pr-2403-f2` | Copilot | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F3 | `review-finding:pr-2403-f3` | Copilot | Major | testing | ORIGINAL | FIXED | RESOLVED |
| F4 | `review-finding:pr-2403-f4` | Copilot | Major | testing | ORIGINAL | FIXED | RESOLVED |
| F5 | `review-finding:pr-2403-f5` | Copilot | Minor | link-integrity | ORIGINAL | FIXED | RESOLVED |
| F6 | `review-finding:pr-2403-f6` | Copilot | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F8 | `review-finding:pr-2403-f8` | Copilot | Minor | documentation | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - The cancelling replay did not simulate queued and running cancellations

- PR number: 2403
- Source review ID: 5381662466
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2403#discussion_r4157317689>
- Concern: The cancelling replay dropped a run only when a newer run on its branch arrived within
  one job time of it, so it missed runs superseded after queueing longer, and it treated a
  cancelled running job as using no runner time. With waits up to 239 minutes this is material,
  so the 12-minute 90th-percentile wait was not supported.
- Solution: V3 step 4 of the #2386 evidence is now an event-driven replay keyed by pull request
  (head repository and branch): a new pull-request run cancels its pull request's queued run and
  its running run, which frees the runner at that moment and counts the time used; push runs are
  never cancelled. The program is embedded in the evidence. All cancelling rows were recomputed:
  at 801 s on one runner, 90th percentile 11 minutes (was 12), maximum 32 (was 79), 12 runs over
  15 minutes (was 28). The #2386 Decision, the #2402 background table, and EPIC row 18 carry the
  corrected figures. The correction strengthens the decision rather than changing it.
- Current-tree verification: at the PR head after the 2026-10-01 16:58 UTC changes, the replay with
  `X=0` reproduces the published 801 s no-cancel rows exactly (206 waits over 15 minutes,
  90th percentile 140 minutes, maximum 239); with `X=1` it prints the new rows. `grep -cE '395
  runs|p90 12 min, max 79'` prints `0` for the evidence and the #2402 spec, and
  `grep -c '376 completed'` prints `1` for the evidence.
- Resolution reference: `docs(issues): [#2386] replay cancellations event by event and narrow the memory claim`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2403#discussion_r4159246388>

### F2 - The memory-limit claim went beyond the sampled window

- PR number: 2403
- Source review ID: 5381662466
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2403#discussion_r4157318108>
- Concern: Sampling began about 12.5 minutes into the 17.75-minute constrained build, and
  `memory.events max` counts limit hits rather than their duration, so the evidence does not show
  the build at its memory limit "throughout".
- Solution: The per-second samples, cut at the build end, were 262 from 14:48:39 UTC: median
  5.85 GiB, 69 above 7 GiB, and only 14 within 1% of the 8 GiB limit, so the claim was wrong
  even for the sampled window. V2 now reports those figures, notes that 40124 of the 41922 limit
  events happened before sampling began, and concludes only that memory pressure added to the CPU
  loss. Failures and Follow-up records the correction.
- Current-tree verification: at the PR head after the 2026-10-01 16:58 UTC changes, the only
  "throughout" in the evidence is the correction note that quotes the old wording.
- Resolution reference: `docs(issues): [#2386] replay cancellations event by event and narrow the memory claim`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2403#discussion_r4159247675>

### F3 - M1 could not observe both workflows or an unaffected pull request

- PR number: 2403
- Source review ID: 5381662466
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2403#discussion_r4157317808>
- Concern: M1 waited only for the first `Container` run, so the older `Testing` run could already
  have finished, and listing one branch cannot show that another pull request was unaffected.
- Solution: M1 of #2402 now sets up a control pull request with an active `Container` run, waits
  until both workflows of commit A are queued or running before pushing commit B, and inspects
  every run. AC3 is reworded to what M1 observes.
- Current-tree verification: at the PR head after the 2026-10-01 16:58 UTC changes, M1 in
  `docs/issues/open/2402-1840-cancel-superseded-pr-runs/ISSUE.md` names the control pull request
  and both workflows of commit A, and AC3 reads "a pull request's runs do not cancel another pull
  request's runs".
- Resolution reference: `docs(issues): [#2402] tighten verification scenarios and correct the replay table`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2403#discussion_r4159246667>

### F4 - One develop run could not show that push runs are not replaced

- PR number: 2403
- Source review ID: 5381662466
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2403#discussion_r4157317897>
- Concern: A shared push group with `cancel-in-progress: false` would pass a single-run check while
  still replacing an older pending run; AC2 needs two overlapping `develop` pushes.
- Solution: M2 of #2402 now requires two `develop` pushes whose runs overlap, with a fork's
  `develop` as the fallback, and expects all four runs to complete and both upstream `Container`
  runs to publish. The risk entry for the push group now points to M2.
- Current-tree verification: at the PR head after the 2026-10-01 16:58 UTC changes, M2 reads "the
  second push's runs start while the first push's `Container` and `Testing` runs are still queued
  or running".
- Resolution reference: `docs(issues): [#2402] tighten verification scenarios and correct the replay table`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2403#discussion_r4159246871>

### F5 - Issue-spec paths in frontmatter go stale when issues are archived

- PR number: 2403
- Source review ID: 5381662466
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2403#discussion_r4157318179>
- Concern: The #2402 frontmatter listed `docs/issues/open/` paths for #2386 and EPIC #1840, which
  move when those issues close; the semantic-link convention asks for `"issue #NNNN"`.
- Solution: The #2402 `related-artifacts` now list `"issue #2386"` and `"issue #1840"`. The
  spec's own `spec-path` remains, because it moves with the spec.
- Current-tree verification: at the PR head after the 2026-10-01 16:58 UTC changes, the only
  `docs/issues/open` path in the first 25 lines of the #2402 spec is its `spec-path`.
- Resolution reference: `docs(issues): [#2402] tighten verification scenarios and correct the replay table`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2403#discussion_r4159247958>

### F6 - The #2386 spec said the cancellation was already delivered

- PR number: 2403
- Source review ID: 5381662466
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2403#discussion_r4157317987>
- Concern: "Delivered in #2402" claimed an implementation that has not happened; #2402 was only
  opened. The same wording appeared in the progress log and the Decision.
- Solution: Both places now say "to be delivered in #2402".
- Current-tree verification: at the PR head after the 2026-10-01 16:58 UTC changes, every
  "delivered in #2402" in the #2386 spec is preceded by "to be".
- Resolution reference: `docs(issues): [#2386] correct the queueing figures and date the decision`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2403#discussion_r4159247137>

### F8 - The Decision gave the measurement date as the decision date

- PR number: 2403
- Source review ID: 5381662466
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2403#discussion_r4157318060>
- Concern: The Decision said it was recorded on 2026-09-30, but the runner-count decision was made
  on 2026-10-01.
- Solution: The Decision now dates the measurements (2026-09-30), the server-size decision
  (2026-09-30), and the runner-count and queueing decision (2026-10-01) separately.
- Current-tree verification: at the PR head after the 2026-10-01 16:58 UTC changes, the Decision
  opens with "Measurements from 2026-09-30" and each decided bullet carries its date.
- Resolution reference: `docs(issues): [#2386] correct the queueing figures and date the decision`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2403#discussion_r4159247368>

## Processing Log

- 2026-10-01 15:35 UTC - Copilot review 5381662466 submitted with seven inline findings.
- 2026-10-01 16:52 UTC - Committed the F1 replay and the F2 fix to the #2386 evidence.
- 2026-10-01 16:57 UTC - Committed the F1 figures, F6, and F8 in the #2386 spec and EPIC #1840.
- 2026-10-01 16:58 UTC - Committed the F1 table, F3, F4, and F5 in the #2402 spec.
- 2026-10-01 18:53 UTC - Re-derived each fix against the PR head and replied on all seven threads.
- 2026-10-01 19:00 UTC - Started this audit.

## Completion Rules

- Re-derive the reply claim against the current tree before replying or resolving a thread.
- Reply on every resolvable thread before resolving it.
- For an outdated thread whose concern was fixed, record `Disposition=FIXED` and
  `Thread state=RESOLVED`, even if GitHub marks the original thread outdated after the push. For a
  duplicate, superseded, or no-change in-PR thread, reply exactly
  `Superseded by <FindingId>: <reason>.`, record `Disposition=NO_ACTION` and
  `Thread state=SUPERSEDED`, then resolve it. A post-merge `NO_ACTION` requires maintainer
  approval to decline the follow-up work.
- A consolidated PR conversation response may cover multiple review rounds only when it names
  every review ID and every finding ID with its disposition and resolution reference. Record its
  durable URL in each related row.
- Cite a fix by its unique Conventional Commit subject or durable reply URL, never by a branch SHA
  that can change after a rebase.
- Refresh review threads using GraphQL and confirm that no unresolved actionable thread remains.
