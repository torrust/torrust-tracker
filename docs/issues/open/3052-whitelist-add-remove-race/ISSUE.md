---
schema-version: 1
doc-type: issue
issue-type: bug
status: planned
priority: p2
epic: null
github-issue: 3052
spec-path: docs/issues/open/3052-whitelist-add-remove-race/ISSUE.md
branch: "3052-whitelist-add-remove-race"
related-pr: null
last-updated-utc: "2026-11-25 17:04"
semantic-links:
  skill-links:
    - create-issue
    - fix-bug
  related-artifacts:
    - .claude/skills/fix-bug/SKILL.md
    - .claude/skills/create-issue/SKILL.md
---

<!-- skill-link: create-issue -->

# Issue #3052 - Concurrent identical whitelist add or remove fails although the operation is idempotent

## Goal

`DatabaseWhitelist::add` and `remove` succeed when called concurrently for the same info-hash, as they
already do when called one after the other, on all three database drivers (SQLite, MySQL, PostgreSQL).

## Background

Found as finding F4 of #2902 (EPIC #1347) while adding unit tests to `packages/tracker-core`.

**What happens, step by step.** `DatabaseWhitelist::add`
(`packages/tracker-core/src/whitelist/repository/persisted.rs`) is written to be idempotent: adding an
info-hash that is already whitelisted returns `Ok`. It does so in two separate database calls: first
`is_info_hash_whitelisted` (a `SELECT`), then, only if the hash is absent, `add_info_hash_to_whitelist`
(an `INSERT`). Nothing makes the two calls atomic, and `whitelist.info_hash` has a UNIQUE constraint.

1. Request A reads: hash H is not whitelisted.
2. Request B reads: hash H is not whitelisted.
3. A inserts H and succeeds.
4. B inserts H and the database rejects it. The store returns
   `InvalidQuery { source: Database(SqliteError { code: 2067, message: "UNIQUE constraint failed: whitelist.info_hash" }), driver: Sqlite3 }`.

`remove` is the mirror image: both callers read "whitelisted", the first `DELETE` removes one row, and
the second removes zero rows, which the store reports as `Error::DeleteFailed` (it requires exactly one
row).

**Contract violated.** The doc comments of both methods ("if not already present", "if it exists")
and the tests `should_not_add_the_same_infohash_to_the_list_twice` and
`should_not_fail_removing_an_infohash_that_is_not_in_the_list` define both operations as idempotent.
They are, but only sequentially.

**Impact.** `WhitelistManager::add_torrent_to_whitelist` (used by the REST whitelist endpoint) calls
`add`. Two simultaneous identical requests (a client retry, two operators, two scripts) make one of them
fail although the hash ends up whitelisted. The database and the in-memory whitelist stay correct (only
the winner updates memory); the observable harm is a false error to the caller. The same applies to the
remove endpoint. The MySQL and PostgreSQL drivers have the same code shape; their behavior is checked
in the reproduction (V2).

## Design Decision

Chosen by the maintainer (2026-11-25): **make the store writes idempotent**, not a transaction and not
error-reclassification in `DatabaseWhitelist`.

- Add: `INSERT ... ON CONFLICT(info_hash) DO NOTHING` (SQLite, PostgreSQL) and
  `INSERT ... ON DUPLICATE KEY UPDATE info_hash = info_hash` (MySQL; not `INSERT IGNORE`, which also
  hides unrelated errors). The database serializes concurrent inserts on the unique index.
- Remove: `DELETE` already tolerates a missing row; accept zero affected rows instead of returning
  `DeleteFailed`.
- `WhitelistStore::add_info_hash_to_whitelist` and `remove_info_hash_from_whitelist` keep returning
  `Result<usize, Error>`, now documented as "number of rows changed; `0` means nothing to do".
- `DatabaseWhitelist::add` and `remove` drop the read-before-write.
- Contract change at the store level: a direct second insert, and a direct delete of a missing hash,
  no longer fail. The three shared-harness tests that document the failures
  (`it_should_fail_trying_to_add_the_same_infohash_twice`,
  `it_should_fail_without_a_source_path_trying_to_add_the_same_infohash_twice`,
  `it_should_fail_without_a_source_path_trying_to_remove_a_missing_infohash`, in
  `databases/driver/mod.rs`) are replaced by tests that state the new no-op behavior and the returned
  row counts.
