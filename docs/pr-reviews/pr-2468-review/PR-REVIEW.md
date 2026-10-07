---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
---

<!-- skill-link: process-pr-review -->

# PR #2468 Review Audit

Source: pull-request reviews and inline review threads for <https://github.com/torrust/torrust-tracker/pull/2468>.

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

Copilot review 5440515841 numbered its four inline findings F5, F6, F9, and F10; they keep those
IDs. Its overview also says the `interval_min` bound lacks protocol justification, but no inline
finding or retrievable suppressed comment carries that assertion, so it has no row (see the
Processing Log).

Human review 5440619059 (da2ce7, round 1) numbered its findings F1-F11. Its F1-F4, F7, F8, and F11
keep their IDs; its F5, F6, F9, and F10 collide with Copilot's and are recorded as F12, F13, F14,
and F15, with the reviewer's IDs in each detail entry.

Human review 5441434837 (da2ce7, round 2) numbered its findings F12 and F13, continuing its own
series; they collide with this audit's F12 and F13 and are recorded as F16 and F17.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F5 | `review-finding:pr-2468-f5` | Copilot | Major | link-integrity | ORIGINAL | FIXED | RESOLVED |
| F6 | `review-finding:pr-2468-f6` | Copilot | Minor | metadata | ORIGINAL | FIXED | RESOLVED |
| F10 | `review-finding:pr-2468-f10` | Copilot | Minor | maintainability | ORIGINAL | FIXED | RESOLVED |
| F9 | `review-finding:pr-2468-f9` | Copilot | Minor | metadata | ORIGINAL | FIXED | RESOLVED |
| F1 | `review-finding:pr-2468-f1` | Human | Major | testing | ORIGINAL | FIXED | RESOLVED |
| F2 | `review-finding:pr-2468-f2` | Human | Minor | testing | ORIGINAL | FIXED | RESOLVED |
| F3 | `review-finding:pr-2468-f3` | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F4 | `review-finding:pr-2468-f4` | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F12 | `review-finding:pr-2468-f12` | Human | Minor | metadata | RE_RAISE_OF:F6 | NO_ACTION | SUPERSEDED |
| F13 | `review-finding:pr-2468-f13` | Human | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F7 | `review-finding:pr-2468-f7` | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F8 | `review-finding:pr-2468-f8` | Human | Suggestion | maintainability | ORIGINAL | FIXED | RESOLVED |
| F14 | `review-finding:pr-2468-f14` | Human | Suggestion | documentation | ORIGINAL | FIXED | RESOLVED |
| F15 | `review-finding:pr-2468-f15` | Human | Suggestion | documentation | ORIGINAL | FIXED | RESOLVED |
| F11 | `review-finding:pr-2468-f11` | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F16 | `review-finding:pr-2468-f16` | Human | Minor | formatting | ORIGINAL | FIXED | RESOLVED |
| F17 | `review-finding:pr-2468-f17` | Human | Nit | correctness | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F5 - The reopened issue's Specification link is stale

- PR number: 2468
- Source review ID: 5440515841
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2468#discussion_r4205419792>
- Concern: The skill's reference search covers repository files only, so it misses the `Specification` link in the #1978 issue body, which named the legacy single file `docs/issues/open/1978-configuration-overhaul-epic.md`.
- Solution: Edited the #1978 issue body to link `docs/issues/open/1978-configuration-overhaul-epic/EPIC.md`, and added a Step 3 item to the skill that checks and edits the reopened issue's `Specification` link.
- Current-tree verification: `gh issue view 1978 --json body` shows the folder path on line 7; `reopen-issue/SKILL.md` Step 3 item 4 contains the `gh issue edit --body-file` command.
- Resolution reference: `docs(skills): [#2466] couple, register, and complete the reopen-issue skill`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2468#discussion_r4205966744>

### F6 - Unquoted issue markers in the ADR frontmatter

- PR number: 2468
- Source review ID: 5440515841
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2468#discussion_r4205419871>
- Concern: YAML parses the unquoted `issue #1978` and `issue #2245` values as `issue` followed by a comment.
- Solution: Quoted both values.
- Current-tree verification: The ADR frontmatter lists `"issue #1978"` and `"issue #2245"`.
- Resolution reference: `docs(adrs): [#2466] quote issue markers and add the affected code`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2468#discussion_r4205967026>

