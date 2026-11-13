---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
---

# PR #2857 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/2857.

## Ownership

The PR author owns this tracked audit record. Reviewers, including repository review agents,
deliver findings through GitHub and have no repository-artifact obligation.

- Post-merge workflow approval: N/A

This record was created late, after the findings below were already fixed and replied to on
GitHub; see the Processing Log.

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

Round 1 is `da2ce7`'s approving review `5889979674` (F1 to F5) and
`copilot-pull-request-reviewer`'s review `5890022304` (F6 and F7, no reviewer-provided ids), both
submitted on the head whose last commit had the subject `fix(container): [#2855] stop the tracker
with timeout --foreground`. `da2ce7`'s review was dismissed by GitHub when the fix commit below
was pushed (a stale-approval dismissal, not a new round).

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2857-f1 | Human | Minor | maintainability | ORIGINAL | NO_ACTION | RESOLVED |
| F2 | review-finding:pr-2857-f2 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-2857-f3 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F4 | review-finding:pr-2857-f4 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F5 | review-finding:pr-2857-f5 | Human | Nit | metadata | ORIGINAL | FIXED | RESOLVED |
| F6 | review-finding:pr-2857-f6 | Copilot | Minor (inferred) | maintainability | ORIGINAL | NO_ACTION | RESOLVED |
| F7 | review-finding:pr-2857-f7 | Copilot | Minor (inferred) | maintainability | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - Evidence not in this PR; `Closes` would close the issue before it lands

- PR number: 2857
- Source review ID: 5889979674
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2857#discussion_r4562610540
- Concern: `manual-verification-evidence.md` for #2855 lives only on the unmerged spec PR #2856's branch, and `create-issue` asks the spec-only PR to merge before implementation; merging this PR first would close #2855 while its evidence is unmerged.
- Solution: No action on the PR structure (the implementation was already open and reviewed before this was raised; splitting it now would discard reviewed work). Stated the intended merge order explicitly in the description instead: #2856 first, then this PR.
- Current-tree verification: `gh-bot pr view 2857 --json body` shows the added "Intended merge order" sentence.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2857#discussion_r4565424073
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2857#discussion_r4565424073

### F2 - Comment says the container is SIGKILLed, not the client

- PR number: 2857
- Source review ID: 5889979674
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2857#discussion_r4562610551
- Concern: `timeout` signals only its direct child (the `docker run` client) and that child's process group, not the container running under the Docker daemon; the comment said "the container ... is SIGKILLed", which is imprecise and contradicts the description/commit message.
- Solution: Reworded the comment to "the tracker never receives the signal and `--kill-after` SIGKILLs the `docker run` client instead (exit 137), leaving the container running."
- Current-tree verification: `contrib/dev-tools/containers/tests/test-mounted-no-persistence-configuration.sh` lines 32-37 (comment above `run_tracker`'s `timeout` call).
- Resolution reference: fix(container): [#2855] address PR #2857 review findings F2-F4,F7
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2857#discussion_r4565425149

### F3 - uutils behaviour stated without its version

- PR number: 2857
- Source review ID: 5889979674
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2857#discussion_r4562610555
- Concern: The comment named uutils `timeout` generically, though the regression is attributed specifically to the 0.8.0-to-0.10.0 upgrade; 0.8.0 is documented as having delivered the signal without `--foreground`.
- Solution: The comment now names the version the failure was observed on, 0.10.0.
- Current-tree verification: Same script lines as F2.
- Resolution reference: fix(container): [#2855] address PR #2857 review findings F2-F4,F7
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2857#discussion_r4565426075

### F4 - "the flag is a no-op there" holds only for this invocation

- PR number: 2857
- Source review ID: 5889979674
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2857#discussion_r4562610563
- Concern: GNU `timeout --foreground` changes process-group behaviour generally (it stays in the caller's process group, so descendants of the command are not timed out); calling it a blanket no-op is wrong for a command with descendants.
- Solution: Reworded to "has the same effect here, for this single-command invocation."
- Current-tree verification: Same script lines as F2.
- Resolution reference: fix(container): [#2855] address PR #2857 review findings F2-F4,F7
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2857#discussion_r4565427002

### F5 - PR title missing `[#2855]`; description omits a touched file

- PR number: 2857
- Source review ID: 5889979674
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2857#discussion_r4562610574
- Concern: `open-pull-request` asks for `[#<issue>]` in the title of a PR that closes the issue (the merge tool copies the title into the merge commit subject), and for the description to name every touched file, including `project-words.txt`.
- Solution: Added `[#2855]` to the title and a "Files touched" section naming both files.
- Current-tree verification: `gh-bot pr view 2857 --json title,body`.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2857#discussion_r4565427781
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2857#discussion_r4565427781

### F6 - Copilot: `--foreground` unsupported on BusyBox `timeout`

- PR number: 2857
- Source review ID: 5890022304
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2857#discussion_r4562638868
- Concern: BusyBox's `timeout` does not support `--foreground`, which could turn a previously-working environment into a hard failure; suggested gating the flag behind a feature check.
- Solution: No action: the script's shebang is `#!/usr/bin/env bash` and it already uses bash-only constructs, so it targets this project's own dev machines and the self-hosted CI runner (GNU or uutils coreutils), never BusyBox.
- Current-tree verification: `head -1 contrib/dev-tools/containers/tests/test-mounted-no-persistence-configuration.sh` shows the bash shebang.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2857#discussion_r4565430472
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2857#discussion_r4565430472

### F7 - Copilot: exit-code check duplicated

- PR number: 2857
- Source review ID: 5890022304
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2857#discussion_r4562638943
- Concern: 0 and 124 were encoded once in the diagnostic `if` and again in the final `test ... -o ...`, and `test ... -o ...` is obsolescent across shells.
- Solution: Consolidated into a single `case "${exit_status}" in 0 | 124) ;; *) ... return 1 ;; esac`, which both prints the diagnostic and fails, replacing `test ... -o ...`.
- Current-tree verification: Ran the fixed script locally (`bash contrib/dev-tools/containers/tests/test-mounted-no-persistence-configuration.sh`); exit 0, both regression markers printed.
- Resolution reference: fix(container): [#2855] address PR #2857 review findings F2-F4,F7
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2857#discussion_r4565428794

## Processing Log

- 2026-11-13 15:44 UTC - Started this record late, after `da2ce7`'s review `5889979674` (5 findings, F1 to F5) and `copilot-pull-request-reviewer`'s review `5890022304` (2 findings, F6 and F7) were already fixed/declined and replied to. F2, F3, F4 and F7 fixed in commit `fix(container): [#2855] address PR #2857 review findings F2-F4,F7`; F5 fixed via `gh-bot pr edit` (title and description); F1 and F6 declined with reasoning. Replied on all seven threads, then resolved them.
