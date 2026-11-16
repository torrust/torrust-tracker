---
pr-number: 2891
pr-url: https://github.com/torrust/torrust-tracker/pull/2891
last-updated-utc: "2026-11-16 22:26"
---

<!-- skill-link: process-pr-review -->

# PR #2891 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/2891.

<!-- Copied verbatim into each audit record after placeholder replacement. -->

## Ownership

The PR author owns this tracked audit record. Reviewers, including repository review agents,
deliver findings through GitHub and have no repository-artifact obligation.

- Post-merge workflow approval: N/A

<!-- Copied verbatim into each audit record. -->

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

Round 1: da2ce7 review 5925551139 (APPROVED, F1 to F6) and Copilot review 5925597419 (four findings, all with reviewer IDs `R1` or `R2`; they are audit findings F7 to F10, and F9 re-raises F3). Round 2: da2ce7 review 5928830303 (COMMENTED) with findings F7 to F9, whose IDs collide with this audit's, so they are audit findings F11 to F13.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2891-f1 | Human | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-2891-f2 | Human | Minor | link-integrity | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-2891-f3 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F4 | review-finding:pr-2891-f4 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F5 | review-finding:pr-2891-f5 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F6 | review-finding:pr-2891-f6 | Human | Nit | metadata | ORIGINAL | FIXED | RESOLVED |
| F7 | review-finding:pr-2891-f7 | Copilot | Minor | testing | ORIGINAL | FIXED | RESOLVED |
| F8 | review-finding:pr-2891-f8 | Copilot | Minor | testing | ORIGINAL | FIXED | RESOLVED |
| F9 | review-finding:pr-2891-f9 | Copilot | Minor | documentation | RE_RAISE_OF:F3 | FIXED | RESOLVED |
| F10 | review-finding:pr-2891-f10 | Copilot | Major | correctness | ORIGINAL | FIXED | RESOLVED |
| F11 | review-finding:pr-2891-f11 | Human | Minor | link-integrity | ORIGINAL | FIXED | RESOLVED |
| F12 | review-finding:pr-2891-f12 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F13 | review-finding:pr-2891-f13 | Human | Nit | formatting | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - CI runs the fixed-port test on the self-hosted runner too

- PR number: 2891
- Source review ID: 5925551139
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2891#discussion_r4590004425
- Concern: D1 claimed CI runs the Rust tests only on GitHub-hosted runners, but the `Containerfile` `test` stage runs the whole suite with `cargo nextest run` and both self-hosted jobs build it; the reviewer asked to argue the blast radius from the build's network namespace and to add the image build to P2's "who else" column.
- Solution: D1's Background lists where the collision can happen: development machines (today), the self-hosted image builds (each BuildKit `RUN` step has its own network namespace, so concurrent builds do not collide), and GitHub-hosted runners. The inventory's Where Tests Run table and P2's "who else" column say the same (commit `docs(issues): [#2394] address the first review round in the inventory`).
- Current-tree verification: `grep -n 'network' Containerfile .github/workflows/container.yaml .github/workflows/testing.yaml` finds no `--network=host`; `Containerfile` line 231 runs `cargo nextest run`; D1 Background and inventory P2 carry the new text.
- Resolution reference: `docs(issues): [#2394] complete the D1 and D2 drafts and record their reproductions`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2891#discussion_r4592335452

### F2 - Drafts cite the pre-move skill paths