### F10 - The new skill lacks semantic Skill Links coupling

- PR number: 2468
- Source review ID: 5440515841
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2468#discussion_r4205419903>
- Concern: The skill depends on `cleanup-completed-issues` but has no `## Skill Links` section and no `skill-link: reopen-issue` marker in the dependent artifact.
- Solution: Added a `## Skill Links` section and the skill's own marker. Placed `skill-link: reopen-issue` markers in the cleanup skill, the frontmatter-validator README, and `lychee.toml`. The templates are listed without a marker, because specs copy template markers.
- Current-tree verification: `rg --hidden 'skill-link: reopen-issue'` matches the four marked files (the skill, the cleanup skill, the validator README, and `lychee.toml`), plus this audit, which quotes the marker; the skill ends with the Skill Links section.
- Resolution reference: `docs(skills): [#2466] couple, register, and complete the reopen-issue skill`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2468#discussion_r4205967263>

### F9 - The spec references #2245 by its lifecycle path

- PR number: 2468
- Source review ID: 5440515841
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2468#discussion_r4205419945>
- Concern: The spec cited the #2245 spec by its `docs/issues/closed/` path, which goes stale if #2245 moves again; the semantic-link convention asks for `issue #NNNN`.
- Solution: `related-artifacts` lists `"issue #2245"`, and the Background links the #2245 GitHub issue.
- Current-tree verification: `grep 'closed/2245' docs/issues/open/2466-1978-announce-interval-upper-bound/*.md` finds nothing.
- Resolution reference: `docs(issues): [#2466] address round-1 spec review findings`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2468#discussion_r4205967472>

### F1 - Bug spec reviewed without the pre-review reproduction

- PR number: 2468
- Source review ID: 5440619059
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2468#discussion_r4205507399>
- Concern: `fix-bug` requires the reproduction before maintainer review, recorded and classified in `manual-verification-evidence.md`; the spec deferred it, and the #2245 evidence shows only the UDP side.
- Solution: Reproduced on 2026-10-07 10:18 UTC against `develop` `7836471b3` plus this PR's documentation commits (stable Rust 1.99.0): the tracker starts, UDP returns `2147483647`, and HTTP returns `2147483648`. Recorded as Reproduced in the new evidence file, section V0, and linked from the Background, the Bug-Fix Process, and M1.
- Current-tree verification: `manual-verification-evidence.md` section V0 holds the configuration and both responses verbatim.
- Resolution reference: `docs(issues): [#2466] address round-1 spec review findings`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2468#discussion_r4205967722>

### F2 - No red-run regression task; AC1 not matched by the planned tests

- PR number: 2468
- Source review ID: 5440619059
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2468#discussion_r4205507422>
- Concern: `fix-bug` requires a regression-test task whose expected output is the red run, then separate fix and green-plus-recheck tasks; the tests checked only the field, while AC1 requires the field, value, and limit.
- Solution: T2 writes the configuration-load tests first with the red run as its expected output; T4 is the fix and T7 the green run plus recheck. The tests, M1, and M2 assert the field, the value, and the limit. Commit Points pair T2 with T4, because red tests cannot pass the pre-push gate alone.
- Current-tree verification: The Implementation Plan has T1-T7 and the Regression Test Strategy names the field, value, and limit.
- Resolution reference: `docs(issues): [#2466] address round-1 spec review findings`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2468#discussion_r4205968027>

### F3 - No migration-guide deliverable

- PR number: 2468
- Source review ID: 5440619059
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2468#discussion_r4205507431>
- Concern: The parent EPIC requires a migration-guide update for configuration public API changes, but the spec planned none.
- Solution: Added the guide update to In Scope, task T6, AC7, and its Acceptance Verification row, and listed the guide in `related-artifacts`.
- Current-tree verification: The spec mentions `migrate-v2-to-v3.md` in In Scope, T6, AC7, and the frontmatter.
- Resolution reference: `docs(issues): [#2466] address round-1 spec review findings`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2468#discussion_r4205968365>

### F4 - Verification policy omits toolchain qualification; checkpoints dropped

