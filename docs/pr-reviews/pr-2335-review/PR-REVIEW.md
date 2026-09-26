---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
    - docs/issues/open/2323-1840-hetzner-self-hosted-ci-runner/ISSUE.md
---

<!-- skill-link: process-pr-review -->

# PR #2335 Review Audit

Source: pull-request reviews and inline review threads for
<https://github.com/torrust/torrust-tracker/pull/2335>.

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

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| OPS-001 | `review-finding:pr-2335-ops-001` | Copilot | Major | correctness | ORIGINAL | FIXED | RESOLVED |
| PERSISTENT-RUNNER-PRIVILEGE | `review-finding:pr-2335-persistent-runner-privilege` | Copilot | Major | security | ORIGINAL | FIXED | RESOLVED |
| DOC-002 | `review-finding:pr-2335-doc-002` | Copilot | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| DOC-001 | `review-finding:pr-2335-doc-001` | Copilot | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F1 | `review-finding:pr-2335-f1` | Human | Major | security | RE_RAISE_OF:PERSISTENT-RUNNER-PRIVILEGE | NO_ACTION | SUPERSEDED |
| F2 | `review-finding:pr-2335-f2` | Human | Minor | correctness | RE_RAISE_OF:OPS-001 | NO_ACTION | SUPERSEDED |
| F3 | `review-finding:pr-2335-f3` | Human | Minor | documentation | RE_RAISE_OF:DOC-002 | NO_ACTION | SUPERSEDED |
| F4 | `review-finding:pr-2335-f4` | Human | Minor | documentation | RE_RAISE_OF:DOC-001 | NO_ACTION | SUPERSEDED |
| F5 | `review-finding:pr-2335-f5` | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### OPS-001 - `timeout-minutes` does not bound time spent queued

- PR number: 2335
- Source review ID: 5307823915
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2335#discussion_r4096463538>
- Concern: `timeout-minutes` starts counting only after a runner picks up the job, so a job queued
  for an offline self-hosted runner is not bounded by it and AC4 is not satisfied by that
  mechanism. The same claim appeared in the single-point-of-failure risk.
- Solution: documented `timeout-minutes` as an execution bound only, recorded GitHub's 24-hour
  queue limit for self-hosted jobs, and made runner-offline detection with alerting and a
  documented fallback procedure explicit in the Deadline bullet, AC4, M5, T8, the risk, and Open
  Question 6.
- Current-tree verification: at the PR head after the 2026-09-26 changes, ISSUE.md contains "`timeout-minutes` bounds only
  execution time" and AC4 requires "A runner-offline condition is detected and alerts maintainers,
  and a documented fallback procedure moves or reruns queued jobs".
- Resolution reference: docs(issues): require offline detection and fallback for #2323 runner
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2335#discussion_r4101805898>

### PERSISTENT-RUNNER-PRIVILEGE - The `docker` group gives fork-PR code root on the host

- PR number: 2335
- Source review ID: 5307823915
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2335#discussion_r4096463598>
- Concern: adding the `runner` account to the `docker` group gives fork-PR code root-equivalent
  control of the persistent host, including the runner install and credentials, with changes that
  persist into later jobs. Keeping Docker Hub secrets off the host does not contain it.
- Solution: the maintainer first ruled root-equivalent access for fork-PR code unacceptable. The
  risk bullet was rewritten to state the full exposure, the security research recorded the
  alternatives, and `torrust-runner-01` was stopped, disabled, and then removed from the
  repository (2026-09-25). After reviewing the contribution profile, the maintainers accepted the
  persistent runner with controls (2026-09-25): approval for all external contributors, required
  organization 2FA (T9, done 2026-09-26), Dependabot and `main`/`releases/**` events on
  GitHub-hosted runners, and publish isolation (T5). The runner was registered again on
  2026-09-26, after T9.
- Current-tree verification: at the PR head after the 2026-09-26 changes, ISSUE.md contains
  "Fork-PR code execution on a persistent runner (accepted with controls)" with the full exposure
  (runner registration, later `push` jobs, the `needs:` publish gate), and T9 and AC7 are marked
  done; `self-hosted-runner-security-research.md` records the history in Current Exposure;
  `gh api repos/torrust/torrust-tracker/actions/runners` reports
  `torrust-runner-01  online  self-hosted,Linux,X64,torrust-hetzner`, and
  `gh api repos/torrust/torrust-tracker/actions/permissions/fork-pr-contributor-approval`
  reports `all_external_contributors` (both checked 2026-09-26).
