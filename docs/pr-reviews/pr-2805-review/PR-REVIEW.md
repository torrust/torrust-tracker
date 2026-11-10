---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
---

<!-- skill-link: process-pr-review -->

# PR #2805 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/2805.

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
- When the current identity cannot resolve a thread (`viewerCanResolve` is `false`), keep the
  disposition established from the current tree and record `Thread state=OPEN`. The reviewer or PR
  author resolves it; permission says nothing about whether the concern was fixed.

## Findings

Round 1 is da2ce7's review `5842841425` (F1 to F9) and Copilot's review `5842867931` (F17 to F19, numbered after the reviewer-provided IDs). Round 2 is da2ce7's review `5843238357` (F10 to F16).

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2805-f1 | Human | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-2805-f2 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-2805-f3 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F4 | review-finding:pr-2805-f4 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F5 | review-finding:pr-2805-f5 | Human | Nit | correctness | ORIGINAL | FIXED | RESOLVED |
| F6 | review-finding:pr-2805-f6 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F7 | review-finding:pr-2805-f7 | Human | Nit | correctness | ORIGINAL | FIXED | RESOLVED |
| F8 | review-finding:pr-2805-f8 | Human | Nit | correctness | ORIGINAL | FIXED | RESOLVED |
| F9 | review-finding:pr-2805-f9 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F10 | review-finding:pr-2805-f10 | Human | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F11 | review-finding:pr-2805-f11 | Human | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F12 | review-finding:pr-2805-f12 | Human | Nit | metadata | ORIGINAL | FIXED | RESOLVED |
| F13 | review-finding:pr-2805-f13 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F14 | review-finding:pr-2805-f14 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F15 | review-finding:pr-2805-f15 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F16 | review-finding:pr-2805-f16 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F17 | review-finding:pr-2805-f17 | Copilot | Suggestion (inferred) | correctness | ORIGINAL | FIXED | RESOLVED |
| F18 | review-finding:pr-2805-f18 | Copilot | Suggestion (inferred) | formatting | ORIGINAL | NO_ACTION | SUPERSEDED |
| F19 | review-finding:pr-2805-f19 | Copilot | Suggestion (inferred) | testing | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - UDP exposure bullet names variants that never reach a response

- PR number: 2805
- Source review ID: 5842841425
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2805#discussion_r4525583446
- Concern: The bullet named UDP error variants built only in tests and key expiry, which cannot occur on UDP.
- Solution: Rewrote the bullet around the whitelist error that does reach `ErrorResponse`; later refined by F10 and F11.
- Current-tree verification: Read udp-server/src/error.rs, event.rs and handlers/error.rs against the new bullet.
- Resolution reference: docs(issues): correct UDP exposure analysis in #2804 spec
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2805#discussion_r4525628315

### F2 - The origin the spec cites is not in the tree

- PR number: 2805
- Source review ID: 5842841425
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2805#discussion_r4525583457
- Concern: The spec cited a draft folder that does not exist.
- Solution: Pointed at the EPIC path and quoted the finding.
- Current-tree verification: grep for `1347-add-tests-primitives` in the spec finds no remaining citation.
- Resolution reference: docs(issues): address review of the #2804 specification
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2805#discussion_r4525697566

### F3 - Drafting-time reproduction not recorded as fix-bug requires

- PR number: 2805
- Source review ID: 5842841425
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2805#discussion_r4525583465
- Concern: No evidence file, no toolchain in the recorded outputs, and the red, fix and green tasks were folded together.
- Solution: Added manual-verification-evidence.md with the reproduction, named the toolchain and split T6 to T8.
- Current-tree verification: Re-ran the temporary test in this session and read the evidence file.
- Resolution reference: docs(issues): address review of the #2804 specification
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2805#discussion_r4525698053

### F4 - Two template passages dropped

- PR number: 2805
- Source review ID: 5842841425
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2805#discussion_r4525583473
- Concern: The Implementation Completion Review text and the test-development loop paragraph were missing.
- Solution: Restored both passages and the checkpoint wording.
- Current-tree verification: Compared the sections with docs/templates/ISSUE.md.
- Resolution reference: docs(issues): address review of the #2804 specification
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2805#discussion_r4525698488