- `Error::InsertFailed` and `Error::DeleteFailed` stay: the authentication-key stores still use them.

Rejected: a transaction (SQLite deferred-transaction upgrade can return `SQLITE_BUSY`; MySQL needs
SERIALIZABLE with gap locks and can deadlock) and re-checking after an error in `DatabaseWhitelist`
(keeps the contract but needs two round trips and treats a real error on a present hash as success).

## Scope

### In Scope

- The three `whitelist_store.rs` drivers, the `WhitelistStore` trait docs, `DatabaseWhitelist`.
- The shared driver test harness cases listed above and a maintained concurrency regression test.

### Out of Scope

- Authentication-key stores (`auth_key_store.rs`), which have a similar read-then-write shape but are
  a separate question (a follow-up issue if wanted).
- `docs/issues/open/2902-1347-add-tests-tracker-core/`, owned by the #2902 session, which adds its own
  persisted-whitelist regression test after this fix merges.
- Any change to REST responses.

## Architectural Decisions

- Related ADRs: none known.
- ADRs to create: None known. The store-contract change is recorded in this spec.

## Design and Ownership Review

Not applicable.

## Bug-Fix Process

Follows `.claude/skills/fix-bug/SKILL.md`:

- **Analysis:** read-then-write without atomicity in `persisted.rs`, UNIQUE constraint and exactly-one-row
  delete check in the drivers (hypothesis above).
- **Reproduction:** V1 (SQLite, done, see below) and V2 (MySQL and PostgreSQL via the Docker suites).
- **Regression-test boundary:** see below.
- **Red test:** recorded before the fix.
- **Fix:** the design decision above.
- **Green and recheck:** regression test green, then the V1 reproduction rerun after the last production
  change.

## Regression Test Strategy

- **Boundary:** a unit test of `DatabaseWhitelist` in `persisted.rs` on a temporary SQLite database,
  `#[tokio::test(flavor = "multi_thread", worker_threads = 4)]`, running many rounds of two
  `tokio::spawn`ed concurrent identical `add` calls and two concurrent identical `remove` calls. The
  race window is the gap between two awaits, so a real multi-threaded runtime and many rounds are
  needed for determinism of the red run; it failed 200 of 200 rounds in the reproduction.
  This is the smallest boundary that exercises the causal seam (the repository's read-then-write over
  the real constraint).
- **Store-level tests:** the shared driver harness cases (run on SQLite in unit tests, and on MySQL and
  PostgreSQL by the Docker suites) assert the new idempotent behavior and row counts, sequentially (AC3).
  The harness also gets maintained concurrent cases at the store level: two concurrent
  `add_info_hash_to_whitelist` calls of one hash both return `Ok` and the hash is listed; two
  concurrent `remove_info_hash_from_whitelist` calls of one hash both return `Ok` and their row counts
  sum to `1`. These run on all three drivers, so MySQL and PostgreSQL have maintained concurrency
  coverage of the causal write (AC1 and AC2); V2 is the one-off reproduction before the fix.
- **Evidence:** red and green output in `manual-verification-evidence.md`.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task                                                                    | Notes / Expected Output                                          |
| --- | ------ | ----------------------------------------------------------------------- | ---------------------------------------------------------------- |
| T1  | DONE   | Reproduce on SQLite and record it                                       | V1 recorded in `manual-verification-evidence.md`                 |
| T2  | DONE   | Reproduce on MySQL and PostgreSQL (Docker)                              | V2 recorded, or infeasibility with attempted commands            |
| T3  | DONE   | Add the concurrency regression test and the concurrent store-level harness cases, and observe them red | Red run recorded on the unfixed code: the `persisted.rs` test fails for `add` and for `remove`; the harness cases fail on SQLite, and on MySQL and PostgreSQL via the Docker suites |
| T4  | DONE   | Make the three drivers idempotent; update trait docs and `DatabaseWhitelist` | Regression test green                                       |
| T5  | DONE   | Replace the three failure-documenting harness tests with no-op tests (the concurrent cases were added red in T3) | SQLite unit tests and Docker suites green |
| T6  | DONE   | Rerun V1 (and V2) after the last production change; run `linter all`    | Evidence names the commit by Conventional Commit subject         |

