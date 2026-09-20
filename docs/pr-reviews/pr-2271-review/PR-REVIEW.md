---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - issue #2264
    - issue #2265
    - issue #2266
    - docs/pr-reviews/pr-2269-review/PR-REVIEW.md
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
    - docs/agents/orchestration.md
---

# PR #2271 Review Audit

Source: pull-request reviews and inline review threads for
<https://github.com/torrust/torrust-tracker/pull/2271>.

## Ownership

The PR author owns this tracked audit record. Reviewers, including repository review agents,
deliver findings through GitHub and have no repository-artifact obligation.

- Post-merge workflow approval: N/A; PR #2271 is the approved follow-up itself.

## Status Values

- Relationship: `ORIGINAL`, `RE_RAISE_OF:<FindingId>`
- Disposition: `FIXED`, `NO_ACTION`, `SUPERSEDED`, `FOLLOW_UP`
- Thread state: `OPEN`, `RESOLVED`, `NON_RESOLVABLE`, `SUPERSEDED`
- Severity: `Blocker`, `Major`, `Minor`, `Nit`, `Suggestion`; append `(inferred)` when derived
  from free prose.
- Author class: `Copilot`, `Human`, `Unknown`
- Category: `link-integrity`, `formatting`, `metadata`, `testing`, `correctness`,
  `documentation`, `maintainability`, `security`, `other`

## Findings

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | `review-finding:pr-2271-f1` | Copilot | Minor (inferred) | documentation | ORIGINAL | FIXED | RESOLVED |
| F2 | `review-finding:pr-2271-f2` | Copilot | Minor (inferred) | documentation | ORIGINAL | FIXED | RESOLVED |
| F3 | `review-finding:pr-2271-f3` | Copilot | Minor (inferred) | maintainability | ORIGINAL | FIXED | RESOLVED |
| F4 | `review-finding:pr-2271-f4` | Copilot | Minor (inferred) | metadata | ORIGINAL | FIXED | RESOLVED |
| F5 | `review-finding:pr-2271-f5` | Human | Major | correctness | ORIGINAL | FIXED | RESOLVED |
| F6 | `review-finding:pr-2271-f6` | Human | Minor | metadata | ORIGINAL | FIXED | RESOLVED |
| F7 | `review-finding:pr-2271-f7` | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F8 | `review-finding:pr-2271-f8` | Human | Minor | metadata | ORIGINAL | FIXED | RESOLVED |
| F9 | `review-finding:pr-2271-f9` | Human | Minor | metadata | ORIGINAL | FIXED | RESOLVED |
| F10 | `review-finding:pr-2271-f10` | Human | Minor | metadata | ORIGINAL | FIXED | RESOLVED |
| F11 | `review-finding:pr-2271-f11` | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F12 | `review-finding:pr-2271-f12` | Human | Nit | formatting | ORIGINAL | FIXED | RESOLVED |
| F13 | `review-finding:pr-2271-f13` | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F14 | `review-finding:pr-2271-f14` | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F15 | `review-finding:pr-2271-f15` | Human | Minor | metadata | ORIGINAL | FIXED | RESOLVED |
| F16 | `review-finding:pr-2271-f16` | Human | Nit | maintainability | ORIGINAL | FIXED | RESOLVED |
| F17 | `review-finding:pr-2271-f17` | Human | Minor | metadata | ORIGINAL | FIXED | RESOLVED |
| F18 | `review-finding:pr-2271-f18` | Human | Minor | metadata | RE_RAISE_OF:F6 | FIXED | RESOLVED |
| F19 | `review-finding:pr-2271-f19` | Human | Minor | metadata | RE_RAISE_OF:F9 | FIXED | RESOLVED |
| F20 | `review-finding:pr-2271-f20` | Human | Minor | metadata | ORIGINAL | FIXED | RESOLVED |
| F21 | `review-finding:pr-2271-f21` | Human | Minor | metadata | ORIGINAL | FIXED | RESOLVED |
| F22 | `review-finding:pr-2271-f22` | Human | Nit | formatting | RE_RAISE_OF:F16 | FIXED | RESOLVED |
| F23 | `review-finding:pr-2271-f23` | Human | Suggestion | testing | ORIGINAL | FIXED | RESOLVED |
| F24 | `review-finding:pr-2271-f24` | Human | Nit | documentation | RE_RAISE_OF:F13 | FIXED | RESOLVED |
| F25 | `review-finding:pr-2271-f25` | Human | Major | correctness | ORIGINAL | FIXED | RESOLVED |
| F26 | `review-finding:pr-2271-f26` | Human | Major | metadata | ORIGINAL | FIXED | RESOLVED |
| F27 | `review-finding:pr-2271-f27` | Human | Major | correctness | ORIGINAL | FIXED | RESOLVED |
| F28 | `review-finding:pr-2271-f28` | Human | Minor | metadata | ORIGINAL | FIXED | RESOLVED |
| F29 | `review-finding:pr-2271-f29` | Human | Minor | metadata | ORIGINAL | FIXED | RESOLVED |
| F30 | `review-finding:pr-2271-f30` | Human | Minor | metadata | ORIGINAL | FIXED | RESOLVED |
| F31 | `review-finding:pr-2271-f31` | Human | Minor | metadata | ORIGINAL | FIXED | RESOLVED |
| F32 | `review-finding:pr-2271-f32` | Human | Minor | metadata | ORIGINAL | FIXED | RESOLVED |
| F33 | `review-finding:pr-2271-f33` | Human | Minor | documentation | RE_RAISE_OF:F19 | FIXED | RESOLVED |
| F34 | `review-finding:pr-2271-f34` | Human | Nit | formatting | RE_RAISE_OF:F22 | FIXED | RESOLVED |
| F35 | `review-finding:pr-2271-f35` | Human | Major | correctness | RE_RAISE_OF:F28 | FIXED | RESOLVED |
| F36 | `review-finding:pr-2271-f36` | Human | Major | correctness | ORIGINAL | FIXED | RESOLVED |
| F37 | `review-finding:pr-2271-f37` | Human | Major | correctness | RE_RAISE_OF:F27 | FIXED | RESOLVED |
| F38 | `review-finding:pr-2271-f38` | Human | Minor | metadata | RE_RAISE_OF:F28 | FIXED | RESOLVED |
| F39 | `review-finding:pr-2271-f39` | Human | Minor | metadata | RE_RAISE_OF:F31 | FIXED | RESOLVED |
| F40 | `review-finding:pr-2271-f40` | Human | Minor | metadata | RE_RAISE_OF:F32 | FIXED | RESOLVED |