### F5 - The three track_caller impls use the DynError conversion

- PR number: 2805
- Source review ID: 5842841425
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2805#discussion_r4525583482
- Concern: The spec said the impls call Located(..).into() and that LocatedError::into() logs.
- Solution: Corrected the conversion names and the logging claim.
- Current-tree verification: Read configuration/src/lib.rs and databases/error.rs and the located-error 3.0.0 source.
- Resolution reference: docs(issues): address review of the #2804 specification
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2805#discussion_r4525698866

### F6 - Quoted HTTP reason belongs to an omitted impl; split code spans

- PR number: 2805
- Source review ID: 5842841425
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2805#discussion_r4525583489
- Concern: The quoted failure reason was one impl's and the code spans were split across lines.
- Solution: Listed each From impl with its message and line; kept spans on one line.
- Current-tree verification: Read http-protocol requests and responses/error.rs; ran markdownlint.
- Resolution reference: docs(issues): address review of the #2804 specification
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2805#discussion_r4525699373

### F7 - Roughly half recomputes to about a third

- PR number: 2805
- Source review ID: 5842841425
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2805#discussion_r4525583497
- Concern: The test and production split was stated wrongly.
- Solution: Stated 62 lines outside asserts.rs: 21 test, 41 production.
- Current-tree verification: Re-ran the grep and the per-line classification.
- Resolution reference: docs(issues): address review of the #2804 specification
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2805#discussion_r4525699855

### F8 - Major bump is not the semver step for 0.1.0 crates

- PR number: 2805
- Source review ID: 5842841425
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2805#discussion_r4525583507
- Concern: The option text said every affected crate needs a major bump.
- Solution: Said a breaking release per crate, naming major and minor cases.
- Current-tree verification: Checked crate versions in the Cargo.toml files.
- Resolution reference: docs(issues): address review of the #2804 specification
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2805#discussion_r4525700364

### F9 - PR description lacks touched files and validation sections

- PR number: 2805
- Source review ID: 5842841425
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2805#discussion_r4525583516
- Concern: The PR body had no touched-files line and no validation line.
- Solution: Edited the PR description to add both.
- Current-tree verification: Read the PR body after the edit.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2805
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2805#discussion_r4525701019

### F10 - A UDP scrape for a non-whitelisted info-hash returns zeroed statistics

- PR number: 2805
- Source review ID: 5843238357
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2805#discussion_r4525883636
- Concern: The scrape half of the UDP trigger was wrong: the UDP scrape never builds the whitelist error.
- Solution: Named only the announce path in the bullet, the reproduction line, M2 and T7.
- Current-tree verification: Read scrape_handler.rs:171-181, udp-core scrape.rs and announce.rs:109.
- Resolution reference: docs(issues): list both UDP exposure paths in the #2804 specification
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2805#discussion_r4525983257

### F11 - The whitelist error is not the only location that reaches a UDP response

- PR number: 2805
- Source review ID: 5843238357
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2805#discussion_r4525883644
- Concern: A database error also reaches the UDP response when persistent_torrent_completed_stat is on.
- Solution: Listed the database error as a second UDP path for T2 and updated T7 and the log entry.
- Current-tree verification: Read the spec bullet and T7 after the edit.
- Resolution reference: docs(issues): list both UDP exposure paths in the #2804 specification
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2805#discussion_r4525983737

### F12 - Progress-log stamps and last-updated-utc postdate their commits

- PR number: 2805
- Source review ID: 5843238357
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2805#discussion_r4525883679
- Concern: The stamps 13:55 and 14:25 were later than the commits carrying them.
- Solution: Set 13:45 and 13:50 and last-updated-utc 14:16, from git log.
- Current-tree verification: git log --format=%cI for the three commits.
- Resolution reference: docs(issues): record real times and number the reproduction apart in #2804 evidence
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2805#discussion_r4525985031

### F13 - Ten replies cite branch commit ids

- PR number: 2805
- Source review ID: 5843238357
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2805#discussion_r4525883694
- Concern: Replies cited branch SHAs instead of commit subjects.
- Solution: Edited all earlier replies in place to cite the commit subjects.
- Current-tree verification: Fetched the comments after the edits and checked none contains a branch SHA.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2805#discussion_r4525985543
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2805#discussion_r4525985543

