---
semantic-links:
  related-artifacts:
    - docs/issues/open/1841-1840-workflow-performance-baseline-analysis/ISSUE.md
    - .github/workflows/container.yaml
    - .github/workflows/testing.yaml
---

# Baseline Workflow Benchmark Results

Recorded on: 2026-05-27

This file is the living benchmark artifact for the workflow-performance EPIC. Update it whenever a later optimization changes the performance profile so future runs can be compared against the same baseline.

## Benchmark Policy

- Capture one cold run after clearing relevant local caches.
- Capture one warm run immediately after the cold run without clearing caches.
- Record both total workflow time and the main internal phases for each workflow.
- Note the cache-reset procedure used to approximate a shared-runner first run.

## Cache Reset Notes

Document the exact commands used before the cold run, including any of the following where applicable:

- `cargo clean`
- Removal of `target/` and other local Rust build artifacts
- Clearing the local cargo registry and git checkout caches if they would affect the run
- Docker builder cache cleanup if the workflow step uses Docker layers locally

## Measurement Table

| Workflow  | Run Type        | Total Time | Main Bottleneck | Notes                                  |
| --------- | --------------- | ---------- | --------------- | -------------------------------------- |
| container | cold / no-cache | TBD        | TBD             | Fill after the first baseline capture. |
| container | warm / cached   | TBD        | TBD             | Fill after the second capture.         |
| testing   | cold / no-cache | TBD        | TBD             | Fill after the first baseline capture. |
| testing   | warm / cached   | TBD        | TBD             | Fill after the second capture.         |

## Internal Phase Breakdown

Record the major steps inside each job, ordered from longest to shortest if possible.

### Container Workflow

| Phase | Cold Run | Warm Run | Notes |
| ----- | -------- | -------- | ----- |
| TBD   | TBD      | TBD      | TBD   |

### Testing Workflow

| Phase | Cold Run | Warm Run | Notes |
| ----- | -------- | -------- | ----- |
| TBD   | TBD      | TBD      | TBD   |

## Linker-Heavy Target Analysis (Container Build Path)

Record the most expensive compile and link targets observed while reproducing the container build path.

| Rank | Target / Package | Estimated Compile+Link Time | Required for Tracker Runtime Image (`yes`/`no`) | Notes |
| ---- | ---------------- | --------------------------- | ----------------------------------------------- | ----- |
| 1    | TBD              | TBD                         | TBD                                             | TBD   |
| 2    | TBD              | TBD                         | TBD                                             | TBD   |
| 3    | TBD              | TBD                         | TBD                                             | TBD   |

Guidance:

- Mark `yes` only when the target is needed to produce or validate the tracker runtime image.
- Mark `no` when the target is outside runtime image needs (for example benches, unrelated binaries, or examples not required by image tests).
- Keep rationale short and concrete in the Notes column.

## Comparison Notes

- What dominated the cold run?
- Which phases benefited from the warm cache?
- Which phases are not helped much by caching?
- Which linker-heavy targets appear unrelated to the final tracker runtime image?
- Which measurements should be repeated after the next optimization?

## Follow-up

After each later workflow optimization, append a new dated note here with the same measurement format so the EPIC retains a simple before/after history.
