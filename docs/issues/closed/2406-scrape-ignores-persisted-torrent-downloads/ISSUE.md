---
schema-version: 1
doc-type: issue
issue-type: bug
status: done
priority: p1
epic: null
github-issue: 2406
spec-path: docs/issues/closed/2406-scrape-ignores-persisted-torrent-downloads/ISSUE.md
branch: "2406-scrape-ignores-persisted-torrent-downloads"
related-pr: 2423
last-updated-utc: "2026-10-04 07:57"
semantic-links:
  skill-links:
    - create-issue
    - fix-bug
    - write-unit-test
  related-artifacts:
    - .github/skills/dev/debugging/fix-bug/SKILL.md
    - .github/skills/dev/planning/create-issue/SKILL.md
    - .github/skills/dev/testing/write-unit-test/SKILL.md
    - docs/adrs/20261002173716_load_persisted_scrape_downloads_with_a_batched_uncached_lookup.md
    - docs/research/20261002-scrape-downloaded-semantics/README.md
    - packages/tracker-core/src/scrape_handler.rs
    - packages/tracker-core/tests/integration.rs
    - tests/persistence/scrape_after_restart.rs
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

The maintainer questioned whether scrape should report only the active swarm (scalability and privacy by default). Research in [docs/research/20261002-scrape-downloaded-semantics/README.md](../../../research/20261002-scrape-downloaded-semantics/README.md) found that BEP 48 defines `downloaded` as a lifetime counter and frames scrape as having no effect on swarm participation, and that the announce response already exposes the persisted count. The maintainer accepted the principle that scrape returns the statistics an announce would return, without side effects. It is documented in the `ScrapeHandler` module Rustdoc (`packages/tracker-core/src/scrape_handler.rs`); the ADR below records only the lookup design.

- **Option A selected.** Scrape reads the persisted count only when the swarm is absent from memory and persistence is enabled. It does not insert a swarm.
- **Option B rejected.** `TorrentAdded` requires a peer announcement that scrape lacks; inserting without it would unbalance the `torrents_total` gauge when peerless cleanup emits `TorrentRemoved`; and with `remove_peerless_torrents = false` scraped entries would never expire. The rough peerless-swarm footprint (about 150-250 bytes per entry) was not the deciding factor.
- **Events:** scrape emits no swarm events.
- **Missing persisted row:** scrape returns zero and inserts nothing.
- **Database errors:** propagate through a new `ScrapeError::Database` variant, mirroring `AnnounceError::Database`.
- **Lookup placement:** a shared tracker-core helper used by both announce and scrape.
- **Construction:** `ScrapeHandler` mirrors `AnnounceHandler` with `new_public` and `new_with_persistent_completed_statistics`.
- **Spam and abuse note:** scraping info-hashes absent from memory costs one database read each when persistence is enabled; add a note to the spam and abuse EPIC draft if it is on `develop`, otherwise record it in the progress log.
- Any future setting to hide persisted counts from responses must apply to announce and scrape together; it is out of scope.

### Decision Refinement (2026-10-02, after implementation review)

- **Which value:** BEP 48 has one `downloaded` field ("ever completed"). With persistence disabled, scrape returns the in-memory count, which covers completions registered while the torrent's swarm has been in memory in this process (peerless cleanup, enabled by default, discards it); with persistence enabled, the persisted lifetime count. An active swarm's in-memory value already is the lifetime count, so active swarms are read from memory exactly as announce does.
- **Batch lookup:** scrape loads the persisted counts of all authorized info-hashes absent from memory with one query per request instead of one per torrent (N+1). New `TorrentMetricsStore::load_torrents_downloads` for SQLite, MySQL, and PostgreSQL.
- **Query size limit:** drivers bind at most `MAX_INFO_HASHES_PER_QUERY = 100` info-hashes per `IN (...)` query and split larger inputs. A full UDP scrape (about 74) fits in one query; the value is independent of protocol request limits, which are discussed in #2417 and may grow.
- **No cache:** scrape is far less frequent than announce; add a cache only if metrics show a need.
- Recorded in the [ADR](../../../adrs/20261002173716_load_persisted_scrape_downloads_with_a_batched_uncached_lookup.md) and the [research document](../../../research/20261002-scrape-downloaded-semantics/README.md) section 5.

