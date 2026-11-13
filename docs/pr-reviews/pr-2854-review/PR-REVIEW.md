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

This record was started late: the first two replies (R1/R2 below, the Copilot thread replies) were
posted before this audit existed, citing the branch commit by its short hash in their own GitHub
comment text instead of the Conventional Commit subject `fix(container): [#2853] address Copilot
review: cap tester stage, trap measure.sh signals` (finding F12 below). Late start logged in the
Processing Log.

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
| R3 | review-finding:pr-2854-r3 | Copilot | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| R4 | review-finding:pr-2854-r4 | Copilot | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F15 | review-finding:pr-2854-f15 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F16 | review-finding:pr-2854-f16 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F17 | review-finding:pr-2854-f17 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F19 | review-finding:pr-2854-f19 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F20 | review-finding:pr-2854-f20 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F21 | review-finding:pr-2854-f21 | Copilot | Major | correctness | ORIGINAL | FIXED | RESOLVED |
| F22 | review-finding:pr-2854-f22 | Copilot | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F23 | review-finding:pr-2854-f23 | Copilot | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F24 | review-finding:pr-2854-f24 | Copilot | Nit | formatting | ORIGINAL | FIXED | RESOLVED |
| F25 | review-finding:pr-2854-f25 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F26 | review-finding:pr-2854-f26 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F27 | review-finding:pr-2854-f27 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F28 | review-finding:pr-2854-f28 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |

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
- Solution: set T6/AC6 back to `IN_PROGRESS`/`TODO`, added manual scenario M4 for a real self-hosted run of the PR head, and changed the PR description from `Closes #2853` to `Related to #2853` until that run's result is recorded. The build step time on the runner was measured later and is recorded in `manual-verification-evidence.md` V4 (F15 below corrects an over-claim this entry's first fix made about one such run).
- Current-tree verification: `manual-verification-evidence.md` V4 holds the six step times, `ISSUE.md`'s M4 row is DONE, and AC6 is TODO until the maintainer decides whether the measured slowdown is acceptable.
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
- Current-tree verification: read the current risk bullet and the `Containerfile`'s `tester` stage; the statement matches both. `git log -p -S'test/test_debug' -- docs/issues/open/2853-container-build-jobs-limit/ISSUE.md` shows the rewritten bullet's text first appears in the commit below, not in the later F13 commit (which touched only the Goal/Scope bullet and the T3 row, per F20).
- Resolution reference: `fix(container): [#2853] address da2ce7 review: fix report arithmetic, evidence, docs nits`
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
- Solution: the first attempt, in `fix(container): [#2853] address da2ce7 review: fix report arithmetic, evidence, docs nits`, was itself incomplete: the 08:50 entry still claimed the 4-job run started then although the samples began at 08:31, and `F19` below caught that this disposition had been marked `FIXED` before that was true. Rewrote the Progress Log again, this time stamping every entry at or after its own commit's `git log --format=%cI` time (never claiming an event before it happened) and matching each measurement entry's stated window to its `results/*.samples.tsv` epochs, and updated `last-updated-utc` to match.
- Current-tree verification: `git log --format='%H %ad %s' --date=iso-strict-local` against every entry's cited commit subject, and each measurement window against its `samples.tsv` first/last epoch; both match.
- Resolution reference: `fix(container): [#2853] address review round 2: locale, PID anchor, stage overlap, audit accuracy`
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
- Concern: `process-pr-review` requires creating `docs/pr-reviews/pr-2854-review/PR-REVIEW.md` before the first reply; the R1/R2 replies were posted without it and cited the branch commit by its short hash instead of its Conventional Commit subject (a rebase would orphan that hash); F1-F11 had neither a reply nor a fix yet at the time this finding was raised.
- Solution: created this audit record (late; logged below), covering R1, R2 and F1-F11 alongside this review's own F12/F13, with every `Resolution reference` citing the commit subject. The earlier two replies (to R1/R2) showed the hash in their own GitHub comment text until they were edited on 2026-11-13 at 13:16 UTC to cite the commit subject (see the Processing Log).
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
- Solution: restored the three summary files to exactly what each run printed, read from the commit `docs(issues): [#2853] spec and measured evidence for the container build jobs limit`, which first added them, rather than a regenerated format. Corrected `report.md`'s 16-jobs "Peak `MemAvailable` drop" cell to 33.0 GiB. Added a sentence to `report.md`'s "Method" section stating which columns are copied unmodified from the run output and which are recomputed from the raw samples, so a future reader does not need to infer it. The wrong 32.9 came from the one-off regeneration script building the baseline from an already-rounded GiB figure instead of the exact kB baseline on the summary's first line — a precision-loss bug distinct from the F4 arithmetic fix itself (which remains correct, since it only changed which sample row's count/RSS pair is reported, not this baseline/minimum subtraction).
- Current-tree verification: a diff of the three restored summary files against that same commit's versions shows no difference; `awk` over `jobs-16.samples.tsv` with the exact baseline `49323728` kB from the file's own first line gives `(49323728-14758164)/1048576 = 32.96` GiB, which `report.md` now states as 33.0.
- Resolution reference: `fix(container): [#2853] restore raw run summaries, correct the 16-jobs delta to 33.0`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2854#discussion_r4562573939

