---
semantic-links:
  related-artifacts:
    - .github/agents/task-reviewer.agent.md
    - docs/issues/open/2637-1347-add-tests-axum-rest-api-server/ISSUE.md
---

# Agent Review Reports - Issue #2637 Add Tests to the axum-rest-api-server Package

> Append one completed independent-review entry at a time. Do not modify, reorder, or remove
> earlier entries. A correction is a new entry that names the earlier conclusion.

## Reports

### 2026-10-27 07:21 UTC - Task Reviewer (slice S1, first review)

- Invocation scope: Slice S1 of #2637 on branch `2637-1347-add-tests-axum-rest-api-server-auth`: the commits above
  `torrust/develop`, including `test(axum-rest-api-server): [#2637] cover the token authentication middleware`
  and `test(axum-rest-api-server): [#2637] cover the shared response contract`.
- Inputs: the issue folder (`ISSUE.md`, `coverage-evidence.md`, `lessons.md`, `test-refactor-plans/`), the EPIC
  and the subissue guidelines, the `write-unit-test` skill, and the two changed Rust files.
- Evidence: the reviewer recomputed the three coverage scopes with clean `cargo llvm-cov` runs (stable Rust
  1.99.1, `cargo-llvm-cov 0.6.16`) and every recorded figure matched; per-file `auth.rs` 142 / 142 and
  `responses.rs` 66 / 66 lines; the EPIC deltas are correct. It applied and reverted six named mutations
  (M1, M3, M5, M8, E4, E7) and each was killed. `cargo +nightly fmt --all -- --check` (nightly Rust toolchain),
  `git diff --check`, `cargo machete --with-metadata`, `cargo clippy --all-targets`, `linter markdown` and
  `linter cspell` passed. No production code changed and no closing keyword is in any commit message.
- Findings:
  - MEDIUM: the S1-close progress-log entries and the `last-updated-utc` stamps of `ISSUE.md` and `EPIC.md` said
    07:40 UTC, later than the commit that wrote them (07:16 UTC).
  - MEDIUM: the finished auth plan still said it needed the maintainer's approval.
  - LOW: EPIC row 10 still said S1 had "started".
  - LOW: the slice status `IN_PROGRESS (PR open next)` is a stale-prone annotation, and T3 stayed `TODO`.
  - LOW: the auth plan said M3 was killed by four cases; eleven fail.
  - LOW: the response tests bound the observed response to a variable named like the helper (`sent`).
  - LOW (note): the cases hard-code the token literals while `send` hides the configured tokens.
- Verdict: REVIEW FAILED (documentation fixes only)
- Follow-up actions:
  - Fix the stamps, the stale approval sentence, the EPIC row, the slice status, T3 and the M3 count; rename the
    shadowing variable. All done in `test(axum-rest-api-server): [#2637] stop shadowing the response helper in the response tests`
    and `docs(issues): [#2637] correct the S1 stamps and stale statuses found by the independent review`.
  - Leave the hard-coded token literals as they are: the literals are the independently specified inputs, which
    the `write-unit-test` skill prefers over constants shared with the production fixture.
  - A short re-review of those edits follows.

### 2026-10-27 07:27 UTC - Task Reviewer (slice S1, re-review)

- Invocation scope: the three commits after the first S1 review: `test(axum-rest-api-server): [#2637] stop shadowing the response helper in the response tests`, `docs(issues): [#2637] correct the S1 stamps and stale statuses found by the independent review` and `docs(issues): [#2637] record the first S1 independent review report`. The fixes listed as done in the first entry were applied after that review ran (07:24 to 07:25 UTC), not at its time.
- Inputs: the first report, the three commits, the issue folder and the EPIC.
- Evidence: `cargo test -p torrust-tracker-axum-rest-api-server` (46 unit and 58 integration tests pass), `cargo +nightly fmt --all -- --check` (nightly Rust toolchain), `cargo clippy -p torrust-tracker-axum-rest-api-server --all-targets` (stable), `linter markdown` and `linter cspell` passed; `git log --format=%aI` shows no new stamp postdates its commit; the tree was unchanged by the review. The reviewer did not rerun the coverage or mutation runs of the first review.
- Findings:
  - LOW: the first entry is stamped 07:21 UTC but describes fixes made at 07:24 to 07:25 UTC; this entry states that order. No further action.
  - LOW: one line of the auth plan's Results is long; it passes the linter.
- Verdict: REVIEW PASSED
- Follow-up actions:
  - None.

### 2026-10-28 16:50 UTC - Task Reviewer (slice S2, first review)

