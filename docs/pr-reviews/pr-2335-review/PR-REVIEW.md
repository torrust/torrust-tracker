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
| F6 | `review-finding:pr-2335-f6` | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F7 | `review-finding:pr-2335-f7` | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F8 | `review-finding:pr-2335-f8` | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F9 | `review-finding:pr-2335-f9` | Human | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F10 | `review-finding:pr-2335-f10` | Human | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F11 | `review-finding:pr-2335-f11` | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F12 | `review-finding:pr-2335-f12` | Human | Suggestion | correctness | ORIGINAL | FIXED | RESOLVED |

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
  done; `self-hosted-runner-security-research.md` records the history in Current Exposure; the
  runner query recorded in `runner-agent-installation.md` step 4 prints
  `torrust-runner-01  Linux  online  false  self-hosted,Linux,X64,torrust-hetzner`, and
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

### F6 - Runner-state text and `last-updated-utc` are stale

- PR number: 2335
- Source review ID: 5317648676
- Reviewer finding ID: F6
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2335#discussion_r4104515282>
- Concern: statements about the runner's registration and state contradicted each other after the
  removal, and ISSUE.md's `last-updated-utc` predated the changes.
- Solution: the research doc's Current Exposure section became a dated history plus the current
  state (registered again after T9), a note records the approval-policy change beside the
  2026-09-24 facts table, the ISSUE.md status note says T9 and T3 are done, and
  `last-updated-utc` was bumped.
- Current-tree verification: at the PR head after the 2026-09-26 changes,
  `self-hosted-runner-security-research.md` contains "History of the runner:" and "Current state
  (2026-09-26)"; ISSUE.md contains "T9 was completed on 2026-09-26, and the runner was then
  registered again (T3)" and `last-updated-utc: 2026-09-26 10:48`.
- Resolution reference: docs(issues): make the #2323 runner-state history coherent; docs(issues): update the #2323 status note after re-registration
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2335#discussion_r4111316053>

### F7 - Unchanged sections adopt the design the Risks bullet rejects

- PR number: 2335
- Source review ID: 5317648676
- Reviewer finding ID: F7
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2335#discussion_r4104515291>
- Concern: Decision, In Scope, ADRs to create, AC5, Design and Ownership, and EPIC row 15 still
  accepted the persistent-runner risk while the Risks bullet rejected it.
- Solution: the maintainers reversed the rejection and accepted the persistent runner with
  controls; the Risks bullet and status note were rewritten, so the listed sections and the EPIC
  row agree with Risks again.
- Current-tree verification: at the PR head after the 2026-09-26 changes, ISSUE.md contains
  "(accepted with controls)" in the Risks bullet and no "this design is rejected"; EPIC row 15
  still says "Moves `container.yaml` `test` to a persistent self-hosted runner".
- Resolution reference: docs(issues): accept the #2323 persistent runner with controls
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2335#discussion_r4111316112>

### F8 - Audit verification cites a removed head and a stale runner state

- PR number: 2335
- Source review ID: 5317648676
- Reviewer finding ID: F8
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2335#discussion_r4104515296>
- Concern: all verification fields cited head `48bc2a60`, which force-pushes removed, and
  PERSISTENT-RUNNER-PRIVILEGE's verification (`torrust-runner-01 offline`) no longer reproduced
  after the removal, which the Processing Log did not record.
- Solution: verification is anchored without a SHA and re-checked; PERSISTENT-RUNNER-PRIVILEGE
  covers the removal, the decision, T9, and the re-registration; the Processing Log gains the
  missing entries.
- Current-tree verification: at the PR head after the 2026-09-26 changes, this record contains no
  `48bc2a60` verification anchor, and the Processing Log has the 2026-09-25 07:35 removal entry.
- Resolution reference: docs(pr-reviews): refresh PR #2335 audit verification
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2335#discussion_r4111316154>

