---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
---

<!-- skill-link: process-pr-review -->

# PR #2498 Review Audit

Source: pull-request reviews and inline review threads for <https://github.com/torrust/torrust-tracker/pull/2498>.

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

Copilot review 5460550155 raised two inline findings without reviewer finding IDs; they are recorded
as F1 and F2. Its body is an overview of those findings and the PR, so it has no row of its own.

Human review 5460660461 (da2ce7, round 1, changes requested) numbered its inline findings F1-F9. Its F3
to F9 keep their IDs. Its F1 and F2 collide with the Copilot findings already recorded as F1 and
F2, so they are recorded as F10 and F11, with the reviewer's IDs in their detail entries. The
review body summarizes those nine findings and the checks that produced no finding, so it has no
row of its own.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | `review-finding:pr-2498-f1` | Copilot | Minor (inferred) | documentation | ORIGINAL | FIXED | RESOLVED |
| F2 | `review-finding:pr-2498-f2` | Copilot | Nit (inferred) | metadata | ORIGINAL | FIXED | RESOLVED |
| F10 | `review-finding:pr-2498-f10` | Human | Major | correctness | ORIGINAL | FIXED | RESOLVED |
| F11 | `review-finding:pr-2498-f11` | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F3 | `review-finding:pr-2498-f3` | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F4 | `review-finding:pr-2498-f4` | Human | Suggestion | documentation | ORIGINAL | FIXED | RESOLVED |
| F5 | `review-finding:pr-2498-f5` | Human | Suggestion | documentation | ORIGINAL | FIXED | RESOLVED |
| F6 | `review-finding:pr-2498-f6` | Human | Nit | metadata | ORIGINAL | FIXED | RESOLVED |
| F7 | `review-finding:pr-2498-f7` | Human | Nit | formatting | ORIGINAL | FIXED | RESOLVED |
| F8 | `review-finding:pr-2498-f8` | Human | Nit | metadata | ORIGINAL | FIXED | RESOLVED |
| F9 | `review-finding:pr-2498-f9` | Human | Nit | link-integrity | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - The Code Scanning reproduction command does not pin the API version

- PR number: 2498
- Source review ID: 5460550155
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2498#discussion_r4222179234>
- Concern: The review inventory's Reproduction section pinned `X-GitHub-Api-Version` for the Code Quality call but not for the Code Scanning call, which makes the captured inventory harder to reproduce if GitHub changes defaults. Copilot rated it medium; the severity is inferred as Minor.
- Solution: Pinned `X-GitHub-Api-Version: 2022-11-28` on the Code Scanning call.
- Current-tree verification: The pinned command returns the same 66 open alerts; the pre-commit gate passes.
- Resolution reference: `docs(security): pin the Code Scanning API version`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2498#discussion_r4222528104>

### F2 - The process document omits the manual Docker scan skill from its skill links

- PR number: 2498
- Source review ID: 5460550155
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2498#discussion_r4222179313>
- Concern: The body of `public-scanner-findings.md` names `run-manual-docker-security-scan` as part of the security skill architecture, but its `semantic-links.skill-links` omitted it. Copilot rated it low; the severity is inferred as Nit.
- Solution: Added the skill to the document's `skill-links`, and the document to the skill's `related-artifacts`, so the link is bidirectional like the other two security skills.
- Current-tree verification: Both frontmatter lists contain the new entry; the pre-commit frontmatter validation and `linter all` pass.
- Resolution reference: `docs(security): link the manual Docker scan skill`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2498#discussion_r4222528431>

### F10 - The Trivy alert numbers in the inventory do not resolve to open alerts

