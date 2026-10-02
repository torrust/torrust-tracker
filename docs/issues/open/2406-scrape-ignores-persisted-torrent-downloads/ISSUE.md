---
schema-version: 1
doc-type: issue
issue-type: bug
status: in-progress
priority: p1
epic: null
github-issue: 2406
spec-path: docs/issues/open/2406-scrape-ignores-persisted-torrent-downloads/ISSUE.md
branch: "2406-scrape-ignores-persisted-torrent-downloads"
related-pr: null
last-updated-utc: "2026-10-02 17:40"
semantic-links:
  skill-links:
    - create-issue
    - fix-bug
    - write-unit-test
  related-artifacts:
    - .github/skills/dev/debugging/fix-bug/SKILL.md
    - .github/skills/dev/planning/create-issue/SKILL.md
    - .github/skills/dev/testing/write-unit-test/SKILL.md
    - docs/adrs/20261002173716_scrape_reports_announce_swarm_stats_without_side_effects.md
    - docs/research/20261002-scrape-downloaded-semantics/README.md
---

<!-- skill-link: create-issue -->

# Issue #2406 - Scrape Ignores Persisted Torrent Downloads After a Restart

## Goal

With `persistent_torrent_completed_stat` enabled, HTTP and UDP scrape report a torrent's persisted `downloaded` count after a tracker restart, even when no peer has announced that torrent in the new process.

## Bug in Plain Terms

The tracker saves the number of completed downloads for each torrent. A clean restart clears the in-memory swarm, but keeps that saved count. An announce lazily restores the count, while scrape only consults memory and reports zero. Operators and clients therefore receive an incorrect per-torrent scrape count until an announce happens after the restart.

## Background

The #1488 SI-22 shutdown reproduction initially found this symptom. M1 independently reproduced it on 2026-10-02 against an isolated local tracker with SQLite persistence: a live UDP scrape returned `completed: 1`; after a clean restart, HTTP returned `downloaded: 0` and UDP returned `completed: 0` before a new announce. The complete commands, environment, and output are in [manual-verification-evidence.md](manual-verification-evidence.md).

The relevant current behavior is:

- `AnnounceHandler::load_downloads_metric_if_needed` in `packages/tracker-core/src/announce_handler.rs` returns early when the in-memory repository contains the torrent. Otherwise, with persistence enabled, it loads the count through `load_torrent_downloads`; the first announce then inserts the swarm with that value.
- `ScrapeHandler::handle_scrape` in `packages/tracker-core/src/scrape_handler.rs` only calls `get_swarm_metadata_or_default`. `ScrapeHandler` has no persistence dependency, so a persisted torrent absent from memory always reports zero.

The global downloads metric is not affected; this issue changes only per-torrent scrape responses.

Relevant history:

| Item | Date | Result |
| --- | --- | --- |
| #1264 | 2025-03 | Persisted torrent data was not loaded at startup. |
| #1502 and PR #1509 | 2025-05 | Every torrent was loaded at startup (`ced2788a`). |
| #1510 | 2025-05 | The demo tracker could not load about 1.9 billion torrents from a 17 GB SQLite database at startup, so startup loading was disabled. |
| #1541 | 2025-05 | The disabled startup-load code was removed (`bd6e06ac`). |
| #1543 | 2025-05 | Global downloads became separately persisted and per-torrent downloads became lazily loaded on announce (`762bf690`). |

## Scope

### In Scope

- Make scrape use the in-memory count when present and otherwise retrieve a persisted count only when persistence is enabled.
- Preserve this behavior for both HTTP and UDP scrape, which use `ScrapeHandler`.
- Add a maintained regression test and prove it fails before the production fix.
- Repeat M1 like-for-like after the fix.

### Out of Scope

- Loading every torrent at startup, rejected by #1510.
- Changing the global downloads metric from #1543.
- Rate limiting or other scrape-abuse protections, tracked by the spam and abuse resistance EPIC #2411 (`docs/issues/open/2411-spam-and-abuse-resistance/EPIC.md`, from the #1488 SI-22 work).
- The HTTP scrape 74-info-hash limit, tracked by #2417 (`docs/issues/open/2417-2411-verify-http-scrape-info-hash-limit/ISSUE.md`).
- Batching persistence writes, tracked by #2418 (`docs/issues/open/2418-batch-persisted-download-writes/ISSUE.md`).
- Changes to the #1488 SI-22 shutdown implementation or specifications.