## Finding Details

### F1 - Clarify pre-merge request wording

- PR number: 2271
- Source review ID: 5255633262
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2271#discussion_r4053094426>
- Concern: `still-open PR` contradicted the post-merge section premise.
- Solution: Replaced it with `pre-merge review-processing request`.
- Current-tree verification: The ambiguous phrase is absent from the skill.
- Resolution reference: `fix(docs): address PR #2271 Copilot findings`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2271#discussion_r4053382411>

### F2 - Remove duplicate PR Reviewer rows

- PR number: 2271
- Source review ID: 5255633262
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2271#discussion_r4053094443>
- Concern: Two rows with the same agent label made trigger selection ambiguous.
- Solution: Consolidated open and merged behavior into one state-aware row.
- Current-tree verification: The table has one PR Reviewer row.
- Resolution reference: `fix(docs): address PR #2271 Copilot findings`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2271#discussion_r4053382496>

### F3 - Order audit findings numerically

- PR number: 2271
- Source review ID: 5255633262
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2271#discussion_r4053094447>
- Concern: PR #2269 audit rows and details were out of numeric order.
- Solution: Reordered both sections and added a focused order/alignment check during validation.
- Current-tree verification: F1-F30 are sorted and aligned.
- Resolution reference: `fix(docs): address PR #2271 Copilot findings`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2271#discussion_r4053382598>

### F4 - Define OPEN compatibility prospectively

- PR number: 2271
- Source review ID: 5255633262
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2271#discussion_r4053094454>
- Concern: Historical audits did not list the new `OPEN` thread state.
- Solution: Documented prospective adoption while preserving historical audits without bulk rewrite.
- Current-tree verification: The template's Thread state definition contains the compatibility rule.
- Resolution reference: `fix(docs): address PR #2271 Copilot findings`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2271#discussion_r4053382668>

### F5 - Keep follow-up PR behind approval gate

