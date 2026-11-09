---
pr-number: 2784
pr-url: https://github.com/torrust/torrust-tracker/pull/2784
last-updated-utc: "2026-11-09 17:24"
---

# PR #2784 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/2784.

## Ownership

The PR author owns this tracked audit record. Reviewers, including repository review agents,
deliver findings through GitHub and have no repository-artifact obligation.

- Post-merge workflow approval: N/A

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

Round 1: Copilot review 5828888059 on the head then under review (three inline findings; the review body reports that Copilot full agentic review timed out, so this was a lite review). F2 is valid and fixed; F1 and F3 need no change, with reasons. Thread state is OPEN for all three: the auto mode classifier denied the bot resolveReviewThread mutation again, so the maintainer or reviewer resolves them.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2784-f1 | Copilot | Medium | documentation | ORIGINAL | NO_ACTION | OPEN |
| F2 | review-finding:pr-2784-f2 | Copilot | Low | documentation | ORIGINAL | FIXED | OPEN |
| F3 | review-finding:pr-2784-f3 | Copilot | Low | formatting | ORIGINAL | NO_ACTION | OPEN |
| F4 | review-finding:pr-2784-f4 | Human | Major | correctness | ORIGINAL | FIXED | OPEN |
| F5 | review-finding:pr-2784-f5 | Human | Minor | documentation | ORIGINAL | FIXED | OPEN |
| F6 | review-finding:pr-2784-f6 | Human | Minor | documentation | ORIGINAL | FIXED | OPEN |
| F7 | review-finding:pr-2784-f7 | Human | Minor | documentation | ORIGINAL | FIXED | OPEN |
| F8 | review-finding:pr-2784-f8 | Human | Minor | documentation | ORIGINAL | FIXED | OPEN |
| F9 | review-finding:pr-2784-f9 | Human | Nit | documentation | ORIGINAL | FIXED | OPEN |
| F10 | review-finding:pr-2784-f10 | Human | Nit | documentation | ORIGINAL | FIXED | OPEN |
| F11 | review-finding:pr-2784-f11 | Human | Nit | documentation | ORIGINAL | FIXED | OPEN |
| F12 | review-finding:pr-2784-f12 | Human | Minor | documentation | ORIGINAL | FIXED | OPEN |
| F13 | review-finding:pr-2784-f13 | Human | Minor | documentation | ORIGINAL | FIXED | OPEN |
| F14 | review-finding:pr-2784-f14 | Human | Minor | documentation | ORIGINAL | FIXED | OPEN |
| F15 | review-finding:pr-2784-f15 | Human | Minor | documentation | ORIGINAL | FIXED | OPEN |
| F16 | review-finding:pr-2784-f16 | Human | Nit | documentation | ORIGINAL | FIXED | OPEN |
| F17 | review-finding:pr-2784-f17 | Human | Nit | maintainability | ORIGINAL | NO_ACTION | OPEN |

Round 2: da2ce7 review 5829122067 (CHANGES_REQUESTED) on the head then under review: eight inline findings (F4 blocks, F5 to F11 do not), all valid and fixed. The reviewer also checked the three Copilot threads; the review says only that the diagram note's names follow the sentence already there, that `default = [  ]` is the form of the sibling manifests, and that rows 5 and 6 reached DONE in their archive commits. Thread state is OPEN for all eleven: the auto mode classifier denied the bot resolveReviewThread mutation, so the maintainer or reviewer resolves them.

Round 3: da2ce7 review 5829737162 (COMMENTED, no approval) on the head then under review: six inline findings, none blocking (F12 to F17; the review labels them F9 to F14), and a failing audit validator on rows F1 and F3. All valid; five fixed, one acknowledged. Thread state is OPEN for all seventeen: the auto mode classifier denies the bot resolveReviewThread mutation.

## Finding Details

### F1 - EPIC row 7 stays IN_PROGRESS although the PR closes #2757.

- PR number: 2784
- Source review ID: 5828888059
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2784#discussion_r4515052659
- Concern: EPIC row 7 stays IN_PROGRESS although the PR closes #2757.
- Solution: No change: the archive PR after the merge sets the row to DONE and moves the spec (precedent: row 6, archive commit "chore(issues): [#2745] archive the closed issue spec to docs/issues/closed").
- Current-tree verification: git log -S shows the DONE text for row 6 arrived in the archive commit, not in the implementation PR.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2784#discussion_r4515745583
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2784#discussion_r4515745583

