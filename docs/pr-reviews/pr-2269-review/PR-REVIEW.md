---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .gitattributes
    - docs/external-snapshots/README.md
    - issue #2264
    - issue #2265
    - issue #2266
---

# PR #2269 Review Audit

Source: pull-request reviews and inline review threads for
<https://github.com/torrust/torrust-tracker/pull/2269>.

## Ownership

The PR author owns this tracked audit record. Reviewers, including repository review agents,
deliver findings through GitHub and have no repository-artifact obligation.

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
| F1 | `review-finding:pr-2269-f1` | Copilot | Minor (inferred) | correctness | ORIGINAL | NO_ACTION | RESOLVED |
| F2 | `review-finding:pr-2269-f2` | Copilot | Minor (inferred) | correctness | ORIGINAL | FIXED | RESOLVED |
| F3 | `review-finding:pr-2269-f3` | Copilot | Suggestion (inferred) | maintainability | ORIGINAL | NO_ACTION | NON_RESOLVABLE |
| F4 | `review-finding:pr-2269-f4` | Copilot | Minor | link-integrity | ORIGINAL | FIXED | RESOLVED |
| F5 | `review-finding:pr-2269-f5` | Copilot | Minor | metadata | ORIGINAL | FOLLOW_UP | NON_RESOLVABLE |
| F6 | `review-finding:pr-2269-f6` | Copilot | Minor | metadata | ORIGINAL | FOLLOW_UP | NON_RESOLVABLE |
| F7 | `review-finding:pr-2269-f7` | Copilot | Minor | metadata | ORIGINAL | FIXED | RESOLVED |
| F8 | `review-finding:pr-2269-f8` | Copilot | Minor | metadata | ORIGINAL | FIXED | RESOLVED |
| F9 | `review-finding:pr-2269-f9` | Copilot | Minor | metadata | ORIGINAL | FIXED | RESOLVED |
| F10 | `review-finding:pr-2269-f10` | Copilot | Minor | metadata | ORIGINAL | FIXED | RESOLVED |
| F11 | `review-finding:pr-2269-f11` | Copilot | Minor | metadata | ORIGINAL | FIXED | RESOLVED |
| F12 | `review-finding:pr-2269-f12` | Copilot | Minor | metadata | ORIGINAL | FIXED | NON_RESOLVABLE |
| F13 | `review-finding:pr-2269-f13` | Copilot | Minor | metadata | ORIGINAL | FIXED | NON_RESOLVABLE |
| F14 | `review-finding:pr-2269-f14` | Copilot | Minor | metadata | ORIGINAL | FIXED | NON_RESOLVABLE |
| F15 | `review-finding:pr-2269-f15` | Copilot | Minor | correctness | ORIGINAL | FOLLOW_UP | NON_RESOLVABLE |
| F16 | `review-finding:pr-2269-f16` | Copilot | Minor | metadata | ORIGINAL | FOLLOW_UP | NON_RESOLVABLE |
| F17 | `review-finding:pr-2269-f17` | Copilot | Minor | metadata | ORIGINAL | FIXED | NON_RESOLVABLE |
| F18 | `review-finding:pr-2269-f18` | Copilot | Minor | metadata | ORIGINAL | FOLLOW_UP | NON_RESOLVABLE |
| F19 | `review-finding:pr-2269-f19` | Copilot | Minor | metadata | ORIGINAL | FIXED | NON_RESOLVABLE |
| F20 | `review-finding:pr-2269-f20` | Copilot | Minor | metadata | ORIGINAL | FOLLOW_UP | NON_RESOLVABLE |
| F21 | `review-finding:pr-2269-f21` | Copilot | Major | correctness | ORIGINAL | FOLLOW_UP | NON_RESOLVABLE |
| F22 | `review-finding:pr-2269-f22` | Human | Major | documentation | ORIGINAL | FOLLOW_UP | OPEN |
| F23 | `review-finding:pr-2269-f23` | Human | Minor | metadata | RE_RAISE_OF:F18 | FOLLOW_UP | OPEN |
| F24 | `review-finding:pr-2269-f24` | Human | Minor | metadata | RE_RAISE_OF:F20 | FOLLOW_UP | OPEN |
| F25 | `review-finding:pr-2269-f25` | Human | Minor | correctness | ORIGINAL | FOLLOW_UP | OPEN |
| F26 | `review-finding:pr-2269-f26` | Human | Minor | correctness | RE_RAISE_OF:F15 | FOLLOW_UP | OPEN |
| F27 | `review-finding:pr-2269-f27` | Human | Minor | metadata | RE_RAISE_OF:F6 | FOLLOW_UP | OPEN |
| F28 | `review-finding:pr-2269-f28` | Human | Minor | metadata | RE_RAISE_OF:F16 | FOLLOW_UP | OPEN |
| F29 | `review-finding:pr-2269-f29` | Human | Suggestion | metadata | ORIGINAL | FOLLOW_UP | OPEN |
| F30 | `review-finding:pr-2269-f30` | Human | Suggestion | metadata | ORIGINAL | FOLLOW_UP | OPEN |

