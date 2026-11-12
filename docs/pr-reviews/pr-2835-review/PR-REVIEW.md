---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
---

<!-- skill-link: process-pr-review -->

# PR #2835 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/2835.

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

F1, F2, R2 and R1 are in Copilot's review 5862741540. F7 to F12 are in da2ce7's review 5870191489; his own IDs F1 to F6 collide with Copilot's F1 and F2, so each is recorded as the reviewer finding ID of an audit-assigned F7 to F12. All are inline threads.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2835-f1 | Copilot | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-2835-f2 | Copilot | Minor | testing | ORIGINAL | FIXED | RESOLVED |
| R2 | review-finding:pr-2835-r2 | Copilot | Minor | testing | ORIGINAL | FIXED | RESOLVED |
| R1 | review-finding:pr-2835-r1 | Copilot | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F7 | review-finding:pr-2835-f7 | Human | Major | documentation | ORIGINAL | FIXED | RESOLVED |
| F8 | review-finding:pr-2835-f8 | Human | Minor | testing | ORIGINAL | FIXED | RESOLVED |
| F9 | review-finding:pr-2835-f9 | Human | Minor | testing | ORIGINAL | FIXED | RESOLVED |
| F10 | review-finding:pr-2835-f10 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F11 | review-finding:pr-2835-f11 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F12 | review-finding:pr-2835-f12 | Human | Suggestion | maintainability | ORIGINAL | NO_ACTION | RESOLVED |

## Finding Details

### F1 - Stop the main flow when the identity lookup is interrupted