## Architectural Decisions

- Related ADRs: [Events are objective facts](../../../adrs/20260727000000_events_are_objective_facts.md).
- ADRs created: [Load persisted scrape downloads with a batched, uncached lookup](../../../adrs/20261002173716_load_persisted_scrape_downloads_with_a_batched_uncached_lookup.md) (technical decisions only).
- Behavior contract: the `ScrapeHandler` module Rustdoc in `packages/tracker-core/src/scrape_handler.rs`, which names the prose-style tests that serve as its executable specification.

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
| T3 | DONE | Add and prove a red regression test | [manual-verification-evidence.md](manual-verification-evidence.md) "Selected Tests" and "Red Run". |
| T4 | DONE | Implement the production fix | Shared `PersistedDownloads` lookup, `ScrapeHandler` paired constructors, `ScrapeError::Database`; announce behavior unchanged. |
| T5 | DONE | Verify green and recheck M1 | [manual-verification-evidence.md](manual-verification-evidence.md) "Green Run" and V2. |
| T7 | DONE | Batch the persisted scrape lookup | One query per scrape request, chunked at 100 info-hashes; unit test proven against a per-torrent mutation; driver tests pass on SQLite, MySQL, and PostgreSQL. See [manual-verification-evidence.md](manual-verification-evidence.md) "Batch Lookup (T7)". |
| T8 | DONE | Automate the manual restart scenario | Protocol-level tests in `http-core` and `udp-server`, plus the root binary `persistence-scrape-after-restart` (UDP completion, application restart on the same SQLite database, HTTP and UDP scrape). All proven against a disabled-lookup mutation. See [manual-verification-evidence.md](manual-verification-evidence.md) "Automated Equivalents of the Manual Scenarios (T8)". |
| T9 | DONE | Review new tests against the `write-unit-test` guide | Smells found and fixed; prose-first AAA comparison and failure-message review recorded in [manual-verification-evidence.md](manual-verification-evidence.md) "Test Review Against the Write-Unit-Test Guide (T9)". |
| T6 | IN_PROGRESS | Complete acceptance and implementation reviews | Acceptance verification table updated; awaiting maintainer review and pre-push checks. |

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
- [x] Implementation completed
- [x] Automatic verification completed (`linter all`, relevant tests, and pre-push checks)
- [x] Initial manual reproduction executed and recorded in issue-local `manual-verification-evidence.md`
- [x] Final manual verification scenarios executed and recorded in issue-local `manual-verification-evidence.md`
- [x] Acceptance criteria reviewed after implementation and updated with evidence
- [x] Evidence-based implementation completion review recorded: issue-local retrospective created for material discoveries, or progress log states why none was needed
- [ ] Reviewer validated acceptance criteria and updated checkboxes
- [ ] Independent reviewer reports recorded in issue-local `agent-review-reports.md` when reviewers received this folder-style specification
- [x] Committer verified spec progress is up to date before commit
- [x] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-10-02 11:09 UTC - Copilot - Drafted the issue specification and independently reproduced the post-restart HTTP and UDP scrape defect - [manual-verification-evidence.md](manual-verification-evidence.md) V1.
- 2026-10-02 11:28 UTC - Copilot - Maintainer approved the spec; created GitHub issue #2406 and moved the spec to `docs/issues/open/` - <https://github.com/torrust/torrust-tracker/issues/2406>.
- 2026-10-02 13:30 UTC - Copilot - Repeated V1 from the recorded PR branch commit and verified `torrents.completed = 1` directly in SQLite after the first clean shutdown - [manual-verification-evidence.md](manual-verification-evidence.md) V1.
- 2026-10-02 17:40 UTC - Copilot - Created implementation branch `2406-scrape-ignores-persisted-torrent-downloads`; researched BEPs and other trackers; maintainer selected Option A under the "scrape = announce statistics without side effects" principle - [research](../../../research/20261002-scrape-downloaded-semantics/README.md); the principle is documented in the `ScrapeHandler` module Rustdoc (`packages/tracker-core/src/scrape_handler.rs`) and the lookup design in the [ADR](../../../adrs/20261002173716_load_persisted_scrape_downloads_with_a_batched_uncached_lookup.md).
- 2026-10-02 17:50 UTC - Copilot - Added two `tracker-core` integration tests; the persisted-downloads test failed against the unfixed code (`downloaded: 0`, expected `7`) - [manual-verification-evidence.md](manual-verification-evidence.md).
- 2026-10-02 18:00 UTC - Copilot - Implemented Option A; integration, unit, and affected-package tests pass; V2 shows HTTP `downloaded: 1` and UDP `completed: 1` after a clean restart. Spam and abuse note: the EPIC draft is not on `develop`, so it is recorded here instead: with persistence enabled, each scrape of an info-hash absent from memory costs one database read (as a first announce does), and Option A adds no memory growth.
- 2026-10-02 18:31 UTC - Copilot - Maintainer refined the decision (value per persistence mode, batch lookup without N+1, query limit independent of #2417, no cache). Implemented the batch lookup as a separate `perf(tracker-core)` commit; the spam and abuse note above now reads one batch query per scrape request instead of one read per info-hash.
- 2026-10-02 18:53 UTC - Copilot - Maintainer asked for automated equivalents of the manual V2/V3 checks. Added protocol-level tests (`http-core`, `udp-server`) and a root restart integration binary; manual verification stays mandatory for the real binary and OS process.
- 2026-10-02 19:03 UTC - Copilot - Pre-push checks passed; opened implementation PR #2423 - <https://github.com/torrust/torrust-tracker/pull/2423>.
- 2026-10-02 20:28 UTC - Copilot - Fixed the five Copilot review findings on #2423 in separate commits - [PR review audit](../../../pr-reviews/pr-2423-review/PR-REVIEW.md).
- 2026-10-03 07:16 UTC - Copilot - Maintainer asked to review every new test against the repository test guides. Fixed the smells found (combined behaviors, hidden seeder state, multiple field assertions, missing AAA markers and assertion messages, one restart scenario covering two protocols) - [manual-verification-evidence.md](manual-verification-evidence.md) T9.
- 2026-10-03 07:48 UTC - Copilot - Maintainer separated the domain behavior from the technical decision: the ADR was renamed to `20261002173716_load_persisted_scrape_downloads_with_a_batched_uncached_lookup.md` and keeps only the lookup design; the behavior contract moved to the `ScrapeHandler` module Rustdoc, which names the prose-style tests that specify it. Earlier links in this log were updated to the new ADR path.
- 2026-10-04 07:57 UTC - Copilot - Human review (da2ce7) on #2423 ran seven rounds (F6-F19) and approved the final head; the pre-push suite and CI passed on it. #2423 was merged at 07:24 UTC as `a3914e03c` and closed #2406. Archived this spec to `docs/issues/closed/`. The reviewer-validation and agent-review-report checkpoints stay unchecked: no acceptance-criteria validation or agent review report was recorded for this specification. The review findings are in the [PR review audit](../../../pr-reviews/pr-2423-review/PR-REVIEW.md).

## Acceptance Criteria

- [x] AC1: With persistence enabled, after a restart, an HTTP scrape reports the persisted `downloaded` count for a torrent that has not announced in the new process.
- [x] AC2: With persistence enabled, after a restart, a UDP scrape reports the persisted completed count for a torrent that has not announced in the new process.
- [x] AC3: With persistence disabled, scrape does not read the database.
- [x] AC4: Announce behavior and the global downloads metric remain unchanged.
- [x] AC5: A maintained regression test fails against the broken behavior and passes after the fix.
- [x] `linter all` exits with code `0`.
- [x] Relevant tests pass on the stable Rust toolchain.
- [x] Manual verification scenarios are executed and documented in issue-local `manual-verification-evidence.md`.
- [x] Acceptance criteria are re-reviewed after implementation and reflect actual behavior.
- [x] Documentation is updated when behavior or workflow changes.

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
| M2 | Post-fix like-for-like recheck | Repeat M1 against the fixed artifact with the same database lifecycle and both scrape protocols. | HTTP `downloaded` and UDP `completed` equal the persisted count before any post-restart announce. | DONE | [manual-verification-evidence.md](manual-verification-evidence.md) V2 |

Manual verification is mandatory even when automatic tests pass. Record the exact commands, actual output, runtime configuration, and relevant logs in the evidence file. A failed scenario must be recorded in the progress log before proceeding.

### Disposable Verification Scripts

None planned. The durable behavior is covered by a maintained Rust regression test; M1 and M2 remain manual real-artifact checks.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| --- | --- | --- |
| AC1 | DONE | M2 (V2) HTTP `downloaded: 1`; `it_should_scrape_the_persisted_downloads_of_a_torrent_absent_from_memory`; root `persistence-scrape-after-restart` |
| AC2 | DONE | M2 (V2) UDP `completed: 1`; same integration test (both protocols use `ScrapeHandler`); `udp-server` request-order test; root `persistence-scrape-after-restart` |
| AC3 | DONE | `it_should_not_scrape_persisted_downloads_when_the_persistent_completed_stat_is_disabled`; `ScrapeHandler::new_public` holds no persistence dependency |
| AC4 | DONE | Existing announce tests, `container::tests::it_should_load_persistent_completed_statistics_when_a_torrent_is_first_announced`, and the global persisted-downloads integration test pass unchanged |
| AC5 | DONE | Red and green output in [manual-verification-evidence.md](manual-verification-evidence.md) |

## Risks and Trade-offs

- Option A does not cache, so each scrape request with torrents absent from memory runs one batch query (one per 100 info-hashes) when persistence is enabled. Scrape-abuse protection is out of scope.
- Adding persistence access to scrape can blur announce and scrape ownership. Prefer the smallest design that makes the shared policy explicit and create an ADR if the responsibility boundary materially changes.
- A database-backed restart test can be slower than a unit test. Keep it focused, deterministic, and isolated because it is the clearest contract boundary for the defect.

## Implementation Completion Review

After implementation, compare observed behavior with this specification. Record invalidated assumptions, material design changes, unexpected validation findings, and reusable lessons.

- Retrospective: Not needed. The only material discovery (scrape scope semantics) is captured in the research document, the `ScrapeHandler` Rustdoc contract, and the ADR; the test boundary and design held as planned. The review rounds on #2423 found process slips (a `FIXED` reply posted before its fix was pushed, branch heads named by commit id) that break existing `process-pr-review` rules; they are recorded in the PR review audit rather than as new lessons.
- Create `implementation-retrospective.md` from `docs/templates/IMPLEMENTATION-RETROSPECTIVE.md` if the selected design, test boundary, or validation findings yield a reusable lesson. Otherwise add a concise progress-log entry explaining why no retrospective was needed.
- When an independent reviewer receives this folder-style specification, record its result in `agent-review-reports.md` using `docs/templates/AGENT-REVIEW-REPORTS.md`.

## References

- Related issues: #1264, #1488, #1502, #1510, #1541, #1543
- Related PRs: #1509
- Related ADRs: [20261002173716](../../../adrs/20261002173716_load_persisted_scrape_downloads_with_a_batched_uncached_lookup.md)
- Research: [scrape `downloaded` semantics](../../../research/20261002-scrape-downloaded-semantics/README.md)
