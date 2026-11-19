---
schema-version: 1
doc-type: epic
status: in-progress
epic: null
github-issue: 2392
spec-path: docs/issues/open/2392-test-isolation/EPIC.md
epic-owner: josecelano
last-updated-utc: "2026-11-19 08:25"
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - .claude/skills/create-issue/SKILL.md
    - docs/issues/open/1419-allow-multiple-integration-tests-at-main-app-level/ISSUE.md
    - docs/issues/closed/2393-2392-test-port-allocation-races/ISSUE.md
    - docs/issues/closed/2394-2392-audit-shared-test-resources/ISSUE.md
    - docs/issues/open/2395-2392-e2e-runner-dynamic-host-ports/ISSUE.md
    - docs/issues/open/2396-2392-e2e-unique-tracker-image-tag/ISSUE.md
    - docs/issues/open/2906-2392-fixed-port-metrics-test-concurrent-runs/ISSUE.md
    - docs/issues/closed/2907-2392-remove-leaked-test-temporary-files/ISSUE.md
    - docs/issues/open/2908-2392-compose-cleanup-after-failed-start/ISSUE.md
    - docs/issues/open/2924-2392-sqlite-test-database-guard/ISSUE.md
    - docs/issues/open/2925-2392-migrate-tests-to-guarded-sqlite-database/ISSUE.md
    - docs/issues/open/2943-2392-figment-jail-process-environment/ISSUE.md
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
| 1 | #2393 - Eliminate parallel test port allocation races | `docs/issues/closed/2393-2392-test-port-allocation-races/ISSUE.md` | DONE | Bug. PR #2859 ignored 21 racing tests and rewrote 4 that never needed a racing port. PR #2874 applied the T5 decisions: deleted 9 tests of test-support code, rewrote the 3 health-check contract tests to probe an address the test holds, and restored 9 socket-release tests on a unique loopback address, picked at random from `127.0.0.0/8` (ignored off Linux). The full suite then passed 150 of 150 runs with no #2393 test ignored (V6). Found during the work: #2867 (UDP health check panics on a malformed reply). |
| 2 | #2394 - Audit shared test resources | `docs/issues/closed/2394-2392-audit-shared-test-resources/ISSUE.md` | DONE | Task. PR #2891 recorded 38 findings in `inventory.md`: 29 isolated, 1 deliberate exception (P3), 5 covered (P1, P9, C1, C2, C3; C3 through order 4, whose scope now includes the local default image tags) and 3 new problems, now orders 6 to 8: D1 (P2) is #2906, D2 (F2) is #2907 and D3 (C9) is #2908. The close-out PR #2898 has merged; GitHub issue #2394 is still open until the maintainer closes it. |
| 3 | #2395 - Use dynamic host ports in the E2E runner | `docs/issues/open/2395-2392-e2e-runner-dynamic-host-ports/ISSUE.md` | TODO | Task. Removes fixed E2E host ports. Independent of orders 1 and 2. |
| 4 | #2396 - Use a unique tracker image tag per CI run | `docs/issues/open/2396-2392-e2e-unique-tracker-image-tag/ISSUE.md` | TODO | Task. Prevents one run from testing another run's image. Latent today (one runner instance per server). Independent of order 3. |
| 5 | #1419 - Allow multiple integration tests at the main app level | `docs/issues/open/1419-allow-multiple-integration-tests-at-main-app-level/ISSUE.md` | IN_PROGRESS | Existing issue, linked as a subissue. Isolation work is done. EPIC #1488 closed on 2026-11-19, so the remaining R8, AC8a and R9 (review the fixture against the final shutdown API, prove cooperative completion, then the closure review) are unblocked and in progress. |
| 6 | #2906 - Let two runs of the fixed-port metrics test share a machine | `docs/issues/open/2906-2392-fixed-port-metrics-test-concurrent-runs/ISSUE.md` | TODO | Bug (#2394 D1, P2). `tests/metrics/fixed_ports.rs` binds `0.0.0.0:17091`-`17094`, so two runs on one machine collide (20 of 20 paired runs had one failure). The proposed fix uses the loopback helper beyond the 9 #2393 tests, which needs a maintainer decision before implementation starts (the Decisions section limits the helper to those 9 tests). |
| 7 | #2907 - Remove the temporary TOML and PEM files tests leave behind | `docs/issues/closed/2907-2392-remove-leaked-test-temporary-files/ISSUE.md` | DONE | Bug (#2394 D2, F2). Narrowed on 2026-11-17 to the `test_config_*.toml` and PEM files that tests leave in the system temporary directory. The `data_*.db` files, 301 of the 303 files one full run leaves, moved to orders 9 and 10. PR #2938 moved the two TOML tests into a `TempDir` and made the TLS helper return a `NamedTempFile`; after it, a full run leaves 0 TOML and 0 PEM files (V2). Found during the work: 18 `figment::Jail` tests in `packages/configuration/src/lib.rs` call `clear_env()`, which removes `TMPDIR` from the whole process while they run, so leaked-file counts are lower bounds (no issue yet). |
| 8 | #2908 - Tear down the qBittorrent E2E stack when it fails to start | `docs/issues/open/2908-2392-compose-cleanup-after-failed-start/ISSUE.md` | TODO | Bug (#2394 D3, C9). A failed `docker compose up` leaves the stack running on the Docker host. Touches the qBittorrent E2E runner, like order 4. |
| 9 | #2924 - Add a guard-owned SQLite test database | `docs/issues/open/2924-2392-sqlite-test-database-guard/ISSUE.md` | TODO | Bug, split from order 7. Adds to the test helpers a guard that owns the SQLite database's directory and a factory that returns a configuration with its guard, next to the existing `ephemeral*()` functions; converts the helpers outside `tracker-core` that build the database path themselves. PR #2923 (#2902) fixes `tracker-core` with the same shape, and order 9 starts after it merges. |
| 10 | #2925 - Move tests to the guard-owned SQLite database and default to no database | `docs/issues/open/2925-2392-migrate-tests-to-guarded-sqlite-database/ISSUE.md` | TODO | Bug, split from order 7. Moves the tests that need persistence to order 9's factory, one pull request per package, then removes the database from `ephemeral*()`. Owns the check that a fresh `TMPDIR` is empty after the suite. Depends on order 9. |
| 11 | #2943 - Keep `figment::Jail` tests from changing the environment other tests see | `docs/issues/open/2943-2392-figment-jail-process-environment/ISSUE.md` | TODO | Bug, found by #2907. The `Jail` tests of `packages/configuration` clear and set the process environment and change the current directory while tests of the same binary that do not use `Jail` run. The order 2 audit classified this resource (G2) as isolated; this issue corrects that row and rechecks G1. The maintainer chooses the design in its T2. Independent of the other orders. |

