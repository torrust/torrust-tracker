---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
---

<!-- skill-link: process-pr-review -->

# PR #2859 Review Audit

Source: pull-request reviews and inline review threads for <https://github.com/torrust/torrust-tracker/pull/2859>.

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

F1 to F5 come from the Copilot review, which has no finding IDs; their severity is inferred from the
medium and low marks of its overview. F6 to F15 come from the da2ce7 review, whose own IDs F1 to F10
collide with the Copilot rows and are kept as `Reviewer finding ID`. F16 to F22 come from the
second da2ce7 review and F23 to F24 from the second Copilot review, both on head `7f8eef0dc`. Most findings concern the shared
port helper, which the maintainer's scope change removed before this round was processed.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2859-f1 | Copilot | Minor (inferred) | documentation | ORIGINAL | NO_ACTION | SUPERSEDED |
| F2 | review-finding:pr-2859-f2 | Copilot | Minor (inferred) | maintainability | ORIGINAL | NO_ACTION | SUPERSEDED |
| F3 | review-finding:pr-2859-f3 | Copilot | Minor (inferred) | maintainability | RE_RAISE_OF:F2 | NO_ACTION | SUPERSEDED |
| F4 | review-finding:pr-2859-f4 | Copilot | Nit (inferred) | metadata | ORIGINAL | FIXED | RESOLVED |
| F5 | review-finding:pr-2859-f5 | Copilot | Nit (inferred) | maintainability | ORIGINAL | NO_ACTION | SUPERSEDED |
| F6 | review-finding:pr-2859-f6 | Human | Major | documentation | ORIGINAL | FIXED | RESOLVED |
| F7 | review-finding:pr-2859-f7 | Human | Major | metadata | ORIGINAL | FIXED | RESOLVED |
| F8 | review-finding:pr-2859-f8 | Human | Minor | testing | ORIGINAL | FIXED | RESOLVED |
| F9 | review-finding:pr-2859-f9 | Human | Minor | testing | ORIGINAL | FIXED | RESOLVED |
| F10 | review-finding:pr-2859-f10 | Human | Minor | correctness | ORIGINAL | NO_ACTION | SUPERSEDED |
| F11 | review-finding:pr-2859-f11 | Human | Minor | metadata | ORIGINAL | FIXED | RESOLVED |
| F12 | review-finding:pr-2859-f12 | Human | Nit | formatting | ORIGINAL | NO_ACTION | SUPERSEDED |
| F13 | review-finding:pr-2859-f13 | Human | Suggestion | maintainability | RE_RAISE_OF:F2 | NO_ACTION | SUPERSEDED |
| F14 | review-finding:pr-2859-f14 | Human | Nit | documentation | ORIGINAL | NO_ACTION | SUPERSEDED |
| F15 | review-finding:pr-2859-f15 | Human | Nit | documentation | ORIGINAL | NO_ACTION | SUPERSEDED |

| F16 | review-finding:pr-2859-f16 | Human | Major | metadata | RE_RAISE_OF:F7 | FIXED | RESOLVED |
| F17 | review-finding:pr-2859-f17 | Human | Minor | testing | RE_RAISE_OF:F9 | FIXED | RESOLVED |
| F18 | review-finding:pr-2859-f18 | Human | Minor | metadata | RE_RAISE_OF:F11 | FIXED | RESOLVED |
| F19 | review-finding:pr-2859-f19 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F20 | review-finding:pr-2859-f20 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F21 | review-finding:pr-2859-f21 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F22 | review-finding:pr-2859-f22 | Human | Minor | other | ORIGINAL | FIXED | RESOLVED |
| F23 | review-finding:pr-2859-f23 | Copilot | Minor (inferred) | testing | ORIGINAL | FIXED | RESOLVED |
| F24 | review-finding:pr-2859-f24 | Copilot | Minor (inferred) | maintainability | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - Tracker-specific wording in the shared port helper docs

- PR number: 2859
- Source review ID: 5903121881
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2859#discussion_r4572554080>
- Concern: The module docs of `packages/test-helpers/src/ports.rs` still spoke of the tracker child after the move into a general-purpose crate.
- Solution: No change. The maintainer narrowed #2393 to a quick patch without a new port helper, and the commit `revert: [#2393] undo the shared port helper move and fixture changes` removed the file.
- Current-tree verification: `git ls-tree --name-only HEAD packages/test-helpers/src/` lists `configuration.rs`, `lib.rs`, `logging.rs` and `random.rs`; `ports.rs` is gone.
- Resolution reference: <REPLY_F1>
- Follow-up PR URL: N/A
- Reply URL: <REPLY_F1>