### F9 - "About 200 USD per month" does not survive the baseline

- PR number: 2335
- Source review ID: 5317648676
- Reviewer finding ID: F9
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2335#discussion_r4104515301>
- Concern: the estimate came from the earlier analysis's 13-minute 16-core job, while the T1
  baseline's core-independent parts alone take about 13.7 minutes; the analysis also priced the
  standard runner as 2-core instead of 4 vCPU.
- Solution: derived the estimate from the baseline (about 18.3 minutes per job; about 290 USD for
  PR runs and 330 USD with push runs), updated the Goal bullet, comparison row, server ratio (about
  a quarter), and research doc, and flagged the 2-core premise in the copied analysis.
- Current-tree verification: at the PR head after the 2026-09-26 changes, ISSUE.md contains
  "about 290 USD per month for the 378 PR runs" and "standard Linux runners for public repositories
  have 4 vCPUs and 16 GB"; no "200 USD" remains outside the dated Progress Log.
- Resolution reference: docs(issues): derive the #2323 larger-runner cost from the baseline
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2335#discussion_r4111316180>

### F10 - Containerfile `RUN` steps cannot reach the host Docker socket

- PR number: 2335
- Source review ID: 5317648676
- Reviewer finding ID: F10
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2335#discussion_r4104515307>
- Concern: research finding 1 said every PR-controlled item, including `Containerfile` `RUN` steps,
  can run `docker run --privileged -v /:/host`, but BuildKit sandboxes `RUN` steps without the
  host socket.
- Solution: finding 1 now separates code the job runs on the host (E2E tools via `cargo run`,
  `contrib/` scripts, workflow steps) from code inside the image build, which needs a BuildKit or
  container escape.
- Current-tree verification: at the PR head after the 2026-09-26 changes,
  `self-hosted-runner-security-research.md` contains "BuildKit runs each `RUN` step in its own
  sandbox without the host Docker socket".
- Resolution reference: docs(issues): scope the #2323 root-escalation claim to host code
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2335#discussion_r4111316212>

### F11 - "Even on ephemeral runners" contradicts finding 2

- PR number: 2335
- Source review ID: 5317648676
- Reviewer finding ID: F11
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2335#discussion_r4104515314>
- Concern: the research intro ruled out untrusted root even on ephemeral runners, while finding 2
  explains why GitHub-hosted ephemeral VMs grant root safely.
- Solution: the intro now scopes the position to machines that outlive the job and links to the
  Decision (2026-09-25) section that revised it.
- Current-tree verification: at the PR head after the 2026-09-26 changes,
  `self-hosted-runner-security-research.md` contains "not acceptable on any machine that outlives
  the job" and no "even on ephemeral runners".
- Resolution reference: docs(issues): reword the #2323 research root-access premise
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2335#discussion_r4111316267>

### F12 - Validator silently skips Findings rows it cannot parse

- PR number: 2335
- Source review ID: 5317648676
- Reviewer finding ID: F12
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2335#discussion_r4104515324>
- Concern: a Findings row with a padded or lowercase ID matched neither pattern, so the validator
  skipped it and still reported `ok` with fewer rows.
- Solution: the validator checks every data row of the `## Findings` table and fails on any row
  that does not parse, and on a missing section.
- Current-tree verification: at the PR head after the 2026-09-26 changes, the validator passes on
  this record, and a temporary copy with `| F5  |` and `| copilot-1 |` fails with "Findings row does
  not parse as a tracking row" for both.
- Resolution reference: fix(pr-reviews): fail on unparsed audit Findings rows
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2335#discussion_r4111316287>

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
- 2026-09-26 11:30 UTC - Fetched reviewer review 5317648676 (seven inline threads, F6 to F12, all
  new findings; F7 was already fixed by the 2026-09-25 decision). Fixed F6 and F8 to F12 in
  separate commits, rebased onto `torrust/develop`, pushed, and replied on all seven threads.

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
