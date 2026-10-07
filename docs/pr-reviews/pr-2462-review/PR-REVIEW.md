---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
    - "issue #2446"
---

<!-- skill-link: process-pr-review -->

# PR #2462 Review Audit

Source: pull-request reviews and inline review threads for
<https://github.com/torrust/torrust-tracker/pull/2462>.

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

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | `review-finding:pr-2462-f1` | Copilot | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F2 | `review-finding:pr-2462-f2` | Copilot | Major | testing | ORIGINAL | FIXED | RESOLVED |
| F3 | `review-finding:pr-2462-f3` | Copilot | Major | correctness | ORIGINAL | FIXED | RESOLVED |
| F4 | `review-finding:pr-2462-f4` | Copilot | Major | correctness | ORIGINAL | FIXED | RESOLVED |
| F5 | `review-finding:pr-2462-f5` | Copilot | Major | documentation | ORIGINAL | FIXED | RESOLVED |
| F6 | `review-finding:pr-2462-f6` | Copilot | Minor | metadata | ORIGINAL | FIXED | RESOLVED |
| F7 | `review-finding:pr-2462-f7` | Copilot | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F8 | `review-finding:pr-2462-f8` | Copilot | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F9 | `review-finding:pr-2462-f9` | Human | Major | testing | RE_RAISE_OF:F2 | FIXED | RESOLVED |
| F10 | `review-finding:pr-2462-f10` | Human | Major | correctness | RE_RAISE_OF:F4 | FIXED | RESOLVED |
| F11 | `review-finding:pr-2462-f11` | Human | Major | correctness | RE_RAISE_OF:F3 | FIXED | RESOLVED |
| F12 | `review-finding:pr-2462-f12` | Human | Major | documentation | RE_RAISE_OF:F5 | FIXED | RESOLVED |
| F13 | `review-finding:pr-2462-f13` | Human | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F14 | `review-finding:pr-2462-f14` | Human | Minor | documentation | RE_RAISE_OF:F7 | FIXED | RESOLVED |
| F15 | `review-finding:pr-2462-f15` | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F16 | `review-finding:pr-2462-f16` | Human | Nit | correctness | RE_RAISE_OF:F1 | FIXED | RESOLVED |
| F17 | `review-finding:pr-2462-f17` | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F18 | `review-finding:pr-2462-f18` | Human | Nit | documentation | RE_RAISE_OF:F8 | FIXED | RESOLVED |
| F19 | `review-finding:pr-2462-f19` | Human | Suggestion | metadata | RE_RAISE_OF:F6 | FIXED | RESOLVED |

## Finding Details

### F1 - Off-by-one axum-rest-api-server dependency count

- PR number: 2462
- Source review ID: 5432054524
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2462#discussion_r4198285911>
- Concern: The Observations said `axum-rest-api-server` went from 12 to 13 workspace deps, while the generated sections of the June report and this report say 13 and 14.
- Solution: The Observations now say 13 to 14, using the tool's own `Workspace deps` counts rather than distinct crates.
- Current-tree verification: `Workspace deps:` lines for `torrust-tracker-axum-rest-api-server` in `docs/issues/open/1669-overhaul-packages/workspace-coupling-report-2026-10-06.md` and the June report inspected; the Observations sentence matches them.
- Resolution reference: `docs(issues): [#2446] correct the axum-rest-api-server dependency count`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2462#discussion_r4205412151>

### F2 - MV3 commands cannot produce the recorded comparison

- PR number: 2462
- Source review ID: 5432054524
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2462#discussion_r4198285757>
- Concern: MV3's commands printed only a count and the drawn edges; nothing generated the metadata edge list or ran a comparison, so the recorded `identical` result and AC5 were not reproducible.
- Solution: MV3 now records a self-contained script that maps `cargo metadata --no-deps` normal `torrust*` edges to diagram node names and compares them with the drawn edges, printing counts and both set differences.
- Current-tree verification: Extracted the MV3 script from `docs/issues/open/2446-1669-establish-baseline-analysis/manual-verification-evidence.md` and ran it after rebasing onto the latest `develop`: `expected edges: 153`, `drawn edges: 153`, `missing from diagram: []`, `extra in diagram: []`.
- Resolution reference: `docs(issues): [#2446] record a reproducible MV3 comparison`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2462#discussion_r4205412432>

### F3 - Test-helpers draft can close without decoupling

