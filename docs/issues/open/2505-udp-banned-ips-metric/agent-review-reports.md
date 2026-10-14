---
semantic-links:
  related-artifacts:
    - .github/agents/complexity-auditor.agent.md
    - .github/agents/task-reviewer.agent.md
    - .github/agents/pr-reviewer.agent.md
    - docs/agents/orchestration.md
---

# Agent Review Reports - The UDP Banned-IPs Metric Counts Clients That Are Not Banned

> Append one completed independent-review entry at a time. Do not modify, reorder, or remove
> earlier entries. A correction is a new entry that names the earlier conclusion.

## Reports

### 2026-10-14 06:45 UTC - Task Reviewer

- Invocation scope: Pre-PR review of issue #2505 on branch `2505-udp-banned-ips-metric`, range
  `torrust/develop..HEAD` (9 commits on top of `b5f1801c6`): acceptance criteria AC1-AC7 and the
  generic checklist items, scope, regression-test strength and design, documentation accuracy,
  commit hygiene, and evidence accuracy.
- Inputs: `ISSUE.md`, `manual-verification-evidence.md`, `performance-evidence.md`,
  `implementation-retrospective.md`, `evidence/recheck-tracker.log`, the full code diff
  (`packages/udp-core/src/services/banning.rs`, `packages/udp-core/benches/ban_service_benchmark.rs`,
  `packages/udp-core/Cargo.toml`, `Cargo.lock`, `packages/udp-server/src/banning/mod.rs`,
  `packages/udp-server/src/banning/event/handler.rs`, `packages/udp-server/src/banning/event/listener.rs`,
  `packages/udp-server/src/statistics/metrics.rs`, `packages/udp-server/src/statistics/mod.rs`,
  `packages/udp-server/tests/server/contract.rs`,
  `packages/rest-api-protocol/src/v1/context/stats/resources/stats.rs`,
  `src/bootstrap/jobs/udp_tracker_server.rs`), the commit messages, `AGENTS.md`, the
  `write-unit-test` and `review-task` skills.
- Evidence (all with stable `rustc 1.99.0 (b940084d7 2026-09-28)`):
  - HEAD: `cargo test -p torrust-tracker-udp-core -p torrust-tracker-udp-server` green (49, 220, 16
    integration, doc tests); `cargo test -p torrust-tracker --lib bootstrap::jobs::udp_tracker_server`
    green (2); root `tests/banning/*` and `metrics-fixed-ports` green; R5/R6 module green 5 times in a
    row; `linter all` exit 0 with no working-tree change; `cargo doc --no-deps` with
    `RUSTDOCFLAGS="-D warnings"` for udp-core, udp-server, and rest-api-protocol exit 0.
  - Per commit (clean `git archive` export with fresh file modification times, separate target dir): every code
    commit (`test(udp-core)` benchmark, `fix(udp-core)`, `fix(udp-server)`, `docs(udp-server)`,
    `perf(udp-core)`) passes `cargo check --workspace --all-targets`, the udp-core and udp-server
    suites, and the root `bootstrap::jobs` tests. Test counts grow as expected (udp-server
    integration 12 to 16 at `fix(udp-core)`; udp-server lib 219 to 220 and root jobs 31 to 32 at
    `fix(udp-server)`).
  - Independent red check on a HEAD export: restoring the old map-length total fails R1 (two
    below-limit cases), R1p, and both R5/R6 below-limit tests; restoring the direct
    `ban_service.write().await.reset_bans()` call in the job fails R4 with `left: 1, right: 0`.
  - `git diff 3a683c5fc HEAD -- packages src Cargo.toml Cargo.lock` shows only the crossing check in
    `packages/udp-core/src/services/banning.rs` (2 lines).
  - Commits are GPG-signed (`%G?` = `U`), Conventional Commits with `[#2505]`; evidence cites
    branch commits by subject only (no branch SHA found in the issue folder); timestamps in the
    evidence are consistent with commit times and log lines.
  - Acceptance-criteria matrix:

    | Criterion | Result | Evidence |
    | --------- | ------ | -------- |
    | AC1 banned total counts only IPs over the limit, both modes | PASS | `banning.rs` counter at `limit + 1`; R1, R1p, R5, R6 green; red reproduced independently; M1-M4, M1b, M3b recorded |
    | AC2 total and gauge correct after reset | PASS | `udp-server` `banning::reset_bans`, job wiring; R2, R3, R4 green; R4 red reproduced independently; M5 recorded |
    | AC3 descriptions match behavior incl. `disabled` meaning | PASS | method, field, metric description, `Metrics::udp_banned_ips_total`, REST field docs; no other docs describe the value |
    | AC4 `BanningStats` and `last_connection_id_errors_reset` removed | PASS | no references outside issue docs |
    | AC5 P1/P2 recorded, pass rule, noise stated | PASS (accepted deviation) | `performance-evidence.md` "Comparison": load test passes (158456.71 vs 148018.51); 9 of 28 Criterion medians over the limit, each explained; deviation stated explicitly in the AC text |
    | AC6 reproduction repeated after the fix and recorded | PENDING | recheck R-V1 to R-V4 ran on the `docs(udp-server)` build, before `perf(udp-core)` changed the crossing check; see finding B1 |
    | AC7 per-event cost does not grow with tracked IPs | PASS | O(1) counter, no map scan; `record_error` P2 medians flat 10-10,000 IPs; B1 instrument check shows a scan grows linearly |
    | `linter all` exits 0 | PASS | rerun by reviewer, exit 0 |
    | Relevant tests pass | PASS | reviewer runs above, HEAD and each commit |
    | Manual scenarios executed and documented | PENDING | not executed on the finished artifact (finding B1) |
    | AC re-reviewed and reflect actual behavior | PENDING | AC6 evidence and the recheck "Code:" line describe a pre-final build (finding B1) |
    | Documentation updated | PASS | AC3 |