## Design Options

The implementer chooses after estimating the memory footprint of a peerless swarm and recording the result in task evidence. The maintainer preference is Option B because performance matters more than the small, bounded lifetime memory cost.

| Option | Behavior after database read | Pros | Cons |
| --- | --- | --- | --- |
| A | Return the persisted value without adding a swarm to memory. | Does not grow memory from scrape. | Repeated scrapes read the database until an announce. |
| B | Add a peerless swarm with the persisted count, as first announce does. | Avoids further database reads while the cached swarm remains resident. | Memory grows per scraped torrent until peerless-torrent cleanup; a later scrape can read the database again after cleanup. |

The decision must answer:

- Whether inserting from scrape emits `TorrentAdded`, and whether that is desirable.
- Whether a missing persisted row is inserted under Option B. Inserting it avoids repeated database reads but makes random-info-hash scrapes consume memory.
- Whether the lookup belongs in `ScrapeHandler` or a shared helper used by announce and scrape.

If the selected behavior adds a material new random-info-hash persistence or memory-growth case, add a note to the spam and abuse resistance EPIC draft. It does not block this bug fix.

### Decision (2026-10-02)

The maintainer questioned whether scrape should report only the active swarm (scalability and privacy by default). Research in [docs/research/20261002-scrape-downloaded-semantics/README.md](../../../research/20261002-scrape-downloaded-semantics/README.md) found that BEP 48 defines `downloaded` as a lifetime counter and frames scrape as having no effect on swarm participation, and that the announce response already exposes the persisted count. The maintainer accepted the principle that scrape returns the statistics an announce would return, without side effects, recorded in the ADR below.

- **Option A selected.** Scrape reads the persisted count only when the swarm is absent from memory and persistence is enabled. It does not insert a swarm.
- **Option B rejected.** `TorrentAdded` requires a peer announcement that scrape lacks; inserting without it would unbalance the `torrents_total` gauge when peerless cleanup emits `TorrentRemoved`; and with `remove_peerless_torrents = false` scraped entries would never expire. The rough peerless-swarm footprint (about 150-250 bytes per entry) was not the deciding factor.
- **Events:** scrape emits no swarm events.
- **Missing persisted row:** scrape returns zero and inserts nothing.
- **Database errors:** propagate through a new `ScrapeError::Database` variant, mirroring `AnnounceError::Database`.
- **Lookup placement:** a shared tracker-core helper used by both announce and scrape.
- **Construction:** `ScrapeHandler` mirrors `AnnounceHandler` with `new_public` and `new_with_persistent_completed_statistics`.
- **Spam and abuse note:** scraping info-hashes absent from memory costs one database read each when persistence is enabled; add a note to the spam and abuse EPIC draft if it is on `develop`, otherwise record it in the progress log.
- Any future setting to hide persisted counts from responses must apply to announce and scrape together; it is out of scope.

## Architectural Decisions

- Related ADRs: [Events are objective facts](../../../adrs/20260727000000_events_are_objective_facts.md).
- ADRs created: [Scrape reports announce swarm statistics without side effects](../../../adrs/20261002173716_scrape_reports_announce_swarm_stats_without_side_effects.md).

## Design and Ownership Review

Not applicable. The production change does not add child processes, network readiness logic, resource cleanup, or reusable fixtures. Revisit this section if the regression test requires a new database fixture with an owned lifetime.

## Bug-Fix Process

Follow `.github/skills/dev/debugging/fix-bug/SKILL.md` in order:

1. Analyze the causal handlers and persistence path. The initial local hypothesis is that scrape reports zero because it cannot load persisted downloads when the swarm is absent from memory.
2. Reproduce against the real artifact. M1 is `DONE` and records the wrong HTTP and UDP responses in [manual-verification-evidence.md](manual-verification-evidence.md).
3. Select and document the smallest deterministic regression-test boundary.
4. Add the regression test before production changes, run it against the broken implementation, and record the red result.
5. Make the smallest production change that restores persisted scrape counts.
6. Record focused green tests, repeat M1 like-for-like, then review every acceptance criterion against observed evidence.

## Regression Test Strategy

