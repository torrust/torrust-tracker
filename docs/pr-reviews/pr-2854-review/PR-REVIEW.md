---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
---

<!-- skill-link: process-pr-review -->

# PR #2854 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/2854.

## Ownership

The PR author owns this tracked audit record. Reviewers, including repository review agents,
deliver findings through GitHub and have no repository-artifact obligation.

- Post-merge workflow approval: N/A

## Status Values

- Relationship: `ORIGINAL`, `RE_RAISE_OF:<FindingId>`
- Disposition: `FIXED`, `NO_ACTION`, `SUPERSEDED`, `FOLLOW_UP`
- Thread state: `OPEN`, `RESOLVED`, `NON_RESOLVABLE`, `SUPERSEDED`
- Severity: `Blocker`, `Major`, `Minor`, `Nit`, `Suggestion`
- Author class: `Copilot`, `Human`, `Unknown`
- Category: `link-integrity`, `formatting`, `metadata`, `testing`, `correctness`, `documentation`,
  `maintainability`, `security`, `other`

## Findings

This record was started late: the first two replies (F1/F2 below, the Copilot thread replies) were
posted before this audit existed, citing the branch commit `334decbf9` instead of its Conventional
Commit subject (finding F12 below). Late start logged in the Processing Log.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| R1 | review-finding:pr-2854-r1 | Copilot | Major | correctness | ORIGINAL | FIXED | RESOLVED |
| R2 | review-finding:pr-2854-r2 | Copilot | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F1 | review-finding:pr-2854-f1 | Human | Major | testing | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-2854-f2 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-2854-f3 | Human | Minor | testing | ORIGINAL | FIXED | RESOLVED |
| F4 | review-finding:pr-2854-f4 | Human | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F5 | review-finding:pr-2854-f5 | Human | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F6 | review-finding:pr-2854-f6 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F7 | review-finding:pr-2854-f7 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F8 | review-finding:pr-2854-f8 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F9 | review-finding:pr-2854-f9 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F10 | review-finding:pr-2854-f10 | Human | Nit | formatting | ORIGINAL | FIXED | RESOLVED |
| F11 | review-finding:pr-2854-f11 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F12 | review-finding:pr-2854-f12 | Human | Minor | maintainability | ORIGINAL | FIXED | RESOLVED |
| F13 | review-finding:pr-2854-f13 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F14 | review-finding:pr-2854-f14 | Human | Major | correctness | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### R1 - Apply the cap to the independent `tester` stage too

- PR number: 2854
- Source review ID: 5889364207
- Source URL: https://github.com/torrust/torrust-tracker/pull/2854#discussion_r4562132403
- Concern: `tester` derives directly from `rust:slim-trixie` and installs `cargo-nextest` with `cargo binstall`, which can fall back to compiling from source when no prebuilt binary is available. That compile inherited neither the default cap nor the build-argument override.
- Solution: added `ARG CARGO_BUILD_JOBS=4` and `ENV CARGO_BUILD_JOBS=${CARGO_BUILD_JOBS}` to the `tester` stage, before its tool install.
- Current-tree verification: `docker build -q --target tester --build-arg CARGO_BUILD_JOBS=2 --tag tmp-tester-check --file Containerfile .` then `docker run --rm --entrypoint printenv tmp-tester-check CARGO_BUILD_JOBS` printed `2`; repeated with no `--build-arg` printed `4` (the compiled-in default). Recorded in `manual-verification-evidence.md` V2/V3.
- Resolution reference: `fix(container): [#2853] address Copilot review: cap tester stage, trap measure.sh signals`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2854#discussion_r4562277725

### R2 - Cancel the background build when the measurement is interrupted

- PR number: 2854
- Source review ID: 5889364207
- Source URL: https://github.com/torrust/torrust-tracker/pull/2854#discussion_r4562132462
- Concern: sending SIGTERM/SIGINT to `measure.sh` stopped the memory watchdog without terminating its background `docker build` client, so an interrupted high-job-count run could keep consuming memory unsupervised.
- Solution: added an INT/TERM trap that sends `TERM` to `build_pid`, waits for it, and exits. The trap body is inlined into the `trap '...' INT TERM` string rather than a named function, because a named function here triggered a ShellCheck SC2329 false positive (the check lost track of the `trap`-only call once a later `status=$?` assignment was added) with no repository precedent for a disable comment.
- Current-tree verification: launched `measure.sh` in the background against a `docker` stub that only sleeps, sent it `SIGINT`, and confirmed the `measure.sh` process exited immediately and the message "measure.sh interrupted; cancelling build pid ..." was written to the summary file.
- Resolution reference: `fix(container): [#2853] address Copilot review: cap tester stage, trap measure.sh signals`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2854#discussion_r4562278024

