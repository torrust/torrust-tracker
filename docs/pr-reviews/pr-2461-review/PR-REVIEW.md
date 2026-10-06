---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
    - docs/issues/open/2458-inject-udp-cookie-cipher/ISSUE.md
    - docs/issues/open/2458-inject-udp-cookie-cipher/manual-verification-evidence.md
---

<!-- skill-link: process-pr-review -->

# PR #2461 Review Audit

Source: pull-request reviews and inline review threads for <https://github.com/torrust/torrust-tracker/pull/2461>,
and for the closed PR it replaced, <https://github.com/torrust/torrust-tracker/pull/2460>.

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
- Category: `link-integrity`, `formatting`, `metadata`, `testing`, `correctness`, `documentation`,
  `maintainability`, `security`, `other`
- An outdated thread whose concern was fixed is `FIXED`/`RESOLVED`, even when GitHub marks the
  original thread outdated after the push. For in-PR feedback, use `NO_ACTION`/`SUPERSEDED` only
  for a duplicate, superseded, or no-change concern. A post-merge `NO_ACTION` requires maintainer
  approval to decline the follow-up work.

## Findings

PR #2460 was closed without merging at 2026-10-06 16:26 UTC. Its source branch had been renamed to
add the required `-spec` suffix, and GitHub kept the PR on the old branch name, so PR #2461 replaced
it with the same commits. Copilot review 5431658413 arrived on the closed PR at 16:36 UTC. Its
findings apply to the same files in PR #2461. Following the process-pr-review skill's
closed-and-superseded section, they are fixed and audited here, with `PR number` 2460 and
`review-finding:pr-2460-...` references.

Review 5431658413 supplied the finding IDs `F8` and `F1` with `[Minor]` and `[Major]` severities;
both IDs are kept. Its body summarizes those two threads and adds no other assertion. Copilot review
5431782940 on PR #2461 (16:48 UTC) supplied no finding IDs, so its four comments take `F2` to `F5`
in source order, skipping the IDs already used. Their severities are inferred from its overview
badges: Medium as Minor, and Low as Suggestion for the "Consider quoting" comment. No finding
re-raises another.

Human review 5431860814 by `da2ce7` (`CHANGES_REQUESTED`, 16:55 UTC) was computed at the head
before the F1 to F5 and F8 fixes. It supplied the finding IDs `F1` to `F11` with bracketed
severities. Its body summarizes the same eleven threads and adds no other assertion. IDs that
do not collide with this audit are kept (`F6`, `F7`, `F9`, `F10`, `F11`). The colliding IDs are
reassigned in source order to `F12` (reviewer `F1`), `F13` (`F2`), `F14` (`F3`), `F15` (`F4`),
`F16` (`F5`), and `F17` (`F8`). Five of these findings repeat a concern that Copilot raised
first:

- F12 repeats F1, and adds a disposable-script record in the spec.
- F14 repeats F8.
- F16 repeats F2.
- F6 repeats F3, and adds planned evidence numbering.
- F7 repeats F4.