## Finding Details

### F1 - Keep the unset whitespace attribute

- PR number: 2269
- Source review ID: 5249647686
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2269#discussion_r4048221446>
- Concern: Copilot stated that `-whitespace` would not disable whitespace-error checking and
  suggested assigning an explicit value instead.
- Solution: No change. The installed Git documentation defines an unset `whitespace` attribute as
  "Do not notice anything as error," which is the intended behavior. The existing `-whitespace`
  rule remains.
- Current-tree verification: `git check-attr whitespace -- <snapshot paths>` reports `unset` for
  both files; `git help gitattributes` documents that unset state as disabling all whitespace-error
  notices.
- Resolution reference: <https://github.com/torrust/torrust-tracker/pull/2269#discussion_r4048407746>
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2269#discussion_r4048407746>

### F2 - Prevent line-ending conversion

- PR number: 2269
- Source review ID: 5249647686
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2269#discussion_r4048221446>
- Concern: With `text` unspecified, client configuration such as `core.autocrlf` could convert line
  endings and violate the snapshot's byte-identity guarantee.
- Solution: Added `-text` to the two external-snapshot attribute patterns while retaining
  `-whitespace`.
- Current-tree verification: `git check-attr text whitespace -- <snapshot paths>` reports both
  attributes as `unset`. A temporary-index `git -c core.autocrlf=true add` retained upstream blob
  IDs `c06e3eede0c910d0ecf12524c34204156f8795ac` and
  `6b0b1270ff0ca8f03867efcd09ba6ddb6392b1e1`.
- Resolution reference: `fix(docs): preserve snapshot line endings`
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2269#discussion_r4048407746>

### F3 - Do not add Copilot runner configuration in this PR

- PR number: 2269
- Source review ID: 5249647686
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2269#pullrequestreview-5249647686>
- Concern: Copilot reported that its full agentic review did not start before timeout and suggested
  ensuring runner availability or adding `copilot-code-review.yml` with `runs-on`.
- Solution: No change. The review still covered 21 of 22 files and produced the actionable inline
  finding. Runner selection is repository automation architecture outside this specification PR and
  requires evidence and coordination under EPIC #2003 rather than an unplanned CI change here.
- Current-tree verification: The submitted review reports 21 of 22 files reviewed and one inline
  comment. This PR adds no Copilot runner policy, and its scope is specifications plus pinned
  evidence.
- Resolution reference: This audit's documented `NO_ACTION` disposition.
- Reply URL: N/A; submitted-review warning has no resolvable thread.

### F4 - Exclude snapshots from hosted link checks

- PR number: 2269
- Source review ID: 5250123822
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2269#discussion_r4048610567>
- Concern: The scheduled online Lychee workflow uses `.github/lychee-online.toml`, so the local-only
  snapshot exclusion did not prevent hosted checks from scanning immutable upstream files.
- Solution: Added the same `LICENSE.md`/`SPEC.md` snapshot path exclusion to the online Lychee
  configuration.
- Current-tree verification: `linter toml` passes. Directly running `lychee --config
  .github/lychee-online.toml` with both snapshot paths reports zero input files, zero links, and no
  errors.
- Resolution reference: `fix(docs): exclude snapshots from online link checks`
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2269#discussion_r4050022278>

### F5 - Use promoted subissue terminology

