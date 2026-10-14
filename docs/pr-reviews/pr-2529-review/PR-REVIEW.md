---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
    - docs/issues/closed/2521-1669-remove-unused-udp-stats-traits/ISSUE.md
---

<!-- skill-link: process-pr-review -->

# PR #2529 Review Audit

Source: pull-request reviews and inline review threads for <https://github.com/torrust/torrust-tracker/pull/2529>.

## Ownership

The PR author owns this tracked audit record. Reviewers, including repository review agents,
deliver findings through GitHub and have no repository-artifact obligation.

- Post-merge workflow approval: N/A

## Status Values

- Relationship: `ORIGINAL`, `RE_RAISE_OF:<FindingId>`
- Disposition: `FIXED`, `NO_ACTION`, `SUPERSEDED`, `FOLLOW_UP`
- Thread state: `OPEN`, `RESOLVED`, `NON_RESOLVABLE`, `SUPERSEDED`. `OPEN` applies prospectively
  to audits created or updated for approved follow-up work; historical audits remain valid without
  bulk migration.
- Severity: `Blocker`, `Major`, `Minor`, `Nit`, `Suggestion`; append `(inferred)` when derived
  from free prose.
- Author class: `Copilot`, `Human`, `Unknown`
- Category: `link-integrity`, `formatting`, `metadata`, `testing`, `correctness`,
  `documentation`, `maintainability`, `security`, `other`
- An outdated thread whose concern was fixed is `FIXED`/`RESOLVED`, even when GitHub marks the
  original thread outdated after the push. For in-PR feedback, use `NO_ACTION`/`SUPERSEDED` only
  for a duplicate, superseded, or no-change concern. A post-merge `NO_ACTION` requires maintainer
  approval to decline the follow-up work.

## Findings

