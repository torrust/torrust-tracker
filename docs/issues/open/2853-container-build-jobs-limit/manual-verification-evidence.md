---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2853-container-build-jobs-limit/ISSUE.md
last-updated-utc: 2026-11-13 09:10
---

# Manual Verification Evidence

## Environment and Prerequisites

- Date (UTC): 2026-11-13
- Artifact under test: the `Containerfile`, `docker-build.sh` and docs changes of this issue (default `CARGO_BUILD_JOBS=4`).
- Environment: Linux, 32 threads, 62 GB RAM, Docker 28.3.3 (embedded BuildKit v0.23.2), no other container build running.

## V2: release build with the default

`CARGO_BUILD_JOBS=4` is the default, and the measurement run for 4 jobs is the default build: `docker build --no-cache --progress=plain --target release --build-arg CARGO_BUILD_JOBS=4 ...` exited 0 in 836 s (peak 12 `rustc`, 10.4 GiB). Raw data: `contrib/dev-tools/experiments/container-build-jobs/results/jobs-4.summary.txt`. Result: pass. Note: the build argument was passed explicitly, equal to the default; the default itself is exercised by CI.

## V3: override and script

```text
$ docker build -q --target chef --build-arg CARGO_BUILD_JOBS=2 --tag tmp-chef-check --file Containerfile .
$ docker run --rm --entrypoint printenv tmp-chef-check CARGO_BUILD_JOBS
2
$ CARGO_BUILD_JOBS=2 bash contrib/dev-tools/containers/docker-build.sh   # `docker` replaced by an echo stub
docker build --target release --tag torrust-tracker:release --build-arg CARGO_BUILD_JOBS=2 --file Containerfile .
$ bash contrib/dev-tools/containers/docker-build.sh                      # variable unset
docker build --target release --tag torrust-tracker:release --file Containerfile .
```

Result: pass.
