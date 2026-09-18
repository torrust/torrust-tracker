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
- Thread state: `RESOLVED`, `NON_RESOLVABLE`, `SUPERSEDED`
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
| F7 | `review-finding:pr-2269-f7` | Copilot | Minor | metadata | ORIGINAL | FIXED | RESOLVED |
| F8 | `review-finding:pr-2269-f8` | Copilot | Minor | metadata | ORIGINAL | FIXED | RESOLVED |
| F9 | `review-finding:pr-2269-f9` | Copilot | Minor | metadata | ORIGINAL | FIXED | RESOLVED |
| F10 | `review-finding:pr-2269-f10` | Copilot | Minor | metadata | ORIGINAL | FIXED | RESOLVED |
| F11 | `review-finding:pr-2269-f11` | Copilot | Minor | metadata | ORIGINAL | FIXED | RESOLVED |

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

## Completion Rules

- Re-derive every reply claim against the current tree before replying or resolving a thread.
- Reply on every resolvable thread before resolving it.
- Refresh review threads using GraphQL and confirm that no unresolved actionable thread remains.
- Commit this audit separately from the review fix.
