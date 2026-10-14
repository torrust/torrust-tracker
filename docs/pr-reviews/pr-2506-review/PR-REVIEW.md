---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
    - docs/issues/open/2505-udp-banned-ips-metric/ISSUE.md
    - docs/issues/open/2505-udp-banned-ips-metric/manual-verification-evidence.md
    - docs/issues/open/2505-udp-banned-ips-metric/udp-banned-ips-probe.sh
---

<!-- skill-link: process-pr-review -->

# PR #2506 Review Audit

Source: pull-request reviews and inline review threads for <https://github.com/torrust/torrust-tracker/pull/2506>.

## Ownership

The PR author owns this tracked audit record. Reviewers, including repository review agents,
deliver findings through GitHub and have no repository-artifact obligation.

- Post-merge workflow approval: <https://github.com/torrust/torrust-tracker/pull/2506#issuecomment-6165699972>

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

Copilot review 5506831377 (submitted 2026-10-13 16:33:47 UTC, before merge) left six inline
findings headed `[<Severity>][F<n>]`. The reviewer-provided IDs `F1` to `F6` do not collide with
any earlier audit finding, so they are used unchanged. The review overview repeats the six
findings and adds no other request, so it adds no row. Rows are listed by finding ID; the inline
source order was F1, F5, F2, F4, F3, F6.

Cameron's review 5515068886 (`da2ce7`, COMMENTED) was submitted on 2026-10-14 at 08:24:48 UTC,
after this pull request merged (2026-10-13 17:50:54 UTC). Its eight inline findings are headed
F7-F14; those IDs do not collide with F1-F6, so they are used unchanged. The maintainer approved
fixing them on the branch of PR #2524, which already changes the specification (approval URL in
Ownership). They stay `FOLLOW_UP`/`OPEN` until PR #2524 merges.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | `review-finding:pr-2506-f1` | Copilot | Major | security | ORIGINAL | FIXED | RESOLVED |
| F2 | `review-finding:pr-2506-f2` | Copilot | Major | testing | ORIGINAL | FIXED | RESOLVED |
| F3 | `review-finding:pr-2506-f3` | Copilot | Major | testing | ORIGINAL | FIXED | RESOLVED |
| F4 | `review-finding:pr-2506-f4` | Copilot | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F5 | `review-finding:pr-2506-f5` | Copilot | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F6 | `review-finding:pr-2506-f6` | Copilot | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F7 | `review-finding:pr-2506-f7` | Human | Major | documentation | ORIGINAL | FOLLOW_UP | OPEN |
| F8 | `review-finding:pr-2506-f8` | Human | Minor | documentation | ORIGINAL | FOLLOW_UP | OPEN |
| F9 | `review-finding:pr-2506-f9` | Human | Minor | documentation | ORIGINAL | FOLLOW_UP | OPEN |
| F10 | `review-finding:pr-2506-f10` | Human | Nit | testing | ORIGINAL | FOLLOW_UP | OPEN |
| F11 | `review-finding:pr-2506-f11` | Human | Suggestion | maintainability | ORIGINAL | FOLLOW_UP | OPEN |
| F12 | `review-finding:pr-2506-f12` | Human | Minor | documentation | ORIGINAL | FOLLOW_UP | OPEN |
| F13 | `review-finding:pr-2506-f13` | Human | Minor | testing | ORIGINAL | FOLLOW_UP | OPEN |
| F14 | `review-finding:pr-2506-f14` | Human | Nit | documentation | ORIGINAL | FOLLOW_UP | OPEN |

## Finding Details

### F1 - Require constant per-event work for the banned-IPs total

- PR number: 2506
- Source review ID: 5506831377
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2506#discussion_r4261329982>
- Concern: The spec left P2 (a filtered scan of the per-IP map) selectable although the handler
  reads the total on every invalid connection ID while holding the `BanService` write lock. Scanning
  an attacker-controlled number of tracked IPs makes a flood progressively more expensive and blocks
  requests waiting for the read lock. Copilot asked to reject P2 or add an acceptance criterion
  requiring O(1) work per event.
- Solution: The maintainer had asked to keep the performance options open until implementation, so
  the conflict was taken to him; he chose an outcome-based criterion. AC7 now requires that the
  work done on each invalid-connection-ID event to maintain and read the banned total does not grow
  with the number of tracked IPs. P2 is labeled "ruled out by AC7"; P1, P3, and P4 remain open. The
  benchmark note that mentioned P2 now covers only P4.
- Current-tree verification: `ISSUE.md` contains AC7 in Acceptance Criteria and in Acceptance
  Verification, the Performance Alternatives introduction states the constraint and cites
  `review-finding:pr-2506-f1`, and the P2 heading reads "P2 (ruled out by AC7)".