- PR number: 2462
- Source review ID: 5432054524
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2462#discussion_r4198285576>
- Concern: The draft's AC escape clauses let the issue complete without decoupling `test-helpers` or making it publishable, contradicting its own Goal and Risks.
- Solution: The draft now moves the helpers into `tests/common/` or a `publish = false` package, rejects the feature-gate and publish-first options with reasons, and AC2/AC4 have no escape clauses.
- Current-tree verification: `docs/issues/drafts/1669-decouple-test-helpers-from-unpublished-crates/ISSUE.md` Goal, options and AC2/AC4 inspected; no remaining escape clause.
- Resolution reference: `docs(issues): [#2446] require real decoupling in the test-helpers draft`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2462#discussion_r4205412664>

### F4 - Optional dependencies are still normal edges

- PR number: 2462
- Source review ID: 5432054524
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2462#discussion_r4198285671>
- Concern: Feature-gated dependencies keep `kind: null` with `optional: true` in Cargo metadata, so the draft's expected "no longer normal deps" outputs could not be met.
- Solution: The draft now says optional edges remain in `cargo metadata`, defines how the coupling tool and the diagram must label them, and adds AC8 for that labeling.
- Current-tree verification: `docs/issues/drafts/1669-gate-server-testing-modules-behind-feature/ISSUE.md` tasks, ACs and AC8 inspected against `cargo metadata --no-deps` output for an optional dependency.
- Resolution reference: `docs(issues): [#2446] represent optional edges in the testing-feature draft`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2462#discussion_r4205412934>

### F5 - Coupling-tool bug draft skips the fix-bug workflow

- PR number: 2462
- Source review ID: 5432054524
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2462#discussion_r4198285477>
- Concern: The bug draft had no issue-local reproduction evidence or classification and no like-for-like recheck, which `fix-bug/SKILL.md:106-147` requires before review.
- Solution: The draft now follows the fix-bug skill and has a new `manual-verification-evidence.md` with R1 Reproduced (five mis-resolved edges) and R2 TODO for the post-fix recheck.
- Current-tree verification: `docs/issues/drafts/1669-coupling-tool-resolve-lib-names-and-renames/ISSUE.md` and its sibling `manual-verification-evidence.md` inspected; R1 output matches the five edges in the regenerated report.
- Resolution reference: `docs(issues): [#2446] follow the fix-bug workflow in the coupling-tool draft`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2462#discussion_r4205413152>

### F6 - Long-lived report links the open-spec path

- PR number: 2462
- Source review ID: 5432054524
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2462#discussion_r4198285838>
- Concern: The report's frontmatter linked the issue spec under `docs/issues/open/`, which moves when #2446 closes; the semantic-link convention prefers the issue reference.
- Solution: The frontmatter now uses `"issue #2446"`.
- Current-tree verification: `docs/issues/open/1669-overhaul-packages/workspace-coupling-report-2026-10-06.md` frontmatter inspected; frontmatter validator passed in pre-commit.
- Resolution reference: `docs(issues): [#2446] reference issue #2446 instead of its open-spec path`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2462#discussion_r4205413411>

### F7 - REST API application and adapter drawn as Core

- PR number: 2462
- Source review ID: 5432054524
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2462#discussion_r4198286055>
- Concern: The diagram put `rest-api-application` and `rest-api-runtime-adapter` in the Core layer, while `docs/packages.md` and `packages/AGENTS.md` give them their own layers.
- Solution: The diagram now has separate `Runtime Adapter` and `REST API Application` subgraphs with their own class definitions.
- Current-tree verification: `docs/media/packages/dependencies-workspace-packages.md` subgraphs inspected; the Mermaid diagram validated; MV3 still reports 153/153 with no differences.
- Resolution reference: `docs(packages): [#2446] place REST API application and adapter in their own layers`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2462#discussion_r4205413703>

### F8 - AC1 checked while the report mentions rest-api-core

- PR number: 2462
- Source review ID: 5432054524
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2462#discussion_r4198285997>
- Concern: AC1 was checked although the hand-written Observations mention `rest-api-core`; the narrowing to generated sections lived only in the log and evidence.
- Solution: AC1 itself now limits the no-mention requirement to the generated sections (everything before `## Observations`); the hand-written Observations may name `rest-api-core` when recording its removal.
- Current-tree verification: AC1 text in `docs/issues/open/2446-1669-establish-baseline-analysis/ISSUE.md` inspected; `rest-api-core` appears only after `## Observations` in the 2026-10-06 report.
- Resolution reference: `docs(issues): [#2446] state AC1's scope in the criterion itself`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2462#discussion_r4205413962>

