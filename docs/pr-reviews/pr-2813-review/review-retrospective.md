---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - docs/pr-reviews/pr-2813-review/PR-REVIEW.md
    - docs/templates/PR-REVIEW-RETROSPECTIVE.md
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
---

<!-- skill-link: process-pr-review -->

# PR Review Retrospective — PR #2813

## Purpose

A blameless record of why PR 2813 (axum-server 0.9 timeouts, issue #2776) drew thirteen review
findings in round 1, written at the maintainer's request. It complements
[PR-REVIEW.md](PR-REVIEW.md), which holds each finding and its resolution.

## Review Summary

| Metric | Value |
| ------ | ----- |
| Review rounds | 1 so far (1 human, 1 bot) |
| Audit findings | 13 (5 Copilot, 8 human) |
| Re-raised findings (`RE_RAISE_OF`) | 0 |
| Findings about the audit record itself | 0 (4 are about the evidence record: F7, F10, F11, and F6 in part) |
| Commits on the branch | 5 before review; the review batch adds 2 (fixes, audit and retrospective) |
| First review to last thread resolution | open: the threads are left for the reviewer to resolve |
| AI models used | Claude Sonnet 5.5 (author), unknown (reviewers) |

Counts come from `PR-REVIEW.md` and `git log torrust/develop..HEAD`.

## What Went Well

1. The spec, red tests and evidence folder existed before the code, so the human review could
   check the change against its own claims and found the weak claim (F6) quickly.
2. All Copilot findings were small and cheap to fix in one batch.

## Findings, Root Causes and Prevention

| ID | What the reviewer flagged | Root cause (why we got it wrong) | Type | What would have prevented it |
| -- | ------------------------- | -------------------------------- | ---- | ---------------------------- |
| F6 | The wrapper-versus-replacement HTTP/2 claim contradicted the removed code; result files were identical | I reported equal numbers as proof without asking why the wrapper should differ, and ran the "with wrapper" build once, before the final code | Misread evidence, unverified claim | Before removing code, state what the old code is expected to do, and compare the measurement with that prediction; a result that contradicts it needs an explanation first |
| F10 | AC4 ticked although HTTP/2 measured 10 s; "already 10 s today" never measured | I treated a requirement I had to interpret as met, and wrote a claim about `develop` from the wrapper build | Misread requirement, unmeasured claim | Leave an acceptance criterion open when the evidence only meets a reading of it; record only what a run showed |
| F11 | Red output paraphrased, no commands or toolchain, green run not recorded, stale Purpose | I summarized results by hand instead of saving command output | Process, documentation gap | Save the exact command and raw output at the moment of the run (a `red-run.txt` header with toolchain), as the `fix-bug` skill asks |
| F12 | HTTP/2 tests passed on any close, including an immediate protocol error; elapsed time ignored | The helper returned a duration I never asserted, and I never checked that a response arrived | Test gap | Review each test by asking what wrong behaviour still makes it pass; assert a lower bound as well as an upper one |
| F9 | Unrelated lockfile downgrades in the dependency commit | `cargo update -p axum-server --precise` resolved other edges; I read only the diff stat | Tooling | Diff `Cargo.lock` against the base after a bump and restore unrelated edges |
| F7, F8 | Commit id cited; EPIC citations without a basis | Cited line numbers and an id that exist only at one head, against the local convention of naming builds by subject and citing at a stated commit | Doc/convention mismatch | Cite by symbol and by commit subject by default |
| F13 | A working note left in a probe | Copied a scratch script into the repo without reading it again | Hygiene | Read every file added to evidence once before commit |
| F1 | Probe script leaves the tracker running on failure | Wrote it as a throwaway, then committed it as evidence | Tooling | Treat committed scripts as code: `set -eu` and a cleanup `trap` |
| F2 | Test can leak the server task on panic | Cleanup after the assertion instead of a guard | Test gap | Use drop guards for test servers |
| F3 | Private key committed without a note | Copied the existing fixtures without their context | Doc gap | Add a README when a key is committed, even a test one |
| F4 | 300 ms timeouts and 3 s deadline may be flaky | Chose speed over margin on a single local run | Test gap (flakiness) | Pick injected values with CI contention in mind (500 ms or more) |
| F5 | `from_tcp_with_timeouts` reads as taking timeouts | Kept the existing public name when its meaning changed | API naming | A doc note when a name's meaning shifts; rename only if the API is not public |

## Root Causes

1. Claims were stronger than the evidence (F6, F10): equal numbers were taken as confirmation, and
   an interpreted criterion was ticked.
2. Evidence was recorded by hand-summary instead of raw output with command and toolchain (F11).
3. Tests were reviewed for "passes when right" but not for "fails when wrong" (F12).
4. Mechanical diffs were not read after the tool ran (F9, F13).

## What We Learnt

1. Predict what the old code should do before measuring, and explain any mismatch.
2. Do not tick a criterion the evidence only meets under a reading you chose; ask.
3. Record raw output and the exact command at the moment of the run.
4. For each new test, name one wrong behaviour that still passes it.

## Improvements for Future Reviews

1. Add "state the expected behaviour of code you remove, then compare" to the `fix-bug` skill's
   evidence step — `.github/skills/dev/debugging/fix-bug/SKILL.md` — PROPOSED
2. Add a "diff `Cargo.lock` against the base after a dependency bump" line to the commit or
   dependency skills — `.github/skills/dev/` — PROPOSED

## Avoiding Overcorrection

The Copilot findings (F1 to F5) are ordinary hygiene; no new rule is justified by them alone. A
checker for lockfile drift is not justified by one occurrence.

## Evidence

- Audit record: `docs/pr-reviews/pr-2813-review/PR-REVIEW.md`
- Pull request: https://github.com/torrust/torrust-tracker/pull/2813
- Reviews: Copilot 5846895000, da2ce7 5846910258 (`gh-bot api repos/torrust/torrust-tracker/pulls/2813/reviews`)