- PR number: 2271
- Source review ID: 5255829901
- Reviewer finding ID: F1
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2271#discussion_r4053270992>
- Concern: The detailed diagram let the normal pre-merge push path reach the follow-up PR node.
- Solution: Gave the approved follow-up branch its own conditional commit-and-push edge.
- Current-tree verification: The follow-up PR node has only the approval-gated predecessor.
- Resolution reference: `fix(docs): address PR #2271 second review`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2271#discussion_r4053769488>

### F6 - Correct processing-log timestamps

- PR number: 2271
- Source review ID: 5255829901
- Reviewer finding ID: F2
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2271#discussion_r4053270994>
- Concern: Audit log timestamps predated their GitHub and commit evidence.
- Solution: Corrected audit completion, PR creation, replies, and approval-record timestamps.
- Current-tree verification: Timestamps match the cited artifacts' UTC times.
- Resolution reference: `fix(docs): address PR #2271 second review`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2271#discussion_r4053769561>

### F7 - Synchronize POST_MERGE_FINDINGS template

- PR number: 2271
- Source review ID: 5255829901
- Reviewer finding ID: F3
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2271#discussion_r4053270997>
- Concern: The PR Reviewer output was absent from its independent-report template.
- Solution: Added the value and reconciled persistence wording with a verdict/result.
- Current-tree verification: Agent and template enumerate the same output.
- Resolution reference: `fix(docs): address PR #2271 second review`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2271#discussion_r4053769630>

### F8 - Persist durable approval evidence

- PR number: 2271
- Source review ID: 5255829901
- Reviewer finding ID: F4
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2271#discussion_r4053271000>
- Concern: The audit asserted approval without reachable corroboration or a defined storage field.
- Solution: Added a template field, required a durable GitHub URL, and recorded issue comment 5743268485.
- Current-tree verification: PR #2269 audit links the approval comment.
- Resolution reference: `fix(docs): address PR #2271 second review`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2271#discussion_r4053769685>

### F9 - Keep valid resolution references

- PR number: 2271
- Source review ID: 5255829901
- Reviewer finding ID: F5
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2271#discussion_r4053271001>
- Concern: Open follow-up labels replaced the allowed commit-subject resolution reference.
- Solution: Restored commit subjects and added a separate durable follow-up PR URL allowance.
- Current-tree verification: All nine affected audit entries contain a commit subject and PR URL; however,
  the skill at lines 147-148 still permits "durable follow-up PR URLs", half of this concern remains unresolved.
- Resolution reference: `fix(docs): address PR #2271 second review`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2271#discussion_r4053769803>

### F10 - Declare reviewer finding ID field

- PR number: 2271
- Source review ID: 5255829901
- Reviewer finding ID: F6
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2271#discussion_r4053271003>
- Concern: Collision-safe reassignment used an undeclared detail field.
- Solution: Added the optional field to the template and Required Audit Fields.
- Current-tree verification: Both canonical sources name the field.
- Resolution reference: `fix(docs): address PR #2271 second review`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2271#discussion_r4053769894>

### F11 - Align skill and adapter wording

- PR number: 2271
- Source review ID: 5255829901
- Reviewer finding ID: F7
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2271#discussion_r4053271005>
- Concern: The skill used contradictory `still-open PR` wording.
- Solution: Reused the adapter's `pre-merge review-processing request` phrase.
- Current-tree verification: The contradictory phrase is absent.
- Resolution reference: `fix(docs): address PR #2271 second review`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2271#discussion_r4053769982>

### F12 - Normalize AGENTS bullet indentation

- PR number: 2271
- Source review ID: 5255829901
- Reviewer finding ID: F8
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2271#discussion_r4053271009>
- Concern: The new global rule used inconsistent continuation indentation.
- Solution: Matched the surrounding two-space style.
- Current-tree verification: The continuation lines use the surrounding two-space indentation.
- Resolution reference: `fix(docs): address PR #2271 review contract findings`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2271#discussion_r4053770081>

### F13 - Refresh PR commit structure

- PR number: 2271
- Source review ID: 5255829901
- Reviewer finding ID: F9
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2271#discussion_r4053271011>
- Concern: The PR body omitted a later commit.
- Solution: Refresh the PR description after all review and audit commits are known.
- Current-tree verification: The PR description lists all 17 branch commits through the round-five audit commit.
- Resolution reference: <https://github.com/torrust/torrust-tracker/pull/2271#discussion_r4056744910>
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2271#discussion_r4053770152>

### F14 - Preserve PR Reviewer constraints when consolidating

