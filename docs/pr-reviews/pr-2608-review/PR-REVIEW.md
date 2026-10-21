---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
    - docs/issues/open/1349-1347-add-tests-http-core/ISSUE.md
    - docs/issues/open/1349-1347-add-tests-http-core/test-refactor-plans/statistics-mod-tests.md
---

<!-- skill-link: process-pr-review -->

# PR #2608 Review Audit

Source: pull-request reviews and inline review threads for <https://github.com/torrust/torrust-tracker/pull/2608>.

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

Copilot's review 5599023298 (COMMENTED, 2026-10-21 11:34:27 UTC) read the head whose subject is
"test(http-core): [#1349] address the S5 independent review", one commit before the pull-request record
commit that was the branch head when the review arrived. It carries three inline threads, F1 to F3, and no
independently actionable assertion in its body. No other review had been submitted when this audit started.
All three assert a defect that the current tree does not have, so none changes the code.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2608-f1 | Copilot | Minor (inferred) | testing | ORIGINAL | NO_ACTION | SUPERSEDED |
| F2 | review-finding:pr-2608-f2 | Copilot | Minor (inferred) | testing | ORIGINAL | NO_ACTION | SUPERSEDED |
| F3 | review-finding:pr-2608-f3 | Copilot | Nit (inferred) | maintainability | ORIGINAL | NO_ACTION | SUPERSEDED |

## Finding Details

### F1 - The expected Prometheus text may embed the indentation of its continuation lines

- PR number: 2608
- Source review ID: 5599023298
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2608#discussion_r4336282199>
- Concern: the exposition string of `it_should_describe_the_request_counter_with_its_description_and_type`
  is written over three lines with a trailing backslash, and the reviewer expects the next lines' indentation
  to be part of the string, so the assertion would fail unless `to_prometheus()` emits those spaces.
- Solution: no change. A backslash before a newline in a Rust string literal skips the newline and all
  whitespace at the start of the next line, so the indentation is not in the string.
- Current-tree verification: `cargo test -p torrust-tracker-http-core --lib statistics::tests` passes with 4
  tests, including this one, against the real `to_prometheus()` output; a leading space in the expected text
  would make it fail.
- Resolution reference: <https://github.com/torrust/torrust-tracker/pull/2608#discussion_r4336301654>
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2608#discussion_r4336301654>

### F2 - The clock set by a test is not restored

- PR number: 2608
- Source review ID: 5599023298
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2608#discussion_r4336282320>
- Concern: `it_should_record_an_event_at_the_time_of_the_clock` calls `CurrentClock::local_set(...)` and does not
  restore the previous time, which could leak into other tests if the clock state is shared.
- Solution: no change. The `Stopped` clock keeps its time in a `thread_local!` (`FIXED_TIME`), and the test
  harness runs each test on its own thread, so a set time cannot reach another test. The test also sets the
  clock before it reads it, so it does not depend on the previous value, and the S2 announce tests use the same
  pattern.
- Current-tree verification: the thread-local declaration was read in the `torrust-clock` source
  (`src/clock/stopped/mod.rs`), and the package suite passes repeated runs and runs with `--test-threads=1`
  (the independent review also ran it 30 times and pinned to one CPU).
- Resolution reference: <https://github.com/torrust/torrust-tracker/pull/2608#discussion_r4336301916>
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2608#discussion_r4336301916>

### F3 - The `labels` helper allocates a `Vec` to build a `LabelSet`

- PR number: 2608
- Source review ID: 5599023298
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2608#discussion_r4336282363>
- Concern: `labels` calls `pairs.to_vec()` on every use, which is noisier and less efficient than a
  non-allocating conversion.
- Solution: no change. `LabelSet` converts from fixed-size arrays and from `Vec<(&str, &str)>`, not from slices,
  and the expected label sets of the table have different lengths (8 and 9 pairs), so they cannot share one
  array type; `to_vec()` is the available conversion. The allocation is in test setup only, and `event.rs`
  uses the same helper.
- Current-tree verification: the `From` implementations were read in `torrust-metrics` (`src/label/set.rs`);
  none takes a slice.
- Resolution reference: <https://github.com/torrust/torrust-tracker/pull/2608#discussion_r4336302150>
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2608#discussion_r4336302150>

## Processing Log

- 2026-10-21 11:34 UTC - Copilot submitted review 5599023298 with three inline threads. Created this audit from
  GraphQL review-thread data: 3 threads, unresolved.
- 2026-10-21 11:36 UTC - Verified each claim against the current tree, replied on each thread with the
  prescribed `Superseded by` reply, then resolved all three. GraphQL data shows 0 unresolved threads.

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