F14, F16 and F7 needed no further change, so they are `NO_ACTION`/`SUPERSEDED`. F12 and F6 asked
for more, which was fixed. Review bodies 5432546687, 5432547029, 5432547318 and 5432547627 are
the empty containers GitHub created for this audit's replies.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F8 | `review-finding:pr-2460-f8` | Copilot | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F1 | `review-finding:pr-2460-f1` | Copilot | Major | testing | ORIGINAL | FIXED | RESOLVED |
| F2 | `review-finding:pr-2461-f2` | Copilot | Minor (inferred) | documentation | ORIGINAL | FIXED | RESOLVED |
| F3 | `review-finding:pr-2461-f3` | Copilot | Minor (inferred) | link-integrity | ORIGINAL | FIXED | RESOLVED |
| F4 | `review-finding:pr-2461-f4` | Copilot | Minor (inferred) | formatting | ORIGINAL | FIXED | RESOLVED |
| F5 | `review-finding:pr-2461-f5` | Copilot | Suggestion (inferred) | metadata | ORIGINAL | FIXED | RESOLVED |
| F12 | `review-finding:pr-2461-f12` | Human | Major | testing | RE_RAISE_OF:F1 | FIXED | RESOLVED |
| F13 | `review-finding:pr-2461-f13` | Human | Major | testing | ORIGINAL | FIXED | RESOLVED |
| F14 | `review-finding:pr-2461-f14` | Human | Major | documentation | RE_RAISE_OF:F8 | NO_ACTION | SUPERSEDED |
| F15 | `review-finding:pr-2461-f15` | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F16 | `review-finding:pr-2461-f16` | Human | Minor | documentation | RE_RAISE_OF:F2 | NO_ACTION | SUPERSEDED |
| F6 | `review-finding:pr-2461-f6` | Human | Minor | link-integrity | RE_RAISE_OF:F3 | FIXED | RESOLVED |
| F7 | `review-finding:pr-2461-f7` | Human | Minor | formatting | RE_RAISE_OF:F4 | NO_ACTION | SUPERSEDED |
| F17 | `review-finding:pr-2461-f17` | Human | Minor | security | ORIGINAL | FIXED | RESOLVED |
| F9 | `review-finding:pr-2461-f9` | Human | Suggestion | documentation | ORIGINAL | FIXED | RESOLVED |
| F10 | `review-finding:pr-2461-f10` | Human | Suggestion | documentation | ORIGINAL | FIXED | RESOLVED |
| F11 | `review-finding:pr-2461-f11` | Human | Suggestion | metadata | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F8 - The spec misstated when `check_seed()` was added

- PR number: 2460
- Source review ID: 5431658413
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2460#discussion_r4197957445>
- Concern: The spec said commit `e3562f069` added the cipher and "kept" an earlier seed check that
  had "made sense" before. In fact that commit added the cipher and `check_seed()` together, so the
  defect is that the new safeguard checked the old seed instead of the new cipher.
- Solution: Rewrote Background item 3 and the References entry. Before `e3562f069`, cookies hashed
  the seed and nothing checked it at startup. That commit replaced the hash with the cipher and, in
  the same patch, added `check_seed()`, which checks the now-unused seed. The other lines the
  reviewer cited only mention `check_seed()` and make no history claim, so they are unchanged.
- Current-tree verification: `git show e3562f069` adds `fn check_seed()` and
  `RANDOM_CIPHER_BLOWFISH`; `git log -S check_seed` finds no earlier commit;
  `git show e3562f069^:src/servers/udp/connection_cookie.rs` shows the seed-hash cookie. The spec
  no longer contains "made sense" or "kept the".
- Resolution reference: `docs(issues): correct #2458 cookie-cipher history`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2460#discussion_r4198706123>

### F1 - The disposable reproducer source was not preserved

- PR number: 2460
- Source review ID: 5431658413
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2460#discussion_r4197957531>
- Concern: Evidence V1 relied on the removed `forge_zeroed_cookie` example but kept only its
  command and output. The fix-bug skill requires temporary reproduction code to be recorded
  verbatim, otherwise reviewers cannot audit or repeat the forgery.
- Solution: Added a "Temporary Code (Verbatim)" section to V1 with the alias mutation diff and the
  full example. Recreated that exact code and ran it again. Added a control run with the alias
  restored to show that the acceptance comes from the mutation. Reverted both temporary changes.
- Current-tree verification: The 2026-10-06 17:06 UTC rerun printed
  `forged cookie accepted for issue time 1728000000`. The control panicked with `ValueExpired`.
  `git status --short` was empty afterwards. Pre-commit `linter all`, including the lychee fragment
  check of `#temporary-code-verbatim`, passed.
- Resolution reference: `docs(issues): record #2458 reproducer source verbatim`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2460#discussion_r4198706415>

### F2 - Scenario M2 was marked done after only its pre-fix half

- PR number: 2461
- Source review ID: 5431782940
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2461#discussion_r4198063765>
- Concern: M2's expected result covered both the pre-fix reproduction and the post-fix recheck,
  but its status was already `DONE (local reproduction)`.
- Solution: Split M2 into M2a (the pre-fix reproduction, `DONE`, described as actually performed)
  and M2b (the post-fix recheck, `TODO`). AC5 cites both, and the V1 evidence heading names M2a.
- Current-tree verification: `ISSUE.md` scenario rows M2a and M2b exist; AC5 reads
  `M2a (before), M2b (after)`; the evidence heading is `V1 - M2a: ...`.
- Resolution reference: `docs(issues): split #2458 forged-cookie scenario into before and after`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2461#discussion_r4198706733>