- PR number: 2269
- Source review ID: 5250123822
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2269#pullrequestreview-5250123822>
- Concern: The EPIC table retained the heading `Draft subissue` after #2265 and #2266 were created.
- Solution: Renamed the column to `Subissue`.
- Current-tree verification: The table heading now reads `Subissue`.
- Resolution reference: `fix(docs): address late PR #2269 findings`
- Reply URL: N/A; suppressed body finding had no thread.

### F6 - Refer to the promoted parent EPIC

- PR number: 2269
- Source review ID: 5250123822
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2269#pullrequestreview-5250123822>
- Concern: Issue #2265 still instructed implementers to update the `draft EPIC`.
- Solution: Changed the term to `parent EPIC`.
- Current-tree verification: The scope names the parent EPIC without a retired lifecycle state.
- Resolution reference: `fix(docs): address late PR #2269 findings`
- Reply URL: N/A; suppressed body finding had no thread.

### F7 - Use stable issue references from the child EPIC

- PR number: 2269
- Source review ID: 5250123822
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2269#discussion_r4048610834>
- Concern: EPIC #2264 linked the movable #2185 and #2233 issue-spec paths from long-lived
  frontmatter.
- Solution: Replaced those two issue-spec paths with `issue #2185` and `issue #2233`; retained the
  concrete evidence-file paths.
- Current-tree verification: EPIC #2264 frontmatter contains the two stable issue references and
  still contains the #2185/#2233 evidence paths.
- Resolution reference: `fix(docs): use stable issue references in metadata`
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2269#discussion_r4050022540>

### F8 - Use stable issue references from #2233 records

- PR number: 2269
- Source review ID: 5250123822
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2269#discussion_r4048610694>
- Concern: Closed issue #2233 linked EPIC #2264 through its movable open-spec path.
- Solution: Replaced the EPIC path with `issue #2264` in the issue frontmatter and in the matching
  implementation-retrospective frontmatter. Body navigation remains a concrete current path.
- Current-tree verification: Both #2233 frontmatter blocks contain `issue #2264`; neither contains
  the EPIC's open-spec path.
- Resolution reference: `fix(docs): use stable issue references in metadata`
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2269#discussion_r4050022961>

### F9 - Use stable issue references from #2185 records

- PR number: 2269
- Source review ID: 5250123822
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2269#discussion_r4048610635>
- Concern: Closed issue #2185 linked EPIC #2264 through its movable open-spec path.
- Solution: Replaced the EPIC path with `issue #2264` in the issue frontmatter and the matching
  agent-review-report frontmatter. The issue-local residual-evidence path remains concrete.
- Current-tree verification: Both #2185 frontmatter blocks contain `issue #2264`; neither contains
  the EPIC's open-spec path.
- Resolution reference: `fix(docs): use stable issue references in metadata`
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2269#discussion_r4050023403>

### F10 - Use a stable child reference from EPIC #2003

- PR number: 2269
- Source review ID: 5250123822
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2269#discussion_r4048610772>
- Concern: Parent EPIC #2003 linked child EPIC #2264 through its movable open-spec path.
- Solution: Replaced the frontmatter path with `issue #2264`. Applied the same invariant to the
  child specs: #2265 references parent `issue #2264`, and #2266 references `issue #2264` and
  predecessor `issue #2265`. Body tables retain navigable current paths.
- Current-tree verification: The affected frontmatter blocks contain stable issue references and
  no lifecycle-sensitive #2264/#2265 spec paths.
- Resolution reference: `fix(docs): use stable issue references in metadata`
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2269#discussion_r4050023662>

### F11 - Use stable issue references from the audit

- PR number: 2269
- Source review ID: 5250123822
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2269#discussion_r4048610900>
- Concern: This long-lived audit linked #2264–#2266 through movable open-spec paths.
- Solution: Replaced all three paths with `issue #2264`, `issue #2265`, and `issue #2266`.
- Current-tree verification: This audit's `related-artifacts` contains all three stable issue
  references and no #2264–#2266 spec path.
- Resolution reference: This audit update.
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2269#discussion_r4050023879>

### F12 - Use a stable EPIC reference from the #2233 retrospective