### F2 - The diagram note mixes node names (udp-srv) and package names (udp-server).

- PR number: 2784
- Source review ID: 5828888059
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2784#discussion_r4515052736
- Concern: The diagram note mixes node names (udp-srv) and package names (udp-server).
- Solution: Valid. The sentence names the diagram nodes throughout.
- Current-tree verification: sed -n 20,28p docs/media/packages/dependencies-workspace-packages.md shows node names only.
- Resolution reference: docs(packages): [#2757] name the diagram nodes consistently in the test-only edges note
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2784#discussion_r4515198533

### F3 - Use default = [] instead of default = [  ].

- PR number: 2784
- Source review ID: 5828888059
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2784#discussion_r4515052807
- Concern: Use default = [] instead of default = [  ].
- Solution: No change: default = [  ] is taplo output; four sibling crates use it and the pre-commit TOML check passes.
- Current-tree verification: grep -rn "default = \[" packages/*/Cargo.toml lists the same form in six lines: this package and five siblings.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2784#discussion_r4515746031
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2784#discussion_r4515746031

### F4 - The stated cause of the url serde change (dropping the configuration edge) is wrong; the no-features lib build failed at the merge base.

- PR number: 2784
- Source review ID: 5829122067
- Reviewer finding ID: F1 (Major, as labelled in the review)
- Source URL: https://github.com/torrust/torrust-tracker/pull/2784#discussion_r4515233784
- Concern: The stated cause of the url serde change (dropping the configuration edge) is wrong; the no-features lib build failed at the merge base.
- Solution: Valid. Measured at ca8efcd4d (cargo check --lib exits 101, E0277). The retrospective, inventory, spec log and EPIC log now state the measured cause. The first implementation commit body stays wrong (history is not rewritten): the maintainer decides, e.g. a squash merge with a corrected message.
- Current-tree verification: evidence/verify.sh first block shows the failure at the merge base; inventory line on axum-health-check-api-server states the measured cause.
- Resolution reference: docs(issues): [#2757] correct the url serde cause and apply review round 1 fixes
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2784#discussion_r4515364071

### F5 - Rule A.6 half applied: the module has no documentation saying it is test support outside semver.

- PR number: 2784
- Source review ID: 5829122067
- Reviewer finding ID: F2 (Minor, as labelled in the review)
- Source URL: https://github.com/torrust/torrust-tracker/pull/2784#discussion_r4515233794
- Concern: Rule A.6 half applied: the module has no documentation saying it is test support outside semver.
- Solution: Valid. src/environment.rs starts with a //! note like the sibling environments.
- Current-tree verification: head -12 packages/axum-health-check-api-server/src/environment.rs shows the note.
- Resolution reference: docs(issues): [#2757] correct the url serde cause and apply review round 1 fixes
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2784#discussion_r4515364536

### F6 - The evidence cites a PR-branch commit by id.

- PR number: 2784
- Source review ID: 5829122067
- Reviewer finding ID: F3 (Minor, as labelled in the review)
- Source URL: https://github.com/torrust/torrust-tracker/pull/2784#discussion_r4515233799
- Concern: The evidence cites a PR-branch commit by id.
- Solution: Valid. The evidence names the commit by subject and the script prints the subject.
- Current-tree verification: grep for a 9-hex id in manual-verification-evidence.md finds only ca8efcd4d, a develop merge commit.
- Resolution reference: docs(issues): [#2757] rewrite the evidence around the committed script
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2784#discussion_r4515364966

### F7 - The evidence cannot be audited from the repository: the commands live in /tmp.

- PR number: 2784
- Source review ID: 5829122067
- Reviewer finding ID: F4 (Minor, as labelled in the review)
- Source URL: https://github.com/torrust/torrust-tracker/pull/2784#discussion_r4515233807
- Concern: The evidence cannot be audited from the repository: the commands live in /tmp.
- Solution: Valid. The script and the diagram checker live in the evidence folder; the transcript prints each command beside its output and runs the baseline at the merge base.
- Current-tree verification: ls docs/issues/open/2757-*/evidence shows verify.sh and check_diagram.py.
- Resolution reference: docs(issues): [#2757] correct the url serde cause and apply review round 1 fixes; docs(issues): [#2757] rewrite the evidence around the committed script
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2784#discussion_r4515365447

### F8 - The diagram still draws six test-helpers arrows no manifest declares; the edge count is stale.

- PR number: 2784
- Source review ID: 5829122067
- Reviewer finding ID: F5 (Minor, as labelled in the review)
- Source URL: https://github.com/torrust/torrust-tracker/pull/2784#discussion_r4515233817
- Concern: The diagram still draws six test-helpers arrows no manifest declares; the edge count is stale.
- Solution: Valid. The six arrows are removed and the count is 135, checked by check_diagram.py.
- Current-tree verification: python3 evidence/check_diagram.py prints only the documented server-lib to net-prim edge as drawn but not declared.
- Resolution reference: docs(issues): [#2757] correct the url serde cause and apply review round 1 fixes
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2784#discussion_r4515365858

### F9 - The new coupling report has no frontmatter.

- PR number: 2784
- Source review ID: 5829122067
- Reviewer finding ID: F6 (Nit, as labelled in the review)
- Source URL: https://github.com/torrust/torrust-tracker/pull/2784#discussion_r4515233829
- Concern: The new coupling report has no frontmatter.
- Solution: Valid. The same semantic-links block as the earlier reports is added by hand.
- Current-tree verification: head -12 of the report shows the frontmatter.
- Resolution reference: docs(issues): [#2757] correct the url serde cause and apply review round 1 fixes
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2784#discussion_r4515366266

### F10 - The deny.toml wrapper entry already existed; only the comment is new.

- PR number: 2784
- Source review ID: 5829122067
- Reviewer finding ID: F7 (Nit, as labelled in the review)
- Source URL: https://github.com/torrust/torrust-tracker/pull/2784#discussion_r4515233840
- Concern: The deny.toml wrapper entry already existed; only the comment is new.
- Solution: Valid. The spec, retrospective, inventory, a spec log correction entry and the PR description say so.
- Current-tree verification: git show ca8efcd4d:deny.toml lists the package as its own wrapper.
- Resolution reference: docs(issues): [#2757] correct the url serde cause and apply review round 1 fixes
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2784#discussion_r4515366630

### F11 - The EPIC row 7 sentence about the implementation PR tells the reader nothing.

- PR number: 2784
- Source review ID: 5829122067
- Reviewer finding ID: F8 (Nit, as labelled in the review)
- Source URL: https://github.com/torrust/torrust-tracker/pull/2784#discussion_r4515233849
- Concern: The EPIC row 7 sentence about the implementation PR tells the reader nothing.
- Solution: Valid. The sentence is removed.
- Current-tree verification: grep -n "Implemented in the implementation PR" EPIC.md finds nothing.
- Resolution reference: docs(issues): [#2757] correct the url serde cause and apply review round 1 fixes
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2784#discussion_r4515367046

### F12 - The audit cites a PR-branch commit by id.

- PR number: 2784
- Source review ID: 5829737162
- Reviewer finding ID: F9 (Minor, as labelled in the review)
- Source URL: https://github.com/torrust/torrust-tracker/pull/2784#discussion_r4515668450
- Concern: The audit cites a PR-branch commit by id.
- Solution: Valid. The audit says "the head then under review" and cites fixes by subject.
- Current-tree verification: grep for a 9-hex id in PR-REVIEW.md prints nothing.
- Resolution reference: docs(pr-reviews): [#2757] record the PR 2784 review round 3 audit
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2784#discussion_r4515746443

### F13 - The two no-change threads do not use the prescribed Superseded by reply and form.

- PR number: 2784
- Source review ID: 5829737162
- Reviewer finding ID: F10 (Minor, as labelled in the review)
- Source URL: https://github.com/torrust/torrust-tracker/pull/2784#discussion_r4515668458
- Concern: The two no-change threads do not use the prescribed Superseded by reply and form.
- Solution: Valid. The prescribed replies are posted on both threads; rows F1 and F3 cite them with NO_ACTION. The thread state stays OPEN until the threads are resolved.
- Current-tree verification: The two Superseded by replies exist (see the reply URLs of F1 and F3); the audit validator no longer reports F1 and F3.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2784#discussion_r4515746945
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2784#discussion_r4515746945

### F14 - The claim that every command is in evidence/verify.sh does not hold (no cargo chef step; the metadata scan has no command).

- PR number: 2784
- Source review ID: 5829737162
- Reviewer finding ID: F11 (Minor, as labelled in the review)
- Source URL: https://github.com/torrust/torrust-tracker/pull/2784#discussion_r4515668468
- Concern: The claim that every command is in evidence/verify.sh does not hold (no cargo chef step; the metadata scan has no command).
- Solution: Valid. verify.sh runs the V5 recipe steps and evidence/check_test_only.py (the scan); the evidence text names them.
- Current-tree verification: evidence/verify.sh has a V5 block and runs check_test_only.py.
- Resolution reference: docs(issues): [#2757] add the container recipe and the test-only scan to the verification script; docs(issues): [#2757] refresh the evidence transcript with the container recipe and the scan
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2784#discussion_r4515747375

### F15 - Two retrospective statements do not match the record (rows 4 to 6 baseline; who found the six test-helpers arrows).

- PR number: 2784
- Source review ID: 5829737162
- Reviewer finding ID: F12 (Minor, as labelled in the review)
- Source URL: https://github.com/torrust/torrust-tracker/pull/2784#discussion_r4515668477
- Concern: Two retrospective statements do not match the record (rows 4 to 6 baseline; who found the six test-helpers arrows).
- Solution: Valid. The retrospective says T1 ran only cargo test and no check builds this package alone, and credits the six arrows to the review.
- Current-tree verification: Root Cause 1 and What Changed 2 of the retrospective.
- Resolution reference: docs(issues): [#2757] align the retrospective with the record
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2784#discussion_r4515747893

### F16 - Two audit statements go further than their sources (grep lines; the reviewer agreeing with the F2 wording).

- PR number: 2784
- Source review ID: 5829737162
- Reviewer finding ID: F13 (Nit, as labelled in the review)
- Source URL: https://github.com/torrust/torrust-tracker/pull/2784#discussion_r4515668488
- Concern: Two audit statements go further than their sources (grep lines; the reviewer agreeing with the F2 wording).
- Solution: Valid. The audit says six lines (this package and five siblings) and quotes only what the review said.
- Current-tree verification: The Finding Details of F3 and the Round 2 paragraph.
- Resolution reference: docs(pr-reviews): [#2757] record the PR 2784 review round 3 audit
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2784#discussion_r4515748312

### F17 - One commit carries the fixes of seven audit rows.

- PR number: 2784
- Source review ID: 5829737162
- Reviewer finding ID: F14 (Nit, as labelled in the review)
- Source URL: https://github.com/torrust/torrust-tracker/pull/2784#discussion_r4515668496
- Concern: One commit carries the fixes of seven audit rows.
- Solution: Acknowledged; history is not rewritten. Every fix of this round is its own Conventional Commit.
- Current-tree verification: git log shows one commit per fix in this round.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2784#discussion_r4515748751
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2784#discussion_r4515748751

## Processing Log

- 2026-11-09 16:32 UTC - Started audit; round 1 (Copilot review 5828888059) processed: F2 fixed in the commit cited above, all three threads replied. Resolving the threads was denied by the auto mode classifier, so they stay OPEN for the maintainer. The fix and this audit go in one push.
- 2026-11-09 16:47 UTC - Round 2 (da2ce7 review 5829122067) processed: F4 to F11 fixed in the commits cited above (the measurement at the merge base settles F4), all eight threads replied, the PR description corrected for F4 and F10. Resolving the threads is again blocked by the classifier. One push for both rounds; Copilot and da2ce7 are to be re-requested by the maintainer after it.
- 2026-11-09 17:24 UTC - Round 3 (da2ce7 review 5829737162) processed: F12 to F16 fixed, F17 acknowledged; the prescribed Superseded by replies are posted on the two Copilot threads and rows F1 and F3 cite them; all six threads of this round replied. Resolving the threads is still blocked by the classifier. Each fix is its own commit; one push.

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
