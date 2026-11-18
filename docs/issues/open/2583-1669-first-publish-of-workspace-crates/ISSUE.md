---
schema-version: 1
doc-type: issue
issue-type: task
status: planned
priority: p2
epic: 1669
github-issue: 2583
spec-path: docs/issues/open/2583-1669-first-publish-of-workspace-crates/ISSUE.md
branch: "2583-1669-first-publish-of-workspace-crates-spec"
related-pr: null
last-updated-utc: "2026-11-18 09:47"
semantic-links:
  skill-links:
    - create-issue
    - release-new-version
  related-artifacts:
    - .claude/skills/create-issue/SKILL.md
    - .claude/skills/release-new-version/SKILL.md
    - .github/workflows/deployment-packages.yaml
    - .github/workflows/deployment.yaml
    - "issue #1669"
    - "issue #2454"
    - "issue #2482"
    - "issue #2576"
    - "issue #2582"
    - "issue #1488"
    - "issue #1347"
    - docs/issues/open/1669-overhaul-packages/EPIC.md
    - docs/issues/open/1347-overhaul-packages-testing/EPIC.md
    - docs/issues/closed/2482-1669-reorganize-shared-test-support/EPIC.md
    - docs/issues/closed/2576-1669-update-all-package-readmes/ISSUE.md
    - docs/issues/closed/2582-1669-remove-health-check-self-dev-dependency/ISSUE.md
    - docs/issues/closed/2454-1669-mark-public-error-enums-non-exhaustive/error-enum-inventory.md
    - docs/adrs/20260629000000_adopt_independent_package_versioning.md
    - docs/release-process/workspace-packages.md
    - Cargo.toml
---

<!-- skill-link: create-issue -->

# Issue #2583 - First publish of the unpublished workspace crates

Parent EPIC: #1669 (Overhaul: Packages).

## Goal