- PR number: 2269
- Source review ID: 5250123822
- Reviewer finding ID: F11; reassigned because F11 already identified a different audit finding.
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2269#pullrequestreview-5250123822>
- Concern: The durable #2233 retrospective used EPIC #2264's movable path.
- Solution: Replaced the path with `issue #2264`.
- Current-tree verification: The retrospective frontmatter contains `issue #2264`.
- Resolution reference: `fix(docs): use stable issue references in metadata`
- Reply URL: N/A; suppressed body finding had no thread.

### F13 - Use a stable EPIC reference from the #2185 review report

- PR number: 2269
- Source review ID: 5250123822
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2269#pullrequestreview-5250123822>
- Concern: Copilot body finding F13 requested `issue #2264` in the durable #2185 review report.
- Solution: Replaced the movable EPIC path with `issue #2264`.
- Current-tree verification: The #2185 report frontmatter contains `issue #2264` and no #2264
  open-spec path.
- Resolution reference: `fix(docs): use stable issue references in metadata`
- Reply URL: N/A; suppressed body finding had no thread.

### F14 - Refresh parent EPIC metadata

- PR number: 2269
- Source review ID: 5250123822
- Reviewer finding ID: F3; reassigned because F3 already identified a different audit finding.
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2269#pullrequestreview-5250123822>
- Concern: EPIC #2003's timestamp predated the child-EPIC additions.
- Solution: Refreshed `last-updated-utc` while synchronizing the #2264 boundary.
- Current-tree verification: EPIC #2003 records a timestamp after the added child content.
- Resolution reference: `fix(docs): use stable issue references in metadata`
- Reply URL: N/A; suppressed body finding had no thread.

### F15 - Keep first-observed GNU timeouts provisional

- PR number: 2269
- Source review ID: 5250123822
- Reviewer finding ID: F14; reassigned because F14 was allocated to an earlier collision.
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2269#pullrequestreview-5250123822>
- Concern: The EPIC described four first-observed GNU timeouts as permanently unremovable.
- Solution: Distinguished seven persistent errors from four timeouts that still require rerun.
- Current-tree verification: The handoff uses the preserved report's rerun-first classification.
- Resolution reference: `fix(docs): address late PR #2269 findings`
- Reply URL: N/A; suppressed body finding had no thread.

### F16 - Refer to the promoted successor specification

- PR number: 2269
- Source review ID: 5250123822
- Reviewer finding ID: F12; reassigned because F12 was allocated to an earlier collision.
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2269#pullrequestreview-5250123822>
- Concern: AC8 and its evidence row called created issue #2266 a `successor draft`.
- Solution: Changed both references to `successor specification`.
- Current-tree verification: AC8 and its evidence row use the promoted lifecycle term.
- Resolution reference: `fix(docs): address late PR #2269 findings`
- Reply URL: N/A; suppressed body finding had no thread.

### F17 - Use a stable parent reference from #2265

- PR number: 2269
- Source review ID: 5250123822
- Reviewer finding ID: F6; reassigned because F6 already identifies a different finding.
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2269#pullrequestreview-5250123822>
- Concern: #2265 frontmatter used EPIC #2264's movable path.
- Solution: Replaced it with `issue #2264`.
- Current-tree verification: #2265 frontmatter contains the stable issue reference.
- Resolution reference: `fix(docs): use stable issue references in metadata`
- Reply URL: N/A; suppressed body finding had no thread.

### F18 - Mark #2266 blocked

- PR number: 2269
- Source review ID: 5250123822
- Reviewer finding ID: F4; reassigned because F4 already identifies a different finding.
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2269#pullrequestreview-5250123822>
- Concern: #2266 frontmatter said `planned` despite its explicit #2265 blocker.
- Solution: Changed frontmatter status to `blocked`.
- Current-tree verification: Frontmatter, blocker prose, T1, and the parent table all agree.
- Resolution reference: `fix(docs): address late PR #2269 findings`
- Reply URL: N/A; suppressed body finding had no thread.

### F19 - Use stable parent and predecessor references from #2266

- PR number: 2269
- Source review ID: 5250123822
- Reviewer finding ID: F7; reassigned because F7 already identifies a different finding.
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2269#pullrequestreview-5250123822>
- Concern: #2266 frontmatter used movable paths for #2264 and #2265.
- Solution: Replaced them with `issue #2264` and `issue #2265`.
- Current-tree verification: #2266 frontmatter contains both stable issue references.
- Resolution reference: `fix(docs): use stable issue references in metadata`
- Reply URL: N/A; suppressed body finding had no thread.

