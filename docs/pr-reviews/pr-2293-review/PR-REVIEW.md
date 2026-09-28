---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
    - docs/issues/open/2347-2003-triage-post-merge-review-findings/ISSUE.md
    - docs/issues/closed/2222-1347-package-coverage-regression-ci/ISSUE.md
---

<!-- skill-link: process-pr-review -->

# PR #2293 Review Audit

Source: pull-request reviews and inline review threads for
<https://github.com/torrust/torrust-tracker/pull/2293>.

PR #2293 merged at 2026-09-22 12:33 UTC. Review 5283816543 was submitted after the merge, at
2026-09-22 21:09 UTC, and labels its findings post-merge. #2347 tracks them, and this record is
created on its approved follow-up branch.

## Ownership

The PR author owns this tracked audit record. Reviewers, including repository review agents,
deliver findings through GitHub and have no repository-artifact obligation.

- Post-merge workflow approval: <https://github.com/torrust/torrust-tracker/issues/2347#issuecomment-5865646588>

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

Audit IDs `F1`-`F8` correspond to the reviewer's `loop F1`-`loop F8` from review 5283816543. No
earlier audit existed, so no ID collides. Findings fixed in follow-up PR #2363 stay `FOLLOW_UP`
and `OPEN` until it merges. The close-out then records them as `FIXED`, citing their commit
subjects. `NO_ACTION` threads are resolved in the close-out. F3 was already fixed on `develop` by
PR #2357, so it is `FIXED` and `RESOLVED` here. That routing and the F5 routing are recorded in the
approval's amendments:
<https://github.com/torrust/torrust-tracker/issues/2347#issuecomment-5867552349> and
<https://github.com/torrust/torrust-tracker/issues/2347#issuecomment-5868016556>.

Copilot review 5275981836 left eight inline threads before the merge. They were resolved without
replies, and they get no rows here, by maintainer decision (approval record above). Review
5283816543 accounts for them: two fixes landed, one was correctly declined (the workspace lint
denies discarding a `#[must_use]` result, so the code keeps `drop(...)`), and the requests that did
not land are F2, F3, and F4 below.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | `review-finding:pr-2293-f1` | Human | Minor | correctness | ORIGINAL | FOLLOW_UP | OPEN |
| F2 | `review-finding:pr-2293-f2` | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F3 | `review-finding:pr-2293-f3` | Human | Minor | formatting | ORIGINAL | FIXED | RESOLVED |
| F4 | `review-finding:pr-2293-f4` | Human | Minor | link-integrity | ORIGINAL | FIXED | RESOLVED |
| F5 | `review-finding:pr-2293-f5` | Human | Minor | testing | ORIGINAL | FOLLOW_UP | OPEN |
| F6 | `review-finding:pr-2293-f6` | Human | Suggestion | security | ORIGINAL | NO_ACTION | RESOLVED |
| F7 | `review-finding:pr-2293-f7` | Human | Suggestion | metadata | ORIGINAL | FIXED | RESOLVED |
| F8 | `review-finding:pr-2293-f8` | Human | Nit | metadata | ORIGINAL | NO_ACTION | RESOLVED |

## Finding Details

### F1 - The report-only check fails when the discovery job fails

- PR number: 2293
- Source review ID: 5283816543
- Reviewer finding ID: loop F1
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2293#discussion_r4076532667>
- Concern: `package-coverage-summary` runs under `always()` with no `continue-on-error`. When
  discovery fails, it writes an empty discovery file, and `package-coverage-check summary` fails
  with a JSON parse error, so the report-only check goes red with a misleading message.
- Solution: follow-up in bug #2361 (EPIC #1347). It decides whether the summary reports that
  discovery was unavailable or fails with a message naming the cause, and adds a regression test.
  The change is to a tool and a workflow, which is outside #2347's documentation scope.
- Current-tree verification: on `develop` at `478516cf`, `cargo run -q -p package-coverage-check --
  summary <empty discovery file> <missing directory>` (stable Rust 1.98.1) exits `1` with
  `invalid JSON: EOF while parsing a value at line 1 column 0`, and a valid empty discovery exits
  `0`. This is recorded in #2361's `manual-verification-evidence.md` V0.
- Resolution reference: <https://github.com/torrust/torrust-tracker/pull/2293#discussion_r4121101850>
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2293#discussion_r4121101850>

