---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
---

<!-- skill-link: process-pr-review -->

# PR #2752 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/2752.

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

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2752-f1 | Copilot | Minor (inferred) | link-integrity | ORIGINAL | NO_ACTION | SUPERSEDED |
| F2 | review-finding:pr-2752-f2 | Copilot | Minor (inferred) | link-integrity | ORIGINAL | NO_ACTION | SUPERSEDED |
| F3 | review-finding:pr-2752-f3 | Copilot | Minor (inferred) | documentation | ORIGINAL | FIXED | RESOLVED |
| F4 | review-finding:pr-2752-f4 | Copilot | Minor (inferred) | maintainability | ORIGINAL | FIXED | RESOLVED |
| F5 | review-finding:pr-2752-f5 | Copilot | Minor (inferred) | maintainability | RE_RAISE_OF:F4 | FIXED | RESOLVED |
| F6 | review-finding:pr-2752-f6 | Copilot | Nit (inferred) | metadata | ORIGINAL | FIXED | RESOLVED |
| F7 | review-finding:pr-2752-f7 | Copilot | Nit (inferred) | metadata | ORIGINAL | FIXED | RESOLVED |
| F8 | review-finding:pr-2752-f8 | Copilot | Nit (inferred) | formatting | ORIGINAL | FIXED | RESOLVED |
| F9 | review-finding:pr-2752-f9 | Copilot | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F10 | review-finding:pr-2752-f10 | Copilot | Minor | metadata | ORIGINAL | FIXED | NON_RESOLVABLE |
| F11 | review-finding:pr-2752-f11 | Copilot | Major (inferred) | correctness | ORIGINAL | FIXED | NON_RESOLVABLE |
| F12 | review-finding:pr-2752-f12 | Copilot | Major | correctness | RE_RAISE_OF:F11 | FIXED | RESOLVED |
| F13 | review-finding:pr-2752-f13 | Copilot | Minor (inferred) | documentation | ORIGINAL | FIXED | RESOLVED |
| F14 | review-finding:pr-2752-f14 | Human | Major | documentation | ORIGINAL | FIXED | RESOLVED |
| F15 | review-finding:pr-2752-f15 | Human | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F16 | review-finding:pr-2752-f16 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F17 | review-finding:pr-2752-f17 | Human | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F18 | review-finding:pr-2752-f18 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F19 | review-finding:pr-2752-f19 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F20 | review-finding:pr-2752-f20 | Human | Nit | metadata | ORIGINAL | FIXED | RESOLVED |
| F21 | review-finding:pr-2752-f21 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F22 | review-finding:pr-2752-f22 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F23 | review-finding:pr-2752-f23 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F24 | review-finding:pr-2752-f24 | Human | Major | documentation | ORIGINAL | FIXED | RESOLVED |
| F25 | review-finding:pr-2752-f25 | Human | Minor | documentation | RE_RAISE_OF:F16 | FIXED | RESOLVED |
| F26 | review-finding:pr-2752-f26 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F27 | review-finding:pr-2752-f27 | Human | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F28 | review-finding:pr-2752-f28 | Human | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F29 | review-finding:pr-2752-f29 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F30 | review-finding:pr-2752-f30 | Human | Nit | metadata | ORIGINAL | FIXED | RESOLVED |
| F31 | review-finding:pr-2752-f31 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F32 | review-finding:pr-2752-f32 | Copilot | Minor (inferred) | correctness | ORIGINAL | FIXED | RESOLVED |
| F33 | review-finding:pr-2752-f33 | Copilot | Minor (inferred) | testing | ORIGINAL | FIXED | RESOLVED |
| F34 | review-finding:pr-2752-f34 | Copilot | Minor (inferred) | testing | RE_RAISE_OF:F33 | FIXED | RESOLVED |
| F35 | review-finding:pr-2752-f35 | Copilot | Nit (inferred) | formatting | ORIGINAL | FIXED | RESOLVED |
| F36 | review-finding:pr-2752-f36 | Copilot | Nit (inferred) | documentation | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - Keep the stable `issue #2751` reference in the closed SI-23 spec