### R3 - Include the independent tester compilation in the overlap bound

- PR number: 2854
- Source review ID: 5889779944
- Source URL: https://github.com/torrust/torrust-tracker/pull/2854#discussion_r4562454192
- Concern: `tester` does not descend from `dependencies_thirdparty`, so its independent `cargo-nextest` source-install fallback can run concurrently with both workspace branches; "at most two" is not a bound for the whole release target.
- Solution: reworded the Stage overlap section to scope the two-stage bound to the two workspace-compiling branches explicitly, and added that `tester`'s own compile path is independent, concurrent with both, and capped on its own by `tester`'s `CARGO_BUILD_JOBS` (R1) rather than by this count. Noted that the measured release target's `tester` install did not fall back to a source compile.
- Current-tree verification: read the `Containerfile`'s `tester` stage (no `FROM dependencies_thirdparty` or `FROM chef` relation) against the reworded section.
- Resolution reference: `fix(container): [#2853] address review round 2: locale, PID anchor, stage overlap, audit accuracy`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2854#discussion_r4562689626

### R4 - Distinguish a new cache key from discarding earlier cached builds

- PR number: 2854
- Source review ID: 5889779944
- Source URL: https://github.com/torrust/torrust-tracker/pull/2854#discussion_r4562454312
- Concern: changing `CARGO_BUILD_JOBS` selects a different cache key; it does not delete the layers cached for a previous value, so a 4→8→4 sequence can reuse the original 4-job layers if nothing else changed and the cache was not pruned. The warning in `docs/containers.md` did not say this.
- Solution: reworded the paragraph to say the value change misses the next layer's cache (a different key), not that it discards other values' cached layers, and that switching back to a value built before can still hit its own retained cache.
- Current-tree verification: read the reworded paragraph in `docs/containers.md`.
- Resolution reference: `fix(container): [#2853] address review round 2: locale, PID anchor, stage overlap, audit accuracy`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2854#discussion_r4562689333

### F15 - "Measured directly on `torrust-runner-01`" introduces a run with no measurement, and it is not at this head

- PR number: 2854
- Source review ID: 5889901103
- Source URL: https://github.com/torrust/torrust-tracker/pull/2854#discussion_r4562551196
- Concern: the runner bullet named a specific run and head while recording no actual duration, contradicting M4/T6/the progress log (which all say the run is "at this head" and pending), and its "not materially different from before this change" phrasing contradicted the next sentence about every-run recompilation.
- Solution: rewrote the bullet to state the before/after job count (8 uncapped → 4) and memory (about 20 GiB → about 10 GiB) explicitly, dropped the specific run/head reference, and stated plainly that the actual job duration at the new default is pending, pointing to M4/V4, with a note to name any future run by its head's commit subject rather than a branch id.
- Current-tree verification: read the bullet in `report.md`'s "Effect on the runners"; it now states the measured step times, which were added after this finding was resolved, and points to evidence section V4 for the run ids.
- Resolution reference: `fix(container): [#2853] address review round 2: locale, PID anchor, stage overlap, audit accuracy`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2854#discussion_r4562691197

