# Container Build: Cargo Parallelism vs Memory

## Question

The `Containerfile` set no limit on Cargo jobs, so every `cargo` started one `rustc` per core and BuildKit ran
independent stages at once. On a 32-thread / 62 GB workstation this reached 31 root-owned `rustc` processes holding
33-38 GB and the kernel OOM killer fired twice. What default `CARGO_BUILD_JOBS` keeps the image build safe on a small
machine, and what does it cost in time?

## Method

[`measure.sh`](measure.sh) runs one uncached build of the release image per job count:

```sh
docker build --no-cache --progress=plain --target release --build-arg CARGO_BUILD_JOBS=<N> --file Containerfile .
```

While it runs it samples `MemAvailable` and the root-owned `rustc` processes (those started by Docker/BuildKit) every 2 seconds,
and it would cancel the build by the PID of the `docker build` client if `MemAvailable` fell below 8 GiB. It refuses to start while
another container build runs, and traps its own INT/TERM to cancel the build if the script itself is interrupted. The uncapped
build was never run. Raw samples and summaries are in [`results/`](results/),
unmodified since each run printed them. "Peak `MemAvailable` drop", "Lowest `MemAvailable`" and "Peak `rustc` RSS" below are copied
from those files as printed (the restored summaries already carry the peak RSS, since a column's maximum is the same number
whichever row it is read from). Only "`rustc` at peak RSS" is new: the original `measure.sh` printed it as an independent maximum
of the process-count column, which can be a different sample than the one holding the peak RSS; see "Per-process memory" for why
reading the two from different samples, not printing them, was the error.

- Machine: 32 threads, 62 GB RAM, Docker 28.3.3 with the embedded BuildKit v0.23.2 (default `docker` builder), run on 2026-11-13. Other
  work (about 13-15 GB) was running on the machine, so "peak used" is the drop of `MemAvailable` from the baseline.
- Artifact under test: the working tree with the `chef`-stage `ARG CARGO_BUILD_JOBS`/`ENV` already applied (the change later committed
  as `fix(container): [#2853] cap the Cargo jobs of the image build with CARGO_BUILD_JOBS`), before the `tester`-stage cap and the
  `measure.sh` signal trap added in the follow-up commit that addressed the first review round.
- Wall time includes everything in the release target: tool installation, the third-party and workspace cooks, the archive, the shipped
  build and the in-image `cargo nextest run`.
- "Peak `rustc` RSS" below is read from the single sample row with the highest summed resident memory of the build's root-owned `rustc`
  processes, and the "concurrent `rustc`" figure next to it is the count from that *same* row, not an independent maximum (an earlier
  version of this report and of `measure.sh` computed the two columns' maxima independently, which can come from different moments; see
  "Per-process memory" below).

## Results

| `CARGO_BUILD_JOBS` | Wall time | `rustc` at peak RSS | Peak `rustc` RSS | Peak `MemAvailable` drop | Lowest `MemAvailable` |
| ------------------ | --------- | -------------------- | ---------------- | ------------------------ | --------------------- |
| 4                  | 836 s     | 4                     | 10.4 GiB         | 10.2 GiB                 | 37.4 GiB              |
| 8                  | 499 s     | 8                     | 19.8 GiB         | 18.2 GiB                 | 28.5 GiB              |
| 16                 | 346 s     | 17                    | 35.5 GiB         | 33.0 GiB                 | 14.1 GiB              |

A separate, independent maximum of concurrent `rustc` processes (regardless of their memory at that moment) was 12, 16 and 24 for the
three runs, against an expected worst case of twice the job count for the two workspace-compiling branches (see "Stage overlap"
below): exactly met at 8 jobs (16), under it at 16 jobs (24 < 32, bounded by the 32 cores available), but above it at 4 jobs (12 > 8).
The peak *count* and the peak *memory* did not happen at the same sample in any of the three runs — at the peak-memory moment the
count is close to the job count itself (one dominant stage), while the peak-count moment is a brief, lower-memory spike. The cause of
that count spike was not isolated; a stage handoff is one candidate, but a stage only starts after its predecessor's whole process
tree has exited, so that alone does not explain an overlap, and no other candidate was checked either.

## Per-process memory

Dividing the peak-RSS row's own memory by its own `rustc` count gives a consistent per-process figure: 2.60 GiB (4 jobs), 2.47 GiB
(8 jobs), 2.09 GiB (16 jobs) — all within the 2.09-2.60 GiB/job range this report's rule of thumb below rounds to 2.5. (An earlier
version of this report divided the independent peak-count maximum into the independent peak-RSS maximum and got 0.9-1.6 GiB/process;
that arithmetic mixed two different moments and is wrong — see "Method" above. The earlier `measure.sh` itself did not divide
anything; it only printed the two independent maxima, each correct on its own, and the division happened in this report's prose.
`measure.sh` now reports the rustc count from the same sample as the peak RSS, so this report's division uses one consistent sample.)