- PR number: 2835
- Source review ID: 5862741540
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2835#discussion_r4541082521
- Concern: A signal during `api user` ended only `report_identity`, so `wait` could still start another command and a spawn failure could replace the `interrupted` diagnostic.
- Solution: `report_identity` returns the interruption to `main`, which reports `interrupted` and exits 1 before the wait starts.
- Current-tree verification: `main.rs` `report_identity` and its caller; the new scenario `it_should_stop_on_sigterm_during_the_lookup_without_running_another_command` passes.
- Resolution reference: fix(wait-for-pr-review): [#2822] stop on a signal during the identity lookup and test its bound
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2835#discussion_r4541757954

### F2 - Test the stated lookup bound rather than only an early return

- PR number: 2835
- Source review ID: 5862741540
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2835#discussion_r4541082594
- Concern: The CLI scenario also accepted a four- or six-second lookup, and nothing covered the ten-second cap.
- Solution: `lookup_bound` is extracted with unit tests for both branches of `min(10 s, timeout / 4)`, and the CLI scenario asserts a lower and an upper time.
- Current-tree verification: `main.rs` unit tests `it_should_bound_the_lookup_at_a_quarter_of_a_short_timeout` and `it_should_cap_the_lookup_bound_at_ten_seconds_for_a_long_timeout`; the scenario's two assertions.
- Resolution reference: fix(wait-for-pr-review): [#2822] stop on a signal during the identity lookup and test its bound
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2835#discussion_r4541758586

### R2 - Complete the stalled-lookup verification before marking M4 done

- PR number: 2835
- Source review ID: 5862741540
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2835#discussion_r4541082663
- Concern: M4 and M4b had ended in `gh_api_error` with an incomplete fake and only M1 and M3 were rerun.
- Solution: M4 and M4b were rerun with the corrected fake and their stdout, exit code and timing are recorded.
- Current-tree verification: The M4 and M4b rows of `manual-verification-evidence.md`.
- Resolution reference: docs(issues): [#2822] keep the manual-verification fake and rerun M4
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2835#discussion_r4541759130

### R1 - Preserve the manual-verification fake and setup

- PR number: 2835
- Source review ID: 5862741540
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2835#discussion_r4541082723
- Concern: The fake lived only in a scratchpad outside the repository.
- Solution: The fake is kept as `evidence/fake-gh-command.sh`, embedded in the evidence file with its setup commands.
- Current-tree verification: `evidence/fake-gh-command.sh` and the evidence file's environment section.
- Resolution reference: docs(issues): [#2822] keep the manual-verification fake and rerun M4
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2835#discussion_r4541759863

### F7 - The red-run breakdown does not recompute from the committed tests

- PR number: 2835
- Source review ID: 5870191489
- Reviewer finding ID: F1
- Source URL: https://github.com/torrust/torrust-tracker/pull/2835#discussion_r4547152232
- Concern: The evidence said 10 new and 4 existing tests failed; the committed tests give 8 new and 6 existing, and two new tests pass without the option.
- Solution: The red-run section is recomputed with the reasons for each group.
- Current-tree verification: `manual-verification-evidence.md` red-run section; the reviewer's recount against the binary at the merge base.
- Resolution reference: test(wait-for-pr-review): [#2822] pin the empty-value message and the interrupted lookup
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2835#discussion_r4548115644

### F8 - The empty-value test passes with the option removed

- PR number: 2835
- Source review ID: 5870191489
- Reviewer finding ID: F2
- Source URL: https://github.com/torrust/torrust-tracker/pull/2835#discussion_r4547152242
- Concern: An unknown option is also a usage error with exit 2.
- Solution: The test also asserts the message `expected a program name or path`.
- Current-tree verification: `it_should_exit_two_when_gh_command_is_empty` in `tests/cli.rs`.
- Resolution reference: test(wait-for-pr-review): [#2822] pin the empty-value message and the interrupted lookup
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2835#discussion_r4548116152

### F9 - The signal-during-lookup scenario does not discriminate its fix

- PR number: 2835
- Source review ID: 5870191489
- Reviewer finding ID: F3
- Source URL: https://github.com/torrust/torrust-tracker/pull/2835#discussion_r4547152249
- Concern: The CLI scenario usually passes on the unfixed code because the spawned child is killed at once.
- Solution: A deterministic unit test returns the interruption of the lookup against `yes`; the evidence says the CLI scenario guards without pinning the regression.
- Current-tree verification: `it_should_return_the_interruption_of_the_identity_lookup_to_the_caller` in `main.rs`; the evidence section on the signal.
- Resolution reference: test(wait-for-pr-review): [#2822] pin the empty-value message and the interrupted lookup
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2835#discussion_r4548116673

### F10 - The AC5 evidence over-generalizes

- PR number: 2835
- Source review ID: 5870191489
- Reviewer finding ID: F4
- Source URL: https://github.com/torrust/torrust-tracker/pull/2835#discussion_r4547152255
- Concern: Only some scenarios reach the fake through `--gh-command`.
- Solution: AC5's text, its evidence row and the PR description are narrowed to the custom-command scenarios.
- Current-tree verification: AC5 in the spec; `gh pr view 2835 --json body`.
- Resolution reference: test(wait-for-pr-review): [#2822] pin the empty-value message and the interrupted lookup
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2835#discussion_r4548117299

### F11 - The help text is not exact about which value skips auth status

- PR number: 2835
- Source review ID: 5870191489
- Reviewer finding ID: F5
- Source URL: https://github.com/torrust/torrust-tracker/pull/2835#discussion_r4547152267
- Concern: `--gh-command gh` is still checked with `auth status`.
- Solution: Help and skill say the value `gh` is checked and any other value is not.
- Current-tree verification: The `--gh-command` doc comment in `main.rs`; the skill option row.
- Resolution reference: test(wait-for-pr-review): [#2822] pin the empty-value message and the interrupted lookup
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2835#discussion_r4548117989

### F12 - Two diagnostics still name `gh` for a custom command

- PR number: 2835
- Source review ID: 5870191489
- Reviewer finding ID: F6
- Source URL: https://github.com/torrust/torrust-tracker/pull/2835#discussion_r4547152271
- Concern: `MalformedResponse` and `DeadlineExceeded` say `gh`.
- Solution: No change: D2 scoped the rename to `gh_api_error` and `gh_unavailable`; the reply offers a follow-up if wanted.
- Current-tree verification: D2 of the merged spec; `error.rs`.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2835#discussion_r4548118436
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2835#discussion_r4548118436

## Processing Log

- 2026-11-11 17:24 UTC - Review 5862741540 (Copilot) submitted on the first head of the PR: F1, F2, R2, R1.
- 2026-11-11 18:24 UTC - The 60-minute wait for da2ce7 ended without a review; the wait stopped and the hand-over says so.
- 2026-11-11 18:27 UTC - Pushed the two commits that fix the four findings (push log time).
- 2026-11-11 18:28 UTC - Replied to and resolved the four threads after that push; the replies cite the pushed commit subjects and their URLs are in the entries.
- 2026-11-12 06:37 UTC - Review 5870191489 (da2ce7, `CHANGES_REQUESTED`) submitted on the head of the previous push: six findings.
- 2026-11-12 08:36 UTC - Pushed the commit that fixes them (push log time).
- 2026-11-12 08:37 UTC - Replied to and resolved the six threads after that push; F12 is declined with its reason (reply URL in the entry).

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