- PR number: 2891
- Source review ID: 5925551139
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2891#discussion_r4590004438
- Concern: Both drafts cited `.github/skills/dev/...` skill paths that `develop` moved to `.claude/skills/<name>/` (#2866).
- Solution: Merged `develop` and pointed D1, D2 and D3 at `.claude/skills/fix-bug/SKILL.md`, `.claude/skills/create-issue/SKILL.md` and `.claude/skills/write-unit-test/SKILL.md`.
- Current-tree verification: `grep -rn '.github/skills' docs/issues/drafts/2392-*/` prints nothing.
- Resolution reference: `docs(issues): [#2394] complete the D1 and D2 drafts and record their reproductions`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2891#discussion_r4592336336

### F3 - Drafts omit sections the create-issue skill requires

- PR number: 2891
- Source review ID: 5925551139
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2891#discussion_r4590004444
- Concern: The drafts lacked the progressive test-development loop with the write-unit-test design review, the deadline statement and post-vertical-slice checkpoint, the toolchain-qualified evidence policy and the retrospective conditions.
- Solution: Added a Test Development Loop section, deadline and design-review checkpoint lines in Design and Ownership Review, the evidence policy and the retrospective and agent-review-report conditions to D1 and D2; D3 carries them from the start.
- Current-tree verification: `grep -c 'Test Development Loop\|Design-review checkpoint\|design-review checkpoint\|Evidence policy' docs/issues/drafts/2392-*/ISSUE.md` finds them in all three drafts.
- Resolution reference: `docs(issues): [#2394] complete the D1 and D2 drafts and record their reproductions`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2891#discussion_r4592337465

### F4 - P8's enumeration does not match the bytes

- PR number: 2891
- Source review ID: 5925551139
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2891#discussion_r4590004454
- Concern: P8 named port ranges and a path that did not match the code (`41001`-`41017`, `42158`, and a non-existent `axum-http-server/src/routes.rs`).
- Solution: Corrected P8's ports and paths; also reran the searches after merging `develop` and added the one new test resource (`tests/http_health_check.rs`) to P5.
- Current-tree verification: `grep -oE '\b4[12][0-9]{3}\b' packages/configuration/src/lib.rs src/bootstrap/config.rs | sort -u` gives `41001`-`41017` and `42151`-`42158`, as P8 now says.
- Resolution reference: `docs(issues): [#2394] address the first review round in the inventory`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2891#discussion_r4592338294

### F5 - "never remove them" contradicts the PEM row

- PR number: 2891
- Source review ID: 5925551139
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2891#discussion_r4590004464
- Concern: D2 said the helpers never remove their files, while its table says the PEM files are removed when the test passes.
- Solution: D2 now says "never remove them, or only when the test passes".
- Current-tree verification: Read D2's Background in the current tree.
- Resolution reference: `docs(issues): [#2394] complete the D1 and D2 drafts and record their reproductions`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2891#discussion_r4592339379

### F6 - Spec status still planned

- PR number: 2891
- Source review ID: 5925551139
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2891#discussion_r4590004472
- Concern: With T1-T5 done and T6 in progress, the #2394 spec's `status: planned` no longer described the issue.
- Solution: Set `status: in-progress`.
- Current-tree verification: `grep -n '^status:' docs/issues/open/2394-2392-audit-shared-test-resources/ISSUE.md` prints `status: in-progress`.
- Resolution reference: `docs(issues): [#2394] address the first review round in the inventory`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2891#discussion_r4592340340

### F7 - Record the paired-run reproduction evidence

- PR number: 2891
- Source review ID: 5925597419
- Reviewer finding ID: R1
- Source URL: https://github.com/torrust/torrust-tracker/pull/2891#discussion_r4590039738
- Concern: D1 reported 20 failures in 40 runs but kept no commands or output, so the later recheck could not repeat the baseline.
- Solution: Reran the reproduction on the merge of `develop` `0373f05f8` and recorded the build command, the held-port run, the 20 paired runs, the toolchain and the `AddrInUse` output in D1's `manual-verification-evidence.md`; V1 is DONE.
- Current-tree verification: The evidence file exists; its tallies (11 plus 9 failed pairs) add up to 20.
- Resolution reference: `docs(issues): [#2394] complete the D1 and D2 drafts and record their reproductions`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2891#discussion_r4592341341

### F8 - Record a controlled leak reproduction baseline

- PR number: 2891
- Source review ID: 5925597419
- Reviewer finding ID: R2
- Source URL: https://github.com/torrust/torrust-tracker/pull/2891#discussion_r4590039807
- Concern: D2's baseline was the accumulated `/tmp` totals, which do not say what one run leaves behind.
- Solution: Ran the pre-push test command with a fresh `TMPDIR` (4093 tests passed): one run left 301 `data_*.db` and 1 `test_config_*.toml` (15 MiB). Recorded in D2's `manual-verification-evidence.md`; the `/tmp` totals stay only as a labeled background measurement.
- Current-tree verification: The evidence file exists and records the commands, toolchain and counts.
- Resolution reference: `docs(issues): [#2394] complete the D1 and D2 drafts and record their reproductions`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2891#discussion_r4592342155

### F9 - Add test-design reviews between implementation increments

- PR number: 2891
- Source review ID: 5925597419
- Reviewer finding ID: R1
- Source URL: https://github.com/torrust/torrust-tracker/pull/2891#discussion_r4590039850
- Concern: D2's T3-T5 add tests but required no recorded test-design review between increments.
- Solution: Fixed with F3: D2's Test Development Loop requires the recorded prose-first Arrange-Act-Assert review after each passing increment, and T3-T5 each name their review before the next task.
- Current-tree verification: `grep -n '^| T[3-5] ' docs/issues/drafts/2392-remove-leaked-test-temporary-files/ISSUE.md` shows the review in each row.
- Resolution reference: `docs(issues): [#2394] complete the D1 and D2 drafts and record their reproductions`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2891#discussion_r4592342987

### F10 - Clean up containers when Compose up fails

- PR number: 2891
- Source review ID: 5925597419
- Reviewer finding ID: R1
- Source URL: https://github.com/torrust/torrust-tracker/pull/2891#discussion_r4590039890
- Concern: `DockerCompose::up` creates its `down --volumes` guard only on success, so a failed `up` leaves the run's containers, network and volumes on the daemon; C5 marked the whole lifecycle isolated.
- Solution: Confirmed in `src/console/ci/compose.rs:73-93` and `services_setup.rs:47`. The maintainer classified it as a new problem with its own draft: the inventory splits it out as C9 (commit `docs(issues): [#2394] address the first review round in the inventory`; totals 38 findings, 3 new problems) and draft D3 tracks the fix. The plain E2E runner was checked and does not leak.
- Current-tree verification: Inventory row C9 and `docs/issues/drafts/2392-compose-cleanup-after-failed-start/ISSUE.md` exist; `TrackerContainer`'s drop (`src/console/ci/e2e/tracker_container.rs:17-26`) removes its container by name.
- Resolution reference: `docs(issues): [#2394] draft the subissue for Compose cleanup after a failed start`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2891#discussion_r4592343775

### F11 - Inventory cites a PR-branch merge commit

- PR number: 2891
- Source review ID: 5928830303
- Reviewer finding ID: F7
- Source URL: https://github.com/torrust/torrust-tracker/pull/2891#discussion_r4592417321
- Concern: The inventory cited the branch's merge commit `da4d1a3cb`, which a rebase would rewrite.
- Solution: The inventory now cites the merged `develop` commit `0373f05f8`.
- Current-tree verification: The `open-pull-request` commit-id check prints nothing at this head.
- Resolution reference: `docs(issues): [#2394] cite the merged develop commit instead of the branch merge in the inventory`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2891#discussion_r4592488048

### F12 - No review audit record

- PR number: 2891
- Source review ID: 5928830303
- Reviewer finding ID: F8
- Source URL: https://github.com/torrust/torrust-tracker/pull/2891#discussion_r4592417327
- Concern: All threads were answered and resolved, but no `docs/pr-reviews/pr-2891-review/PR-REVIEW.md` existed, and the replies cite branch commit ids.
- Solution: Created this record from `docs/templates/PR-REVIEW-TEMPLATE.md`, logged the late start, and mapped the branch ids the round-1 replies cite to commit subjects in the Processing Log.
- Current-tree verification: `validate-audit-record.py --pr-number 2891 --base torrust/develop` (its `gh` calls run through `gh-bot`) exits 0.
- Resolution reference: `docs(pr-reviews): [#2394] record the review audit of PR 2891`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2891#discussion_r4592488567

### F13 - Evidence files lack the template's shape

- PR number: 2891
- Source review ID: 5928830303
- Reviewer finding ID: F9
- Source URL: https://github.com/torrust/torrust-tracker/pull/2891#discussion_r4592417335
- Concern: The D1 and D2 evidence files did not keep the shape of `docs/templates/MANUAL-VERIFICATION-EVIDENCE.md`.
- Solution: Both now have Preserving Verification Artifacts, the V1 Goal/Initial state/Status lines, Steps Performed, Observed Result, Conclusion and Failures and Follow-up, with the same content.
- Current-tree verification: Read both files in the current tree.
- Resolution reference: `docs(issues): [#2394] give the D1 and D2 evidence files the template's shape`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2891#discussion_r4592489092

## Processing Log

- 2026-11-16 22:13 UTC - Replied to F1 to F10 (replies created from 22:12:35Z to 22:13:16Z) and resolved their threads after pushing the round-1 fixes, before this record existed: a late start of the audit. The replies cite branch commit ids; they map to these commit subjects: `abc52e336` = `docs(issues): [#2394] address the first review round in the inventory`, `c27bd08f6` = `docs(issues): [#2394] complete the D1 and D2 drafts and record their reproductions`, `2938b08db` = `docs(issues): [#2394] draft the subissue for Compose cleanup after a failed start`, and `da4d1a3cb` = the merge of `develop` at `0373f05f8` into the branch.
- 2026-11-16 22:26 UTC - Started audit: created this record from `docs/templates/PR-REVIEW-TEMPLATE.md` after da2ce7's round-2 review 5928830303 (F8 asked for it). Normalized round 1 (F1 to F10) and round 2 (F11 to F13), and committed the F11 and F13 fixes.

<!-- Copied verbatim into each audit record. -->

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