### F16 - The skill's commit-id check prints four branch commit ids

- PR number: 2854
- Source review ID: 5889901103
- Source URL: https://github.com/torrust/torrust-tracker/pull/2854#discussion_r4562551204
- Concern: `ISSUE.md`'s progress log, `report.md` and this audit record's own entries cited branch commit ids instead of Conventional Commit subjects, which a rebase would orphan.
- Solution: rewrote `ISSUE.md`'s progress log and this audit record's R1/R2 narrative and `Resolution reference` fields to cite every commit by subject, and removed the one remaining run-id reference from `report.md` (also fixed by F15).
- Current-tree verification: searched `report.md`, `ISSUE.md` and this record for a backtick-quoted 7-10 character hex string; the only remaining hits are commit subjects (which are not hex) and one kB figure in F14's narrative, no bare branch commit id.
- Resolution reference: `fix(container): [#2853] address review round 2: locale, PID anchor, stage overlap, audit accuracy`; `fix(container): [#2853] address review round 3: audit timestamps, report accuracy`; `docs(pr-reviews): [#2853] correct the audit log and drop quoted commit hashes`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2854#discussion_r4562689932

### F17 - Small report inaccuracies left by the rewrite

- PR number: 2854
- Source review ID: 5889901103
- Source URL: https://github.com/torrust/torrust-tracker/pull/2854#discussion_r4562551210
- Concern: 2.09 GiB fell outside the stated 2.2-2.6 GiB/job range; the report said the earlier `measure.sh` "divided" the two maxima when it only printed them; the stage-handoff explanation for the 12-process sample was physically implausible (a step ends only when its process tree exits); and a sentence was duplicated.
- Solution: corrected the range to 2.09-2.60 GiB/job; reworded the `measure.sh` sentence to say it only printed the two independent maxima and that the division happened in this report's prose; replaced the stage-handoff explanation with a statement that the cause was not isolated and that a handoff alone does not explain it; removed the duplicated sentence (same fix as F23/C3).
- Current-tree verification: read the corrected "Per-process memory" and "Results" sections and the "Method" bullet; the duplicated sentence is gone.
- Resolution reference: `fix(container): [#2853] address review round 2: locale, PID anchor, stage overlap, audit accuracy`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2854#discussion_r4562691500

### F19 - Several audit entries state more than the tree and the forge show

- PR number: 2854
- Source review ID: 5889901103
- Source URL: https://github.com/torrust/torrust-tracker/pull/2854#discussion_r4562551215
- Concern: this record's own Processing Log entries were stamped 11:05 UTC although they sit in commits made at 10:52 and 10:59; F8's row recorded `FIXED` while `ISSUE.md`'s 08:50 entry was still wrong and the 10:55 entries sat in a 10:46 commit; line 34 called the two early replies "F1/F2" where it meant "R1/R2"; and the claim that editing the R1/R2 replies would lose their thread association is false (editing a comment keeps its thread).
- Solution: corrected "F1/F2" to "R1/R2"; rewrote this record's own Processing Log so every entry is stamped at or after its own commit's time, never claiming an event before it happened; corrected F8's entry to state its first attempt was itself incomplete, with a corrected resolution reference; removed the false claim about losing thread association, describing the branch-commit-id text as a historical fact about what the early replies said rather than a constraint on editing them.
- Current-tree verification: read the corrected Processing Log and F8 entries; `git log --format=%cI` against each cited commit.
- Resolution reference: `fix(container): [#2853] address review round 2: locale, PID anchor, stage overlap, audit accuracy`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2854#discussion_r4562691848

### F20 - F6's resolution reference now names a commit that did not change the F6 bullet