Review 5520183826 (round 1, `copilot-pull-request-reviewer`, state `COMMENTED`, submitted
2026-10-14 15:12 UTC on the head whose tip commit was
`docs(issues): [#2521] note that #2505 removed BanningStats`,
<https://github.com/torrust/torrust-tracker/pull/2529#pullrequestreview-5520183826>) left three inline comments without severity brackets; its
overview marks each "Low severity". They are `F1` to `F3` in creation order, the same numbering the
human review below uses. The review body is an overview with no further request.

Review 5520537884 (round 1, `da2ce7`, state `APPROVED`, submitted 2026-10-14 15:38 UTC on the same
head, <https://github.com/torrust/torrust-tracker/pull/2529#pullrequestreview-5520537884>) approved and left three inline comments bracketed
`[Minor][F4]`, `[Nit][F5]`, and `[Nit][F6]`; the rows keep those IDs. GitHub dismissed the approval
at 15:45 UTC as stale, when the push of the F1 to F3 fixes landed (the dismissal event names that
push's head, whose tip was `docs(issues): [#2521] link PR #2529 and record the review fixes`). Its body recommended no change for F1 to F3; that
view is recorded in each of those detail entries. Its other paragraphs report what it checked and
are not further findings.

Reviews 5520654599, 5520655081, and 5520655500 (josecelano, `COMMENTED`, 15:46 UTC) and 5520788265,
5520788662, and 5520789063 (josecelano, `COMMENTED`, 15:56 UTC) are the containers GitHub created
for the replies; none carries a finding.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | `review-finding:pr-2529-f1` | Copilot | Nit (inferred) | documentation | ORIGINAL | FIXED | RESOLVED |
| F2 | `review-finding:pr-2529-f2` | Copilot | Nit (inferred) | documentation | ORIGINAL | FIXED | RESOLVED |
| F3 | `review-finding:pr-2529-f3` | Copilot | Nit (inferred) | maintainability | ORIGINAL | FIXED | RESOLVED |
| F4 | `review-finding:pr-2529-f4` | Human | Minor | testing | ORIGINAL | FIXED | RESOLVED |
| F5 | `review-finding:pr-2529-f5` | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F6 | `review-finding:pr-2529-f6` | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - Fence the `udp-core` statistics event example as Rust

- PR number: 2529
- Source review ID: 5520183826
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2529#discussion_r4272623646>
- Concern: The event-sending example in `packages/udp-core/src/statistics/mod.rs` is fenced `text`, which loses Rust syntax highlighting; the reviewer suggested `rust,ignore` and claimed it would also reduce doc/API drift.
- Solution: Fenced the example `rust,ignore`, a fence the package already uses in `connection_cookie.rs`. The drift claim does not hold: `ignore` blocks are not compiled, so the change gives highlighting only, as the reply says. Review 5520537884 recommended no change for the same reason; the maintainer-approved change was kept because highlighting is still a readability gain at no cost.
- Current-tree verification: `git grep -n 'rust,ignore' -- packages/udp-core/src/statistics/mod.rs` returns line 18; `RUSTDOCFLAGS="-D warnings" cargo +nightly doc --no-deps -p torrust-tracker-udp-core` (nightly 1.101.0) builds with no warnings, and `cargo test --doc` lists the block as ignored.
- Resolution reference: `docs(udp-core, udp-server): [#2521] highlight the statistics event examples as Rust`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2529#discussion_r4273015878>

### F2 - Fence the `udp-server` statistics event example as Rust

- PR number: 2529
- Source review ID: 5520183826
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2529#discussion_r4272623806>
- Concern: The same `text` fence in `packages/udp-server/src/statistics/mod.rs`; the reviewer suggested `rust,ignore`, or `rust,no_run` for a doctest.
- Solution: Fenced the example `rust,ignore`. `no_run` would still compile the snippet, which fails because it needs an `event_sender` in scope, as review 5520537884 also noted.
- Current-tree verification: `git grep -n 'rust,ignore' -- packages/udp-server/src/statistics/mod.rs` returns line 20; nightly rustdoc with `-D warnings` builds cleanly and `cargo test --doc` lists the block as ignored.
- Resolution reference: `docs(udp-core, udp-server): [#2521] highlight the statistics event examples as Rust`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2529#discussion_r4273016255>

### F3 - The explicit read-guard drop in the adapted test hides its purpose

- PR number: 2529
- Source review ID: 5520183826
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2529#discussion_r4272623914>
- Concern: `drop(stats);` in `it_should_initialize_metrics_when_constructed_with_default` exists only for Clippy's `significant_drop_tightening`, which is easy to miss; the reviewer suggested scoping the guard in a block.
- Solution: The maintainer chose the cleanest option even beyond the adapted test. A `metrics_snapshot()` test helper copies the metrics, so the read guard never outlives the read, and all four tests in the file that used `drop(stats)` now read through it. Review 5520537884 had recommended keeping the explicit drop as the module's idiom; the change removes that idiom from the module entirely, so the tests stay uniform.
- Current-tree verification: `git grep -c 'drop(stats)' -- packages/udp-server/src/statistics/repository.rs` finds nothing and `metrics_snapshot(&repo)` appears four times; `cargo +stable test -p torrust-tracker-udp-server --lib statistics::repository` (stable 1.99.0) passes, and Clippy on all targets reports no diagnostics.
- Resolution reference: `test(udp-server): [#2521] read metrics snapshots instead of dropping read guards`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2529#discussion_r4273016615>

### F4 - `udp-core` loses its only check that a new repository starts with its described metrics

- PR number: 2529
- Source review ID: 5520537884
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2529#discussion_r4272922573>
- Concern: The deleted `udp-core` service test compared a new repository with `describe_metrics()`, which was the package's only check of the repository's initial described state; in `udp-server`, only the deleted equality covered the eleventh described metric, `udp_tracker_server_performance_avg_processed_requests_total`.
- Solution: Added `it_should_contain_the_described_metrics_when_created` to `udp-core`'s `statistics::repository` and the missing eleventh assertion to `udp-server`'s `it_should_be_initialized_with_described_metrics`. Each was seen to fail on a mutation (a repository starting from `Metrics::default()`; the eleventh `describe_counter` removed) and passes once the mutation is reverted.
- Current-tree verification: both tests are present at `packages/udp-core/src/statistics/repository.rs:64` and `packages/udp-server/src/statistics/repository.rs:191`; `cargo +stable test` for both packages passes, also on a throwaway merge with `develop` at `988c3bc76`.
- Resolution reference: `test(udp-core, udp-server): [#2521] restore the described-metrics checks lost with the services tests`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2529#discussion_r4273129291>

### F5 - The evidence overstates which `/api/v1/metrics` values matched

- PR number: 2529
- Source review ID: 5520537884
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2529#discussion_r4272922584>
- Concern: The evidence, AC3, and the PR body say only processing-time values differ, but `/api/v1/metrics` also differs in `recorded_at` and in the order of metrics and series.
- Solution: Recounted from the evidence files (`recorded_at` differs on 15 of 17 samples; the order of the 27 JSON metrics and 12 Prometheus families differs). The evidence now describes the comparison per endpoint and lists the excluded run-dependent differences; AC3 and the PR body say the same.
- Current-tree verification: `git grep -n 'ignores order and' -- docs/issues/open/2521-1669-remove-unused-udp-stats-traits/` matches line 106 of the evidence, and `git grep -n 'which the comparison ignores'` matches the AC3 row of `ISSUE.md`; the PR body was edited with `gh pr edit`.
- Resolution reference: `docs(issues): [#2521] state what the metrics endpoint comparison ignores`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2529#discussion_r4273129616>

### F6 - The spec does not record the rebase onto the #2505 merge

- PR number: 2529
- Source review ID: 5520537884
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2529#discussion_r4272922594>
- Concern: The 14:47 progress-log entry says no rebase is needed, but the branch was rebased onto `8045bd252` when the PR was opened, and the spec records neither that rebase nor the pre-push run on the rebased branch; it also asked for `related-pr: 2529`.
- Solution: A new progress-log entry corrects the 14:47 entry, records the rebase and the checks run on the rebased branch, states that T7 and M1/M2 ran before the rebase, and records the test run on a throwaway merge with `develop` after #2527 merged first. `related-pr: 2529` was set by `docs(issues): [#2521] link PR #2529 and record the review fixes`.
- Current-tree verification: `git grep -n 'Correction to the 14:47 entry' -- docs/issues/open/2521-1669-remove-unused-udp-stats-traits/ISSUE.md` matches, and the frontmatter reads `related-pr: 2529`.
- Resolution reference: `docs(issues): [#2521] record the rebase onto the #2505 merge and the combined test run`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2529#discussion_r4273129946>

## Processing Log

- 2026-10-14 15:12 UTC - Review 5520183826 (`copilot-pull-request-reviewer`, `COMMENTED`) left
  inline comments 4272623646, 4272623806, and 4272623914.
- 2026-10-14 15:17 UTC - Fetched the review threads; only the three Copilot threads existed.
- 2026-10-14 15:38 UTC - Review 5520537884 (`da2ce7`, `APPROVED`) left inline comments
  4272922573, 4272922584, and 4272922594; it was noticed only after the Copilot replies, when the
  review list was fetched again for this audit.
- 2026-10-14 15:41 UTC - Committed the F3 fix after the maintainer chose the snapshot helper for
  all four tests.
- 2026-10-14 15:42 UTC - Committed the F1 and F2 fix.
- 2026-10-14 15:43 UTC - Committed the spec update linking PR #2529, then pushed; GitHub dismissed
  the approval of review 5520537884 as stale at 15:45.
- 2026-10-14 15:46 UTC - Posted the F1, F2, and F3 replies.
- 2026-10-14 15:49 UTC - At the maintainer's request to process F4 to F6 in the same round,
  committed the F4 fix after both mutation checks failed as expected.
- 2026-10-14 15:51 UTC - Committed the F5 fix.
- 2026-10-14 15:53 UTC - Committed the F6 fix after running the combined-code tests on a
  throwaway merge with `develop`; edited the PR body for F5.
- 2026-10-14 15:56 UTC - Pushed F4 to F6 and posted their replies.
- 2026-10-14 15:57 UTC - Created this audit.

## Completion Rules

- Re-derive the reply claim against the current tree before replying or resolving a thread.
- Reply on every resolvable thread before resolving it.
- For an outdated thread whose concern was fixed, record `Disposition=FIXED` and
  `Thread state=RESOLVED`, even if GitHub marks the thread outdated after the push. For a
  duplicate, superseded, or no-change in-PR thread, reply exactly
  `Superseded by <FindingId>: <reason>.`, record `Disposition=NO_ACTION` and
  `Thread state=SUPERSEDED`, then resolve it. A post-merge `NO_ACTION` requires maintainer
  approval to decline the follow-up work.
- A consolidated PR conversation response may cover multiple review rounds only when it names
  every review ID and every finding ID with its disposition and resolution reference. Record its
  durable URL in each related row.
- Cite a fix by its unique Conventional Commit subject or durable reply URL, never by a branch SHA
  that can change after a rebase.
- Refresh review threads using GraphQL and confirm that no unresolved actionable thread remains.