- Invocation scope: slice S2 of #2637 on branch `2637-1347-add-tests-axum-rest-api-server-routes`: the commits above `torrust/develop`, including `refactor(axum-rest-api-server): [#2637] separate the router layers from the routes` and the test commits for `v1/routes.rs`, `routes.rs` and the health check handler.
- Inputs: the issue folder, the EPIC, the subissue guidelines, the `write-unit-test` skill, ADR `20261021190318` rule C, and the changed Rust files.
- Evidence: the reviewer recomputed the three coverage scopes with clean `cargo llvm-cov` runs (stable Rust 1.99.1, `cargo-llvm-cov 0.6.16`) and every recorded figure matched, including the "before" figures rebuilt from a `develop` snapshot and the six EPIC deltas; the production refactor commit is a pure move (same chain and order, same 5 s default, same `#[instrument]`); eight mutations (G1, G5, L1, L5, L8, H1, L6, L10) behaved as the plans say, L6 and L10 surviving by design; `cargo +nightly fmt --all -- --check` (nightly Rust toolchain), `cargo clippy --all-targets`, `cargo machete --with-metadata`, `git diff --check`, `linter markdown` and `linter cspell` passed; no closing keyword and no cited branch SHA; every log stamp was at or before its commit.
- Findings:
  - MEDIUM: the row 4 plan approval and completed-file approval were recorded in the plan index and plan but not in the progress log.
  - LOW: package-level figures in two plans were measured before the rebase onto `develop` and were not labelled as such.
  - LOW: the routes plan's case table expected a content type that the tests do not assert.
  - LOW: the timeout test bound a variable named like the helper that reads the response (`received`).
  - LOW: one rejection test name was narrower than its cases (a wrong token).
  - LOW: the note on log coverage did not say that the integration test most likely protects only the server-error response event and not the `on_failure` closure.
  - LOW (declined): `v1/routes.rs`'s helper returns a content type that three tests ignore; the two `409` tables assert it, so the helper stays as it is.
- Verdict: REVIEW PASSED
- Follow-up actions:
  - The approvals were added to the progress log, the figures were labelled, the table and the log note were corrected, and the helper and the test were renamed: `test(axum-rest-api-server): [#2637] stop shadowing the response reader in the router tests` and `docs(issues): [#2637] fix the records items of the S2 independent review`.

### 2026-11-03 17:23 UTC - Task Reviewer (slice S3, first review)

- Invocation scope: slice S3 of #2637 on branch `2637-1347-add-tests-axum-rest-api-server-commands`: the seven commits above `torrust/develop` (ledger rows 6-11), including `test(axum-rest-api-server): [#2637] cover the auth key handlers` and `test(axum-rest-api-server): [#2637] cover the whitelist handlers`.
- Inputs: the issue folder, the EPIC, the two changed Rust files, `Cargo.toml` and `Cargo.lock`. The reviewer did not re-read the subissue guidelines or the `write-unit-test` skill in full and judged AAA and naming against the plans' prose-first tables. It was told not to modify tracked files, so this entry was written by the implementing agent from its report.
- Evidence: clean `cargo llvm-cov` runs (stable Rust 1.99.1, `cargo-llvm-cov 0.6.16`) matched every recorded figure: aggregate 1,806 / 1,931 (93.53%), unit-only 1,744 / 1,931 (90.32%), integration-only 682 / 916 (74.45%); the per-file figures of all six files; the uncovered-line sum 187; and the six EPIC deltas. Ten named mutations were re-applied and killed by assertions (A2, A4, F1, W3, W5, W10, the whitelist responses M2, the auth-key responses M8, the whitelist routes M3 and M4) and reverted exactly. `cargo +nightly fmt --all -- --check` (nightly Rust toolchain), `git diff --check`, `cargo clippy --all-targets`, `cargo machete --with-metadata`, `linter markdown` and `linter cspell` passed; all relative links and the anchor resolve; no production code changed, no closing keyword and no commit id in the docs; no stamp is later than its commit.
- Findings:
  - MEDIUM: the whitelist routes plan, the evidence and the progress log said the method mutants are killed "by the integration tests only". The reviewer's variants (the method replaced and left unregistered for the original verb, so the route answers `405`) are killed by the `v1/routes.rs` table; only the variants with a dummy handler need the integration tests. Reproduced by the implementing agent.
  - MEDIUM: the EPIC `last-updated-utc` was older than its own newest log entry.
  - LOW: the auth-key handlers plan said F1 is killed by "the four forwarding assertions"; there are three.
  - LOW: `lessons.md` said the whitelist `add` and `remove` `500` is pinned by a test; only `add` is.
  - LOW (declined): a few reload and delete tests pass an irrelevant key outcome to the fake; harmless, and the duplicated helpers are already in `lessons.md`.