- PR number: 2468
- Source review ID: 5440619059
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2468#discussion_r4205507445>
- Concern: The spec dropped the template's manual-verification Notes, including toolchain qualification, and two workflow checkpoints.
- Solution: Restored the Notes block and status values, and the reviewer-validation and `agent-review-reports.md` checkpoints.
- Current-tree verification: The Manual Verification Scenarios section ends with the Notes block; the Workflow Checkpoints include both restored items.
- Resolution reference: `docs(issues): [#2466] address round-1 spec review findings`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2468#discussion_r4205968603>

### F12 - Unquoted issue markers parse as the bare string `issue`

- PR number: 2468
- Source review ID: 5440619059
- Reviewer finding ID: F5
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2468#discussion_r4205507464>
- Concern: Same as F6: quote the ADR's `issue #1978` and `issue #2245` values.
- Solution: Superseded by F6, which quoted both values.
- Current-tree verification: As for F6.
- Resolution reference: <https://github.com/torrust/torrust-tracker/pull/2468#discussion_r4205968866>
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2468#discussion_r4205968866>

### F13 - The EPIC log dates the reopen at the decision time

- PR number: 2468
- Source review ID: 5440619059
- Reviewer finding ID: F6
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2468#discussion_r4205507473>
- Concern: The #1978 Progress Log dated the reopen and the move at 08:29 UTC, the time of the maintainer's decision; GitHub shows the reopen comment at 08:50:40Z and the move came later in the PR.
- Solution: Split the entry into the 08:29 decision and an 08:50 entry for the GitHub actions, saying the specification PR moves the spec. The skill now says to date each entry at the action it records.
- Current-tree verification: `EPIC.md` has the two entries; `gh issue view 1978 --json comments` reports the reopen comment at 2026-10-07T08:50:40Z.
- Resolution reference: `docs(issues): [#1978] date the reopen at the GitHub reopen, not the decision`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2468#discussion_r4205969141>

### F7 - Skill Step 4 omits two edits the founding reopen made

- PR number: 2468
- Source review ID: 5440619059
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2468#discussion_r4205507481>
- Concern: Step 4 did not mention clearing the Acceptance Criteria checkbox that mirrors a reset row, or updating References.
- Solution: Extended the Acceptance Verification bullet to the matching checkbox and added a References bullet.
- Current-tree verification: `reopen-issue/SKILL.md` Step 4 contains both bullets.
- Resolution reference: `docs(skills): [#2466] couple, register, and complete the reopen-issue skill`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2468#discussion_r4205969378>

### F8 - Register the new lifecycle skill and name its dependencies

- PR number: 2468
- Source review ID: 5440619059
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2468#discussion_r4205507491>
- Concern: The skill was registered only through a cross-link, not in `docs/issues/README.md` or `docs/issues/closed/README.md`, and did not name the artifacts it restates.
- Solution: Registered the skill in both READMEs; the Skill Links section names the validator README, `lychee.toml`, and the EPIC and ISSUE templates.
- Current-tree verification: Both READMEs link `.github/skills/dev/planning/reopen-issue/SKILL.md`.
- Resolution reference: `docs(skills): [#2466] couple, register, and complete the reopen-issue skill`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2468#discussion_r4205969626>

### F14 - ADR lacks Affected Code and planned code back-links

- PR number: 2468
- Source review ID: 5440619059
- Reviewer finding ID: F9
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2468#discussion_r4205507499>
- Concern: `create-adr` Step 3.5 asks for an Affected Code section and module-level back-links from the code.
- Solution: Added the Affected Code section to the ADR, and planned the back-links in the spec's In Scope, T3, and T5 (spec commit `docs(issues): [#2466] address round-1 spec review findings`).
- Current-tree verification: The ADR has `## Affected Code` naming `packages/primitives/src/announce.rs` and `packages/udp-server/src/handlers/announce.rs`.
- Resolution reference: `docs(adrs): [#2466] quote issue markers and add the affected code`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2468#discussion_r4205969838>

### F15 - v2 inherits the bound through the shared type

- PR number: 2468
- Source review ID: 5440619059
- Reviewer finding ID: F10
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2468#discussion_r4205507504>
- Concern: v2 declares `announce_policy: AnnouncePolicy` too, so "v2 out of scope" read as "unchanged" although v2 inherits the bound.
- Solution: Out of Scope now says v2 inherits the bound through the shared type, reaching only library users of `v2_0_0`.
- Current-tree verification: `packages/configuration/src/v2_0_0/core.rs:17` declares `pub announce_policy: AnnouncePolicy`; the spec's Out of Scope states the inheritance.
- Resolution reference: `docs(issues): [#2466] address round-1 spec review findings`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2468#discussion_r4205970037>

