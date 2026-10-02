---
schema-version: 1
doc-type: epic
status: planned
epic: null
github-issue: 2392
spec-path: docs/issues/open/2392-test-isolation/EPIC.md
epic-owner: josecelano
last-updated-utc: "2026-10-02 13:30"
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - .github/skills/dev/planning/create-issue/SKILL.md
    - docs/issues/open/1419-allow-multiple-integration-tests-at-main-app-level/ISSUE.md
    - docs/issues/open/2393-2392-test-port-allocation-races/ISSUE.md
    - docs/issues/open/2394-2392-audit-shared-test-resources/ISSUE.md
    - docs/issues/open/2395-2392-e2e-runner-dynamic-host-ports/ISSUE.md
    - docs/issues/open/2396-2392-e2e-unique-tracker-image-tag/ISSUE.md
    - docs/issues/open/1840-improve-pr-workflow-performance-epic/EPIC.md
    - docs/issues/closed/2386-1840-self-hosted-runner-minimum-capacity/ISSUE.md
    - docs/issues/closed/2132-add-sigterm-to-main/native-shutdown-test-plan.md
---

<!-- skill-link: create-issue -->

# EPIC #2392 - Test Isolation

Related EPIC: #1840 - Improve PR Workflow Performance (related, not parent)

## Goal

Every automated test is independent of every other test and of other test runs in the same
environment (one host, one Docker daemon). Each test owns the resources it uses: network ports,
container images and names, filesystem paths, and process-global state. A test result then depends
only on the code under test, not on what else happens to be running.

The purpose is a robust test suite without flaky tests. Faster CI through parallel runs is a side
benefit, not the goal.

## Why This Is Needed

When tests share a resource, one test can break another, or silently test the wrong thing:

- **Flaky local and CI runs.** The full nextest suite fails intermittently with `AddrInUse`,
  because fixtures release a selected port before using it or claim fixed ports in the ephemeral
  range (reproduced locally; also seen in the #2298 container build).
- **Colliding E2E runs.** The E2E runner publishes fixed host ports, so a second run on the same
  Docker host fails before testing anything.
- **Wrong results without a failure.** CI tags each run's image with a fixed name
  (`torrust-tracker:local`, `torrust-tracker:e2e-local`). Two runs on one Docker host can overwrite
  the tag, so one run can test the other run's image and still pass.
- **Unknown shared resources.** #2386 did not audit fixed container names, networks, or `/tmp`
  paths; process-global state (environment variables, global subscribers, statics) has no
  inventory either.
- **Eroded trust.** Failures unrelated to the tested behavior teach contributors to rerun instead
  of investigate, which hides real regressions.

Isolation also improves the code itself: a component that must work with an OS-assigned port has
to report the address it bound (observability), and a component that accepts injected paths and
names instead of globals is easier to test in isolation (testability).

Some suites are already isolated: #1419 moved the main-application integration suites to port `0`
and temporary workspaces, #2132 designed the native-process fixture around port `0` plus
startup-log discovery, and the qBittorrent E2E runner uses random Compose project names and
Docker-assigned host ports. This EPIC applies the same approach everywhere else.

## Scope

### In Scope

- Network ports: fixed ports, and ports selected then released before use.
- Container resources: image tags, container, network, and Compose project names.
- Filesystem paths: shared storage, configuration, database, and temporary paths.
- Process-global state shared between tests in one process: environment variables, global
  subscribers, and statics.
- An inventory of shared test resources, with a disposition for each.
- Isolation rules documented where contributors write tests.
- Tests whose contract requires a fixed resource (for example fixed-port listeners), which stay
  but are documented as deliberate exceptions.

### Out of Scope

- Production default ports and deployment configuration.
- The shared Cargo target directory between runner instances (#2386). Cargo's lock keeps concurrent
  builds correct; they only wait for each other, which is a speed concern for EPIC #1840.
- Adding runner instances or changing the self-hosted runner setup (#2386, EPIC #1840).
- Cooperative shutdown of in-process tracker jobs (#1488).
- Speeding up the test suite for its own sake (EPIC #1840).

## Relationship to EPIC #1840

EPIC #1840 makes pull-request checks faster. Isolation is a prerequisite for one of its options,
running several jobs per self-hosted server (#2386), but this EPIC is driven by robustness and
flaky-test prevention and does not depend on #1840. The two EPICs are related, not nested.

## Subissues

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| Order | Issue | Local Spec | Status | Notes |
| ----- | ----- | ---------- | ------ | ----- |
| 1 | #2393 - Eliminate parallel test port allocation races | `docs/issues/open/2393-2392-test-port-allocation-races/ISSUE.md` | TODO | Bug. Reproduced flaky `AddrInUse` in the nextest suite. First because it breaks normal local and CI runs today. |
| 2 | #2394 - Audit shared test resources | `docs/issues/open/2394-2392-audit-shared-test-resources/ISSUE.md` | TODO | Task. Inventories the resource classes in scope, records a disposition for each finding, and drafts subissues for confirmed problems. |
| 3 | #2395 - Use dynamic host ports in the E2E runner | `docs/issues/open/2395-2392-e2e-runner-dynamic-host-ports/ISSUE.md` | TODO | Task. Removes fixed E2E host ports. Independent of orders 1 and 2. |
| 4 | #2396 - Use a unique tracker image tag per CI run | `docs/issues/open/2396-2392-e2e-unique-tracker-image-tag/ISSUE.md` | TODO | Task. Prevents one run from testing another run's image. Latent today (one runner instance per server). Independent of order 3. |
| 5 | #1419 - Allow multiple integration tests at the main app level | `docs/issues/open/1419-allow-multiple-integration-tests-at-main-app-level/ISSUE.md` | BLOCKED | Existing issue, linked as a subissue. Isolation work is done; the remaining AC8a waits on #1488. |

## Delivery Strategy

Deliver order 1 first: it fixes failures contributors already hit. Start order 2 early, because
its findings may add subissues. Orders 3 and 4 touch only the E2E runners and CI workflows and can
proceed in parallel. #1419 closes on its own schedule after #1488.

Test isolation rules (apply to every subissue):

1. A test owns every resource it uses and releases it on success, failure, and drop paths.
2. Prefer OS-assigned ports (port `0`) and read the bound address from the service (registry,
   startup log, `docker port`, or `docker compose port`).
3. Never select a port by binding and releasing it before the service under test binds it.
4. Give per-run names to shared-namespace resources: image tags, containers, networks, Compose
   projects, and paths (use `tempfile::TempDir` for paths).
5. Do not change process-global state from a test that shares its process with other tests; pass
   configuration explicitly instead.
6. When a test contract needs a fixed resource, document why in the test. Fixed ports must be
   outside the ephemeral range (`32768`-`60999` by default on Linux).

For each subissue implementation in this EPIC, the default completion policy is:

1. Run automatic checks (`linter all`, relevant tests, pre-push checks when applicable).
2. Run manual verification scenarios and record evidence.
3. Re-review acceptance criteria after implementation and update verification evidence.
4. Complete an evidence-based implementation review. Create or update an issue-local
   retrospective for reusable lessons, material design changes, or meaningful deviations from the
   plan; otherwise record why one was unnecessary in the issue progress log.

### Phase 1 - Stop known flakiness

- Outcome: the full nextest suite no longer fails with port-allocation `AddrInUse`.
- Exit criteria: order 1 is done, including its repeated full-suite recheck.

### Phase 2 - Know the remaining shared resources

- Outcome: every shared test resource in scope is inventoried with a disposition.
- Exit criteria: order 2 is done and any confirmed problems have subissues in this EPIC.

### Phase 3 - Isolated E2E runs

- Outcome: two E2E runs on one Docker host neither collide nor test each other's image.
- Exit criteria: orders 3 and 4 are done, and `docs/self-hosted-runner.md` no longer lists fixed
  host ports or the shared image tag as concurrency limits.

### Phase 4 - Document and close

- Outcome: the test isolation rules are documented for contributors (for example in
  `docs/testing.md` and the `write-unit-test` skill), and #1419 is closed or its remaining
  non-isolation work is tracked outside this EPIC.
- Exit criteria: all subissues are done or explicitly handed off.

## Progress Tracking

### Workflow Checkpoints

- [x] Epic spec drafted in `docs/issues/drafts/`
- [x] Epic spec reviewed and approved by user/maintainer
- [x] GitHub epic issue created and issue number added to this spec
- [x] Subissues created and linked in this spec (including #1419 as an existing subissue)
- [ ] Subissue statuses kept up to date in the `Subissues` table
- [ ] For each implemented subissue: automatic checks completed and recorded
- [ ] For each implemented subissue: manual verification completed and recorded
- [ ] For each implemented subissue: acceptance criteria reviewed post-implementation
- [ ] For each implemented subissue: implementation completion review recorded
- [ ] Epic acceptance criteria reviewed and checked off
- [ ] Epic issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-09-30 13:38 UTC - GitHub Copilot - Drafted the EPIC after a local reproduction of flaky
  `AddrInUse` failures, grouping the new port-race bug, a new E2E host-port task, and existing
  #1419. #2386 anticipated this EPIC.
- 2026-09-30 14:38 UTC - josecelano, GitHub Copilot - Broadened the scope from port conflicts to
  test isolation, driven by robustness and flaky-test prevention rather than speed. Added the
  shared-resource audit and unique image tag subissues; related (not parented) the EPIC to #1840.
- 2026-09-30 14:58 UTC - GitHub Copilot - Maintainer approved the drafts. Created EPIC #2392 and
  subissues #2393-#2396, linked them and existing #1419 as GitHub sub-issues, and moved the
  specifications to `docs/issues/open/`.

## Acceptance Criteria

- [x] AC1: All required subissues are created and linked, and #1419 is linked as an existing
  subissue.
- [ ] AC2: The full nextest suite runs repeatedly without failures caused by shared resources.
- [ ] AC3: Two E2E runs on one Docker host run concurrently, and each tests its own image.
- [ ] AC4: The shared-resource audit is complete; every finding is isolated, a documented
  deliberate exception, or tracked by a subissue.
- [ ] AC5: The test isolation rules are documented where contributors write tests.
- [ ] Every completed subissue includes automated and manual verification evidence, a
  post-implementation acceptance criteria review, and an implementation completion review.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| ----- | ---------------------- | -------- |
| AC1 | DONE | GitHub sub-issues of #2392: #2393, #2394, #2395, #2396, #1419 |
| AC2 | TODO | Order 1 `manual-verification-evidence.md` |
| AC3 | TODO | Orders 3 and 4 `manual-verification-evidence.md` |
| AC4 | TODO | Order 2 inventory |
| AC5 | TODO | Documentation PR |

## Risks and Trade-offs

- **Dynamic resources need discovery.** Each test must read back what it was assigned. Reuse
  existing mechanisms (runtime registry, startup log, `docker port`) rather than adding new ones.
- **Some contracts need fixed resources.** For example, `tests/metrics/fixed_ports.rs` deliberately
  covers fixed-port listeners, and the CLI precedence test needs distinct predictable ports. These
  stay fixed but are documented and kept outside the ephemeral range.
- **Scope growth.** A broad isolation EPIC can absorb unrelated test work. Only resources shared
  between tests or runs belong here; coverage work stays in EPIC #1347.
- **Probabilistic evidence.** Repeated runs only lower the chance of a missed race; the isolation
  rules are the real guarantee.

## References

- Related EPICs: #1840 (PR workflow performance), #1347 (package testing)
- Related issues: #1419, #2386, #2132, #2298, #1488
- Related ADRs: none