Publish every publishable library crate of the workspace that is not yet on crates.io, one crate at
a time and in dependency order, at the end of EPIC #1669. Publish the next version of the library
crates that are already on crates.io in the same order, with the semver-major bumps they owe. The
root `torrust-tracker` crate and the console client are published elsewhere (see
[Decisions](#maintainer-decisions)).

## Background

The [independent versioning ADR](../../../adrs/20260629000000_adopt_independent_package_versioning.md)
says every publishable workspace crate is published to crates.io on its own, through
`deployment-packages.yaml`, and that the tracker release (`deployment.yaml`) publishes only the
root `torrust-tracker` crate. Most workspace crates have never been published, so nothing outside
this repository can depend on them, and the console client cannot be extracted (its library
dependencies must be on crates.io first).

### Maintainer Decisions

The maintainer decided on 2026-10-20:

| Question                         | Decision                                                                                                                                                             |
| -------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Tracking                         | One tracking issue with a per-crate checklist in dependency order (leaves first). No sub-issue per crate; a crate that needs real pre-publish work gets its own sub-issue, linked from the checklist. |
| When                             | One crate at a time, at the end of EPIC #1669, after the API overhaul. This issue is opened now and stays `Blocked` until its prerequisites are met.                |
| First version of unpublished crates | `0.1.0`, their current value; no bump.                                                                                                                            |
| Root `torrust-tracker`           | Out of scope. The next tracker application release (`deployment.yaml`, `release-new-version` skill) publishes it, with the semver-major bump it owes.               |
| Version-bump pull requests       | One small pull request per crate whose version changes, right before that crate's publish.                                                                          |
| Console client `torrust-tracker-client` | Not published here. Its first publish comes from its own repository in the client-extraction issue.                                                          |
| crates.io owners                 | `josecelano` and `da2ce7`, as for the four published workspace crates.                                                                                              |
| Crates kept unpublished          | None beyond the ten `publish = false` members.                                                                                                                      |
| Other prerequisites              | EPIC #1488 (shutdown overhaul) closes first. The EPIC #1347 (package testing) part of this row was replaced on 2026-11-18 by a per-crate testing gate (see below). |
| `cargo package` failure          | Fix it in its own sub-issue (P1, #2582), created together with this issue.                                                                                                  |
| Stale publish docs               | Fix them in this issue before the first publish (T2).                                                                                                                |

The maintainer decided on 2026-11-18:

| Question                  | Decision                                                                                                                                                                                                 |
| ------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| EPIC #1347 prerequisite   | Per crate, not EPIC-wide. A crate may be published once its own unit-testing coverage was overhauled in EPIC #1347. This issue no longer waits for EPIC #1347 to close as a whole.                        |
| Tracking the testing gate | Per crate in this spec: the "Testing (#1347)" column of the [Per-Crate Checklist](#per-crate-checklist).                                                                                                 |

What follows from this decision, without any new decision:

- Publishing stays in dependency order. A crate can be published when its own testing gate is met
  **and** every crate it depends on or dev-depends on is `PUBLISHED`. A crate whose testing gate is
  met still waits for dependencies whose gate is not.
- The other prerequisites did not change (see [Open Questions](#open-questions) OQ2 and OQ3).
- T2 (publish procedure check) was never blocked by EPIC #1347 and is still not blocked.

State on 2026-11-18 (EPIC #1347 `Subissues` table and the GitHub state of each sub-issue):

- **Testing gate met** (the crate's #1347 sub-issues are closed): `primitives` (#2824, #2876), `udp-core`
  (#1348), `http-core` (#1349), `axum-http-server` (#2136, #2140), `udp-server` (#2149, #2283) and
  `axum-rest-api-server` (#2637).
- **Sub-issue open:** `core` (#2902, `TODO`).
- **No #1347 sub-issue yet** (13 crates): `events`, `http-protocol`, `udp-protocol`,
  `rest-api-protocol`, `configuration`, `swarm-coordination-registry`, `client-lib`,
  `rest-api-application`, `rest-api-client`, `axum-server`, `test-helpers`,
  `rest-api-runtime-adapter` and `axum-health-check-api-server`.
- `primitives` (level 0, no workspace dependencies) is the only crate whose testing gate and
  dependency gate are both met. The five other crates with a met testing gate all depend on crates
  without one. `primitives` still waits for prerequisites 3 and 5, and EPIC #2912 (remove source
  locations from errors) requires its SI-2 (#2914, `TODO`) to land before `primitives` 4.0.0 is
  published. That constraint comes from EPIC #2912; it is not a prerequisite decided in this issue.
- The client extraction (#2594) needs `http-protocol`, `udp-protocol` and `client-lib` published
  (rows 3, 4 and 8). None of the three has a #1347 sub-issue, and they depend only on each other,
  so meeting their testing gate is the shortest path to unblock #2594.

### Open Questions

Pending maintainer decisions, raised by the 2026-11-18 decision. They are not decided here; until
they are, the spec keeps the stricter reading (noted in each question).

| ID  | Question | Until decided |
| --- | -------- | ------------- |
| OQ1 | Does every publishable crate need its own #1347 testing sub-issue, or can a small crate meet the gate with a recorded coverage review and no new tests? | A crate without a closed #1347 sub-issue does not meet the gate (`NONE` in the checklist). |
| OQ2 | Does the EPIC #1488 prerequisite also become per crate (only the crates whose API the shutdown overhaul changes), or does it stay EPIC-wide? EPIC #1488 is expected to close soon. | EPIC-wide: prerequisite 3 as written. |
| OQ3 | Does prerequisite 5 (the rest of EPIC #1669's API work is done, so this is the last publishing step) stay as it is, or become per crate too? | As written: prerequisite 5 applies to every crate. |
| OQ4 | `test-helpers` is already published (3.0.0) and is a dev-dependency of six checklist crates and of the root crate. What is its testing gate? | Same as OQ1: not met. This blocks the six checklist crates that dev-depend on it. |

### Facts checked on 2026-10-20

> **Stale on 2026-11-18; T4 owns the update.** `cargo metadata --no-deps` on `develop` now lists 35
> workspace members, 13 with `publish = false` (still 22 publishable). `primitives` is `3.1.0`
> locally (3.0.0 on crates.io) and `axum-health-check-api-server` is `0.2.0`, while the checklist
> says `0.1.0`. `test-helpers` now depends only on `configuration`. Six crates list themselves as
> a path-only dev-dependency (no `version`) to enable their `testing` feature. The facts and the
> checklist's dependency and version columns below are kept as checked on 2026-10-20.

- **Publishable crates.** `cargo metadata --no-deps` lists 32 workspace members. 22 have no
  `publish = false`. The other 10 (`e2e-tools`, `persistence-benchmark`,
  `torrent-repository-benchmarking` and the seven `contrib/dev-tools/` members) are never published.
- **crates.io.** 4 of the 22 are published, all at 3.0.0 and owned by `josecelano` and `da2ce7`:
  `torrust-tracker`, `torrust-tracker-configuration`, `torrust-tracker-primitives` and
  `torrust-tracker-test-helpers`. The other 18 names return `404` from
  `https://crates.io/api/v1/crates/<name>`, so nobody else has taken them. The crates already
  extracted from this workspace (`torrust-clock`, `torrust-metrics` and others) are owned by
  `josecelano` only.
- **Local versions.** The 18 unpublished crates are `0.1.0`. `configuration`, `primitives` and
  `test-helpers` are `3.0.0`, the same number as their published version; the root crate is
  `3.0.0-develop`.
- **Dependencies.** Every path dependency between workspace crates also has a `version`
  requirement, and none points to a `publish = false` crate. `test-helpers` 3.0.0 on crates.io
  depends only on `configuration`; the local `test-helpers` also depends on `client-lib`,
  `http-protocol`, `primitives` and `udp-protocol`.
- **Dev-dependencies must be published first.** `cargo package` resolves dev-dependencies that
  have a `version` requirement against the registry, so a crate's workspace dev-dependencies must
  be on crates.io before it. A path-only dev-dependency (no `version`) would be removed from the
  published manifest instead.
- **Packaging check.** `cargo package --workspace --no-verify` (stable Rust toolchain, cargo
  1.99.0) packages crates in dependency order and fails on
  `torrust-tracker-axum-health-check-api-server`:

  ```text
  error: failed to prepare local package for uploading

  Caused by:
    no matching package named `torrust-tracker-axum-health-check-api-server` found
    location searched: crates.io index
    required by package `torrust-tracker-axum-health-check-api-server v0.1.0 (.../packages/axum-health-check-api-server)`
  ```

  The crate lists itself as a dev-dependency (inherited from before the #1816 rename), so it can
  only be packaged once it is already published. With that line removed locally, the same command
  packages all 32 members; the only warnings are missing metadata on `publish = false` dev tools.
  Removing the self dev-dependency is real pre-publish work, so it gets its own sub-issue (P1, #2582).
  `cargo package -p <crate> --no-verify` on a single unpublished crate fails until its workspace
  dependencies are on crates.io; that is expected and is why the order below matters.

### Semver bumps owed

[Pre-publish API checklist](../../open/1669-overhaul-packages/EPIC.md#pre-publish-api-checklist)
item 4: a crate whose public error enum gained `#[non_exhaustive]` since its last publish gets a
semver-major bump. #2454 marked such enums in `torrust-tracker`, `torrust-tracker-configuration`
and `torrust-tracker-primitives` (see its
[per-crate completion record](../../closed/2454-1669-mark-public-error-enums-non-exhaustive/error-enum-inventory.md#per-crate-completion)),
and removed `configuration::Error::Infallible`. `torrust-tracker-test-helpers` probably owes one
too after #2482. The 18 unpublished crates owe nothing: their first publish starts their history.
The root crate's major bump (3.0.0 to 4.0.0) is recorded here for the tracker release that
publishes it.

## Prerequisites

The first publish of a crate starts only when all of these hold. Prerequisites 1, 2, 3 and 5
apply to every crate; prerequisites 4 and 6 are per crate, so crates become publishable at
different times, always in dependency order (every dependency and dev-dependency first):

1. **#2482 is closed.** The test-support reorganisation can change public APIs (server
   `testing`/`environment` modules, `test-helpers`, `Mock*` types in `tracker-core`), change
   dev-dependencies, and add crates. Crates are published with their final API and dependency
   graph.
2. **#2576 is merged.** crates.io shows each published version's README and `Cargo.toml` metadata
   and they cannot be changed without a new release. #2576 states it must merge before the first
   publish of any unpublished crate.
3. **EPIC #1488 (shutdown overhaul) is closed.** It changes the server environments and shutdown
   API of the server crates.
4. **Per crate: its EPIC #1347 testing gate is met.** The crate's own unit-testing coverage was
   overhauled in EPIC #1347 (package testing): its #1347 sub-issues are closed. The gate is tracked
   in the "Testing (#1347)" column of the [Per-Crate Checklist](#per-crate-checklist). EPIC #1347
   does not need to be closed as a whole (maintainer decision of 2026-11-18). What meets the gate
   for a crate without a sub-issue, and for `test-helpers`, is pending
   ([Open Questions](#open-questions) OQ1 and OQ4).
5. **The rest of EPIC #1669's API work is done**, so this is the EPIC's last publishing step.
6. **Per crate:** the
   [Pre-publish API checklist](../../open/1669-overhaul-packages/EPIC.md#pre-publish-api-checklist)
   is satisfied, and any pre-publish sub-issue linked from the crate's checklist row is closed.

## Scope

### In Scope

- Checking the checklist again when the prerequisites are met (crate set, dependency order, names,
  versions, packaging check, testing gates), because #2482, #1488 and #1347 can change them.
- Bringing the package publish procedure up to date before the first publish (T2).
- For each crate whose version changes: a small pull request that sets its version and every
  dependent's `version` requirement, right before its publish.
- Publishing each crate through `deployment-packages.yaml`, one at a time, in dependency order
  (every dependency and dev-dependency first), with a signed `pkg/<crate>/v<semver>` tag.
- Checking each publish on crates.io and docs.rs, including that the README links #2576 recorded
  as pending (each crate's own docs.rs page) now resolve.
- Making `josecelano` and `da2ce7` the crates.io owners of each newly published crate.
- Opening a sub-issue for each crate that needs real pre-publish work, and linking it from the
  checklist.

### Out of Scope

- Publishing the root `torrust-tracker` crate. The next tracker application release publishes it
  (`deployment.yaml`, `release-new-version` skill) with its owed bump to 4.0.0, after every crate
  in this checklist is published.
- Publishing the console client `torrust-tracker-client` and extracting it to its own repository
  (the client-extraction issue, blocked by this one because `client-lib`, `http-protocol` and
  `udp-protocol` must be on crates.io first).
- Changing any crate's public API, README or metadata beyond the version number (sub-issues,
  #2482 and #2576 own those changes).
- The ten `publish = false` members.
- Publishing a crate twice; a fix after publish is a new patch or minor version.

## Per-Crate Checklist

Order is dependency order, leaves first, counting normal, build and dev-dependencies between
workspace crates (it matches the order `cargo package --workspace` used). Crates in the same level
do not depend on each other and can go in any order within the level. The row and level numbers
only present that order: a crate waits for the crates it depends on or dev-depends on, not for
every crate in an earlier row or level. "Depends on" and
"Dev-depends on" drop the `torrust-tracker-` prefix. The root crate (level 9) is not listed: it
depends on 15 of these crates and is published by the tracker release after all of them.

Status values: `TODO`, `BLOCKED`, `IN_PROGRESS`, `PUBLISHED`, `DEFERRED`.

"Testing (#1347)" is the crate's testing gate (prerequisite 4), checked on 2026-11-18 against the
EPIC #1347 [`Subissues` table](../1347-overhaul-packages-testing/EPIC.md#subissues) and GitHub:

- `MET`: the crate's #1347 sub-issues are closed.
- `OPEN`: the crate has a #1347 sub-issue that is not closed.
- `NONE`: the crate has no #1347 sub-issue. What meets its gate is pending (OQ1; OQ4 for
  `test-helpers`).

How the rows of that table are counted: a crate's sub-issues are the rows about that package's
own tests, which are its "add tests", test-review and coverage rows and their test-support
follow-ups (#2876 for `primitives`). A bug found by the test work (#2838) and the coverage-tooling
rows (#2222, #2301, #2361) belong to no crate and do not count.

A `MET` gate alone does not unblock a row: every crate in its "Depends on" and "Dev-depends on"
columns must be `PUBLISHED` first, and prerequisites 1, 2, 3, 5 and 6 must hold. Update the column
when a #1347 sub-issue is created or closed; EPIC #1347 owns the sub-issues themselves.

| Order | Level | Crate | Depends on | Dev-depends on | Testing (#1347) | Version to publish | Pre-publish notes | Status |
| ----- | ----- | ----- | ---------- | -------------- | --------------- | ------------------ | ----------------- | ------ |
| 1 | 0 | `torrust-tracker-primitives` | - | - | `MET`: [#2824](../../closed/2824-1347-add-tests-primitives/ISSUE.md) and [#2876](../../closed/2876-1347-peer-id-test-helper/ISSUE.md) closed | 4.0.0 | Published 3.0.0. Major owed (#2454). EPIC #2912 requires #2914 (SI-2) to land before 4.0.0 is published. | BLOCKED |
| 2 | 0 | `torrust-tracker-events` | - | - | `NONE`: no sub-issue (OQ1) | 0.1.0 | `RecvError` stays exhaustive on purpose (#2454 E1). | BLOCKED |
| 3 | 0 | `torrust-tracker-http-protocol` | - | - | `NONE`: no sub-issue (OQ1) | 0.1.0 | Blocks the client extraction. | BLOCKED |
| 4 | 0 | `torrust-tracker-udp-protocol` | - | - | `NONE`: no sub-issue (OQ1) | 0.1.0 | `RequestParseError` stays exhaustive on purpose (#2454 E2). Blocks the client extraction. | BLOCKED |
| 5 | 0 | `torrust-tracker-rest-api-protocol` | - | - | `NONE`: no sub-issue (OQ1) | 0.1.0 | - | BLOCKED |
| 6 | 1 | `torrust-tracker-configuration` | primitives | - | `NONE`: no sub-issue (OQ1) | 4.0.0 | Published 3.0.0. Major owed (#2454; `Error::Infallible` removed). EPIC #2912 requires #2917 (SI-5) to land before 4.0.0 is published. | BLOCKED |
| 7 | 1 | `torrust-tracker-swarm-coordination-registry` | events, primitives | - | `NONE`: no sub-issue (OQ1) | 0.1.0 | - | BLOCKED |
| 8 | 1 | `torrust-tracker-client-lib` | http-protocol, udp-protocol | - | `NONE`: no sub-issue (OQ1) | 0.1.0 | LGPL license links fixed by #2576. Blocks the client extraction. | BLOCKED |
| 9 | 1 | `torrust-tracker-rest-api-application` | primitives, rest-api-protocol | - | `NONE`: no sub-issue (OQ1) | 0.1.0 | - | BLOCKED |
| 10 | 1 | `torrust-tracker-rest-api-client` | rest-api-protocol | - | `NONE`: no sub-issue (OQ1) | 0.1.0 | LGPL license links fixed by #2576. | BLOCKED |
| 11 | 2 | `torrust-tracker-axum-server` | configuration | - | `NONE`: no sub-issue (OQ1) | 0.1.0 | - | BLOCKED |
| 12 | 2 | `torrust-tracker-test-helpers` | client-lib, configuration, http-protocol, primitives, udp-protocol | - | `NONE`: no sub-issue (OQ1, OQ4) | 4.0.0 (probably) | Published 3.0.0. Major probably owed after #2482; decided in T4 against the published API. Dev-dependency of 6 crates below and of the root crate. | BLOCKED |
| 13 | 2 | `torrust-tracker-client` | client-lib, http-protocol, udp-protocol | - | - | - | Console CLI. Not published here: its first publish comes from its own repository in the client-extraction issue. | DEFERRED |
| 14 | 3 | `torrust-tracker-core` | configuration, events, primitives, swarm-coordination-registry | test-helpers | `OPEN`: [#2902](../2902-1347-add-tests-tracker-core/ISSUE.md) open (`TODO`) | 0.1.0 | #2482 may change its `Mock*` types. | BLOCKED |
| 15 | 4 | `torrust-tracker-http-core` | configuration, core, events, http-protocol, primitives, swarm-coordination-registry | test-helpers | `MET`: [#1349](../../closed/1349-1347-add-tests-http-core/ISSUE.md) closed | 0.1.0 | - | BLOCKED |
| 16 | 4 | `torrust-tracker-udp-core` | configuration, core, events, primitives, swarm-coordination-registry, udp-protocol | - | `MET`: [#1348](../../closed/1348-1347-add-tests-udp-core/ISSUE.md) closed | 0.1.0 | - | BLOCKED |
| 17 | 5 | `torrust-tracker-axum-http-server` | axum-server, configuration, core, http-core, http-protocol, primitives, swarm-coordination-registry | client-lib, test-helpers | `MET`: [#2136](../../closed/2136-1347-add-tests-axum-http-server/ISSUE.md) and [#2140](../../closed/2140-1347-review-axum-http-server-integration-tests/ISSUE.md) closed | 0.1.0 | #2482 and #1488 may change its public `testing`/`environment` module. | BLOCKED |
| 18 | 5 | `torrust-tracker-udp-server` | client-lib, configuration, core, events, primitives, swarm-coordination-registry, udp-core, udp-protocol | test-helpers | `MET`: [#2149](../../closed/2149-1347-add-focused-udp-server-package-tests/ISSUE.md) and [#2283](../../closed/2283-1347-increase-udp-server-package-coverage/ISSUE.md) closed | 0.1.0 | #2482 and #1488 may change its public `testing`/`environment` module. | BLOCKED |
| 19 | 6 | `torrust-tracker-rest-api-runtime-adapter` | configuration, core, http-core, primitives, rest-api-application, rest-api-protocol, swarm-coordination-registry, udp-core, udp-server | - | `NONE`: no sub-issue (OQ1) | 0.1.0 | - | BLOCKED |
| 20 | 7 | `torrust-tracker-axum-rest-api-server` | axum-server, configuration, core, http-core, primitives, rest-api-application, rest-api-client, rest-api-protocol, rest-api-runtime-adapter, swarm-coordination-registry, udp-core, udp-server | rest-api-client, test-helpers | `MET`: [#2637](../../closed/2637-1347-add-tests-axum-rest-api-server/ISSUE.md) closed | 0.1.0 | #2482 and #1488 may change its public `testing`/`environment` module. | BLOCKED |
| 21 | 8 | `torrust-tracker-axum-health-check-api-server` | axum-server, configuration, primitives | axum-http-server, axum-rest-api-server, test-helpers, udp-server | `NONE`: no sub-issue (OQ1) | 0.1.0 | The self dev-dependency is removed (sub-issue P1, #2582, closed). Its four workspace dev-dependencies go first. | BLOCKED |

### Pre-Publish Sub-Issues

| ID | Crate | Work | Issue | Status |
| -- | ----- | ---- | ----- | ------ |
| P1 | `torrust-tracker-axum-health-check-api-server` | Remove the self dev-dependency. Its tests pass without it (checked 2026-10-20: 3 unit and 8 integration tests). | [#2582](../../closed/2582-1669-remove-health-check-self-dev-dependency/ISSUE.md) | DONE |

## Per-Crate Publish Procedure

For each crate, in dependency order (every dependency and dev-dependency first), follow
[Publishing a Workspace Package](../../../release-process/workspace-packages.md#publishing-a-workspace-package)
(the `deployment-packages.yaml` path), as updated by T2. In short, without repeating that
document:

1. Confirm its testing gate is `MET` in the "Testing (#1347)" column (check the sub-issue state
   on GitHub again), every crate it depends on or dev-depends on is `PUBLISHED` in this checklist
   and indexed on crates.io, and its pre-publish sub-issues are closed.
2. If its version changes, open a small pull request to `develop` that sets it and every
   dependent's `version` requirement, and wait for the merge.
3. `cargo package -p <crate>` passes (it now resolves its dependencies from crates.io).
4. Push `releases/pkg/<crate>/v<semver>` from `develop`; `deployment-packages.yaml` tests and
   publishes it. Then push the signed `pkg/<crate>/v<semver>` tag.
5. For a new crate, add the missing one of `josecelano` and `da2ce7` with
   `cargo owner --add <login> <crate>`; the account behind the publishing token becomes the first
   owner.
6. Check the crate page on crates.io (version, README, metadata, license, owners) and its docs.rs
   build, then set the row to `PUBLISHED` with the date and add a Progress Log entry.

## Architectural Decisions

- Related ADRs:
  [`20260629000000_adopt_independent_package_versioning.md`](../../../adrs/20260629000000_adopt_independent_package_versioning.md)
  (independent versions, one crate per publish, branch and tag conventions) and
  [`20261005145329_return_result_only_for_concretely_fallible_public_apis.md`](../../../adrs/20261005145329_return_result_only_for_concretely_fallible_public_apis.md)
  (the error-API rules behind the Pre-publish API checklist).
- ADRs to create: none known.

## Design and Ownership Review

Not applicable: release work, no runtime code.

## Bug-Fix Process

Not applicable to this issue. The `cargo package` failure is fixed in sub-issue P1.

## Regression Test Strategy

Not applicable: no runtime behaviour changes. Each publish is checked by `cargo package`, the
`deployment-packages.yaml` test job, and the crates.io and docs.rs checks below.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID | Status | Task | Notes / Expected Output |
| -- | ------ | ---- | ----------------------- |
| T1 | DONE | Open pre-publish sub-issue P1 | #2582 created with this issue on 2026-10-20, linked to #1669 and to its checklist row. It does not wait for the prerequisites. |
| T2 | TODO | Bring the publish procedure up to date | Not blocked. The release-process spec #2808 already fixed the pre-publish defects: the `pkg/...` push from `torrust/main` is gone, the tag is created on the published commit, and the `release-new-version` crate list and every-package version bump are gone. What remains for this issue is to confirm the procedure in `docs/release-process/workspace-packages.md` fits the first publish (publish order, and each crate's version together with its dependents' `version` requirements in one commit), and to fix any gap found there. |
| T3 | BLOCKED | Wait for the prerequisites | For every crate: #2482 closed (done), #2576 merged (done), #1488 closed, the rest of the EPIC's API work done. Per crate: its "Testing (#1347)" gate `MET` and its dependencies `PUBLISHED`; #2582 closed (done) before row 21. EPIC #1347 does not need to close as a whole. |
| T4 | BLOCKED | Check the checklist again | Re-run the checks in "Facts checked" (crate set, order, crates.io names, versions, `cargo package --workspace --no-verify`) and update the checklist; record them in `manual-verification-evidence.md`. Decide the `test-helpers` version against its published 3.0.0 API. |
| T5 | BLOCKED | Publish each crate in dependency order | One crate at a time with the per-crate procedure; each row `PUBLISHED` with date and evidence. Dependency order means every dependency and dev-dependency first, not row or level order. A crate is published when its own gates are met; the task does not wait for every crate's testing gate before the first publish. |
| T6 | BLOCKED | Hand over | Tell the client-extraction issue (#2594) that `client-lib`, `http-protocol` and `udp-protocol` are published, and the next tracker release that every library crate is published and the root crate owes 4.0.0. |
| T7 | BLOCKED | Verify and review acceptance criteria | Automatic checks and manual scenarios recorded; acceptance criteria re-reviewed. |

## Commit Points

| Task | Coherent change set | Commit policy |
| ---- | ------------------- | ------------- |
| T1 | P1 issue number in this spec and the EPIC | One `docs(issues)` commit. |
| T2 | Publish procedure in `docs/release-process/workspace-packages.md` and the `release-new-version` skill | One `docs(release)` commit, in its own pull request. |
| T4 | Checklist check and evidence | One `docs(issues)` commit. |
| T5 | One crate's version and its dependents' `version` requirements | One `chore(<package>)` commit in a pull request for that crate. |
| T5 | One crate's row set to `PUBLISHED`, with evidence | One `docs(issues)` commit per crate. |
| T7 | Final evidence and acceptance review | One `docs(issues)` commit. |

Every commit is a signed Conventional Commit. Publishing itself creates no commit; it pushes a
release branch and a signed tag.

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/drafts/1669-first-publish-of-workspace-crates/ISSUE.md`
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue created and issue number added to this spec
- [ ] Spec-only PR merged into `develop` before implementation
- [ ] #2482 and #1488 closed and #2576 merged before the first publish; each crate's #1347 testing gate `MET` before that crate's publish
- [ ] Implementation completed (every checklist row `PUBLISHED` or `DEFERRED` with a reason)
- [ ] Automatic verification completed (`linter all`, `cargo package`, `deployment-packages.yaml` runs)
- [ ] Manual verification scenarios executed and recorded in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] Evidence-based implementation completion review recorded: issue-local retrospective created for material discoveries, or progress log states why none was needed
- [ ] Reviewer validated acceptance criteria and updated checkboxes
- [ ] Independent reviewer reports recorded in issue-local `agent-review-reports.md` when reviewers received this folder-style specification
- [ ] Committer verified spec progress is up to date before commit
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-10-20 17:20 UTC - GitHub Copilot (Claude Opus 5.5) - Drafted the spec from the maintainer's
  2026-10-20 publishing decisions. Checked `cargo metadata` (22 publishable crates, 4 on
  crates.io), the crates.io API (the 18 other names are free), the dependency order including
  dev-dependencies, and `cargo package --workspace --no-verify` (fails only on the
  `axum-health-check-api-server` self dev-dependency; proposed sub-issue P1).
- 2026-10-20 17:37 UTC - GitHub Copilot (Claude Opus 5.5) - Applied the maintainer's answers (see
  Maintainer Decisions): first versions stay `0.1.0`; the root crate and the console client are out
  of scope; one pull request per version bump; owners `josecelano` and `da2ce7`; #1488 and #1347
  added as prerequisites; P1 drafted with this issue; T2 fixes the stale publish procedure.
  Corrected the member count to 32 (10 with `publish = false`).
- 2026-10-20 17:41 UTC - GitHub Copilot (Claude Opus 5.5) - The maintainer approved this spec and P1 and
  chose a CI packaging check for P1. Created #2583 (labels `task`, `packages`, `Blocked`) and P1 as
  #2582 (labels `Bug`, `packages`), linked both as subissues of #1669, and moved both specs to
  `docs/issues/open/` for one spec-only PR on branch
  `2583-1669-first-publish-of-workspace-crates-spec`.
- 2026-10-21 07:44 UTC - GitHub Copilot (Claude Opus 5.5) - The client-extraction issue is #2594, blocked by this
  issue on GitHub. With the maintainer's approval, T6 and the References name it and link its
  spec instead of the removed draft.
- 2026-11-11 12:34 UTC - Claude Code (Claude Sonnet 5.5) - PR torrust/torrust-tracker#2825 (issue #2808) re-scoped T2 to what the release-process spec #2808 left (the `torrust/main` push, the old step 6 and the `release-new-version` crate list were fixed there).
- 2026-11-18 09:47 UTC - Claude Code (Claude Fable 5.2) - Applied the maintainer's 2026-11-18 decision:
  the EPIC #1347 prerequisite is per crate (see Maintainer Decisions). Prerequisite 4, T3, T5, the
  procedure's step 1, AC7 and the workflow checkpoint changed; the checklist gained the
  "Testing (#1347)" column, filled from the EPIC #1347 `Subissues` table and the GitHub state of
  each sub-issue (6 crates `MET`, `core` `OPEN`, 13 `NONE`). Four questions are recorded as pending
  maintainer decisions (Open Questions OQ1-OQ4). Noted, without fixing them, the "Facts checked on
  2026-10-20" items that are stale on `develop` (T4 owns the update). Spec-only change; no crate's
  status changed.

## Acceptance Criteria

- [ ] AC1: Every checklist crate is `PUBLISHED` on crates.io at the version in its row, or
      `DEFERRED` with a recorded reason and owner.
- [ ] AC2: Crates were published one at a time, each after every crate it depends on or
      dev-depends on.
- [ ] AC3: `torrust-tracker-configuration` and `torrust-tracker-primitives` were published with a
      semver-major bump, and `torrust-tracker-test-helpers` with the bump decided in T4.
- [ ] AC4: Each published crate has a signed `pkg/<crate>/v<semver>` tag and a successful
      `deployment-packages.yaml` run.
- [ ] AC5: Each published crate's crates.io page shows the #2576 README and metadata, its docs.rs
      build succeeded, and its README's docs.rs link resolves.
- [ ] AC6: Each newly published crate is owned on crates.io by `josecelano` and `da2ce7`.
- [ ] AC7: No crate was published before #2482 and #1488 closed and #2576 merged, nor before its
      own #1347 testing gate was `MET`.
- [ ] AC8: The package publish procedure in `docs/release-process/workspace-packages.md` and the
      `release-new-version` skill match the ADR and both workflows before the first publish (T2).
- [ ] `linter all` exits with code `0`
- [ ] Manual verification scenarios are executed and documented in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria are re-reviewed after implementation and reflect actual behavior
- [ ] Documentation is updated when behavior/workflow changes

## Verification Plan

### Automatic Checks

- `linter all` for each spec and evidence commit.
- `cargo package --workspace --no-verify` (stable Rust toolchain) at T4, and
  `cargo package -p <crate>` (with verification) before each publish.
- The `deployment-packages.yaml` test and publish jobs for each crate.
- `cargo build --workspace` and `cargo test --doc --workspace` after each version-bump commit.

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID | Scenario                          | Human-oriented command/steps                                                                                                       | Expected Result                                                                 | Status  | Evidence                                     |
| -- | --------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------- | ------- | -------------------------------------------- |
| M1 | Check again before the first publish | Run the "Facts checked" commands again: `cargo metadata --no-deps`, `curl https://crates.io/api/v1/crates/<name>` for each crate, `cargo package --workspace --no-verify` | Checklist matches the tree; every unpublished name is still free; packaging passes | BLOCKED | `manual-verification-evidence.md` section V1 |
| M2 | Each crate as a crates.io user sees it | After each publish open `https://crates.io/crates/<crate>` and `https://docs.rs/<crate>`; check version, README, license, owners (`cargo owner --list <crate>`) | Matches the checklist row and AC5/AC6                                           | BLOCKED | `manual-verification-evidence.md` section V2 |
| M3 | Use a published crate from outside the workspace | In a scratch crate outside the repository, `cargo add <crate>@<version>` for a leaf (`torrust-tracker-udp-protocol`) and a top crate (`torrust-tracker-core`), then `cargo build` | Builds from crates.io alone                                                     | BLOCKED | `manual-verification-evidence.md` section V3 |

Notes:

- Manual verification is mandatory even when automated checks pass.
- Record the toolchain for each command (for example `stable Rust toolchain` for `cargo package`).
- Create `manual-verification-evidence.md` from `docs/templates/MANUAL-VERIFICATION-EVIDENCE.md`.
- If a scenario fails, record the failure and diagnosis in the progress log before proceeding.

### Disposable Verification Scripts

None planned. The crates.io name check is a `curl` loop recorded in the evidence.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| ----- | ---------------------- | -------- |
| AC1   | TODO                   |          |
| AC2   | TODO                   |          |
| AC3   | TODO                   |          |
| AC4   | TODO                   |          |
| AC5   | TODO                   |          |
| AC6   | TODO                   |          |
| AC7   | TODO                   |          |
| AC8   | TODO                   |          |

## Risks and Trade-offs

- **Irreversible.** A published version cannot be changed or deleted, only yanked. Mitigation: the
  prerequisites, a `cargo package` check per crate, and one crate at a time.
- **Stale order.** #2482, #1488 and #1347 can add crates or change dependencies. Mitigation: T4
  checks the checklist again before the first publish.
- **Publishing spread over time.** With a per-crate testing gate the workspace stays partly
  published for longer, and a crate published early can still change before its dependents are
  published, which costs it a new version. Mitigation: dependency order, and prerequisites 3 and 5
  still apply to every crate (OQ2 and OQ3 decide whether that stays).
- **Long tail.** 20 publishes, one at a time, can take days; crates.io also rate-limits how many new
  crates one account can publish in a short time. Mitigation: nothing depends on publishing fast;
  the checklist records where the work stopped.
- **Half-published workspace.** If a crate fails mid-way, its dependents wait. Mitigation: the
  failing crate gets a sub-issue; crates already published stay valid.
- **Name squatting.** The 18 names are free today but not reserved. Mitigation: T4 re-checks them;
  reserving them early would publish placeholder crates, which this issue does not do.

## Implementation Completion Review

- Retrospective: `Not yet assessed`
- If needed, create `implementation-retrospective.md` from
  `docs/templates/IMPLEMENTATION-RETROSPECTIVE.md` in this directory.
- If no retrospective is needed, add a concise progress-log entry explaining why.
- Independent reviewers record results in `agent-review-reports.md` using
  `docs/templates/AGENT-REVIEW-REPORTS.md`.

## References

- Parent EPIC: #1669, [`EPIC.md`](../../open/1669-overhaul-packages/EPIC.md)
- Prerequisites: #2482
  ([`EPIC.md`](../../closed/2482-1669-reorganize-shared-test-support/EPIC.md)), #2576
  ([`ISSUE.md`](../../closed/2576-1669-update-all-package-readmes/ISSUE.md)), #1488
  ([`ISSUE.md`](../../open/1488-overhaul-tracker-shutdown/ISSUE.md)), #1347
  ([`EPIC.md`](../../open/1347-overhaul-packages-testing/EPIC.md))
- Semver input: #2454
  ([error enum inventory](../../closed/2454-1669-mark-public-error-enums-non-exhaustive/error-enum-inventory.md))
- Pre-publish sub-issue: #2582
  ([`ISSUE.md`](../../closed/2582-1669-remove-health-check-self-dev-dependency/ISSUE.md))
- Blocks: #2594, the client extraction
  ([`ISSUE.md`](../2594-1669-extract-torrust-tracker-client-to-standalone-repo/ISSUE.md))
- Procedure: [`docs/release-process/workspace-packages.md`](../../../release-process/workspace-packages.md#publishing-a-workspace-package);
  the root crate's tracker release: `release-new-version` skill
- Cargo reference: [Publishing on crates.io](https://doc.rust-lang.org/cargo/reference/publishing.html)