### F3 - Scenario evidence links pointed at wrong or missing sections

- PR number: 2461
- Source review ID: 5431782940
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2461#discussion_r4198063825>
- Concern: M1 pointed at V1, which records the forgery, and M3 pointed at V3, which does not exist.
- Solution: The pending scenarios M1, M2b and M3 now say "Not yet recorded". Only M2a links to the
  existing V1.
- Current-tree verification: In the manual-verification table, only the M2a row references
  `manual-verification-evidence.md`.
- Resolution reference: `docs(issues): split #2458 forged-cookie scenario into before and after`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2461#discussion_r4198707015>

### F4 - The evidence file had a duplicate `Failures and Follow-up` section

- PR number: 2461
- Source review ID: 5431782940
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2461#discussion_r4198063884>
- Concern: A second, template-text copy of `## Failures and Follow-up` followed the real one.
- Solution: Removed the leftover template copy.
- Current-tree verification: `grep -c '^## Failures and Follow-up'` on the evidence file returns 1.
- Resolution reference: `docs(issues): tidy #2458 verification evidence`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2461#discussion_r4198707234>

### F5 - `last-updated-utc` was unquoted in the evidence file

- PR number: 2461
- Source review ID: 5431782940
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2461#discussion_r4198063931>
- Concern: Quote the value to avoid YAML timestamp coercion and to match `ISSUE.md`.
- Solution: Quoted it. An unquoted value without seconds is not a YAML timestamp, so there was no
  coercion bug, but quoting keeps the issue folder consistent.
- Current-tree verification: The evidence frontmatter reads `last-updated-utc: "2026-10-06 17:06"`.
- Resolution reference: `docs(issues): tidy #2458 verification evidence`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2461#discussion_r4198707504>

### F12 - V1's forging source was not recorded, and the spec lacked a disposable-script record

- PR number: 2461
- Source review ID: 5431860814
- Reviewer finding ID: F1
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2461#discussion_r4198130287>
- Concern: The same missing verbatim source as F1. It also asked the spec to record the disposable
  script's location, automatic-test rationale, and removal or retention owner, as the issue
  template and create-issue require.
- Solution: The source part was already fixed for F1. Added a "Disposable Verification Scripts"
  subsection to the spec's Verification Plan for `forge_zeroed_cookie`, covering location,
  rationale, retention, and language.
- Current-tree verification: `ISSUE.md` has `### Disposable Verification Scripts` with the
  `forge_zeroed_cookie` entry; the evidence file has `#### Temporary Code (Verbatim)`.
- Resolution reference: `docs(issues): record #2458 reproducer source verbatim`; `docs(issues): record the #2458 disposable reproducer in the spec`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2461#discussion_r4198953844>

### F13 - No regression-test task recorded a red run before the fix

- PR number: 2461
- Source review ID: 5431860814
- Reviewer finding ID: F2
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2461#discussion_r4198130298>
- Concern: fix-bug requires a regression-test task with a recorded red run before the fix,
  followed by separate fix and green-plus-recheck tasks. T3 and T4 bundled tests with fixes, and
  none of R1 to R4 can fail against the current code. The reviewer suggested a `compile_fail`
  doctest for R2.
