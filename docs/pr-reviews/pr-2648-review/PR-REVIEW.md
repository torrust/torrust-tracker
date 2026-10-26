---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
    - docs/issues/open/2645-2482-label-optional-and-self-dependency-edges/ISSUE.md
---

<!-- skill-link: process-pr-review -->

# PR #2648 Review Audit

Source: pull-request reviews and inline review threads for <https://github.com/torrust/torrust-tracker/pull/2648>.

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

Five reviews were submitted so far. The review of `copilot-pull-request-reviewer` (5642274652) read the head
whose subject is `docs(issues): [#2645] link the implementation pull request in the spec`; its body
repeats its single inline thread, F1, so the body adds no row. A second review of the same account
(5643781107) and the review of `da2ce7` (5644687426, verdict `CHANGES_REQUESTED`) read the head whose
subject is `docs(issues): [#2645] record the transitive feature fix and the PR 2648 review audit`.
The body of the second review repeats its thread, F2. The body of the `da2ce7` review repeats its five
inline threads and adds a long account of what was checked and what passed, which holds no further
actionable assertion, so it adds no row. `da2ce7` numbered his threads F2 to F6, which collide with this
record's numbering, so the table maps them: his F2 is F3 here, and so on, and each detail names the
reviewer's own ID. Two further reviews followed on the head whose subject is
`docs(pr-reviews): [#2645] record the second review round of PR 2648`: the review of
`copilot-pull-request-reviewer` (5648804588), which recommends approval with no open issue and adds no row,
and the review of `da2ce7` (5650797924, verdict `APPROVED`, with an `ACK` comment of the same head,
<https://github.com/torrust/torrust-tracker/pull/2648#issuecomment-6414098722>). The latter carries two inline
nit threads, which keep his IDs F8 and F9 here (no collision); its body repeats them and reports the checks, so
the body adds no row. The maintainer authorized pushing after that `ACK` to fix the two nits, through the
review session, so this record is not a GitHub artifact for that authorization.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2648-f1 | Copilot | Major | correctness | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-2648-f2 | Copilot | Major | correctness | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-2648-f3 | Human | Major | correctness | RE_RAISE_OF:F2 | FIXED | RESOLVED |
| F4 | review-finding:pr-2648-f4 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F5 | review-finding:pr-2648-f5 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F6 | review-finding:pr-2648-f6 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F7 | review-finding:pr-2648-f7 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F8 | review-finding:pr-2648-f8 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F9 | review-finding:pr-2648-f9 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - Resolve transitive feature activation before labelling an edge test-only

- PR number: 2648
- Source review ID: 5642274652
- Reviewer finding ID: R1
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2648#discussion_r4370184771>
- Concern: with `default = ["testing"]` and `testing = ["dep:c"]` the tool reported
  `optional (testing)` although a default build enables `c`, because it looked only at the features that
  list the dependency. The reviewer asked to follow local feature references with cycle protection, to hide
  the implicit dependency feature only from the displayed names, and to add tests for aliases and default
  chains.
- Solution: the finding was right. The tool now collects every feature that enables the dependency directly
  or through other features, stops on cycles, and hides only the implicit `c = ["dep:c"]` feature from the
  label, so a default chain shows `optional (default, testing)` and `optional (testing)` means only
  `testing` enables the dependency. Five tests were added (default chain, alias chain, the implicit feature
  through an alias, a cycle, and a feature that enables a feature of another dependency); four failed
  before the fix. The report legend, the specification's completion review, the evidence and the
  retrospective were updated.
- Current-tree verification: reproduced first with a unit test and with a scratch workspace
  (`default = ["testing"]`, `testing = ["y"]`), which printed `optional (testing)`; after the fix the
  scratch workspace prints `optional (default, testing)` and `cargo test -p workspace-coupling` passes
  (40 tests).
- Resolution reference: fix(workspace-coupling): [#2645] follow feature references when labelling an optional edge
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2648#discussion_r4371184732>

### F2 - Preserve explicit features that share the dependency key

- PR number: 2648
- Source review ID: 5643781107
- Reviewer finding ID: R2
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2648#discussion_r4371273086>
- Concern: the label dropped every feature named like the dependency, so with `c = ["dep:c", "extras"]`,
  `extras = []` and `testing = ["dep:c"]` it read `optional (testing)`, and a dependency renamed to
  `default` lost its default-build activation from the label. The reviewer asked to drop only the implicit
  singleton shape, to always keep `default`, and to add tests for explicit same-name features and renamed
  dependencies.
- Solution: the finding was right. The tool now drops a same-name feature only when it is exactly
  `["dep:key"]`, no other feature names `dep:key` and the name is not `default`. Three tests were added
  (explicit feature that enables more, explicit feature beside another `dep:key`, dependency renamed to
  `default`); all three failed before the fix. The spec's Known limits describe the rule.
- Current-tree verification: the three new tests failed on the previous head and pass now, and the V4 run
  of the evidence prints `optional (testing, y)` for the explicit case.
- Resolution reference: fix(workspace-coupling): [#2645] keep explicit features named like the dependency in the label
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2648#discussion_r4374811670>

### F3 - This removes every feature named like the dependency key, not only Cargo's implicit one

- PR number: 2648
- Source review ID: 5644687426
- Reviewer finding ID: F2
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2648#discussion_r4371941792>
- Concern: the same defect as F2, traced at the same head, raised by `da2ce7` after F2 and mapped to it
  before acting. He also pointed out that the code comment and the spec call the removal Cargo's implicit
  feature, which misleads a reader, and proposed to restore the exact-shape guard only below edition 2024,
  with a test built from 2024 metadata.
- Solution: fixed by the same commit as F2, which restores the exact-shape guard and tests the explicit
  same-name feature; the comment and the spec now state the rule. The edition limit was not adopted: Cargo
  1.99.1 at edition 2024 still adds the implicit feature (see F4), so the guard applies at every edition.
  The guard also requires that no other feature names `dep:key`, since Cargo adds no implicit feature then.
- Current-tree verification: same as F2.
- Resolution reference: fix(workspace-coupling): [#2645] keep explicit features named like the dependency in the label
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2648#discussion_r4374811828>

### F4 - The guidance describes pre-2024 manifests

- PR number: 2648
- Source review ID: 5644687426
- Reviewer finding ID: F3
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2648#discussion_r4371941799>
- Concern: the retrospective guidance and the evidence's "Cargo also accepts `testing = ["y"]` ... and then
  adds the implicit feature" describe pre-2024 manifests; the reviewer stated from the edition rules that
  edition 2024 creates no implicit feature and rejects the bare form, and asked to tell rows 3 to 7 to write
  `testing = ["dep:c"]` and to name the scratch workspace's edition.
- Solution: the premise was checked and does not hold for Cargo 1.99.1: a scratch workspace with
  `edition = "2024"` accepts `testing = ["y"]` and `cargo metadata` prints `y = ["dep:y"]`. The evidence
  now names the edition and the Cargo version and prints the edition per package (V4), and the retrospective
  says either syntax may appear. The advice to write only `dep:c` was not adopted, since the tool must
  handle both forms.
- Current-tree verification: V4 of the evidence, run on a clean checkout, prints `edition 2024` and the
  implicit feature for the packages `e` and `f`.
- Resolution reference: docs(issues): [#2645] state that edition 2024 still adds the implicit feature
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2648#discussion_r4374811995>

### F5 - "optional means no feature enables it" overstates the label

- PR number: 2648
- Source review ID: 5644687426
- Reviewer finding ID: F4
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2648#discussion_r4371941802>
- Concern: a bare `optional` is also printed when only a feature named after the dependency enables it, so
  the legend's "no feature enables it" is inaccurate; "no other feature" would be.
- Solution: the legend now says `optional` means no feature other than the dependency's own implicit one
  enables it.
- Current-tree verification: the V2 diff of the evidence shows the new legend line, and the report the tool
  prints for this repository contains it.
- Resolution reference: docs(workspace-coupling): [#2645] say what a bare optional label means in the report legend
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2648#discussion_r4374812137>

### F6 - "on the first head" is not accurate

- PR number: 2648
- Source review ID: 5644687426
- Reviewer finding ID: F5
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2648#discussion_r4371941806>
- Concern: this audit said review 5642274652 was on the first head, but it read the head that records
  `related-pr: 2648`, committed after the pull request opened.
- Solution: the audit names that head by its subject.
- Current-tree verification: the Findings introduction of this record names
  `docs(issues): [#2645] link the implementation pull request in the spec`.
- Resolution reference: docs(pr-reviews): [#2645] name the head the first review read by its subject
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2648#discussion_r4374812290>

### F7 - The environment line lags the runs the evidence records

- PR number: 2648
- Source review ID: 5644687426
- Reviewer finding ID: F6
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2648#discussion_r4371941815>
- Concern: the evidence's environment line said the runs took place from 08:09 to 08:10, but the file also
  records runs at 08:12:15 and 08:34:49, and the V2 legend lines came from a later commit; the record did not
  say which tree each run used.
- Solution: the environment section lists each run with its UTC time and tree (the working tree before the
  commit of the change, or a commit), and V4 re-runs every scratch case on a clean checkout, with the V2 diff
  re-taken at the same time.
- Current-tree verification: the environment section and V4 of the evidence.
- Resolution reference: docs(issues): [#2645] state the tree and time of every manual verification run
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2648#discussion_r4374812424>

### F8 - Append an event-time correction for the resolution claim

- PR number: 2648
- Source review ID: 5650797924
- Reviewer finding ID: F8
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2648#discussion_r4376355235>
- Concern: the 07:22 UTC Processing Log entry says the threads were resolved after the record was committed and
  pushed, but the commit carrying it is dated 07:23:00 UTC, so that sequence cannot belong to 07:22. The
  reviewer asked for an appended correction, using a measured resolution time if available, otherwise stating
  the final state without claiming the chronology.
- Solution: the finding was right. A correction entry was appended; the earlier entry is kept. No resolution
  time was measured, so the correction says that and gives the observed final state.
- Current-tree verification: the Processing Log of this record holds the 07:22 entry unchanged and the
  correction entry after it.
- Resolution reference: docs(pr-reviews): [#2645] correct the chronology of the resolution claim in the audit log
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2648#discussion_r4377291288>

### F9 - Restrict the pre-commit explanation to the runs it fits

- PR number: 2648
- Source review ID: 5650797924
- Reviewer finding ID: F9
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2648#discussion_r4376355244>
- Concern: the evidence said the runs of 2026-10-25 happened before the commits of the changes they tested, but
  V1 ran at 08:10:01 UTC, after its implementation commit (08:08:46 UTC); only the second and third runs
  precede their fixes.
- Solution: the finding was right (commit times read from `git log`). The note now covers only the second and
  third runs and says that V1 ran after its commit.
- Current-tree verification: `git log` shows the feature commit at 2026-10-25T08:08:46Z; the environment
  section of the evidence carries the restricted note.
- Resolution reference: docs(issues): [#2645] restrict the working-tree note to the runs that preceded their commits
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2648#discussion_r4377291671>

## Processing Log

- 2026-10-25 14:36 UTC - The review of `copilot-pull-request-reviewer` arrived with one inline thread, F1. It was
  reproduced against the tree and confirmed. Fixed in its own signed commit, then replied on the thread
  after pushing the fix, and resolved the thread.
- 2026-10-26 07:22 UTC - The second review of Copilot (thread F2) and the review of `da2ce7` (five threads,
  F3 to F7 here) arrived. All six were checked against the tree; F2 and F3 are the same defect, and the
  premise of F4 (edition 2024 creates no implicit feature) was tested and does not hold for Cargo 1.99.1,
  so that thread was answered with the evidence and the documentation fix, not the proposed code limit.
  Fixed each independent finding in its own signed commit (F2 and F3 together), replied on the six threads
  after pushing, and resolved them after this record was committed and pushed.
- 2026-10-26 12:37 UTC - Correction to the 07:22 UTC entry, which said that the threads were resolved after this
  record was committed and pushed: the commit that carries that entry is dated 07:23:00 UTC, so the sequence
  cannot belong to 07:22. The record was committed at 07:23:00 UTC, then pushed, and the six threads were resolved
  after the push; the times of the pushes and of the resolutions were not measured. The final resolved state of
  the threads was observed afterwards, in a refresh of the thread data, without a measured event time.
- 2026-10-26 12:40 UTC - The `ACK` of `da2ce7` on the head dated 07:23:00 UTC arrived with two non-blocking nit
  threads, F8 and F9, and an approval review of Copilot with no finding. Both nits were checked against the tree
  and `git log` and confirmed. The maintainer authorized pushing after the `ACK`. Fixed each in its own signed
  commit and pushed; replied on both threads after the push. The threads are resolved after this record is
  committed and pushed, and da2ce7 is asked to review again, since the `ACK` covered the previous head.

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