- PR number: 2498
- Source review ID: 5460660461
- Reviewer finding ID: F1
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2498#discussion_r4222272843>
- Concern: None of the 31 Trivy alert numbers cited by the review inventory, catalog, bulk runtime-CVE record, `CVE-2026-85091.md`, evidence V3, or the zlib spec was an open alert; the open Trivy alerts for the same CVE ids had other numbers, so the zlib closure criterion targeted the wrong alerts.
- Solution: Root cause: `security-scan.yaml` scans `torrust/tracker:develop` on schedule and a local `torrust-tracker:local` build on push, under one SARIF category; Trivy uses the image name as the alert location, so each finding has two alerts and only the latest `develop` upload's series is open. The push run for `554c47f67` flipped the series at 16:55:43Z, after the first export. Added a Trivy Alert Series table with both numbers for every finding, cited both numbers everywhere a Trivy alert is cited, and added guidance to key Trivy findings by CVE id and package. With maintainer approval, opened bug #2499 for the workflow defect, with its spec and reproduction evidence on this branch.
- Current-tree verification: The scheduled-scan numbers equal the 31 `torrust/tracker` alerts fixed at 16:55:43Z and the push-scan numbers equal the 31 open Trivy alerts, paired one-to-one by CVE id and package; the rerun V2 reconciliation reports 68 source findings, none missing, none extra, and none in two clusters.
- Resolution reference: `docs(security): record both Trivy alert series`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2498#discussion_r4222528650>

### F11 - The boundary for Trivy alerts in Code Scanning is stated three ways

- PR number: 2498
- Source review ID: 5460660461
- Reviewer finding ID: F2
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2498#discussion_r4222272863>
- Concern: `public-scanner-findings.md` scoped the process to CodeQL and Code Quality, the CVE catalog skill's description sent every Code Scanning finding to the triage skill while its body said only source-level ones, and the routing table matched a Trivy alert on two rows; evidence V1 still said the sources route without overlap.
- Solution: Stated the split once and the same way everywhere: the triage process inventories and clusters every Code Scanning finding from any tool, including Trivy, and Code Quality findings; CVE reachability analysis is recorded through `catalog-security-vulnerabilities` and linked from the cluster. Updated the process document, both skills, the analysis README, evidence V1, and the M1 expected result.
- Current-tree verification: No remaining statement in the security docs or skills routes Code Scanning only for source-level or CodeQL findings; `linter markdown` passes.
- Resolution reference: `docs(security): state the Trivy routing split once`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2498#discussion_r4222528923>

### F3 - The evidence file does not follow the manual verification evidence template

- PR number: 2498
- Source review ID: 5460660461
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2498#discussion_r4222272865>
- Concern: The #2493 `manual-verification-evidence.md` lacked the template's frontmatter, environment and date, steps performed, and observed output; V2 stated counts without the query, time, or result.
- Solution: Rebuilt the file from `docs/templates/MANUAL-VERIFICATION-EVIDENCE.md` with commands actually run on 2026-10-08, their observed output, the reconciliation script verbatim, and a Failures section recording that the first export's raw output was not retained.
- Current-tree verification: The file has the template's frontmatter and sections; the rerun commands produced the recorded output; the pre-commit gate passes.
- Resolution reference: `docs(issues): [#2493] record verification evidence from the template`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2498#discussion_r4222529289>

### F4 - Record scanner severity and path for each inventory row

- PR number: 2498
- Source review ID: 5460660461
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2498#discussion_r4222272880>
- Concern: The process asks every cluster to record scanner severity and the skill asks each finding to record severity and affected path, but the inventory and catalog recorded neither.
- Solution: Added Scanner severity and Path columns to the inventory coverage table, a per-alert severity column to the Trivy Alert Series table, and a Scanner severity column to the catalog. Made the V2 reconciliation script read the alert-number column from the end of the row so it still works with the wider table.
- Current-tree verification: Values match the API `security_severity_level` and Code Quality severity fields; the updated V2 script reports no gaps.
- Resolution reference: `docs(security): record scanner severity and paths`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2498#discussion_r4222529698>

### F5 - The priority table has no row for needs-investigation

