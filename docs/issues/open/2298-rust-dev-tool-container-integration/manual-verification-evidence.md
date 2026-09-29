---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2298-rust-dev-tool-container-integration/ISSUE.md
last-updated-utc: 2026-09-29 18:45
---

# Manual Verification Evidence

## Purpose

Record real, human-oriented verification of the completed behavior. This is
evidence from commands or interactions actually performed against the artifact;
do not invent commands, output, logs, or results.

## Environment and Prerequisites

- Date and time (UTC): 2026-09-29 16:15 to 19:30
- Artifact under test: branch `2298-rust-dev-tool-container-integration`, patches
  `docs(issues): [#2298] record root causes and selected container design` plus the working-tree
  changes to `Containerfile`, `.dockerignore`, and `Cargo.toml` (D1, D3, D4). Baseline runs used
  the `develop` `Containerfile` at merge commit `bc90cde1b`.
- Operating system / environment: Linux workstation, Docker 28.3.3 (BuildKit, default `docker`
  driver, local layer cache). Not the self-hosted runner.
- Prerequisites and setup performed: each scenario started from a warm cache produced by a
  preceding full `docker build --target test_debug --file Containerfile .` on the same
  `Containerfile`. Raw BuildKit logs are kept locally in `.tmp/2298/` (git-ignored) and are the
  source of every number below. Stage status is the BuildKit marker (`CACHED` or `DONE <s>`) on
  the `RUN`/`COPY` step of the named stage.

## Verification Processes

### V1 - M5/M6 cache measurement, baseline `Containerfile` (manifest-only recipe stage, #1852)

- Goal: establish the cache behaviour the new design must match (spec scenarios M5, M6 on the
  baseline).
- Initial state: `develop` `Containerfile`; warm cache from `baseline-run0`
  (`dependencies_thirdparty_debug` cook 45.7 s, `dependencies_debug` cook 23.6 s, `build_debug`
  archive 34.6 s).
- Status: `DONE`

#### Steps Performed

1. M5: appended a comment line to one tracker `.rs` file; ran
   `docker build --target test_debug --file Containerfile .` (`baseline-m5-source-change.log`).
2. Reverted step 1. M6a: appended a comment line to `packages/primitives/Cargo.toml`; rebuilt
   (`baseline-m6-manifest-change.log`).
3. Reverted step 2. M6b: toggled a feature in a `packages/*/Cargo.toml` (a change that alters
   external feature resolution); rebuilt (`baseline-m6b-feature-change.log`).

#### Observed Result

```text
Scenario | recipe stage                | thirdparty cook | full cook   | build_debug archive
M5       | CACHED (all COPY + prepare) | CACHED          | CACHED      | DONE 32.0s
M6a      | DONE (prepare 0.3s + 0.2s)  | CACHED          | CACHED      | DONE 30.1s
M6b      | DONE (prepare 0.3s + 0.4s)  | DONE 46.9s      | DONE 19.0s  | DONE 26.4s
```

#### Conclusion

Baseline recorded. The recipe layer is protected from source edits as #1852 intended; the
third-party cook survives a manifest comment change thanks to #1869's `--external-only` recipe;
a feature toggle rebuilds both cooks because it changes external feature resolution. M6b is the
expected invalidation, not a defect.

### V2 - M5/M6 cache measurement, D1 `Containerfile` (canonical `COPY . .` recipe stage)

- Goal: gate decision D1. The third-party cook must stay `CACHED` for source-only and
  manifest-only changes; otherwise fall back to D2.
- Initial state: D1 applied (manifest-copy and stub lists deleted, `COPY . /build/src` restored);
  warm cache from `d1-run0` (`thirdparty` cook 42.5 s, full cook 26.7 s, archive 30.5 s; the cooks
  rebuilt once because the recipe produced from real sources differs from the stub-based one).
- Status: `DONE`

#### Steps Performed

1. M5: same `.rs` comment edit as V1; rebuilt `test_debug` (`d1-m5-source-change.log`).
2. Reverted. M6a: same `packages/primitives/Cargo.toml` comment edit; rebuilt
   (`d1-m6-manifest-change.log`).