### F2 - Duplicated candidate-port generator in the TCP and UDP helpers

- PR number: 2859
- Source review ID: 5903121881
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2859#discussion_r4572554092>
- Concern: The candidate-port iterator was repeated in `reserve_free_ports` and `reserve_free_udp_address`.
- Solution: No change. The commit `revert: [#2393] undo the shared port helper move and fixture changes` removed `reserve_free_udp_address` with the rest of the shared helper.
- Current-tree verification: `git ls-tree --name-only HEAD packages/test-helpers/src/` lists `configuration.rs`, `lib.rs`, `logging.rs` and `random.rs`; `ports.rs` is gone.
- Resolution reference: <REPLY_F2>
- Follow-up PR URL: N/A
- Reply URL: <REPLY_F2>

### F3 - Duplicated candidate-port generator, second location

- PR number: 2859
- Source review ID: 5903121881
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2859#discussion_r4572554103>
- Concern: The same duplication as F2, reported at the second copy.
- Solution: No change, for the same reason as F2: the commit `revert: [#2393] undo the shared port helper move and fixture changes` removed the helper.
- Current-tree verification: `git ls-tree --name-only HEAD packages/test-helpers/src/` lists `configuration.rs`, `lib.rs`, `logging.rs` and `random.rs`; `ports.rs` is gone.
- Resolution reference: <REPLY_F3>
- Follow-up PR URL: N/A
- Reply URL: <REPLY_F3>

### F4 - Progress-log entry without a time and time zone