- Resolution reference: `docs(issues): [#2505] require constant per-event cost for the banned-IPs total`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2506#discussion_r4261676132>

### F2 - Complete the request-path benchmark plan

- PR number: 2506
- Source review ID: 5506831377
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2506#discussion_r4261330078>
- Concern: `docs/benchmarking.md` and the `run-benchmarks` workflow require, for request-path
  changes, the focused Criterion benchmark plus `aquatic_udp_load_test`, an explicit one-sided pass
  rule, and issue-local `performance-evidence.md`. The plan only ran Criterion and said "no
  regression" without a measurable threshold.
- Solution: Added a "Performance Evidence Plan" section modeled on issue #2458: the existing
  `udp_ban_service` Criterion groups plus a new `udp_ban_service/record_error` benchmark for the
  per-event step (no benchmark covered it; task B1 adds it in its own commit before the baseline),
  five `aquatic_udp_load_test` runs with the issue #2342 settings, P1 before any production change
  (T3) and P2 after (T9), and a one-sided pass rule, all recorded in `performance-evidence.md`.
  Updated AC5, the implementation plan, the commit points, the layer table, and the automatic
  checks.
- Current-tree verification: `ISSUE.md` has the "Performance Evidence Plan" section, rows B1, T3,
  and T9 in the plan and commit points, AC5 referencing the one-sided pass rule, and the
  `aquatic_udp_load_test` command in Automatic Checks; `linter lychee` passes, including the
  `docs/benchmarking.md#e2e-udp-load-testing` fragment.
- Resolution reference: `docs(issues): [#2505] plan the request-path performance evidence`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2506#discussion_r4261676793>

### F3 - Keep the semantic change and its dependent test updates in one green commit

- PR number: 2506
- Source review ID: 5506831377
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2506#discussion_r4261330167>
- Concern: T4-T5 changes `get_banned_ips_total()`, but the `udp-server` handler and listener unit
  tests that pin the old distinct-IP count were scheduled for the later T8 commit, so the T4-T5
  commit would fail its own test suite.
- Solution: Moved those test updates into T5 and the T4-T5 commit point; T8 is documentation only.
  The commit-point guidance now requires every commit to leave the workspace test suite green.
- Current-tree verification: `packages/udp-server/src/banning/event/handler.rs` test
  `it_should_publish_the_distinct_client_ip_total_after_a_connection_cookie_error` expects 2 with
  `BanService::new(1)` and one error per IP, and `listener.rs` builds `BanService::new(1)` and
  asserts a total of 1 after one error, so both would change under the fix. `ISSUE.md` T5 and the
  T4-T5 commit point list the handler and listener test updates, and T8 reads "Documentation
  updates".
- Resolution reference: `docs(issues): [#2505] keep the banned-count fix and its dependent test updates in one commit`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2506#discussion_r4261677339>

### F4 - Resolve the required-versus-optional planned-test scope

- PR number: 2506
- Source review ID: 5506831377
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2506#discussion_r4261330124>
- Concern: R1p and R6 were labeled "recommended" (and the PR description called the property test
  optional), while T4, T5, and the commit point required them.
- Solution: The maintainer made both tests required. Removed the "recommended" labels, marked the
  property-based layer Required, and aligned T4. The PR description was updated to stop calling the
  property test optional.
- Current-tree verification: `grep -n -i "recommended\|optional" ISSUE.md` matches only a
  historical progress-log entry; T4 reads "R1, R5, and R6 regression tests, red"; `gh pr view 2506`
  shows the description listing "`udp-core` unit tests and a property test".
- Resolution reference: `docs(issues): [#2505] make the property and disabled-mode tests required`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2506#discussion_r4261677082>

### F5 - Validate probe arguments before dereferencing them

- PR number: 2506
- Source review ID: 5506831377
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2506#discussion_r4261330039>
- Concern: With `set -u`, running the probe without a command, or `invalid-announce`/`connect`
  without a source IP, aborted with an "unbound variable" error instead of printing the usage and
  exiting 2.
- Solution: Reproduced both failures (exit 1 with `$1: unbound variable` and `$2: unbound
  variable`), then added a `usage` helper and a `require_source_ip` check. The evidence file notes
  that this validation was added after the reproduction run and does not change any valid
  invocation.
- Current-tree verification: no arguments, `connect`, `invalid-announce`, and `bogus` each print
  the usage and exit 2; `connect 127.0.0.1` with no tracker running prints `<no response>` and
  exits 0; `shellcheck` passes.
- Resolution reference: `docs(issues): [#2505] validate the reproduction probe arguments`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2506#discussion_r4261676441>

### F6 - Record the exact reproduction commands