### F1 - AC6 is marked DONE, but its condition is not met at this head

- PR number: 2854
- Source review ID: 5889429427
- Source URL: https://github.com/torrust/torrust-tracker/pull/2854#discussion_r4562180428
- Concern: `container.yaml` passes no `CARGO_BUILD_JOBS` build argument for the self-hosted runner, `report.md` deferred the runner comparison to after merge, the issue marked AC6 `DONE` anyway, and the PR description used `Closes #2853`, which would have auto-closed the issue with that criterion unmet. `build`/`build_shipped` recompile the whole workspace on every source change (`COPY . /build/src`), so the exposure is not limited to a rare cold build.
- Solution: set T6/AC6 back to `IN_PROGRESS`/`TODO`, added manual scenario M4 for the real self-hosted run at this PR's head, and changed the PR description from `Closes #2853` to `Related to #2853` until that run's result is recorded. The real run on `torrust-runner-01` for commit `334decbf9` (GitHub Actions run 43410572813) is used as the actual evidence instead of a deferred comparison.
- Current-tree verification: `gh-bot api repos/torrust/torrust-tracker/actions/runs/43410572813/jobs` confirms the job runs on `torrust-runner-01`; its duration is recorded in `manual-verification-evidence.md` V4 once it completes.
- Resolution reference: `fix(container): [#2853] address da2ce7 review: fix report arithmetic, evidence, docs nits`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2854#discussion_r4562418701

### F2 - The evidence file lacks the records `fix-bug` requires for an infeasible reproduction

- PR number: 2854
- Source review ID: 5889429427
- Source URL: https://github.com/torrust/torrust-tracker/pull/2854#discussion_r4562180440
- Concern: no reproduction section, no `Reproduced`/`Trigger only`/`Infeasible` classification, no kernel-OOM-log excerpt backing the 31-process/33-38 GB figures, and "Artifact under test" did not name the code state actually measured (the measurements predate the three implementation commits).
- Solution: added a "Reproduction (V1)" section classified `Infeasible`, with the attempted commands (none, by design), the blocking constraint (a third incident would again end every session on the shared machine), and the strongest substitute evidence (the capped measurements). Clarified that the exact kernel log lines are not reproduced verbatim because they were reported by the maintainer, not captured directly by this session. Renamed "Artifact under test" to name the two Conventional Commit subjects that bracket the measured state.
- Current-tree verification: read `manual-verification-evidence.md` after the edit; the reproduction section and the named commits are present.
- Resolution reference: `fix(container): [#2853] address da2ce7 review: fix report arithmetic, evidence, docs nits`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2854#discussion_r4562418154

### F3 - M2 is DONE, but its command (no `--build-arg`) was not run as written

- PR number: 2854
- Source review ID: 5889429427
- Source URL: https://github.com/torrust/torrust-tracker/pull/2854#discussion_r4562180451
- Concern: the recorded M2 run passed `--build-arg CARGO_BUILD_JOBS=4` explicitly, which does not prove the compiled-in default reaches Cargo with no flag at all, and CI never checks that either.
- Solution: ran `docker build --target chef` and `--target tester` with no `--build-arg` and confirmed `printenv CARGO_BUILD_JOBS` prints `4` in both; recorded the exact commands and output in `manual-verification-evidence.md` V2.
- Current-tree verification: commands re-run while fixing this finding: `docker build -q --target chef --tag tmp-chef-default --file Containerfile .` then `docker run --rm --entrypoint printenv tmp-chef-default CARGO_BUILD_JOBS` → `4`; same for `--target tester` → `4`.
- Resolution reference: `fix(container): [#2853] address da2ce7 review: fix report arithmetic, evidence, docs nits`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2854#discussion_r4562420225

### F4 - "A single `rustc` is about 0.9-1.6 GiB" is contradicted by the raw samples