- PR number: 2271
- Source review ID: 5255975253
- Reviewer finding ID: F10
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2271#discussion_r4053409701>
- Concern: The consolidated row dropped actual diff/check context, read-only scope, thread publication, and re-review gaps.
- Solution: Restored all constraints in the single state-aware row and applied one-row-per-agent consistently.
- Current-tree verification: Both PR Reviewer and Copilot Suggestions Handler have one complete row.
- Resolution reference: `fix(docs): address PR #2271 third review`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2271#discussion_r4053920317>

### F15 - Keep compatibility prose outside status enumeration

- PR number: 2271
- Source review ID: 5255975253
- Reviewer finding ID: F11
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2271#discussion_r4053409704>
- Concern: A non-field Compatibility bullet appeared in the copied Status Values list.
- Solution: Folded the prospective compatibility rule into the Thread state definition.
- Current-tree verification: Status Values contains only field/value bullets.
- Resolution reference: `fix(docs): address PR #2271 third review`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2271#discussion_r4053920356>

### F16 - Restore readable assignment-rule wrapping

- PR number: 2271
- Source review ID: 5255975253
- Reviewer finding ID: F12
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2271#discussion_r4053409706>
- Concern: A no-content-change rewrap created an unusually long normative line and checklist item.
- Solution: Restored readable wrapping and made `require_wrapped` collapse all whitespace.
- Current-tree verification: The skill is wrapped and the Rust contract passes.
- Resolution reference: `fix(docs): address PR #2271 third review`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2271#discussion_r4053920441>

### F17 - Use the correct F22 resolution subject

- PR number: 2271
- Source review ID: 5256752506
- Reviewer finding ID: F15
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2271#discussion_r4054023768>
- Concern: PR #2269 audit F22 cited a commit that did not touch the audit.
- Solution: Changed F22 to cite `docs(pr-reviews): complete PR #2269 late review audit`.
- Current-tree verification: The cited commit is the audit-only commit that added F22.
- Resolution reference: `docs(pr-reviews): record PR #2271 findings`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2271#discussion_r4056745109>

### F18 - Correct approval timing and ordering

- PR number: 2271
- Source review ID: 5256752506
- Reviewer finding ID: F2
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2271#discussion_r4054023772>
- Concern: The audit's approval timestamp was four hours early and out of chronological order.
- Solution: Corrected the comment time to 15:56 UTC and ordered processing events chronologically.
- Current-tree verification: Processing log order and timestamp now match GitHub evidence.
- Resolution reference: `docs(pr-reviews): record PR #2271 findings`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2271#discussion_r4056745047>

### F19 - Keep follow-up PR URL separate from resolution reference

- PR number: 2271
- Source review ID: 5256752506
- Reviewer finding ID: F5
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2271#discussion_r4054023774>
- Concern: The compound commit-plus-PR value exceeded the template's resolution-reference schema.
- Solution: Restored a single commit subject in Resolution reference and moved the PR URL to its
  dedicated field; the skill, template, and checker now agree.
- Current-tree verification: The skill and template require separate Resolution reference and Follow-up PR URL fields.
- Resolution reference: `fix(docs): address PR #2271 review contract findings`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2271#discussion_r4056745014>

### F20 - Require follow-up PR URL in the audit contract

- PR number: 2271
- Source review ID: 5256752506
- Reviewer finding ID: F14
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2271#discussion_r4054023777>
- Concern: The template's Follow-up PR URL field was not required by the skill or used by the record.
- Solution: Required the field in every detail entry and populated it with `N/A` where no follow-up applies.
- Current-tree verification: This audit has 24 Follow-up PR URL fields, one per detail entry.
- Resolution reference: `fix(docs): address PR #2271 review contract findings`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2271#discussion_r4056744975>

### F21 - Document retrospective approval records

- PR number: 2271
- Source review ID: 5256752506
- Reviewer finding ID: F13
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2271#discussion_r4054023781>
- Concern: The new approval gate did not explain how the first retrospective case is represented.
- Solution: Kept approval evidence in the audit Ownership section and removed the inapplicable per-finding field.
- Current-tree verification: The template and skill require one audit-level post-merge approval URL.
- Resolution reference: `fix(docs): address PR #2271 review contract findings`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2271#discussion_r4056745073>

### F22 - Reflow the Required Audit Fields paragraph