- PR number: 2752
- Source review ID: 5801499766
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2752#discussion_r4493457398
- Concern: The closed SI-23 specification lists a free-text `"issue #2751"` entry in `semantic-links.related-artifacts`; the reviewer prefers a resolvable path or URL.
- Solution: No change. The entry is the stable reference format the frontmatter validator accepts, and a path to an `open/` spec breaks when it moves to `closed/`. PR 2698 made the same decision for SI-24 (`review-finding:pr-2698-f1`).
- Current-tree verification: `docs/issues/closed/2449-1488-si-23-migrate-rest-api-test-environment/ISSUE.md` lists `"issue #2751"`; the pre-commit frontmatter validator passed on the commit that introduced it.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2752#discussion_r4494417164
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2752#discussion_r4494417164

### F2 - Keep the stable `issue #2751` reference in the closed SI-24 spec

- PR number: 2752
- Source review ID: 5801499766
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2752#discussion_r4493457497
- Concern: Same concern as F1 for the closed SI-24 specification.
- Solution: No change, for the reason given for F1.
- Current-tree verification: `docs/issues/closed/2450-1488-si-24-migrate-health-check-api-test-environment/ISSUE.md` lists `"issue #2751"`; the frontmatter validator passed.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2752#discussion_r4494417848
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2752#discussion_r4494417848

### F3 - Reconcile "not merged yet" with the published 0.5.0 release