The maintained regression boundary is a `tracker-core` collaboration/integration test with a real SQLite database, following the existing persistence tests in `packages/tracker-core/tests/integration.rs`. It must persist a per-torrent count, create a fresh tracker-core environment with the same database and empty in-memory repository, call `ScrapeHandler`, and assert the restored count.

This is not currently a unit test: `ScrapeHandler` neither owns a persistence dependency nor exposes a pure persistence lookup seam. The observable contract requires both the persistence repository and the restarted in-memory state. Before implementing the test, assess whether the selected production design creates a focused pure decision helper that merits a unit test; retain the restart integration test regardless because it protects the persistence contract. Use the `write-unit-test` skill for any test-producing increment.

For each added or refactored test, record a prose-first Arrange-Act-Assert review in task evidence. The review must confirm that the visible causal difference is the fresh in-memory repository backed by the same database, the fixture owns only incidental setup, and the production Act plus independently specified persisted count remain visible. Option A was selected, so also assert that scrape does not insert the torrent into the in-memory repository.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID | Status | Task | Notes / Expected Output |
| --- | --- | --- | --- |
| T1 | DONE | Reproduce M1 on a local tracker | [manual-verification-evidence.md](manual-verification-evidence.md) V1 records the incorrect post-restart HTTP and UDP responses. |
| T2 | DONE | Choose Option A or B | Option A; see Decision (2026-10-02), the research document, and the ADR. |
| T3 | TODO | Add and prove a red regression test | Use a real SQLite restart boundary; record the failing stable-Rust command and prose-first test review. |
| T4 | TODO | Implement the production fix | Preserve announce and global-metric behavior; rerun focused tests after the change. |
| T5 | TODO | Verify green and recheck M1 | Record green test output and like-for-like HTTP and UDP results after a clean restart. |
| T6 | TODO | Complete acceptance and implementation reviews | Re-review acceptance criteria, record deviations or lessons, and create a retrospective only when warranted. |

## Commit Points

| Task | Coherent change set | Commit policy |
| --- | --- | --- |
| T2 | Design decision and any approved issue-spec clarification | Commit documentation only when it is independently reviewable. |
| T3 | Regression test plus its reviewed task evidence | Commit after red proof, focused validation, and maintainer review. |
| T4 | Production fix and immediately affected tests | Commit after focused green validation and review. |
| T5-T6 | Final evidence and specification status | Commit after final validation; do not create an empty commit for a no-change decision. |