- Solution: Made R2 a maintained `compile_fail` doctest that is red on the current code, with a
  pinned error code and a compiling companion doctest. Stated why R1, R3 and R4 cannot be red
  before the fix, and named their mutate-then-restore substitutes. Split the plan into T3 (record
  R2's red run), T4 to T7 (fix), and T8 (green and recheck). Explained that the red doctest is
  committed with T4, because pre-commit runs `cargo test --doc`.
- Current-tree verification: A temporary `compile_fail` doctest probe in
  `packages/udp-core/src/lib.rs`, referencing `ZEROED_TEST_CIPHER_BLOWFISH`, made
  `cargo test --doc -p torrust-tracker-udp-core` (stable Rust 1.99.0) report "Test compiled
  successfully, but it's marked `compile_fail`". The probe was reverted and `git status --short`
  was empty. The Implementation Plan lists T1 to T8 as described.
- Resolution reference: `docs(issues): plan a pre-fix red regression test for #2458`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2461#discussion_r4198954167>

### F14 - False history of the seed check, raised again

- PR number: 2461
- Source review ID: 5431860814
- Reviewer finding ID: F3
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2461#discussion_r4198130305>
- Concern: The same history error as F8.
- Solution: No further change; F8's fix covers it.
- Current-tree verification: `ISSUE.md` Background item 3 says `e3562f069` added the cipher and
  `check_seed()` in the same patch, and contains neither "made sense" nor "kept the".
- Resolution reference: <https://github.com/torrust/torrust-tracker/pull/2461#discussion_r4198954473>
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2461#discussion_r4198954473>

### F15 - Bug-Fix Process step 2 still said "not yet attempted"

- PR number: 2461
- Source review ID: 5431860814
- Reviewer finding ID: F4
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2461#discussion_r4198130309>
- Concern: Step 2 still described a future reproduction with a network announce, although T1 was
  `DONE` and V1 was classified Reproduced with an in-process `check`.
- Solution: Rewrote step 2 as completed and classified it. It now states what was run and links
  V1 and M2a, and the network announce moved to the post-fix recheck M2b.
- Current-tree verification: Step 2 in `ISSUE.md` begins "Reproduction (done before maintainer
  review; outcome: Reproduced)".
- Resolution reference: `docs(issues): mark #2458 reproduction step as completed`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2461#discussion_r4198954765>

### F16 - M2 marked done with steps that did not run, raised again

- PR number: 2461
- Source review ID: 5431860814
- Reviewer finding ID: F5
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2461#discussion_r4198130319>
- Concern: The same M2 problem as F2. M2 was `DONE` but described a network announce that V1 did
  not send.
- Solution: No further change. F2's split implemented the reviewer's second option: M2a describes
  what V1 ran, and the network announce is kept for M2b.
- Current-tree verification: The M2a row describes the alias mutation, tracker start, and
  in-process `check`; the M2b row holds the `UdpTrackerClient::send` announce and is `TODO`.
- Resolution reference: <https://github.com/torrust/torrust-tracker/pull/2461#discussion_r4198955043>
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2461#discussion_r4198955043>

### F6 - Pending scenarios had no evidence process of their own

- PR number: 2461
- Source review ID: 5431860814
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2461#discussion_r4198130325>
- Concern: The same wrong M1 evidence link as F3. It also asked to point M1 at its own future
  process and to number M3 to match.
- Solution: F3 had removed the wrong links. This finding added planned processes for the pending
  scenarios: M1 to V2, M2b to V3, and M3 to V4, each marked as not yet recorded.
- Current-tree verification: The scenario rows cite V1 (M2a), and V2, V3, V4 "(not yet recorded)"
  for M1, M2b, M3.
- Resolution reference: `docs(issues): assign planned evidence sections to #2458 scenarios`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2461#discussion_r4198955312>

### F7 - Duplicate `Failures and Follow-up` heading, raised again

- PR number: 2461
- Source review ID: 5431860814
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2461#discussion_r4198130335>
- Concern: The same duplicate heading as F4.
- Solution: No further change; F4's fix covers it.
- Current-tree verification: `grep -c '^## Failures and Follow-up'` on the evidence file returns 1.
- Resolution reference: <https://github.com/torrust/torrust-tracker/pull/2461#discussion_r4198955585>
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2461#discussion_r4198955585>

### F17 - The secrecy ADR that governs the key's redaction was not linked

- PR number: 2461
- Source review ID: 5431860814
- Reviewer finding ID: F8
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2461#discussion_r4198130343>
- Concern: "Related ADRs: none found" missed the secrecy ADR, which covers private keys and keeping
  `Debug` and tracing redacted. The spec should say whether the key uses `secrecy` or why not, and
  should link `handle-secrets`.
- Solution: Linked the ADR. The spec explains that `secrecy::SecretBox<S>` requires `S: Zeroize`,
  that `blowfish` 0.10 implements only `ZeroizeOnDrop` (behind its `zeroize` feature), and that
  wrapping raw key bytes would rerun the key schedule on every call. It therefore plans a
  hand-written `[REDACTED]` `Debug` with no exposing accessor. R4 now asserts that exact output and
  the absence of a unique test key. The new ADR records the deviation and the `zeroize` decision.
  Added `handle-secrets` and the ADR to the frontmatter.