Other findings:

- Memory scales roughly with the job count: about 2.5 GiB of `rustc` RSS per unit of `CARGO_BUILD_JOBS` (see "Per-process memory").
- Time does not fall in proportion: going from 4 to 8 jobs saves 40%, and from 8 to 16 saves another 31%, at double the memory each step.
- With 16 jobs on this machine the build still left 14 GiB free. The 31-process incidents were not reproduced (the uncapped build is
  never run); the per-process size here (about 2-2.6 GiB) is higher than the 1.2-2.2 GiB estimated in the issue from the incidents,
  so 31 such processes would need more than the 33-38 GB observed — the incidents likely had some processes still ramping up when the
  kernel log was captured. Which command started those builds remains unconfirmed.

## Decision

Default `CARGO_BUILD_JOBS=4`: a peak of about 10.4 GiB of `rustc` for the release target, which fits a 16 GB machine and needs no swap, at
a cost of about 1.7x the time of 8 jobs on the measuring machine (836 s against 499 s, cold). Bigger machines raise it with
`--build-arg CARGO_BUILD_JOBS=N`. Rule of thumb for the docs: peak RAM for the build is about 2.5 GiB times the job count, plus the OS,
Docker and anything else running. Machines with 8 GB or less should use 2 (not measured, extrapolated).

## Stage overlap

Not changed in the `Containerfile`. For the release target the compiling stages are `dependencies_thirdparty` (third-party cook), then,
once it finishes, two independent branches that both derive from it: `dependencies` (workspace cook) → `build` (nextest archive), and
`dependencies_shipped_thirdparty` (shipped third-party cook) → `dependencies_shipped` (shipped cook) → `build_shipped` (binary build).
Every branch hangs off `dependencies_thirdparty`, so **at most two** of these workspace-compiling stages run at once, not three as an
earlier version of this report said. That bound does not cover the whole release target, though: `tester` does not derive from
`dependencies_thirdparty` (or from `chef` at all; see the `Containerfile`), so its own `cargo binstall` of `cargo-nextest` can fall
back to a source compile independently of, and concurrently with, both workspace branches — a third, unbound compile path, capped
individually by `tester`'s own `CARGO_BUILD_JOBS` (R1) but not by this two-stage count. Serializing the two workspace branches (for
example coupling one branch's `FROM` to the other's output) would make the shipped build wait for the test cook, losing parallelism
that cached CI builds rely on, and would couple stages that are independent by design. The rule of thumb already covers the observed
worst case for the measured release target (see "Results" above): the measured runs kept no `tester`-stage build log, so whether
`cargo binstall` used a prebuilt download (from the crate's release artifacts or a binary mirror, not crates.io) or fell back to a
source compile is assumed from the `Containerfile`'s own behavior, not shown by anything recorded here. A machine-wide limit is
BuildKit `max-parallelism`, which belongs to the machine configuration, not the repository.

## Effect on the runners

- GitHub-hosted runners (`ubuntu-latest`) have 4 vCPUs: Cargo already used at most 4 jobs, so the cap changes nothing there.
- The self-hosted runner has 8 vCPU and 16 GB (`docs/self-hosted-runner.md`). It was already memory-bound: eight parallel `rustc`
  exhausted its RAM on first use and the documented fix was a 16 GB swap file. `container.yaml` passes no `CARGO_BUILD_JOBS` build
  argument, so before this change the runner compiled uncapped, one `rustc` per vCPU (8), needing about 20 GiB by the table above
  (more than its 16 GB RAM, relying on the swap file); after this change it compiles at the new default of 4, needing about 10 GiB,
  comfortably inside its RAM. `build` and `build_shipped` both `COPY . /build/src`, so every source change recompiles the whole
  workspace regardless of Docker layer caching for those two stages; this is not a one-time cold-build effect, it happens on every
  PR into `develop` and every push to it.
  - The runner's actual `Test (Docker) (release)` job duration at the new default, compared against the prior unlimited-jobs
    baseline (the three successful `develop` runs of 2026-11-10/11, each about 12.5 minutes for the image build step, with the
    runner's persistent Docker store keeping cached layers from earlier runs), is pending: see manual scenario M4 and its evidence
    section V4, named by the commit subject of the PR head they measure rather than a branch commit id (a rebase would orphan an id).
    If the capped build regresses meaningfully, `container.yaml` can pass a higher `CARGO_BUILD_JOBS` for that runner specifically.
- Plain `cargo build` and `cargo test` outside containers read `CARGO_BUILD_JOBS` too, so the variable also limits them. The
  `Containerfile` sets it as an image environment variable only; it does not change host builds.
