---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
    - docs/issues/open/2466-1978-announce-interval-upper-bound/ISSUE.md
---

<!-- skill-link: process-pr-review -->

# PR #2680 Review Audit

Source: pull-request reviews and inline review threads for <https://github.com/torrust/torrust-tracker/pull/2680>.

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

One review was submitted so far: the review of `copilot-pull-request-reviewer` (5681855365, submitted
2026-10-28 11:26 UTC) on the head whose subject is
`docs(issues): [#2466] cite the verified commit by subject in the evidence file`. Its body warns that
Copilot could not run its full agentic review before the timeout; the three inline threads it carries
are the only findings, and the body adds no other actionable row. The findings carry no `[Severity]`
bracket, so each severity is inferred from the "Medium" or "Low" label in the review overview: Medium
is recorded as `Minor` and Low as `Nit`.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2680-f1 | Copilot | Minor (inferred) | maintainability | ORIGINAL | NO_ACTION | SUPERSEDED |
| F2 | review-finding:pr-2680-f2 | Copilot | Minor (inferred) | testing | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-2680-f3 | Copilot | Nit (inferred) | documentation | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - Keep `announce_interval_on_the_wire` a plain `fn` instead of a `const fn`

- PR number: 2680
- Source review ID: 5681855365
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2680#discussion_r4399979381>
- Concern: Making `announce_interval_on_the_wire` a `const fn` adds compile-time constraints that must
  survive MSRV and dependency updates, with no benefit because the value is computed at runtime.
- Solution: No change. The function is `const` because the repository's Clippy gate requires it: the
  workspace denies warnings, which turns `clippy::missing_const_for_fn` into an error for a function
  whose body only calls `const` items (`AnnounceIntervalInSecs::get_i32` and `I32::new`). The first
  commit attempt of the change that added the function failed the pre-commit hook on exactly this
  lint, and the function became `const` to fix it. If a later dependency or MSRV change makes the
  body non-`const`, the compiler fails the build at that definition, so the maintenance risk is
  visible immediately rather than silent.
- Current-tree verification: changed `const fn announce_interval_on_the_wire` to `fn` in
  `packages/udp-server/src/handlers/announce.rs` and ran
  `cargo clippy -p torrust-tracker-udp-server --all-targets -- -D warnings`: it failed with
  `error: this could be a const fn` (`-D clippy::missing-const-for-fn`). Restored the file and
  confirmed `git status --short` was empty and line 187 reads `const fn announce_interval_on_the_wire`.
- Resolution reference: <https://github.com/torrust/torrust-tracker/pull/2680#discussion_r4400112289>
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2680#discussion_r4400112289>

### F2 - Simplify the boundary-interval test setup

- PR number: 2680
- Source review ID: 5681855365
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2680#discussion_r4399979440>
- Concern: `it_should_convey_the_largest_interval_and_clamp_out_of_range_peer_counts_for_both_address_families`
  builds the boundary interval verbosely and may keep an unused `first_out_of_range`; use
  `AnnounceIntervalInSecs::MAX` and drop the variable if it is no longer used.
- Solution: Fixed the verbose construction: the interval is now
  `AnnounceIntervalInSecs::new(AnnounceIntervalInSecs::MAX).unwrap()`. Kept `first_out_of_range`,
  because the premise that it is unused is wrong: the test still uses it as `stats.incomplete`, the
  clamped peer count it asserts as `i32::MAX`.
- Current-tree verification: inspected the test: `first_out_of_range` is read once, as
  `incomplete: first_out_of_range`. Ran
  `cargo test -p torrust-tracker-udp-server it_should_convey_the_largest_interval`
  (1 passed) and `cargo clippy -p torrust-tracker-udp-server --all-targets -- -D warnings` (clean).
- Resolution reference: `test(udp-server): [#2466] build the boundary interval from AnnounceIntervalInSecs::MAX`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2680#discussion_r4400112536>

### F3 - Reword the "wrapping negative" doc comment on `saturating_wire_i32`

- PR number: 2680
- Source review ID: 5681855365
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2680#discussion_r4399979473>
- Concern: "clamp instead of wrapping negative" is misleading for a `u32` input; an out-of-range value
  overflows the signed 32-bit field and appears negative on the wire.
- Solution: Reworded the doc comment to say that a `u32` above `i32::MAX` would overflow into a
  negative number on the wire, so the helper clamps to `i32::MAX`.
- Current-tree verification: read the doc comment above `saturating_wire_i32` in
  `packages/udp-server/src/handlers/mod.rs` and confirmed the new wording; ran
  `cargo +nightly fmt --all -- --check` (clean) and
  `cargo clippy -p torrust-tracker-udp-server --all-targets -- -D warnings` (clean).
- Resolution reference: `docs(udp-server): [#2466] say that out-of-range counters overflow into negative wire values`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2680#discussion_r4400112765>

## Processing Log

- 2026-10-28 11:26 UTC - Copilot submitted review 5681855365 with three inline threads (F1-F3).
- 2026-10-28 11:38 UTC - Committed the F2 fix: `test(udp-server): [#2466] build the boundary interval from AnnounceIntervalInSecs::MAX`.
- 2026-10-28 11:39 UTC - Committed the F3 fix: `docs(udp-server): [#2466] say that out-of-range counters overflow into negative wire values`.
- 2026-10-28 11:41 UTC - Replied on the F1, F2, and F3 threads. The F1 reply uses the prescribed `Superseded by F1: <reason>.` form because it declines a no-change concern; it points to this record for the reasoning.
- 2026-10-28 11:42 UTC - Resolved the F1, F2, and F3 threads; a refreshed GraphQL fetch showed zero unresolved threads.

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