Every test-producing increment requires the `write-unit-test` skill's prose-first Arrange-Act-Assert comparison before maintainer review and commit. Use a narrow Conventional Commit scope and GPG-sign every commit.

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/drafts/scrape-ignores-persisted-torrent-downloads/ISSUE.md`
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue created and issue number added to this spec
- [x] Spec-only PR merged into `develop` before implementation; implementation begins in a separate follow-up branch after this specification is merged
- [ ] Implementation completed
- [ ] Automatic verification completed (`linter all`, relevant tests, and pre-push checks)
- [x] Initial manual reproduction executed and recorded in issue-local `manual-verification-evidence.md`
- [ ] Final manual verification scenarios executed and recorded in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] Evidence-based implementation completion review recorded: issue-local retrospective created for material discoveries, or progress log states why none was needed
- [ ] Reviewer validated acceptance criteria and updated checkboxes
- [ ] Independent reviewer reports recorded in issue-local `agent-review-reports.md` when reviewers received this folder-style specification
- [ ] Committer verified spec progress is up to date before commit
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-10-02 11:09 UTC - Copilot - Drafted the issue specification and independently reproduced the post-restart HTTP and UDP scrape defect - [manual-verification-evidence.md](manual-verification-evidence.md) V1.
- 2026-10-02 11:28 UTC - Copilot - Maintainer approved the spec; created GitHub issue #2406 and moved the spec to `docs/issues/open/` - <https://github.com/torrust/torrust-tracker/issues/2406>.
- 2026-10-02 13:30 UTC - Copilot - Repeated V1 from the recorded PR branch commit and verified `torrents.completed = 1` directly in SQLite after the first clean shutdown - [manual-verification-evidence.md](manual-verification-evidence.md) V1.
- 2026-10-02 17:40 UTC - Copilot - Created implementation branch `2406-scrape-ignores-persisted-torrent-downloads`; researched BEPs and other trackers; maintainer selected Option A under the "scrape = announce statistics without side effects" principle - [research](../../../research/20261002-scrape-downloaded-semantics/README.md), [ADR](../../../adrs/20261002173716_scrape_reports_announce_swarm_stats_without_side_effects.md).

## Acceptance Criteria

- [ ] AC1: With persistence enabled, after a restart, an HTTP scrape reports the persisted `downloaded` count for a torrent that has not announced in the new process.
- [ ] AC2: With persistence enabled, after a restart, a UDP scrape reports the persisted completed count for a torrent that has not announced in the new process.
- [ ] AC3: With persistence disabled, scrape does not read the database.
- [ ] AC4: Announce behavior and the global downloads metric remain unchanged.
- [ ] AC5: A maintained regression test fails against the broken behavior and passes after the fix.
- [ ] `linter all` exits with code `0`.
- [ ] Relevant tests pass on the stable Rust toolchain.
- [ ] Manual verification scenarios are executed and documented in issue-local `manual-verification-evidence.md`.
- [ ] Acceptance criteria are re-reviewed after implementation and reflect actual behavior.
- [ ] Documentation is updated when behavior or workflow changes.

## Verification Plan

### Automatic Checks

- Focused `tracker-core` regression test on the stable Rust toolchain, first red and then green.
- Affected `tracker-core` test suite on the stable Rust toolchain.
- `linter all`.
- Pre-push checks before push, when applicable.

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID | Scenario | Human-oriented command/steps | Expected Result | Status | Evidence |
| --- | --- | --- | --- | --- | --- |
| M1 | Pre-fix persisted scrape after restart | Start a local SQLite-backed tracker with `persistent_torrent_completed_stat = true`; announce `started` and `completed`; stop cleanly; restart with the same database; scrape the torrent over HTTP and UDP before another announce. | Before the fix, both protocol responses report zero. After the fix, both report the persisted count. | DONE (pre-fix) | [manual-verification-evidence.md](manual-verification-evidence.md) V1 |
| M2 | Post-fix like-for-like recheck | Repeat M1 against the fixed artifact with the same database lifecycle and both scrape protocols. | HTTP `downloaded` and UDP `completed` equal the persisted count before any post-restart announce. | TODO | `manual-verification-evidence.md` V2 |

Manual verification is mandatory even when automatic tests pass. Record the exact commands, actual output, runtime configuration, and relevant logs in the evidence file. A failed scenario must be recorded in the progress log before proceeding.

### Disposable Verification Scripts

None planned. The durable behavior is covered by a maintained Rust regression test; M1 and M2 remain manual real-artifact checks.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| --- | --- | --- |
| AC1 | TODO | M2 HTTP response and focused regression test |
| AC2 | TODO | M2 UDP response and focused regression test |
| AC3 | TODO | Focused test with persistence disabled |
| AC4 | TODO | Focused announce and metrics tests |
| AC5 | TODO | Recorded red and green regression-test output |

## Risks and Trade-offs

- Option A does not cache, so repeated scrapes of a torrent absent from memory each read the database when persistence is enabled. Scrape-abuse protection is out of scope.
- Adding persistence access to scrape can blur announce and scrape ownership. Prefer the smallest design that makes the shared policy explicit and create an ADR if the responsibility boundary materially changes.
- A database-backed restart test can be slower than a unit test. Keep it focused, deterministic, and isolated because it is the clearest contract boundary for the defect.

## Implementation Completion Review

After implementation, compare observed behavior with this specification. Record invalidated assumptions, material design changes, unexpected validation findings, and reusable lessons.

- Retrospective: Not yet assessed.
- Create `implementation-retrospective.md` from `docs/templates/IMPLEMENTATION-RETROSPECTIVE.md` if the selected design, test boundary, or validation findings yield a reusable lesson. Otherwise add a concise progress-log entry explaining why no retrospective was needed.
- When an independent reviewer receives this folder-style specification, record its result in `agent-review-reports.md` using `docs/templates/AGENT-REVIEW-REPORTS.md`.

## References

- Related issues: #1264, #1488, #1502, #1510, #1541, #1543
- Related PRs: #1509
- Related ADRs: [20261002173716](../../../adrs/20261002173716_scrape_reports_announce_swarm_stats_without_side_effects.md)
- Research: [scrape `downloaded` semantics](../../../research/20261002-scrape-downloaded-semantics/README.md)