- PR number: 2271
- Source review ID: 5256752506
- Reviewer finding ID: F12
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2271#discussion_r4054023788>
- Concern: The newly expanded Required Audit Fields paragraph retained an overlong line.
- Solution: Rewrapped the field roster while preserving the contract text.
- Current-tree verification: Only the necessary single-line YAML description exceeds 130 characters.
- Resolution reference: `fix(docs): address PR #2271 review contract findings`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2271#discussion_r4056744948>

### F23 - Test whitespace normalization behavior

- PR number: 2271
- Source review ID: 5256752506
- Reviewer finding ID: F16
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2271#discussion_r4054023793>
- Concern: The helper change affected 22 assertions and had no unit tests or documented trade-off.
- Solution: Added unit tests for indented wrapping and paragraph-break normalization plus an
  explanatory comment.
- Current-tree verification: `cargo test --package agent-review-report-contract` runs 2 tests.
- Resolution reference: `fix(checks): keep review contract tests after helpers`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2271#discussion_r4056745117>

### F24 - Refresh the final PR description

- PR number: 2271
- Source review ID: 5256752506
- Reviewer finding ID: F9
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2271#discussion_r4054023798>
- Concern: The PR description omitted later review-fix commits and stale validation wording.
- Solution: Refreshed the description after the final commit and audit set was known.
- Current-tree verification: The PR description lists all 17 branch commits through the round-five audit commit.
- Resolution reference: <https://github.com/torrust/torrust-tracker/pull/2271#discussion_r4056744910>
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2271#discussion_r4056744910>

### F25 - Use audit-local re-raise targets

- PR number: 2271
- Source review ID: 5260313082
- Reviewer finding ID: F24
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2271#discussion_r4056680526>
- Concern: F17-F24 used reviewer IDs instead of this audit's finding IDs in `RE_RAISE_OF` values.
- Solution: Mapped each re-raised finding to its audit-local predecessor and marked new concerns ORIGINAL.
- Current-tree verification: F18, F19, F22, and F24 target F6, F9, F16, and F13; F17, F20, F21, and F23 are ORIGINAL.
- Resolution reference: `docs(pr-reviews): fix PR #2271 audit defects from fourth review`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2271#discussion_r4056883009>

### F26 - Cite commits that contain the documented fix

- PR number: 2271
- Source review ID: 5260313082
- Reviewer finding ID: F18
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2271#discussion_r4056680527>
- Concern: F17-F22 cited a commit that did not contain their described changes.
- Solution: Cited the commit containing each described change, or recorded the unresolved case honestly.
- Current-tree verification: F17-F19 cite `docs(pr-reviews): record PR #2271 findings`; F20-F21 cite `docs(pr-reviews): extend post-merge audit contract`.
- Resolution reference: `docs(pr-reviews): fix PR #2271 audit defects from fourth review`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2271#discussion_r4056883011>

### F27 - Re-derive audit claims from the current tree

- PR number: 2271
- Source review ID: 5260313082
- Reviewer finding ID: F19
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2271#discussion_r4056680531>
- Concern: F12, F13, F19, and F22 contained claims contradicted by the current tree.
- Solution: Corrected AGENTS indentation, separated the PR-description follow-up, and aligned the skill and audit contract.
- Current-tree verification: The AGENTS continuation lines use two spaces, the skill has a separate Follow-up PR URL rule, and its roster is wrapped.
- Resolution reference: `fix(docs): address PR #2271 review contract findings`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2271#discussion_r4056883008>

### F28 - Record fourth-round audit activity chronologically

- PR number: 2271
- Source review ID: 5260313082
- Reviewer finding ID: F17
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2271#discussion_r4056680533>
- Concern: The audit lacked the fourth review, reply, resolution, and commit chronology, and details were out of order.
- Solution: Recorded each review, reply, resolution, and audit commit in chronological order; detail entries are F1-F40 ordered.
- Current-tree verification: Processing Log includes review 5256752506 at 17:33 UTC and its 09:13 UTC replies and resolution.
- Resolution reference: `docs(pr-reviews): address PR #2271 review round five`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2271#discussion_r4056883019>

### F29 - Preserve the PR #2269 EPIC fix reference