3. Reverted. M6b: same feature toggle; rebuilt (`d1-m6b-feature-change.log`).

#### Observed Result

```text
Scenario | recipe stage                       | thirdparty cook | full cook   | build_debug archive
M5       | DONE (COPY 0.1s, prepare 0.2+0.2s) | CACHED          | CACHED      | DONE 33.2s
M6a      | DONE (COPY 0.1s, prepare 0.3+0.3s) | CACHED          | CACHED      | DONE 36.8s
M6b      | DONE (COPY 0.1s, prepare 0.2+0.2s) | DONE 43.4s      | DONE 20.7s  | DONE 27.3s
```

The `COPY --from=recipe` steps in both cook stages reported `CACHED` in M5 and M6a even though
the producing `recipe` stage had re-run.

#### Conclusion

D1 passes the gate. Cache behaviour is identical to the baseline in all three scenarios: BuildKit
keys `COPY --from` on the content checksum of the copied recipe files, so a re-run recipe stage
that emits byte-identical recipes keeps the cook layers cached. The recipe stage re-run costs
about 0.5 s. D2 (generated stubs) is not needed and was not implemented.

### V3 - `test_debug` on D1 + D3 + D4

- Goal: the debug test archive builds and runs with the allow-list context and the
  `default-members` positive list; test scope is unchanged from the baseline.
- Initial state: D1, D3 (`.dockerignore` allow-list), D4 (`default-members`; `cargo nextest archive`
  without `--workspace`/`--exclude`) applied. First attempt failed in `dependencies_debug` because
  `default-members` named `packages/*` crates that were only auto-discovered through path
  dependencies; the `--external-only` skeleton strips those, so Cargo could not resolve them. Fixed
  by listing every in-repo crate explicitly in `[workspace].members`.
- Status: `DONE`

#### Steps Performed

1. `docker build --target test_debug --file Containerfile .` (`d4-test-debug.log`).
2. Compared the nextest summary line with `baseline-run0.log`.

#### Observed Result

```text
baseline: Starting 1121 tests across 38 binaries ... 1121 tests run: 1121 passed, 0 skipped
D1+D3+D4: Starting 1121 tests across 38 binaries ... 1121 tests run: 1121 passed, 0 skipped
```

Stage durations: `thirdparty` cook 46.5 s (rebuilt: `Cargo.toml` membership changed), full cook
9.4 s, archive 34.1 s. The warm-up archive in `dependencies_debug` and the final archive in
`build_debug` compiled no `contrib/dev-tools` crate and no verification tool.

#### Conclusion

Met. Identical test scope (1121 tests / 38 binaries) with no `--exclude` list; the positive list
selects exactly what the negative lists used to leave in.

### V4 - Release `test` target and `runtime` image (M2/M3 release path)

- Goal: the release archive builds and runs; the final `runtime` image builds and contains only
  tracker binaries.
- Initial state: same as V3.
- Status: `IN_PROGRESS`

#### Steps Performed

1. `docker build --target test --file Containerfile .` — first attempt (`d4-test-release.log`) was
   interrupted by a host restart during the release compile; to be rerun by the maintainer
   outside the IDE.
2. `docker build --target runtime --tag torrust-tracker:2298-local --file Containerfile .` — pending.
3. Image content inspection — pending.

#### Observed Result

```text
pending
```

#### Conclusion

Pending rerun.

### V5 - M1, M2, M3, M7 (disposable package and context probe)

- Status: `TODO`
- M1 (reproduce failure on the baseline), M2 (new default-member package needs no container edit),
  M3 (non-default member stays out of the archive), and M7 (context contains only allow-listed
  paths) are not yet executed.

### V6 - M4 hosted Container workflow

- Status: `TODO` (requires the fork PR).

## Failures and Follow-up

- V3 first attempt: `cargo chef cook` in `dependencies_debug` failed to resolve `default-members`
  entries for auto-discovered `packages/*` crates because the `--external-only` skeleton drops path
  dependencies. Remediation: list all in-repo crates explicitly in `[workspace].members` (recorded
  as ADR agreement 5). Rerun passed.
- V4: interrupted by a host restart (the Docker build saturated the workstation while the IDE was
  running). Rerun scheduled outside the IDE.
