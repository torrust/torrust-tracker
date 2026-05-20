---
doc-type: issue
issue-type: task
status: done
priority: p2
github-issue: 1748
spec-path: docs/issues/closed/1748-remove-redundant-compose-step-from-container-workflow.md
branch: 1748-remove-redundant-compose-step-from-container-workflow
related-pr: null
last-updated-utc: null
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - docs/issues/README.md
    - .github/workflows/container.yaml
    - .github/workflows/testing.yaml
---

# Remove Redundant Compose Step From Container Workflow

## Overview

The `container` workflow still includes a `Compose` step that runs:

- `docker compose -f compose.qbittorrent-e2e.sqlite3.yaml build`
- `docker compose -f compose.qbittorrent-e2e.mysql.yaml build`
- `docker compose -f compose.qbittorrent-e2e.postgresql.yaml build`

This step no longer provides unique verification value and adds significant CI time.

- GitHub issue: [#1748](https://github.com/torrust/torrust-tracker/issues/1748)
- Affected workflow: [`.github/workflows/container.yaml`](../../.github/workflows/container.yaml)
- Related workflow: [`.github/workflows/testing.yaml`](../../.github/workflows/testing.yaml)

## Background

Historically, the `Compose` step in `container.yaml` was used as a lightweight check to ensure
compose configuration remained buildable.

The project now has dedicated compose runtime coverage in `testing.yaml` (`docker-e2e` job):

- `e2e_tests_runner --tracker-image torrust-tracker:e2e-local --skip-build`
- `qbittorrent_e2e_runner --tracker-image torrust-tracker:e2e-local --skip-build --db-driver sqlite3`
- `qbittorrent_e2e_runner --tracker-image torrust-tracker:e2e-local --skip-build --db-driver mysql`
- `qbittorrent_e2e_runner --tracker-image torrust-tracker:e2e-local --skip-build --db-driver postgresql`

As a result, compose files are actively validated by tests that matter at runtime.

## Problem

The `Compose` step in `container.yaml` is redundant and expensive:

- It performs only extra build invocations, not runtime verification.
- It can trigger repeated image builds in the same job.
- It increases CI duration in the `container` workflow substantially.
- It makes Docker layer-cache behavior harder to reason about in workflow diagnostics.

## Proposed Change

Remove the `Compose` step from the `test` job in `.github/workflows/container.yaml`.

Keep the existing `Build` + `Inspect` steps in `container.yaml` for image build integrity checks,
while retaining compose runtime validation in `testing.yaml` (`docker-e2e`).

## Goals

- [ ] Remove the `Compose` step from `.github/workflows/container.yaml`.
- [ ] Keep `container` workflow matrix build behavior unchanged (`debug` and `release`).
- [ ] Keep compose runtime verification in `.github/workflows/testing.yaml`.
- [ ] Confirm reduced CI duration for `container` workflow after merge.

## Non-Goals

- Changing compose files used by E2E tests.
- Modifying test logic in `e2e_tests_runner` or `qbittorrent_e2e_runner`.
- Altering publish jobs in `container.yaml`.
