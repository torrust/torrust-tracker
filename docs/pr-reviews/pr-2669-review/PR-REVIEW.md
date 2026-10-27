---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
    - docs/issues/open/2668-1488-si-20-configure-shutdown-policy/ISSUE.md
    - docs/issues/open/2668-1488-si-20-configure-shutdown-policy/manual-verification-evidence.md
---

<!-- skill-link: process-pr-review -->

# PR #2669 Review Audit

Source: pull-request reviews and inline review threads for <https://github.com/torrust/torrust-tracker/pull/2669>.

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

One review was submitted so far: the review of `copilot-pull-request-reviewer` (5664064939), submitted at
08:27:54 UTC on the head whose subject is `docs(issues): add issue specification for #2668`. It has three
inline threads and a body that only summarizes them, so the body adds no row. The reviewer labelled its
findings `R1` to `R3`; they are kept as the `Reviewer finding ID` and the audit assigns `F1` to `F3` in source
order, which is the order of the review's comment ids.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------- | ------------ | -------- | ----------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2669-f1 | Copilot | Major | testing | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-2669-f2 | Copilot | Minor | testing | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-2669-f3 | Copilot | Major | correctness | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - Record the required pre-review bug reproduction

- PR number: 2669
- Source review ID: 5664064939
- Reviewer finding ID: R2
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2669#discussion_r4386450358>
- Concern: the specification said the bug workflow was not applicable, although it corrects a real defect
  (process exit code `0` after a failed or aborted shutdown). The `fix-bug` skill ("Reproduce Before Review")
  then requires a pre-review reproduction attempt recorded as `Reproduced`, `Trigger only`, or `Infeasible` in
  `manual-verification-evidence.md`, plus the workflow link, the regression boundary, and the final
  like-for-like recheck plan.
- Solution: the finding was right, and the reproduction was run before the reply. On the unmodified binary four
  network attempts (an idle connection, a partial header, a REST API body, and an announce body) did not fail a
  component: every run exited 0 within 4 s, so no reproduction is claimed for the unmodified artifact. With a
  temporary fault-injection patch, reverted afterwards, an aborted component and a failed component both gave
  exit status 0 and the success message, for SIGTERM and SIGINT. The classification is therefore
  `Reproduced, with a temporary fault-injection patch`, stated as such. The evidence file was created from the
  template, with the script and configuration under `evidence/`; the two "Not applicable" statements became the
  workflow link, the regression boundary, and the recheck plan; T3 was split into a red test and the fix; T1 is
  marked done; M1 and M5 were rewritten; and the disposable script is recorded. The issue type stays `task`,
  which the maintainer confirmed.
- Current-tree verification: `grep -n "Reproduced, with a temporary fault-injection patch"
  docs/issues/open/2668-1488-si-20-configure-shutdown-policy/manual-verification-evidence.md` finds the
  classification, and `git show torrust/develop:src/app.rs | cmp - src/app.rs` printed nothing after the revert,
  so no fault-injection code is in the tree. The Bug-Fix Process and Regression Test Strategy sections of the
  specification now name the skill and no longer say "Not applicable".
- Resolution reference: `docs(issues): record the pre-review reproduction of exit code 0 after a failed shutdown`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2669#discussion_r4386921636>

### F2 - Use the registered lifecycle test target

- PR number: 2669
- Source review ID: 5664064939
- Reviewer finding ID: R3
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2669#discussion_r4386450401>
- Concern: the Automatic Checks listed `cargo test --test integration`, but the root `Cargo.toml` registers
  `tests/lifecycle/signals.rs` as `lifecycle-signals` and has no `integration` target, so the command would not
  run the executable-boundary tests of T6.
- Solution: the finding was right about the effect and imprecise about the mechanism. The command does not fail
  target selection: `cargo test --test integration --no-run` exited 0 and built five `tests/integration.rs`
  executables of other workspace packages, while `cargo test -p torrust-tracker --test integration --no-run`
  reports no such target for the root package. Either way it never runs the root lifecycle tests. The check now
  uses `cargo test --test lifecycle-signals`, and T6 registers each new test file as its own `[[test]]` target,
  because Cargo does not discover files in subdirectories of `tests/`. The other commands in the specification
  were rechecked: `cargo test -p torrust-tracker-configuration --no-run` builds and
  `contrib/dev-tools/git/hooks/pre-push.sh` exists.
- Current-tree verification: `cargo test --test lifecycle-signals --no-run` finished and printed the
  `tests/lifecycle/signals.rs` executable, and `grep -n "test integration"
  docs/issues/open/2668-1488-si-20-configure-shutdown-policy/ISSUE.md` finds nothing.
- Resolution reference: `docs(issues): use the registered lifecycle-signals test target in the #2668 checks`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2669#discussion_r4386921912>

### F3 - Make scenario M3 satisfy the chain rule

- PR number: 2669
- Source review ID: 5664064939
- Reviewer finding ID: R1
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2669#discussion_r4386450429>
- Concern: scenario M3 set the deadline to 5 s, the HTTP drain to 2 s, and the listener drain to 1 s, and left the
  UDP drain at its 5 s default, so `max(2, 5) + 1 = 6 >= 5` violates the specification's own rule
  `max(H, U) + L < P`. Startup would reject the configuration before the scenario reached the drain failure it
  shows.
- Solution: the finding was right. M3 now sets `udp_request_drain_in_secs = 1` (`max(2, 1) + 1 = 3 < 5`). Every
  other worked example was rechecked against the rule: M2 (defaults, 20 < 25), M4 (now six explicit
  configurations with their arithmetic, one of them the accepted boundary), M6 (HTTP drain 3 s, 10 < 25), M7
  (defaults, 20 < 25), T6 (the M3 budgets), and the `DEADLINE_IN_SECS=30` override example (20 < 30). The real-binary
  attempts of F1 also showed that a held request ends within about 5 s on its own, so M6 now uses a 3 s drain
  and M7 uses the temporary fault-injection patch; those two rows are part of the same fix because they were
  re-derived in the same sweep.
- Current-tree verification: `grep -n "udp_request_drain_in_secs = 1"
  docs/issues/open/2668-1488-si-20-configure-shutdown-policy/ISSUE.md` finds the T6 and M3 rows and the
  progress-log entry, and the arithmetic of each example above was recomputed by hand against
  `max(H, U) + L < P`.
- Resolution reference: `docs(issues): make the #2668 worked examples satisfy the shutdown chain rule`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2669#discussion_r4386922220>

## Processing Log

- 2026-10-27 08:27 UTC - The review of `copilot-pull-request-reviewer` was submitted with three inline threads
  (GitHub `submitted_at` 08:27:54Z). It was not processed before the author's report on the same pull request,
  which looked at CI only; the coordinator pointed it out afterwards.
- 2026-10-27 09:06 UTC - Each finding was checked against the current tree and confirmed (F2 with the mechanism
  corrected). The fixes were made in three signed commits, with author times 09:02:40 (F1), 09:05:00 (F2), and
  09:06:30 (F3).
- 2026-10-27 09:18 UTC - The three fix commits were pushed to the pull-request branch (the reviews and comments
  were re-read at 09:17 with the head already on GitHub: still one review and the same three threads). Replies
  were then posted on all three threads (GitHub `created_at` 09:17:47Z, 09:17:49Z, and 09:17:51Z), and the three
  threads were resolved. A refreshed GraphQL fetch lists no unresolved thread.

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