## Delivery Strategy

Deliver order 1 first: it fixes failures contributors already hit. Start order 2 early, because
its findings may add subissues. Orders 3 and 4 touch only the E2E runners and CI workflows and can
proceed in parallel. #1419 waited on #1488, which closed on 2026-11-19; its remaining work is
independent of the other orders. Order 2 added orders 6 to 8:
orders 6 and 7 make the suite fail on a shared machine and gate AC2; they are independent of each
other and of the E2E orders. Order 8 touches the qBittorrent E2E runner and belongs to Phase 3.
Order 7 was split: it keeps the TOML and PEM files, and orders 9 and 10 take the SQLite files, in
that order (10 depends on 9). Order 7 is independent of both. Orders 9 and 10 gate AC2 as well.
Order 11 was found while order 7 was implemented. It is independent of every other order. It does
not gate AC2 today, because no test has been seen to fail from it; if its T1 finds one, this
section and AC2 are updated.

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

Racing port shapes found by #2393 (its spec's Inventory section has the per-test table):

- **Bind, drop, reuse.** The test binds port `0`, reads the address, drops the socket and hands
  the address to the server (rule 3).
- **Release, then rebind.** The test stops or fails a server that bound port `0`, then rebinds the
  released port to prove it was released. A concurrent port-`0` bind can receive that port.
- **Released port handed back within one test.** A test stops a service and starts another on port
  `0`; the kernel can hand back the released port while the registry still holds it.
- **Released port probed and expected dead.** A test stops a service and expects nothing to answer
  on its released port; another test can bind that port and answer. Three of the 21 ignored tests
  (the health-check contract tests) have this shape.

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
- State: done. PR #2859 stopped the races by ignoring 21 tests, and PR #2874 applied the T5
  decisions on them; the full suite then passed 150 of 150 runs with no #2393 test ignored (#2393
  V6).

### Phase 2 - Know the remaining shared resources

- Outcome: every shared test resource in scope is inventoried with a disposition.
- Exit criteria: order 2 is done and any confirmed problems have subissues in this EPIC.
- State: done. Order 2's inventory has 38 findings; its 3 new problems are orders 6 to 8.

### Phase 3 - Isolated E2E runs

- Outcome: two E2E runs on one Docker host neither collide nor test each other's image.
- Exit criteria: orders 3, 4 and 8 are done, and `docs/self-hosted-runner.md` no longer lists fixed
  host ports or the shared image tag as concurrency limits.

### Phase 4 - Document and close

- Outcome: the test isolation rules are documented for contributors (for example in
  `docs/testing.md` and the `write-unit-test` skill), and #1419 is closed or its remaining
  non-isolation work is tracked outside this EPIC.
- Exit criteria: all subissues are done or explicitly handed off.

## Decisions

Decisions taken while delivering the EPIC. They apply to every subissue.