- PR number: 2854
- Source review ID: 5889901103
- Source URL: https://github.com/torrust/torrust-tracker/pull/2854#discussion_r4562551218
- Concern: F6's `Resolution reference` cited the F13 commit, which only changed `ISSUE.md`'s Goal/Scope bullet and T3 row, not the risk bullet F6 is about; the two-subject form in an earlier draft of this entry had already failed the validator for an unrelated reason (a trailing `(F13)` after the closing backtick).
- Solution: changed F6's `Resolution reference` to the commit that actually introduced the rewritten risk bullet's text.
- Current-tree verification: `git log -p -- docs/issues/open/2853-container-build-jobs-limit/ISSUE.md` shows the risk bullet's current wording first appearing in that commit.
- Resolution reference: `fix(container): [#2853] address review round 2: locale, PID anchor, stage overlap, audit accuracy`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2854#discussion_r4562692163

### F21 - `grep -v "$$"` is not anchored to the PID column

- PR number: 2854
- Source review ID: 5890001923
- Source URL: https://github.com/torrust/torrust-tracker/pull/2854#discussion_r4562625585
- Concern: `pgrep -af ... | grep -v "$$"` filters by substring over the whole line, so a running `docker build` whose command line happens to contain the same digits as this script's PID would be filtered out by accident, producing a false negative and allowing unsafe concurrent builds.
- Solution: replaced the `grep -v "$$"` filter with `awk -v self="$$" '$1 != self'`, which compares only the first (PID) field of each `pgrep -a` line.
- Current-tree verification: confirmed `pgrep -af 'docker...build'` does not match its own invocation on this machine (no self-match to exclude in the first place), and that the new `awk` filter still excludes a line by exact PID.
- Resolution reference: `fix(container): [#2853] address review round 2: locale, PID anchor, stage overlap, audit accuracy`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2854#discussion_r4562689029

### F22 - The numeric formatting is locale-dependent

- PR number: 2854
- Source review ID: 5890001923
- Source URL: https://github.com/torrust/torrust-tracker/pull/2854#discussion_r4562625682
- Concern: `measure.sh`'s `awk`/`printf` output used this machine's locale (comma decimal separators, visible in the committed `results/*.summary.txt`), which is not machine-parseable or reproducible across environments.
- Solution: added `export LC_ALL=C` near the top of `measure.sh`, before any numeric output. The already-committed summary files keep their comma formatting, since they are raw run output (F14) and were produced before this fix.
- Current-tree verification: read the added `export LC_ALL=C` line.
- Resolution reference: `fix(container): [#2853] address review round 2: locale, PID anchor, stage overlap, audit accuracy`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2854#discussion_r4562690233

### F23 - Duplicated sentence in `report.md`

- PR number: 2854
- Source review ID: 5890001923
- Source URL: https://github.com/torrust/torrust-tracker/pull/2854#discussion_r4562625746
- Concern: "It refuses to start while another container build runs." appeared twice in the "Method" section.
- Solution: same fix as F17; removed the duplicate.
- Current-tree verification: read the "Method" section; the sentence appears once.
- Resolution reference: `fix(container): [#2853] address review round 2: locale, PID anchor, stage overlap, audit accuracy`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2854#discussion_r4562690562

### F24 - Unquoted YAML timestamp

- PR number: 2854
- Source review ID: 5890001923
- Source URL: https://github.com/torrust/torrust-tracker/pull/2854#discussion_r4562625797
- Concern: `manual-verification-evidence.md`'s `last-updated-utc` was an unquoted YAML timestamp-like scalar, inconsistent with the other documents in this PR, which quote it.
- Solution: quoted the value.
- Current-tree verification: read the frontmatter; `last-updated-utc` is now a quoted string.
- Resolution reference: `fix(container): [#2853] address review round 2: locale, PID anchor, stage overlap, audit accuracy`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2854#discussion_r4562690930

### F25 - The new Method sentence says peak `rustc` RSS is "not in the summary files", but each summary carries it