### F2 - This names a workflow that does not exist

- PR number: 2293
- Source review ID: 5283816543
- Reviewer finding ID: loop F2
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2293#discussion_r4076532671>
- Concern: `docs/testing.md` names `Generate Coverage Report (PR)`, but the workflow is
  `Generate Coverage Reports (PR)`.
- Solution: `docs(testing): [#2347] name the PR coverage workflow correctly`.
- Current-tree verification: `.github/workflows/generate_coverage_pr.yaml` begins
  `name: Generate Coverage Reports (PR)`. On `develop` at `478516cf`, `docs/testing.md` reads
  ``The `Generate Coverage Report (PR)` workflow``.
- Resolution reference: `docs(testing): [#2347] name the PR coverage workflow correctly`
- Follow-up PR URL: <https://github.com/torrust/torrust-tracker/pull/2363>
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2293#discussion_r4124957650>

### F3 - The `semantic-links.related-artifacts` list is mis-indented

- PR number: 2293
- Source review ID: 5283816543
- Reviewer finding ID: loop F3
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2293#discussion_r4076532677>
- Concern: the #2222 refactor plan's `related-artifacts` entries were indented 4, 3, 4, 6, and 6
  spaces, so they did not parse as one list.
- Solution: already fixed on `develop` by PR #2357 (#2281). #2347's own identical fix became
  empty when the branch was rebased.
- Current-tree verification: on `develop` at `478516cf`, `yaml.safe_load` on the plan's
  frontmatter raised `ParserError while parsing a block mapping`. On `develop` at `8a953724`, it
  returns the five entries.
- Resolution reference: `docs(issues): [#2281] fix malformed frontmatter indentation in closed records`
- Follow-up PR URL: <https://github.com/torrust/torrust-tracker/pull/2357>
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2293#discussion_r4121102377>

### F4 - This reproduction instruction cannot be followed from the merged tree

- PR number: 2293
- Source review ID: 5283816543
- Reviewer finding ID: loop F4
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2293#discussion_r4076532684>
- Concern: the evidence says to reproduce by checking out `88822c5c` and `aa026584`, but neither
  is reachable in `torrust/torrust-tracker`, and the recorded commands name the pre-rename
  package.
- Solution: `docs(issues): [#2347] map #2222 evidence commits to merged subjects` appends a
  correction that maps both ids to merged commit subjects.
- Current-tree verification: both objects exist only as dangling objects in a local clone
  (`git branch -a --contains` is empty), and GitHub returns `No commit found for SHA` for both.
  `git diff` against `feat(ci): add package coverage discovery tool` and
  `feat(ci): add package coverage regression report` shows the `package-coverage-regression`
  package byte-identical; only `Cargo.lock` differs.
- Resolution reference: `docs(issues): [#2347] map #2222 evidence commits to merged subjects`
- Follow-up PR URL: <https://github.com/torrust/torrust-tracker/pull/2363>
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2293#discussion_r4124957952>

### F5 - The feature's central path was never verified on a hosted runner

- PR number: 2293
- Source review ID: 5283816543
- Reviewer finding ID: loop F5
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2293#discussion_r4076532690>
- Concern: M2 and M3 are `TODO`, but the #2222 folder is archived under `closed/`. No recorded
  run executed `compare` on a hosted runner, which leaves the open risk that the source prefix
  might not match the report's file keys.
- Solution: follow-up in the existing #2301 ("Review package coverage rollout"), whose scope
  already includes hosted evidence and reconciling M2 and M3. Its spec now records the hosted run
  below. T2 missed this owner at first; the amendment in the Findings intro corrects the routing.