- Verdict: REVIEW FAILED (documentation only)
- Follow-up actions:
  - The two MEDIUM and two LOW items were fixed in one documentation commit: the routes plan now records both kinds of method mutant (M3, M4 and M3b, M4b), the evidence and log wording is corrected, the EPIC stamp is updated, and the F1 count and the `lessons.md` wording are corrected. A short re-review of the documentation follows.

### 2026-11-03 17:24 UTC - Task Reviewer (slice S3, re-review)

- Invocation scope: the commit `docs(issues): [#2637] record the first S3 review and correct its documentation findings`, which answers the four findings of the first S3 review.
- Inputs: that commit, the issue folder and the EPIC.
- Evidence: the four fixes were verified (the routes plan, evidence and log now separate a method replaced by a dummy handler, killed only by the integration tests, from a dropped method, killed by the `v1/routes.rs` table and reproduced; the EPIC stamp; the three forwarding assertions; the `lessons.md` wording). No stamp is later than its commit; every relative link and anchor resolves; `linter markdown` and `linter cspell` pass; `git diff --check` is clean; the tree is clean. The reviewer did not re-run the dummy-handler variants M3 and M4 and relied on the implementing agent's earlier run. The commit changes only documentation, so the Rust checks of the first review stand. As before, this entry was written by the implementing agent from the report.
- Findings:
  - None.
- Verdict: REVIEW PASSED
- Follow-up actions:
  - None.

### 2026-11-04 13:38 UTC - Task Reviewer (slice S4, first review)

- Invocation scope: slice S4 of #2637 on branch `2637-1347-add-tests-axum-rest-api-server-queries`: the nine commits above `torrust/develop` (ledger rows 12-17), including `test(axum-rest-api-server): [#2637] cover the torrent handlers`, `test(axum-rest-api-server): [#2637] cover the stats handlers` and `test(axum-rest-api-server): [#2637] cover the stats responses`.
- Inputs: the issue folder, the EPIC, the S1-S3 entries of this file and the four changed Rust files (`torrent/handlers.rs`, `stats/handlers.rs`, `stats/responses.rs`, `test_support.rs`). The reviewer did not re-read the subissue guidelines or the `write-unit-test` skill in full and judged AAA and naming against the plans' prose-first tables and the S3 decisions. It was told not to modify tracked files, so this entry was written by the implementing agent from its report.
- Evidence: clean `cargo llvm-cov` runs (stable Rust 1.99.1, `cargo-llvm-cov 0.6.16`) matched every recorded figure: aggregate 2,091 / 2,175 (96.14%), unit-only 2,067 / 2,175 (95.03%), integration-only 682 / 916 (74.45%), the per-file figures, and the 108 uncovered unit-only lines (`server.rs` 74, `testing/environment.rs` 31, `auth_key/handlers.rs` 3). Twelve mutations were re-applied and each was killed by an assertion, except the equivalent S9, which survives as the plan says; the claims that `axum_extra` reads an empty `offset=` as absent (T11 survives the query cases) and that the stats handler swap is killed only by the integration tests were confirmed. `cargo +nightly fmt --all -- --check` (nightly Rust toolchain), `cargo clippy --all-targets`, `cargo machete --with-metadata`, `git diff --check`, `linter markdown` and `linter cspell` passed; the EPIC deltas are arithmetically correct; no stamp is later than its commit; no commit id and no closing keyword; no production code changed; the `TODO(#2637)` comment points only to a tracked folder.
- Findings:
  - LOW: the S4 paragraph of the evidence and the EPIC log said every file of the v1 contexts is fully covered by unit tests, but three `Debug` lines of the auth-key handlers are not.
  - LOW: the torrent routes plan read as if the handler swap were observable only through a real service, while the unit read-routes test also kills it for the torrent routes (not for the stats routes).
  - LOW: the stats responses plan listed one killer for P4; the Prometheus case of the stats handler test also kills it.
  - LOW (observation): the doc comment of `get_torrent_handler` says `500` for an unknown torrent while the handler and the module documentation say `200`. Production documentation outside this issue's scope.
  - LOW (note): the handler tests assert the start or a part of a body as a boolean in a tuple; deliberate, because the full bodies are pinned in `stats/responses.rs`.
- Verdict: REVIEW PASSED
- Follow-up actions:
  - The three plan and evidence wording items were corrected, and the doc comment observation was added to `lessons.md`; the boolean assertion note was declined as deliberate. No further review is required.

