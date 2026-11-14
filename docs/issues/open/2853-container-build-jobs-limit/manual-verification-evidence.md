---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2853-container-build-jobs-limit/ISSUE.md
last-updated-utc: "2026-11-14 08:26"
---

# Manual Verification Evidence

## Environment and Prerequisites

- Date (UTC): 2026-11-13
- Artifact under test: the changes of commits `fix(container): [#2853] cap the Cargo jobs of the image build with CARGO_BUILD_JOBS`
  and `fix(container): [#2853] address Copilot review: cap tester stage, trap measure.sh signals` (the `Containerfile` and
  `docker-build.sh` as they stand at this document's `last-updated-utc`).
- Environment: Linux, 32 threads, 62 GB RAM, Docker 28.3.3 (embedded BuildKit v0.23.2), no other container build running.

## Reproduction (V1)

Status: **Infeasible**. The defect is memory exhaustion of the host machine itself, reported by the maintainer from the kernel log of
two real incidents (2026-11-11 11:32 and 2026-11-12 21:10): 31 root-owned `rustc` processes holding 33-38 GB of RSS, RAM exhausted,
8 GB swap full, and the kernel OOM killer then killing the desktop application and every agent session in it. Reproducing it would
mean repeating an uncapped build on the same machine, which already crashed it twice.

- Attempted commands: none against the uncapped build; `measure.sh` refuses to run it by design (see `report.md`, "Method").
- Blocking constraint: a third incident would again end every other agent session sharing the machine; the maintainer's own prompt for
  this issue treats it as unsafe.
- Strongest substitute evidence: the capped measurements at 4, 8 and 16 jobs in `report.md` and `results/`, which show memory growing
  with the job count at a similar per-process size (2.09-2.60 GiB, against the 1.2-2.2 GiB the issue estimated from the incidents; see
  `report.md`, "Other findings" for why those are not directly comparable) and confirm that bounding the Cargo jobs bounds the `rustc`
  memory of the build. The exact kernel OOM log lines are not reproduced here verbatim; they are not in this repository and were
  reported to this session by the maintainer, not captured by it directly.

## V2: release build with the default, no override

```text
$ docker build -q --target chef --tag tmp-chef-default --file Containerfile .
sha256:c0ad752484cf24e9352a58df6f630699cb893a837a22050a9cd9533b6824259b
$ docker run --rm --entrypoint printenv tmp-chef-default CARGO_BUILD_JOBS
4
$ docker build -q --target tester --tag tmp-tester-default --file Containerfile .
sha256:43a8a8394bc4b5500c46a2e2510d2483cbf9843c417982e1427ed669c3b513a0
$ docker run --rm --entrypoint printenv tmp-tester-default CARGO_BUILD_JOBS
4
```

No `--build-arg` was passed in either command, so this shows the compiled-in default (`4`) reaching both `chef`- and `tester`-derived
stages, which `--build-arg CARGO_BUILD_JOBS=4` alone (passed explicitly) would not have proven. The full release build at this
default is the `jobs-4` measurement run in `report.md` (`results/jobs-4.summary.txt`): `docker build --no-cache --progress=plain
--target release --build-arg CARGO_BUILD_JOBS=4 --tag torrust-tracker:jobs-4 --file Containerfile .` exited 0 in 836 s, peak `rustc`
RSS 10.4 GiB. Result: pass.

## V3: override and script

```text
$ docker build -q --target chef --build-arg CARGO_BUILD_JOBS=2 --tag tmp-chef-check --file Containerfile .
$ docker run --rm --entrypoint printenv tmp-chef-check CARGO_BUILD_JOBS
2
$ docker build -q --target tester --build-arg CARGO_BUILD_JOBS=2 --tag tmp-tester-check --file Containerfile .
$ docker run --rm --entrypoint printenv tmp-tester-check CARGO_BUILD_JOBS
2
$ CARGO_BUILD_JOBS=2 bash contrib/dev-tools/containers/docker-build.sh   # `docker` replaced by an echo stub
docker build --target release --tag torrust-tracker:release --build-arg CARGO_BUILD_JOBS=2 --file Containerfile .
$ bash contrib/dev-tools/containers/docker-build.sh                      # variable unset
docker build --target release --tag torrust-tracker:release --file Containerfile .
```

Result: pass.

## V4: self-hosted runner effect

Step times of `Build Tracker Image` in the `Test (Docker) (release)` job, all on `torrust-runner-01`, read on 2026-11-13 with
`gh api repos/torrust/torrust-tracker/actions/runs/<run id>/jobs` (the step's `started_at` and `completed_at`):

| Run id      | Built                                                                                          | Cargo jobs   | Build step |
| ----------- | ---------------------------------------------------------------------------------------------- | ------------ | ---------- |
| 42846692510 | `develop`, 2026-11-10                                                                          | uncapped (8) | 721 s      |
| 43016685734 | `develop`, 2026-11-11                                                                          | uncapped (8) | 723 s      |
| 43059027926 | `develop`, 2026-11-11                                                                          | uncapped (8) | 749 s      |
| 43410572813 | this PR at `fix(container): [#2853] address Copilot review: cap tester stage, trap measure.sh signals` | 4    | 814 s      |
| 43413696734 | this PR at `docs(pr-reviews): [#2853] fix audit record table formatting and reply URLs`        | 4            | 842 s      |
| 43420090960 | this PR at `fix(container): [#2853] address review round 3: audit timestamps, report accuracy` | 4            | 824 s      |

Means: 731 s uncapped, 827 s at 4 jobs, a difference of 96 s (13%). Limits of this comparison: the runs built different commits, and
the state of the runner's Docker layer cache at each run was not checked.

In those three runs of this PR the build step passed and the next step, `Run Persistence Transition Regression`, failed. The
same step failed on `develop` at the branch's merge base of that time (run 43299605132, 2026-11-12). Issue #2855 fixed it on
`develop`, and after this branch was rebased onto that fix the whole `Test (Docker) (release)` job passed (run 43520878226,
2026-11-14: build step 837 s, at 4 jobs, not included in the means above).

Result: measured. The maintainer accepted the 13% slower build step for AC6 on 2026-11-13. The acceptance was
given in the agent session chat, not on GitHub, before 15:17 UTC (the time GitHub's edit history of the pull request description shows it changed to `Closes #2853`).
