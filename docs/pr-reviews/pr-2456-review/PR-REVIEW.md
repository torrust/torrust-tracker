---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
    - docs/issues/open/2454-1669-mark-public-error-enums-non-exhaustive/ISSUE.md
---

<!-- skill-link: process-pr-review -->

# PR #2456 Review Audit

Source: pull-request reviews and inline review threads for <https://github.com/torrust/torrust-tracker/pull/2456>.

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
- Category: `link-integrity`, `formatting`, `metadata`, `testing`, `correctness`, `documentation`,
  `maintainability`, `security`, `other`
- An outdated thread whose concern was fixed is `FIXED`/`RESOLVED`, even when GitHub marks the
  original thread outdated after the push. For in-PR feedback, use `NO_ACTION`/`SUPERSEDED` only
  for a duplicate, superseded, or no-change concern. A post-merge `NO_ACTION` requires maintainer
  approval to decline the follow-up work.

## Findings

Copilot review 5431591489 and da2ce7 review 5431651745 both numbered their findings from `F1`.
Copilot's review came first, so its `F1` to `F3` keep their IDs, and da2ce7's `F1` to `F6` become
`F4` to `F9`; each detail entry records the reviewer's original ID. Copilot's inline comments carry
`[Major]` brackets, which the rows follow, although its overview badges read `Low`. da2ce7's `F1`
and `F3` ask for the same changes as Copilot's `F1` and `F3`, so `F4` and `F6` are re-raises
answered by the original fixes. Neither review body adds an assertion beyond its inline threads.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | `review-finding:pr-2456-f1` | Copilot | Major | testing | ORIGINAL | FIXED | RESOLVED |
| F2 | `review-finding:pr-2456-f2` | Copilot | Major | maintainability | ORIGINAL | FIXED | RESOLVED |
| F3 | `review-finding:pr-2456-f3` | Copilot | Major | correctness | ORIGINAL | FIXED | RESOLVED |
| F4 | `review-finding:pr-2456-f4` | Human | Major | testing | RE_RAISE_OF:F1 | NO_ACTION | SUPERSEDED |
| F5 | `review-finding:pr-2456-f5` | Human | Minor | testing | ORIGINAL | FIXED | RESOLVED |
| F6 | `review-finding:pr-2456-f6` | Human | Minor | correctness | RE_RAISE_OF:F3 | NO_ACTION | SUPERSEDED |
| F7 | `review-finding:pr-2456-f7` | Human | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F8 | `review-finding:pr-2456-f8` | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F9 | `review-finding:pr-2456-f9` | Human | Suggestion | testing | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - The spec allowed waiving the manual verification scenario

- PR number: 2456
- Source review ID: 5431591489
- Reviewer finding ID: F1
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2456#discussion_r4197901111>
- Concern: The spec let M1 be waived when automated checks passed, while the create-issue skill
  makes manual verification mandatory even when automated tests pass.
- Solution: Made M1 unconditional, dropped "(only if not waived)", restored the template's
  acceptance-criteria wording, and added M1's positive half (the same `match` with a `_` arm
  compiles).
- Current-tree verification: Manual Verification Scenarios and the Acceptance Criteria in the
  #2454 `ISSUE.md` contain no waiver; `linter all` passed in pre-commit.
- Resolution reference: `docs(issues): [#2454] make the manual verification scenario mandatory`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2456#discussion_r4203807537>

### F2 - No task owned the SemVer impact on already-published crates

- PR number: 2456
- Source review ID: 5431591489
- Reviewer finding ID: F2
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2456#discussion_r4197901053>
- Concern: Adding `#[non_exhaustive]` breaks downstream exhaustive matches, and the spec named no
  task that handles the version bump of crates already on crates.io.
- Solution: Per the maintainer's decision, the per-package publish flow owns version bumps. T1's
  inventory records, for each already-published crate (`torrust-tracker-configuration`,
  `torrust-tracker-primitives`, `torrust-tracker-test-helpers`, all 3.0.0), that its next publish
  must be a semver-major bump (minor for `0.x`). AC6 requires that record, EPIC #1669's pre-publish
  checklist gains item 4 stating the rule, and Out of Scope and Risks say so.
- Current-tree verification: T1, AC6, Out of Scope, and Risks in the #2454 `ISSUE.md`, and item 4
  of the EPIC #1669 Pre-publish API checklist, state the rule.
- Resolution reference: `docs(issues): [#2454] assign semver bump handling to the publish flow`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2456#discussion_r4203807718>

### F3 - The placeholder variant was conflated with checklist item 3

- PR number: 2456
- Source review ID: 5431591489
- Reviewer finding ID: F3
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2456#discussion_r4197900975>
- Concern: `configuration::Error::Infallible` is a never-constructed variant of a populated enum,
  not a `Result<_, Infallible>` or an empty enum, so EPIC checklist item 3 does not cover it; and
  Out of Scope forbade removing variants that T4 must resolve.
- Solution: Background and In Scope cite the `handle-errors-in-code` rule against placeholder
  variants, keeping item 3 for `Result<_, Infallible>` and empty enums; Out of Scope exempts the
  maintainer-approved removal of a never-constructed placeholder variant.