- PR number: 2506
- Source review ID: 5506831377
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2506#discussion_r4261330221>
- Concern: The evidence showed reconstructed shorthand: the V3 loop could not produce the numbered,
  truncated output shown, and V2 and V4 omitted their `cut` and `head` filters, which the
  bug-evidence workflow does not allow.
- Solution: Replaced every step with the command exactly as it was passed to the shell (including
  the configuration heredoc, the tracker start command, and every output filter), followed by the
  verbatim output, taken from the session's tool results. Where the run printed a simplified label
  (the V3 `echo` of the loop), the text says so. Removed the after-the-fact truncation notes.
- Current-tree verification: `manual-verification-evidence.md` V1 to V4 each show a `bash` command
  block and a verbatim `text` output block; the V3 command contains
  `printf "%2d: " $i; ./probe.sh invalid-announce 127.0.0.4 | grep "error message" | cut -c1-90`,
  and V4 step 3 contains `./probe.sh stats | head -3`.
- Resolution reference: `docs(issues): [#2505] record the exact reproduction commands`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2506#discussion_r4261677597>

### F7 - The inherent method had no doc comment

- PR number: 2506
- Source review ID: 5515068886
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2506#discussion_r4268430817>
- Concern: the "Contracts violated" bullet credited "Returns the total number of banned IPs." to
  the inherent `BanService::get_banned_ips_total`, which had no doc comment; the sentence was only on
  the trait, and removing the timestamp field also makes the `reset_bans` doc false.
- Solution: on the PR #2524 branch, the bullet, Scope, AC3, and its evidence row now say the fix adds
  a doc comment to the inherent method and updates the `reset_bans` doc, which that branch's code
  does.
- Current-tree verification: on the PR #2524 branch, `ISSUE.md`'s bullet says the inherent method
  had no doc comment; `packages/udp-core/src/services/banning.rs` documents `get_banned_ips_total`
  and `reset_bans`.
- Resolution reference: <https://github.com/torrust/torrust-tracker/pull/2506#discussion_r4269647671>
- Follow-up PR URL: <https://github.com/torrust/torrust-tracker/pull/2524>
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2506#discussion_r4269647671>

### F8 - The evidence said invalid connection IDs are not logged

- PR number: 2506
- Source review ID: 5515068886
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2506#discussion_r4268430827>
- Concern: the reproduction evidence said invalid connection IDs are not logged at the `info`
  level, but `evidence/tracker.log` has one `WARN` line per invalid ID.
- Solution: on the PR #2524 branch, the sentence is replaced by the 15 condensed `WARN` lines per
  process, with the script that condensed them.
- Current-tree verification: on the PR #2524 branch, `manual-verification-evidence.md` quotes log
  lines 122, 132, 133, 143-147, 149-154, and 497.
- Resolution reference: <https://github.com/torrust/torrust-tracker/pull/2506#discussion_r4269647993>
- Follow-up PR URL: <https://github.com/torrust/torrust-tracker/pull/2524>
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2506#discussion_r4269647993>

### F9 - The method dates from 1ce2e3327, not the package extraction