- PR number: 2859
- Source review ID: 5903121881
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2859#discussion_r4572554119>
- Concern: The new entry in the spec progress log had a date only, unlike the earlier entries.
- Solution: The entry that replaced it after the scope change had the same gap. It now reads `2026-11-14 09:14 UTC`, the commit time of the change it describes.
- Current-tree verification: `grep -n "^- 2026-11-14" docs/issues/open/2393-2392-test-port-allocation-races/ISSUE.md` prints the entry starting `2026-11-14 09:14 UTC - Claude Code (Claude Sonnet 5.5)`.
- Resolution reference: docs(issues): [#2393] stamp the progress log entry with its time and model
- Follow-up PR URL: N/A
- Reply URL: <REPLY_F4>

### F5 - Indexing `held[0]` relies on an implicit invariant

- PR number: 2859
- Source review ID: 5903121881
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2859#discussion_r4572554139>
- Concern: `reserve_free_udp_address` read `held[0]` from `reserve_from(1, ...)` without stating the one-element invariant.
- Solution: No change. The commit `revert: [#2393] undo the shared port helper move and fixture changes` removed the function.
- Current-tree verification: `git ls-tree --name-only HEAD packages/test-helpers/src/` lists `configuration.rs`, `lib.rs`, `logging.rs` and `random.rs`; `ports.rs` is gone.
- Resolution reference: <REPLY_F5>
- Follow-up PR URL: N/A
- Reply URL: <REPLY_F5>

### F6 - Plan rows marked DONE without their outputs

- PR number: 2859
- Source review ID: 5903160091
- Reviewer finding ID: F1
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2859#discussion_r4572584176>
- Concern: T2 to T5 were `DONE` although the seam, the red and green evidence and the design review they named were never produced, and V2 to V4 stayed `TODO`.
- Solution: After the scope change the plan has new rows. Each `DONE` row now names an output that is in the tree: T2 the inventory, T3 the ignored tests, T4 the rewritten tests. T5 is `BLOCKED` on the maintainer and T6 is `IN_PROGRESS` with the 150-run recheck. V2 and V3 are `NOT_APPLICABLE` with the reason; V4 is filled by F8.
- Current-tree verification: `grep -n "^| T[1-6] |" docs/issues/open/2393-2392-test-port-allocation-races/ISSUE.md` and `grep -n "Status" docs/issues/open/2393-2392-test-port-allocation-races/manual-verification-evidence.md` show the rows and states above.
- Resolution reference: docs(issues): [#2393] mark the plan rows by what ran and the evidence entries by the new scope
- Follow-up PR URL: N/A
- Reply URL: <REPLY_F6>

### F7 - Closing keyword while the issue has open work

- PR number: 2859
- Source review ID: 5903160091
- Reviewer finding ID: F2
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2859#discussion_r4572584183>
- Concern: The PR body said `Closes #2393` while T6, the acceptance criteria and the evidence were still open.
- Solution: The PR body now says `Related to #2393`. T5 waits for the maintainer to decide the fate of the eighteen ignored tests, so this PR does not close the issue.
- Current-tree verification: `gh api repos/torrust/torrust-tracker/pulls/2859 --jq .body` starts with `Related to #2393`.
- Resolution reference: <CONV_URL>
- Follow-up PR URL: N/A
- Reply URL: <REPLY_F7>

### F8 - No red run claim and too few green runs

- PR number: 2859
- Source review ID: 5903160091
- Reviewer finding ID: F3
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2859#discussion_r4572584189>
- Concern: The entry claimed no red run was possible although V1 had observed the failure, and nine green runs would also pass with no fix at a probability of 0.70.
- Solution: The claim was removed with the scope change. The recheck was sized for the claim: 150 full-suite runs, which give about 95% confidence that the 1-in-50 `axum-http-server` rate of V1 is gone. The runs and their result are recorded as V4.
- Current-tree verification: `manual-verification-evidence.md` V4 records two rechecks of 150 runs of `cargo nextest run --tests --all-features --no-fail-fast`: 2 failures on head `14cf46913` from a third racing shape, fixed in `fix(tests): [#2393] start the health-check API before stopping the probed service`, then 150 of 150 passed on head `363c30ac2`.
- Resolution reference: docs(issues): [#2393] record the 150-run recheck
- Follow-up PR URL: N/A
- Reply URL: <REPLY_F8>

### F9 - Release-then-rebind assertions left out of the audit

- PR number: 2859
- Source review ID: 5903160091
- Reviewer finding ID: F4
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2859#discussion_r4572584194>
- Concern: The tests that rebind a released ephemeral port after a stop or a drop were not in the inventory, although V1 failed on that kind of assertion.
- Solution: The inventory now names both racing shapes. The eight stop and drop tests in the four environment modules, and the orphaned-processor test in `udp-server` `launcher.rs` that waits for the same rebind, are ignored and listed as needing a maintainer decision.
- Current-tree verification: `grep -rn "flaky port race, see #2393" packages src | wc -l` prints `18`.
- Resolution reference: fix(tests): [#2393] ignore the tests that rebind a released port
- Follow-up PR URL: N/A
- Reply URL: <REPLY_F9>

### F10 - Helper range overlaps the fixed metrics ports

- PR number: 2859
- Source review ID: 5903160091
- Reviewer finding ID: F5
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2859#discussion_r4572584199>
- Concern: The draw range of the shared helper contained `17091`-`17094`, which `tests/metrics/fixed_ports.rs` binds.
- Solution: No change. The commit `revert: [#2393] undo the shared port helper move and fixture changes` removed the shared helper and its eleven call sites. The #2581 helper keeps its CLI callers only.
- Current-tree verification: `git ls-tree --name-only HEAD packages/test-helpers/src/` lists `configuration.rs`, `lib.rs`, `logging.rs` and `random.rs`; `ports.rs` is gone.
- Resolution reference: <REPLY_F10>
- Follow-up PR URL: N/A
- Reply URL: <REPLY_F10>

### F11 - Progress-log entry without model and time, stale frontmatter

- PR number: 2859
- Source review ID: 5903160091
- Reviewer finding ID: F6
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2859#discussion_r4572584206>
- Concern: The entry named no model and no `HH:MM UTC` time, and `last-updated-utc` still read `2026-10-02 13:30`.
- Solution: The entry now carries `09:14 UTC` and `Claude Code (Claude Sonnet 5.5)`, the model that wrote it. `last-updated-utc` and `status` were updated.
- Current-tree verification: `sed -n 5,12p docs/issues/open/2393-2392-test-port-allocation-races/ISSUE.md` shows `status: in-progress` and a `2026-11-14` value for `last-updated-utc`.
- Resolution reference: docs(issues): [#2393] stamp the progress log entry with its time and model
- Follow-up PR URL: N/A
- Reply URL: <REPLY_F11>

### F12 - Overlong doc lines and tracker context in the shared helper

- PR number: 2859
- Source review ID: 5903160091
- Reviewer finding ID: F7
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2859#discussion_r4572584211>
- Concern: A text replacement left a 130-column line and the module docs referred to `43152` and the tracker child.
- Solution: No change. The commit `revert: [#2393] undo the shared port helper move and fixture changes` removed the file.
- Current-tree verification: `git ls-tree --name-only HEAD packages/test-helpers/src/` lists `configuration.rs`, `lib.rs`, `logging.rs` and `random.rs`; `ports.rs` is gone.
- Resolution reference: <REPLY_F12>
- Follow-up PR URL: N/A
- Reply URL: <REPLY_F12>

### F13 - Repeated candidate generator

- PR number: 2859
- Source review ID: 5903160091
- Reviewer finding ID: F8
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2859#discussion_r4572584216>
- Concern: The same duplication as F2.
- Solution: No change, for the same reason as F2: the commit `revert: [#2393] undo the shared port helper move and fixture changes` removed the helper.
- Current-tree verification: `git ls-tree --name-only HEAD packages/test-helpers/src/` lists `configuration.rs`, `lib.rs`, `logging.rs` and `random.rs`; `ports.rs` is gone.
- Resolution reference: <REPLY_F13>
- Follow-up PR URL: N/A
- Reply URL: <REPLY_F13>

### F14 - Crate API descriptions omit the new `ports` module

- PR number: 2859
- Source review ID: 5903160091
- Reviewer finding ID: F9
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2859#discussion_r4572584222>
- Concern: `packages/test-helpers/README.md` and `docs/packages.md` listed only `configuration`, `logging` and `random`.
- Solution: No change. The commit `revert: [#2393] undo the shared port helper move and fixture changes` removed the `ports` module, so those lists are accurate again.
- Current-tree verification: `grep -n "pub mod" packages/test-helpers/src/lib.rs` lists `configuration`, `logging` and `random`.
- Resolution reference: <REPLY_F14>
- Follow-up PR URL: N/A
- Reply URL: <REPLY_F14>

### F15 - Comment drops the "by default" qualifier

- PR number: 2859
- Source review ID: 5903160091
- Reviewer finding ID: F10
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2859#discussion_r4572584229>
- Concern: The fixture comment in `axum-http-server` said the kernel never assigns the range, without "by default".
- Solution: No change. The commit `revert: [#2393] undo the shared port helper move and fixture changes` restored the original fixture and removed the comment.
- Current-tree verification: `grep -c "kernel never assigns" packages/axum-http-server/src/server.rs` prints `0`.
- Resolution reference: <REPLY_F15>
- Follow-up PR URL: N/A
- Reply URL: <REPLY_F15>

### F16 - Closing keyword still in the PR body

- PR number: 2859
- Source review ID: 5903429720
- Reviewer finding ID: F2
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2859#discussion_r4572801929>
- Concern: At head `7f8eef0dc` the body still opened with `Closes #2393` while T5 was blocked and the ignore attributes point at #2393 for the pending decision.
- Solution: Same fix as F7: the PR body now says `Related to #2393`.
- Current-tree verification: `gh api repos/torrust/torrust-tracker/pulls/2859 --jq .body` starts with `Related to #2393`.
- Resolution reference: <CONV_URL>
- Follow-up PR URL: N/A
- Reply URL: <REPLY_F16>

### F17 - Release-then-rebind tests still unclassified

- PR number: 2859
- Source review ID: 5903429720
- Reviewer finding ID: F4
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2859#discussion_r4572801931>
- Concern: The four environment modules re-bind a released ephemeral port at once, and their drop-path waits do the same, but none was ignored or listed.
- Solution: Same fix as F9: the eight tests are ignored and listed, with the second racing shape described in the inventory.
- Current-tree verification: `grep -rn "flaky port race, see #2393" packages src | wc -l` prints `18`.
- Resolution reference: fix(tests): [#2393] ignore the tests that rebind a released port
- Follow-up PR URL: N/A
- Reply URL: <REPLY_F17>

### F18 - Progress-log entry still without time and model

- PR number: 2859
- Source review ID: 5903429720
- Reviewer finding ID: F6
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2859#discussion_r4572801933>
- Concern: The replacement entry also had no `HH:MM UTC` time and no model, and `last-updated-utc` was stale.
- Solution: Same fix as F11.
- Current-tree verification: `grep -n "^- 2026-11-14" docs/issues/open/2393-2392-test-port-allocation-races/ISSUE.md` prints the entry starting `2026-11-14 09:14 UTC - Claude Code (Claude Sonnet 5.5)`.
- Resolution reference: docs(issues): [#2393] stamp the progress log entry with its time and model
- Follow-up PR URL: N/A
- Reply URL: <REPLY_F18>

### F19 - Required bug-spec sections deleted by the rewrite

- PR number: 2859
- Source review ID: 5903429720
- Reviewer finding ID: F11
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2859#discussion_r4572801934>
- Concern: The rewrite removed `Bug-Fix Process`, `Regression Test Strategy`, `Commit Points`, `Acceptance Verification` and the manual scenario table, and a checkpoint still described the dropped red-then-green process.
- Solution: The five sections are back, restated for the narrowed scope: the bug-fix process lists the steps that still apply, the regression strategy says no test is added and why, and the scenario table tracks M1 to M4 with their status. The checkpoints follow the new plan.
- Current-tree verification: `grep -n "^## \|^### " docs/issues/open/2393-2392-test-port-allocation-races/ISSUE.md` lists `Bug-Fix Process`, `Regression Test Strategy`, `Commit Points`, `Manual Verification Scenarios` and `Acceptance Verification`.
- Resolution reference: docs(issues): [#2393] restate the bug-fix sections for the narrowed scope
- Follow-up PR URL: N/A
- Reply URL: <REPLY_F19>

### F20 - Verification plan points at scenarios that do not exist

- PR number: 2859
- Source review ID: 5903429720
- Reviewer finding ID: F12
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2859#discussion_r4572801936>
- Concern: The plan named M1 and M4 in the evidence file, which only has V1 to V4, and V2 and V3 still described the abandoned repairs.
- Solution: The plan names V1 and V4, and V2 and V3 are `NOT_APPLICABLE` with the reason. The scenario table restored for F19 maps M1 to M4 to those sections.
- Current-tree verification: `grep -n "^### V\|Status" docs/issues/open/2393-2392-test-port-allocation-races/manual-verification-evidence.md` shows V2 and V3 as `NOT_APPLICABLE`.
- Resolution reference: docs(issues): [#2393] mark the plan rows by what ran and the evidence entries by the new scope
- Follow-up PR URL: N/A
- Reply URL: <REPLY_F20>

### F21 - Text still following the old plan

- PR number: 2859
- Source review ID: 5903429720
- Reviewer finding ID: F13
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2859#discussion_r4572801938>
- Concern: The "after a failure" sentence, the old task numbers T5 and T4 in the related-issues table and the completion-review wording followed the previous plan.
- Solution: The sentence now says "after a stop, a drop or a failure", the related-issues rows no longer cite task numbers, and the completion review asks for the T5 outcome.
- Current-tree verification: `grep -c "in T5;\|precedent for T4\|selected fixture" docs/issues/open/2393-2392-test-port-allocation-races/ISSUE.md` prints `0`.
- Resolution reference: docs(issues): [#2393] align the remaining spec text with the narrowed plan
- Follow-up PR URL: N/A
- Reply URL: <REPLY_F21>

### F22 - No replies, resolutions or audit record

- PR number: 2859
- Source review ID: 5903429720
- Reviewer finding ID: F14
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2859#discussion_r4572801943>
- Concern: None of the 15 threads had a reply and the PR had no audit record.
- Solution: This audit record was created before the first reply, and every thread of both rounds was replied to and then resolved.
- Current-tree verification: PENDING_F22
- Resolution reference: docs(pr-reviews): [#2393] create the PR 2859 review audit record
- Follow-up PR URL: N/A
- Reply URL: <REPLY_F22>

### F23 - Dead-address test assumes port 1 is closed

- PR number: 2859
- Source review ID: 5903455135
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2859#discussion_r4572822006>
- Concern: The rewritten test assumed nothing listens on `127.0.0.1:1`, which is not guaranteed, and its comment confused binding with connecting.
- Solution: The test binds a `tokio::net::TcpSocket` to port `0` and never listens on it. The test owns the port, so nothing else can listen there, and a connection to it is refused.
- Current-tree verification: `cargo nextest run -p torrust-tracker-axum-rest-api-server -E 'test(nothing_listens_on_the_checked_address)'` prints `1 test run: 1 passed`.
- Resolution reference: fix(tests): [#2393] check the dead address on a bound socket that never listens
- Follow-up PR URL: N/A
- Reply URL: <REPLY_F23>

### F24 - Racy default constructor of the UDP fixture

- PR number: 2859
- Source review ID: 5903455135
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2859#discussion_r4572822041>
- Concern: `StoppedUdpServerOnFreeAddress::new()` still chose a released address, which invites new tests to reuse the racing pattern.
- Solution: The constructor is renamed `on_a_released_free_address` with a doc comment that names the hazard and points to `bound_to` with port `0`. Only the two ignored tests call it.
- Current-tree verification: `grep -n "on_a_released_free_address" packages/udp-server/src/server/mod.rs` prints the definition and the two ignored callers.
- Resolution reference: refactor(tests): [#2393] name the UDP fixture constructor after its released address
- Follow-up PR URL: N/A
- Reply URL: <REPLY_F24>

## Processing Log

- 2026-11-14 09:24 UTC - Started audit. Copilot reviewed head `0e43f34a9` at 08:23:56Z (COMMENTED,
  five inline threads, with the "didn't start before the timeout" warning) and da2ce7 reviewed the
  same head at 08:30:39Z (CHANGES_REQUESTED, ten inline threads). Between the reviews and this
  processing, the maintainer narrowed #2393 and the branch reverted the shared helper, so nine
  findings no longer apply to the current tree.

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
