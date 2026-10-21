---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
    - docs/issues/open/2482-1669-reorganize-shared-test-support/EPIC.md
    - docs/issues/open/2482-1669-reorganize-shared-test-support/test-support-inventory.md
---

<!-- skill-link: process-pr-review -->

# PR #2614 Review Audit

Source: pull-request reviews and inline review threads for <https://github.com/torrust/torrust-tracker/pull/2614>.

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

Round 1: Copilot's review 5601788677 (COMMENTED, 2026-10-21 15:03:54 UTC) read the head whose subject is
"docs(issues): [#2482] record the maintainer's approval of the corrected spec". It carries two inline
threads, F1 and F2, and no independently actionable assertion in its body. Copilot labelled F1 "Medium
severity" and F2 "Low severity"; the severities below are inferred from those labels. Both findings were
correct, and F1 also corrected a statement in this PR's own spec that was copied from the orchestrator's
review without being checked.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2614-f1 | Copilot | Major (inferred) | correctness | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-2614-f2 | Copilot | Minor (inferred) | correctness | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - #2416 also changes the HTTP and UDP test environments and needs the same coordination rule

- PR number: 2614
- Source review ID: 5601788677
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2614#discussion_r4338428888>
- Concern: the spec said #1488's other open subissue, #2410, "changes no environment". Its child #2416
  lists task T9, which changes the HTTP and UDP environments, and T9 is still `TODO`. Rows 4 and 5 were
  therefore exempt from coordination with those shutdown changes. The reviewer asks for the same
  merge-first-or-justify-independence rule to apply to #2416.
- Solution: the "Coordination with EPIC #1488" rule names #2416 with the same rule as #2449 and #2450 and drops the
  "changes no environment" sentence. Rows 4 and 5 say "After #2416 (its T9 changes this environment)". The Risks
  bullet lists four subissues, and the long-pole risk says rows 4 to 7 wait on #1488 (#2416, #2449, #2450). The
  Progress Log has an appended correction entry, because the 13:59 entry listed #2410 among the refreshed facts.
- Current-tree verification: read `docs/issues/open/2416-2410-si-22-4-stop-producers-before-listeners/ISSUE.md`
  (T9 row: "HTTP and UDP environments stop servers first", status `TODO`) and `gh issue view 2416` (state
  `OPEN`); after the fix, `grep -n 2416` in `EPIC.md` shows the rule, rows 4 and 5, both risks and the
  correction entry.
- Resolution reference: docs(issues): [#2482] coordinate the environment moves with #2416
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2614#discussion_r4338655050>

### F2 - Removing the unreleased `test-helpers` modules does not need a major bump by itself

- PR number: 2614
- Source review ID: 5601788677
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2614#discussion_r4338428992>
- Concern: the spec called removing `test-helpers`' `http` and `udp` modules a semver-major change to a crate
  "already published as 3.0.0". At tag `v3.0.0` the crate exports only `configuration` and `random`, so those
  modules were never released. The reviewer asks to make the bump conditional on #2583's comparison with the
  published API, and to correct the same claim in subissue row 2, the #2583 related-work row, the Semver risk
  and the inventory.
- Solution: the O4 weakness, row 2, the #2583 related-work row, the Semver risk and the inventory say that the
  published 3.0.0 exports only `configuration` and `random`, that `logging`, `http` and `udp` are unreleased,
  and that #2583 decides every bump by comparing the final API with the published one. The Progress Log has an
  appended correction entry. #2583's own spec ("probably" a major bump) is outside this PR; its T4 already decides
  against the published API.
- Current-tree verification: `git show v3.0.0:packages/test-helpers/src/lib.rs` shows `pub mod configuration;` and
  `pub mod random;` only, and so does the `lib.rs` of the `torrust-tracker-test-helpers-3.0.0.crate` downloaded
  from `static.crates.io`; `git log --diff-filter=A` dates `http.rs` and `udp.rs` to 2026-07-29. After the fix,
  `grep -in 'semver-major\|need a semver'` over the two spec files matches only the appended correction entry in
  the Progress Log.
- Resolution reference: docs(issues): [#2482] leave the test-helpers semver bump to #2583
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2614#discussion_r4338655384>

## Processing Log

- 2026-10-21 15:03 UTC - Copilot submitted review 5601788677 with two inline threads. GraphQL data: 2 threads,
  unresolved.
- 2026-10-21 15:26 UTC - Verified both claims against the current tree and replied on each thread (reply times
  15:26:53 and 15:26:55 UTC) after pushing the two fix commits (commit times 15:21 and 15:23 UTC).
- 2026-10-21 15:27 UTC - Resolved both threads. A GraphQL query showed 0 unresolved threads.

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