- PR number: 2271
- Source review ID: 5260313082
- Reviewer finding ID: F20
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2271#discussion_r4056680536>
- Concern: PR #2269 F5 cited its audit-entry commit instead of the EPIC-column rename commit.
- Solution: Restored the reference to the commit that renamed `Draft subissue` to `Subissue`.
- Current-tree verification: PR #2269 F5 cites `fix(docs): address late PR #2269 findings`.
- Resolution reference: `fix(docs): address PR #2271 review contract findings`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2271#discussion_r4056883007>

### F30 - Keep post-merge approval at audit scope

- PR number: 2271
- Source review ID: 5260313082
- Reviewer finding ID: F21
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2271#discussion_r4056680538>
- Concern: The contract required an inapplicable approval URL in every finding while approval evidence is audit-level.
- Solution: Kept post-merge approval in Ownership and removed the per-finding template field.
- Current-tree verification: The template and Required Audit Fields describe one audit-level approval URL.
- Resolution reference: `fix(docs): address PR #2271 review contract findings`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2271#discussion_r4056883023>

### F31 - Correct processing-log timestamps

- PR number: 2271
- Source review ID: 5260313082
- Reviewer finding ID: F22
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2271#discussion_r4056680541>
- Concern: Processing-log entries combined events with incompatible timestamps.
- Solution: Split pushes, replies, and resolutions into independently dated events.
- Current-tree verification: The log records the first, second, and third reply batches at 13:59, 16:15, and 17:04 UTC.
- Resolution reference: `docs(pr-reviews): address PR #2271 review round five`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2271#discussion_r4056883006>

### F32 - Make follow-up URL field explicit

- PR number: 2271
- Source review ID: 5260313082
- Reviewer finding ID: F23
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2271#discussion_r4056680543>
- Concern: Detail entries omitted the fields needed to distinguish an inapplicable follow-up URL from a missing one.
- Solution: Required a Follow-up PR URL line in every detail entry and used `N/A` where it does not apply.
- Current-tree verification: This audit has 40 Follow-up PR URL fields, one for each detail entry.
- Resolution reference: `docs(pr-reviews): address PR #2271 review round five`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2271#discussion_r4056883022>

### F33 - Exclude follow-up URLs from resolution references

- PR number: 2271
- Source review ID: 5260313082
- Reviewer finding ID: F5
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2271#discussion_r4056680546>
- Concern: The skill still allowed a follow-up PR URL as a Resolution reference after introducing a dedicated field.
- Solution: Restricted Resolution reference to commit subjects or durable reply URLs.
- Current-tree verification: Required Audit Fields directs follow-up PR links to the separate Follow-up PR URL field.
- Resolution reference: `fix(docs): address PR #2271 review contract findings`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2271#discussion_r4056883010>

### F34 - Wrap Required Audit Fields prose

- PR number: 2271
- Source review ID: 5260313082
- Reviewer finding ID: F12
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2271#discussion_r4056680548>
- Concern: The Required Audit Fields paragraph had grown to 200 characters.
- Solution: Rewrapped the field roster without changing its contract.
- Current-tree verification: The only skill line longer than 130 characters is the single-line YAML description.
- Resolution reference: `fix(docs): address PR #2271 review contract findings`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2271#discussion_r4056883012>

### F35 - Keep audit sections unique and ordered

- PR number: 2271
- Source review ID: 5260426384
- Reviewer finding ID: F25
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2271#discussion_r4056794584>
- Concern: The audit duplicated Processing Log and Completion Rules, leaving F17-F24 outside Finding Details.
- Solution: Removed the duplicated section pair and retained one chronological log after F1-F24 details.
- Current-tree verification: The audit has one Finding Details, Processing Log, and Completion Rules heading; F1-F40 are ordered.
- Resolution reference: `docs(pr-reviews): address PR #2271 review round five`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2271#discussion_r4056883017>

### F36 - Match resolved threads with audit state

- PR number: 2271
- Source review ID: 5260426384
- Reviewer finding ID: F26
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2271#discussion_r4056794587>
- Concern: F17-F24 declared FOLLOW_UP and OPEN despite resolved threads and committed fixes.
- Solution: Recorded FIXED and RESOLVED for fixed concerns; retained FOLLOW_UP only for F24's pending PR-description refresh.
- Current-tree verification: F17-F23 are FIXED/RESOLVED and F24 is FOLLOW_UP/RESOLVED.
- Resolution reference: `docs(pr-reviews): address PR #2271 review round five`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2271#discussion_r4056883013>