- Findings:
  - B1 (blocking, manual evidence / AC6): the like-for-like recheck was executed against the build
    of `docs(udp-server): [#2505] describe the banned-IPs metric as IPs over the limit`
    (`manual-verification-evidence.md` "Recheck After the Fix (T10)", the "Code:" bullet says it is
    "the last commit of the fix"). The later `perf(udp-core): [#2505] detect the ban crossing with
    one comparison` changed the production counting expression
    (`packages/udp-core/src/services/banning.rs` lines 37-41), so the finished artifact was never
    exercised manually, and the "last commit of the fix" statement is no longer accurate. The
    Summary table, ISSUE AC6, and the retrospective do not mention this. Unit tests and mutations
    cover variant B, but automated output is not manual evidence.
  - N1 (non-blocking, AC5 record): the maintainer decision recorded in the 2026-10-14 05:32 progress
    log entry (and in `performance-evidence.md` "P2 Investigation", dated 2026-10-13) predates the
    final P2 run (05:44-06:01 UTC), which adds new failures (`is_banned/ipv6_above_threshold/10000`,
    a larger `increase_counter/repeated/ipv6` excess). The deviation is documented clearly and
    honestly; record explicitly that the maintainer reviewed the final 9-failure table, and align
    the decision date between the two documents.
  - N2 (non-blocking, test strength): the renamed handler test
    `it_should_publish_the_banned_ip_total_after_a_connection_cookie_error`
    (`packages/udp-server/src/banning/event/handler.rs` lines 133-161) expects 2 with two tracked
    IPs that are both over the limit, so it also passes with the old map-length total (verified).
    It guards publication only. Adding one tracked client below the limit would make it
    distinguish the two meanings.
  - N3 (non-blocking, test determinism): R5/R6 (`packages/udp-server/tests/server/contract.rs`
    lines 706-766) poll with `sleep` under a 5 s deadline. This is inherent to the package-level
    asynchronous layer the spec required and was stable in 8 + 8 + 5 runs. The below-limit tests
    read the gauge once the L-th error is recorded but possibly before the L-th publication, so an
    off-by-one-at-limit regression would be caught only probabilistically there; R1 and R1p catch it
    deterministically. Consider a short comment noting R1 owns the boundary.
  - N4 (non-blocking, commit hygiene): `perf(udp-core): [#2505] detect the ban crossing with one
    comparison` also carries T10 evidence (`evidence/recheck-tracker.log`) and evidence-doc changes,
    and T9 and T10 evidence share one `docs(issues)` commit although the Commit Points table lists
    them separately. History must not be rewritten (AGENTS.md); mention it in the PR body if
    relevant.
  - N5 (non-blocking, evidence): the variant-B section claims the udp-server, root reset-job, and
    root banning suites and clippy passed without a verbatim output block. The reviewer reproduced
    the suites green at that commit, so no action is required beyond optionally adding the output.
  - N6 (non-blocking, duplication): `connection_cookie_error_from` is duplicated in
    `packages/udp-server/src/banning/mod.rs` tests and `src/bootstrap/jobs/udp_tracker_server.rs`
    tests; acceptable across crates.
  - Test design checklist: names state one behavior and condition; causal state (limit, error
    counts, policy) is visible in each Arrange; `record_connection_id_errors`,
    `with_client_errors`, `send_invalid_connection_ids`, and `start_tracker_with_connection_id_validation`
    name coherent actions or states and are not parameter bags; production Acts are visible;
    expected values are independent (R1p uses its own model); the prose-first AAA comparison is
    recorded for R1-R6 and the updated tests. Only N2 and N3 noted.
  - Scope: no unrelated changes; quickcheck is a dev-dependency with `default-features = false`
    and was already in `Cargo.lock`.
