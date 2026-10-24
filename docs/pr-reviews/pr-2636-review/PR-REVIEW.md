---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
---

<!-- skill-link: process-pr-review -->

# PR #2636 Review Audit

Source: pull-request reviews and inline review threads for <https://github.com/torrust/torrust-tracker/pull/2636>.

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

Round 1: Copilot's review 5639728582 (COMMENTED, submitted 2026-10-24T19:58:25Z, on the opening head after the spec-record commit) opens two inline threads, F1 and F2; its overview warns that the agentic review timed out and its file table calls the moved helper files "module no longer exported", which describes the rename and raises no actionable assertion. da2ce7's review 5639731135 (APPROVED, submitted 2026-10-24T19:59:04Z, same head, dismissed by the 2026-10-24T20:10:46Z push of the fix commits) opens two inline threads, F3 and F4, which he labelled F1 and F2, next to an ACK of that head. Severities are the labels of the source comments; the Copilot severity "Medium" is recorded as Minor (inferred).

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2636-f1 | Copilot | Minor (inferred) | documentation | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-2636-f2 | Copilot | Minor (inferred) | documentation | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-2636-f3 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F4 | review-finding:pr-2636-f4 | Human | Nit | maintainability | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - AC1 says "no dependency" but means no direct dependency

- PR number: 2636
- Source review ID: 5639728582
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2636#discussion_r4368331312>
- Concern: AC1 is stated as "no dependency", but the acceptance verification says it means direct dependencies and notes that `primitives` stays reachable through `configuration`.
- Solution: AC1 says "no direct dependency", which is what V1 checks with `cargo tree --depth 1`, and a Progress Log entry records the change.
- Current-tree verification: read AC1 and the Acceptance Verification row; ran `cargo tree -p torrust-tracker-test-helpers -e normal --depth 1`, which lists four direct dependencies.
- Resolution reference: docs(issues): [#2628] say that AC1 means no direct dependency
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2636#discussion_r4368370960>

### F2 - Template instructions left in the retrospective

- PR number: 2636
- Source review ID: 5639728582
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2636#discussion_r4368331337>
- Concern: the retrospective keeps two instructional template lines, so it reads as a partly instantiated template.
- Solution: the two lines are removed and the file starts at its Purpose section.
- Current-tree verification: read the head of the file; the only template instruction was the two-line blockquote.
- Resolution reference: docs(issues): [#2628] drop the template instructions from the retrospective
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2636#discussion_r4368371052>

### F3 - The write-unit-test skill still says mock servers

- PR number: 2636
- Source review ID: 5639731135
- Reviewer finding ID: F1
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2636#discussion_r4368333448>
- Concern: `.github/skills/dev/testing/write-unit-test/SKILL.md` still tells readers to look in `test-helpers` for "mock servers, fixture generators, and utility types", although the PR rewrites the other descriptions.
- Solution: the line now describes test configurations, logging and random data and points protocol helpers with one consumer to that consumer's `tests/common/`.
- Current-tree verification: read the line and searched the skills, `docs` (outside issue specs, audits and ADRs), `AGENTS.md`, `packages/AGENTS.md` and `tests/AGENTS.md` for "mock server"; no other description of `test-helpers` remains.
- Resolution reference: docs(skills): [#2628] describe test-helpers as generic in the write-unit-test skill
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2636#discussion_r4368371125>

### F4 - Module-level dead-code allow departs from the #2238 rule

- PR number: 2636
- Source review ID: 5639731135
- Reviewer finding ID: F2
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2636#discussion_r4368333452>
- Concern: the module-level `allow(dead_code)` in the moved files departs from the closed #2238 rule (module level only where a whole module is unused by one binary).
- Solution: the retrospective now names the rule and the reasons for the departure: most binaries leave one of the two modules wholly unused, and the item-level form would need an allow on all 16 functions of the two files.
- Current-tree verification: read the #2238 spec lines and counted the functions of both files with `grep -c`; listed which test binary calls which helper.
- Resolution reference: docs(issues): [#2628] cite the #2238 dead-code rule as a deliberate departure
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2636#discussion_r4368371205>

## Processing Log

- 2026-10-24 20:12 UTC - Started audit. Copilot (19:58:25Z) and da2ce7 (19:59:04Z) reviewed the opening head and left four threads, F1 to F4. Verified each claim against the tree, committed one fix each (20:03:07Z, 20:04:08Z, 20:07:15Z and 20:07:50Z), pushed them together (the push dismissed da2ce7's approval at 20:10:46Z), replied on each thread (20:11:15Z to 20:11:20Z) and resolved all four (GraphQL showed 0 unresolved). A new ACK is needed on the head that carries this audit record.

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
