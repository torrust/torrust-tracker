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
another container build runs. The uncapped build was never run. Raw samples and summaries are in [`results/`](results/).

- Machine: 32 threads, 62 GB RAM, Docker 28.3.3 with the embedded BuildKit v0.23.2 (default `docker` builder), run on 2026-11-13. Other
  work (about 13-15 GB) was running on the machine, so "peak used" is the drop of `MemAvailable` from the baseline, and "peak `rustc` RSS"
  is the sum of the resident memory of the build's `rustc` processes. Use the second column for the memory the build needs.
- Wall time includes everything in the release target: tool installation, the third-party and workspace cooks, the archive, the shipped
  build and the in-image `cargo nextest run`.

## Results

| `CARGO_BUILD_JOBS` | Wall time | Peak concurrent `rustc` | Peak `rustc` RSS | Peak `MemAvailable` drop | Lowest `MemAvailable` |
| ------------------ | --------- | ----------------------- | ---------------- | ------------------------ | --------------------- |
| 4                  | 836 s     | 12                      | 10.4 GiB         | 10.2 GiB                 | 37.4 GiB              |
| 8                  | 499 s     | 16                      | 19.8 GiB         | 18.2 GiB                 | 28.5 GiB              |
| 16                 | 346 s     | 24                      | 35.5 GiB         | 33.0 GiB                 | 14.1 GiB              |

Findings:

- Memory scales roughly with the job count: about 2.2-2.6 GiB of `rustc` RSS per unit of `CARGO_BUILD_JOBS`. A single `rustc` is
  about 0.9-1.6 GiB.
- The peak number of `rustc` is 1.5-3 times the job count, because BuildKit overlaps stages: after `dependencies_thirdparty` the test cook
  (`dependencies`) and the shipped cook (`dependencies_shipped_thirdparty`) run together, and `build` and `build_shipped` run together later.
  A cap bounds each `cargo`, not the whole build.
- Time does not fall in proportion: going from 4 to 8 jobs saves 40%, and from 8 to 16 saves another 31%, at double the memory each step.
- With 16 jobs on this machine the build still left 14 GiB free. The 31-process peaks of the incidents were not reproduced (the uncapped build is never run); they are consistent with one `rustc` per core times overlapping stages, but which command started those builds is unconfirmed.

## Decision

Default `CARGO_BUILD_JOBS=4`: a peak of about 10-11 GiB of `rustc` for the release target, which fits a 16 GB machine and needs no swap, at a cost of
about 1.7x the time of 8 jobs on the measuring machine (836 s against 499 s, cold). Bigger machines raise it with `--build-arg CARGO_BUILD_JOBS=N`.
Rule of thumb for the docs: peak RAM for the build is about 2.5 GiB times the job count, plus the OS, Docker and anything else running.
Machines with 8 GB or less should use 2 (not measured, extrapolated).

## BuildKit stage overlap

Not changed in the `Containerfile`. Serializing the overlapping stages (for example `FROM dependencies AS ...` chains, or a dummy
`COPY --from=` dependency) would make the shipped cook wait for the test cook, losing the parallelism that cached CI builds rely on, and it would
couple stages that are independent by design. The overlap is bounded by the number of independent branches (three at most in the release target), and the
rule of thumb already accounts for it. A machine-wide limit is BuildKit `max-parallelism`, which belongs to the machine configuration, not the repository.

## Effect on the runners

- GitHub-hosted runners (`ubuntu-latest`) have 4 vCPUs: Cargo already used 4 jobs, so the cap changes nothing.
- The self-hosted runner has 8 vCPU and 16 GB (`docs/self-hosted-runner.md`). It was already memory-bound: eight parallel `rustc` exhausted its RAM and the
  documented fix was a 16 GB swap file. By the table, 8 jobs need about 20 GiB for the release target, more than the machine has, while 4 jobs need about 10 GiB.
  How checked: the table above, and the run history of the `Test (Docker) (release)` job on that runner (`torrust-runner-01`), taken from the GitHub API
  (`gh-bot api repos/torrust/torrust-tracker/actions/runs/<id>/jobs`): the three successful develop runs of 2026-11-10/11 took 14-15 minutes in total, of which
  the image build step was about 12.5 minutes, because the runner's Docker store keeps the cached layers. A cold build is rarer there and may be slower by up to
  the measured factor. Not changed: no `build-args` in `container.yaml`. Compare the job duration after merge, and if it regresses, pass
  `CARGO_BUILD_JOBS=6` or similar from the workflow for the self-hosted branch only.
- Plain `cargo build` and `cargo test` outside containers read `CARGO_BUILD_JOBS` too, so the variable also limits them. The `Containerfile` sets it as
  an image environment variable only; it does not change host builds.