- Verdict: REVIEW FAILED
- Follow-up actions:
  - Implementer (B1): build HEAD (`cargo build --bin torrust-tracker`), rerun R-V1 to R-V3 with the
    same configuration and probe, and record the commands and verbatim output in
    `manual-verification-evidence.md`, citing `perf(udp-core): [#2505] detect the ban crossing with
    one comparison`. For R-V4, either rerun it (one reset interval) or record a maintainer-accepted
    rationale that the reset path is byte-identical since the earlier recheck build (the code diff
    from that build to HEAD is the 2-line crossing check only). Correct the "last commit of the fix"
    wording, then update AC6, the manual-verification checkbox, and the acceptance review.
  - Implementer (N1): record the maintainer's explicit acceptance of the final P2 table and align the
    decision date.
  - Optional: N2 and N3 improvements; N5 output block.
  - Issue spec: no checkboxes were changed in this review because the caller restricted edits to
    this report file; the Workflow Checkpoint "Reviewer validated acceptance criteria and updated
    checkboxes" stays open until B1 is resolved and a follow-up review passes.

### 2026-10-14 08:45 UTC - Task Reviewer (follow-up)

- Invocation scope: Follow-up pre-PR review of issue #2505 on branch `2505-udp-banned-ips-metric`,
  range `torrust/develop..HEAD` (11 commits on top of `b5f1801c6`), after the 2026-10-14 06:45 UTC
  report (REVIEW FAILED on B1). New commits reviewed:
  `test(udp-server): [#2505] make the handler test tell banned IPs from tracked IPs` and
  `docs(issues): [#2505] recheck the finished code and record the task review`. Goals: B1 resolved
  on the finished code, N1 and N2 addressed, new commits green, spec status accurate, no regression.
- Inputs: the earlier report above, `ISSUE.md`, `manual-verification-evidence.md` ("Final Recheck
  on the Finished Code (T10)", "Reviewer Follow-Up - Handler Test Distinguishes the Meanings (N2)",
  "Failures and Follow-up"), `performance-evidence.md`, `implementation-retrospective.md`,
  `evidence/final-recheck-tracker.log`, `packages/udp-server/src/banning/event/handler.rs`, commit
  metadata, the local `.tmp/udp-banned-ips-metric/repro/` working folder (read only).