- PR number: 2854
- Source review ID: 5889429427
- Source URL: https://github.com/torrust/torrust-tracker/pull/2854#discussion_r4562180457
- Concern: the per-process estimate was computed by dividing two independent column maxima (peak process count, peak summed RSS) that came from different sample rows, which is not a valid mean and contradicted the raw samples (true per-process figures are 2.09-2.60 GiB, not 0.9-1.6).
- Solution: fixed `measure.sh`'s summary `awk` to track the sample row with the highest summed RSS and report that same row's process count (`rustc_count_at_peak_rss`), instead of two independent maxima. Regenerated the three `results/*.summary.txt` files from the existing raw samples with the corrected formula, and rewrote `report.md`'s table and "Per-process memory" section with the consistent, joint figures.
- Current-tree verification: `awk` recomputation against `results/jobs-{4,8,16}.samples.tsv` gives (count, RSS) pairs (4, 10.38 GiB), (8, 19.79 GiB), (17, 35.49 GiB) — per-process 2.60/2.47/2.09 GiB, matching the new `report.md` text.
- Resolution reference: `fix(container): [#2853] address da2ce7 review: fix report arithmetic, evidence, docs nits`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2854#discussion_r4562419904

### F5 - "three at most in the release target" does not match the stage graph

- PR number: 2854
- Source review ID: 5889429427
- Source URL: https://github.com/torrust/torrust-tracker/pull/2854#discussion_r4562180464
- Concern: the release target's compiling stages all hang off `dependencies_thirdparty`, so at most two branches (`dependencies`→`build` and `dependencies_shipped_thirdparty`→`dependencies_shipped`→`build_shipped`) run concurrently, not three; the observed 1.5-3x peak process count was also not explained by stage overlap alone.
- Solution: rewrote `report.md`'s "Stage overlap" section (renamed from "BuildKit stage overlap") to state the DAG correctly (at most two concurrent compiling stages) and added an honest note that the peak-count anomaly at 4 jobs (12 > 2×4) is most likely a brief handoff-moment artifact of the 2-second sampling interval, not fully isolated, rather than asserting a specific unverified mechanism.
- Current-tree verification: read `Containerfile`'s `FROM` graph for the release target; every compiling stage traces back to `dependencies_thirdparty` through exactly one of the two branches.
- Resolution reference: `fix(container): [#2853] address da2ce7 review: fix report arithmetic, evidence, docs nits`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2854#discussion_r4562416772

### F6 - This risk contradicts itself and the `Containerfile`

- PR number: 2854
- Source review ID: 5889429427
- Source URL: https://github.com/torrust/torrust-tracker/pull/2854#discussion_r4562180469
- Concern: the risk bullet said the `ENV` "also reaches `cargo nextest run` in the `test` stages", then that those stages derive from `tester`, not `chef` — self-contradictory, since `tester` at that point had no `CARGO_BUILD_JOBS` of its own either.
- Solution: rewrote the bullet to state that `test`/`test_debug` derive from `tester` (not `chef`), that `tester` now declares its own `CARGO_BUILD_JOBS` (added for R1), but that `cargo nextest run` itself compiles nothing — the test binaries were already compiled by the preceding build stages — so the variable has no effect on it either way. A later pass (F13) corrected a residual inconsistency between this bullet, the Goal section, and the `Containerfile` comment about which stages the cap is declared in.
- Current-tree verification: read the current risk bullet and the `Containerfile`'s `tester` stage; the statement matches both.
- Resolution reference: `fix(container): [#2853] fix tester/chef wording contradiction, add audit record`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2854#discussion_r4562417156

### F7 - A changed value invalidates the cache from the first `RUN` of `chef` onward, and the docs do not say so

- PR number: 2854
- Source review ID: 5889429427
- Source URL: https://github.com/torrust/torrust-tracker/pull/2854#discussion_r4562180476
- Concern: the `ARG`/`ENV` enter the cache key of every later `RUN` in stages that derive from `chef` (and now `tester`), so a value other than the cached one re-runs `apt-get`, the tool installs, and every cook; `docs/containers.md` recommended raising the value without this caveat.
- Solution: added a paragraph to `docs/containers.md`'s "Memory and CPU Use" section stating the cache-invalidation effect for both the `chef`- and `tester`-derived stages and recommending picking one value per machine.
- Current-tree verification: read the added paragraph in `docs/containers.md`.
- Resolution reference: `fix(container): [#2853] address da2ce7 review: fix report arithmetic, evidence, docs nits`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2854#discussion_r4562419381

### F8 - The progress-log times do not match the sample epochs