- PR number: 2752
- Source review ID: 5801499766
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2752#discussion_r4493457562
- Concern: The paragraph says torrust-server-lib PR 4 is "not merged yet" and then cites a merge commit and the published 0.5.0 release.
- Solution: Changed the wording to "merged".
- Current-tree verification: `grep -ni "not merged"` over the spec folder finds nothing.
- Resolution reference: docs(issues): [#2751] state that the torrust-server-lib deprecation PR is merged
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2752#discussion_r4494418352

### F4 - Merge the duplicate Rollback sections

- PR number: 2752
- Source review ID: 5801499766
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2752#discussion_r4493457639
- Concern: The spec has two `## Rollback` sections, which is redundant and can diverge.
- Solution: Removed the first section and kept the later one, which also notes that the dependency bump can be reverted independently.
- Current-tree verification: `grep -c "^## Rollback" ISSUE.md` prints 1.
- Resolution reference: docs(issues): [#2751] merge the duplicate Rollback sections of the SI-18 spec
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2752#discussion_r4494418889

### F5 - Merge the duplicate Rollback sections (re-raise of F4)

- PR number: 2752
- Source review ID: 5801499766
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2752#discussion_r4493457701
- Concern: Same request as F4, raised on the second `## Rollback` heading.
- Solution: Fixed by the same commit as F4.
- Current-tree verification: `grep -c "^## Rollback" ISSUE.md` prints 1.
- Resolution reference: docs(issues): [#2751] merge the duplicate Rollback sections of the SI-18 spec
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2752#discussion_r4494419433

### F6 - Quote last-updated-utc in the evidence file

- PR number: 2752
- Source review ID: 5801499766
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2752#discussion_r4493457759
- Concern: The evidence file leaves `last-updated-utc` unquoted while `ISSUE.md` quotes it, so a YAML parser can read it as a timestamp.
- Solution: Quoted the value.
- Current-tree verification: Line 4 of `manual-verification-evidence.md` is `last-updated-utc: "2026-11-06 14:00"`.
- Resolution reference: docs(issues): [#2751] quote the last-updated-utc value in the SI-18 evidence file
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2752#discussion_r4494419950

### F7 - Remove the duplicate related artifact

- PR number: 2752
- Source review ID: 5801499766
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2752#discussion_r4493457817
- Concern: `semantic-links.related-artifacts` lists `packages/axum-health-check-api-server/src/server.rs` twice.
- Solution: Removed the second entry.
- Current-tree verification: `grep -c "axum-health-check-api-server/src/server.rs" ISSUE.md` prints 1.
- Resolution reference: docs(issues): [#2751] remove the duplicate related artifact from the SI-18 spec
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2752#discussion_r4494420438

### F8 - Use one spelling of behavior

- PR number: 2752
- Source review ID: 5801499766
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2752#discussion_r4493457884
- Concern: The spec writes "Behaviour" in some places and "behavior" in others.
- Solution: Changed the three "behaviour" spellings to "behavior".
- Current-tree verification: `grep -ni behaviour` over the spec folder finds nothing.
- Resolution reference: docs(issues): [#2751] use the American spelling of behavior in the SI-18 spec
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2752#discussion_r4494420960

### F9 - Update the cleared external eligibility gate

- PR number: 2752
- Source review ID: 5803331264
- Reviewer finding ID: F1
- Source URL: https://github.com/torrust/torrust-tracker/pull/2752#discussion_r4494797047
- Concern: The eligibility gate still marks the `torrust-server-lib` item as blocked on 0.4.0, although the split section and the evidence record the published 0.5.0 release with migration guidance and the support window.
- Solution: Ticked the item and replaced the blocker text with the 0.5.0 guidance. The reviewer's ID F1 collided with audit finding F1, so the audit assigned F9.
- Current-tree verification: the gate item is `[x]` and cites 0.5.0 and its `CHANGELOG.md`; the changelog read from the local torrust-server-lib clone lists migration steps and says the items stay for all 0.5.x releases and are removed in 0.6.0.
- Resolution reference: docs(issues): [#2751] mark the torrust-server-lib gate item complete with the 0.5.0 guidance
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2752#discussion_r4495030346

### F10 - Record the spec-only branch name

- PR number: 2752
- Source review ID: 5803331264
- Reviewer finding ID: F10
- Source URL: https://github.com/torrust/torrust-tracker/pull/2752#pullrequestreview-5803331264
- Concern: The spec's `branch:` frontmatter uses the suffix-free name reserved for the implementation branch, but the `create-issue` skill requires the `-spec` branch name for the spec-only stage.
- Solution: Set `branch:` to `2751-deprecate-legacy-shutdown-api-spec`.
- Current-tree verification: line 10 of `ISSUE.md` is `branch: "2751-deprecate-legacy-shutdown-api-spec"`, the name of this PR's head branch.
- Resolution reference: docs(issues): [#2751] record the spec-only branch name in the SI-18 spec
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2752#issuecomment-6690075811

### F11 - Resolve the deprecation release dependency cycle

- PR number: 2752
- Source review ID: 5803331264
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2752#pullrequestreview-5803331264
- Concern: #2583 publishes the unpublished crates only after EPIC #1488 closes, but SI-19 is part of that EPIC and requires the public support window to have ended, so the crates cannot publish the promised deprecations before removal.
- Solution: Confirmed against #2583 (prerequisite "EPIC #1488 closed"). The maintainer chose Option A: the five unpublished crates have no public support window, so SI-19 may remove their symbols without waiting; the window applies only to `torrust-server-lib` and the root `HaltSignal` and `NestedServerTask`. Updated SI-18 (Versioning Decisions, requirement 5, risks) and the SI-19 draft, and told the EPIC #1669 coordinator that #2583 needs no change.
- Current-tree verification: `grep -n "support window" ISSUE.md` in SI-18 shows the Versioning Decisions entry; the SI-19 draft start gate names the already-public symbols only.
- Resolution reference: docs(issues): [#2751] resolve the SI-18 and SI-19 release-order cycle
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2752#issuecomment-6690075811

### F12 - Resolve the remaining root-crate release cycle (re-raise of F11)

- PR number: 2752
- Source review ID: 5803954711
- Reviewer finding ID: F11
- Source URL: https://github.com/torrust/torrust-tracker/pull/2752#discussion_r4495302631
- Concern: #2583 publishes the libraries after EPIC #1488 closes and the root 4.0.0 after them, but the SI-19 start gate waits for a root-types support window, so the unpublished-crate exemption left the cycle for `HaltSignal` and `NestedServerTask`. The reviewer reused ID F11, so the audit assigned F12.
- Solution: Checked the root types against the tree: they first appear in `3ee28b723`, after the published 3.0.0, and are in no published release, so they need no support window. Limited the support window to `torrust-server-lib` in SI-18 (Versioning Decisions) and in the SI-19 draft (start gate and dependencies). This refines the maintainer's Option A decision using the same principle (a window only for already-public symbols).
- Current-tree verification: `git grep "HaltSignal\|NestedServerTask" v3.0.0` finds nothing; the SI-19 draft start gate now names only `torrust-server-lib` 0.5.x.
- Resolution reference: docs(issues): [#2751] limit the support window to torrust-server-lib
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2752#discussion_r4496337482

### F13 - Restore disposition and OPEN status rules

- PR number: 2752
- Source review ID: 5803954711
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2752#discussion_r4495302709
- Concern: The audit's copied Status Values and Completion Rules omit the template rule that a thread the current account cannot resolve keeps its disposition and gets `Thread state=OPEN`.
- Solution: Added the rule to both sections, copied from `docs/templates/PR-REVIEW-TEMPLATE.md`.
- Current-tree verification: the audit's Status Values section matches the template (diff shows only a blank line); both sections contain the `viewerCanResolve` rule.
- Resolution reference: docs(pr-reviews): [#2751] restore the unresolvable-thread rule in the PR 2752 audit
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2752#discussion_r4496337862

### F14 - Keep the maintainer-decision label on what was decided

- PR number: 2752
- Source review ID: 5823078440
- Reviewer finding ID: F1
- Source URL: https://github.com/torrust/torrust-tracker/pull/2752#discussion_r4510566692
- Concern: The "maintainer decision" label covered a root-type exemption the record did not attribute to the maintainer; the audit itself called it a refinement.
- Solution: Moved the root-type conclusion into its own paragraph, marked as derived from the tree and pending confirmation; the maintainer then confirmed it on 2026-11-09 and the spec records that.
- Current-tree verification: The spec paragraph reads "Derived from the tree, then confirmed by the maintainer on 2026-11-09"; the "Support window" bullet no longer mentions the root types.
- Resolution reference: docs(issues): [#2751] mark the root-type support window conclusion as derived
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2752#discussion_r4511987844

### F15 - List the legacy adapter types and constructors SI-19 removes

- PR number: 2752
- Source review ID: 5823078440
- Reviewer finding ID: F2
- Source URL: https://github.com/torrust/torrust-tracker/pull/2752#discussion_r4510566700
- Concern: The SI-18 symbol list omitted `Spawner::spawn_launcher`, `LaunchRequest`, the UDP `Running`, REST `Running::new`, the UDP derived `Running::new` and the health-check `RunningServer`, which SI-19 is told to remove.
- Solution: Added them to the SI-18 scope list.
- Current-tree verification: The scope list names each of those items; `grep -n "spawn_launcher\|LaunchRequest\|RunningServer" ISSUE.md` finds them.
- Resolution reference: docs(issues): [#2751] list the legacy adapter types and constructors in the SI-18 scope
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2752#discussion_r4511988235

### F16 - Define the deprecation text for symbols removed before first publication

- PR number: 2752
- Source review ID: 5823078440
- Reviewer finding ID: F3
- Source URL: https://github.com/torrust/torrust-tracker/pull/2752#discussion_r4510566708
- Concern: The `since` and removal-wording bullets and Deprecation Requirements 2 and 4 assumed releases that SI-19 comes before.
- Solution: Defined the attribute and release-note text per symbol group in Versioning Decisions and aligned requirements 2 and 4 and the release-notes criterion.
- Current-tree verification: Versioning Decisions lists the three groups; requirement 4 names only the `torrust-server-lib` release notes.
- Resolution reference: docs(issues): [#2751] define the deprecation text for symbols removed before first publication
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2752#discussion_r4511988583

### F17 - Define the end of the torrust-server-lib support window

- PR number: 2752
- Source review ID: 5823078440
- Reviewer finding ID: F4
- Source URL: https://github.com/torrust/torrust-tracker/pull/2752#discussion_r4510566713
- Concern: The SI-19 start gate waited for a window whose end nothing defined, which could recreate the release-order cycle.
- Solution: Defined the end: no fixed date, it ends at the maintainer's approval of 0.6.0, and the maintainer publishes 0.6.0 as part of SI-19. The maintainer confirmed this on 2026-11-09.
- Current-tree verification: Both SI-18 (Versioning Decisions) and the SI-19 start gate state the end condition and the publisher.
- Resolution reference: docs(issues): [#2751] define the end of the torrust-server-lib support window
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2752#discussion_r4511988988

### F18 - Record the gate commands and output in the evidence file

- PR number: 2752
- Source review ID: 5823078440
- Reviewer finding ID: F5
- Source URL: https://github.com/torrust/torrust-tracker/pull/2752#discussion_r4510566718
- Concern: Scenario M1 was DONE but the evidence file had neither links nor command output, and lacked the template sections.
- Solution: Rewrote `manual-verification-evidence.md` from the template with the executed commands, their output, a conclusion and repository paths for the linked records.
- Current-tree verification: The file has Environment, V1 Steps Performed, Observed Result and Conclusion sections.
- Resolution reference: docs(issues): [#2751] record the gate commands and output in the SI-18 evidence
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2752#discussion_r4511989349

### F19 - Correct the PR description

- PR number: 2752
- Source review ID: 5823078440
- Reviewer finding ID: F6
- Source URL: https://github.com/torrust/torrust-tracker/pull/2752#discussion_r4510566725
- Concern: The description understated the SI-19 change and omitted the audit file.
- Solution: Rewrote the PR description (a fix outside the tree), then updated it again after the maintainer's confirmations.
- Current-tree verification: `gh pr view 2752 --json body` shows the SI-19 change, the support-window summary and the audit file.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2752#issuecomment-6738454510
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2752#discussion_r4511989679

### F20 - Update the timestamps and the progress log

- PR number: 2752
- Source review ID: 5823078440
- Reviewer finding ID: F7
- Source URL: https://github.com/torrust/torrust-tracker/pull/2752#discussion_r4510566733
- Concern: `last-updated-utc` of SI-18 and SI-19 was stale and the SI-18 progress log had no time of day or entry for the window change.
- Solution: Updated both timestamps and added timed progress-log entries.
- Current-tree verification: Both files carry a 2026-11-09 `last-updated-utc`; the progress log has timed entries for the window change.
- Resolution reference: docs(issues): [#2751] update the SI-18 and SI-19 timestamps and the SI-18 progress log
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2752#discussion_r4511990003

### F21 - Name the drafts folder as the drafting location

- PR number: 2752
- Source review ID: 5823078440
- Reviewer finding ID: F8
- Source URL: https://github.com/torrust/torrust-tracker/pull/2752#discussion_r4510566737
- Concern: The checkpoint named the open path as the drafting location, reading as a move onto itself.
- Solution: Restored the checkpoint text so that it names the drafts folder as the drafting location (the draft file itself was moved to `open/`, not restored).
- Current-tree verification: The checkpoint line names the drafts folder.
- Resolution reference: docs(issues): [#2751] name the drafts folder as the SI-18 drafting location
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2752#discussion_r4511990298

### F22 - Attribute the root crate publication to the tracker release

- PR number: 2752
- Source review ID: 5823078440
- Reviewer finding ID: F9
- Source URL: https://github.com/torrust/torrust-tracker/pull/2752#discussion_r4510566744
- Concern: The spec said #2583 publishes the root crate, but #2583 lists it as out of scope.
- Solution: Removed that sentence and stated that the next tracker application release publishes the root crate after the libraries.
- Current-tree verification: `grep -n "does not publish the root crate" ISSUE.md` finds the corrected sentence.
- Resolution reference: docs(issues): [#2751] attribute the root crate publication to the tracker release
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2752#discussion_r4511990598

### F23 - Record that row 6 of #2482 has landed

- PR number: 2752
- Source review ID: 5823078440
- Reviewer finding ID: F10
- Source URL: https://github.com/torrust/torrust-tracker/pull/2752#discussion_r4510566747
- Concern: The spec still treated row 6 of #2482 as a future edit; it was merged on 2026-11-06.
- Solution: Updated the ordering bullet and the Risks bullet: row 6 (#2745) is merged and only row 7 (#2757) remains.
- Current-tree verification: `grep -n -i "row 6" ISSUE.md` prints the ordering bullet and the Risks bullet, both saying row 6 has landed.
- Resolution reference: docs(issues): [#2751] record that row 6 of #2482 has landed
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2752#discussion_r4511990926

### F24 - Mark rules written on 2026-11-09 as proposed, not decided on 2026-11-06

- PR number: 2752
- Source review ID: 5825152043
- Reviewer finding ID: F11
- Source URL: https://github.com/torrust/torrust-tracker/pull/2752#discussion_r4512170061
- Concern: The deprecation text written in answer to F16 sat under "Decided by the maintainer on 2026-11-06" and replaced the two bullets the maintainer decided.
- Solution: Moved the text under a "Proposed by the author, not yet confirmed" paragraph and restored the maintainer's since and removal bullets in the decided list.
- Current-tree verification: `grep -n "Proposed by the author" ISSUE.md` finds the paragraph; the decided list holds the since/removal bullet.
- Resolution reference: docs(issues): [#2751] mark the unpublished-symbol deprecation text as proposed
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2752#discussion_rhttps://github.com/torrust/torrust-tracker/pull/2752#discussion_r4512598467

### F25 - Align T2, T5, M4 and the Risks bullet with the deprecation text (re-raise of F16)

- PR number: 2752
- Source review ID: 5825152043
- Reviewer finding ID: F3
- Source URL: https://github.com/torrust/torrust-tracker/pull/2752#discussion_r4512170069
- Concern: T2, T5, the release-notes Scope bullet, M4 and the Risks `since` bullet still prescribed the wording the rules replaced.
- Solution: Rewrote those rows to follow Versioning Decisions.
- Current-tree verification: `grep -n "next breaking release" ISSUE.md` no longer matches T2; T5 has no release note.
- Resolution reference: docs(issues): [#2751] align the SI-18 plan and scope with the deprecation text decisions
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2752#discussion_rhttps://github.com/torrust/torrust-tracker/pull/2752#discussion_r4512598830

### F26 - Cite the maintainer's confirmation from a PR comment

- PR number: 2752
- Source review ID: 5825152043
- Reviewer finding ID: F18
- Source URL: https://github.com/torrust/torrust-tracker/pull/2752#discussion_r4512170083
- Concern: The maintainer's two confirmations exist only in the author's own records; the capture holds no comment from the maintainer's account.
- Solution: The maintainer posted their own comment on the PR, https://github.com/torrust/torrust-tracker/pull/2752#issuecomment-6740143243, and the spec cites it at both confirmations. The first reply was a FOLLOW_UP; the second reply, cited below, reports the fix.
- Current-tree verification: `grep -n "comment on PR 2752" ISSUE.md` prints the two lines that cite the maintainer's comment.
- Resolution reference: docs(issues): [#2751] cite the maintainer's comment for the support-window confirmations
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2752#discussion_r4512769396

### F27 - Name the LaunchRequest type and its rx_halt field

- PR number: 2752
- Source review ID: 5825152043
- Reviewer finding ID: F13
- Source URL: https://github.com/torrust/torrust-tracker/pull/2752#discussion_r4512170092
- Concern: `LaunchRequest` has no constructor and the entry left out the type itself, so T2 could not deprecate it.
- Solution: Changed the entry to name the type and its `rx_halt` field.
- Current-tree verification: `spawner.rs:18` is a plain struct; the entry reads "`LaunchRequest` (the type and its `rx_halt` field)".
- Resolution reference: docs(issues): [#2751] name the LaunchRequest type and its rx_halt field in the SI-18 scope
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2752#discussion_rhttps://github.com/torrust/torrust-tracker/pull/2752#discussion_r4512599616

### F28 - Word the root-type deprecation note separately

- PR number: 2752
- Source review ID: 5825152043
- Reviewer finding ID: F14
- Source URL: https://github.com/torrust/torrust-tracker/pull/2752#discussion_r4512170105
- Concern: The prescribed note said "before the crate's first publication", which is untrue for the root crate, published at 3.0.0.
- Solution: Gave the root types their own note: removed before the next root release; the slip fallback names both cases.
- Current-tree verification: The Versioning Decisions list has separate bullets for the unpublished crates and the root types.
- Resolution reference: docs(issues): [#2751] word the root-type deprecation note separately from the unpublished crates
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2752#discussion_rhttps://github.com/torrust/torrust-tracker/pull/2752#discussion_r4512599910

### F29 - Record the real output of the gate commands

- PR number: 2752
- Source review ID: 5825152043
- Reviewer finding ID: F16
- Source URL: https://github.com/torrust/torrust-tracker/pull/2752#discussion_r4512170116
- Concern: The Observed Result block was not the output of the listed commands: wrong order, a written summary, merged lines, and missing hits.
- Solution: Replaced it with the raw output of each command in print order and added the missing `states.rs` and `manager.rs` hits.
- Current-tree verification: The evidence file lists each command with its raw output.
- Resolution reference: docs(issues): [#2751] record the real output of the gate commands in the SI-18 evidence
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2752#discussion_rhttps://github.com/torrust/torrust-tracker/pull/2752#discussion_r4512600254

### F30 - Stamp last-updated-utc no later than the carrying commit

- PR number: 2752
- Source review ID: 5825152043
- Reviewer finding ID: F17
- Source URL: https://github.com/torrust/torrust-tracker/pull/2752#discussion_r4512170126
- Concern: Two `last-updated-utc` stamps differed from their commits' committer times.
- Solution: Set both stamps from the commit time and left both files unchanged afterwards.
- Current-tree verification: Both stamps are 12:08 and the carrying commit is at 12:08:50 UTC.
- Resolution reference: docs(issues): [#2751] set the SI-18 timestamps from the commit time
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2752#discussion_rhttps://github.com/torrust/torrust-tracker/pull/2752#discussion_r4512600519

### F31 - Correct the F23 verification command

- PR number: 2752
- Source review ID: 5825152043
- Reviewer finding ID: F19
- Source URL: https://github.com/torrust/torrust-tracker/pull/2752#discussion_r4512170135
- Concern: `grep -n "row 6"` prints only one of the two bullets because the Risks bullet spells it "Row 6".
- Solution: Changed the recorded command to a case-insensitive grep.
- Current-tree verification: `grep -n -i "row 6" ISSUE.md` prints both bullets.
- Resolution reference: docs(pr-reviews): [#2751] correct the F23 verification command in the PR 2752 audit
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2752#discussion_rhttps://github.com/torrust/torrust-tracker/pull/2752#discussion_r4512600843

### F32 - Reconcile "deprecated only, not deleted" with SI-19 removal

- PR number: 2752
- Source review ID: 5825238769
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2752#discussion_r4512239083
- Concern: The spec said the root types are deprecated only, not deleted, and later said SI-19 may remove them.
- Solution: Reworded to say SI-18 only deprecates them and SI-19 removes them under the stated conditions.
- Current-tree verification: `grep -n "SI-18 only deprecates" ISSUE.md` finds the bullet.
- Resolution reference: docs(issues): [#2751] state that SI-18 only deprecates the root types and SI-19 removes them
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2752#discussion_rhttps://github.com/torrust/torrust-tracker/pull/2752#discussion_r4512601280

### F33 - Record the exact crates.io command and its output

- PR number: 2752
- Source review ID: 5825238769
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2752#discussion_r4512239154
- Concern: Step 4 was not reproducible as written: the output was a summarized list and the extraction command was not recorded.
- Solution: Recorded the full `curl | python3` command and its own output.
- Current-tree verification: Step 4 of the evidence file holds the command and `[('0.5.0', '2026-11-06'), ('0.4.0', '2026-10-26')]`.
- Resolution reference: docs(issues): [#2751] record the exact crates.io command and output in the SI-18 evidence
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2752#discussion_rhttps://github.com/torrust/torrust-tracker/pull/2752#discussion_r4512601575

### F34 - Record the exact crates.io command and its output (re-raise of F33)

- PR number: 2752
- Source review ID: 5825238769
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2752#discussion_r4512239197
- Concern: Same request as F33, raised on a second line of the evidence file.
- Solution: Fixed by the same commit as F33.
- Current-tree verification: See F33.
- Resolution reference: docs(issues): [#2751] record the exact crates.io command and output in the SI-18 evidence
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2752#discussion_rhttps://github.com/torrust/torrust-tracker/pull/2752#discussion_r4512601968

### F35 - Change "form" to "format"

- PR number: 2752
- Source review ID: 5825238769
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2752#discussion_r4512239240
- Concern: The F1 solution said "reference form" for an accepted reference format.
- Solution: Changed the word to "format".
- Current-tree verification: `grep -c "reference format the frontmatter" PR-REVIEW.md` prints 1.
- Resolution reference: docs(pr-reviews): [#2751] fix a word in the F1 solution of the PR 2752 audit
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2752#discussion_rhttps://github.com/torrust/torrust-tracker/pull/2752#discussion_r4512602275

### F36 - Make the F21 solution precise

- PR number: 2752
- Source review ID: 5825238769
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2752#discussion_r4512239279
- Concern: The F21 solution read as if the draft file itself was restored, but the draft was moved to `open/`.
- Solution: Reworded to say the checkpoint text was restored to name the drafts folder and the file itself was moved.
- Current-tree verification: The F21 solution names the drafts folder as the drafting location and says the file was moved, not restored.
- Resolution reference: docs(pr-reviews): [#2751] make the F21 solution precise in the PR 2752 audit
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2752#discussion_rhttps://github.com/torrust/torrust-tracker/pull/2752#discussion_r4512602646

## Processing Log

- 2026-11-06 14:26 UTC - Copilot review 5801499766 submitted with 8 inline findings (F1 to F8); F5 re-raises F4.
- 2026-11-06 15:46 UTC - Five fix commits for F3 to F8 were made (committer times 15:44:22 to 15:46:14 UTC); they were pushed before the replies below.
- 2026-11-06 15:50 UTC - Replied on all 8 threads (the replies were created between 15:49:39 and 15:49:57 UTC after the fixes were pushed).
- 2026-11-06 16:27 UTC - Resolved all 8 threads; each mutation returned `isResolved: true`.
- 2026-11-06 16:30 UTC - Found Copilot review 5803331264 (submitted 16:21 UTC) with one inline finding (F9, reviewer ID F1, renumbered because F1 was taken) and two review-body findings (F10, F11).
- 2026-11-06 16:36 UTC - Fix commits for F9 to F11 were made (committer times 16:31:22 to 16:36:50 UTC) and pushed before the replies below.
- 2026-11-06 16:39 UTC - Replied on the F9 thread (16:39:20 UTC) and posted one PR conversation response covering F9 to F11 (16:39:23 UTC); resolved the F9 thread, which returned `isResolved: true`. F10 and F11 have no thread (`NON_RESOLVABLE`).
- 2026-11-06 18:12 UTC - Found Copilot review 5803954711 (submitted 17:04 UTC) with two inline findings: F12 (reviewer ID F11, a re-raise of F11) and F13. The fix commits were made at 18:11:35 and 18:12:37 UTC.
- 2026-11-06 18:51 UTC - Replied on both threads (18:51:36 and 18:51:39 UTC) before the push had succeeded: the first push failed with a network disconnect, so the replies cited commits that were not yet on the remote.
- 2026-11-06 18:57 UTC - The second push succeeded; resolved both threads, which returned `isResolved: true`.
- 2026-11-09 10:50 UTC - Found review 5823078440 by da2ce7 (submitted 08:04 UTC) with ten inline findings, F1 to F10, renumbered F14 to F23 because those IDs were taken. The fix commits were made between 10:50:11 and 10:57:05 UTC.
- 2026-11-09 11:00 UTC - Pushed the fix commits (pull-request head checked against the local head afterwards), then replied on all ten threads (11:00:36 to 11:00:58 UTC). The replies for F14 and F17 said the conclusions were pending the maintainer's confirmation.
- 2026-11-09 11:01 UTC - The maintainer confirmed both in chat; recorded in the spec by a commit made at 11:01:17 UTC and pushed.
- 2026-11-09 11:04 UTC - Posted one PR conversation response covering F14 to F23 (11:04:31 UTC) and resolved all ten threads, each returning `isResolved: true`.
- 2026-11-09 12:03 UTC - Started on Cameron's review 5825152043 (submitted 11:22 UTC, 8 inline findings; his F11, F3 re-raise, F18, F13, F14, F16, F17, F19, audited as F24 to F31) and Copilot's review 5825238769 (submitted 11:31 UTC, 5 inline findings, audited as F32 to F36). The fix commits were made between 12:03:36 and 12:12:01 UTC.
- 2026-11-09 12:15 UTC - Pushed the fix commits and checked the pull-request head against the local head, before any reply.
- 2026-11-09 12:15 UTC - Replied on all 13 threads (12:15:26 to 12:15:56 UTC). The F26 reply is a FOLLOW_UP: it needs the maintainer's comment on the PR.
- 2026-11-09 12:16 UTC - Resolved the 12 fixed threads, each returning `isResolved: true`; the F26 thread stays open.
- 2026-11-09 12:32 UTC - The maintainer posted a comment on PR 2752 (12:32:35 UTC, from `josecelano`, at the maintainer's request) confirming the root-type conclusion and the end of the 0.5.x window. The spec cites it in a commit made at 12:32:55 UTC and pushed; the pull-request head matched the local head afterwards.
- 2026-11-09 12:36 UTC - Replied on the F26 thread (12:36:09 UTC) and resolved it, which returned `isResolved: true`. All threads of the five rounds are now resolved.

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
