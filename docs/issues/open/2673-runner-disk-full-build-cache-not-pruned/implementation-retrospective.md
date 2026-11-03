---
semantic-links:
  skill-links:
    - write-markdown-docs
  related-artifacts:
    - docs/templates/IMPLEMENTATION-RETROSPECTIVE.md
    - docs/issues/open/2673-runner-disk-full-build-cache-not-pruned/ISSUE.md
---

# Implementation Retrospective — Runner Disk Full, Build Cache Not Pruned

## Purpose

Record evidence-based process improvements discovered while implementing issue 2673. This is a blameless review of the implementation approach; it does not replace acceptance-criteria verification, which is recorded in [ISSUE.md](ISSUE.md) and [manual-verification-evidence.md](manual-verification-evidence.md).

## Outcome

On 2026-10-27 the disk of the self-hosted runner `torrust-runner-01` reached 100% and the daily prune freed 0 B for three days. The cause is orphaned build state held by BuildKit inside `dockerd`, which counted all of the build cache as in use until `docker.service` was restarted (V2, V4). `docker-ci-prune.sh` now restarts Docker when the disk is still at or above 70% after the prune and no job runs, prunes again, and exits non-zero at or above 85% (V5, V6). The trigger that creates the orphaned state is still unproven and is deferred (`deferred-proposals.md`, R1).

Seven scheduled runs (2026-10-28 to 2026-11-03) all succeeded with the disk at 51% to 53% after the prune, and none needed the Docker restart (V6, [report of 2026-11-03](../../../self-hosted-runner-reports/20261103-171940-runner-report.md)).

## What Went Well

1. The incident was reproduced before it was fixed: the full disk and the 0 B prune were recorded (V1), so the fix could be judged against a real symptom.
2. The wrong hypothesis was rejected with evidence, not argued away: removing the four containers changed nothing, and the 117 pinned build records pointed at the real cause (V2).
3. The script change was made red first, with mutation checks showing that each guard test fails when the guard it protects is removed (V5).
4. The restart path, which the real 70% threshold may not reach for days, was exercised on the real idle runner with a lowered threshold before the scheduled runs (V6).
5. Runner changes were made only with the maintainer's approval and only while the runner was idle, and the unit it replaced was kept on the host.
6. A reviewer finding that the maintainer declined (a job can arrive between the idle check and the restart) was kept as a documented limitation and a design, not silently dropped or implemented without approval.

## What Changed During Implementation

1. **The first hypothesis was wrong.** The first health report named four orphaned containers as the cause. Removing them changed nothing; the second report corrects it, and the first is not edited because reports are immutable.
2. **The Docker restart, not the prune, freed the disk.** The disk went from 100% to 42% only after `docker.service` was restarted (V4).
3. **A prune result was misread.** The first report read the `Total reclaimed space` lines, which belong to the container, image and volume prunes, as the build-cache result. Only the lines starting `Total:` are the build-cache prune (V1).
4. **A review sat unprocessed.** The Copilot review of the health-report pull request arrived at 09:46 UTC and was not processed until the maintainer reported unresolved threads at 12:41 UTC.
5. **Commits went in before the gate passed.** On two occasions a loosely chained `pre-commit && git commit` committed although a check had failed.
6. **GB and GiB were mixed.** `df` and `du` print binary units and Docker prints decimal units; documents labelled both as GB until a follow-up pull request relabelled the `df` and `du` figures.
7. **The leak was not reproduced.** A cancelled-build experiment on a local Docker 28.3.3 did not reproduce the pinned state, so the trigger stayed a correlation (V2).

## Root Cause

1. The first report was written from one snapshot and a plausible correlation, and the specification inherited its cause without a step that tried to falsify it. Recording a hypothesis in a spec makes it look established.
2. The prune log lines have similar names and the two kinds were not distinguished in the report template or the runbook, so a reader had to know which line meant which prune.
3. Nothing prompted the agent to look for new reviews after each push, and the pre-commit result was not tied mechanically to the commit command.
4. No convention stated which unit a figure uses, and the tools in play disagree.

## Improvements for Future Work

1. In specifications and health reports, label a cause as a hypothesis until a test has tried to disprove it, and require the next report to state corrections.
2. Name the build-cache lines (`Total:`) and the other prune lines in the health-report template or the `check-runner-health` skill, so the right line is read.
3. After every push, fetch the review threads of the pull requests being worked on, and treat a new review as part of the task, not as an interruption.
4. Gate the commit on the pre-commit exit status (`rc=$?`, commit only on 0) and re-run the gate after every edit, including a one-line one.
5. Label each figure with its source and unit (GiB for `df` and `du`, GB for Docker), and say so in documents that mix them.
6. Only if the problem recurs: reproduce the leak trigger on the runner's Docker stack (R1) and consider the lock and job-started hook (R5), as recorded in `deferred-proposals.md`.

## Avoiding Overcorrection

The evidence does not justify new alerting or scheduled workflows: the maintainer decided to wait and implement the recorded proposals only if a broken runner becomes a recurring problem. It also does not justify a general rule against recording hypotheses; the failure was presenting one as established.

## Evidence

- [ISSUE.md](ISSUE.md)
- [manual-verification-evidence.md](manual-verification-evidence.md): V1 reproduction, V2 diagnosis, V4 recovery, V5 tests, V6 deployment and scheduled runs
- [deferred-proposals.md](deferred-proposals.md)
- [Health report of 2026-10-27 09:12 UTC](../../../self-hosted-runner-reports/20261027-091200-runner-report.md) and [of 11:43 UTC](../../../self-hosted-runner-reports/20261027-114300-runner-report.md)
- Pull request review audits: `docs/pr-reviews/pr-2672-review/PR-REVIEW.md` and `docs/pr-reviews/pr-2675-review/PR-REVIEW.md`
