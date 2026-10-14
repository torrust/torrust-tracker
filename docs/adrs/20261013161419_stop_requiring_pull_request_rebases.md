---
semantic-links:
  skill-links:
    - create-adr
    - open-pull-request
    - merge-pull-request
  related-artifacts:
    - .github/skills/dev/planning/create-adr/SKILL.md
    - .github/skills/dev/git-workflow/open-pull-request/SKILL.md
    - .github/skills/dev/git-workflow/merge-pull-request/SKILL.md
    - contrib/dev-tools/git/github-merge.py
    - "issue #2507"
---

<!-- skill-link: create-adr -->

# Stop Requiring Pull-Request Rebases

## Scope

Root ADR. It governs the repository-wide contribution workflow: when a pull-request branch
targeting `develop` should or must be rebased, and what the maintainer merge tool requires before
merging.
No package owns it.

## Description

For years the project has rebased every pull request onto the latest `develop` before merging it.
The rule was never recorded as a decision. It appears only as workflow instructions: the
`open-pull-request` skill requires a rebase before every push, the `merge-pull-request` skill
requires one immediately before merging, and from PR #2229 until issue #2507 the vendored
`github-merge.py` refused to merge when GitHub's pull-request merge ref was not based on the current
`develop` tip. GitHub
itself enforced up-to-date branches through ruleset 94237 from 2023, but that ruleset has been
disabled since 2026-05-05 because its required checks could no longer be satisfied. The implicit
motivation was a history in which each change appears to be built on top of the previous one, and,
more importantly, CI results that test exactly what lands, so integration problems are detected
before they reach `develop`.

The context changed. AI agents now prepare most pull requests. Merged pull requests rose from 20 in
April 2026 to 170 in September 2026, and the median number of open pull requests waiting to merge
rose from 0 to 8 in October 2026. Every merge makes every other open pull request stale, so the
always-rebase rule turns each merge into up to N rebases:

- **CI cost.** A rebase of a code pull request reruns about 92 runner-minutes of required workflows.
  Force-pushes per merged pull request rose from 0.39 (June) to 1.58 (October); an estimated 29% of
  pull-request CI time in early October was spent re-testing rebased heads.
- **Review noise.** Review records, evidence files, and issue specifications cite commits. Every
  rebase rewrites the ids they cite, so about two dozen commits since September only repaired
  orphaned ids, lost log lines, or re-ran verification after a rebase. Rules such as "cite branch
  commits by subject, never by id" exist only because of rebasing.
- **Lost approvals.** `dismiss_stale_reviews_on_push` dismisses approvals on every force-push,
  including byte-identical rebases.
- **Serialization.** Merging becomes a queue that must be drained one rebase-and-wait cycle at a
  time.

Meanwhile, the history benefit is small. The maintainer merge tool always creates a `--no-ff` merge
commit per pull request, so `develop` was never a linear history; `git log --first-parent develop`
already lists exactly one merge per pull request in merge order regardless of whether branches were
rebased.

There is one real technical constraint. `github-merge.py` pushes a locally signed merge commit, and
GitHub's `pull_request` ruleset accepts that push only when the pull request's recorded base
(`base.sha` in the API) equals the current `develop` tip. GitHub records that base when the pull
request is opened, its head is pushed, or its base is edited, but not reliably when `develop`
advances through locally pushed merges. This ADR originally named the freshness of
`refs/pull/<n>/merge` as the condition; the sandbox experiment of the implementing issue showed that
pushes are accepted with a stale merge ref and rejected with a stale `base.sha`
([evidence](../issues/open/2507-stop-requiring-pull-request-rebases/manual-verification-evidence.md),
sections T1 and T5).

Evidence and reproduction commands: issue-local
[`baseline-evidence.md`](../issues/open/2507-stop-requiring-pull-request-rebases/baseline-evidence.md).

## Agreement

Rebasing a pull request onto the latest `develop` is **recommended but no longer required**. The
requirement is dropped only for performance reasons, to reduce CPU consumption and runner costs;
the value of detecting integration problems before merging is unchanged.

Contributors and agents **should** rebase:

- when opening a pull request;
- when they know `develop` has changed since the last push; and
- in general, whenever they can and it is not a bottleneck or a problem.

Skip a rebase that would be a bottleneck or a problem, for example when it would force a queue of
pull requests to rerun expensive CI after every merge, or invalidate commit ids cited by an
in-progress review round.

A pull request **must** be rebased or otherwise updated only when:

1. **Conflicts.** It no longer merges cleanly into `develop`.
2. **Dependency.** It needs a change that landed on `develop` after it was opened, or a reviewer
   shows that the combination with newer `develop` changes must be tested before merge (for example
   both touch the same Rust module or CI workflow).
3. **Merge precondition.** The pull request's recorded base is stale and the maintainer merge
   tool's own refresh is unavailable (see below).

Merge workflow requirements:

- The merge tool refuses to sign or push a merge while the pull request's recorded base is not the
  current `develop` tip. It refreshes that base itself with a no-op base edit (setting the pull
  request's base to `develop` again), which adds no commit, timeline event, or CI run. When that
  refresh is unavailable, its recovery instructions name the same edit as a command, then GitHub's
  "Update branch" (a merge commit from `develop`, signed by GitHub, which keeps existing commit
  ids), and a rebase as alternatives.
- Review records may cite pull-request commit ids. Without mandatory rebases those ids usually stay
  valid and, because merges are `--no-ff`, remain reachable from `develop` after merge. Existing
  "cite by subject" guidance remains a safe default because rebases still happen.

Integration safety relies on the recommended rebases and on the post-merge `push` workflows on
`develop`, which test every merged state. A breakage found there is fixed forward or reverted like
any other regression.

GitHub configuration: no repository rule may require up-to-date branches. Any ruleset with required
status checks must set `strict_required_status_checks_policy` to `false`, must be satisfiable by
both code and documentation-only pull requests, and must not need to be disabled to merge a
known-good pull request. The implementing issue reconciles the existing rulesets accordingly.

Transition: this recommendation applies as soon as this ADR is merged. The pull request that adds
this ADR also updates the `open-pull-request` and `merge-pull-request` skills and `AGENTS.md`, so no
workflow instruction keeps the old mandatory rule. Until the implementing issue changes the merge
tool, a behind pull request still needs a head push (rebase or "Update branch") when the tool
refuses it as stale.

### Alternatives Considered

| Option                                                 | Pros                                                                                                                                                                                                                                      | Cons                                                                                                                                                                                                                   |
| ------------------------------------------------------ | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **A. Keep always-rebase** (status quo)                 | Every merge is tested exactly as it lands; tidy full graph; no workflow or tool changes; merge tool already works this way.                                                                                                               | Cost grows with queue length (each merge forces N rebases and N CI reruns); rewritten ids break review evidence; approvals dismissed; merges serialize; rule contradicts `AGENTS.md`; no recorded rationale behind it. |
| **B. Never rebase, merge whatever GitHub accepts**     | Simplest rule; no CI reruns; ids always stable.                                                                                                                                                                                           | Blocked by the ruleset interaction: GitHub rejects the merge push with `GH013` while the recorded base is stale; conflicting PRs still need updates; no way to request integration testing for risky overlaps.         |
| **C. Recommend, do not require** (chosen)              | Removes mandatory rebases and most CI reruns; keeps early detection whenever a rebase is cheap; ids cited in reviews usually stay valid; approvals survive; required updates remain for conflicts, dependencies, and merge preconditions. | Some combinations of PRs are first tested together on `develop` after merge; full graph shows overlapping merge bubbles; merge tool, skills, and rulesets must change; needs a reliable refresh of the recorded base.  |
| **D. Merge queue / GitHub-side merge**                 | GitHub batches and tests merges automatically.                                                                                                                                                                                            | Loses maintainer-signed merges with `Tree-SHA512` and review ACKs that `github-merge.py` provides; merge queue needs organization-level features; larger process change than the problem needs.                        |
| **E. Merge `develop` into PR branches as the routine** | No id rewrite; GitHub "Update branch" does it in one click; GitHub signs the merge commit.                                                                                                                                                | Adds merge commits from `develop` to every PR; still reruns CI per update if done routinely. Accepted as a merge-time refresh, not as a routine.                                                                       |

### Consequences

- Positive: fewer CI runs, stable commit ids during review, fewer dismissed approvals, faster
  draining of the merge queue, and one contradiction between `AGENTS.md` and the skills removed.
- Negative: when a recommended rebase is skipped, a pull request may be merged on top of `develop` changes it was never tested with;
  semantic conflicts between concurrently merged pull requests surface on `develop` CI instead of on
  the pull request. The full commit graph is less linear; first-parent history is unchanged.
- Follow-up: the implementing issue updates the merge tool, its tests and README, the
  `open-pull-request` and `merge-pull-request` skills, and `AGENTS.md`; reconciles the disabled
  rulesets 94237 and 19245352; and measures force-pushes and CI time again.

## Date

2026-10-13

## References

- Implementing issue: #2507
  ([specification](../issues/open/2507-stop-requiring-pull-request-rebases/ISSUE.md))
- PR #2229 — stale merge-base guard in `github-merge.py`
- Issue #1743 — documentation-only CI fast path
- Issue #2022 — vendor and document the maintainer merge workflow
- [Adopt a self-hosted Hetzner runner for the container test job](20260926142648_adopt_self_hosted_hetzner_runner_for_container_tests.md)
- [PR #2320 review retrospective](../pr-reviews/pr-2320-review/review-retrospective.md)
- [GitHub Docs: About rulesets](https://docs.github.com/en/repositories/configuring-branches-and-merges-in-your-repository/managing-rulesets/about-rulesets)