- PR number: 2854
- Source review ID: 5889429427
- Source URL: https://github.com/torrust/torrust-tracker/pull/2854#discussion_r4562180489
- Concern: a progress-log entry's timestamp did not match the raw sample epochs it described, a later entry's timestamp preceded the commit that added it, and `last-updated-utc` was stale.
- Solution: rewrote the Progress Log with timestamps taken from the sample epochs (`results/*.samples.tsv`) and `git log --format=%cI` for each commit, in chronological order, and updated `last-updated-utc`.
- Current-tree verification: `git log --format='%H %ad %s' --date=iso-strict-local` against the entries' cited commits; times match.
- Resolution reference: `fix(container): [#2853] address da2ce7 review: fix report arithmetic, evidence, docs nits`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2854#discussion_r4562420505

### F9 - The `docker-build.sh` example works only from the repository root

- PR number: 2854
- Source review ID: 5889429427
- Source URL: https://github.com/torrust/torrust-tracker/pull/2854#discussion_r4562180501
- Concern: `docker-build.sh`'s own comment example (`CARGO_BUILD_JOBS=8 ./docker-build.sh`) only works from the script's own directory, while `docs/containers.md` has the correct relative form from the repository root.
- Solution: changed the script's comment to the working form, `./contrib/dev-tools/containers/docker-build.sh`.
- Current-tree verification: read the updated comment in `contrib/dev-tools/containers/docker-build.sh`.
- Resolution reference: `fix(container): [#2853] address da2ce7 review: fix report arithmetic, evidence, docs nits`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2854#discussion_r4562417840

### F10 - New concrete Markdown documents use lowercase filenames