- Current-tree verification: Background, In Scope, and Out of Scope in the #2454 `ISSUE.md`;
  `rg -n 'Infallible' packages console src` matches only the declaration.
- Resolution reference: `docs(issues): [#2454] scope the Infallible placeholder variant correctly`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2456#discussion_r4203807889>

### F4 - The manual verification waiver has no basis in the skill

- PR number: 2456
- Source review ID: 5431651745
- Reviewer finding ID: F1
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2456#discussion_r4197951757>
- Concern: The same as F1: make M1 unconditional and restore the template's acceptance-criteria
  line.
- Solution: No separate change; F1's fix covers every point.
- Current-tree verification: same as F1.
- Resolution reference: <https://github.com/torrust/torrust-tracker/pull/2456#discussion_r4203808095>
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2456#discussion_r4203808095>

### F5 - The verification plan omitted toolchain-qualified evidence

- PR number: 2456
- Source review ID: 5431651745
- Reviewer finding ID: F2
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2456#discussion_r4197951766>
- Concern: The skill requires recording the Rust toolchain for each recorded `cargo` result, and
  the plan did not.
- Solution: Automatic Checks now require recording `rustc --version` for each `cargo` command
  whose result is recorded, naming M1's version-dependent `E0004` text and the mixed
  nightly/stable pre-push checks.
- Current-tree verification: the Verification Plan in the #2454 `ISSUE.md` states the rule.
- Resolution reference: `docs(issues): [#2454] require toolchain-qualified verification evidence`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2456#discussion_r4203808232>

### F6 - Out of Scope forbade removing the placeholder variant

- PR number: 2456
- Source review ID: 5431651745
- Reviewer finding ID: F3
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2456#discussion_r4197951773>
- Concern: The same as F3: Out of Scope's "no removing variants" conflicted with T4 resolving
  the `Infallible` placeholder variant.
- Solution: No separate change; F3's fix covers it.
- Current-tree verification: same as F3.
- Resolution reference: <https://github.com/torrust/torrust-tracker/pull/2456#discussion_r4203808401>
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2456#discussion_r4203808401>

### F7 - T3 pointed at EPIC structures that do not exist

- PR number: 2456
- Source review ID: 5431651745
- Reviewer finding ID: F4
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2456#discussion_r4197951778>
- Concern: T3's "EPIC #1669 publication order" and per-crate checklist ticks referred to EPIC
  content that does not exist.
- Solution: T3 orders crates by dependency (a crate before its dependents); per-crate completion
  is a status column in the T1 inventory, linked from the EPIC by T5. AC5, its Acceptance
  Verification row, In Scope, and the commit point match, and the GitHub issue body's plan step 3
  was updated.
- Current-tree verification: T1, T3, T5, and AC5 in the #2454 `ISSUE.md`; the live issue body no
  longer mentions a publication order.
- Resolution reference: `docs(issues): [#2454] define crate order and the per-crate completion record`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2456#discussion_r4203808576>

### F8 - The new tracked item had no EPIC Details row

- PR number: 2456
- Source review ID: 5431651745
- Reviewer finding ID: F5
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2456#discussion_r4197951790>
- Concern: EPIC #1669 tracked #2454 without a matching row in its Details table.
- Solution: Added an "Error enum audit" row with the #2454 link, the spec link, status `TODO`,
  and a note.
- Current-tree verification: the row follows "UDP type consolidation" in the EPIC Details table;
  `linter markdown` and `linter lychee` pass.
- Resolution reference: `docs(issues): [#2454] add the EPIC #1669 Details row for #2454`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2456#discussion_r4203808770>

### F9 - A doctest guard needs the test-development loop

- PR number: 2456
- Source review ID: 5431651745
- Reviewer finding ID: F6
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2456#discussion_r4197951797>
- Concern: If T2 picks a `compile_fail` doctest, the plan adds a test and should follow the
  skill's test-development loop.
- Solution: T2 now requires that a chosen doctest's first increment follows the
  `write-unit-test` progressive loop, with its design review recorded before T3 starts.
- Current-tree verification: the T2 row in the #2454 `ISSUE.md` states the requirement.
- Resolution reference: `docs(issues): [#2454] require the test loop if T2 picks a doctest guard`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2456#discussion_r4203808995>

## Processing Log

- 2026-10-06 16:55 UTC - Started audit. Fetched nine unresolved threads from Copilot review
  5431591489 (16:30 UTC) and da2ce7 review 5431651745 (16:36 UTC, changes requested) with
  `github-review-threads`.
- 2026-10-06 20:31 UTC - Committed the fixes for F1 to F3, F5, and F7 to F9, plus a spec progress
  log entry; updated the #2454 issue body to match.
- 2026-10-07 06:04 UTC - Rebased onto `develop` and pushed the fixes; pre-push passed.
- 2026-10-07 06:33 UTC - Replied to all nine threads; recorded reply URLs.
- 2026-10-07 06:52 UTC - Wrote this audit record.

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