- PR number: 2506
- Source review ID: 5515068886
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2506#discussion_r4268430830>
- Concern: `get_banned_ips_total` was added with the `len()` body by `1ce2e3327` (#1145); the
  extraction the specification cited only moved the file, so the value was wrong since the metric
  was introduced.
- Solution: on the PR #2524 branch, "Origin" names `1ce2e3327`, and "Risks and Trade-offs" says the
  value has been wrong since the metric was introduced.
- Current-tree verification: `git show 1ce2e3327` adds `get_banned_ips_total` returning
  `self.accurate_error_counter.len()`; the PR #2524 branch's `ISSUE.md` cites it.
- Resolution reference: <https://github.com/torrust/torrust-tracker/pull/2506#discussion_r4269648212>
- Follow-up PR URL: <https://github.com/torrust/torrust-tracker/pull/2524>
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2506#discussion_r4269648212>

### F10 - R2's "passes on develop" holds only for its reset leg

- PR number: 2506
- Source review ID: 5515068886
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2506#discussion_r4268430837>
- Concern: R2's re-count leg is red on `develop`, so the note that R2 passes there was too broad.
- Solution: on the PR #2524 branch, R2's note says only the reset leg passes on `develop`.
- Current-tree verification: the PR #2524 branch's `ISSUE.md` R2 bullet says "The reset leg passes
  on `develop`" and that the re-count leg is red.
- Resolution reference: <https://github.com/torrust/torrust-tracker/pull/2506#discussion_r4269648429>
- Follow-up PR URL: <https://github.com/torrust/torrust-tracker/pull/2524>
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2506#discussion_r4269648429>

### F11 - Give the leftover removals their own commit point

- PR number: 2506
- Source review ID: 5515068886
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2506#discussion_r4268430847>
- Concern: the removal of the unused trait and the write-only field changes no tested behavior and
  should not be grouped with the banned-count fix.
- Solution: the maintainer chose to act on it and accepted rewriting the unmerged PR #2524 history:
  an interactive rebase split the removals into
  `refactor(udp-core): [#2505] remove the unused BanningStats trait and the write-only reset timestamp`,
  placed before `fix(udp-core): [#2505] count only IPs over the limit as banned`. The Commit Points
  table and the AC4 evidence name the refactor commit.
- Current-tree verification: on the PR #2524 branch the refactor commit exists once, its state
  passed the workspace check, the `udp-core` and `udp-server` tests, fmt, clippy, and the pre-commit
  hook, and the final tree is byte-identical to the pre-split head.
- Resolution reference: <https://github.com/torrust/torrust-tracker/pull/2506#discussion_r4269648684>
- Follow-up PR URL: <https://github.com/torrust/torrust-tracker/pull/2524>
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2506#discussion_r4269648684>

### F12 - P1 and P2 named options and measurement phases

- PR number: 2506
- Source review ID: 5515068886
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2506#discussion_r4268430850>
- Concern: P1 and P2 named both the performance alternatives and the measurement phases, so AC5
  said "P2 meets the pass rule" while option P2 was ruled out.
- Solution: on the PR #2524 branch, the phases are "baseline" and "after" in the current text, and
  an "Identifiers" glossary lists every short identifier (the same change as PR #2524 finding F4).
- Current-tree verification: the PR #2524 branch's `ISSUE.md` has the "Identifiers" section, and
  `performance-evidence.md` uses P1/P2 only for options.
- Resolution reference: <https://github.com/torrust/torrust-tracker/pull/2506#discussion_r4269648920>
- Follow-up PR URL: <https://github.com/torrust/torrust-tracker/pull/2524>
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2506#discussion_r4269648920>

### F13 - The plan dropped the P4 scan benchmark condition

- PR number: 2506
- Source review ID: 5515068886
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2506#discussion_r4268430858>
- Concern: option P4 stayed open, but the rewritten performance plan no longer required measuring
  its scan, and F1's solution above said the benchmark note "now covers only P4".
- Solution: on the PR #2524 branch, "Performance Alternatives" again requires a scan benchmark if
  option P4 is chosen, and says it did not apply because option P1 was chosen. F1's solution
  sentence above is accurate again.
- Current-tree verification: the PR #2524 branch's `ISSUE.md` contains "If option P4 is chosen,
  also add a benchmark case for its scan".
- Resolution reference: <https://github.com/torrust/torrust-tracker/pull/2506#discussion_r4269649229>
- Follow-up PR URL: <https://github.com/torrust/torrust-tracker/pull/2524>
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2506#discussion_r4269649229>

### F14 - Scope described the old benchmark plan

- PR number: 2506
- Source review ID: 5515068886
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2506#discussion_r4268430867>
- Concern: Scope and "Risks and Trade-offs" still pointed at the original benchmark plan.
- Solution: on the PR #2524 branch, both point at the Performance Evidence Plan.
- Current-tree verification: the PR #2524 branch's `ISSUE.md` Scope bullet says "Measure performance
  before and after the change as the "Performance Evidence Plan" describes".
- Resolution reference: <https://github.com/torrust/torrust-tracker/pull/2506#discussion_r4269649553>
- Follow-up PR URL: <https://github.com/torrust/torrust-tracker/pull/2524>
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2506#discussion_r4269649553>

## Processing Log

- 2026-10-13 16:36 UTC - Started audit: fetched review 5506831377 and its six unresolved threads
  with the `github-review-threads` tool; asked the maintainer to decide F1 and F4.
- 2026-10-13 17:07 UTC - Pushed one signed fix commit per finding (F1 to F6); the pre-push checks
  passed. Replied on all six threads with the disposition, verification, and resolution reference.
- 2026-10-13 17:09 UTC - Updated the PR description for F4 and recorded this audit; the threads are
  resolved after it is committed and pushed.
- 2026-10-14 09:55 UTC - Late review: Cameron's review 5515068886 (findings F7-F14) arrived after
  the merge. After read-only triage, the maintainer approved fixing the findings on the PR #2524
  branch; the approval was posted as a comment on this pull request.
- 2026-10-14 10:31 UTC - Fixed F7-F14 on the PR #2524 branch (pushed at 10:28 UTC) and replied on
  each thread with the follow-up reference; the threads stay open until PR #2524 merges.
- 2026-10-14 10:36 UTC - Recorded F7-F14 in this audit as `FOLLOW_UP`/`OPEN`, on the PR #2524 branch.

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