### F14 - Recorded run time postdates the commit carrying its output

- PR number: 2805
- Source review ID: 5843238357
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2805#discussion_r4525883661
- Concern: The evidence said the run was about 14:10, after the commit that carries it.
- Solution: Recorded the window between two named commits, that exact times were not kept, and set last-updated-utc no later than the carrying commit.
- Current-tree verification: Read the evidence file after the edit.
- Resolution reference: docs(issues): record real times and number the reproduction apart in #2804 evidence
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2805#discussion_r4525984181

### F15 - The PR has no audit record

- PR number: 2805
- Source review ID: 5843238357
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2805#discussion_r4525883671
- Concern: No docs/pr-reviews/pr-2805-review/PR-REVIEW.md, and the no-change reply lacked the prescribed form.
- Solution: Added this audit and changed the no-change reply to the prescribed form.
- Current-tree verification: Ran validate-audit-record.py (see the Processing Log).
- Resolution reference: docs(pr-reviews): add the review audit record for PR #2805
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2805#discussion_r4525984571

### F16 - M1 points at evidence section V1, which holds the PeerRole reproduction

- PR number: 2805
- Source review ID: 5843238357
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2805#discussion_r4525883712
- Concern: Section numbering collided with the scenario rows and a note was stale.
- Solution: Renamed the reproduction D1 and updated the note.
- Current-tree verification: Read the headings of the evidence file.
- Resolution reference: docs(issues): record real times and number the reproduction apart in #2804 evidence
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2805#discussion_r4525986055

### F17 - async track_caller claim presented as fact

- PR number: 2805
- Source review ID: 5842867931
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2805#discussion_r4525603521
- Concern: A claim marked to verify was stated as fact.
- Solution: Reworded it as a hypothesis for T1.
- Current-tree verification: Read the table row after the edit.
- Resolution reference: docs(issues): address review of the #2804 specification
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2805#discussion_r4525701587

### F18 - Tables use double leading pipes

- PR number: 2805
- Source review ID: 5842867931
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2805#discussion_r4525603597
- Concern: The reviewer saw rows starting with two pipes.
- Solution: No change: the tables use a single leading pipe.
- Current-tree verification: grep -n '^||' on the spec finds nothing; prettier and markdownlint pass.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2805#discussion_r4525702011
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2805#discussion_r4525702011

### F19 - Path assertions not fully specified and may be flaky

- PR number: 2805
- Source review ID: 5842867931
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2805#discussion_r4525603654
- Concern: The test strategy did not define which paths to forbid.
- Solution: Listed the forbidden substrings.
- Current-tree verification: Read the Regression Test Strategy section.
- Resolution reference: docs(issues): address review of the #2804 specification
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2805#discussion_r4525702483

## Processing Log

- 2026-11-10 14:22 UTC - Started audit for review rounds 1 and 2.

## Completion Rules

- Re-derive the reply claim against the current tree before replying or resolving a thread.
- Reply on every resolvable thread before resolving it.
- For an outdated thread whose concern was fixed, record `Disposition=FIXED` and
  `Thread state=RESOLVED`, even if GitHub marks the thread outdated after the push. For a
  duplicate, superseded, or no-change in-PR thread, reply exactly
  `Superseded by <FindingId>: <reason>.`, record `Disposition=NO_ACTION` and
  `Thread state=SUPERSEDED`, then resolve it. A post-merge `NO_ACTION` requires maintainer
  approval to decline the follow-up work. When the current identity cannot resolve a thread
  (`viewerCanResolve` is `false`), still reply, keep the disposition established from the current
  tree, and record `Thread state=OPEN`; the reviewer or PR author resolves it.
- A consolidated PR conversation response may cover multiple review rounds only when it names
  every review ID and every finding ID with its disposition and resolution reference. Record its
  durable URL in each related row.
- Cite a fix by its unique Conventional Commit subject or durable reply URL, never by a branch SHA
  that can change after a rebase.
- Refresh review threads using GraphQL and confirm that no unresolved actionable thread remains.