### 2026-11-06 08:23 UTC - independent reviewer (slice S5, first review)

Written by the implementing agent (Claude Code, Sonnet 5.5) from the report of an independent review subagent that ran in an isolated worktree and changed no tracked file.

- Invocation scope: slice S5 of #2637 on branch `2637-1347-add-tests-axum-rest-api-server-lifecycle`: the commits above `torrust/develop` (the post-merge records, the F3 and F4 test-only fix, the row 18 plan and tests, and the re-measure after the rebase). Ledger row 18.
- Inputs: the issue folder, the EPIC, the write-unit-test and review-task skills, and the changed Rust files (`server.rs`, `test_support.rs`, `routes.rs`, `v1/routes.rs`, `Cargo.toml`).
- Evidence: clean `cargo llvm-cov` runs (stable Rust 1.99.1, `cargo-llvm-cov 0.6.16`) matched every recorded figure (unit-only lines 2,321 / 2,400, aggregate 2,345 / 2,400, integration-only 682 / 903, 167 unit and 58 integration tests, 45 uncovered `server.rs` lines); the EPIC deltas are arithmetically correct; mutations S1-S7, S8b and S9 were killed by assertions or the deadline, S8 survived; the F3 and F4 checks reproduced (a pending future failed 19 `v1::routes` tests and 8 `routes` tests with the deadline message, and the `data_*.db` count in the temporary directory did not change); fmt, clippy, machete, `git diff --check`, markdown and cspell passed; only test code, `test_support`, a dev-dependency and docs changed; stamps are not later than their commits; no closing keywords or commit SHAs.
- Findings:
  - MEDIUM: the S8 "equivalent mutant" claim was wrong. `cancel()` cancels the caller's token (the production caller passes a clone), which the registration-failure test never inspected.
  - LOW: the plan index showed row 18 as `IN_PROGRESS` while the plan and ledger said `DONE`.
  - LOW: the body of `server-tests.md` (Results, expected effect, the #2733 row) still carried pre-rebase figures; only the log corrected them.
  - LOW: the `check_fn` tests (N2, N3) use `reqwest::Client::new()`, which honors a proxy in the environment.
  - INFO: `routes.rs` keeps its own `DEADLINE` next to the shared `TEST_DEADLINE`; it predates the slice.
- Verdict: REVIEW PASSED
- Follow-up actions:
  - The registration-failure test now asserts that the caller's token is cancelled (`test(axum-rest-api-server): [#2637] assert that a failed registration cancels the caller's token`); S8 and S8b are killed.
  - The plan index, the plan body and the figures were refreshed in one documentation commit; the proxy note is recorded in the plan's review outcome.
  - The `DEADLINE` of `routes.rs` is declined as out of scope (not touched by this slice).

### 2026-11-06 16:44 UTC - independent reviewer (slice S6, final review)

Written by the implementing agent (Claude Code, Sonnet 5.5) from the report of an independent review subagent that ran in an isolated worktree and changed no tracked file.

- Invocation scope: slice S6 of #2637 on branch `2637-1347-add-tests-axum-rest-api-server-final`: ledger rows 19-31, the `lib.rs` documentation fix, the `authenticate` duplicate-token test, the mutation and manual evidence, the final measurement and the retrospective.
- Inputs: the issue folder, the EPIC rows and tables, the S6 plans, the review-task and write-unit-test skills, and the changed Rust files.
- Evidence: clean `cargo llvm-cov` runs (stable Rust 1.99.1, `cargo-llvm-cov 0.6.16`) matched every recorded figure (aggregate 2,348 / 2,404 lines, 97.67%; unit-only 2,324 / 2,404, 96.67%; integration-only 682 / 903; 170 unit and 58 integration tests) and both EPIC tables with their deltas; `cargo test`, `cargo +nightly fmt --all -- --check` and `linter all` passed; replacing `|` with `^` in `authenticate` made exactly the new shared-value case fail.
- Findings:
  - LOW: the unviable-mutant categories of `mutation-evidence.md` overlapped and did not add up to 44.
  - LOW: the auth-key handlers plan still said "Review outcome: pending" although the maintainer had approved the file.
  - LOW (INFO): the `500` statements in `lessons.md` and the manual evidence are supported by `unhandled_rejection_response` and the `auth.rs` unit tests.
- Verdict: REVIEW PASSED
- Follow-up actions:
  - The unviable categories were recounted from the per-mutant logs (16, 16, 4 and 8, total 44) and the stale review outcome was corrected, both in the final documentation commit. No further review is required.