- Evidence (stable `rustc 1.99.0 (b940084d7 2026-09-28)`):
  - B1 timeline: `test(udp-server)` committed 06:51:19 UTC; `target/debug/torrust-tracker` mtime
    06:52:01 UTC; discarded start 06:52:12 UTC; final start 06:53:20 UTC (log first line and
    `start-time.txt`); reset 07:53:20 UTC; SIGTERM 07:53:45 UTC. `perf(udp-core)` (05:41:58 UTC)
    precedes the build. `git diff ebeee743c HEAD -- packages src Cargo.toml Cargo.lock` touches only
    the handler test, so the binary carries the variant-B production code. `cargo build --bin
    torrust-tracker` at HEAD with a clean tree is a no-op (`Finished ... in 1.58s`, nothing
    compiled), so the existing binary matches the HEAD sources.
  - The local `.tmp/.../repro/tracker.log` is byte-identical to the committed
    `evidence/final-recheck-tracker.log`; `probe.sh` is identical to `udp-banned-ips-probe.sh`
    (unchanged since the base); `tracker.toml` is identical to the "Tracker configuration" block;
    the logged configuration matches (limit 10, `strict`, reset 3600 s). The log shows 1, 2, and 11
    cookie errors at the F-V1, F-V2, and F-V3 times recorded in the evidence, no overlap between
    steps, the new metric description ("... exceed the limit ..."), the
    `connection id errors filter cleared` reset at 07:53:20, and one cookie error at 07:53:37 for
    F-V4 step 3. The documented values (0, 0, 1, 0 after the reset) are consistent with the log.
  - N2: reproduced M0 on a `git archive HEAD` export (not the worktree) with the map-length total
    restored: the handler test fails with `left: 3`, `right: 2` (1 passed, 1 failed). Scratch copy
    removed.
  - N1: `performance-evidence.md` lines 380-383 date the decision to 2026-10-13 after the 21:35:33
    UTC comparison and explain the 05:32 progress-log stamp; lines 464-466 record the maintainer's
    confirmation covering the final nine-failure table.
  - Tests at HEAD (code identical to `test(udp-server)`; the later commit is docs only):
    `cargo test -p torrust-tracker-udp-core -p torrust-tracker-udp-server` green (49, 220, 16, doc
    tests); `cargo test -p torrust-tracker --lib bootstrap::jobs::udp_tracker_server` green (2); the
    three root `banning-*` test targets green. `linter all` exit 0, working tree unchanged.
  - Acceptance-criteria matrix:

    | Criterion | Result | Evidence |
    | --------- | ------ | -------- |
    | AC1 | PASS | unchanged since the earlier report; handler test now also distinguishes the meanings (M0 reproduced) |
    | AC2 | PASS | unchanged; F-V4 shows 0 right after the reset on the finished code |
    | AC3 | PASS | unchanged; the logged metric description matches |
    | AC4 | PASS | unchanged |
    | AC5 | PASS (accepted deviation) | N1 resolved: decision date aligned, maintainer confirmation of the final P2 table recorded |
    | AC6 | PASS | F-V1 to F-V4 on the final code, cross-checked against commit times, binary mtime, no-op rebuild, and the committed log |
    | AC7 | PASS | unchanged |
    | `linter all` exits 0 | PASS | rerun, exit 0 |
    | Relevant tests pass | PASS | runs above |
    | Manual scenarios executed and documented | PASS | "Final Recheck on the Finished Code (T10)"; see N7 for a stale table reference |
    | AC re-reviewed and reflect actual behavior | PASS | AC6 row and progress log updated |
    | Documentation updated | PASS | unchanged |

- Findings:
  - B1 (earlier report): resolved.
  - N1 and N2 (earlier report): resolved as described. N3, N5, N6 accepted; N4 disposition (no
    history rewrite, mention in the PR body) is acceptable.
  - B2 (blocking, test-design evidence): the handler test changed in
    `test(udp-server): [#2505] make the handler test tell banned IPs from tracked IPs`
    (`packages/udp-server/src/banning/event/handler.rs` lines 133-163), but no prose-first
    Arrange-Act-Assert comparison is recorded for the changed test. The only recorded design review
    of this test (`manual-verification-evidence.md` lines 1073-1075, "Test Design Review (T4, T5)")
    describes the old causal state (one client over the limit and the event client at the limit)
    and no longer matches the code; the N2 section (lines 1248-1281) records the mutation proof
    only. The test itself is sound (causal state visible in `with_client_errors(&[...])`,
    production Act visible, expected 2 stated independently, failure message names all three
    clients).
  - N7 (non-blocking, spec accuracy): the Manual Verification Scenarios table in `ISSUE.md` lines
    591-594 still cites "V1 and R-V1" ... "V4 and R-V4" as evidence; R-V1 to R-V4 ran on the
    pre-final build. Cite F-V1 to F-V4 (optionally keeping R-V1 to R-V4).
  - N8 (non-blocking, completion review): `implementation-retrospective.md` was not updated with the
    lessons from the follow-up: a manual recheck must follow the last production change (B1), and
    sequential manual steps must not be issued as parallel tool calls (the discarded interleaved
    run). Both are reusable process lessons; add them under "What Changed During Implementation" or
    "Improvements for Future Work".
  - N9 (non-blocking, naming): `client_within_the_limit_ip` is described as "at the limit" in the
    assertion message and the evidence; `client_at_the_limit_ip` would match. Optional.
- Verdict: REVIEW FAILED
- Follow-up actions:
  - Implementer (B2): add a short "Test design review (prose-first AAA)" record for the changed
    handler test to `manual-verification-evidence.md` (N2 section): Arrange - one client over the
    limit, the event client and a third client at the limit; Act - one connection-cookie error from
    the event client through `handle_event`; Assert - the published total is 2 (stated
    independently, the old map-length meaning gives 3). State that the temporary prose was removed
    and the code expresses it. Do not rewrite the earlier T4/T5 record; note that it is superseded
    for this test.
  - Implementer (optional, same commit): N7 table evidence, N8 retrospective lessons, N9 rename.
  - Issue spec: no checkboxes were changed (the caller restricted edits to this report file).
    "Reviewer validated acceptance criteria and updated checkboxes" stays open until B2 is
    resolved; all other checked items were verified as accurate.