### F9 - MV evidence lines lack the commands that printed them

- PR number: 2462
- Source review ID: 5439725107
- Reviewer finding ID: F1
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2462#discussion_r4204784906>
- Concern: Re-raises F2 and extends it: MV1 and MV2 also recorded lines that no recorded command printed.
- Solution: MV3 fixed as in F2 (`docs(issues): [#2446] record a reproducible MV3 comparison`). MV1 and MV2 were rerun with commands that print every recorded line (member counts, generated-section comparison, tool exit code, timestamp-only diff), and the Environment section records each scenario's run time.
- Current-tree verification: Reran MV1/MV2 (2026-10-07 09:06 UTC) and MV3 (08:32 UTC) as recorded in `docs/issues/open/2446-1669-establish-baseline-analysis/manual-verification-evidence.md`; after rebasing onto the latest `develop`, the regenerated report's generated sections differ only in the `Generated:` stamp and MV3 still reports 153/153.
- Resolution reference: `docs(issues): [#2446] record the commands behind every MV1 and MV2 output line`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2462#discussion_r4205414224>

### F10 - Optional-dependency route cannot meet T2-T5, AC3 or MV2

- PR number: 2462
- Source review ID: 5439725107
- Reviewer finding ID: F2
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2462#discussion_r4204784914>
- Concern: Re-raises F4: an optional dependency is still a normal dependency, so the draft's tasks, AC3 and MV2 could not be met as written.
- Solution: Fixed by the F4 commit.
- Current-tree verification: `docs/issues/drafts/1669-gate-server-testing-modules-behind-feature/ISSUE.md` inspected as in F4.
- Resolution reference: `docs(issues): [#2446] represent optional edges in the testing-feature draft`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2462#discussion_r4205414514>

### F11 - Test-helpers options and escape clauses miss the Goal

- PR number: 2462
- Source review ID: 5439725107
- Reviewer finding ID: F3
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2462#discussion_r4204784918>
- Concern: Re-raises F3: two of three options cannot meet the Goal and AC2/AC4 let the issue close without decoupling.
- Solution: Fixed by the F3 commit.
- Current-tree verification: `docs/issues/drafts/1669-decouple-test-helpers-from-unpublished-crates/ISSUE.md` inspected as in F3.
- Resolution reference: `docs(issues): [#2446] require real decoupling in the test-helpers draft`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2462#discussion_r4205414719>

### F12 - Bug draft has no reproduction evidence

- PR number: 2462
- Source review ID: 5439725107
- Reviewer finding ID: F4
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2462#discussion_r4204784922>
- Concern: Re-raises F5: the fix-bug rule binds the draft, which had no reproduction evidence or scenario.
- Solution: Fixed by the F5 commit.
- Current-tree verification: `docs/issues/drafts/1669-coupling-tool-resolve-lib-names-and-renames/ISSUE.md` and its evidence file inspected as in F5.
- Resolution reference: `docs(issues): [#2446] follow the fix-bug workflow in the coupling-tool draft`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2462#discussion_r4205414920>

### F13 - Finding 3 omits the fifth mis-resolved edge

- PR number: 2462
- Source review ID: 5439725107
- Reviewer finding ID: F5
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2462#discussion_r4204784931>
- Concern: Finding 3 said four edges are falsely reported as unreferenced; the `axum-http-server` to `client-lib` dev edge is a fifth.
- Solution: Finding 3 now lists five edges including the dev edge; the spec's AC2 evidence and the coupling-tool draft's R1 say five.
- Current-tree verification: The five "No ... references found" edges in `docs/issues/open/1669-overhaul-packages/workspace-coupling-report-2026-10-06.md` counted against Finding 3's list.
- Resolution reference: `docs(issues): [#2446] count the axum-http-server dev edge in finding 3`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2462#discussion_r4205415186>

### F14 - REST API application and adapter drawn in Core

- PR number: 2462
- Source review ID: 5439725107
- Reviewer finding ID: F6
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2462#discussion_r4204784938>
- Concern: Re-raises F7.
- Solution: Fixed by the F7 commit.
- Current-tree verification: `docs/media/packages/dependencies-workspace-packages.md` inspected as in F7.
- Resolution reference: `docs(packages): [#2446] place REST API application and adapter in their own layers`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2462#discussion_r4205415469>