### F20 - Use a stable #2003 reference from #2266

- PR number: 2269
- Source review ID: 5250123822
- Reviewer finding ID: F15; reassigned because F15 was allocated to an earlier collision.
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2269#pullrequestreview-5250123822>
- Concern: #2266 frontmatter retained the movable EPIC #2003 path.
- Solution: Replaced it with `issue #2003`.
- Current-tree verification: #2266 frontmatter contains the stable parent-automation reference.
- Resolution reference: `fix(docs): address late PR #2269 findings`
- Reply URL: N/A; suppressed body finding had no thread.

### F21 - Do not reject absent frontmatter universally

- PR number: 2269
- Source review ID: 5250123822
- Reviewer finding ID: F2; reassigned because F2 already identifies a different finding.
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2269#pullrequestreview-5250123822>
- Concern: T3 appeared to reject every Markdown document without frontmatter, contrary to scope.
- Solution: Limited missing-frontmatter errors to strict profiles that require frontmatter and
  retained malformed-delimiter diagnostics for present blocks.
- Current-tree verification: T3 now distinguishes malformed present blocks from absent optional
  frontmatter.
- Resolution reference: `fix(docs): address late PR #2269 findings`
- Reply URL: N/A; suppressed body finding had no thread.

### F22 - Complete the audit for review 5250123822

- PR number: 2269
- Source review ID: 5255050603
- Reviewer finding ID: F12
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2269#discussion_r4052607933>
- Concern: The audit omitted twelve independently actionable suppressed body findings.
- Solution: Added F5, F6, and F12-F21 with collision-safe IDs, full details, and dispositions.
- Current-tree verification: Every suppressed review-body assertion has one tracking row and one
  matching detail entry.
- Resolution reference: PR #2271 (open follow-up).
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2269#discussion_r4053066935>

### F23 - Align #2266 status with its blocker

- PR number: 2269
- Source review ID: 5255050603
- Reviewer finding ID: F13
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2269#discussion_r4052607936>
- Concern: `status: planned` contradicted #2266's explicit blocked state.
- Solution: Changed status to `blocked`.
- Current-tree verification: Frontmatter and all three in-body blocker signals agree.
- Resolution reference: PR #2271 (open follow-up).
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2269#discussion_r4053066989>

### F24 - Complete stable #2003 references

- PR number: 2269
- Source review ID: 5255050603
- Reviewer finding ID: F14
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2269#discussion_r4052607938>
- Concern: EPIC #2264 and issue #2266 retained lifecycle-sensitive paths to EPIC #2003.
- Solution: Replaced both with `issue #2003`.
- Current-tree verification: Neither frontmatter block contains the #2003 open-spec path.
- Resolution reference: PR #2271 (open follow-up).
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2269#discussion_r4053067048>

### F25 - Restore the historical draft path

- PR number: 2269
- Source review ID: 5255050603
- Reviewer finding ID: F15
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2269#discussion_r4052607941>
- Concern: A dated #2233 progress entry was rewritten to a path that did not exist on that date.
- Solution: Restored the original draft path and added the stable note `now EPIC #2264`.
- Current-tree verification: The 2026-09-16 entry names the path created on that date.
- Resolution reference: PR #2271 (open follow-up).
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2269#discussion_r4053067116>

### F26 - Correct the GNU timeout evidence

- PR number: 2269
- Source review ID: 5255050603
- Reviewer finding ID: F16
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2269#discussion_r4052607946>
- Concern: The handoff treated first-observed GNU timeouts as permanently unremovable.
- Solution: Distinguished persistent errors from timeouts awaiting a confirming rerun.
- Current-tree verification: The EPIC now agrees with its preserved residual report.
- Resolution reference: PR #2271 (open follow-up).
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2269#discussion_r4053067174>

### F27 - Retire `draft EPIC` terminology

