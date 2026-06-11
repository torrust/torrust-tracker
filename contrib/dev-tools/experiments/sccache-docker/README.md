# sccache Docker Integration Experiments

This directory contains progressive experiments to determine the best strategy for using sccache
inside Docker builds — specifically for the Torrust Tracker `container.yaml` workflow.

## Experiment Structure

| #   | Directory         | Description                                                           | Status     |
| --- | ----------------- | --------------------------------------------------------------------- | ---------- |
| 1   | `01-basic-build/` | Single-stage Docker build: sccache with BuildKit `--mount=type=cache` | ✅ Done    |
| 2   | `02-multi-stage/` | Multi-stage build mirroring the real Containerfile structure          | 🔲 Pending |
| 3   | `03-gha-backend/` | Mocked GHA backend credentials to test sccache with remote cache      | 🔲 Pending |

## How to Run

Each experiment directory has a `Dockerfile` and test crate. Run from that directory:

```sh
cd <experiment-dir>
docker buildx build --load --progress=plain --no-cache -t sccache-experiment -f Dockerfile .
```

The `--no-cache` flag ensures every layer rebuilds (avoids cached Docker layers from previous runs).

## Results Tracking

Each experiment logs:

- Build output (wall time per step)
- sccache stats (cache hits/misses)
- Key findings and limitations

Results are documented in each experiment's directory.