- PR number: 2854
- Source review ID: 5889429427
- Source URL: https://github.com/torrust/torrust-tracker/pull/2854#discussion_r4562180509
- Concern: `write-markdown-docs` requires lowercase filenames for new concrete documents; `REPORT.md` is not among the uppercase exceptions (the `sccache-docker` experiment's `REPORT.md` predates the rule).
- Solution: renamed `REPORT.md` to `report.md` with `git mv`, and updated every reference (`docs/containers.md`, `docs/issues/open/2853-container-build-jobs-limit/ISSUE.md`).
- Current-tree verification: `grep -rn "REPORT.md" docs/issues/open/2853-container-build-jobs-limit docs/containers.md contrib/dev-tools/experiments/container-build-jobs` returns nothing live; `linter all` passes (covers markdown link checks).
- Resolution reference: `fix(container): [#2853] address da2ce7 review: fix report arithmetic, evidence, docs nits`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2854#discussion_r4562417517

### F11 - Heading-case cross-reference mismatch

- PR number: 2854
- Source review ID: 5889429427
- Source URL: https://github.com/torrust/torrust-tracker/pull/2854#discussion_r4562180513
- Concern: the `Containerfile` comment and the issue spec said "Memory and CPU use" while `docs/containers.md`'s heading is "Memory and CPU Use".
- Solution: changed every cross-reference (`Containerfile`, `docker-build.sh`, `ISSUE.md`) to the heading's own capitalization.
- Current-tree verification: `grep -rn "Memory and CPU" Containerfile docs/containers.md docs/issues/open/2853-container-build-jobs-limit/ISSUE.md contrib/dev-tools/containers/docker-build.sh` shows one spelling.
- Resolution reference: `fix(container): [#2853] address da2ce7 review: fix report arithmetic, evidence, docs nits`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2854#discussion_r4562419639

### F12 - Two review threads were answered with no review audit record, citing a branch commit id

- PR number: 2854
- Source review ID: 5889641059
- Source URL: https://github.com/torrust/torrust-tracker/pull/2854#discussion_r4562346507
- Concern: `process-pr-review` requires creating `docs/pr-reviews/pr-2854-review/PR-REVIEW.md` before the first reply; the R1/R2 replies were posted without it and cited the branch commit id `334decbf9` instead of its Conventional Commit subject (a rebase would orphan that id); F1-F11 had neither a reply nor a fix yet at the time this finding was raised.
- Solution: created this audit record (late; logged below), covering R1, R2 and F1-F11 alongside this review's own F12/F13, with every `Resolution reference` citing the commit subject. The earlier two replies (to R1/R2) remain on GitHub with the commit id, since GitHub comments cannot be edited by this identity's normal flow without risk of losing the thread association; this audit record is the corrected, durable reference going forward.
- Current-tree verification: this file exists at `docs/pr-reviews/pr-2854-review/PR-REVIEW.md`.
- Resolution reference: `fix(container): [#2853] fix tester/chef wording contradiction, add audit record`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2854#discussion_r4562418412

### F13 - The `tester` cap contradicts the spec, the description and the `chef` comment

- PR number: 2854
- Source review ID: 5889641059
- Source URL: https://github.com/torrust/torrust-tracker/pull/2854#discussion_r4562346523
- Concern: `ISSUE.md`'s Goal/Scope section and its T3 row said the cap lives "in the `chef` base stage" / "stages outside `chef` need no change", the PR description said "the `tester` stage only runs prebuilt test archives and needs nothing", and the `Containerfile`'s `chef`-stage comment said every Rust stage "derives from this one" — all written before `tester` got its own `ARG`/`ENV` for R1, and not updated afterward. The earlier reply's `printenv` check on `tester` was also not yet recorded in `manual-verification-evidence.md` at the time this finding was raised (it was added in the same push that fixed F1-F11, before this finding was read).
- Solution: rewrote the `Containerfile`'s `chef`-stage comment, `ISSUE.md`'s Goal/Scope bullet and its T3 row to name both `chef` and `tester` as the two base stages that declare `CARGO_BUILD_JOBS`. Rewrote the PR description to the same effect. Confirmed the `tester` `printenv` check (with and without `--build-arg`) is recorded in `manual-verification-evidence.md` V2/V3.
- Current-tree verification: `grep -n "chef\|tester" docs/issues/open/2853-container-build-jobs-limit/ISSUE.md` and the `Containerfile` comment show one consistent statement; `manual-verification-evidence.md` V2/V3 include the `tester` commands and their `4`/`2` output.
- Resolution reference: `fix(container): [#2853] fix tester/chef wording contradiction, add audit record`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2854#discussion_r4562419076

### F14 - The three committed run summaries were rewritten after the runs, and one corrected figure is itself wrong

- PR number: 2854
- Source review ID: 5889901103
- Source URL: https://github.com/torrust/torrust-tracker/pull/2854#discussion_r4562551182
- Concern: the F4 fix overwrote `results/jobs-{4,8,16}.summary.txt` with the output of a differently-formatted `measure.sh` that none of the three runs executed, with no note that the files were regenerated; the 16-jobs line's `peak_used_delta_gib=32,9` matches neither the run's own output nor a correct recomputation from the unchanged `jobs-16.samples.tsv` (33.0), and `report.md:41` copied the wrong value.
- Solution: restored the three summary files to exactly what each run printed, read from the commit that first added them (`git show a3aaab735:.../jobs-<N>.summary.txt`), rather than a regenerated format. Corrected `report.md`'s 16-jobs "Peak `MemAvailable` drop" cell to 33.0 GiB. Added a sentence to `report.md`'s "Method" section stating which columns are copied unmodified from the run output (`MemAvailable` drop, lowest `MemAvailable`) and which are recomputed from the raw samples (`rustc` at peak RSS, peak `rustc` RSS), so a future reader does not need to infer it. The wrong 32.9 came from the one-off regeneration script building the baseline from an already-rounded GiB figure instead of the exact kB baseline on the summary's first line — a precision-loss bug distinct from the F4 arithmetic fix itself (which remains correct, since it only changed which sample row's count/RSS pair is reported, not this baseline/minimum subtraction).
- Current-tree verification: `git diff a3aaab735 -- contrib/dev-tools/experiments/container-build-jobs/results/` against the current tree shows no difference in the three summary files; `awk` over `jobs-16.samples.tsv` with the exact baseline `49323728` kB from the file's own first line gives `(49323728-14758164)/1048576 = 32.96` GiB, which `report.md` now states as 33.0.
- Resolution reference: `fix(container): [#2853] restore raw run summaries, correct the 16-jobs delta to 33.0`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2854#discussion_r4562573939

## Processing Log

- 2026-11-13 11:05 UTC - Started audit late (after R1, R2, F1-F11 replies/fixes were already in progress or posted); backfilled all entries from the current tree and the GitHub review/comment history.
- 2026-11-13 11:05 UTC - Replied on all resolvable threads (R1, R2, F1-F11, F12, F13) and resolved them via GraphQL `resolveReviewThread`.
- 2026-11-13 11:10 UTC - Added F14 (raw-summary overwrite, wrong corrected figure), fixed it, replied, and resolved its thread.

## Completion Rules

- Re-derive the reply claim against the current tree before replying or resolving a thread.
- Reply on every resolvable thread before resolving it.
- Cite a fix by its unique Conventional Commit subject or durable reply URL, never by a branch SHA
  that can change after a rebase.
- Refresh review threads using GraphQL and confirm that no unresolved actionable thread remains.