- PR number: 2854
- Source review ID: 5890103485
- Reviewer finding ID: F21 (da2ce7 reused this id for a new finding; collides with this record's existing Copilot F21, so reassigned per the skill's collision rule)
- Source URL: https://github.com/torrust/torrust-tracker/pull/2854#discussion_r4562691809
- Concern: the restored summaries print `peak_rustc_rss_gib` directly, and the maximum of that column is by definition the same number whichever row it comes from, so only "`rustc` at peak RSS" (the count) is actually new; the clause calling the earlier script's two independent maxima "wrong" also overstated — printing them was fine, dividing one by the other was the error.
- Solution: reworded the "Method" bullet to say only the count column is recomputed, and that printing the two independent maxima was fine; only reading them from different samples (not printing them) was the error.
- Current-tree verification: read the reworded "Method" section; it no longer claims the RSS figure is absent from the summaries.
- Resolution reference: `fix(container): [#2853] address review round 3: audit timestamps, report accuracy`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2854#discussion_r4562838627

### F26 - Three rows record FIXED with verification lines the tree contradicts

- PR number: 2854
- Source review ID: 5890286739
- Reviewer finding ID: F22 (da2ce7 reused this id for a new finding; collides with this record's existing Copilot F22, so reassigned per the skill's collision rule)
- Source URL: https://github.com/torrust/torrust-tracker/pull/2854#discussion_r4562806786
- Concern: F16's row claimed no bare branch commit id remained, but two branch commit hashes still appeared in F12's and F14's prose; F8's row claimed the 08:50 progress-log entry no longer said the 4-job run started then, but it still did; F19's row claimed this record's own Processing Log was restamped, but its two 11:05 UTC entries, which sit in a commit made at 10:52 describing replies posted at 10:54, were unchanged.
- Solution: replaced the two remaining bare hashes with a commit subject and a plain description; fixed `ISSUE.md`'s 08:50 entry to no longer claim the measurement started then; left the two 11:05 UTC entries as originally written (their content was accurate) and appended a correction entry below, rather than rewriting them in place, stating the actual commit and reply times. Also folded in the collision this very finding caused: da2ce7 reused F21-F23 for new findings that collide with this record's existing Copilot F21-F23, reassigned here as F25-F27, with the reviewer's original id recorded in each.
- Current-tree verification: a backtick-quoted 7-10 character hex string search of `ISSUE.md`, `report.md` and this record finds no remaining bare branch commit id; `ISSUE.md`'s 08:50 entry read directly; the Processing Log's two original entries and the appended correction read directly.
- Resolution reference: `fix(container): [#2853] address review round 3: audit timestamps, report accuracy`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2854#discussion_r4562838932

### F27 - The new `tester` sentences overstate what the report shows

- PR number: 2854
- Source review ID: 5890286739
- Reviewer finding ID: F23 (da2ce7 reused this id for a new finding; collides with this record's existing Copilot F23, so reassigned per the skill's collision rule)
- Source URL: https://github.com/torrust/torrust-tracker/pull/2854#discussion_r4562806800
- Concern: `cargo binstall` fetches prebuilt binaries from the crate's release artifacts or a binary mirror, not crates.io; no build log was kept for the measured runs, so "does not fall back to a source compile" was not shown by anything in `results/`; the cross-reference to "Effect on the runners" leads to a section that does not mention `tester`.
- Solution: reworded the Stage overlap section to drop the crates.io claim, state that whether `tester` used a prebuilt download or fell back to a source compile is assumed from the `Containerfile` alone (no build log kept), and removed the cross-reference to "Effect on the runners".
- Current-tree verification: read the reworded paragraph in `report.md`'s "Stage overlap" section.
- Resolution reference: `fix(container): [#2853] address review round 3: audit timestamps, report accuracy`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2854#discussion_r4562839358

### F28 - The appended correction repeats the timing problem it corrects, and one sentence does not parse

- PR number: 2854
- Source review ID: 5890524264
- Reviewer finding ID: F25 (collides with this record's F25, so reassigned per the skill's collision rule)
- Source URL: https://github.com/torrust/torrust-tracker/pull/2854#discussion_r4562957889
- Concern: the correction entry stamped 11:40 UTC reports replies that were posted at 11:42:12 to 11:42:17 UTC; those replies said "in the commit addressing this round" instead of citing the commit subject; one sentence of the entry does not parse; and the F26 Concern still quoted two branch commit hashes.
- Solution: left the 11:40 UTC entry as written and appended a correction with the reply times and the sentence restated; edited the three replies, and thirteen earlier replies with the same problem, to cite their commit subject; removed the two quoted hashes.
- Current-tree verification: the `open-pull-request` commit-id check printed nothing after the commit below; `validate-audit-record.py --pr-number 2854` reported status ok with 30 rows before this row was added.
- Resolution reference: `docs(pr-reviews): [#2853] correct the audit log and drop quoted commit hashes`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2854#discussion_r4563776957