- PR number: 2498
- Source review ID: 5460660461
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2498#discussion_r4222272892>
- Concern: GSF-006 is a `p2` investigation on a Tier 1 surface, but no row in the priority assignment table covered an unconfirmed finding.
- Solution: Added a row: `p2` on a Tier 1 surface, `p3` on a lower tier, re-prioritized by the matching row once the investigation confirms or rules out impact.
- Current-tree verification: The table in `docs/security/README.md` has the new row; GSF-006 follows from it.
- Resolution reference: `docs(security): prioritize needs-investigation findings`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2498#discussion_r4222530153>

### F6 - Two catalog values fall outside the defined vocabularies

- PR number: 2498
- Source review ID: 5460660461
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2498#discussion_r4222272907>
- Concern: GSF-010's disposition and the surfaces of GSF-010 and GSF-011 were not among the defined values, and the #2493 spec spelled `need-investigation`.
- Solution: Used the exact defined disposition and surface values in every catalog row, linked both vocabulary definitions above the table, and corrected the spelling.
- Current-tree verification: Every catalog Disposition and Surface cell is a defined value; a search finds no `need-investigation`; `linter lychee` passes.
- Resolution reference: `docs(security): use the defined catalog vocabularies`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2498#discussion_r4222530477>

### F7 - The security index row is one place early

- PR number: 2498
- Source review ID: 5460660461
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2498#discussion_r4222272919>
- Concern: Rows in the `docs/index.md` table are ordered by file name, but `security/README.md` came before `schemas/README.md`.
- Solution: Swapped the two rows.
- Current-tree verification: `security/README.md` now follows `schemas/README.md`.
- Resolution reference: `docs(index): order the security row by file name`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2498#discussion_r4222530832>

### F8 - The spec layout drifts from the template

- PR number: 2498
- Source review ID: 5460660461
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2498#discussion_r4222272926>
- Concern: The #2493 Progress Log was an H2 after Implementation Completion Review, all four specs omitted Risks and Trade-offs and References, and `related-pr` was `null`.
- Solution: Moved the #2493 Progress Log under Progress Tracking, added Risks and Trade-offs and References to the #2493, #2495, #2496, and #2497 specs, nested Acceptance Verification under Verification Plan, set `related-pr: 2498`, and logged the change in each spec.
- Current-tree verification: Heading order in all four specs matches `docs/templates/ISSUE.md`; each `last-updated-utc` equals its newest log entry; `linter markdown` passes.
- Resolution reference: `docs(issues): align the security specs with the issue template`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2498#discussion_r4222531113>

### F9 - The policy document links the spec by its open path

- PR number: 2498
- Source review ID: 5460660461
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2498#discussion_r4222272940>
- Concern: `public-scanner-findings.md` linked the #2493 spec by its `docs/issues/open/` path, which breaks when the spec is archived.
- Solution: Replaced the link with the issue number and a note that the spec moves to `docs/issues/closed/` when the issue closes.
- Current-tree verification: No file under `docs/security/` or `.github/skills/dev/maintenance/` links `issues/open/2493`.
- Resolution reference: `docs(security): link the triage spec by issue number`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2498#discussion_r4222531485>

## Processing Log

- 2026-10-08 17:45 UTC - Fetched Copilot review 5460550155 and da2ce7 review 5460660461 with their 11 inline threads using GraphQL; recorded F1, F2, F10 (reviewer F1), F11 (reviewer F2), and F3 to F9.
- 2026-10-08 17:48 UTC - Confirmed F10 against the live alert API and traced it to the two Trivy image names in `security-scan.yaml`; the maintainer approved opening a bug issue.
- 2026-10-08 17:52 UTC - The maintainer approved the bug spec; created issue #2499.
- 2026-10-08 18:10 UTC - Committed every fix, rebased on `develop`, pushed after the pre-push gate passed, and replied on all 11 threads.
- 2026-10-08 18:12 UTC - Created this audit record after recording the review round in the #2493 spec.

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
