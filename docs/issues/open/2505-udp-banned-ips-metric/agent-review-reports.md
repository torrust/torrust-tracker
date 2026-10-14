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