### F11 - Step 2 prose names `git fetch`; the block runs `git pull --ff-only`

- PR number: 2468
- Source review ID: 5440619059
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2468#discussion_r4205507509>
- Concern: The caution named a command the block does not run.
- Solution: The prose now names `git pull --ff-only` and says the behind count catches a skipped or failed pull.
- Current-tree verification: `reopen-issue/SKILL.md` Step 2 prose matches its block.
- Resolution reference: `docs(skills): [#2466] couple, register, and complete the reopen-issue skill`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2468#discussion_r4205970304>

### F16 - Template blocks not copied verbatim

- PR number: 2468
- Source review ID: 5441434837
- Reviewer finding ID: F12
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2468#discussion_r4206173006>
- Concern: The template marks Status Values and Completion Rules as copied verbatim; this record rewrapped the Category line and dropped the last Completion Rules bullet.
- Solution: Copied both blocks from `docs/templates/PR-REVIEW-TEMPLATE.md` instead of the #2452 audit they had been copied from.
- Current-tree verification: `diff` of both sections against the template shows no difference apart from the template's guidance comment.
- Resolution reference: `docs(pr-reviews): [#2466] copy template blocks verbatim and log the second rebase on #2468`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2468#discussion_r4207455245>

### F17 - Two record statements no longer held at the round-2 head

- PR number: 2468
- Source review ID: 5441434837
- Reviewer finding ID: F13
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2468#discussion_r4206173023>
- Concern: The log recorded only the first rebase, and F10's verification counted four marker matches where the audit itself made a fifth.
- Solution: Added a 10:53 correction entry for the second rebase and push, and scoped F10's count to the four marked files plus this audit.
- Current-tree verification: The Processing Log has the 10:53 entry; F10's verification names the four files.
- Resolution reference: `docs(pr-reviews): [#2466] copy template blocks verbatim and log the second rebase on #2468`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2468#discussion_r4207455641>

## Processing Log

- 2026-10-07 09:46 UTC - Copilot review 5440515841 submitted findings F5, F6, F9, and F10.
- 2026-10-07 09:56 UTC - Human review 5440619059 (da2ce7, round 1) requested changes with eleven findings, recorded as F1-F4, F7, F8, F11, and F12-F15.
- 2026-10-07 10:49 UTC - Reproduced the defect for F1, fixed the other findings in four commits, edited the #1978 issue body for F5, rebased onto `develop` (9 commits behind), pushed after the pre-push suite passed, and replied to all fifteen threads. Copilot's overview remark that the `interval_min` bound lacks protocol justification has no inline thread; it was raised with the maintainer as an open specification question instead of being recorded as a finding.
- 2026-10-07 10:53 UTC - Correction to the 10:49 entry, which was committed before this step: `develop` had moved 10 more commits, so the branch was rebased again onto `24bf4746a` and force-pushed with this record, then all fifteen threads were resolved.
- 2026-10-07 11:12 UTC - Human review 5441434837 (da2ce7, round 2) verified the fifteen round-1 rows and requested changes with two findings, recorded as F16 and F17.
- 2026-10-07 13:12 UTC - The maintainer kept one type for `interval` and `interval_min` because both measure the same quantity, although no protocol bounds `interval_min`. Decision 2 states that reason in `docs(issues): [#2466] justify the shared interval_min type by meaning, not protocol`; the #2466 issue body was updated to match.
- 2026-10-07 13:26 UTC - Fixed F16 and F17, rebased onto `develop` (19 commits behind), pushed after the pre-push suite passed, and replied to both threads.
- 2026-10-07 14:37 UTC - Human review 5443287949 (da2ce7, round 3, 13:48 UTC, at `docs(pr-reviews): [#2466] record round-2 review findings on #2468`) approved with no new findings, verified F16 and F17 fixed, and confirmed the `interval_min` rationale and its record. No unresolved thread remains. At the maintainer's request, the branch was then rebased onto the latest `develop` with this entry, which dismisses that approval.

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