- PR number: 2269
- Source review ID: 5255050603
- Reviewer finding ID: F17
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2269#discussion_r4052607950>
- Concern: #2265 still referred to promoted EPIC #2264 as a draft.
- Solution: Changed the scope item to `parent EPIC`.
- Current-tree verification: The retired lifecycle term is absent from the scope item.
- Resolution reference: PR #2271 (open follow-up).
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2269#discussion_r4053067227>

### F28 - Retire `successor draft` terminology

- PR number: 2269
- Source review ID: 5255050603
- Reviewer finding ID: F17; split because the thread contains two independent corrections.
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2269#discussion_r4052607950>
- Concern: #2265 AC8 called created issue #2266 a successor draft.
- Solution: Changed AC8 and its evidence row to `successor specification`.
- Current-tree verification: Both locations use the promoted lifecycle term.
- Resolution reference: PR #2271 (open follow-up).
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2269#discussion_r4053067227>

### F29 - Use stable issue identities in References

- PR number: 2269
- Source review ID: 5255050603
- Reviewer finding ID: F18
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2269#discussion_r4052607956>
- Concern: #2265 and #2266 References used non-linking `open/` paths for cross-issue identities.
- Solution: Replaced parent, predecessor, and successor paths with #2264, #2265, and #2266.
- Current-tree verification: Cross-issue References use stable issue numbers.
- Resolution reference: PR #2271 (open follow-up).
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2269#discussion_r4053067256>

### F30 - Inventory `epic:` on EPIC profiles

- PR number: 2269
- Source review ID: 5255050603
- Reviewer finding ID: F19
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2269#discussion_r4052607959>
- Concern: `epic:` is undocumented on EPIC profiles and has conflicting observed meanings.
- Solution: Added the #2264 parent value and #1938 self-reference as an explicit variation for
  issue #2265 to resolve.
- Current-tree verification: #2265 scope names both examples and the missing documented field.
- Resolution reference: PR #2271 (open follow-up).
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2269#discussion_r4053067285>

## Processing Log

- 2026-09-18 15:38 UTC - Copilot submitted review 5249647686 with one inline
  thread and a full-review timeout warning.
- 2026-09-18 15:50 UTC - Rebased onto `torrust/develop` at `ae243b1a`; retained upstream's newer
  EPIC #2003 timestamp while preserving the #2264 child-EPIC entry.
- 2026-09-18 15:55 UTC - Verified Git attribute semantics, added `-text`, and proved blob identity
  under `core.autocrlf=true`.
- 2026-09-18 16:00 UTC - Pushed `fix(docs): preserve snapshot line endings`, replied to the inline
  thread, verified reply presence, and resolved thread `PRRT_kwDOGp2yqc6jzFXj`.
- 2026-09-18 16:01 UTC - Recorded all findings and dispositions in this audit.
- 2026-09-18 16:27 UTC - Copilot submitted review 5250123822 with six inline findings.
- 2026-09-18 16:55 UTC - Rebased onto `torrust/develop` at `fde6833c`; the folder-style artifact
  migration replayed without conflicts.
- 2026-09-18 17:05 UTC - Added the hosted snapshot exclusion and replaced lifecycle-sensitive
  issue-spec paths in current frontmatter with stable issue references.
- 2026-09-18 17:15 UTC - Pushed the review fixes, replied to all six threads, verified reply
  presence, and resolved every second-round thread.
- 2026-09-19 07:59 UTC - Human review 5255050603 arrived after PR #2269 merged with one Major,
  five Minor, and two Suggestion threads; normalized its F17 thread into two independent findings.
- 2026-09-19 09:06 UTC - Maintainer explicitly approved post-merge remediation and required the
  previously undocumented process to be added to repository instructions, skills, agents, and
  orchestration guidance.
- 2026-09-19 09:06 UTC - Added the twelve omitted Copilot body findings and nine late-human
  findings; all unmerged corrections remain `FOLLOW_UP` until the follow-up PR merges.
- 2026-09-19 09:12 UTC - Opened follow-up PR #2271, replied to all eight late threads with their
  `FOLLOW_UP` disposition, and left them open pending merge.

## Completion Rules

- Re-derive every reply claim against the current tree before replying or resolving a thread.
- Reply on every resolvable thread before resolving it.
- Refresh review threads using GraphQL and confirm that no unresolved actionable thread remains.
- Commit this audit separately from the review fix.