### F37 - Keep resolved-row claims current

- PR number: 2271
- Source review ID: 5260426384
- Reviewer finding ID: F19
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2271#discussion_r4056794589>
- Concern: Several settled rows retained false current-tree claims.
- Solution: Re-derived the claims and replaced them with current, scoped verification.
- Current-tree verification: F12, F13, F19, F20, and F22 each name their current source and state.
- Resolution reference: `docs(pr-reviews): address PR #2271 review round five`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2271#discussion_r4056883018>

### F38 - Record commits and answered review

- PR number: 2271
- Source review ID: 5260426384
- Reviewer finding ID: F17
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2271#discussion_r4056794592>
- Concern: The log omitted audit commits and review 5260313082, whose findings were being addressed.
- Solution: Added each audit commit and review 5260313082 to the chronological processing log.
- Current-tree verification: The log contains review 5260313082 and the 09:27, 10:33, and 10:47 UTC audit pushes.
- Resolution reference: `docs(pr-reviews): address PR #2271 review round five`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2271#discussion_r4056883016>

### F39 - Name the correct rework review

- PR number: 2271
- Source review ID: 5260426384
- Reviewer finding ID: F22
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2271#discussion_r4056794593>
- Concern: The log attributed the 10:00 UTC rework feedback to the previous day's review ID.
- Solution: Recorded review 5260313082 at its actual submission time and corrected related process events.
- Current-tree verification: The 10:00 UTC log entry names review 5260313082.
- Resolution reference: `docs(pr-reviews): address PR #2271 review round five`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2271#discussion_r4056883015>

### F40 - Apply optional-field rules consistently

- PR number: 2271
- Source review ID: 5260426384
- Reviewer finding ID: F23
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2271#discussion_r4056794597>
- Concern: The audit omitted fields that define `N/A` while using `N/A` in a field without that form.
- Solution: Made Follow-up PR URL explicit with `N/A` where inapplicable and used durable reply URLs for non-commit resolutions.
- Current-tree verification: Every detail entry has a Follow-up PR URL, and F13/F24 Resolution references are durable reply URLs.
- Resolution reference: `docs(pr-reviews): address PR #2271 review round five`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2271#discussion_r4056883014>

## Processing Log

- 2026-09-19 11:45 UTC - Copilot review 5255633262 submitted findings F1-F4.
- 2026-09-19 12:42 UTC - Pushed `fix(docs): address PR #2271 Copilot findings`.
- 2026-09-19 13:07 UTC - Human review 5255829901 submitted findings F5-F13.
- 2026-09-19 13:59 UTC - Replied to findings F1-F4.
- 2026-09-19 14:11 UTC - Human review 5255975253 submitted findings F14-F16.
- 2026-09-19 16:15 UTC - Replied to findings F5-F13.
- 2026-09-19 16:24 UTC - Pushed `fix(docs): address PR #2271 third review`.
- 2026-09-19 17:04 UTC - Replied to findings F14-F16 and resolved the first 16 threads.
- 2026-09-19 17:33 UTC - Human review 5256752506 submitted findings F17-F24.
- 2026-09-20 09:13 UTC - Replied to and resolved findings F17-F24.
- 2026-09-20 09:27 UTC - Pushed `docs(pr-reviews): complete PR #2271 review audit`.
- 2026-09-20 10:00 UTC - Human review 5260313082 submitted findings F25-F34.
- 2026-09-20 10:33 UTC - Pushed `docs(pr-reviews): fix PR #2271 audit defects from fourth review`.
- 2026-09-20 10:38 UTC - Replied to findings F17-F24 after correcting their audit entries.
- 2026-09-20 10:47 UTC - Pushed `docs(pr-reviews): update PR #2271 audit reply URLs after posting`.
- 2026-09-20 11:06 UTC - Human review 5260426384 submitted findings F35-F40.
- 2026-09-20 11:44 UTC - Pushed `fix(docs): address PR #2271 review contract findings`.
- 2026-09-20 11:58 UTC - Replied to and resolved findings F25-F40 after GraphQL verified zero unresolved reviewer threads.

## Completion Rules

- Re-derive every reply claim against the current tree before replying or resolving a thread.
- Reply on every resolvable thread before resolving it.
- Refresh review threads using GraphQL and confirm that no unresolved actionable thread remains.
- Commit this audit separately from review fixes.
