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
another container build runs, and traps its own INT/TERM to cancel the build if the script itself is interrupted. It refuses to
start while another container build runs. The uncapped build was never run. Raw samples and summaries are in [`results/`](results/),
unmodified since each run printed them. "Peak `MemAvailable` drop" and "Lowest `MemAvailable`" below are copied from those files
as printed; "`rustc` at peak RSS" and "Peak `rustc` RSS" are recomputed from the raw samples (see the next bullet and "Per-process
memory") and are not in the summary files, since the original `measure.sh` reported two independent column maxima instead (see
"Per-process memory" for why that was wrong).

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
three runs, against an expected worst case of twice the job count (see "Stage overlap" below): exactly met at 8 jobs (16), under it at
16 jobs (24 < 32, bounded by the 32 cores available), but above it at 4 jobs (12 > 8). The peak *count* and the peak *memory* did not
happen at the same sample in any of the three runs — at the peak-memory moment the count is close to the job count itself (one
dominant stage), while the peak-count moment is a brief, lower-memory spike. The most likely explanation is a transient overlap at
the handoff between two `RUN` steps of the *same* chain (the old step's last `rustc` processes still exiting while the new step's
first ones have already started), caught by the 2-second sampling interval; this was not isolated further.

## Per-process memory

Dividing the peak-RSS row's own memory by its own `rustc` count gives a consistent per-process figure: 2.60 GiB (4 jobs), 2.47 GiB
(8 jobs), 2.09 GiB (16 jobs) — all within the 2.2-2.6 GiB/job used for the rule of thumb below. (An earlier version of this report
and of `measure.sh` divided the independent peak-count maximum into the independent peak-RSS maximum and got 0.9-1.6 GiB/process; that
arithmetic mixed two different moments and is wrong — see "Method" above. `measure.sh` now keeps the two figures from the same sample.)

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
Every branch hangs off `dependencies_thirdparty`, so **at most two** compiling stages run at once in this target, not three as an earlier
version of this report said. Serializing them (for example coupling one branch's `FROM` to the other's output) would make the shipped
build wait for the test cook, losing parallelism that cached CI builds rely on, and would couple stages that are independent by design.
The rule of thumb already covers the observed worst case (see "Results" above). A machine-wide limit is BuildKit `max-parallelism`, which
belongs to the machine configuration, not the repository.

## Effect on the runners

- GitHub-hosted runners (`ubuntu-latest`) have 4 vCPUs: Cargo already used at most 4 jobs, so the cap changes nothing there.
- The self-hosted runner has 8 vCPU and 16 GB (`docs/self-hosted-runner.md`). It was already memory-bound: eight parallel `rustc`
  exhausted its RAM on first use and the documented fix was a 16 GB swap file. By the table above, 8 jobs need about 20 GiB for the
  release target (more than the machine's 16 GB RAM, though swap absorbs the rest), while 4 jobs need about 10 GiB, comfortably inside
  it. `container.yaml` passes no `CARGO_BUILD_JOBS` build argument, so with this change that runner's `Test (Docker) (release)` job now
  builds at the new default of 4 instead of uncapped (one `rustc` per vCPU, 8, same as the two incidents that justified the swap file,
  not materially different from before this change). `build` and `build_shipped` both `COPY . /build/src`, so every source change on
  that branch recompiles the whole workspace; the step is not a one-time cold build, it runs this way on every PR into `develop` and
  every push to it.
  - Measured directly on `torrust-runner-01`: the run of this PR's head commit after the Copilot-review fixes
    (`run 43410572813`, head `334decbf9`) is the first real data point at the new default. Its `Test (Docker) (release)` job duration
    and the prior unlimited-jobs baseline (the three successful `develop` runs of 2026-11-10/11, each about 12.5 minutes for the image
    build step, with the runner's persistent Docker store keeping cached layers) are compared once that run completes; see the issue's
    manual-verification-evidence for the recorded numbers. If the capped build regresses meaningfully, `container.yaml` can pass a
    higher `CARGO_BUILD_JOBS` for that runner specifically.
- Plain `cargo build` and `cargo test` outside containers read `CARGO_BUILD_JOBS` too, so the variable also limits them. The
  `Containerfile` sets it as an image environment variable only; it does not change host builds.