- Current-tree verification: run <https://github.com/torrust/torrust-tracker/actions/runs/36305549957>
  (PR #2351, `pull_request` event) produced comparison artifacts with non-zero counts, for
  example `torrust-tracker-udp-server` base `5250/5378` and head `5524/5651`.
- Resolution reference: <https://github.com/torrust/torrust-tracker/pull/2293#discussion_r4121103023>
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2293#discussion_r4121103023>

### F6 - No `permissions:` block, against the guidance this PR adds

- PR number: 2293
- Source review ID: 5283816543
- Reviewer finding ID: loop F6
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2293#discussion_r4076532699>
- Concern: `generate_coverage_pr.yaml` declares no `permissions:`, although the same PR adds a
  least-privilege rule.
- Solution: no action, approved by the maintainer. PRs to `develop` come from forks, and fork
  `pull_request` runs get a read-only token. The gap predates this PR, and 11 of 15 workflows
  share it.
- Current-tree verification: `grep -c 'permissions:' .github/workflows/generate_coverage_pr.yaml`
  prints `0`.
- Resolution reference: <https://github.com/torrust/torrust-tracker/pull/2293#discussion_r4121103316>
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2293#discussion_r4121103316>

### F7 - The new skill ships without its semantic skill-link markers

- PR number: 2293
- Source review ID: 5283816543
- Reviewer finding ID: loop F7
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2293#discussion_r4076532707>
- Concern: nothing carries a `skill-link: implement-workflow` marker, and the skill has no
  `Skill Links` section.
- Solution: `docs(skills): [#2347] link implement-workflow to its workflow artifacts` adds the
  marker to the workflow (a comment) and to `.github/workflows/AGENTS.md` (frontmatter), and a
  `Skill Links` section to the skill.
- Current-tree verification: on `develop` at `478516cf`, `git grep 'skill-link: implement-workflow'`
  returns nothing.
- Resolution reference: `docs(skills): [#2347] link implement-workflow to its workflow artifacts`
- Follow-up PR URL: <https://github.com/torrust/torrust-tracker/pull/2363>
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2293#discussion_r4124958211>

### F8 - Stale `last-updated-utc` in both issue-folder docs (not live on `develop`)

- PR number: 2293
- Source review ID: 5283816543
- Reviewer finding ID: loop F8
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2293#discussion_r4076532713>
- Concern: both documents' stamps predated their last edit at the PR head.
- Solution: no action, approved by the maintainer; the finding is not live.
- Current-tree verification: on `develop` at `478516cf`, every `last-updated-utc` in the #2222
  folder reads `2026-09-22 16:40`, set by `chore(issues): archive closed issue 2222 spec`.
- Resolution reference: <https://github.com/torrust/torrust-tracker/pull/2293#discussion_r4121103835>
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2293#discussion_r4121103835>

## Processing Log

- 2026-09-26 12:03 UTC - The maintainer's approval to track the late review in #2347 was recorded (<https://github.com/torrust/torrust-tracker/issues/2347#issuecomment-5846100762>).
- 2026-09-26 12:04 UTC - Posted a tracking reply naming #2347 on all eight threads (the first at `created_at` 12:04:58Z), with the disposition pending.
- 2026-09-28 07:44 UTC - The maintainer approved the dispositions (Ownership section).
- 2026-09-28 09:51 UTC - Amendment: F5 goes to the existing #2301, and F1 goes to bug #2361.
- 2026-09-28 10:20 UTC - Amendment: F3 was fixed upstream by PR #2357. The branch was rebased onto `develop` at `8a953724`, and #2347's own F3 commit was dropped as already upstream.
- 2026-09-28 10:24 UTC - Follow-up PR #2363 opened as a draft with the F2, F4, and F7 fix commits.
- 2026-09-28 10:29 UTC - Posted a disposition reply on each of the eight threads (`created_at` 10:29:08Z-10:29:21Z).
- 2026-09-28 10:32 UTC - Resolved the F3 thread; the other seven stay unresolved until the close-out, or until their owning issue's fix merges.
- 2026-09-28 10:38 UTC - Started this audit. Copilot review 5275981836 (eight threads, resolved before merge without replies) is recorded as context only, without rows, as the maintainer approved.
- 2026-09-28 11:48 UTC - Correction: the 2026-09-26 12:04 UTC entry is stamped at the first tracking reply; the last was posted at 12:05:10Z.
- 2026-09-28 17:15 UTC - #2347 T7 close-out after PR #2363 merged into `develop` (17:00:27Z). josecelano posted the T7 replies on F2, F4, and F7 (`created_at` 17:07:44Z-17:07:48Z), then resolved those threads and the F6 and F8 `NO_ACTION` threads. This close-out records F2, F4, and F7 as `FIXED`/`RESOLVED`, each citing its fixing commit with its T7 reply as Reply URL, and F6 and F8 as `RESOLVED`, keeping the disposition replies. F1 stays `FOLLOW_UP`/`OPEN` (#2361); F5 stays `FOLLOW_UP`/`OPEN` (#2301).

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