- Current-tree verification: In the local Cargo registry, `secrecy-0.10.3/src/lib.rs:58` declares
  `pub struct SecretBox<S: Zeroize + ?Sized>`, and `blowfish-0.10.0/src/lib.rs` has only
  `impl<T: ByteOrder> ZeroizeOnDrop for Blowfish<T>`. Pre-commit `linter all` (lychee) resolved the
  new ADR link.
- Resolution reference: `docs(issues): link the secrecy ADR from #2458`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2461#discussion_r4198955839>

### F9 - Template checkpoints were missing

- PR number: 2461
- Source review ID: 5431860814
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2461#discussion_r4198130349>
- Concern: The spec dropped the template's spec-only-PR checkpoint and the reviewer and committer
  checkpoints.
- Solution: Restored them. The spec-only-PR checkpoint is left unchecked and names PR #2461.
- Current-tree verification: Workflow Checkpoints include the spec-only-PR, reviewer-validation,
  agent-review-reports, and committer lines from `docs/templates/ISSUE.md`.
- Resolution reference: `docs(issues): restore #2458 template checkpoints`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2461#discussion_r4198956131>

### F10 - create-issue Step 6 still branched again after Step 5 pushed

- PR number: 2461
- Source review ID: 5431860814
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2461#discussion_r4198130356>
- Concern: Step 6 items 1 and 2 still said to branch from `develop` and name the branch, after the
  new Step 3 decision and the Step 5 push.
- Solution: Merged items 1 and 2 into one back-reference to the Step 3 decision ("Do not branch
  again"), and renumbered the rest.
- Current-tree verification: Step 6 item 1 in `create-issue/SKILL.md` refers to "Decide the
  Spec-Only PR and Name the Branch"; no other repository file cites Step 6 item numbers
  (`grep -rn 'Step 6 item'` over `.github`, `docs/templates`, and `AGENTS.md` is empty).
- Resolution reference: `docs(skills): point create-issue Step 6 at the Step 3 branch decision`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2461#discussion_r4198956427>

### F11 - The `develop` commit under test was not named

- PR number: 2461
- Source review ID: 5431860814
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2461#discussion_r4198130361>
- Concern: "on top of `develop`" did not say which `develop` commit was tested.
- Solution: "Artifact under test" now names `develop` at `9be79fc5a` for both runs.
- Current-tree verification: `git merge-base --is-ancestor 9be79fc5a torrust/develop` succeeds.
  The branch reflog shows it was created from `9be79fc5a`, and the only later rebase (17:52 UTC)
  came after the 17:06 UTC rerun.
- Resolution reference: `docs(issues): name the develop commit under test in #2458 evidence`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2461#discussion_r4198956666>

## Processing Log

- 2026-10-06 16:54 UTC - Started audit at the maintainer's request. Fetched all threads of PR #2460
  (two, unresolved) and PR #2461 (four, unresolved) with `github-review-threads`, and both Copilot
  reviews. Confirmed that review 5431658413 was submitted after PR #2460 closed. After read-only
  triage, the maintainer approved handling the PR #2460 findings on PR #2461 and documenting this
  case in the process-pr-review skill.
- 2026-10-06 17:05 UTC - Committed the F8 fix.
- 2026-10-06 17:06 UTC - Recreated and reran the reproducer for F1, with a control run.
- 2026-10-06 17:09 UTC - Committed the F1, F2/F3, and F4/F5 fixes.
- 2026-10-06 17:51 UTC - Committed the process-pr-review skill update.
- 2026-10-06 17:52 UTC - Rebased onto the latest `develop` (two new commits) and pushed after the
  pre-push checks passed.
- 2026-10-06 17:54 UTC - Replied to all six threads; recorded reply URLs.
- 2026-10-06 17:55 UTC - Committed and pushed this audit. `reply-status` confirmed replies on the
  six threads, which were then resolved one by one. It also showed eleven new threads from human
  review 5431860814 (`da2ce7`). The maintainer approved processing that round.
- 2026-10-06 18:11 UTC - Committed the F15 fix. Probed and reverted a `compile_fail` doctest for
  F13.
- 2026-10-06 18:14 UTC - Committed the F13, F12, F6, F17, F9, F11, and F10 fixes (through 18:18 UTC)
  and pushed after the pre-push checks passed.
- 2026-10-06 18:20 UTC - Replied to all eleven threads; recorded reply URLs.

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