- Resolution reference: docs(issues): state the full fork-PR exposure of the #2323 runner; docs(issues): accept the #2323 persistent runner with controls
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2335#discussion_r4101806067>

### DOC-002 - The one-minute estimate for the remaining steps is wrong

- PR number: 2335
- Source review ID: 5307823915
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2335#discussion_r4096463639>
- Concern: the baseline table reports a median of 218 s for the E2E steps, so "all other steps
  together take about one minute" understates the non-build portion of the job.
- Solution: stated the E2E range separately (3.4 to 7.5 minutes, median 3.6) and the remaining
  setup and cleanup steps as about one minute, measured at 59 s in run `35972794173`.
- Current-tree verification: at the PR head after the 2026-09-26 changes, `benchmark-results.md` contains "The E2E steps
  take 3.4 to 7.5 minutes (median 3.6)".
- Resolution reference: docs(issues): correct non-build step durations in #2323 baseline
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2335#discussion_r4101806192>

### DOC-001 - The recorded runner output does not match its query

- PR number: 2335
- Source review ID: 5307823915
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2335#discussion_r4096463688>
- Concern: the documented `gh api` query emits only name, status, and labels, so the recorded
  output with OS and busy fields cannot be reproduced.
- Solution: extended the query to emit all five fields joined by two spaces, re-ran it, and
  recorded its exact output.
- Current-tree verification: at the PR head after the 2026-09-26 changes, `runner-agent-installation.md` shows the query
  with `.os` and `.busy` fields and the output
  `torrust-runner-01  Linux  online  false  self-hosted,Linux,X64,torrust-hetzner`.
- Resolution reference: docs(issues): align #2323 runner check output with its query
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2335#discussion_r4101806305>

### F1 - The accepted security model understates the fork-PR exposure

- PR number: 2335
- Source review ID: 5310557886
- Reviewer finding ID: F1
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2335#discussion_r4098761369>
- Concern: the accepted risk describes cache tampering on a host without credentials, while the
  setup gives any fork-PR job root-equivalent, persistent control, including the runner
  registration, the tokens of later push jobs, and a publish gate that can be faked; moving the `test` job
  also brings `main` and `releases/**` events onto the host.
- Solution: no separate change; it requests the same current-tree change as
  PERSISTENT-RUNNER-PRIVILEGE. That fix incorporates the exposure details listed here, and the
  research records them under finding 1.
- Current-tree verification: the PERSISTENT-RUNNER-PRIVILEGE checks at the PR head after the 2026-09-26 changes; the risk
  bullet names the runner registration, later `push` jobs, the `needs:` publish gate, and the
  `main` and `releases/**` events.
- Resolution reference: <https://github.com/torrust/torrust-tracker/pull/2335#discussion_r4101806447>
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2335#discussion_r4101806447>

### F2 - `timeout-minutes` is presented as a queue bound

- PR number: 2335
- Source review ID: 5310557886
- Reviewer finding ID: F2
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2335#discussion_r4098761378>
- Concern: four places tie `timeout-minutes` to the offline-runner case, but it only bounds a
  running job, and the fallback remained an open question.
- Solution: no separate change; it requests the same current-tree change as OPS-001.
- Current-tree verification: the OPS-001 checks at the PR head after the 2026-09-26 changes.
- Resolution reference: <https://github.com/torrust/torrust-tracker/pull/2335#discussion_r4101806658>
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2335#discussion_r4101806658>

### F3 - "About one minute" does not recompute from the table

- PR number: 2335
- Source review ID: 5310557886
- Reviewer finding ID: F3
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2335#discussion_r4098761383>
- Concern: by the table's medians the job minus the build step is about 314 s, of which 218 s is
  E2E, so the one-minute sentence is wrong.
- Solution: no separate change; it requests the same current-tree change as DOC-002.
- Current-tree verification: the DOC-002 checks at the PR head after the 2026-09-26 changes.
- Resolution reference: <https://github.com/torrust/torrust-tracker/pull/2335#discussion_r4101806789>
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2335#discussion_r4101806789>