## Processing Log

- 2026-11-13 11:05 UTC - Started audit late (after R1, R2, F1-F11 replies/fixes were already in progress or posted); backfilled all entries from the current tree and the GitHub review/comment history.
- 2026-11-13 11:05 UTC - Replied on all resolvable threads (R1, R2, F1-F11, F12, F13) and resolved them via GraphQL `resolveReviewThread`.
- 2026-11-13 11:10 UTC - Added F14 (raw-summary overwrite, wrong corrected figure), fixed it, replied, and resolved its thread.
- 2026-11-13 11:28 UTC - Added R3, R4 (an earlier, unprocessed Copilot review), F15-F17, F19, F20 (da2ce7's next round) and F21-F24 (a newer Copilot review): fixed all eleven, replied on every thread, and resolved them via GraphQL `resolveReviewThread`.
- 2026-11-13 11:40 UTC - Correction: the two entries above timestamped 11:05 UTC were themselves wrong (da2ce7's F22 round). The first entry's commit was made at 10:52 UTC; the replies the second entry describes were posted at 10:54 UTC, after that commit, not at 11:05. Both entries' content stands; only their timestamps were inaccurate. Renumbered the automated review's PID-anchor/locale findings this record had assigned F21-F24 did not collide here, since da2ce7 independently reused F21-F23 for its own new findings in the same round that reported this correction; those are recorded below as F25-F27 per the skill's collision rule. Also fixed: ISSUE.md:124 no longer claims the 4-job measurement started at 08:50 (it started 08:31); `report.md`'s Method section no longer claims the peak RSS figure is absent from the summaries (only the process-count-at-peak-RSS figure is new); the Stage overlap section no longer claims `cargo binstall` fetches from crates.io or that the measured runs are shown not to have fallen back to a source compile; and two remaining branch-commit hashes (in F12's and F14's prose) were replaced with commit subjects or removed. Fixed, replied and resolved F21(da2ce7)/F22/F23(da2ce7), renumbered F25-F27 below, in `fix(container): [#2853] address review round 3: audit timestamps, report accuracy`.
- 2026-11-13 13:17 UTC - Correction to the 11:40 UTC entry above, which is left as written. Its timestamp is wrong: the three replies it reports were posted at 11:42:12 to 11:42:17 UTC (GitHub `created_at`), after 11:40. Its sentence beginning "Renumbered the automated review's" does not parse; it meant: this record had already assigned F21 to F24 to the automated review's four findings, and da2ce7 then used F21 to F23 for three new findings of its own, so those three are recorded as F25 to F27.
- 2026-11-13 13:17 UTC - Edited 16 of this PR's own review replies at 13:16:05 to 13:16:28 UTC (GitHub `updated_at`) so each cites a Conventional Commit subject: the R1 and R2 replies cited a branch commit hash, the eleven replies posted at 11:24 UTC said "the commit addressing this review round", and the three posted at 11:42 UTC said "the commit addressing this round". The eleven 11:24 UTC replies were posted before their commit existed (committed 11:29:01 UTC). Removed the two branch commit hashes quoted in the F26 Concern; the `open-pull-request` commit-id check printed both before this change.
- 2026-11-13 13:26 UTC - Added F28 after the fact: its fix was committed at 13:17:14 UTC and pushed, the reply was posted at 13:21:54 UTC (GitHub `created_at`) and the thread was then resolved. A GraphQL read after that showed 31 of 31 review threads resolved.

## Completion Rules

- Re-derive the reply claim against the current tree before replying or resolving a thread.
- Reply on every resolvable thread before resolving it.
- Cite a fix by its unique Conventional Commit subject or durable reply URL, never by a branch SHA
  that can change after a rebase.
- Refresh review threads using GraphQL and confirm that no unresolved actionable thread remains.