### F15 - Drafts omit the completion-review conditions

- PR number: 2462
- Source review ID: 5439725107
- Reviewer finding ID: F7
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2462#discussion_r4204784946>
- Concern: The three drafts' Implementation Completion Review sections carried only `Retrospective: Not yet assessed`, without the conditions `create-issue` requires and `docs/templates/ISSUE.md` carries.
- Solution: Copied the template's retrospective, no-discovery progress-log and agent-review-report conditions into all three drafts.
- Current-tree verification: The section in `docs/issues/drafts/1669-decouple-test-helpers-from-unpublished-crates/ISSUE.md`, `docs/issues/drafts/1669-gate-server-testing-modules-behind-feature/ISSUE.md` and `docs/issues/drafts/1669-coupling-tool-resolve-lib-names-and-renames/ISSUE.md` compared with `docs/templates/ISSUE.md`.
- Resolution reference: `docs(issues): [#2446] keep the template's completion-review conditions in the drafts`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2462#discussion_r4205415667>

### F16 - Dependency count uses distinct crates

- PR number: 2462
- Source review ID: 5439725107
- Reviewer finding ID: F8
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2462#discussion_r4204784954>
- Concern: Re-raises F1: the count should follow the report's own `Workspace deps:` lines.
- Solution: Fixed by the F1 commit.
- Current-tree verification: `docs/issues/open/1669-overhaul-packages/workspace-coupling-report-2026-10-06.md` inspected as in F1.
- Resolution reference: `docs(issues): [#2446] correct the axum-rest-api-server dependency count`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2462#discussion_r4205415898>

### F17 - Diagram draws an undeclared external edge

- PR number: 2462
- Source review ID: 5439725107
- Reviewer finding ID: F9
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2462#discussion_r4204784964>
- Concern: The diagram draws 154 edges, including `server-lib --> net-prim`, which no workspace manifest declares, while its prose and AC5 describe 153 manifest edges.
- Solution: The prose now says 153 manifest edges plus one external-to-external edge drawn for context, and the AC5 evidence row says "153 workspace edges" and names the extra edge.
- Current-tree verification: `docs/media/packages/dependencies-workspace-packages.md` prose and the AC5 row in `docs/issues/open/2446-1669-establish-baseline-analysis/ISSUE.md` inspected; MV3 (which excludes `server-lib` edges) reports 153/153.
- Resolution reference: `docs(packages): [#2446] state the diagram's external-to-external edge`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2462#discussion_r4205416212>

### F18 - AC1 qualification only in the evidence

- PR number: 2462
- Source review ID: 5439725107
- Reviewer finding ID: F10
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2462#discussion_r4204784971>
- Concern: Re-raises F8.
- Solution: Fixed by the F8 commit.
- Current-tree verification: AC1 in `docs/issues/open/2446-1669-establish-baseline-analysis/ISSUE.md` inspected as in F8.
- Resolution reference: `docs(issues): [#2446] state AC1's scope in the criterion itself`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2462#discussion_r4205416455>

### F19 - Report links the open-spec path

- PR number: 2462
- Source review ID: 5439725107
- Reviewer finding ID: F11
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2462#discussion_r4204784977>
- Concern: Re-raises F6.
- Solution: Fixed by the F6 commit.
- Current-tree verification: `docs/issues/open/1669-overhaul-packages/workspace-coupling-report-2026-10-06.md` frontmatter inspected as in F6.
- Resolution reference: `docs(issues): [#2446] reference issue #2446 instead of its open-spec path`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2462#discussion_r4205416709>

## Processing Log

- 2026-10-06 17:12 UTC - Copilot review 5432054524 posted eight inline findings, F1 to F8, with its own IDs F1 to F8.
- 2026-10-07 08:36 UTC - Human review 5439725107 (da2ce7) requested changes with eleven inline findings, his F1 to F11. They collide with the Copilot IDs, so they are recorded as F9 to F19 with the original ID in "Reviewer finding ID"; eight re-raise Copilot findings.
- 2026-10-07 09:45 UTC - Fixed all findings in one commit per concern, rebased onto the latest `develop`, confirmed the report and diagram still reproduce, pushed once after the pre-push suite passed, and replied on all 19 threads before recording them here.

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