## Commit Points

| Task | Coherent change set                                          | Commit policy                                        |
| ---- | ------------------------------------------------------------ | ---------------------------------------------------- |
| T3   | The `persisted.rs` regression test plus its red evidence    | Commit after focused validation and required review. |
| T3   | The concurrent store-level harness cases plus their red evidence | Commit after focused validation and required review, as its own commit: each reviewed test-design increment is committed before the next planned file. |
| T4   | Driver and `DatabaseWhitelist` change, trait docs            | Commit after focused validation and required review. |
| T5   | Harness test updates                                         | Commit after focused validation and required review. |
| T6   | Final evidence and spec progress                             | Commit after focused validation and required review. |

Record a justified no-change decision in the task's evidence without creating an empty commit. For
test-producing work, use the `write-unit-test` skill and complete an explicit design review after
each passing test increment, before maintainer review and commit. Confirm that the test exposes the
one causal initial-state difference; its fixture owns only incidental mechanics; and the production
Act plus independently specified expected result remain visible. The review must use the mandatory
prose-first Arrange-Act-Assert comparison: write temporary prose for each section, refactor until
the code expresses it, remove redundant prose, and retain only irreducible context. Record this
review in task evidence or a file-local test plan. Assess helper boundaries by meaningful named
actions and abstraction-level alignment, not caller count: a single-use helper is valid when it
keeps the test readable and hides only incidental mechanics. Commit each reviewed test-design
increment before starting the next planned file or behavior area. Keep final verification and
completion evidence separate when it improves reviewability. Use a Conventional Commit message
with the narrow affected scope, and sign every commit with GPG.

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/open/3052-whitelist-add-remove-race/ISSUE.md`
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue created (#3052) and issue number added to this spec
- [x] Spec-only PR merged into `develop` before implementation
- [x] Implementation completed
- [x] Automatic verification completed (`linter all`, relevant tests, and any pre-push checks)
- [x] Manual verification scenarios executed and recorded in issue-local `manual-verification-evidence.md`
- [x] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] Evidence-based implementation completion review recorded
- [ ] Reviewer validated acceptance criteria and updated checkboxes
- [ ] Independent reviewer reports recorded in issue-local `agent-review-reports.md` when reviewers received this folder-style specification
- [ ] Committer verified spec progress is up to date before commit
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-11-25 14:15 UTC - Claude Code (Claude Sonnet 5.5) - Reproduced on SQLite (V1), drafted the spec after the maintainer chose idempotent store writes - `manual-verification-evidence.md` V1
- 2026-11-25 14:18 UTC - Claude Code (Claude Sonnet 5.5) - Maintainer approved the spec; GitHub issue #3052 created; spec moved to open/
- 2026-11-25 16:55 UTC - Claude Code (Claude Sonnet 5.5) - Spec PR #3053 merged; added the asserting `persisted.rs` concurrency tests (one for add, one for remove) and observed both red on SQLite - `manual-verification-evidence.md` T3
- 2026-11-25 16:56 UTC - Claude Code (Claude Sonnet 5.5) - Added the concurrent store-level harness cases and observed them red on SQLite - `manual-verification-evidence.md` T3
- 2026-11-25 16:58 UTC - Claude Code (Claude Sonnet 5.5) - Observed the concurrent harness cases red on MySQL and PostgreSQL (T2, V2 before-fix) - `manual-verification-evidence.md` V2
- 2026-11-25 17:04 UTC - Claude Code (Claude Sonnet 5.5) - Made the three drivers idempotent, dropped the read in DatabaseWhitelist, replaced the failure harness tests; green on SQLite, MySQL and PostgreSQL; V1 and V2 rerun; MySQL add row count reported 1 on a duplicate (not a contract) - `manual-verification-evidence.md`

## Acceptance Criteria

- [x] AC1: Two concurrent `DatabaseWhitelist::add` calls for the same info-hash both return `Ok` and the hash is whitelisted once (SQLite regression test in `persisted.rs`; the store-level concurrent harness case on SQLite, MySQL and PostgreSQL via the Docker suites).
- [x] AC2: Two concurrent `DatabaseWhitelist::remove` calls for the same info-hash both return `Ok` and the hash is no longer whitelisted (same two tests as AC1: `persisted.rs` on SQLite, the concurrent harness case on all three drivers).
- [x] AC3: `WhitelistStore::add_info_hash_to_whitelist` succeeds for a new and for an existing hash; `remove_info_hash_from_whitelist` returns `Ok(1)` for a present hash and `Ok(0)` for a missing one, on all three drivers. The add row count is not a contract: MySQL reports `1` for a duplicate (found-rows flag), SQLite and PostgreSQL `0` (documented on the trait).
- [x] AC4: The regression test and the concurrent store-level harness cases were observed red before the fix and green after.
- [ ] `linter all` exits with code `0`
- [ ] Relevant tests pass
- [ ] Manual verification scenarios are executed and documented in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria are re-reviewed after implementation and reflect actual behavior
- [ ] Documentation is updated when behavior/workflow changes (trait docs)

## Verification Plan

### Automatic Checks

- `linter all`
- `cargo test -p torrust-tracker-core` (stable toolchain)
- MySQL and PostgreSQL driver suites as run by `.github/workflows/db-compatibility.yaml`
- Pre-push checks

### Manual Verification Scenarios

| ID  | Scenario                                          | Human-oriented command/steps                                                                   | Expected Result                               | Status | Evidence                                     |
| --- | ------------------------------------------------- | ---------------------------------------------------------------------------------------------- | --------------------------------------------- | ------ | -------------------------------------------- |
| V1  | Concurrent add and remove on SQLite               | Temporary test: 200 rounds of two concurrent `add` then two concurrent `remove` of one hash    | Before fix: failures. After fix: none         | DONE   | `manual-verification-evidence.md` section V1 (before-fix result recorded) |
| V2  | Same on MySQL and PostgreSQL                      | Same temporary test against the Docker databases                                               | Before fix: failures. After fix: none         | DONE   | `manual-verification-evidence.md` section V2 |

### Disposable Verification Scripts

None. The reproduction is a temporary Rust test whose code is recorded verbatim in the evidence file;
it only measures (it counts failures and never asserts). The maintained regression test (T3) is a separate
asserting test and the red evidence comes from it.

### Acceptance Verification

| AC ID | Status | Evidence |
| ----- | ------ | -------- |
| AC1   | DONE   | `persisted.rs` add test and harness case green on SQLite, MySQL, PostgreSQL (evidence T3, V2) |
| AC2   | DONE   | `persisted.rs` remove test and harness case green on the three drivers (evidence T3, V2) |
| AC3   | DONE   | harness idempotency tests green on the three drivers; MySQL add count finding in V2 |
| AC4   | DONE   | red runs in evidence T3 and V2; green runs in V1 and V2 after the fix |

## Risks and Trade-offs

- **Store contract change:** callers can no longer tell "added" from "already present" except by the
  returned row count. Only `DatabaseWhitelist` and the persistence benchmark call these methods; both
  stay valid.
- **MySQL affected-rows semantics:** `ON DUPLICATE KEY UPDATE info_hash = info_hash` was expected to
  report 0 affected rows on a duplicate. Verified on the Docker suite: the driver connects with the
  found-rows flag, so it reports 1. The add row count is therefore not a contract (see AC3); only removal
  counts are asserted.
- **Timing-dependent red run:** mitigated by many rounds on a multi-threaded runtime (200 of 200 failed
  in V1).

## Implementation Completion Review

After implementation, compare the result with this specification. Record
invalidated assumptions, material design changes, unexpected validation
findings, and reusable lessons.

- Retrospective: `Not needed`: the one material discovery (MySQL reports 1 affected row on a duplicate add because of the found-rows flag) is recorded in the Risks section, AC3 and the V2 evidence.
- If needed, create `implementation-retrospective.md` from the repository
  template at `docs/templates/IMPLEMENTATION-RETROSPECTIVE.md` in this issue
  specification's directory.
- If no retrospective is needed, add a concise progress-log entry explaining
  why the work had no material discovery.
- When an independent reviewer receives this folder-style specification, it
  records its result in `agent-review-reports.md` using
  `docs/templates/AGENT-REVIEW-REPORTS.md`. Do not create that artifact for a
  legacy standalone specification or when no folder-style specification was
  supplied.

## References

- Related issues: #2902 (finding F4), EPIC #1347
- Related ADRs: none