- 2026-11-14 (maintainer): a test that races on a shared resource is disabled first with
  `#[ignore = "<reason>, see #<issue>"]`, then deleted, rewritten, or kept disabled with a recorded
  reason. A flaky test is worse for development than a missing one, and some tests that need a
  fixed or pre-selected resource may not be worth having.
- 2026-11-14 (maintainer): a server that takes over an already-bound socket (listener handoff) was
  considered for #2393 and rejected for now, because it changes the production start APIs of four
  server crates. #2393 T5 restores its ignored release tests without it (next decisions).
- 2026-11-14 (maintainer): no new shared port helper. A random port drawn and released below the
  ephemeral range still races across concurrent processes (several agents on one development
  machine, several pipelines on one runner). The #2581 helper (`reserve_free_ports`) stays as it is
  for scenarios that must name a bind address before a child process starts.
- 2026-11-15 (maintainer, #2393 T5): of the 21 tests #2393 ignored, delete the 9 that only check
  test-support code, rewrite the 3 health-check contract tests to probe an address the test holds,
  and restore the 9 tests of the tracker's socket-release contract on a unique loopback address.
- 2026-11-15 (maintainer, #2393 T5): one shared test helper in `torrust-tracker-test-helpers`
  returns a random loopback address in `127.0.0.0/8`. It chooses an IP address, not a port, so it
  does not contradict the "no new port helper" decision; only the 9 restored tests use it, and they
  are ignored off Linux. The #2393 spec's Architectural Decisions section has the details.
- 2026-11-17 (maintainer, #2907 T2): the temporary-file work is split into focused subissues of
  this EPIC, with no nested EPIC, so that each pull request is easy to review. #2907 keeps the
  TOML and PEM files; the SQLite files go to two new subissues, #2924 and #2925 (orders 9 and 10).
- 2026-11-17 (maintainer, #2907 T2): the `ephemeral()` test configuration may default to no
  database. Where that does not fit, a specialised factory is fine, and the `ephemeral` functions
  may be renamed; names are open. Tests that need persistence opt in through a factory that
  returns a guard the test keeps. The guard is a `TempDir`, independent of the test environment,
  because some tests start a second environment on the same database.
- 2026-11-18 (maintainer, recorded in the archived EPIC #1488 spec): EPIC #1488 closes without
  waiting for #1419, and #1419 continues under this EPIC with its remaining R8, AC8a and R9.

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
- 2026-11-14 22:47 UTC - Claude Code (Claude Opus 5.5), EPIC coordinator - Synced the EPIC after
  #2393's PR #2859 merged: order 1 is in progress (T5 open), recorded the maintainer's decisions
  (disable racing tests first, no listener handoff for now, no new port helper) and the three
  racing port shapes, and pointed #2394's port inventory at #2393's.
- 2026-11-15 12:11 UTC - Claude Code (Claude Opus 5.5), EPIC coordinator - Addressed the first
  review round on PR #2862: named the 18 and 21 ignored tests behind the 150-run and 20-run #2393
  rechecks, added a fourth racing shape (released port probed and expected dead), and corrected the
  wording of the four rewritten tests.
- 2026-11-15 12:37 UTC - Claude Code (Claude Opus 5.5), EPIC coordinator - Second review round on
  PR #2862: merged `develop`, which carries the #2393 T5 decisions (PR #2863); order 1 now has
  T5 decided and T7 to T11 open, the Decisions section records the T5 decisions and the loopback
  helper, and AC2 and the lost-coverage risk point at #2393 T10 and T9.
- 2026-11-16 13:54 UTC - Claude Code (Claude Opus 5.5), #2393 session - #2393 closed after PR
  #2874 merged: order 1 is DONE and its spec moved to `docs/issues/closed/`.
- 2026-11-16 14:23 UTC - Claude Code (Claude Opus 5.5), #2393 session - Review of PR #2883: set
  Phase 1's state to done, gave AC2 the V6 evidence (it stays TODO until the order 2 audit), and
  marked the lost-coverage risk resolved.
- 2026-11-17 08:52 UTC - Claude Code (Claude Opus 5.5), #2394 session - #2394's PR #2891
  merged: order 2 is DONE, its spec moved to `docs/issues/closed/`, and its row links the three
  new-problem drafts D1, D2 and D3. AC4 stays TODO until their GitHub issues exist.
- 2026-11-17 12:35 UTC - Claude Code (Claude Opus 5.5), EPIC coordinator - Created #2906, #2907
  and #2908 from drafts D1, D2 and D3, moved their specs to `docs/issues/open/` and added them as
  orders 6 to 8; extended order 4 (#2396) with the local default image tags (#2394 C3, decision
  Q2). Phase 2 is done and AC4 is DONE; AC2 now waits on orders 6 and 7.
- 2026-11-17 15:22 UTC - Claude Code (Claude Opus 5.5), EPIC coordinator - From da2ce7's review of PR #2910:
  placed orders 6 to 8 in the Delivery Strategy and order 8 in Phase 3's exit criteria; order 8's
  note now points at order 4; order 2's row records that the close-out PR merged and #2394 waits
  on the maintainer.
- 2026-11-17 18:05 UTC - Claude Code (Claude Fable 5.2), EPIC coordinator - Split order 7 after
  #2907's T2: #2907 keeps the TOML and PEM files, and two drafts take the SQLite files as orders
  9 and 10. Recorded the maintainer's two decisions of 2026-11-17. AC1 is TODO again until the
  two GitHub issues exist; AC2 now also waits on orders 9 and 10.
- 2026-11-17 18:10 UTC - Claude Code (Claude Fable 5.2), EPIC coordinator - PR #2923 (#2902) fixes
  the SQLite leak inside `tracker-core`; orders 9 and 10 skip that package and reuse its shape.
- 2026-11-17 18:39 UTC - Claude Code (Claude Fable 5.2), EPIC coordinator - The maintainer approved the two
  drafts. Created #2924 and #2925, moved their specs to `docs/issues/open/`, and updated #2907's
  title and body on GitHub. AC1 stays TODO until the maintainer links the two sub-issues.
- 2026-11-18 13:23 UTC - Claude Code (Claude Fable 5.2), EPIC coordinator - Added order 11
  (#2943), which the maintainer approved as a draft: #2907 found that the `figment::Jail` tests
  change the environment that other tests of the same binary read, so audit finding G2 is not
  isolated. AC4 stays DONE because the finding is now tracked by a subissue. AC1 also waits on
  the link of #2943.
- 2026-11-18 13:39 UTC - Claude Code (Claude Opus 5.5), #2907 session - Order 7 is DONE: PR #2938 merged
  and GitHub closed #2907. Archived its spec to `docs/issues/closed/`. AC2 now waits on orders 6,
  9 and 10.
- 2026-11-19 08:25 UTC - Claude Code (Claude Fable 5.2), EPIC coordinator - Order 5 (#1419) is IN_PROGRESS:
  EPIC #1488 closed on 2026-11-19 (close-out PR #2951), the maintainer decided on 2026-11-18
  that #1419 continues under this EPIC, and its session has started. Its remaining work is R8,
  AC8a and R9.

## Acceptance Criteria

- [ ] AC1: All required subissues are created and linked, and #1419 is linked as an existing
  subissue.
- [ ] AC2: The full nextest suite runs repeatedly without failures caused by shared resources.
- [ ] AC3: Two E2E runs on one Docker host run concurrently, and each tests its own image.
- [x] AC4: The shared-resource audit is complete; every finding is isolated, a documented
  deliberate exception, or tracked by a subissue.
- [ ] AC5: The test isolation rules are documented where contributors write tests.
- [ ] Every completed subissue includes automated and manual verification evidence, a
  post-implementation acceptance criteria review, and an implementation completion review.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| ----- | ---------------------- | -------- |
| AC1 | TODO | GitHub sub-issues of #2392: #2393, #2394, #2395, #2396, #1419; the audit added #2906, #2907 and #2908. The split of #2907 added #2924 and #2925, which are created but not yet linked as sub-issues (the maintainer links them). #2907 then found #2943 (order 11), created and also not yet linked. |
| AC2 | TODO | Order 1 `manual-verification-evidence.md` (V6: 150 of 150 full-suite runs passed with no #2393 test ignored, after PR #2874). The order 2 audit found two more shared resources that make the suite fail: fixed ports that collide across concurrent runs (order 6) and leaked temporary files that fill the disk (order 7, split into orders 7, 9 and 10). Order 7 is done (PR #2938). Stays TODO until orders 6, 9 and 10 are done. |
| AC3 | TODO | Orders 3 and 4 `manual-verification-evidence.md` |
| AC4 | DONE | Order 2 `inventory.md`: 38 findings: isolated (28), a deliberate exception (1), or tracked by a subissue (5 covered by orders 1, 3 and 4; 3 new problems, orders 6 to 8; and G2, order 11). The inventory itself still counts G2 as isolated (29) until order 11 corrects its row. |
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
- **Lost coverage while tests are ignored.** Resolved: #2393 T9 restored the 9 tracker release
  tests, which prove that a server releases its socket after a stop, a drop or a failure, on a
  unique loopback address picked at random. They run on Linux and are conservatively ignored on
  other systems: macOS and the BSDs configure only `127.0.0.1`, and CI runs no tests off Linux.

## References

- Related EPICs: #1840 (PR workflow performance), #1347 (package testing)
- Related issues: #1419, #2386, #2132, #2298, #1488
- Related ADRs: none