### F4 - The recorded runner output cannot come from its command

- PR number: 2335
- Source review ID: 5310557886
- Reviewer finding ID: F4
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2335#discussion_r4098761389>
- Concern: the query emits three fields, while the recorded line has five, including `Linux` and
  `busy=false`.
- Solution: no separate change; it requests the same current-tree change as DOC-001.
- Current-tree verification: the DOC-001 checks at the PR head after the 2026-09-26 changes.
- Resolution reference: <https://github.com/torrust/torrust-tracker/pull/2335#discussion_r4101806936>
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2335#discussion_r4101806936>

### F5 - Server type says "To be recorded in T2" while T2 is DONE

- PR number: 2335
- Source review ID: 5310557886
- Reviewer finding ID: F5
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2335#discussion_r4098761395>
- Concern: the server-type cell defers to T2, but T2 is DONE and leaves the server type to Open
  Question 1.
- Solution: pointed the cell at Open Question 1 and noted that 8 vCPU / 16 GB at about €69
  matches the shared-vCPU CPX42 line of the earlier cost table.
- Current-tree verification: at the PR head after the 2026-09-26 changes, ISSUE.md's server-type cell starts with
  "Open (Open Question 1)".
- Resolution reference: docs(issues): point #2323 server type at open question 1
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2335#discussion_r4101807073>

## Processing Log

- 2026-09-24 17:26 UTC - Fetched Copilot review 5307823915 (four inline threads; the review body is
  a summary with no additional actionable assertion).
- 2026-09-24 18:50 UTC - Fixed OPS-001, DOC-002, and DOC-001 in separate commits; held
  PERSISTENT-RUNNER-PRIVILEGE for a maintainer decision.
- 2026-09-24 21:30 UTC - Maintainer ruled root-equivalent access for fork-PR code unacceptable;
  researched alternatives, stopped and disabled the runner, and put the design under review.
- 2026-09-25 06:20 UTC - Fetched reviewer review 5310557886 (five inline threads; the review body
  summarizes the same five findings). Mapped F1 to F4 as re-raises of the Copilot findings and F5
  as a new finding.
- 2026-09-25 06:30 UTC - Fixed PERSISTENT-RUNNER-PRIVILEGE and F5, pushed, and replied on all nine
  threads.
- 2026-09-25 07:00 UTC - Ran `validate-audit-record.py --pr-number 2335 --base torrust/develop`:
  `{"status": "ok", "rows": 5, "log_entries": 5, "failures": 0}`. The script only parses `F<n>`
  rows, so the four Copilot rows that keep their reviewer-provided IDs were checked by hand: each
  resolution subject appears exactly once in `torrust/develop..HEAD`, and each reply's
  `in_reply_to_id` is its source comment.
- 2026-09-25 07:02 UTC - `reply-status --login josecelano` exited 0; resolved all nine threads;
  a GraphQL refresh reports 9 threads, 0 unresolved.
- 2026-09-25 07:35 UTC - Removed `torrust-runner-01` from the repository and the server
  (`docs(issues): record removal of the #2323 runner`). This entry and the matching update to
  PERSISTENT-RUNNER-PRIVILEGE were added on 2026-09-26 after reviewer finding F8.
- 2026-09-25 07:47 UTC - Fixed the validator gap noted at 07:00 in
  `fix(pr-reviews): validate reviewer-provided finding IDs in audit records`. The fixed script
  reports `{"status": "ok", "rows": 9, "log_entries": 7, "failures": 0}` for this audit, and a
  temporary copy with a wrong OPS-001 severity and a reply URL from another thread fails with both
  errors.
- 2026-09-25 16:34 UTC - Maintainers accepted the persistent runner with controls
  (`docs(issues): accept the #2323 persistent runner with controls`).
- 2026-09-26 09:49 UTC - T9 controls done and `torrust-runner-01` registered again.
- 2026-09-26 11:15 UTC - F8: replaced the `48bc2a60` verification anchors with "the PR head after
  the 2026-09-26 changes", re-checked every verification string against the current tree, and
  brought PERSISTENT-RUNNER-PRIVILEGE's Solution and verification up to the removal, the
  decision, and the re-registration.

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
